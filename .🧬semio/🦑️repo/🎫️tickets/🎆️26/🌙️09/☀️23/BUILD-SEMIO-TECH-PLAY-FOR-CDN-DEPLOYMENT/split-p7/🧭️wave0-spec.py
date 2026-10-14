#!/usr/bin/env python3
"""🧭️ Writes the two standard wave-0 move specs of one artifact (split lane P7): every artifact-root definition except the
declaration glue moves to the snapshot facet (`domain`), and every editor-root definition of the named regions moves to
`✏️editor/🧰️kit` (`editor-kit`).

usage: 🧭️wave0-spec.py <out dir> <artifact dir (repo-relative)> <editor module, e.g. editor::cad> <kit regions,comma> [keep root names,comma]
"""
import json, os, subprocess, sys
HERE = os.path.dirname(os.path.abspath(__file__))
REPO = os.path.abspath(os.path.join(HERE, *[".."] * 8))
out, artifact, editor_mod, regions = sys.argv[1:5]
keep = set((sys.argv[5] if len(sys.argv) > 5 else "definition,declaration,pilot_languages").split(","))
S = artifact + "/🏅️standards/🔖️1/🪆️subsets/✳️any"
def items(path):
    rows = subprocess.run(["python3", os.path.join(HERE, "📋️items.py"), os.path.join(REPO, path)], capture_output=True, text=True).stdout.split("\n")
    for row in rows:
        if row:
            yield (row.split("\t") + [""])[:5]
root = [name for _, _, kind, name, _ in items(artifact + "/🦀️.rs") if kind not in ("use", "impl", "macro", "?", "mod") and name not in keep]
roots = ["standards", "schema", "diff", "mutations", "snapshot", "examples", "editor", "viewer", "host"]
json.dump({"moves": [{"from": artifact + "/🦀️.rs", "from_module": "crate", "to": S + "/🧬️schema/📸️snapshot/🦀️.rs", "to_module": "crate::standards::v1::subsets::any::schema::snapshot", "root_relative": roots, "items": root}]},
          open(os.path.join(out, "🚚️domain.json"), "w", encoding="utf-8"), ensure_ascii=False, indent=1)
wanted = regions.split(",")
kit = [name for _, _, kind, name, region in items(S + "/✏️editor/🦀️.rs") if kind not in ("use", "impl", "macro", "?", "mod") and region.split(" / ")[0] in wanted]
name = editor_mod.split("::")[-1]
json.dump({"moves": [{"from": S + "/✏️editor/🦀️.rs", "from_module": "crate::" + editor_mod, "to": S + "/✏️editor/🧰️kit/🦀️.rs", "to_module": "crate::" + editor_mod + "::kit", "items": kit,
                      "doc": f"//! 🧰️ {name} editor kit: identifiers, the dispatch context and the ui-value and helper plumbing every command, mode\n//! and panel builds on.",
                      "mount": "#[path = \"🧰️kit/🦀️.rs\"]\npub mod kit;"}]}, open(os.path.join(out, "🚚️editor-kit.json"), "w", encoding="utf-8"), ensure_ascii=False, indent=1)
print("root:", root)
print("kit:", kit)
