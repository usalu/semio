"""Macro-diff template (`protocol::row_patch!` / `list_delta!`) for id-keyed artifacts: set-list rewrite, nested `nest` lanes cloned from a sibling lane, json/ts/graphql/proto twins that keep the committed layout."""
import json, re
from common import *
import diff as D


def rows_name(s):
    return s.full[:-5]


def clone_rust_lane(text, P, s, clone_stem, clone_row, model):
    stem, row = rows_name(s), s.row_def
    if "pub %sPatch of" % stem in text:
        return text
    key = re.search(r"/// 🔑️ Key extractor of the foreign `%s` rows[^\n]*\n.*?\nimpl [^\n]*\n.*?\n\}\n\n" % clone_row, text, re.S).group(0)
    patch = re.search(r"protocol::row_patch! \{\n[^\n]*\n[^\n]*\n[^\n]*\n    pub %sPatch of [^\n]*\n\}\n\n" % clone_stem, text).group(0)
    rows = re.search(r"protocol::list_delta! \{\n[^\n]*\n[^\n]*\n[^\n]*\n    pub %sRows \{[^\n]*\}\n\}\n\n" % clone_stem, text).group(0)
    blk = (key + patch + rows).replace(clone_stem, stem).replace(clone_row, row)
    blk = re.sub(r"set \{ [^}]* \}", "set {  }", blk, 1)
    blk = blk.replace("`layers`", "`%s`" % s.lane)
    parent = [p for p in s.parents][0]
    at = text.index("    pub %sPatch of" % parent.full[:-5])
    i = text.rfind("protocol::row_patch! {", 0, at)
    return text[:i] + blk + text[i:]


def patch_rust(text, P, by, model, clone_stem, clone_row):
    for s in by.values():
        if s.parents:
            text = clone_rust_lane(text, P, s, clone_stem, clone_row, model)
    text = text.replace("(scalar leaves; nested rows travel with the whole-row update)", "(typed leaves; nested rows through `nest` lanes)")
    for s in by.values():
        pat = re.compile(r"(pub %s of [\w:]+ \{ set \{ )([^}]*)( \})( nest \{[^}]*\})? \}" % s.full)
        m = pat.search(text)
        assert m, "patch macro missing: " + s.full
        sets = ", ".join("%s: %s" % (l.field, ("%s::%s" % (model, l.rust_ty)) if l.ty.startswith("enum:") else l.ty) for l in s.fields)
        nest = " nest { %s }" % ", ".join("%s: %s" % (n, r) for n, r in s.lanes) if s.lanes else ""
        text = text[: m.start()] + m.group(1) + sets + m.group(3) + nest + " }" + text[m.end():]
    return text


def pretty_defs(d):
    head = json.dumps({k: v for k, v in d.items() if k != "$defs"}, indent=2, ensure_ascii=False)
    body = ",\n".join("    %s: %s" % (json.dumps(k, ensure_ascii=False), json.dumps(v, ensure_ascii=False)) for k, v in d["$defs"].items())
    return head[:-2] + ',\n  "$defs": {\n' + body + "\n  }\n}\n"


def json_def(s, url):
    props, req = {}, []
    for l in s.fields:
        props[camel(l.field)] = {"anyOf": [{"$ref": "%s#/$defs/%s/properties/%s" % (url, l.owner_def, camel(l.path[-1]))}, {"type": "null"}]}
        req.append(camel(l.field))
    for n, r in s.lanes:
        props[camel(n)] = {"$ref": "#/$defs/%s" % r}
        req.append(camel(n))
    return {"type": "object", "additionalProperties": False, "required": req, "properties": props}


def patch_json(text, by, url, clone_stem, clone_row):
    d = json.loads(text)
    defs = d["$defs"]
    out = {}
    for k, v in defs.items():
        out[k] = v
    for s in by.values():
        stem = rows_name(s)
        if stem + "Rows" not in out:
            for suffix in ("Inserted", "Modified", "Moved", "Removed", "Rows"):
                out[stem + suffix] = json.loads(json.dumps(out[clone_stem + suffix]).replace(clone_stem, stem).replace("/$defs/%s\"" % clone_row, "/$defs/%s\"" % s.row_def))
        out[s.full] = json_def(s, url)
    d["$defs"] = dict(sorted(out.items()))
    return pretty_defs(d)


