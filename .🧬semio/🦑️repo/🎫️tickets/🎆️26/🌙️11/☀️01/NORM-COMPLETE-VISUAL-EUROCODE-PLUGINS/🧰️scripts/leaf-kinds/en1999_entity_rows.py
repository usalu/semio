"""🗂️ Insert/remove kinds for the wave-5 en1999 collections (fatigue categories, web patch loads, ring stiffeners), one crate per entity. Usage: python -I en1999_entity_rows.py [--apply]"""
import copy, json, os, re, shutil, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from common import *
import en1999 as E, en1999_rows as R, wire as W

STAGE_DIR = TICKET + "/🗑️generated/en1999-v5/stage-rows"


def row(slug, snake, camel, coll, collp, ccoll, ty, en, de):
    return dict(slug=slug, pas=pascal(slug), snake=snake, coll=coll, collp=collp, camel=camel, ccoll=ccoll, ty=ty, en=en, de=de, add_de=de + " hinzufügen", rem_de=de + " entfernen",
                doc_en="The complete %s record to add." % en, doc_de="Der vollständige hinzuzufügende Datensatz: %s." % de, rem_en="The %s this mutation removes." % en, rem_de_doc="Entfernt wird: %s." % de)


ROWS = [
    row("fatigue-category", "fatigue_category", "fatigueCategory", "fatigue_categories", "FatigueCategories", "fatigueCategories", "FatigueCategory", "fatigue detail site", "Ermüdungsdetail-Stelle"),
    row("patch-load", "patch_load", "patchLoad", "patch_loads", "PatchLoads", "patchLoads", "PatchLoad", "web patch load", "Stegeinzellast"),
    row("ring-stiffener", "ring_stiffener", "ringStiffener", "ring_stiffeners", "RingStiffeners", "ringStiffeners", "RingStiffener", "ring stiffener", "Ringsteife"),
    row("transverse-stiffener", "transverse_stiffener", "transverseStiffener", "transverse_stiffeners", "TransverseStiffeners", "transverseStiffeners", "TransverseStiffener", "transverse stiffener", "Quersteife"),
]


def main():
    art = E.art
    R.ROWS[:] = ROWS
    F = Files(STAGE_DIR)
    existing = set(re.findall(r'^    "([a-z0-9-]+)",$', rd(art.mut + "/🦀️.rs"), re.M))
    rows = [k for k in R.kinds() if k.kind not in existing]
    if not rows:
        print("nothing to do")
        return
    groups = {"rows-" + r["slug"]: [k for k in rows if k.r is r] for r in ROWS}
    groups = {g: ks for g, ks in groups.items() if ks}
    names = W.create_crates(art, F, groups, art.mut + "/" + art.anchor_dir)
    groups_of = {k.kind: names[g][0] for g, ks in groups.items() for k in ks}
    kinds_all = re.findall(r'^    "([a-z0-9-]+)",$', rd(art.mut + "/🦀️.rs"), re.M) + [k.kind for k in rows]
    tags = {k: i for i, k in enumerate(kinds_all)}
    W.register(art, F, names)
    R.write_kinds(art, F, rows, tags)
    R.wire_twins(art, F, rows, groups_of)
    R.edit_rules(art, F, rows)
    for r in ROWS:
        ks = [k for k in rows if k.r is r]
        if ks:
            R.text_codec(art, F, ks, groups_of)
    W.binary_protocol(art, F, kinds_all)
    W.harness(art, F, rows, None)
    base = json.loads(rd(art.fix + "/🏋️change-member-n-ed/✅apply/🔣️.before.json"))
    base.update(copy.deepcopy(E.NEW_ROWS))
    top = list(json.loads(rd(art.fix + "/🏋️change-member-n-ed/✅apply/🔣️.diff.json")).keys())
    R.TOP_KEYS[:] = top + [k for k in E.NEW_ROWS if k not in top]
    R.fixtures(art, F, rows, base)
    R.harness_dupes(art, F, rows)
    R.taxonomy(art, F, rows)
    R.oracle(art, F, rows)
    print("kinds", len(rows), "files", len(F.written))
    if "--apply" in sys.argv:
        print("applied", F.apply())


if __name__ == "__main__":
    shutil.rmtree(STAGE_DIR, ignore_errors=True)
    main()
