"""EN 1996 owner v5: generates the id-selector mutation kinds of the new wall fields (`bracingSystem`, `diaphragmStiffnessNPerM`) and of the new collections (`chases`, `movementJoints`) from `field-adder/specs/en1996-v5.json`, on top of the leaf-kinds templates (`leaf-kinds/index_selector.py`, `keyed.py`, `placements.py`): leaf files, group crates (at most eight kinds each), registration (workspace, locks, build-owners, script, dependencies), root enum / KINDS / ts / json / graphql / proto, edit rules with `IdRemoval`, binary tags, harness rows, oracle vectors, taxonomy and the committed mutation fixtures. Idempotent: kinds that exist are skipped.
Usage: `/home/deck/semio/.venv/bin/python -I en1996_gen_kinds_v5.py [--apply]`; without `--apply` the result stays in `🗑️generated/en1996-v5/stage`."""
import copy, json, os, re, shutil, sys

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE + "/leaf-kinds")
APPLY = "--apply" in sys.argv
sys.argv = [sys.argv[0], "en1996"]
import placements as PL
from common import *
import index_selector as I
import keyed as K

SPEC = json.load(open(HERE + "/field-adder/specs/en1996-v5.json", encoding="utf8"))
STAGE = TICKET + "/🗑️generated/en1996-v5/stage"
art, ST = PL.art, PL.ST
WALL = I.Link("walls", "MasonryWall", "id", "wall", "Wall", "Wand")


def enum_ty(name, enums):
    return "enum:" + name if name in enums else name


def snake_to_camel(name):
    return camel(name)


def build_kinds(defs):
    enums = {e["name"] for e in SPEC.get("enums", [])}
    kinds = []
    ent = I.Ent("wall", SPEC["wall_kinds"]["emoji"], [WALL], "Wall", "Wand", slug="wall", group=SPEC["wall_kinds"]["group"])
    props = defs["MasonryWall"]["properties"]
    for f in SPEC["fields"]:
        leaf = I.Leaf(ent, [f["field"]], enum_ty(f["type"], enums), props[snake_to_camel(f["field"])].get("x-semio-ui"), owner="MasonryWall", kind=f["kind"])
        k = I.SetKind(leaf)
        k.family = ""
        kinds.append(k)
    for c in SPEC["collections"]:
        lk = c["link"]
        ent = I.Ent(c["slug"], c["emoji"], [I.Link(lk["coll"], c["row"], "id", lk["single"], lk["en"], lk["de"])], lk["en"], lk["de"], slug=c["slug"], group=c["group"])
        row = defs[c["row"]]["properties"]
        for f in c["fields"]:
            leaf = I.Leaf(ent, [f["field"]], enum_ty(f["type"], enums), row[snake_to_camel(f["field"])].get("x-semio-ui"), owner=c["row"], kind="change-%s-%s" % (c["slug"], f["kind_suffix"]))
            k = I.SetKind(leaf)
            k.family = f.get("family", "")
            kinds.append(k)
        ins, rm = I.pair(ent, "insert-" + c["slug"], "remove-" + c["slug"], ST, by_index=False)
        for x in (ins, rm):
            x.family = ""
        kinds += [ins, rm]
    for k in kinds:
        k.group = k.ent.group
    return kinds


def gql_type(leaf):
    return "Float" if leaf.ty == "f64" else ("Int" if leaf.ty in ("u32", "u8") else ("Boolean" if leaf.ty == "bool" else "String"))


def proto_type(leaf):
    return "double" if leaf.ty == "f64" else ("uint32" if leaf.ty in ("u32", "u8") else ("bool" if leaf.ty == "bool" else "string"))


def graphql_gap(F, kinds):
    p = art.mut + "/🔗️.graphql"
    s = F.read(p)
    m = re.search(r"^union En1996Mutation = [^\n]*", s, re.M)
    s = s[: m.end()] + "".join(" | " + k.record for k in kinds) + s[m.end():]
    blocks = ""
    for k in kinds:
        if isinstance(k, I.SetKind):
            blocks += "type %s {\n  id: String\n  %s: %s\n}\n\n" % (k.record, k.leaf.newc, gql_type(k.leaf))
        elif isinstance(k, I.InsertKind):
            blocks += "type %s {\n  index: Int\n  item: String\n}\n\n" % k.record
        else:
            blocks += "type %s {\n  id: String\n}\n\n" % k.record
    F.write(p, s.rstrip("\n") + "\n\n" + blocks.rstrip("\n") + "\n")


