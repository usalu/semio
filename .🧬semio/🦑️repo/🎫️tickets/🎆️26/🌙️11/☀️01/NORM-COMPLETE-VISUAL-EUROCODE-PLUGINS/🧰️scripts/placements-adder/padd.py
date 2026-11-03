#!/usr/bin/env python3
"""Adds the persisted `placements` collection (id, x, y, z, yaw) to a norm artifact by cloning the artifact's own last row collection
through every twin, idempotently. Usage: python3 -I padd.py <artifact-key> [--dry]"""
import json, re, os, sys, difflib

NORM = "/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts"
SUBSET = "/🏅️standards/🔖️1/🪆️subsets/✳️any"
DRY = "--dry" in sys.argv
FIELDS = ["x", "y", "z", "yaw"]


def rd(p):
    return open(p, encoding="utf-8").read()


def wr(p, t):
    if DRY:
        return
    open(p, "w", encoding="utf-8").write(t)


def edit(p, fn):
    t = rd(p)
    n = fn(t)
    if n != t:
        if DRY:
            print("   ", sum(1 for l in difflib.unified_diff(t.splitlines(), n.splitlines(), n=0) if l[:1] in "+-" and l[:3] not in ("+++", "---")), "changed lines")
        wr(p, n)
        print("  edited", p.split("/artifacts/")[-1][:120] if "/artifacts/" in p else p)
    else:
        print("  unchanged", p.split(SUBSET)[-1])


class Art:
    def __init__(self, folder, prefix, snake, camel, pascal, table=None, model=None, new_model="Placement"):
        self.root = NORM + "/" + folder + SUBSET
        self.P, self.snake, self.camel, self.pascal = prefix, snake, camel, pascal
        self.model, self.new_model = model or pascal, new_model
        self.table = table
        self.lower = prefix.lower()

    def path(self, rel):
        return self.root + rel

    def rn(self, s):
        if self.model != self.pascal:
            s = s.replace(self.model, self.new_model)
        return s.replace(self.pascal, "Placement").replace(self.snake, "placements").replace(self.camel, "placements")

    def tok(self):
        return re.compile(r"\b(%s|%s|%s)" % (self.pascal, self.snake, self.camel))


def clone_line(text, line_re, A, attrs=True, indent_attr=True, count=None, rn=None, fix=None):
    """Duplicate each line matching line_re (with its directly preceding attribute lines) right after itself, renamed."""
    rn = rn or A.rn
    lines = text.split("\n")
    out, i, hits = [], 0, 0
    while i < len(lines):
        l = lines[i]
        out.append(l)
        if re.search(line_re, l):
            hits += 1
            block = [l]
            j = len(out) - 2
            while attrs and j >= 0 and re.match(r"\s*(#\[|/\*\*|/// )", out[j]) and not re.search(line_re, out[j]):
                block.insert(0, out[j])
                j -= 1
            new = [rn(b) for b in block]
            if fix:
                new = fix(new)
            if new[-1] not in lines[i + 1:i + 2 + len(new)] and rn(l) not in lines[max(0, i - 1):i + 3]:
                out.extend(new)
        i += 1
    if count is not None:
        assert hits == count, (line_re, hits, count)
    return "\n".join(out)


def emoji_fix(new):
    return [re.sub(r"(/\*\* )\S+( @state)", r"\1📍️\2", x) for x in new]


def layer_model_struct(A, path):
    def go(t):
        if "pub struct %s {" % A.new_model in t:
            return t
        m = re.search(r"((?:^(?:#\[[^\n]*|///[^\n]*)\n)+)pub struct %s \{\n.*?\n\}\n" % A.model, t, re.M | re.S)
        attrs = "\n".join(l for l in m.group(1).split("\n") if l.startswith("#[")) + "\n"
        struct = ("/// 📍️ Persisted pose of one scene entity (`id` is the entity id; SI: m, rad). Entities without a placement follow the documented default layout.\n" + attrs +
                  "pub struct " + A.new_model + " {\n    pub id: String,\n    /// ↔️ Origin x [m].\n    pub x: f64,\n    /// ↕️ Origin y [m].\n    pub y: f64,\n    /// ⬆️ Origin z [m].\n    pub z: f64,\n    /// 🧭️ Yaw about +Z [rad].\n    pub yaw: f64,\n}\n")
        return t[:m.end()] + "\n" + struct + t[m.end():]
    edit(path, go)


