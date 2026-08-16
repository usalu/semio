#!/usr/bin/env python3
# 🐍️ SCRATCH (ticket-local): W2 packet P3 (norm) — pass 2.
# Writes the real editor window/surface TS twins, the artifact-root DIALECT/DOCUMENT_SCHEMA consts
# + document_codec fix, and the whole real (non-scaffold) viewer surface for all 15 norm subsets.
from pathlib import Path

ROOT = Path("/Users/ueli/Documents/semio")
PLUGIN = ROOT / "✏️s/🔌️plugins/📕️norm"
ARTIFACTS_DIR = PLUGIN / "🗿️artifacts"

APPS = [
    dict(dir="📓️iso16757", variant="iso16757", pascal="Iso16757", label="ISO 16757", label_de="ISO 16757"),
    dict(dir="📔️vdi3805", variant="vdi3805", pascal="Vdi3805", label="VDI 3805", label_de="VDI 3805"),
    dict(dir="📕️din4108", variant="din4108", pascal="Din4108", label="DIN 4108", label_de="DIN 4108"),
    dict(dir="📗️din16798", variant="din16798", pascal="Din16798", label="DIN EN 16798", label_de="DIN EN 16798"),
    dict(dir="📘️en1990", variant="en1990", pascal="En1990", label="EN 1990", label_de="EN 1990"),
    dict(dir="📘️en1991", variant="en1991", pascal="En1991", label="EN 1991", label_de="EN 1991"),
    dict(dir="📘️en1992", variant="en1992", pascal="En1992", label="EN 1992", label_de="EN 1992"),
    dict(dir="📘️en1993", variant="en1993", pascal="En1993", label="EN 1993", label_de="EN 1993"),
    dict(dir="📘️en1994", variant="en1994", pascal="En1994", label="EN 1994", label_de="EN 1994"),
    dict(dir="📘️en1995", variant="en1995", pascal="En1995", label="EN 1995", label_de="EN 1995"),
    dict(dir="📘️en1996", variant="en1996", pascal="En1996", label="EN 1996", label_de="EN 1996"),
    dict(dir="📘️en1997", variant="en1997", pascal="En1997", label="EN 1997", label_de="EN 1997"),
    dict(dir="📘️en1998", variant="en1998", pascal="En1998", label="EN 1998", label_de="EN 1998"),
    dict(dir="📘️en1999", variant="en1999", pascal="En1999", label="EN 1999", label_de="EN 1999"),
    dict(dir="📙️din18599", variant="din18599", pascal="Din18599", label="DIN V 18599", label_de="DIN V 18599"),
]
for a in APPS:
    a["dialect_const"] = f"{a['pascal'].upper()}_DIALECT"
    a["doc_schema_const"] = f"{a['pascal'].upper()}_DOCUMENT_SCHEMA"
    a["variant_upper"] = a["variant"].upper()

STD = "🏅️standards/🔖️1/🪆️subsets/✳️any"


def subset(app):
    return ARTIFACTS_DIR / app["dir"] / STD


def w(path: Path, content: str):
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(content, encoding="utf-8")


# ---------------------------------------------------------------------------
# 1) editor window + surface TS twins
# ---------------------------------------------------------------------------

def editor_inputs_ts(app):
    return f"""/** 📥️ {app['label']} editor — inputs window: typed twin of `🦀️component.rs`'s view-model
 * boundary (the raw compliance document, rendered as pretty-printed JSON). */

export interface {app['pascal']}InputsViewModel {{
  windowKindId: "norm-{app['variant']}-inputs";
  bodyKey: "norm.{app['variant']}.play.inputs";
  documentJson: string;
}}

export const {app['variant_upper']}_INPUTS_WINDOW_KIND_ID = "norm-{app['variant']}-inputs" as const;
export const {app['variant_upper']}_INPUTS_BODY_KEY = "norm.{app['variant']}.play.inputs" as const;
"""


