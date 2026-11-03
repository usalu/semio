"""EN 1997 top-level leaf `structureId`: clones the `change-design-situation` kind (string scalar, no row selector) and wires it like the generated kinds."""
import copy, json, os, re, shutil, sys
sys.path.insert(0, os.path.dirname(__file__))
from common import *
import en1997 as E, model as M, wire as W

from en1997 import PRIVATE_STAGE
art = E.art
SRC = "change-design-situation"
KIND = "change-structure-id"


def main():
    F = Files(PRIVATE_STAGE)
    if KIND in re.findall(r'^    "([a-z0-9-]+)",$', rd(art.mut + "/🦀️.rs"), re.M):
        print("already present")
        return
    ent = M.Ent("structure", "🏗️", [], "Structure", "Bauwerk")
    leaf = M.Leaf(ent, ["structure_id"], "String", {"label": {"en": "Structure id", "de": "Bauwerkskennung"}}, False)
    leaf.kind, leaf.module, leaf.record = KIND, KIND.replace("-", "_"), pascal(KIND)
    leaf.dirname = emoji_dir("🏗️", KIND)
    src = next(d for d in os.listdir(art.mut) if d.endswith(SRC))
    subs = [("DesignSituation", "StructureId"), ("design_situation", "structure_id"), ("design-situation", "structure-id"), ("designSituation", "structureId"), ("Design Situation", "Structure Id"), ("Change design situation", "Change structure id"), ("Bemessungssituation ändern", "Bauwerkskennung ändern"), (src, leaf.dirname)]
    def conv(t):
        for a, b in subs:
            t = t.replace(a, b)
        return t
    for root, _, files in os.walk(art.mut + "/" + src):
        for fn in files:
            p = os.path.join(root, fn)
            rel = p[len(art.mut + "/" + src):]
            F.write(art.mut + "/" + leaf.dirname + rel, conv(rd(p)))
    cat = art.mut + "/" + leaf.dirname + "/🔣️.json"
    kinds_all = re.findall(r'^    "([a-z0-9-]+)",$', rd(art.mut + "/🦀️.rs"), re.M) + [KIND]
    j = json.loads(F.read(cat))
    j["binaryTag"] = len(kinds_all) - 1
    j["emoji"] = "🏗️"
    F.write(cat, dumps(j))
    ui = art.mut + "/" + leaf.dirname + "/🧬️schema/🔣️.json"
    s = json.loads(F.read(ui))
    p = s["properties"]["newStructureId"]["x-semio-ui"]
    p["label"] = {"en": "Structure id", "de": "Bauwerkskennung"}
    p["description"] = {"en": "The identifier of the geotechnical structure this document describes.", "de": "Die Kennung des geotechnischen Bauwerks, das dieses Dokument beschreibt."}
    p["group"] = "value"
    F.write(ui, dumps(s))
    d = "semio-s-artifact-norm-en1997-mut-design"
    mounts = art.mut + "/" + next(x for x in os.listdir(art.mut) if x.endswith("design")) + "/🦀️.rs"
    t = F.read(mounts)
    F.write(mounts, rep(t, "//#endregion 🪢️Mounts", '#[path = "../%s/🦀️.rs"]\npub mod %s;\n//#endregion 🪢️Mounts' % (leaf.dirname, leaf.module)))
    groups_of = {KIND: d}
    E.root_twins(F, [leaf], groups_of)
    W.edit_rules(art, F, [leaf], lambda l: "structureId", [])
    E.text_codec(F, [leaf], groups_of)
    W.binary_protocol(art, F, kinds_all)
    E.harness(F, [leaf])
    E.oracle(F, [leaf])
    W.taxonomy(art, F, [leaf])
    base = json.loads(rd(E.RICH))
    after = copy.deepcopy(base)
    after["structureId"] = base["structureId"] + "-alt"
    top = list(json.loads(rd(art.fix + "/⛰️change-slope-angle/✅apply/🔣️.diff.json")).keys())
    diff = {k: None for k in top}
    diff["structureId"] = after["structureId"]
    b = art.fix + "/" + leaf.dirname + "/✅apply"
    F.write(b + "/🔣️.before.json", dumps(base))
    F.write(b + "/🔣️.mutation.json", dumps({"mutation": "changeStructureId", "newStructureId": after["structureId"]}))
    F.write(b + "/🔣️.after.json", dumps(after))
    F.write(b + "/🔣️.outcome.json", dumps({"status": "applied"}))
    F.write(b + "/🔣️.diff.json", dumps(diff))
    print("files", len(F.written), F.apply() if "--apply" in sys.argv else "")


if __name__ == "__main__":
    shutil.rmtree(PRIVATE_STAGE, ignore_errors=True)
    main()