def ts_iface(A, path, style_hint=None):
    def go(t):
        pas = A.model
        m = re.search(r"export interface (\w*%s) \{(.*?)\n\}\n" % pas, t, re.S)
        if not m or "Placement {" in t:
            return t
        name = m.group(1)
        new_name = A.rn(name)
        iface = "export interface %s {\n  id: string;\n  x: number;\n  y: number;\n  z: number;\n  yaw: number;\n}\n" % new_name
        t = t[:m.end()] + "\n" + iface + t[m.end():]
        pm = re.search(r"^export const parse%s: NormWireReader<%s> = normWireObject<%s>\(\{.*\}\);$" % (name, name, name), t, re.M)
        if pm:
            body = "id: normWireRequired(normWireString), " + ", ".join("%s: normWireRequired(normWireNumber)" % f for f in FIELDS)
            line = "export const parse%s: NormWireReader<%s> = normWireObject<%s>({ %s });" % (new_name, new_name, new_name, body)
            t = t[:pm.end()] + "\n" + line + t[pm.end():]
        return t
    edit(path, go)


def ts_root_field(A, path, iface_name_re=r"En\d+(?:Snapshot|Artifact)"):
    def go(t):
        t = clone_line(t, r"^\s+%s: \w*%s\[\];" % (A.camel, A.model), A, fix=emoji_fix)
        def p(m):
            body = m.group(0)
            if "placements:" in body:
                return body
            entry = re.search(r"%s: [^,]*?parse\w*%s\)+(?:, \(\) => \[\]\))?" % (A.camel, A.model), body)
            assert entry, "ts parse entry"
            return body.replace(entry.group(0), entry.group(0) + ", " + A.rn(entry.group(0)), 1)
        return re.sub(r"^export const parse%s: .*$" % iface_name_re, p, t, flags=re.M)
    edit(path, go)


def graphql_snapshot(A, path, tok=None):
    def go(t):
        if "type Placement" in t or re.search(r"type \w*Placement \{", t):
            return t
        m = re.search(r"type (\w*%s) \{.*?\n\}\n|type (\w*%s) \{[^\n]*\}\n" % ((tok or A.model), (tok or A.model)), t, re.S)
        name = m.group(1) or m.group(2)
        nn = A.rn(name)
        multi = "\n" in m.group(0).strip("\n")
        if multi:
            block = "type %s {\n  id: String!\n  x: Float!\n  y: Float!\n  z: Float!\n  yaw: Float!\n}\n" % nn
        else:
            block = "type %s { id: String!, x: Float!, y: Float!, z: Float!, yaw: Float! }\n" % nn
        t = t[:m.end()] + "\n" * (1 if multi else 0) + block + t[m.end():]
        return clone_line(t, r"^\s+%s: \[\w*%s!\]!" % (A.camel, (tok or A.model)), A)
    edit(path, go)


def proto_snapshot(A, path, tok=None):
    def go(t):
        if re.search(r"message \w*Placement ", t):
            return t
        m = re.search(r"message (\w*%s) \{[^\n]*\}\n|message (\w*%s) \{\n.*?\n\}\n" % ((tok or A.model), (tok or A.model)), t, re.S)
        name = m.group(1) or m.group(2)
        nn = A.rn(name)
        if m.group(1):
            block = "message %s { string id = 1; double x = 2; double y = 3; double z = 4; double yaw = 5; }\n" % nn
        else:
            block = "\nmessage %s {\n  string id = 1;\n  double x = 2;\n  double y = 3;\n  double z = 4;\n  double yaw = 5;\n}\n" % nn
        fm = re.search(r"^(\s+)repeated %s %s = (\d+);\n" % (name, A.snake), t, re.M)
        start = t.rfind("\nmessage ", 0, fm.start()) + 1
        end = t.find("\n}", fm.end() - 1)
        numbers = [int(n) for n in re.findall(r"= (\d+);", t[start:end + 2])]
        line = "%srepeated %s placements = %d;\n" % (fm.group(1), nn, max(numbers) + 1)
        t = t[:fm.end()] + line + t[fm.end():]
        return t[:m.end()] + block + t[m.end():]
    edit(path, go)