def editor_results_ts(app):
    return f"""/** 📊️ {app['label']} editor — results window: typed twin of `🦀️component.rs`'s view-model
 * boundary (every computed compliance check, one row each). */

export type {app['pascal']}CheckStatus = "Pass" | "Fail" | "NotApplicable";

export interface {app['pascal']}CheckRow {{
  clause: string;
  status: {app['pascal']}CheckStatus;
  computed: number;
  limit: number;
  utilization: number;
  message: string;
}}

export interface {app['pascal']}ResultsViewModel {{
  windowKindId: "norm-{app['variant']}-results";
  bodyKey: "norm.{app['variant']}.play.results";
  checks: {app['pascal']}CheckRow[];
}}

export const {app['variant_upper']}_RESULTS_WINDOW_KIND_ID = "norm-{app['variant']}-results" as const;
export const {app['variant_upper']}_RESULTS_BODY_KEY = "norm.{app['variant']}.play.results" as const;
"""


def editor_surface_ts(app):
    return f"""/** ✏️ {app['label']} editor — subset-level typed twin. Mirrors the editor manifest's mode/
 * window vocabulary; namespaced re-exports (not a blanket `export *`) since every window twin
 * independently declares its own same-shaped `*ViewModel` interface. */

export const {app['variant_upper']}_EDITOR_DIALECT = {{ artifactKind: "s.norm.{app['variant']}", standard: "1", subset: "*" }} as const;

export const {app['variant_upper']}_EDIT_MODE_ID = "edit" as const;

export * as inputsWindow from "./🎭️modes/✏️edit/🪟️windows/📥️inputs/🟦️component";
export * as resultsWindow from "./🎭️modes/✏️edit/🪟️windows/📊️results/🟦️component";
"""


def write_editor_ts(app):
    root = subset(app) / "✏️editor"
    w(root / "🎭️modes/✏️edit/🪟️windows/📥️inputs/🟦️component.ts", editor_inputs_ts(app))
    w(root / "🎭️modes/✏️edit/🪟️windows/📊️results/🟦️component.ts", editor_results_ts(app))
    w(root / "🟦️component.ts", editor_surface_ts(app))


# ---------------------------------------------------------------------------
# 2) artifact-root DIALECT / DOCUMENT_SCHEMA + document_codec fix
# ---------------------------------------------------------------------------

# apps whose ArtifactKind region lost its doc-comment/region-open line to a concurrent sweep
# (confirmed via git diff before this packet touched anything — see the report's "pre-existing
# repo damage" note).
BROKEN_ARTIFACT_KIND_HEADER = {"din4108", "din16798", "din18599"}


def fix_artifact_root(app):
    path = ARTIFACTS_DIR / app["dir"] / "🦀️component.rs"
    text = path.read_text(encoding="utf-8")
    variant = app["variant"]

    if f"pub const {app['dialect_const']}:" in text:
        print("  (already fixed, skipping)", variant)
        return

    if variant in BROKEN_ARTIFACT_KIND_HEADER:
        broken = '\n// `)` so the\n/// artifact node, not the app, owns its own kind declaration.\npub fn artifact_kind()'
        fixed = (
            "\n//#region 🔖️ArtifactKind\n"
            "/// 🗿️ The computed-compliance artifact this standard publishes on its app's `report:out` port —\n"
            "/// lifted out of the pre-migration manifest's inline `.artifact_kind(ArtifactKindSpec { .. })` so the\n"
            "/// artifact node, not the app, owns its own kind declaration.\n"
            "pub fn artifact_kind()"
        )
        assert broken in text, f"[{variant}] broken ArtifactKind header not found"
        text = text.replace(broken, fixed, 1)

    # DIALECT + DOCUMENT_SCHEMA consts, right after the artifact_kind() fn's own closing brace — anchor
    # on the fn body itself (not a region marker: some apps' `artifact_kind()` isn't region-wrapped at
    # all, e.g. en1995-en1999), so this is robust to that pre-existing per-app formatting drift.
    marker = f'crate::app_surface::artifact_kind_spec("{variant}", "{app["label"]}")\n}}\n'
    assert marker in text, f"[{variant}] artifact_kind() fn body not found verbatim"
    dialect_block = (
        f"\n/// 🪪️ This subset's canonical `(artifact_kind, standard, subset)` coordinate (ticket\n"
        f"/// 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §1) — lives at the ARTIFACT level, not\n"
        f"/// under the sibling `editor` module, so a viewer file can read it without ever importing through it.\n"
        f"pub const {app['dialect_const']}: semio_framework_plugin::app::Dialect = semio_framework_plugin::app::Dialect {{\n"
        f'    artifact_kind: "s.norm.{variant}",\n'
        f'    standard: semio_framework_plugin::app::StandardId("1"),\n'
        f"    subset: semio_framework_plugin::app::SubsetId::ANY,\n"
        f"}};\n"
        f'pub const {app["doc_schema_const"]}: &str = "semio.norm.{variant}/v1";\n'
    )
    text = text.replace(marker, marker + dialect_block, 1)

    # document_codec::<X>() -> document_codec::<EditorApp<X>>() (runtime ArtifactApp bound needs the adapter)
    old_codec = f".document_codec::<crate::apps::{variant}::{app['pascal']}PlayApp>()"
    new_codec = f".document_codec::<semio_framework_plugin::EditorApp<crate::editor::{variant}::{app['pascal']}PlayApp>>()"
    assert old_codec in text, f"[{variant}] document_codec call not found"
    text = text.replace(old_codec, new_codec, 1)

    path.write_text(text, encoding="utf-8")


