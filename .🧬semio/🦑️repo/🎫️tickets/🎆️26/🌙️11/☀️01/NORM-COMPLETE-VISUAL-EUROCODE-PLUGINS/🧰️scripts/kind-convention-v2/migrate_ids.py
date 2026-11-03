"""🆔 Index selectors to id selectors for the set, insert and remove kinds of one norm artifact.

`python -I migrate_ids.py <en1991|en1996|en1998> [--apply]` reads the artifact's edit rules, regenerates the `diff` and `inverse` leaves of every kind that
still addresses a list row by position (`index`, `wallIndex`, ...) from the `index_selector` templates, patches the payload struct, the schema json and ts twins
and the edit rules in place, and rewrites the mutation fixtures (a position becomes the row id read from the scenario's before snapshot). Rows are addressed by
`<parent>Id` and a last link `id`; removal is by id only (`IdRemoval` of the shared edit rules); `index` stays the insert position. The diff lanes
(`list_delta!`, `row_patch!`) of these artifacts are id-keyed already and untouched. Stage: `🗑️generated/kind-convention-v2/stage/<artifact>`; a staged file is applied
only when the target did not change since it was read. Idempotent: a second run finds no index selector.

📎 [leaf-kinds templates](../leaf-kinds/index_selector.py) - [shared id removals](../../../../../../../../✏️s/🔌️plugins/📕️norm/📇️registry/🧬️contract/🖥️app-surface/✏️edit-rules/🦀️.rs)
"""
import os as _o, sys as _s; _o.environ.get("TH_FORCE") or (_s.stderr.write("unsafe against the current tree (destructive or not idempotent), refusing to run (TH_FORCE=1 forces)\n"), _s.exit(2))  # unsafe-guard
import copy, glob, hashlib, json, os, re, shutil, sys

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, os.path.join(os.path.dirname(HERE), "leaf-kinds"))
from common import *
import index_selector as I

POSITIONAL = {"en1994"}
EMOJI = {"en1991": "🏋️", "en1994": "🧩️", "en1996": "🪨️", "en1998": "🫨️", "en1999": "🪶️"}
RULE = re.compile(r'(?P<ty>SetFieldRule|InsertItemRule|RemoveItemRule|IdRemoval) \{ path: "(?P<path>[^"]+)", kind: "(?P<kind>[^"]+)", selectors: &\[(?P<sel>[^\]]*)\](?P<rest>[^}]*)\}')
SEL = re.compile(r'SelectorKey::(Index|Id)\("([^"]+)"\)|SelectorKey::(Implied)')
PRIMITIVE = {"f64", "u32", "u8", "bool", "String"}
DESIGN_CASE_PATHS = {"cases", "cases[].numbers", "cases[].texts"}


def sha(text):
    return hashlib.sha256(text.encode()).hexdigest()


class Stage:
    """🗂️ Staged writer with an optimistic-concurrency check: a path is applied only when its content equals what was read."""

    def __init__(self, root):
        self.root, self.seen, self.written = root, {}, {}

    def read(self, path):
        if path in self.written:
            return self.written[path]
        text = rd(path)
        self.seen.setdefault(path, sha(text))
        return text

    def write(self, path, text):
        assert len(path[len(SEMIO) + 1:].encode()) < 256, "path of 256 bytes or more: " + path
        if path not in self.seen:
            self.seen[path] = sha(rd(path)) if os.path.exists(path) else None
        self.written[path] = text
        target = self.root + path
        os.makedirs(os.path.dirname(target), exist_ok=True)
        open(target, "w", encoding="utf-8").write(text)

    def apply(self, gate=None):
        changed, conflicts = [], []
        if gate in self.written and (sha(rd(gate)) if os.path.exists(gate) else None) != self.seen[gate]:
            return [], [gate]
        for path, text in sorted(self.written.items()):
            current = sha(rd(path)) if os.path.exists(path) else None
            if current != self.seen[path]:
                conflicts.append(path)
            elif current != sha(text):
                os.makedirs(os.path.dirname(path), exist_ok=True)
                open(path, "w", encoding="utf-8").write(text)
                changed.append(path)
        return changed, conflicts


def words(name):
    return " ".join(re.sub(r"([A-Z])", r" \1", name).lower().split())


def kind_folder(root, kind):
    found = [d for d in glob.glob(root + "/*") if os.path.isdir(d) and os.path.basename(d).endswith(kind) and not re.search(r"[a-z0-9]", os.path.basename(d)[: -len(kind)])]
    assert len(found) == 1, (kind, found)
    return found[0]