FORMS = [dict(indent=2, ensure_ascii=False), dict(separators=(",", ":"), ensure_ascii=False), dict(ensure_ascii=False), dict(indent=2, ensure_ascii=True)]


class Hybrid:
    """indent-2 document whose definition entries are one-liners."""
    @staticmethod
    def dumps(d, **_):
        dk = "$defs" if "$defs" in d else "definitions"
        rest = {k: v for k, v in d.items() if k != dk}
        body = json.dumps(rest, indent=2, ensure_ascii=False)
        entries = ",\n".join("    %s: %s" % (json.dumps(k, ensure_ascii=False), json.dumps(v, ensure_ascii=False)) for k, v in d[dk].items())
        return body[:-2] + ',\n  "%s": {\n%s\n  }\n}' % (dk, entries)


class Lines:
    """top-level keys one per line with compact values, `$defs` entries one per line."""
    @staticmethod
    def dumps(d):
        c = dict(separators=(",", ":"), ensure_ascii=False)
        out = []
        for k, v in d.items():
            if k in ("$defs", "definitions"):
                ents = ",\n".join("    " + json.dumps(a, ensure_ascii=False) + ":" + json.dumps(x, **c) for a, x in v.items())
                out.append('  "%s":{\n%s\n  }' % (k, ents))
            else:
                out.append("  " + json.dumps(k) + ":" + json.dumps(v, **c))
        return "{\n" + ",\n".join(out) + "\n}"


def dumps_with(form, d):
    if form == "lines":
        return Lines.dumps(d)
    return Hybrid.dumps(d) if form == "hybrid" else json.dumps(d, **form)


def json_form(data_text):
    d = json.loads(data_text)
    for f in FORMS + ["hybrid", "lines"]:
        for tail in ("\n", ""):
            try:
                if dumps_with(f, d) + tail == data_text:
                    return f, tail
            except Exception:
                pass
    raise AssertionError("unknown json formatting")


def json_snapshot(A, path):
    def go(t):
        data = json.loads(t)
        form, tail = json_form(t)
        dk = "$defs" if "$defs" in data else ("definitions" if "definitions" in data else None)
        defs = data[dk] if dk else {}
        if "placements" in data["properties"]:
            return t
        dname = next((k for k in defs if k.endswith(A.model)), "Placement")
        ref = json.dumps(data["properties"][A.camel])
        prop = json.loads(A.rn(ref))
        pd = {"title": A.rn(dname), "type": "object", "additionalProperties": False, "required": ["id"] + FIELDS, "properties": {"id": {"type": "string", "x-semio-ui": {"widget": "reference", "label": {"en": "Placed entity", "de": "Platziertes Element"}, "ref": {"kind": "entity"}}}}}
        for f, en, de, unit, disp, fac, prec, step in [("x", "Origin x", "Ursprung x", "m", "m", 1, 3, 0.1), ("y", "Origin y", "Ursprung y", "m", "m", 1, 3, 0.1), ("z", "Origin z", "Ursprung z", "m", "m", 1, 3, 0.1), ("yaw", "Yaw about Z", "Gierwinkel um Z", "rad", "°", 57.29577951308232, 1, 0.0174532925)]:
            pd["properties"][f] = {"type": "number", "x-semio-ui": {"label": {"en": en, "de": de}, "widget": "stepper", "unit": unit, "displayUnit": disp, "displayFactor": fac, "precision": prec, "step": step}}
        if dk is None:
            del pd["title"]
            prop = {k: v for k, v in data["properties"][A.camel].items() if k != "items"}
            prop["items"] = pd
            prop = {"type": "array", "items": pd, **{k: v for k, v in prop.items() if k not in ("type", "items")}}
        else:
            if "title" not in defs[dname]:
                del pd["title"]
            if "x-semio-formats" in defs[dname]:
                pd["x-semio-formats"] = defs[dname]["x-semio-formats"]
            nd = {}
            for k, v in defs.items():
                nd[k] = v
                if k == dname:
                    nd[A.rn(dname)] = pd
            data[dk] = nd
        props = {}
        for k, v in data["properties"].items():
            props[k] = v
            if k == A.camel:
                props["placements"] = prop
        data["properties"] = props
        if "required" in data and A.camel in data["required"]:
            data["required"].insert(data["required"].index(A.camel) + 1, "placements")
        return dumps_with(form, data) + tail
    edit(path, go)


