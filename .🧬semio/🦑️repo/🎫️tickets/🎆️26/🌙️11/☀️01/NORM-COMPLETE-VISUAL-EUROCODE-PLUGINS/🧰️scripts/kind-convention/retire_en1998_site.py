"""🗑️ Retires the whole-site `update-site` kind and the dead `replace_all` helper of en1998 through every layer. Usage: python -I retire_en1998_site.py"""
import os as _o, sys as _s; _o.environ.get("TH_FORCE") or (_s.stderr.write("unsafe against the current tree (destructive or not idempotent), refusing to run (TH_FORCE=1 forces)\n"), _s.exit(2))  # unsafe-guard
import glob, json, os, re, shutil, sys
sys.path.insert(0, os.path.join(os.path.dirname(__file__), "..", "leaf-kinds"))
from common import *

A = NORM + "/🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any"
FW = SEMIO + "/🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/🗂️schema-document-index.json"
K = "update-site"


def rw(p, fn):
    t = rd(p)
    u = fn(t)
    if u != t:
        open(p, "w", encoding="utf-8").write(u)


def drop(t, rx, flags=0):
    return re.sub(rx, "", t, flags=flags)


def main():
    for d in glob.glob(A + "/*/*/🌚️update-site") + glob.glob(A + "/*/*/*/🌚️update-site"):
        shutil.rmtree(d)
    m = A + "/🧬️schema/🧬️mutations"
    rw(m + "/🏢️buildings-site/🦀️.rs", lambda t: drop(t, r'#\[path = "\.\./🌚️update-site/🦀️\.rs"\]\npub mod update_site;\n'))
    rw(m + "/🏢️buildings/🦀️.rs", lambda t: t.replace(", update_site}", "}").replace("{update_site, ", "{").replace(", update_site,", ","))
    def root(t):
        t = drop(t, r"use semio_s_artifact_norm_en1998_mut_buildings::update_site;\n")
        t = drop(t, r"    UpdateSite\(update_site::UpdateSite\),\n")
        t = drop(t, r'    "update-site",\n')
        return drop(t, r"/// 🔁 Replaces a whole collection.*?\n}\n\n", re.S)
    rw(m + "/🦀️.rs", root)
    rw(m + "/🟦️.ts", lambda t: drop(drop(t, r"import [^\n]*\bUpdateSite\b[^\n]*\n"), r"  \| UpdateSite\n|  updateSite: parseUpdateSite,\n"))
    def js(t):
        d = json.loads(t)
        d["oneOf"] = [x for x in d["oneOf"] if not x.get("$ref", "").endswith("/%s/schema.json" % K)]
        return json.dumps(d, indent=2, ensure_ascii=False) + "\n"
    rw(m + "/🔣️.json", js)
    rw(m + "/🔗️.graphql", lambda t: drop(drop(t, r"  \| UpdateSite\n").replace("update-site, ", ""), r'# kind: "update-site"\ntype UpdateSite \{[^\n]*\n'))
    rw(m + "/🛰️.proto", lambda t: drop(drop(t, r"// kind: update-site\nmessage UpdateSite \{[^\n]*\n"), r"    UpdateSite update_site = \d+;\n"))
    rw(m + "/🧪️tests/🔬️unit/🦀️.rs", lambda t: drop(t, r"use semio_s_artifact_norm_en1998_mut_buildings::update_site;\n").replace(
        "async fn update_site_applies_plateau_acceleration", "async fn change_site_s_ap_r_applies_plateau_acceleration").replace(
        "En1998Mutation::UpdateSite(update_site::UpdateSite { site: site.clone() })", "En1998Mutation::ChangeSiteSApR(semio_s_artifact_norm_en1998_mut_buildings::change_site_s_ap_r::ChangeSiteSApR { new_s_ap_r: 2.0 })"))
    rw(A + "/🚪️io/📝️text/🔺️diff/🧪️tests/🔬️unit/🦀️.rs", lambda t: t.replace("update_site_diff_updates", "change_site_s_ap_r_diff_updates").replace(
        "En1998Mutation::UpdateSite(crate::mutations::update_site::UpdateSite { site: site.clone() })", "En1998Mutation::ChangeSiteSApR(crate::mutations::change_site_s_ap_r::ChangeSiteSApR { new_s_ap_r: 2.0 })"))
    rw(A + "/🚪️io/📝️text/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs", lambda t: t.replace("op_text_round_trips_update_site", "op_text_round_trips_insert_building"))
    def text(t):
        t = drop(t, r"use semio_s_artifact_norm_en1998_mut_buildings::update_site(?:::UpdateSite)?;\n")
        t = drop(t, r"        En1998Mutation::UpdateSite\(p\) => [^\n]*\n")
        t = drop(t, r'        "update-site" => [^\n]*\n')
        return drop(t, r"        En1998Mutation::UpdateSite\(UpdateSite \{ site: snap\.site\.clone\(\) \}\),\n")
    rw(A + "/🚪️io/📝️text/🧬️mutations/🦀️.rs", text)
    rw(A + "/🚪️io/💾️binary/🧬️mutations/📡️.protocol.semio", lambda t: drop(t, r"record update-site tag=\d+\nfield payload bytes\n"))
    h = A + "/🧪️tests/🫨️mutate-en1998-1"
    rw(h + "/🦀️.rs", lambda t: drop(drop(t, r'        "update-site(?:-noop)?" => \(\n(?:            [^\n]*\n)+        \),\n'), r'    "update-site(?:-noop)?",\n'))
    rw(h + "/🐍️.py", lambda t: drop(t, r'    "update-site(?:-noop)?"[:,][^\n]*\n'))
    rw(h + "/🥒️.feature", lambda t: drop(t, r"      \| update-site(?:-noop)? +\|[^\n]*\n"))
    def orc(t):
        o = json.loads(t)
        for c in o["mutationCatalogs"]:
            c["vectors"] = [v for v in c["vectors"] if v["mutationId"] != K]
            c["kinds"] = [k for k in c["kinds"] if k != K]
        for mm in o["mutationManifests"]:
            mm["mutations"] = [x for x in mm["mutations"] if x["id"] != K]
        return dumps(o)
    rw(A + "/🔮️oracles/🔣️.json", orc)
    def tax(t):
        d = json.loads(t)
        d["rows"] = [r for r in d["rows"] if not (r["artifact"].endswith("en1998") and r["kind"] == K)]
        return dumps(d)
    rw(TAXONOMY, tax)
    rw(FW, lambda t: drop(t, r'      "[^"\n]*en1998/mutation/update-site/schema\.json": "[^\n]*\n'))
    print("done")


main()