# ---------------------------------------------------------------------------
# 3) viewer surface (real content — never references `editor`)
# ---------------------------------------------------------------------------

def viewer_root_rs(app):
    p, v, dc, dsc = app["pascal"], app["variant"], app["dialect_const"], app["doc_schema_const"]
    return f"""//! 👁️ {app['label']} viewer — the read-only counterpart of `✏️editor` for this subset (ticket
//! 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.2). `{p}Viewer` implements
//! `ArtifactViewer`, never `ArtifactEditor`/`ArtifactApp` — `ViewerApp<{p}Viewer>` (framework SDK) is
//! the sole runtime adapter, so this file can never structurally emit an artifact or draft mutation.
//! MUST NOT import anything from the sibling editor module (`policyViewerPurityBreaches`).

use crate::artifacts::{v}::{{{dc}, {dsc}}};
use crate::artifacts::{v}::{p}Snapshot;
use crate::viewer::{v}::modes::view;
use crate::viewer::{v}::modes::view::windows::report;
use semio_framework_plugin::{{
    ArtifactView, ArtifactViewer, ConfigView, Fault, NoConfig, NoConfigMutation, NoPresence, NoPresenceMutation, NoTransient, NoTransientMutation, UiNode, ViewEmit, Viewer,
}};
// 🚧️ SDK GAP: see the identical note in `✏️editor/🦀️component.rs` — `Dialect` is only reachable
// through `app`, not yet in the crate-root re-export list.
use semio_framework_plugin::app::{{Dialect, InteractionView}};
use semio_framework_plugin::ui_text;
use semio_framework_plugin::Label;
use store::EngineHandles;

//#region 🔖️Command
/// 👁️ The viewer declares no actions (no utilities, no mutations), so its typed command channel has
/// exactly one inert variant — a real per-command payload module the way `✏️editor/🎮️commands/*`
/// carries them would be pure ceremony for a surface that never dispatches anything through `handle`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum {p}ViewCommand {{
    #[default]
    Noop,
}}

impl protocol::OpBinary for {p}ViewCommand {{
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {{
        Ok(Vec::new())
    }}
    fn decode_op(_bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {{
        Ok({p}ViewCommand::Noop)
    }}
}}
//#endregion 🔖️Command

//#region 🔖️Viewer
#[derive(Default, Clone, Copy)]
pub struct {p}Viewer;

impl ArtifactViewer for {p}Viewer {{
    type Snapshot = {p}Snapshot;
    type Mutation = crate::artifacts::{v}::op::{p}Mutation;
    type Config = NoConfig;
    type ConfigMutation = NoConfigMutation;
    type Presence = NoPresence;
    type PresenceMutation = NoPresenceMutation;
    type Transient = NoTransient;
    type TransientMutation = NoTransientMutation;
    type Command = {p}ViewCommand;

    const DIALECT: Dialect = {dc};
    const DOCUMENT_SCHEMA: &'static str = {dsc};

    fn initial_snapshot() -> {p}Snapshot {{
        {p}Snapshot::default()
    }}

    /// 👁️ Structurally read-only: the sole `{p}ViewCommand::Noop` variant never carries a config
    /// change, so this always returns the empty `ViewEmit`. Kept as a real dispatch (not
    /// `unreachable!()`) so a future view-only action is a pure addition here, never a signature change.
    fn handle(_command: &Self::Command, _doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>, _interaction: &InteractionView<'_>, _engines: &EngineHandles) -> Result<ViewEmit<Self::ConfigMutation>, Fault> {{
        Ok(ViewEmit::default())
    }}

    fn render(body_key: &str, doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>) -> UiNode {{
        match body_key {{
            report::BODY_KEY => report::render(doc.snapshot),
            _ => ui_text(Label::data(format!("Unknown body: {{body_key}}"))),
        }}
    }}
}}
//#endregion 🔖️Viewer

//#region 🔖️Manifest
pub fn create_{v}_viewer() -> semio_framework_plugin::AppDefinition {{
    Viewer::builder({dc})
        .document(["semio", "norm", "{v}"])
        .icon_id("check-circle")
        .mode_def(view::definition())
        .default_mode_id(crate::app_surface::MODE_VIEW)
        .window_kind_def(report::definition())
        .default_layout(view::layout())
        .build_definition()
}}
//#endregion 🔖️Manifest

//#region 🧪️Tests
#[cfg(test)]
mod tests {{
    use super::*;

    #[test]
    fn create_{v}_viewer_builds_a_definition_for_this_dialect() {{
        let def = create_{v}_viewer();
        assert_eq!(def.dialect, {dc}.into());
    }}

    #[test]
    fn viewer_dialect_matches_the_artifact_coordinate() {{
        assert_eq!(<{p}Viewer as ArtifactViewer>::DIALECT, {dc});
    }}

    #[test]
    fn an_unknown_body_key_falls_back_to_a_text_node() {{
        let snapshot = {p}Snapshot::default();
        let history = semio_framework_plugin::HistoryView::empty();
        let doc = ArtifactView::new(&snapshot, &history);
        let json = serde_json::to_string(&<{p}Viewer as ArtifactViewer>::render("nope", &doc, &ConfigView {{ snapshot: &NoConfig::default() }})).expect("json");
        assert!(json.contains("Unknown body"));
    }}
}}
//#endregion 🧪️Tests
"""