def para_clone(text, A, pred, fix=None):
    """Clone every blank-line separated paragraph satisfying pred right after itself, renamed."""
    paras = text.split("\n\n")
    if any(A.rn(p) == p for p in []):
        return text
    out = []
    existing = set(paras)
    for p in paras:
        out.append(p)
        if pred(p):
            n = A.rn(p)
            n = "\n".join(x for x in n.split("\n") if not x.startswith("//#"))
            if fix:
                n = fix(n)
            if n not in existing:
                out.append(n)
    return "\n\n".join(out)


def clone_block(t, start_re, A):
    lines = t.split("\n")
    out, i = [], 0
    while i < len(lines):
        m = re.match(start_re, lines[i])
        out.append(lines[i])
        if m:
            ind = m.group(1)
            j = i
            while lines[j] != ind + "}":
                j += 1
            blk = lines[i:j + 1]
            out.extend(blk[1:])
            new = [A.rn(x) for x in blk]
            if new[0] not in lines:
                out.extend(new)
            i = j
        i += 1
    return "\n".join(out)


def sub_inline_empty(t, A):
    pat = r"self\.%s\.as_ref\(\)\.map_or\(true, \|rows\| rows\.is_empty\(\)\)" % A.snake
    if re.search(pat, t) and A.rn("self.%s.as_ref()" % A.snake) not in t:
        t = re.sub(pat, lambda m: m.group(0) + " && " + A.rn(m.group(0)), t, count=1)
    return t


def diff_rs(A, path, patch_set="x: f64, y: f64, z: f64, yaw: f64"):
    L = A.pascal
    def go(t):
        if "PlacementKey" in t or "Placement" + "Delta" in t or "PlacementRows" in t:
            return t
        def pred(p):
            return bool(re.search(r"%sKey\b|protocol::(row_patch|list_delta)! \{\n(?:.*\n)*?.*%ss?(Patch|Delta|Rows)\b|impl protocol::list_delta::KeyOf<[^>]*%s>|row_key!.*%s" % (L, L, L, L), p)) and "ActionKey" not in p
        def fix(n):
            return re.sub(r"(Placement(?:Patch|Rows|Delta)?\w* of [^{]*\{ set \{)[^}]*(\})", lambda m: m.group(1) + " " + patch_set + " " + m.group(2), n) if " set {" in n else n
        t = para_clone(t, A, pred, fix)
        t = clone_line(t, r"^\s+pub %s: (?:Option<)?\w+%ss?(?:Delta|Rows)>?,?$" % (A.snake, L), A)
        t = clone_line(t, r"^\s+self\.%s\.absorb\(other\.%s\);" % (A.snake, A.snake), A)
        t = clone_block(t, r"^(\s+)if let Some\(theirs\) = other\.%s \{" % A.snake, A)
        t = clone_block(t, r"^(\s+)if let Some\(rows\) = &self\.%s \{" % A.snake, A)
        t = sub_inline_empty(t, A)
        t = clone_line(t, r"^\s+next\.%s = self\.%s\.commit_onto" % (A.snake, A.snake), A)
        t = clone_line(t, r"^\s+&& self\.%s\.is_empty\(\)" % A.snake, A)
        t = clone_line(t, r"^\s+%s: self\.%s\.(inverse\(|as_ref\(\)\.map)" % (A.snake, A.snake), A)
        return t
    edit(path, go)


def entry_span(line, key):
    i = line.find(key + ": ")
    if i < 0:
        return None
    depth, j = 0, i
    while j < len(line):
        c = line[j]
        if c in "([{":
            depth += 1
        elif c in ")]}":
            if depth == 0:
                break
            depth -= 1
        elif c == "," and depth == 0:
            break
        j += 1
    return i, j


