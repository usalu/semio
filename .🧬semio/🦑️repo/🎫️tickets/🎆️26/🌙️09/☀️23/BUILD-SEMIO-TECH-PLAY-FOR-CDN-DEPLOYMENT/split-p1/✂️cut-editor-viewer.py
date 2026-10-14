#!/usr/bin/env python3
"""✂️ Cuts the editor into facet crates and the viewer into its own crate (lane P1, slice S2e).

- `-editor-commands` (`✏️editor/🎮️commands`, new root): the editor's action ids, `EnergyModelEditorCommand` with its
  action bridge and op codecs, the inspector's action definitions and the UI id helpers — the vocabulary panels and
  windows dispatch, moved down out of the editor root so they no longer reach up into it.
- `-editor-config`, `-editor-results`, `-editor-interaction` (their folders), `-editor-edit` (`🎭️modes/✏️edit`: the
  edit mode, its tools and windows), `-editor-panels` (`📌️panels`, new wiring root: artifact tree and inspector).
- `-editor` (`✏️editor`): `EnergyModelEditor`, its reduce seam, retained commands, store preparation, examples and manifest.
- `-viewer` (`👁️viewer`): the read-only twin with its windows.
"""
import importlib.util, json, os, re, sys
spec = importlib.util.spec_from_file_location("artifact_lib", os.path.join(os.path.dirname(os.path.abspath(__file__)), "✂️artifact_lib.py"))
L = importlib.util.module_from_spec(spec); spec.loader.exec_module(L)
ROOT = f"{L.ART}/🦀️.rs"; ED = f"{L.ANY}/✏️editor"; VW = f"{L.ANY}/👁️viewer"
root = L.rd(ROOT)
if "as editor_commands;" in root: sys.exit("already applied")
HEADER = "#![allow(async_fn_in_trait)]\n#![allow(clippy::too_many_arguments)]\n\n"
NEW = {"editor_commands": "semio-s-artifact-energy-model-editor-commands", "editor_config": "semio-s-artifact-energy-model-editor-config",
       "editor_results": "semio-s-artifact-energy-model-editor-results", "editor_interaction": "semio-s-artifact-energy-model-editor-interaction",
       "editor_edit": "semio-s-artifact-energy-model-editor-edit", "editor_panels": "semio-s-artifact-energy-model-editor-panels",
       "editor": "semio-s-artifact-energy-model-editor", "viewer": "semio-s-artifact-energy-model-viewer"}
# 1. editor root -> commands
ed = L.rd(f"{ED}/🦀️.rs")
ed, ids = L.take(ed, "//#region 🏷️ActionIds\n", "//#endregion 🏷️ActionIds\n")
ed, command = L.take(ed, "//#region 🔖️Command\n", "//#endregion 🔖️Command\n")
ed, gas_ids = L.take_item(ed, "pub fn gas_kind_id(", "\n}\n")
ed, gas_table = L.take_item(ed, "pub const GAS_KIND_IDS:", ";\n")
ed, rough_ids = L.take_item(ed, "pub fn surface_roughness_id(", "\n}\n")
ed, rough_table = L.take_item(ed, "pub const SURFACE_ROUGHNESS_IDS:", ";\n")
ed, ui = L.take(ed, "//#region 🔖️UiHelpers\n", "//#endregion 🔖️UiHelpers\n")
ed, inspector_fn = L.take_item(ed, "fn inspector_action(", "\n}\n")
ed, inspector_defs = L.take_item(ed, "pub fn inspector_action_definitions()", "\n}\n")
cmd_root = (f"//! 🎮️ Energy model editor commands: the action ids every window and panel dispatches, `EnergyModelEditorCommand`\n"
            f"//! with its action bridge and op codecs, the inspector's action definitions and the UI id helpers.\n\n" + HEADER + "@@ALIASES@@\n\n"
            + "use semio_framework_plugin::{Fault, FaultCode, FaultOrigin};\nuse semio_framework_ui_locale::LocalizedLabel;\nuse semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};\nuse crate::model::SurfaceClass;\n\n"
            + ids + "\n" + command + "\n" + ui + "\n//#region 🆔️KindIds\n" + gas_ids + "\n" + gas_table + "\n" + rough_ids + "\n" + rough_table + "//#endregion 🆔️KindIds\n\n//#region 🔍️InspectorActions\n"
            + inspector_fn + "\n" + inspector_defs + "\n//#endregion 🔍️InspectorActions\n")