def viewer_mode_rs(app):
    p, v = app["pascal"], app["variant"]
    return f"""//! 👁️ {app['label']} viewer — the `view` mode: a single full-pane Report window, the read-only
//! counterpart of the editor's inputs/results split — ticket
//! 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §1 only requires "at least one window"
//! for a viewer packet.

use crate::viewer::{v}::modes::view::windows::report;
use semio_framework_plugin::{{ModeDefinition, WindowLayout}};

//#region 🔖️Definition
/// 🧱️ Stitched into the viewer manifest by `crate::viewer::{v}::create_{v}_viewer`.
pub fn definition() -> ModeDefinition {{
    crate::app_surface::view_mode_definition()
}}

/// 🪟️ Single full-pane Report window — the read-only viewer has no inputs/results split to allocate.
pub fn layout() -> WindowLayout {{
    crate::app_surface::single_window_layout(report::WINDOW_KIND_ID, "Report")
}}
//#endregion 🔖️Definition

//#region 🧪️Tests
#[cfg(test)]
mod tests {{
    use super::*;

    #[test]
    fn the_mode_is_the_viewers_default() {{
        assert_eq!(definition().id, crate::app_surface::MODE_VIEW);
    }}
}}
//#endregion 🧪️Tests
"""


def viewer_window_rs(app):
    p, v = app["pascal"], app["variant"]
    return f"""//! 📊️ {app['label']} viewer — the Report window: a read-only table of every computed compliance
//! check, built from the same subset `🧬️schema/💡️inferences::evaluate` pure snapshot→`CheckReport`
//! function the editor's own results window uses — this file imports nothing from the sibling editor
//! surface (`policyViewerPurityBreaches` forbids it outright). Uses the framework `TableWindowKit`
//! (ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.6), the right tool for
//! compliance/report data per the contract's own guidance.

use crate::artifacts::{v}::{p}Snapshot;
// 🚧️ SDK GAP: `WindowKit`/`TableWindowKit`/`TableView` are not yet in `semio_framework_plugin`'s
// curated crate-root re-export list — only reachable through `app`, same class of gap as `Dialect`.
use semio_framework_plugin::app::{{TableView, TableWindowKit, WindowKit}};
use semio_framework_plugin::{{UiNode, WindowKindDefinition}};

//#region 🔖️Constants
pub const WINDOW_KIND_ID: &str = TableWindowKit::KIND_ID;
pub const BODY_KEY: &str = TableWindowKit::KIND_ID;
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the viewer manifest by `crate::viewer::{v}::create_{v}_viewer`. Read-only variant
/// — a viewer never declares `editable_window_kind()`'s `set-cell` command.
pub fn definition() -> WindowKindDefinition {{
    TableWindowKit::window_kind()
}}
//#endregion 🔖️Definition

//#region 🔖️Render
/// 👁️ Pure `{p}Snapshot -> UiNode` read: recomputes the compliance report straight off the document
/// (the same pure inference the editor's results window renders through `NormHost`), then tables it.
pub fn render(document: &{p}Snapshot) -> UiNode {{
    let report = crate::artifacts::{v}::standards::v1::subsets::any::schema::inferences::evaluate(document);
    TableWindowKit::render(&TableView {{ columns: crate::app_surface::report_table_columns(), rows: crate::app_surface::report_table_rows(&report) }})
}}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {{
    use super::*;

    #[test]
    fn definition_declares_the_shared_table_window_kind() {{
        let def = definition();
        assert_eq!(def.id, TableWindowKit::KIND_ID);
    }}

    #[test]
    fn render_produces_a_node_for_the_default_document() {{
        let document = {p}Snapshot::default();
        let _node = render(&document);
    }}
}}
//#endregion 🧪️Tests
"""