def diff_ts(A, path, patch_fields=None):
    L, P = A.pascal, A.P
    pat_iface = "{ x: (number) | null; y: (number) | null; z: (number) | null; yaw: (number) | null; }"
    def go(t):
        if re.search(r"interface \w*PlacementDelta|interface \w*PlacementRows", t):
            return t
        lines = t.split("\n")
        out, i = [], 0
        names = rf"\w*{L}s?(?:Patch|Removal|Insertion|Relocation|Modification|Delta|Removed|Inserted|Moved|Modified|Rows)"
        while i < len(lines):
            l = lines[i]
            out.append(l)
            m = re.match(r"export interface (%s) \{(.*)$" % names, l)
            c = re.match(r"export const parse(%s): " % names, l)
            if m:
                if l.rstrip().endswith("}"):
                    new = A.rn(l)
                    if m.group(1).endswith("Patch"):
                        new = re.sub(r"\{.*\}$", pat_iface, new)
                    out.append(new)
                else:
                    j = i + 1
                    blk = [l]
                    while lines[j] != "}":
                        blk.append(lines[j])
                        j += 1
                    blk.append("}")
                    out.extend(blk[1:])
                    nb = [A.rn(b) for b in blk]
                    if m.group(1).endswith("Patch"):
                        nb = [nb[0]] + ["  %s: number | null;" % f for f in FIELDS] + ["}"]
                    if i + j + 1 < len(lines) and lines[j + 1] == "":
                        out.append("")
                        j += 1
                    out.extend(nb)
                    if j + 1 < len(lines) and lines[j + 1] == "":
                        pass
                    i = j
            elif c:
                new = A.rn(l)
                if c.group(1).endswith("Patch"):
                    body = ", ".join("%s: normWireDefault(normWireNullable(normWireNumber), () => null)" % f for f in FIELDS)
                    new = re.sub(r"\(\{.*\}\);$", "({ " + body + " });", new)
                out.append(new)
            i += 1
        t = "\n".join(out)
        t = clone_line(t, r"^\s+%s\??: \w*%ss?(Delta|Rows)\b" % (A.camel, L), A, fix=emoji_fix)
        def dp(m):
            line = m.group(0)
            if "placements:" in line:
                return line
            sp = entry_span(line, A.camel)
            return line[:sp[1]] + ", " + A.rn(line[sp[0]:sp[1]]) + line[sp[1]:]
        t = re.sub(r"^export const parse%sDiff: .*$" % P, dp, t, flags=re.M)
        return t
    edit(path, go)


def diff_ts_imports(A, path):
    def imp(m):
        line = m.group(0)
        if re.search(r"\bPlacement\b|%s|\b%s\b" % (re.escape(A.rn(A.P + A.model)), re.escape(A.new_model)), line):
            return line
        line = re.sub(r"(type (\w*%s),)" % A.model, lambda k: k.group(1) + " type %s," % A.rn(k.group(2)), line, count=1)
        line = re.sub(r"( (parse\w*%s),)" % A.model, lambda k: k.group(1) + " %s," % A.rn(k.group(2)), line, count=1)
        return line
    edit(path, lambda t: re.sub(r"^import \{[^\n]*\btype \w*%s\b[^\n]*$" % A.model, imp, t, flags=re.M))


def diff_graphql(A, path):
    L, P = A.pascal, A.P
    def go(t):
        if re.search(r"type \w*Placement(Delta|Rows)", t):
            return t
        def pred(p):
            return bool(re.match(r"type \w*%ss?(Patch|Removal|Insertion|Relocation|Modification|Delta|Removed|Inserted|Moved|Modified|Rows) \{" % L, p))
        def fix(n):
            if re.match(r"type \w*PlacementPatch \{", n):
                return n.split("{")[0] + "{\n" + "\n".join("  %s: Float" % f for f in FIELDS) + "\n}"
            return n
        if re.search(r"^\s+%s: JSON" % A.camel, t, re.M):
            return clone_line(t, r"^\s+%s: JSON" % A.camel, A, attrs=False)
        t = para_clone(t, A, pred, fix)
        return clone_line(t, r"^\s+%s: \[?\w*%ss?(Delta|Rows)\b" % (A.camel, L), A, attrs=False)
    edit(path, go)