os.makedirs(f"{ED}/🎮️commands", exist_ok=True)
if os.path.exists(f"{ED}/🎮️commands/📌️.empty.md"): os.remove(f"{ED}/🎮️commands/📌️.empty.md")
L.wr(f"{ED}/🎮️commands/🦀️.rs", cmd_root)
d = ed.index("\n\n") + 2
ed = ed[:d] + HEADER + "@@ALIASES@@\n\n" + ed[d:]
ed = ed.rstrip("\n") + '\n\n//#region 👥️Presence\n#[path = "👥️presence/🧬️schema/🦀️.rs"]\npub mod presence;\n//#endregion 👥️Presence\n'
L.wr(f"{ED}/🦀️.rs", ed)
# 2. facet roots
for p in (f"{ED}/🎚️config/🦀️.rs", f"{ED}/📊️results/🦀️.rs", f"{ED}/🕹️interaction/🦀️.rs"):
    t = L.rd(p); d = t.index("\n\n") + 2; L.wr(p, t[:d] + HEADER + "@@ALIASES@@\n\n" + t[d:])
edit = L.rd(f"{ED}/🎭️modes/✏️edit/🦀️.rs"); d = edit.index("\n\n") + 2
edit = edit[:d] + HEADER + "@@ALIASES@@\n\n" + edit[d:].rstrip("\n") + ('\n\n//#region 🛠️Tools\npub mod tools {\n    #[path = "../🛠️tools/⚡️simulation/🦀️.rs"]\n    pub mod simulation;\n}\n//#endregion 🛠️Tools\n\n'
    '//#region 🪟️Windows\npub mod windows {\n    #[path = "../🪟️windows/🧊️model/🦀️.rs"]\n    pub mod model;\n    #[path = "../🪟️windows/⚡️simulation/🦀️.rs"]\n    pub mod simulation;\n'
    '    #[path = "../🪟️windows/🌳️structure/🦀️.rs"]\n    pub mod structure;\n    #[path = "../🪟️windows/📊️zones/🦀️.rs"]\n    pub mod zones;\n}\n//#endregion 🪟️Windows\n')
L.wr(f"{ED}/🎭️modes/✏️edit/🦀️.rs", edit)
L.wr(f"{ED}/📌️panels/🦀️.rs", "//! 📌️ Energy model editor panels: the artifact tree (outliner) and the inspector, both reading the one `\"energyModel\"`\n//! interaction domain.\n\n"
     + HEADER + "@@ALIASES@@\n\n" + '#[path = "🗿️artifact/🦀️.rs"]\npub mod artifact;\n#[path = "🔍️inspection/🦀️.rs"]\npub mod inspection;\n')
vw = L.rd(f"{VW}/🦀️.rs"); d = vw.index("\n\n") + 2
mt = re.search(r'#\[path = "\."\]\npub mod viewer \{.*?\n\}\n', root, re.S).group(0)
vw = vw[:d] + HEADER + "@@ALIASES@@\n\n" + vw[d:].rstrip("\n") + ('\n\n//#region 🧩️Facets\n#[path = "👥️presence/🧬️schema/🦀️.rs"]\npub mod presence;\npub mod config {\n    #[path = "../🎚️config/🧬️schema/🦀️.rs"]\n    pub mod schema;\n}\n'
    'pub mod modes {\n    pub mod view {\n        #[path = "../../🎭️modes/👁️view/🦀️.rs"]\n        mod component;\n        pub use component::*;\n        pub mod windows {\n'
    '            #[path = "../../../🎭️modes/👁️view/🪟️windows/🧊️model/🦀️.rs"]\n            pub mod model;\n            #[path = "../../../🎭️modes/👁️view/🪟️windows/⚡️simulation/🦀️.rs"]\n            pub mod simulation;\n'
    '            #[path = "../../../🎭️modes/👁️view/🪟️windows/🌳️structure/🦀️.rs"]\n            pub mod structure;\n            #[path = "../../../🎭️modes/👁️view/🪟️windows/📊️zones/🦀️.rs"]\n            pub mod zones;\n        }\n    }\n}\n//#endregion 🧩️Facets\n')