def proto_gap(F, kinds):
    p = art.mut + "/🛰️.proto"
    s = F.read(p)
    nums = [int(x) for x in re.findall(r"= (\d+);\n", s[: s.index("\n  }\n}")])]
    n = max(nums)
    entries, msgs = "", ""
    for k in kinds:
        n += 1
        entries += "    %s %s = %d;\n" % (k.record, k.module, n)
        if isinstance(k, I.SetKind):
            msgs += "message %s {\n  string id = 1;\n  %s %s = 2;\n}\n" % (k.record, proto_type(k.leaf), snake(k.leaf.newc))
        elif isinstance(k, I.InsertKind):
            msgs += "message %s {\n  uint32 index = 1;\n  string item = 2;\n}\n" % k.record
        else:
            msgs += "message %s {\n  string id = 1;\n}\n" % k.record
    i = s.index("  }\n}\nmessage")
    s = s[:i] + entries + s[i:]
    F.write(p, s.rstrip("\n") + "\n" + msgs)


def edit_rules(F, kinds):
    p = art.mut + "/🧭️edit-rules/🦀️.rs"
    s = F.read(p)
    sets, ins, rem = "", "", ""
    for k in kinds:
        coll = camel(k.ent.last.coll)
        if isinstance(k, I.SetKind):
            sets += '        SetFieldRule { path: "%s[].%s", kind: "%s", selectors: &[SelectorKey::Id("id")], value: "%s" },\n' % (coll, camel(k.leaf.field), k.kind, k.leaf.newc)
        elif isinstance(k, I.InsertKind):
            ins += '        InsertItemRule { path: "%s", kind: "%s", selectors: &[], index: "index", item: "%s" },\n' % (coll, k.kind, k.item_c)
        else:
            rem += '        IdRemoval { path: "%s", kind: "%s", selectors: &[], id: "id" },\n' % (coll, k.kind)
    s = rep(s, "    ],\n    insert_item:", sets + "    ],\n    insert_item:", 1)
    s = rep(s, "    ],\n    remove_item:", ins + "    ],\n    remove_item:", 1)
    s = rep(s, "];\n\n/// 🎯️ Resolves an editor edit", rem + "];\n\n/// 🎯️ Resolves an editor edit", 1)
    F.write(p, s)


def other_locks(F, names):
    for lock in ("/home/deck/semio/🌎️hub/Cargo.lock", "/home/deck/semio/✏️s/🧑‍💻dev/🧹️fixture-sweep/Cargo.lock", "/home/deck/semio/✏️s/🔌️plugins/📕️norm/🏭️bridge/Cargo.lock"):
        if not os.path.exists(lock):
            continue
        s = F.read(lock)
        parts = re.split(r"(?m)^(?=\[\[package\]\]$)", s)
        head, blocks = parts[0], [x.rstrip("\n") for x in parts[1:]]
        name_of = lambda b: re.search(r'name = "([^"]+)"', b).group(1)
        anchor = art.anchor_crate
        if not any(name_of(b) == anchor for b in blocks):
            continue
        tmpl = next(b for b in blocks if name_of(b) == anchor)
        for g, (cname, d) in names.items():
            if any(name_of(b) == cname for b in blocks):
                continue
            at = next((i for i, b in enumerate(blocks) if name_of(b) > cname), len(blocks))
            blocks.insert(at, tmpl.replace(anchor, cname))
        out = []
        for b in blocks:
            m = re.search(r"dependencies = \[\n(.*?)\n\]", b, re.S)
            if m and ' "%s",' % anchor in m.group(1).split("\n") and name_of(b) != anchor and not name_of(b).startswith(art.crate):
                deps = m.group(1).split("\n")
                deps += [' "%s",' % c for c, _ in names.values() if ' "%s",' % c not in deps]
                deps.sort(key=lambda x: x.strip().strip('",'))
                b = b.replace(m.group(1), "\n".join(deps), 1)
            out.append(b)
        F.write(lock, head + "\n\n".join(out) + "\n")


def dependency_index(F, names):
    p = "/home/deck/semio/🔒️dependencies.json"
    s = F.read(p)
    full = "✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪨️en1996/" + art.rel
    anchor_dir = art.anchor_dir
    line = '        "%s/%s/📦️packages/🦀️rust/Cargo.toml",\n' % (full, anchor_dir)
    if line not in s:
        return
    add = ""
    for g, (cname, d) in names.items():
        l = '        "%s/%s/📦️packages/🦀️rust/Cargo.toml",\n' % (full, os.path.basename(d))
        if l not in s:
            add += l
    F.write(p, s.replace(line, line + add, 1))


def blank_diff(template):
    return json.loads(json.dumps(template))


def load(path):
    return json.load(open(path, encoding="utf8"))


def dump_json(obj):
    return json.dumps(obj, indent=2, ensure_ascii=False) + "\n"