def diff_proto(A, path):
    L = A.pascal
    def go(t):
        if re.search(r"message \w*Placement(Delta|Rows)", t):
            return t
        def pred(p):
            return bool(re.match(r"message \w*%ss?(Patch|Removal|Insertion|Relocation|Modification|Delta|Removed|Inserted|Moved|Modified|Rows) \{" % L, p))
        def fix(n):
            if re.match(r"message \w*PlacementPatch \{", n):
                return n.split("{")[0] + "{\n" + "\n".join("  optional double %s = %d;" % (f, k + 1) for k, f in enumerate(FIELDS)) + "\n}"
            return n
        if not re.search(r"(Delta|Rows)", t):
            return t
        t = para_clone(t, A, pred, fix)
        fm = re.search(r"^(\s+)(optional )?(\w*%ss?(?:Delta|Rows)) %s = (\d+);\n" % (L, A.snake), t, re.M)
        start = t.rfind("\nmessage ", 0, fm.start()) + 1
        end = t.find("\n}", fm.end() - 1)
        nums = [int(n) for n in re.findall(r"= (\d+);", t[start:end + 2])]
        line = "%s%s%s placements = %d;\n" % (fm.group(1), fm.group(2) or "", A.rn(fm.group(3)), max(nums) + 1)
        return t[:fm.end()] + line + t[fm.end():]
    edit(path, go)


def diff_json(A, path):
    L = A.pascal
    def go(t):
        data = json.loads(t)
        form, tail = json_form(t)
        dk = "$defs" if "$defs" in data else "definitions"
        defs = data[dk]
        if "placements" in data["properties"]:
            return t
        keys = [k for k in defs if re.search(r"%ss?(Patch|Removal|Insertion|Relocation|Modification|Delta|Removed|Inserted|Moved|Modified|Rows)$" % L, k)]
        nd = {}
        for k, v in defs.items():
            nd[k] = v
            if k == keys[-1]:
                for k2 in keys:
                    s = A.rn(json.dumps(defs[k2]))
                    v2 = json.loads(s)
                    if k2.endswith("Patch"):
                        old = defs[k2]
                        sample = next(iter(old["properties"].values()))
                        number = {"anyOf": [{"type": "number"}, {"type": "null"}]}
                        v2["properties"] = {f: number for f in FIELDS}
                        if "required" in v2:
                            v2["required"] = list(FIELDS)
                    nd[A.rn(k2)] = v2
        data[dk] = nd
        props = {}
        for k, v in data["properties"].items():
            props[k] = v
            if k == A.camel:
                props["placements"] = json.loads(A.rn(json.dumps(v)))
        data["properties"] = props
        if "required" in data and A.camel in data["required"]:
            data["required"].insert(data["required"].index(A.camel) + 1, "placements")
        return dumps_with(form, data) + tail
    edit(path, go)


def diff_layers(A):
    d = "/🧬️schema/🔺️diff"
    diff_rs(A, A.path(d + "/🦀️.rs"))
    diff_ts(A, A.path(d + "/🟦️.ts"))
    diff_ts_imports(A, A.path(d + "/🟦️.ts"))
    diff_graphql(A, A.path(d + "/🔗️.graphql"))
    diff_proto(A, A.path(d + "/🛰️.proto"))
    diff_json(A, A.path(d + "/🔣️.json"))


def value_end(t, i):
    """Index just past the JSON value starting at t[i]."""
    c = t[i]
    if c in "[{":
        depth, j, instr = 0, i, False
        while j < len(t):
            ch = t[j]
            if instr:
                if ch == "\\":
                    j += 1
                elif ch == '"':
                    instr = False
            elif ch == '"':
                instr = True
            elif ch in "[{":
                depth += 1
            elif ch in "]}":
                depth -= 1
                if depth == 0:
                    return j + 1
            j += 1
    m = re.compile(r'"(?:[^"\\]|\\.)*"|[^,}\]\s]+').match(t, i)
    return m.end()


EMPTY_DELTA = '{"removed": [], "inserted": [], "moved": [], "modified": []}'


def fixture_insert(t, A, key=None):
    key = key or A.camel
    pat = re.compile(r'"%s"\s*:\s*' % key)
    out, pos, hits = [], 0, 0
    for m in pat.finditer(t):
        if m.start() < pos:
            continue
        s = m.end()
        e = value_end(t, s)
        tail = t[e:e + 80]
        if re.match(r'\s*,\s*"placements"', tail):
            continue
        val = t[s:e]
        new = "[]" if val.lstrip().startswith("[") else (EMPTY_DELTA if val.lstrip().startswith("{") and ('"removed"' in val or val.strip() in ("{}",)) else None)
        if new is None:
            continue
        ls = t.rfind("\n", 0, m.start()) + 1
        indent = re.match(r"\s*", t[ls:m.start()]).group(0) if t[ls:m.start()].strip() == "" else None
        sep = (",\n" + indent) if indent is not None else ", "
        colon = ": " if ": " in t[m.start():s] else ":"
        out.append(t[pos:e])
        out.append('%s"placements"%s%s' % (sep, colon, new))
        pos = e
        hits += 1
    out.append(t[pos:])
    return "".join(out), hits