L.wr(f"{VW}/🦀️.rs", vw)
# 3. root: editor/viewer trees leave
root = root.replace(re.search(r'(?:// [^\n]*\n)*#\[path = "\."\]\npub mod editor \{.*?\n\}\n', root, re.S).group(0), "")
root = root.replace(mt, "")
root = L.replace_once(root, "extern crate semio_s_artifact_energy_model_diff as diff;\n", "extern crate semio_s_artifact_energy_model_diff as diff;\n" + "".join(f"extern crate {pkg.replace('-', '_')} as {a};\n" for a, pkg in sorted(NEW.items())))
L.wr(ROOT, root)
# 4. references
moved = set(re.findall(r"(?m)^pub (?:const|fn|struct|enum|type|static) ([A-Za-z_][A-Za-z0-9_]*)", cmd_root))
moved_alt = "|".join(sorted(map(re.escape, moved), key=len, reverse=True))
E = r"\bcrate\s*::\s*editor\s*::\s*model\s*::\s*"
RULES = [(E + r"config\b", "crate::editor_config"), (E + r"interaction\b", "crate::editor_interaction"), (E + r"results\b", "crate::editor_results"),
         (E + r"panels\s*::\s*", "crate::editor_panels::"), (E + r"modes\s*::\s*edit\b", "crate::editor_edit"), (E + r"modes\b", "crate::editor_edit"),
         (E + r"(" + moved_alt + r")\b", r"crate::editor_commands::\1"), (E + r"presence\b", "crate::editor::presence"), (E, "crate::editor::"),
         (r"\bcrate\s*::\s*editor\s*::\s*model\b", "crate::editor"),
         (r"\bcrate\s*::\s*viewer\s*::\s*model\s*::\s*", "crate::viewer::"), (r"\bcrate\s*::\s*viewer\s*::\s*model\b", "crate::viewer")]
ROOTS = {"editor_commands": f"{ED}/🎮️commands/🦀️.rs", "editor_config": f"{ED}/🎚️config/🦀️.rs", "editor_results": f"{ED}/📊️results/🦀️.rs",
         "editor_interaction": f"{ED}/🕹️interaction/🦀️.rs", "editor_edit": f"{ED}/🎭️modes/✏️edit/🦀️.rs", "editor_panels": f"{ED}/📌️panels/🦀️.rs",
         "editor": f"{ED}/🦀️.rs", "viewer": f"{VW}/🦀️.rs"}
owned = {}
for a in ["editor_commands", "editor_config", "editor_results", "editor_interaction", "editor_edit", "editor_panels", "viewer", "editor"]:
    for f in L.closure_files(ROOTS[a]): owned.setdefault(f, a)
for r, dirs, fs in os.walk(L.ART):
    dirs[:] = [x for x in dirs if x not in ("target", "node_modules", "📦️packages")]
    for f in fs:
        if not f.endswith(".rs"): continue
        p = os.path.join(r, f); t = L.rd(p); a = owned.get(p)
        t2 = L.rewrite_code(t, RULES)
        if a: t2 = L.rewrite_code(t2, [(r"\bcrate\s*::\s*" + a + r"\s*::\s*", "crate::"), (r"\bcrate\s*::\s*" + a + r"\b(?!\s*::)", "crate")])
        if a == "editor":
            t2 = L.rewrite_code(t2, [(r"(?<![\w:])super\s*::\s*(" + moved_alt + r")\b", r"crate::editor_commands::\1")]) if p != ROOTS["editor"] else t2
        if t2 != t: L.wr(p, t2)
# editor root names the moved vocabulary it uses
ed = L.rd(ROOTS["editor"])
used = sorted(n for n in moved if re.search(r"(?<![\w:])" + re.escape(n) + r"\b", L.strip(ed)))
if used: ed = ed.replace("@@ALIASES@@\n\n", "@@ALIASES@@\n\nuse crate::editor_commands::{" + ", ".join(used) + "};\n", 1)
L.wr(ROOTS["editor"], ed)
for a, p in ROOTS.items():
    t = L.rd(p); t2 = re.sub(r"(?m)^use crate::(" + "|".join(NEW) + r");\n", "", t)
    if t2 != t: L.wr(p, t2)
DESC = {"editor_commands": "Energy model editor commands: action ids, the editor command with its bridge and codecs, inspector actions",
        "editor_config": "Energy model editor config: simulation run settings and result field, their mutations and codecs",
        "editor_results": "Energy model editor results: the run's per-surface payload decoded into a colour map and legend",
        "editor_interaction": "Energy model editor interaction: target ids and granularity of the energyModel interaction domain",
        "editor_edit": "Energy model editor edit mode: the simulation tool and the model, simulation, structure and zones windows",
        "editor_panels": "Energy model editor panels: the artifact tree and the inspector",
        "editor": "Energy model editor: EnergyModelEditor, its reduce seam, retained commands, examples and manifest",
        "viewer": "Energy model viewer: the read-only twin with its windows"}
json.dump([{"owner": os.path.dirname(p), "package": NEW[a], "alias": a, "description": DESC[a]} for a, p in ROOTS.items()],
          open(os.path.join(os.path.dirname(os.path.abspath(__file__)), "✂️editor-viewer-crates.json"), "w"), ensure_ascii=False, indent=1)
print("moved vocabulary:", len(moved))