def viewer_window_ts(app):
    p, v = app["pascal"], app["variant"]
    return f"""/** 📊️ {app['label']} viewer — Report window: typed twin of `🦀️component.rs`'s view-model. Read-only
 * mirror of the `TableWindowKit` payload (columns/rows of strings) — no mutation-shaped fields,
 * matching the viewer's `ViewEmit`-only contract. */

export interface {p}ViewReportRow {{
  clause: string;
  status: string;
  utilization: string;
  message: string;
}}

export interface {p}ViewReportViewModel {{
  windowKindId: "framework.window.table";
  bodyKey: "framework.window.table";
  columns: string[];
  rows: {p}ViewReportRow[];
}}

export const {app['variant_upper']}_VIEW_REPORT_WINDOW_KIND_ID = "framework.window.table" as const;
export const {app['variant_upper']}_VIEW_REPORT_BODY_KEY = "framework.window.table" as const;
"""


def viewer_surface_ts(app):
    v = app["variant"]
    return f"""/** 👁️ {app['label']} viewer — subset-level typed twin. Read-only counterpart of
 * `✏️editor/🟦️component.ts`: mirrors the viewer manifest's mode/window vocabulary, no mutation-shaped
 * exports (no command payload types, no config schema beyond the framework's own empty config). */

export const {app['variant_upper']}_VIEWER_DIALECT = {{ artifactKind: "s.norm.{v}", standard: "1", subset: "*" }} as const;

export const {app['variant_upper']}_VIEW_MODE_ID = "view" as const;

export * from "./🎭️modes/👁️view/🪟️windows/📊️report/🟦️component";
"""


def write_viewer(app):
    viewer = subset(app) / "👁️viewer"
    scaffold_mode = viewer / "🎭️modes/👁️view"
    import shutil
    if scaffold_mode.exists():
        shutil.rmtree(scaffold_mode)
    scaffold_root_rs = viewer / "🦀️component.rs"
    scaffold_root_ts = viewer / "🟦️component.ts"

    w(scaffold_root_rs, viewer_root_rs(app))
    w(scaffold_root_ts, viewer_surface_ts(app))
    w(viewer / "🎭️modes/👁️view/🦀️component.rs", viewer_mode_rs(app))
    # required-but-empty mode-level facets (mirrors the scaffold's own shape)
    for facet in ["🎚️config", "🎮️commands", "👥️presence", "🫧️transient"]:
        w(viewer / f"🎭️modes/👁️view/{facet}/📌️empty.md", "")
    w(viewer / "🎭️modes/👁️view/🪟️windows/📊️report/🦀️component.rs", viewer_window_rs(app))
    w(viewer / "🎭️modes/👁️view/🪟️windows/📊️report/🟦️component.ts", viewer_window_ts(app))
    for facet in ["🎚️config", "🎚️options", "🎬️actions", "👥️presence", "🪛️utilities", "🫧️transient"]:
        w(viewer / f"🎭️modes/👁️view/🪟️windows/📊️report/{facet}/📌️empty.md", "")


def main():
    for app in APPS:
        print("ts+viewer", app["variant"])
        write_editor_ts(app)
        fix_artifact_root(app)
        write_viewer(app)


if __name__ == "__main__":
    main()