class Rule:
    def __init__(self, match):
        self.ty, self.path, self.kind, self.rest, self.line = match.group("ty"), match.group("path"), match.group("kind"), match.group("rest"), match.group(0)
        self.sels = [("Implied", None) if m.group(3) else (m.group(1), m.group(2)) for m in SEL.finditer(match.group("sel"))]

    @property
    def colls(self):
        parts = self.path.split("[]")
        parts = parts[:-1] if self.ty == "SetFieldRule" else parts
        return [part.lstrip(".").split(".")[-1] for part in parts]

    def field(self, name):
        found = re.search(r'%s: "([^"]+)"' % name, self.rest)
        return found.group(1) if found else None

    @property
    def has_index(self):
        return any(mode == "Index" for mode, _ in self.sels)


class Artifact:
    def __init__(self, short, stage):
        self.short, self.stage = short, stage
        self.name = EMOJI[short] + short
        self.P = short.replace("en", "En", 1)
        self.any = NORM + "/🗿️artifacts/" + self.name + "/🏅️standards/🔖️1/🪆️subsets/✳️any"
        self.mut = self.any + "/🧬️schema/🧬️mutations"
        self.fix = self.any + "/🧪️tests/🧫️fixtures/🧬️mutations"
        self.rules_path = self.mut + "/🧭️edit-rules/🦀️.rs"
        self.model = "semio_s_artifact_norm_%s_model" % short
        self.diff_crate = "semio_s_artifact_norm_%s_diff" % short
        self.snap = self.P + "Snapshot"
        self.positional = short in POSITIONAL
        if not self.positional:
            self.read_lanes(self.any + "/🧬️schema/🔺️diff")

    def read_lanes(self, diff_dir):
        text = "".join(rd(p) for p in sorted(glob.glob(diff_dir + "/**/🦀️.rs", recursive=True)) if "/🧪️tests/" not in p)
        self.delta_patch = {m.group(1): m.group(2) for m in re.finditer(r"pub (\w+) \{ removal: \w+, insertion: \w+, relocation: \w+, modification: \w+, row: [\w:]+, patch: (\w+),", text)}
        self.sets, self.nests = {}, {}
        for m in re.finditer(r"pub (\w+Patch) of [\w:]+ \{ set \{(?P<set>[^}]*)\}(?: nest \{(?P<nest>[^}]*)\})? \}", text):
            self.sets[m.group(1)] = [x.split(":")[0].strip() for x in m.group("set").split(",") if x.strip()]
            self.nests[m.group(1)] = {x.split(":")[0].strip(): x.split(":")[1].strip() for x in (m.group("nest") or "").split(",") if x.strip()}
        root = re.search(r"pub struct %sDiff \{(.*?)\n\}" % self.P, text, re.S).group(1)
        self.root_fields = {m.group(1): m.group(2) for m in re.finditer(r"pub (\w+): (\w+Delta),", root)}

    def lanes(self, colls):
        delta = self.root_fields[snake(colls[0])]
        out = [(delta, self.delta_patch[delta])]
        for coll in colls[1:]:
            delta = self.nests[out[-1][1]][snake(coll)]
            out.append((delta, self.delta_patch[delta]))
        return out

    def single(self, coll):
        if self.positional:
            return camel(snake(coll)[:-1] if snake(coll).endswith("s") else snake(coll))
        delta = self.root_fields[snake(coll)] if snake(coll) in self.root_fields else None
        if delta is None:
            for nest in self.nests.values():
                if snake(coll) in nest:
                    delta = nest[snake(coll)]
        if delta is None:
            raise KeyError("no diff lane for " + coll)
        return camel(re.sub(r"(?<!^)([A-Z])", r"_\1", delta[len(self.P):-5]).lower())

    def rules(self):
        text = self.stage.read(self.rules_path)
        return text, [Rule(m) for m in RULE.finditer(text)]


class Lanes(I.Style):
    """🛣️ Diff lane names read from the diff crate: a chain of collection fields maps to its `Delta` and `Patch` types; nest and top lane are bare."""

    def __init__(self, art, chains, opt_index):
        super().__init__(art.P, art.model, art.diff_crate, art.snap, "", mode="single", opt_index=opt_index, tagged=False)
        self.chains = chains

    def rows(self, links):
        return self.chains[tuple(l.coll for l in links)][0]

    def patch(self, links):
        return self.chains[tuple(l.coll for l in links)][1]

    def nest(self, expr):
        return expr


