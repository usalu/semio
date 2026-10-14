#!/usr/bin/env python3
"""✂️ Closes the editor/viewer layering left by S2e and the stale crate-root names the inline-mount closure bug hid (lane P1, S2h).

- `ResultField`/`result_field` move from editor-results to editor-config (config stores and validates the field; results
  colours by it), ending the config <-> results cycle.
- `window_shared_action_definitions` moves from the editor to the edit mode crate (it lists edit-window verbs only), so the
  simulation window's laws no longer reach the editor.
- Panels import the command vocabulary from editor-commands, and the bridge (`args_bridge`) is public there, so panels and
  their tests no longer reach the editor.
- `artifact_kind()` moves to the snapshot crate beside the document schema id it names; editor and hub call it there.
- The sqlite snapshot declaration-admission laws mount in the aggregator, which owns `declaration()`.
- Self-alias paths (`crate::editor_edit::`, `crate::viewer::`) become `crate::`; flat-root leftovers name their owners.
"""
import importlib.util, os, re, sys
spec = importlib.util.spec_from_file_location("artifact_lib", os.path.join(os.path.dirname(os.path.abspath(__file__)), "✂️artifact_lib.py"))
L = importlib.util.module_from_spec(spec); spec.loader.exec_module(L)
A = L.ANY; ED = f"{A}/✏️editor"; EDIT = f"{ED}/🎭️modes/✏️edit"; VIEW = f"{A}/👁️viewer"
HUB = f"{L.REPO}/🌎️hub/🧩️compositions/🔋️energy/🦀️.rs"
def edit(path, *pairs, regex=False):
    t = L.rd(path); o = t
    for a, b in pairs:
        if regex: t = re.sub(a, b, t)
        else:
            if a not in t: sys.exit(f"anchor missing in {path}: {a[:80]!r}")
            t = t.replace(a, b)
    if t != o: L.wr(path, t)
def files(root):
    for r, d, fs in os.walk(root):
        d[:] = [x for x in d if x not in ("target", "node_modules", "📦️packages")]
        for f in fs:
            if f.endswith(".rs"): yield os.path.join(r, f)

# 1. commands bridge public; editor and panel test call it
edit(f"{ED}/🎮️commands/🦀️.rs", ("\nmod args_bridge {\n", "\npub mod args_bridge {\n"))
edit(f"{ED}/🦀️.rs", ("        args_bridge::command_from_action(action, args)\n", "        crate::editor_commands::args_bridge::command_from_action(action, args)\n"))
edit(f"{ED}/📌️panels/🔍️inspection/🧪️tests/🔬️unit/🦀️.rs",
     ("    use semio_framework_plugin::ArtifactEditor;\n", ""),
     ("    <crate::editor::EnergyModelEditor as ArtifactEditor>::command_from_action(&action, Some((semio_framework_plugin::DslValue::Object(entries)).clone()), None)",
      "    crate::editor_commands::args_bridge::command_from_action(&action, Some(&semio_framework_plugin::DslValue::Object(entries)))"))
for p in (f"{ED}/📌️panels/🔍️inspection/🦀️.rs", f"{ED}/📌️panels/🗿️artifact/🦀️.rs"):
    edit(p, ("use crate::editor::{", "use crate::editor_commands::{"))

# 2. window_shared_action_definitions -> edit mode crate
er = L.rd(f"{ED}/🦀️.rs")
start = er.index("/// 🪟️ The verbs a WINDOW renders but any window may have to dispatch")
end = er.index("}\n", er.index("pub fn window_shared_action_definitions()", start)) + 2
moved = er[start:end]
er = er[:start] + er[end:].lstrip("\n")
er = er.replace("    actions.extend(window_shared_action_definitions());\n", "    actions.extend(crate::editor_edit::window_shared_action_definitions());\n")
L.wr(f"{ED}/🦀️.rs", er)
moved = moved.replace("[`inspector_action_definitions`]", "`semio_s_artifact_energy_model_editor_commands::inspector_action_definitions`")
ed = L.rd(f"{EDIT}/🦀️.rs")
anchor = "//#region 🔖️Definition\n"
ed = ed.replace(anchor, "//#region 🪟️SharedWindowActions\n" + moved + "//#endregion 🪟️SharedWindowActions\n\n" + anchor, 1)
L.wr(f"{EDIT}/🦀️.rs", ed)
edit(f"{EDIT}/🪟️windows/⚡️simulation/🧪️tests/🔬️unit/🦀️.rs", ("crate::editor::window_shared_action_definitions()", "crate::window_shared_action_definitions()"))
for p in files(A):
    edit(p, (r"`crate::editor::model::window_shared_action_definitions`", "`semio_s_artifact_energy_model_editor_edit::window_shared_action_definitions`"), regex=True)