def fixtures(A, key=None, skip=("🧬️schema", "🎬️scene", "target", "node_modules")):
    n = 0
    for dp, ds, fs in os.walk(A.root):
        ds[:] = [d for d in ds if d not in skip]
        for f in fs:
            if not f.endswith(".json") or f == "🎲️field-metadata.json":
                continue
            p = os.path.join(dp, f)
            t = rd(p)
            if '"%s"' % (key or A.camel) not in t:
                continue
            nt, hits = fixture_insert(t, A, key)
            if hits:
                json.loads(nt)
                wr(p, nt)
                n += 1
    print("  fixtures updated:", n)


def sub1(t, pat, repl, flags=0, count=1):
    n = len(re.findall(pat, t, flags))
    assert n == count, (pat[:100], "expected", count, "found", n)
    return re.sub(pat, repl, t, flags=flags)


def sql_table(A, table, last_line_re=None):
    cols = ["id INTEGER PRIMARY KEY", "document_id INTEGER NOT NULL REFERENCES %s_document(id)" % A.lower, "ordinal INTEGER NOT NULL CHECK(ordinal>=0)", "logical_id TEXT NOT NULL", "x REAL", "y REAL", "z REAL", "yaw REAL"] + sum([["%s_ieee754_bits INTEGER NOT NULL" % f, "%s_ieee754_class TEXT NOT NULL" % f] for f in FIELDS], [])
    return "CREATE TABLE %s(%s);" % (table, ", ".join(cols))


def sqlite_extent(A, sql_path, semantic_json, ts_test, rs_tests, old_tables, table_name):
    """Recompute schemaBytes (sum of statement bytes + table-name bytes, the repo's check_schema formula) and widths."""
    def sb(text):
        e = 0
        for s in text.split(";"):
            s = s.strip()
            if s:
                e += len(s.encode()) + len(s[len("CREATE TABLE "):].split("(")[0].strip().encode())
        return e
    new_bytes = sb(rd(sql_path))
    def sj(t):
        d = json.loads(t)
        d["schemaBytes"] = new_bytes
        d["tableWidths"][table_name] = 16
        d["tableWidths"] = dict(sorted(d["tableWidths"].items()))
        return json.dumps(d, indent=2, ensure_ascii=False) + "\n"
    edit(semantic_json, sj)
    def tt(t):
        t = re.sub(r'(normSemanticContract\["schemaBytes"\]\)\.toEqual\()\d+', lambda m: m.group(1) + str(new_bytes), t)
        if '"%s":16' % table_name not in t:
            t = re.sub(r'("tableWidths"\]\)\.toEqual\(\{)([^}]*)\}', lambda m: m.group(1) + ",".join(sorted(m.group(2).split(",") + ['"%s":16' % table_name])) + "}", t)
        return t
    edit(ts_test, tt)
    return new_bytes


def literal_sweep(A, extra_re=None, skip=("🧬️schema/🔺️diff", "target", "node_modules")):
    n = 0
    rx = extra_re or r"^\s+%s: (vec!\[\]|Vec::new\(\)),?$" % A.snake
    for dp, ds, fs in os.walk(A.root):
        ds[:] = [d for d in ds if d != "target" and d != "node_modules"]
        if "🔺️diff" in dp:
            continue
        for f in fs:
            if f.endswith(".rs"):
                p = os.path.join(dp, f)
                t = rd(p)
                if re.search(rx, t, re.M):
                    nt = clone_line(t, rx, A, attrs=False)
                    if nt != t:
                        wr(p, nt)
                        n += 1
    print("  literal sweep files:", n)


def append_after_last(t, regex, new_lines):
    lines = t.split("\n")
    idx = [i for i, l in enumerate(lines) if re.search(regex, l)]
    assert idx, regex
    if new_lines[0] in lines:
        return t
    i = idx[-1]
    return "\n".join(lines[:i + 1] + new_lines + lines[i + 1:])