def selector_names(art, rule):
    """🏷️ `(old, new)` per index selector: `<parent>Id` for the parents of the chain, `id` for the addressed row of a set rule."""
    colls = rule.colls
    out = []
    for position, (mode, name) in enumerate(rule.sels):
        if mode != "Index":
            continue
        is_row = rule.ty == "SetFieldRule" and position == len(colls) - 1
        base = name[:-5] if name.endswith("Index") and name != "index" else art.single(colls[position])
        out.append((name, "id" if is_row else base + "Id"))
    return out


def struct_of(folder):
    text = rd(folder + "/🦀️.rs")
    m = re.search(r"pub struct (\w+) \{(.*?)\n\}", text, re.S)
    return m.group(1), re.findall(r"pub (\w+): ([^,\n]+),", m.group(2))


def entity(art, rule, emoji, names):
    colls, renamed, links = rule.colls, dict(names), []
    for position, coll in enumerate(colls):
        label = words(art.single(coll))
        if position == len(colls) - 1:
            sel = "id"
        else:
            mode, name = rule.sels[position]
            sel = snake(renamed.get(name, name))
        links.append(I.Link(snake(coll), "", sel, snake(art.single(coll)), label, label))
    return I.Ent(rule.kind, emoji, links, links[-1].en, links[-1].de)


def chains_of(art, ent, rule):
    lanes = art.lanes(rule.colls)
    return {tuple(l.coll for l in ent.links[: i + 1]): lanes[i] for i in range(len(lanes))}


def leaf_of(art, rule, ent, record, fields, lanes):
    wanted = rule.path.split("[]")[-1].lstrip(".").replace(".", "").replace("_", "").lower()
    hit = [f for f in art.sets[lanes[-1][1]] if f.replace("_", "").lower() == wanted]
    assert len(hit) == 1, (rule.kind, wanted, hit)
    wanted_new = rule.field("value").replace("_", "").lower()
    new = [name for name, _ in fields if name.replace("_", "").lower() == wanted_new][0]
    ty = dict(fields)[new]
    leaf = I.Leaf(ent, [hit[0]], ty if ty in PRIMITIVE else "other", {}, kind=rule.kind)
    leaf.field = leaf.access = hit[0]
    leaf.new, leaf.module, leaf.record = new, rule.kind.replace("-", "_"), record
    return leaf


def keep_guard(old_diff, new_diff, new_field):
    guard = re.search(r"    if !payload\.%s\.is_finite\(\) \|\|[^\n{]*\{\n[^\n]*\n    \}\n" % new_field, old_diff)
    if guard and "index" not in guard.group(0):
        return re.sub(r"    if !payload\.%s\.is_finite\(\) \{\n[^\n]*\n    \}\n" % new_field, lambda m: guard.group(0), new_diff, 1)
    return new_diff


def set_kind(art, rule, emoji, names, folder, record, fields):
    ent = entity(art, rule, emoji, names)
    lanes = art.lanes(rule.colls)
    style = Lanes(art, chains_of(art, ent, rule), False)
    leaf = leaf_of(art, rule, ent, record, fields, lanes)
    kind = I.SetKind(leaf)
    kind.module, kind.record = leaf.module, record
    old = rd(folder + "/🔺️diff/🦀️.rs")
    return keep_guard(old, kind.diff_rs(style), leaf.new), kind.inverse_rs(style)


def pair_kinds(art, insert, remove, emoji, names, folders, structs):
    ent = entity(art, remove, emoji[0], names)
    style_for = lambda opt: Lanes(art, chains_of(art, ent, remove), opt)
    ins_record, ins_fields = structs["insert"]
    rem_record, rem_fields = structs["remove"]
    opt = dict(ins_fields).get("index", "").startswith("Option")
    style = style_for(opt)
    rem = I.RemoveKind(ent, remove.kind, style, False)
    ins = I.InsertKind(ent, insert.kind, rem, style)
    rem.module, rem.record, ins.module, ins.record = remove.kind.replace("-", "_"), rem_record, insert.kind.replace("-", "_"), ins_record
    rem.inserter, rem.inverse_of, ins.inverse_of = ins, ins, rem
    ins.item = snake(insert.field("item"))
    rem.sel_field = "id"
    ent.emoji = emoji[0]
    ins_code = (ins.diff_rs(style), ins.inverse_rs(style))
    ent.emoji = emoji[1]
    return ins_code, (rem.diff_rs(style), rem.inverse_rs(style))