def ts_patch(s):
    iface, parser = D.ts_patch(s)
    for n, r in s.lanes:
        iface = iface.replace("  %s: %s | null;\n" % (camel(n), r), "  %s: %s;\n" % (camel(n), r))
        parser = parser.replace("%s: normWireRequired(normWireNullable(normWireRef(() => parse%s)))" % (camel(n), r), "%s: normWireRequired(normWireRef(() => parse%s))" % (camel(n), r))
    return iface, parser


def patch_ts(text, by, clone_stem, clone_row):
    for s in by.values():
        stem = rows_name(s)
        if "export interface %sRows " % stem not in text:
            ifaces, parsers = "", ""
            for suffix in ("Inserted", "Modified", "Moved", "Patch", "Removed", "Rows"):
                if suffix == "Patch":
                    continue
                m = re.search(r"export interface %s%s \{.*?\n\}\n\n" % (clone_stem, suffix), text, re.S)
                ifaces += m.group(0).replace(clone_stem, stem).replace(clone_row, s.row_def)
                parsers += re.search(r"export const parse%s%s: [^\n]*\n" % (clone_stem, suffix), text).group(0).replace(clone_stem, stem).replace("parse" + clone_row, "parse" + s.row_def).replace(clone_row, s.row_def)
            ifaces += "export interface %s {\n}\n\n" % s.full
            parsers += "export const parse%s: NormWireReader<%s> = normWireObject<%s>({  });\n" % (s.full, s.full, s.full)
            parent = s.parents[0].full[:-5]
            i = text.index("export interface %sInserted {" % parent)
            text = text[:i] + ifaces + text[i:]
            j = text.index("export const parse%sInserted:" % parent)
            text = text[:j] + parsers + text[j:]
            line = next(l for l in text.split("\n")[:8] if l.startswith("import { type Anchor"))
            text = text.replace(line, line.replace(" parseAnchor,", " parseAnchor, parse%s, type %s," % (s.row_def, s.row_def), 1), 1)
    for s in by.values():
        iface, parser = ts_patch(s)
        pi = re.compile(r"export interface %s \{.*?\n\}\n" % s.full, re.S)
        pp = re.compile(r"export const parse%s: [^\n]*\n" % s.full)
        assert pi.search(text) and pp.search(text), s.full
        text = pi.sub(lambda m: iface, text, 1)
        text = pp.sub(lambda m: parser, text, 1)
    return text


def gql_patch(s):
    out = D.gql_patch(s)
    for n, r in s.lanes:
        out = out.replace("  %s: %s\n" % (camel(n), r), "  %s: %s!\n" % (camel(n), r))
    return out


def proto_patch(s):
    out = D.proto_patch(s)
    for n, r in s.lanes:
        out = out.replace("  optional %s %s = " % (r, n), "  %s %s = " % (r, n))
    return out


def patch_blocks(text, by, kind, clone_stem, clone_row):
    kw = "type" if kind == "gql" else "message"
    for s in by.values():
        stem = rows_name(s)
        if "%s %sRows {" % (kw, stem) not in text:
            parts = ""
            for suffix in ("Removed", "Inserted", "Moved", "Modified", "Rows"):
                m = re.search(r"%s %s%s \{.*?\n\}\n\n" % (kw, clone_stem, suffix), text, re.S)
                parts += m.group(0).replace(clone_stem, stem).replace(clone_row, s.row_def)
            parent = s.parents[0].full
            i = text.index("%s %s {" % (kw, parent))
            text = text[:i] + "%s %s {\n}\n\n" % (kw, s.full) + parts + text[i:]
    for s in by.values():
        blk = gql_patch(s) if kind == "gql" else proto_patch(s)
        head = "type %s {" % s.full if kind == "gql" else "message %s {" % s.full
        pat = re.compile(re.escape(head) + r".*?\n\}\n", re.S)
        assert pat.search(text), s.full
        text = pat.sub(lambda m: blk, text, 1)
    return text