SAMPLE = {"chase": {"id": "chase-1", "wallId": "wall-north", "kind": "Vertical", "xM": 1.0, "yM": 0.4, "widthM": 0.08, "lengthM": 1.2, "depthM": 0.03}, "movement-joint": {"id": "joint-1", "wallId": "wall-north", "positionM": 2.5}}
CHANGE = {"wallId": "wall-south", "kind": "Horizontal", "xM": 2.0, "yM": 0.8, "widthM": 0.1, "lengthM": 1.5, "depthM": 0.02, "positionM": 3.0, "bracingSystem": "Confined", "diaphragmStiffnessNPerM": 50000000.0}


def fixtures(F, kinds):
    fix = art.fix
    base = load(fix + "/" + "🔄change-in-plane-moment" + "/✅apply/🔣️.before.json")
    wall_diff = load(fix + "/🔄change-in-plane-moment/✅apply/🔣️.diff.json")
    empty = {"removed": [], "inserted": [], "moved": [], "modified": []}
    for k in kinds:
        d = fix + "/" + k.dirname + "/✅apply"
        before, after, diff = copy.deepcopy(base), copy.deepcopy(base), copy.deepcopy(wall_diff)
        wall = diff["walls"]["modified"][0]
        for key in wall["patch"]:
            if not isinstance(wall["patch"][key], dict):
                wall["patch"][key] = None
        wall_id = base["walls"][0]["id"]
        coll = camel(k.ent.last.coll)
        if isinstance(k, I.SetKind):
            field = camel(k.leaf.field)
            value = CHANGE[field]
            if coll == "walls":
                after["walls"][0][field] = value
                wall["patch"][field] = value
                mutation = {"id": wall_id, k.leaf.newc: value}
            else:
                row = copy.deepcopy(SAMPLE[k.ent.slug])
                before[coll], after[coll] = [row], [dict(row, **{field: value})]
                diff["walls"]["modified"] = []
                patch = {f: None for f in row if f != "id"}
                patch[field] = value
                diff[coll] = dict(empty, modified=[{"id": row["id"], "patch": patch}])
                mutation = {"id": row["id"], k.leaf.newc: value}
        elif isinstance(k, I.InsertKind):
            row = copy.deepcopy(SAMPLE[k.ent.slug])
            before[coll], after[coll] = [], [row]
            diff["walls"]["modified"] = []
            diff[coll] = dict(empty, inserted=[{"index": 0, "row": row}])
            mutation = {"index": 0, k.item_c: row}
        else:
            row = copy.deepcopy(SAMPLE[k.ent.slug])
            before[coll], after[coll] = [row], []
            diff["walls"]["modified"] = []
            diff[coll] = dict(empty, removed=[{"id": row["id"], "index": 0}])
            mutation = {"id": row["id"]}
        F.write(d + "/🔣️.before.json", dump_json(before))
        F.write(d + "/🔣️.after.json", dump_json(after))
        F.write(d + "/🔣️.diff.json", dump_json(diff))
        F.write(d + "/🔣️.mutation.json", dump_json({k.record: mutation}))
        F.write(d + "/🔣️.outcome.json", dump_json({"status": "applied"}))


def main():
    shutil.rmtree(STAGE, ignore_errors=True)
    F = K.KFiles(STAGE)
    defs = load(art.any + "/🧬️schema/📸️snapshot/🔣️.json")["$defs"]
    existing = K.kinds_of(art, F)
    kinds = [k for k in build_kinds(defs) if k.kind not in existing]
    if not kinds:
        print("nothing to do: all kinds exist")
        return
    assert len({k.kind for k in kinds}) == len(kinds)
    tags = K.binary_protocol_keep(art, F, kinds, set())
    existing_crates = set(re.findall(r'"(semio-s-artifact-norm-en1996-mut-[a-z0-9-]+)"', rd(art.root + "/Cargo.toml")))
    groups = K.group_kinds(art, kinds, existing_crates)
    names = K.create_crates(art, F, groups)
    groups_of = {k.kind: names[g][0] for g, ks in groups.items() for k in ks}
    K.register(art, F, names)
    other_locks(F, names)
    dependency_index(F, names)
    K.leaf_files(art, F, kinds, defs, tags)
    K.root_wire(art, F, kinds, groups_of)
    edit_rules(F, kinds)
    graphql_gap(F, kinds)
    proto_gap(F, kinds)
    PL.harness(art, F, kinds)
    K.oracle(art, F, kinds)
    K.taxonomy(art, F, kinds)
    fixtures(F, kinds)
    print("kinds", len(kinds), "crates", sorted(names), "files", len(F.written))
    if APPLY:
        print("applied", F.apply())


if __name__ == "__main__":
    main()