# 3. ResultField -> editor config
rr = L.rd(f"{ED}/📊️results/🦀️.rs")
rs, re_ = rr.index("//#region 🔖️Field\n"), rr.index("//#endregion 🔖️Field\n") + len("//#endregion 🔖️Field\n")
field = rr[rs:re_]; rr = rr[:rs] + rr[re_:].lstrip("\n")
rr = rr.replace("use crate::editor_config::EnergyModelConfig;\n", "use crate::editor_config::ResultField;\n")
L.wr(f"{ED}/📊️results/🦀️.rs", rr)
field = field.replace("/// 🏷️ English caption stem, used by [`legend_caption`].", "/// 🏷️ English caption stem, used by `semio_s_artifact_energy_model_editor_results::legend_caption`.")
cfg = L.rd(f"{ED}/🎚️config/🦀️.rs")
cfg = cfg.replace("//#endregion 🔖️Config\n", "//#endregion 🔖️Config\n\n" + field, 1)
cfg = cfg.replace("(`crate::editor::model::results::ResultField`)", "([`ResultField`])")
L.wr(f"{ED}/🎚️config/🦀️.rs", cfg)
cfg_files = set(L.closure_files(f"{ED}/🎚️config/🦀️.rs"))
for p in files(A):
    t = L.rd(p)
    t2 = L.rewrite_code(t, [(r"\bcrate::editor_results::(ResultField|result_field)\b", r"crate::editor_config::\1"),
                            (r"\bcrate::editor_results::\{(\s*(?:result_field|ResultField)\s*(?:,\s*(?:result_field|ResultField)\s*)?)\}", r"crate::editor_config::{\1}")])
    if p in cfg_files: t2 = L.rewrite_code(t2, [(r"\bcrate::editor_config::", "crate::")])
    if t2 != t: L.wr(p, t2)

# 4. self-alias paths
for root, alias in ((EDIT, "editor_edit"), (VIEW, "viewer")):
    for p in L.closure_files(f"{root}/🦀️.rs"):
        t = L.rd(p); t2 = re.sub(r"\bcrate::" + alias + r"::", "crate::", t)
        if t2 != t: L.wr(p, t2)

# 5. flat-root leftovers
edit(f"{A}/🧵️simulation-session/🧪️tests/🔬️unit/🦀️.rs", ("crate::Engine::run(", "crate::sim::Engine::run("))
edit(f"{ED}/🦀️.rs", ("use crate::{EnergyModelMutation};\n", "use crate::mutations::EnergyModelMutation;\n"))
edit(f"{VIEW}/🦀️.rs", ("use crate::{EnergyModelMutation};\n", "use crate::mutations::EnergyModelMutation;\n"))
edit(f"{A}/🚪️io/🦀️.rs", ("    use crate::{EnergyModelDiff, EnergyModelMutation};\nuse crate::snapshot::{EnergyModelSnapshot};\n",
                         "    use crate::diff::EnergyModelDiff;\n    use crate::mutations::EnergyModelMutation;\n    use crate::snapshot::EnergyModelSnapshot;\n"))

# 6. artifact_kind -> snapshot
agg = L.rd(f"{L.ART}/🦀️.rs")
ks, ke = agg.index("//#region 🔖️ArtifactKind\n"), agg.index("//#endregion 🔖️ArtifactKind\n") + len("//#endregion 🔖️ArtifactKind\n")
kind = agg[ks:ke]; agg = agg[:ke].replace(kind, "") + agg[ke:]
agg = agg.replace("use {semio_framework_plugin::ArtifactKindSpec,semio_framework_artifact_reference::Dialect,semio_framework_plugin::MediaClass,semio_framework_plugin::MediaForm,semio_framework_plugin::MediaType,semio_framework_plugin::OsMediaCapability,semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId};\n",
                  "use semio_framework_artifact_reference::{Dialect, StandardId, SubsetId};\n")
agg = re.sub(r"\n\n\n+", "\n\n", agg)
kind = kind.replace("pub fn artifact_kind() -> ArtifactKindSpec {\n    ArtifactKindSpec {", "pub fn artifact_kind() -> semio_framework_plugin::ArtifactKindSpec {\n    use semio_framework_plugin::{MediaClass, MediaForm, MediaType, OsMediaCapability};\n    semio_framework_plugin::ArtifactKindSpec {")
sn = L.rd(f"{A}/🧬️schema/📸️snapshot/🦀️.rs")
sn = sn.replace("\n//#region 🧬️Facet\n", "\n" + kind + "\n//#region 🧬️Facet\n", 1)
L.wr(f"{A}/🧬️schema/📸️snapshot/🦀️.rs", sn)
edit(f"{ED}/🦀️.rs", ("        .artifact_kind(crate::artifact_kind())\n", "        .artifact_kind(crate::snapshot::artifact_kind())\n"))
edit(HUB, ("semio_s_artifact_energy_model::artifact_kind().id", "semio_s_artifact_energy_model_snapshot::artifact_kind().id"))

# 7. sqlite declaration-admission laws -> aggregator
sq = f"{A}/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs"
t = L.rd(sq); m = re.search(r'(?:#\[cfg\(test\)\]\s*)?#\[path\s*=\s*"🧪️tests/🦀️.rs"\]\s*mod tests;\n?', t)
if not m: sys.exit("sqlite test mount missing")
L.wr(sq, t[:m.start()] + t[m.end():])
tt = f"{A}/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs"
edit(tt, ("use crate::sqlite::EnergyModelSnapshot;", "use crate::snapshot::EnergyModelSnapshot;"),
     ("crate::EnergyModelMutation", "crate::mutations::EnergyModelMutation"), ("crate::ENERGY_MODEL_DOCUMENT_SCHEMA", "crate::snapshot::ENERGY_MODEL_DOCUMENT_SCHEMA"))
agg = agg.rstrip("\n") + '\n\n//#region 🧪️Tests\n#[cfg(test)]\n#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs"]\nmod sqlite_snapshot_declaration_tests;\n//#endregion 🧪️Tests\n'
L.wr(f"{L.ART}/🦀️.rs", agg)
print("ok")
