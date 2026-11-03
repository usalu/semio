"""EN 1994 nested action position leaves: `change-<beam|slab>-action-x-m` and `change-slab-action-y-m` for `beams|slabs[id=P].actions[id=A].xM` and `slabs[id=P].actions[id=A].yM` (beams carry `CharacteristicAction`, slabs `SlabAction`), parents addressed by id (`beamId`) and the action by its `id` (merge mode, same engine as `en1994_actions.py`).
Run: python3 -I en1994_action_leaves.py [--apply]  (stage `🗑️generated/leaf-kinds/stage-en1994-action-leaves`; idempotent: kinds already in `KINDS` are skipped).
"""
import copy, json, os, re, shutil, sys
sys.path.insert(0, os.path.dirname(__file__))
from common import *
import index_selector as I, keyed as K

SHORT = "en1994"
DIRNAME = next(n for n in os.listdir(NORM + "/🗿️artifacts") if n.endswith(SHORT))
EMOJI = DIRNAME[: -len(SHORT)]
ROOT = NORM + "/🗿️artifacts/" + DIRNAME
ANYD = ROOT + "/🏅️standards/🔖️1/🪆️subsets/✳️any"
STAGE_N = TICKET + "/🗑️generated/leaf-kinds/stage-en1994-action-leaves"
TEMPLATE = "change-beam-stud-count"
ST = I.Style("En1994", "semio_s_artifact_norm_en1994_model", "semio_s_artifact_norm_en1994_diff", "En1994Snapshot", "../../../📸️snapshot/🟦️.ts", mode="lanes", opt_index=False, tagged=False, item_field=lambda e: "action")

_s = rd(ANYD + "/🧬️schema/🧬️mutations/🦀️.rs")
ANCHOR_CRATE = re.search(r"^use (semio_s_artifact_norm_en1994_mut_[a-z0-9_]+)::%s;" % TEMPLATE.replace("-", "_"), _s, re.M).group(1).replace("_", "-")
_p = re.search(r'\[workspace\.dependencies\.%s\]\npath = "([^"]+)"' % ANCHOR_CRATE, rd(ROOT + "/Cargo.toml")).group(1)
ANCHOR_DIR = _p.split("/🧬️mutations/")[1].split("/")[0]
art = K.Art(SHORT, EMOJI, ST, SHORT, ANCHOR_DIR, ANCHOR_CRATE, TEMPLATE, None)

COLLS = [
    dict(coll="beams", single="beam", row="CompositeBeam", en="Beam", de="Träger", de_action="Trägereinwirkung", emoji="🏋️", ref="change-beam-action-q-area-pa", action_row="CharacteristicAction", fields=("x_m",)),
    dict(coll="slabs", single="slab", row="CompositeSlab", en="Slab", de="Decke", de_action="Deckeneinwirkung", emoji="🧱️", ref="change-slab-action-q-area-pa", action_row="SlabAction", fields=("x_m", "y_m")),
]
LEAVES = [
    ("x_m", "xM", "position of the point force along the span", "Position der Einzellast entlang der Spannweite", "↔️", 0.05),
    ("y_m", "yM", "position of the point force across the width", "Position der Einzellast quer zur Breite", "↕️", 0.05),
]


def kinds_all():
    out = []
    for c in COLLS:
        parent = I.Link(c["coll"], c["row"], c["single"] + "_id", c["single"], c["en"], c["de"])
        child = I.Link("actions", c["action_row"], "id", "action", c["en"] + " action", c["de_action"])
        ent = I.Ent(c["single"] + "-action", c["emoji"], [parent, child], c["en"] + " action", c["de_action"], slug=c["single"] + "-action", group=c["single"] + "-action-position")
        for field, cam, en, de, emoji, step in (l for l in LEAVES if l[0] in c["fields"]):
            ui = {"label": {"en": "%s %s" % (c["en"], en), "de": "%s: %s" % (c["de_action"], de)}, "unit": "m", "step": step, "precision": 2}
            leaf = I.Leaf(ent, [field], "f64", ui, kind="change-%s-action-%s" % (c["single"], field.replace("_", "-")))
            k = I.SetKind(leaf)
            k.family, k.c, k.cam = "", c, cam
            out.append(k)
    return out


