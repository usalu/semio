"""🗂️ Insert/remove kinds for the seven top-level en1999 collections (materials, sections, connections, fire scenarios, fatigue details, cold-formed sheets, shells). Usage: python -I en1999_collections.py [--apply]"""
import copy, json, os, re, shutil, sys
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", "leaf-kinds"))
from common import *
import en1999 as E, en1999_rows as R, wire as W

STAGE_DIR = TICKET + "/🗑️generated/kind-convention/stage-en1999"


def row(slug, snake, camel, coll, collp, ccoll, ty, en, de):
    return dict(slug=slug, pas=pascal(slug), snake=snake, coll=coll, collp=collp, camel=camel, ccoll=ccoll, ty=ty, en=en, de=de, add_de=de + " hinzufügen", rem_de=de + " entfernen",
                doc_en="The complete %s record to add." % en, doc_de="Der vollständige hinzuzufügende Datensatz: %s." % de, rem_en="The %s this mutation removes." % en, rem_de_doc="Entfernt wird: %s." % de)


ROWS = [
    row("material", "material", "material", "materials", "Materials", "materials", "AluminiumMaterial", "material", "Werkstoff"),
    row("section", "section", "section", "sections", "Sections", "sections", "AluminiumSection", "section", "Querschnitt"),
    row("connection", "connection", "connection", "connections", "Connections", "connections", "AluminiumConnection", "connection", "Anschluss"),
    row("fire-scenario", "fire_scenario", "fireScenario", "fire_scenarios", "FireScenarios", "fireScenarios", "FireScenario", "fire scenario", "Brandszenario"),
    row("fatigue-detail", "fatigue_detail", "fatigueDetail", "fatigue_details", "FatigueDetails", "fatigueDetails", "FatigueDetail", "fatigue detail", "Ermüdungsdetail"),
    row("cold-formed-sheet", "cold_formed_sheet", "coldFormedSheet", "cold_formed", "ColdFormed", "coldFormed", "ColdFormedSheet", "cold-formed sheet", "Kaltgeformtes Blech"),
    row("shell", "shell", "shell", "shells", "Shells", "shells", "AluminiumShell", "shell", "Schale"),
]


def main():
    art = E.art
    R.ROWS[:] = ROWS
    R.GROUP = "collection-rows"
    F = Files(STAGE_DIR)
    existing = set(re.findall(r'^    "([a-z0-9-]+)",$', rd(art.mut + "/🦀️.rs"), re.M))
    rows = [k for k in R.kinds() if k.kind not in existing]
    if not rows:
        print("nothing to do")
        return
    groups = R.pseudo_group(rows)
    names = W.create_crates(art, F, groups, art.mut + "/" + art.anchor_dir)
    groups_of = {k.kind: names[g][0] for g, ks in groups.items() for k in ks}
    kinds_all = list(existing_ordered(art)) + [k.kind for k in rows]
    tags = {k: i for i, k in enumerate(kinds_all)}
    W.register(art, F, names)
    R.write_kinds(art, F, rows, tags)
    R.wire_twins(art, F, rows, groups_of)
    R.edit_rules(art, F, rows)
    R.text_codec(art, F, rows, groups_of)
    dedupe_model_import(art, F)
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


def existing_ordered(art):
    return re.findall(r'^    "([a-z0-9-]+)",$', rd(art.mut + "/🦀️.rs"), re.M)


def dedupe_model_import(art, F):
    p = art.any + "/🚪️io/📝️text/🧬️mutations/🦀️.rs"
    s = F.read(p)
    m = re.search(r"use semio_s_artifact_norm_en1999_model::\{([^}]*)\};", s)
    names = list(dict.fromkeys(x.strip() for x in m.group(1).split(",") if x.strip()))
    F.write(p, s[: m.start()] + "use semio_s_artifact_norm_en1999_model::{%s};" % ", ".join(names) + s[m.end():])


if __name__ == "__main__":
    shutil.rmtree(STAGE_DIR, ignore_errors=True)
    main()