def positional_text(art, rule, folder, role, renames, item=None):
    """🧭 The `diff` and `inverse` of a kind whose diff lane is still positional (`En1994BeamsRows`, patches carry the base position `index`): the payload carries
    ids, a lookup at the top of each function turns them into the base positions the lane needs, every other use of the old position fields follows."""
    colls = [snake(c) for c in rule.colls]
    news = [snake(n) for _, n, _ in renames]
    olds = [snake(o) for o, _, _ in renames]
    names = ["index", "action_index"][: len(news)]
    if role == "insert":
        news, olds, names = [], [], []
    texts = []
    for sub, fail in (("🔺️diff", None), ("↩️inverse", None)):
        text = rd(folder + "/" + sub + "/🦀️.rs")
        closure = "Ok((|| {" in text
        lines, parent = [], "base"
        for position, (coll, new, name) in enumerate(zip(colls, news, names)):
            if sub == "🔺️diff":
                err = 'return protocol::MutationOutcome::error("mutation.target-missing", format!("%s {} does not exist.", payload.%s), [payload.%s.clone()]);' % (words(art.single(rule.colls[position])), new, new)
            else:
                err = "return Vec::new();" if closure else "return Ok(Vec::new());"
            lines.append("    let Some(%s) = %s.%s.iter().position(|row| row.id == payload.%s) else {\n        %s\n    };\n" % (name, parent, coll, new, err))
            parent = "base.%s[%s]" % (coll, name)
        marker = re.search(r"pub fn (diff|inverse)\b[^\n]*\{\n(    Ok\(\(\|\| \{\n)?", text)
        lookup = "".join(lines)
        if marker.group(2):
            lookup = "".join("    " + l if i % 3 == 0 and False else l for i, l in enumerate(lookup.splitlines(True)))
        text = text[: marker.end()] + lookup + text[marker.end():]
        if role in ("set", "remove"):
            def literal(match):
                body = match.group(2)
                for (old, new) in zip(olds, news):
                    body = re.sub(r"\b%s: payload\.%s\b" % (old, old), "%s: payload.%s.clone()" % (new, new), body)
                return "%s {%s}" % (match.group(1), body)
            text = re.sub(r"\b((?:Change|Remove)[A-Za-z0-9]+) \{([^{}]*)\}", literal, text)
            for old, name in sorted(zip(olds, names), key=lambda x: -len(x[0])):
                text = re.sub(r"payload\.%s\b(?!\.clone\(\))" % old, name, text)
        else:
            text = re.sub(r"\b(Remove[A-Za-z0-9]+) \{ index: payload\.index \}", lambda m: "%s { id: payload.%s.id.clone() }" % (m.group(1), item), text)
        texts.append(text)
    return texts[0], texts[1]


def rename_struct(text, renames):
    for old, new in renames:
        text, n = re.subn(r"pub %s: usize," % re.escape(snake(old)), "pub %s: String," % snake(new), text)
        assert n == 1, (old, new)
    return text


def id_prop(old, new, label, single):
    ui = copy.deepcopy(old.get("x-semio-ui", {}))
    text = ui.get("label") or label
    return {"type": "string", "x-semio-ui": {"widget": "reference", "role": "target", "label": text, "description": {"en": "The %s this mutation addresses, by id." % text["en"].lower(), "de": "Das adressierte Element (%s), per Kennung." % text["de"]}, "ref": {"kind": kebab(snake(single))}, "group": ui.get("group", "target"), "order": ui.get("order", 10)}}


def rename_schema(text, renames, labels):
    doc = json.loads(text)
    props = {}
    for key, value in doc["properties"].items():
        hit = [(o, n, s) for o, n, s in renames if o == key]
        props[hit[0][1] if hit else key] = id_prop(value, hit[0][1], labels[hit[0][0]], hit[0][2]) if hit else value
    doc["properties"] = props
    doc["required"] = [next((n for o, n, _ in renames if o == r), r) for r in doc["required"]]
    return dumps(doc)


def rename_ts(text, renames):
    for old, new, _ in renames:
        text, a = re.subn(r"(\n  )%s: number;" % re.escape(old), r"\g<1>%s: string;" % new, text)
        text, b = re.subn(r'%s: normWireRequired\(normWireRange\(normWireInteger, \{"minimum":0\}\)\)' % re.escape(old), "%s: normWireRequired(normWireString)" % new, text)
        assert a == 1 and b == 1, (old, new, a, b)
    imp = re.search(r'import \{ ([^}]*) \} from "@semio-tech/norm-js/registry/contract";', text)
    names = {x.strip() for x in imp.group(1).split(",")} | {"normWireString"}
    body = text.replace(imp.group(0), "")
    names -= {n for n in ("normWireInteger", "normWireRange") if n not in body}
    return text.replace(imp.group(0), 'import { %s } from "@semio-tech/norm-js/registry/contract";' % ", ".join(sorted(names, key=lambda x: x.replace("type ", ""))))