def testkit(F, kinds):
    p = art.mut + "/🧰️testkit/🦀️.rs"
    s = F.read(p)
    lines = ""
    for k in kinds:
        c = k.c
        if "En1994Mutation::%s(" % k.record in s:
            continue
        lines += "        En1994Mutation::%s(%s::%s { %s_id: base.%s[0].id.clone(), id: base.%s[0].actions[0].id.clone(), %s: 1.5 }),\n" % (k.record, k.module, k.record, c["single"], c["coll"], c["coll"], k.leaf.new)
    i = s.index("    ]\n}", s.index("pub fn demo_mutation_cases"))
    F.write(p, s[:i] + lines + s[i:])


def positional_diff(kinds, F):
    for k in kinds:
        ref = art.mut + "/" + next(n for n in os.listdir(art.mut) if n.endswith(k.c["ref"]))
        text = rd(ref + "/🔺️diff/🦀️.rs")
        text = text.replace("QAreaPa", k.record[len("Change%sAction" % k.c["en"]):]).replace("q_area_pa", k.leaf.field).replace("q-area-pa", k.kind.rsplit("action-", 1)[1])
        F.write(art.mut + "/" + k.dirname + "/🔺️diff/🦀️.rs", text)


def fix_sets(kinds, F):
    for k in kinds:
        c = k.c
        ref = art.fix + "/" + next(n for n in os.listdir(art.fix) if n.endswith(c["ref"])) + "/✅apply"
        base = json.loads(rd(ref + "/🔣️.before.json"))
        diff = json.loads(rd(ref + "/🔣️.diff.json"))
        fields = json.loads(rd(ref + "/🔣️.mutation.json"))
        parent_id, action_id = list(fields.values())[0]["%sId" % c["single"]], list(fields.values())[0]["id"]
        after = copy.deepcopy(base)
        parent = next(r for r in after[c["coll"]] if r["id"] == parent_id)
        action = next(a for a in parent["actions"] if a["id"] == action_id)
        value = 2.0 if k.cam == "xM" else 0.5
        action[k.cam] = value
        patch = diff[c["coll"]]["modified"][0]["actions"]["modified"][0]
        for key in patch:
            if key != "index":
                patch[key] = None
        patch[k.cam] = value
        d = art.fix + "/" + k.dirname + "/✅apply"
        F.write(d + "/🔣️.before.json", dumps(base))
        F.write(d + "/🔣️.mutation.json", dumps({k.record: {"%sId" % c["single"]: parent_id, "id": action_id, k.leaf.newc: value}}))
        F.write(d + "/🔣️.after.json", dumps(after))
        F.write(d + "/🔣️.outcome.json", dumps({"status": "applied"}))
        F.write(d + "/🔣️.diff.json", dumps(diff))


def main():
    F = K.KFiles(STAGE_N)
    F.merge = True
    defs = json.loads(rd(art.any + "/🧬️schema/📸️snapshot/🔣️.json"))["$defs"]
    existing = set(K.kinds_of(art, F))
    new_kinds = [k for k in kinds_all() if k.kind not in existing]
    if not new_kinds:
        print("nothing to do: all action position kinds exist")
        return
    tags = K.binary_protocol_keep(art, F, new_kinds, set())
    existing_crates = set(re.findall(r'"(semio-s-artifact-norm-%s-mut-[a-z0-9-]+)"' % SHORT, rd(art.root + "/Cargo.toml")))
    groups = K.group_kinds(art, new_kinds, existing_crates)
    names = K.create_crates(art, F, groups)
    groups_of = {k.kind: names[g][0] for g, ks in groups.items() for k in ks}
    K.register(art, F, names)
    K.leaf_files(art, F, new_kinds, defs, tags)
    positional_diff(new_kinds, F)
    K.root_wire(art, F, new_kinds, groups_of)
    K.edit_rules(art, F, new_kinds)
    testkit(F, new_kinds)
    K.harness_rs_py(art, F, new_kinds)
    K.harness(art, F, new_kinds)
    K.oracle(art, F, new_kinds)
    K.taxonomy(art, F, new_kinds)
    fix_sets(new_kinds, F)
    print("kinds", len(new_kinds), "crates", len(names), "files", len(F.written))
    if "--apply" in sys.argv:
        print("applied", F.apply(), "merged", len(F.merged))


if __name__ == "__main__":
    shutil.rmtree(STAGE_N, ignore_errors=True)
    main()