def field_meta_fieldspec(A, path, test_path=None):
    """FieldSpec-style registries (en1990)."""
    rows = [
        '    ("placements", FieldSpec::new("Placements", "Platzierungen")),',
        '    ("placements[].id", FieldSpec::new("Placed entity id", "Platzierte Element-Id")),',
        '    ("placements[].x", FieldSpec::quantity("Placement x", "Platzierung x", UnitId::Metre)),',
        '    ("placements[].y", FieldSpec::quantity("Placement y", "Platzierung y", UnitId::Metre)),',
        '    ("placements[].z", FieldSpec::quantity("Placement z", "Platzierung z", UnitId::Metre)),',
        '    ("placements[].yaw", FieldSpec::quantity("Placement yaw about Z", "Platzierung Gierwinkel um Z", UnitId::Radian)),',
    ]
    edit(path, lambda t: append_after_last(t, r'^\s+\("%s\[\]\.\w+", ' % A.camel, rows))
    if test_path:
        paths = ["placements", "placements[].id", "placements[].x", "placements[].y", "placements[].z", "placements[].yaw"]
        edit(test_path, lambda t: append_after_last(t, r'^\s+"%s\[\]\.\w+",' % A.camel, ['        "%s",' % p for p in paths]))


def core(A, root_ts=("/🟦️.ts", "/📸️snapshot/🟦️.ts"), twin_tok=None):
    s = "/🧬️schema"
    layer_model_struct(A, A.path(s + "/📸️snapshot/🦀️.rs"))
    edit(A.path(s + "/📸️snapshot/🦀️.rs"), lambda t: clone_line(t, r"^\s+(pub )?%s: (Vec<%s>,|vec!\[\],|Vec::new\(\),)" % (A.snake, A.model), A))
    edit(A.path(s + "/🦀️.rs"), lambda t: clone_line(t, r"^\s+(pub )?%s: " % A.snake, A))
    for f in root_ts:
        ts_iface(A, A.path(s + f))
        ts_root_field(A, A.path(s + f))
    for f in ("/🔗️.graphql", "/📸️snapshot/🔗️.graphql"):
        graphql_snapshot(A, A.path(s + f), twin_tok)
    for f in ("/🛰️.proto", "/📸️snapshot/🛰️.proto"):
        proto_snapshot(A, A.path(s + f), twin_tok)
    for f in ("/🔣️.json", "/📸️snapshot/🔣️.json"):
        json_snapshot(A, A.path(s + f))
    diff_layers(A)
    literal_sweep(A)
    fixtures(A)


def multiline_literal_sweep(A, field=None):
    """After every `field: vec![ ... ],` initialiser (multi-line) add `placements: vec![],`."""
    field = field or A.snake
    n = 0
    pat = re.compile(r"^(\s+)%s: (?:vec!)?\[" % field if False else r"^([ \t]+)%s: vec!\[" % field, re.M)
    for dp, ds, fs in os.walk(A.root):
        ds[:] = [d for d in ds if d not in ("target", "node_modules")]
        if "🔺️diff" in dp:
            continue
        for f in fs:
            if not f.endswith(".rs"):
                continue
            p = os.path.join(dp, f)
            t = rd(p)
            out, pos, hit = [], 0, False
            for m in pat.finditer(t):
                if m.start() < pos:
                    continue
                i = t.index("[", m.start())
                depth, j, instr = 0, i, False
                while True:
                    ch = t[j]
                    if instr:
                        if ch == "\\":
                            j += 1
                        elif ch == '"':
                            instr = False
                    elif ch == '"':
                        instr = True
                    elif ch == "[":
                        depth += 1
                    elif ch == "]":
                        depth -= 1
                        if depth == 0:
                            break
                    j += 1
                k = j + 1
                if t[k:k + 1] != ",":
                    continue
                k += 1
                if re.match(r"\s*placements:", t[k:k + 40]):
                    continue
                out.append(t[pos:k])
                out.append("\n%splacements: vec![]," % m.group(1))
                pos = k
                hit = True
            if hit:
                out.append(t[pos:])
                wr(p, "".join(out))
                n += 1
    print("  multiline literal sweep files:", n)