def ids_in(snapshot, colls, positions):
    node, out = snapshot, []
    for coll, position in zip(colls, positions):
        items = node.get(coll) if isinstance(node, dict) else None
        if not isinstance(items, list) or not isinstance(position, int) or position >= len(items) or "id" not in items[position]:
            out.append("missing-%s" % position)
            node = None
        else:
            out.append(items[position]["id"])
            node = items[position]
    return out


def rewrite_fixtures(art, stage, kind, renames, colls, positions):
    count = 0
    for folder in glob.glob(art.fix + "/*"):
        if not (os.path.basename(folder).endswith(kind) and not re.search(r"[a-z0-9]", os.path.basename(folder)[: -len(kind)])):
            continue
        for scenario in glob.glob(folder + "/*"):
            mutation, before = scenario + "/🔣️.mutation.json", scenario + "/🔣️.before.json"
            if not (os.path.exists(mutation) and os.path.exists(before)):
                continue
            wire = json.loads(stage.read(mutation))
            flat = "mutation" in wire
            payload = wire if flat else wire[next(iter(wire))]
            if not any(old in payload for old, _, _ in renames):
                continue
            snapshot = json.loads(rd(before))
            ids = dict(zip([o for o, _, _ in renames], ids_in(snapshot, colls, [payload.get(o) for o in positions])))
            fresh = {}
            for key, value in payload.items():
                hit = [n for o, n, _ in renames if o == key]
                fresh[hit[0] if hit else key] = ids[key] if hit else value
            stage.write(mutation, dumps(fresh if flat else {next(iter(wire)): fresh}))
            count += 1
    return count


def finish(art, stage, folder, kind, diff, inverse, renames, labels, colls, positions, report, what):
    stage.write(folder + "/🔺️diff/🦀️.rs", diff)
    stage.write(folder + "/↩️inverse/🦀️.rs", inverse)
    stage.write(folder + "/🦀️.rs", rename_struct(stage.read(folder + "/🦀️.rs"), [(o, n) for o, n, _ in renames]))
    stage.write(folder + "/🧬️schema/🔣️.json", rename_schema(stage.read(folder + "/🧬️schema/🔣️.json"), renames, labels))
    stage.write(folder + "/🧬️schema/🟦️.ts", rename_ts(stage.read(folder + "/🧬️schema/🟦️.ts"), renames))
    report["fixtures"] += rewrite_fixtures(art, stage, kind, renames, colls, positions)
    report[what] += 1


def label_book(art, rules):
    book = {}
    for rule in rules:
        if rule.ty != "SetFieldRule" or not rule.has_index:
            continue
        folder = kind_folder(art.mut, rule.kind)
        doc = json.loads(rd(folder + "/🧬️schema/🔣️.json"))
        for position, (mode, name) in enumerate(rule.sels):
            label = (doc["properties"].get(name) or {}).get("x-semio-ui", {}).get("label") if mode == "Index" else None
            if label:
                book.setdefault(tuple(rule.colls[: position + 1]), label)
    return book


def label_for(book, art, colls, upto):
    return book.get(tuple(colls[: upto + 1])) or {"en": words(art.single(colls[upto])).capitalize(), "de": words(art.single(colls[upto])).capitalize()}


def new_line(rule, names):
    line = rule.line
    for old, new in names:
        line = line.replace('SelectorKey::Index("%s")' % old, 'SelectorKey::Id("%s")' % new, 1)
    return line


def removal_line(rule, names):
    return new_line(rule, names).replace("RemoveItemRule {", "IdRemoval {").replace('index: "%s"' % rule.field("index"), 'id: "id"')


def edit_rules_text(text, changes, removals):
    for old, new in changes:
        assert old in text, old[:120]
        text = text.replace(old, new, 1)
    for rule in removals:
        assert rule.line in text, rule.line[:120]
        text = re.sub(r"[ \t]*" + re.escape(rule.line) + r",?\n", "", text, count=1)
    return text


def adopt_shared(art, text, entries):
    """🔁 The family's `ID_REMOVALS` and `resolve_edit` on the shared `IdRemoval`: existing entries gain `selectors`, the local struct goes, an existing `resolve_edit` stays."""
    local = re.search(r"/// 🆔️ A collection whose remove kind addresses the row by its id instead of its position\.\n#\[derive\(Clone, Copy, Debug\)\]\npub struct IdRemoval \{.*?\n\}\n\n", text, re.S)
    if local:
        text = text.replace(local.group(0), "")
    block = re.search(r"((?:/// [^\n]*\n)*)pub const ID_REMOVALS: &\[IdRemoval\] = &\[\n(.*?)\];\n\n?", text, re.S)
    kept = []
    if block:
        kept = [(e if "selectors" in e else e.replace(", id:", ", selectors: &[], id:")) for e in block.group(2).split("\n") if e.strip()]
        text = text.replace(block.group(0), "")
    has_resolve = re.search(r"pub fn resolve_edit\(", text) is not None
    if not has_resolve:
        resolve = re.search(r"/// 🎯️ Resolves an editor edit into concrete kind mutations[^\n]*\npub fn resolve_edit\(.*?\n\}\n", text, re.S)
        if resolve:
            text = text.replace(resolve.group(0), "")
    rows = list(dict.fromkeys(kept + ["        " + e + "," for e in entries]))
    doc = block.group(1) if block and block.group(1) else "/// 🆔️ Every collection of this family removed by row id, never by position.\n"
    marker = re.search(r"\n(const |fn |/// 🔀️|/// 🎯️)", text[text.index("pub const EDIT_RULES"):])
    cut = text.index("pub const EDIT_RULES") + marker.start() + 1 if marker else len(text)
    text = text[:cut].rstrip("\n") + "\n" + doc + "pub const ID_REMOVALS: &[IdRemoval] = &[\n" + "\n".join(rows) + "\n];\n\n" + text[cut:].lstrip("\n")
    if not has_resolve:
        text = text.rstrip("\n") + "\n\n/// 🎯️ Resolves an editor edit into concrete kind mutations: row-id removals by [`ID_REMOVALS`], every other path through [`EDIT_RULES`].\npub fn resolve_edit(document: &%s, edit: &NormEdit) -> Result<Vec<%sMutation>, String> {\n    EDIT_RULES.resolve_with_id_removals(ID_REMOVALS, document, edit)\n}\n" % (art.snap, art.P)
    head = re.search(r"use semio_s_plugin_norm_app_surface::\{([^}]*)\};", text)
    names = {x.strip() for x in head.group(1).split(",")} | {"IdRemoval", "NormEdit"}
    text = text.replace(head.group(0), "use semio_s_plugin_norm_app_surface::{%s};" % ", ".join(sorted(names, key=lambda x: (x[:1].islower(), x))), 1)
    for needed in ("use crate::%sMutation;" % art.P, "use %s::%s;" % (art.model, art.snap)):
        if needed not in text:
            text = text.replace("use semio_s_plugin_norm_app_surface::{", needed + "\nuse semio_s_plugin_norm_app_surface::{", 1)
    return text


def current_structs(art):
    out = {}
    for path in glob.glob(art.mut + "/*/🦀️.rs"):
        match = re.search(r"pub struct (\w+) \{(.*?)\n\}", rd(path), re.S)
        if match and "MutationLeaf" in rd(path):
            out[match.group(1)] = re.findall(r"pub (\w+): ([^,\n]+),", match.group(2))
    return out


def renamed_fields(old_names, structs_fields):
    """🔁 `(old, new)` pairs where the facet or codec still spells a position field the payload struct now spells as an id."""
    if len(old_names) != len(structs_fields):
        return []
    return [(old, new) for old, (new, ty) in zip(old_names, structs_fields) if old != new and ty == "String"]


def sync_proto(text, structs):
    for match in list(re.finditer(r"message (\w+) \{([^}]*)\}", text)):
        record, body = match.group(1), match.group(2)
        if record not in structs:
            continue
        fields = re.findall(r"(?:optional )?\w+ (\w+) = \d+;", body)
        fresh = body
        for old, new in renamed_fields(fields, structs[record]):
            fresh = re.sub(r"\buint32 %s\b" % re.escape(old), "string %s" % new, fresh, count=1)
        text = text.replace(match.group(0), match.group(0).replace(body, fresh), 1)
    return text


def sync_graphql(text, structs):
    for match in list(re.finditer(r"type (\w+) \{([^}]*)\}", text)):
        record, body = match.group(1), match.group(2)
        if record not in structs:
            continue
        fields = re.findall(r"(\w+): \w+!?", body)
        fresh = body
        for old, new in renamed_fields(fields, [(camel(n), ty) for n, ty in structs[record]]):
            fresh = re.sub(r"\b%s: Int(!?)" % re.escape(old), r"%s: String\1" % new, fresh, count=1)
        text = text.replace(match.group(0), match.group(0).replace(body, fresh), 1)
    return text


def chains_by_record(art, rules, structs):
    by_kind = {r.kind: r for r in rules}
    out = {}
    for path in glob.glob(art.mut + "/*/🦀️.rs"):
        text = rd(path)
        match = re.search(r"pub struct (\w+) \{", text)
        kind = re.search(r'kind: "([^"]+)"', text)
        if match and kind and kind.group(1) in by_kind:
            out[match.group(1)] = by_kind[kind.group(1)]
    return out


def sync_text_codec(art, text, structs, chains):
    for record, fields in structs.items():
        kind = kebab(snake(record)).lstrip("-")
        encode = re.search(r"^(\s*%sMutation::%s\(p\) => format!\(.*)$" % (art.P, record), text, re.M)
        decode = re.search(r"^(\s*\"[a-z0-9-]+\" => Ok\(%sMutation::%s\(%s \{.*)$" % (art.P, record, record), text, re.M)
        if not encode or not decode:
            continue
        used = re.findall(r"enc_json\(&p\.(\w+)\)", encode.group(1))
        pairs = renamed_fields(used, [f for f in fields if f[0] in used or f[1] == "String" or True][: len(used)]) if len(used) <= len(fields) else []
        line_e, line_d = encode.group(1), decode.group(1)
        for old, new in pairs:
            line_e = line_e.replace("%s={}" % kebab(old), "%s={}" % kebab(new), 1).replace("enc_json(&p.%s)" % old, "enc_json(&p.%s)" % new, 1)
            line_d = line_d.replace('%s: dec_json(&arg("%s")?)?' % (old, kebab(old)), '%s: dec_json(&arg("%s")?)?' % (new, kebab(new)), 1)
        text = text.replace(encode.group(1), line_e, 1).replace(decode.group(1), line_d, 1)
        rule = chains.get(record)
        demo_start = text.find("pub fn demo_mutation_cases")
        if rule is None or demo_start < 0:
            continue
        demo = re.search(r"%sMutation::%s\(%s \{(.*?)\}\)" % (art.P, record, record), text[demo_start:], re.S)
        if not demo:
            continue
        names = re.findall(r"\b(\w+): (\d+)usize", demo.group(1))
        stale = [(n, v) for n, v in names if n not in dict(fields)]
        if not stale:
            continue
        sels = [s for s in dict(fields) if s.endswith("_id") or s == "id"]
        colls = rule.colls
        body = demo.group(1)
        chain_values = [v for _, v in stale]
        for (old, value), new in zip(stale, [s for s in sels]):
            depth = sels.index(new)
            expr = 'String::from("%s-%d")' % (kebab(snake(colls[depth])).rstrip("s"), int(chain_values[depth]) + 1)
            body = re.sub(r"\b%s: %susize" % (re.escape(old), re.escape(value)), "%s: %s" % (new, expr), body, count=1)
        text = text.replace(demo.group(0), demo.group(0).replace(demo.group(1), body), 1)
    return text


def sync_facets(art, stage, rules, report):
    """🧬 Facet and codec texts that spell payload fields follow the payload structs: proto, graphql and the explicit text codec."""
    structs = current_structs(art)
    for path, fn in ((art.mut + "/🛰️.proto", sync_proto), (art.mut + "/🔗️.graphql", sync_graphql)):
        if os.path.exists(path):
            text = stage.read(path)
            fresh = fn(text, structs)
            if fresh != text:
                stage.write(path, fresh)
                report["facets"] += 1
    codec = art.any + "/🚪️io/📝️text/🧬️mutations/🦀️.rs"
    if os.path.exists(codec) and "enc_json(&p." in rd(codec):
        text = stage.read(codec)
        fresh = sync_text_codec(art, text, structs, chains_by_record(art, rules, structs))
        if fresh != text:
            stage.write(codec, fresh)
            report["text_codec"] += 1


def wire_editor(art, stage, report):
    root = art.mut + "/🦀️.rs"
    text = stage.read(root)
    if "resolve_edit" not in text:
        text = re.sub(r"pub use crate::edit_rules::\{?([^};]*)\}?;", lambda m: "pub use crate::edit_rules::{resolve_edit, %s};" % m.group(1).strip().rstrip(","), text, count=1)
        stage.write(root, text)
    for path in glob.glob(art.any + "/✏️editor/🎮️commands/*/🦀️.rs"):
        body = stage.read(path)
        fresh = re.sub(r"semio_s_artifact_norm_%s_mutations::EDIT_RULES\.resolve\(document, edit\)" % art.short, "semio_s_artifact_norm_%s_mutations::resolve_edit(document, edit)" % art.short, body)
        if fresh != body:
            stage.write(path, fresh)
            report["editor"] += 1


def run(short, apply):
    stage = Stage(TICKET + "/🗑️generated/kind-convention-v2/stage/" + short)
    shutil.rmtree(stage.root, ignore_errors=True)
    art = Artifact(short, stage)
    text, rules = art.rules()
    by_path = {}
    for rule in rules:
        by_path.setdefault(rule.path, {})[rule.ty] = rule
    book = label_book(art, rules)
    report = dict(set=0, insert=0, remove=0, fixtures=0, editor=0, facets=0, text_codec=0, skipped=[])
    changes, removals = [], []
    for rule in rules:
        if rule.path in DESIGN_CASE_PATHS:
            continue
        try:
            if rule.ty == "SetFieldRule" and rule.has_index:
                names = selector_names(art, rule)
                folder = kind_folder(art.mut, rule.kind)
                record, fields = struct_of(folder)
                renames = [(o, n, art.single(rule.colls[[s for _, s in rule.sels].index(o)])) for o, n in names]
                if art.positional:
                    diff, inverse = positional_text(art, rule, folder, "set", renames)
                else:
                    diff, inverse = set_kind(art, rule, os.path.basename(folder)[: -len(rule.kind)], names, folder, record, fields)
                labels = {o: label_for(book, art, rule.colls, [s for _, s in rule.sels].index(o)) for o, _ in names}
                finish(art, stage, folder, rule.kind, diff, inverse, renames, labels, rule.colls, [o for o, _ in names], report, "set")
                changes.append((rule.line, new_line(rule, names)))
            elif rule.ty == "RemoveItemRule":
                insert = by_path[rule.path]["InsertItemRule"]
                names = selector_names(art, rule)
                parent_renames = names
                row_old = rule.field("index")
                folders = {"insert": kind_folder(art.mut, insert.kind), "remove": kind_folder(art.mut, rule.kind)}
                structs = {k: struct_of(f) for k, f in folders.items()}
                emoji = [os.path.basename(folders["insert"])[: -len(insert.kind)], os.path.basename(folders["remove"])[: -len(rule.kind)]]
                if art.positional:
                    item = snake(insert.field("item")) if insert.field("item") else [n for n, ty in structs["insert"][1] if n != "index"][0]
                    item = [n for n, ty in structs["insert"][1] if n != "index"][0]
                    row_renames = [(row_old, "id", art.single(rule.colls[-1]))]
                    ins = positional_text(art, rule, folders["insert"], "insert", [], item)
                    rem = positional_text(art, rule, folders["remove"], "remove", [(o, n, art.single(rule.colls[[s for _, s in rule.sels].index(o)])) for o, n in parent_renames] + row_renames)
                else:
                    ins, rem = pair_kinds(art, insert, rule, emoji, parent_renames, folders, structs)
                depth = len(rule.colls)
                last_single = art.single(rule.colls[-1])
                for what, kind, folder, code, extra in (("insert", insert.kind, folders["insert"], ins, []), ("remove", rule.kind, folders["remove"], rem, [(row_old, "id", last_single)])):
                    renames = [(o, n, art.single(rule.colls[[s for _, s in rule.sels].index(o)])) for o, n in parent_renames] + extra
                    labels = {o: label_for(book, art, rule.colls, [s for _, s in rule.sels].index(o)) for o, _ in parent_renames}
                    labels[row_old] = label_for(book, art, rule.colls, depth - 1)
                    olds = [o for o, _, _ in renames]
                    finish(art, stage, folder, kind, code[0], code[1], renames, labels, rule.colls, olds, report, what)
                changes.append((insert.line, new_line(insert, parent_renames)))
                removals.append((rule, parent_renames))
        except (AssertionError, KeyError, IndexError, ValueError) as error:
            report["skipped"].append((rule.kind, repr(error)))
    text = edit_rules_text(text, changes, [r for r, _ in removals])
    if removals or "ID_REMOVALS" in text:
        text = adopt_shared(art, text, [removal_line(r, n).strip() for r, n in removals])
    stage.write(art.rules_path, text)
    wire_editor(art, stage, report)
    sync_facets(art, stage, rules, report)
    print(json.dumps(report, ensure_ascii=False, indent=1))
    if apply:
        changed, conflicts = stage.apply(art.rules_path)
        print("applied", len(changed), "files; conflicts:", conflicts)
    else:
        print("staged", len(stage.written), "files at", stage.root)


if __name__ == "__main__":
    run(sys.argv[1], "--apply" in sys.argv)
