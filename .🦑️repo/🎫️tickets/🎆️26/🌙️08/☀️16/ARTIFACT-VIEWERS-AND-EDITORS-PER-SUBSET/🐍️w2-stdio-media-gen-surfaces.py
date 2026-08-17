#!/usr/bin/env python3
"""Generator for W2 packet P1-stdio-media (ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET).
Stamps out the 17 thin editor+viewer surface pairs for stdio's png/jpg/bmp/tiff/gif/svg/mp4/mp3/wav/
avi/html/md kinds, from a per-subset metadata table gathered by hand-reading each artifact's own
schema/io files (see the ticket's w2-stdio-media-report.md for the recipe). Scratch tool for THIS
authoring session only — not permanent repo tooling, not committed.
"""
import os

REPO = "/Users/ueli/Documents/semio"
ART = os.path.join(REPO, "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts")
GLUE = os.path.join(REPO, "✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/📦️glue.rs")

def p(*parts):
    return os.path.join(*parts)

def find_subset_dir(kind_dir, std_dirname, subset_dirname):
    base = p(ART, kind_dir, "🏅️standards", std_dirname, "🪆️subsets", subset_dirname)
    assert os.path.isdir(base), f"MISSING {base}"
    return base

# ------------------------------------------------------------------------------------------------
# Per-subset metadata (hand-verified against each artifact's own schema/io/component.rs — see report)
# ------------------------------------------------------------------------------------------------
SUBSETS = [
    dict(subset_key="png", kind_dir="📷️png", std="🔖️1.2", subset="✳️any", std_mod="v1_2", subset_mod="any",
         X="Png", kind="png", Snapshot="PngSnapshot", Mutation="PngMutation",
         dialect_const="PNG_DIALECT", artifact_kind="s.stdio.png", standard="1.2", dsubset="*",
         doc_schema_const="STDIO_PNG_DOCUMENT_SCHEMA", kit="image", raster_field="pixels",
         mutation_variant="SetPixels", mime="image/png", encode_fn="encode_png", has_wh=True),
    dict(subset_key="jpg_any", kind_dir="📷️jpg", std="🔖️jfif-1.01", subset="✳️any", std_mod="v_jfif_1_01", subset_mod="any",
         X="JpgAny", kind="jpg", Snapshot="JpgSnapshot", Mutation="JpgMutation",
         dialect_const="JPG_ANY_DIALECT", artifact_kind="s.stdio.jpg", standard="jfif-1.01", dsubset="*",
         doc_schema_const="STDIO_JPG_DOCUMENT_SCHEMA", kit="image", raster_field="pixels",
         mutation_variant="SetPixels", mime="image/jpeg", encode_fn="encode_jpg", has_wh=True),
    dict(subset_key="jpg_baseline", kind_dir="📷️jpg", std="🔖️jfif-1.01", subset="✳️baseline", std_mod="v_jfif_1_01", subset_mod="baseline",
         X="JpgBaseline", kind="jpg", Snapshot="JpgSnapshot", Mutation="JpgMutation",
         dialect_const="JPG_BASELINE_DIALECT", artifact_kind="s.stdio.jpg", standard="jfif-1.01", dsubset="baseline",
         doc_schema_const="STDIO_JPG_DOCUMENT_SCHEMA", kit="image", raster_field="pixels",
         mutation_variant="SetPixels", mime="image/jpeg", encode_fn="encode_jpg", has_wh=True),
    dict(subset_key="bmp", kind_dir="🖼️bmp", std="🔖️v3", subset="✳️any", std_mod="v_v3", subset_mod="any",
         X="Bmp", kind="bmp", Snapshot="BmpSnapshot", Mutation="BmpMutation",
         dialect_const="BMP_DIALECT", artifact_kind="s.stdio.bmp", standard="v3", dsubset="*",
         doc_schema_const="STDIO_BMP_DOCUMENT_SCHEMA", kit="image", raster_field="pixels",
         mutation_variant="SetPixelData", mime="image/bmp", encode_fn="encode_bmp", has_wh=True),
    dict(subset_key="tiff_any", kind_dir="🖼️tiff", std="🔖️6.0", subset="✳️any", std_mod="v6_0", subset_mod="any",
         X="TiffAny", kind="tiff", Snapshot="TiffSnapshot", Mutation="TiffMutation",
         dialect_const="TIFF_ANY_DIALECT", artifact_kind="s.stdio.tiff", standard="6.0", dsubset="*",
         doc_schema_const="STDIO_TIFF_DOCUMENT_SCHEMA", kit="image", raster_field="pixels",
         mutation_variant="SetPixels", mime="image/tiff", encode_fn="encode_tiff", has_wh=False),
    dict(subset_key="tiff_baseline", kind_dir="🖼️tiff", std="🔖️6.0", subset="✳️baseline", std_mod="v6_0", subset_mod="baseline",
         X="TiffBaseline", kind="tiff", Snapshot="TiffSnapshot", Mutation="TiffMutation",
         dialect_const="TIFF_BASELINE_DIALECT", artifact_kind="s.stdio.tiff", standard="6.0", dsubset="baseline",
         doc_schema_const="STDIO_TIFF_DOCUMENT_SCHEMA", kit="image", raster_field="pixels",
         mutation_variant="SetPixels", mime="image/tiff", encode_fn="encode_tiff", has_wh=False),
    dict(subset_key="gif_87a", kind_dir="🎞️gif", std="🔖️87a", subset="✳️any", std_mod="v87a", subset_mod="any",
         X="Gif87a", kind="gif", Snapshot="GifSnapshot", Mutation="GifMutation",
         dialect_const="GIF_87A_DIALECT", artifact_kind="s.stdio.gif", standard="87a", dsubset="*",
         doc_schema_const="STDIO_GIF_DOCUMENT_SCHEMA", kit="image_indexed", raster_field="indices",
         mutation_variant="SetImagePixels", mime="image/gif", encode_fn="encode_gif", has_wh=True),
    dict(subset_key="gif_89a", kind_dir="🎞️gif", std="🔖️89a", subset="✳️any", std_mod="v89a", subset_mod="any",
         X="Gif89a", kind="gif", Snapshot="GifSnapshot", Mutation="GifMutation",
         dialect_const="GIF_89A_DIALECT", artifact_kind="s.stdio.gif", standard="89a", dsubset="*",
         doc_schema_const="STDIO_GIF_DOCUMENT_SCHEMA", kit="image_indexed", raster_field="indices",
         mutation_variant="SetFramePixels", mime="image/gif", encode_fn="encode_gif", has_wh=True),
    dict(subset_key="svg_any", kind_dir="🎨️svg", std="🔖️1.1", subset="✳️any", std_mod="v1_1", subset_mod="any",
         X="SvgAny", kind="svg", Snapshot="SvgSnapshot", Mutation="SvgMutation",
         dialect_const="SVG_ANY_DIALECT", artifact_kind="s.stdio.svg", standard="1.1", dsubset="*",
         doc_schema_const="STDIO_SVG_DOCUMENT_SCHEMA", kit="image_svg", raster_field=None,
         mutation_variant="SetSnapshot", mime="image/svg+xml", encode_fn=None, has_wh=False),
    dict(subset_key="svg_basic", kind_dir="🎨️svg", std="🔖️1.1", subset="✳️basic", std_mod="v1_1", subset_mod="basic",
         X="SvgBasic", kind="svg", Snapshot="SvgSnapshot", Mutation="SvgMutation",
         dialect_const="SVG_BASIC_DIALECT", artifact_kind="s.stdio.svg", standard="1.1", dsubset="basic",
         doc_schema_const="STDIO_SVG_DOCUMENT_SCHEMA", kit="image_svg", raster_field=None,
         mutation_variant="SetSnapshot", mime="image/svg+xml", encode_fn=None, has_wh=False),
    dict(subset_key="svg_tiny", kind_dir="🎨️svg", std="🔖️1.1", subset="✳️tiny", std_mod="v1_1", subset_mod="tiny",
         X="SvgTiny", kind="svg", Snapshot="SvgSnapshot", Mutation="SvgMutation",
         dialect_const="SVG_TINY_DIALECT", artifact_kind="s.stdio.svg", standard="1.1", dsubset="tiny",
         doc_schema_const="STDIO_SVG_DOCUMENT_SCHEMA", kit="image_svg", raster_field=None,
         mutation_variant="SetSnapshot", mime="image/svg+xml", encode_fn=None, has_wh=False),
    dict(subset_key="mp4", kind_dir="🎥️mp4", std="🔖️isobmff", subset="✳️any", std_mod="isobmff", subset_mod="any",
         X="Mp4", kind="mp4", Snapshot="Mp4Snapshot", Mutation="Mp4Mutation",
         dialect_const="MP4_DIALECT", artifact_kind="s.stdio.mp4", standard="isobmff", dsubset="*",
         doc_schema_const="STDIO_MP4_DOCUMENT_SCHEMA", kit="media", media_kind="Video"),
    dict(subset_key="mp3", kind_dir="🎵️mp3", std="🔖️mpeg1-layer3", subset="✳️any", std_mod="mpeg1_layer3", subset_mod="any",
         X="Mp3", kind="mp3", Snapshot="Mp3Snapshot", Mutation="Mp3Mutation",
         dialect_const="MP3_DIALECT", artifact_kind="s.stdio.mp3", standard="mpeg1-layer3", dsubset="*",
         doc_schema_const="STDIO_MP3_DOCUMENT_SCHEMA", kit="media", media_kind="Audio"),
    dict(subset_key="wav", kind_dir="🔊️wav", std="🔖️riff-pcm", subset="✳️any", std_mod="riff_pcm", subset_mod="any",
         X="Wav", kind="wav", Snapshot="WavSnapshot", Mutation="WavMutation",
         dialect_const="WAV_DIALECT", artifact_kind="s.stdio.wav", standard="riff-pcm", dsubset="*",
         doc_schema_const="STDIO_WAV_DOCUMENT_SCHEMA", kit="media", media_kind="Audio"),
    dict(subset_key="avi", kind_dir="📼️avi", std="🔖️1.0", subset="✳️any", std_mod="v1_0", subset_mod="any",
         X="Avi", kind="avi", Snapshot="AviSnapshot", Mutation="AviMutation",
         dialect_const="AVI_DIALECT", artifact_kind="s.stdio.avi", standard="1.0", dsubset="*",
         doc_schema_const="STDIO_AVI_DOCUMENT_SCHEMA", kit="media", media_kind="Video"),
    dict(subset_key="html", kind_dir="🌐️html", std="🔖️5", subset="✳️any", std_mod="v5", subset_mod="any",
         X="Html", kind="html", Snapshot="HtmlSnapshot", Mutation="HtmlMutation",
         dialect_const="HTML_DIALECT", artifact_kind="s.stdio.html", standard="5", dsubset="*",
         doc_schema_const="STDIO_HTML_DOCUMENT_SCHEMA", kit="text"),
    dict(subset_key="md", kind_dir="📝️md", std="🔖️commonmark", subset="✳️any", std_mod="v_commonmark", subset_mod="any",
         X="Md", kind="md", Snapshot="MdSnapshot", Mutation="MdMutation",
         dialect_const="MD_DIALECT", artifact_kind="s.stdio.md", standard="commonmark", dsubset="*",
         doc_schema_const="STDIO_MD_DOCUMENT_SCHEMA", kit="text"),
]

for s in SUBSETS:
    s["base"] = find_subset_dir(s["kind_dir"], s["std"], s["subset"])
    s["artifact_mod"] = f'crate::artifacts::{s["kind"]}::standards::{s["std_mod"]}::subsets::{s["subset_mod"]}'
    s["kind_root_mod"] = f'crate::artifacts::{s["kind"]}'
    # 🔀 baseline subsets reuse the ✳️any subset's schema type verbatim (D4 Tier-1) but their own
    # 🚪️io module carries only the validator, not the codec fns — those two subsets' image window
    # needs the ✳️any io module for encode_*.
    s["io_subset_mod"] = "any" if s["subset_mod"] == "baseline" else s["subset_mod"]
    s["io_mod"] = f'crate::artifacts::{s["kind"]}::standards::{s["std_mod"]}::subsets::{s["io_subset_mod"]}'

print(f"{len(SUBSETS)} subsets loaded, all base dirs verified.")

ICONS = {"image": "image", "image_indexed": "image", "image_svg": "image", "media": "play", "text": "file-text"}

# ------------------------------------------------------------------------------------------------
# File writers
# ------------------------------------------------------------------------------------------------
def write(path, content):
    os.makedirs(os.path.dirname(path), exist_ok=True)
    with open(path, "w", encoding="utf-8") as f:
        f.write(content)

def editor_root_rs(s):
    kit = s["kit"]
    if kit in ("image", "image_indexed"):
        field = s["raster_field"]
        cmd_decl = f"    SetPixelRegion {{ {field}: Vec<u8> }},"
        if kit == "image":
            arm = f'            {s["X"]}EditCommand::SetPixelRegion {{ {field} }} => Ok(Emit::mutations(vec![{s["Mutation"]}::{s["mutation_variant"]} {{ {field}: {field}.clone() }}])),'
        else:
            arm = f'            {s["X"]}EditCommand::SetPixelRegion {{ {field} }} => Ok(Emit::mutations(vec![{s["Mutation"]}::{s["mutation_variant"]} {{ index: 0, {field}: {field}.clone() }}])),'
    elif kit == "image_svg":
        cmd_decl = "    SetPixelRegion { source: String },"
        arm = (f'            {s["X"]}EditCommand::SetPixelRegion {{ source }} => match <{s["Snapshot"]} as store::ArtifactDsl>::parse_dsl(source) {{\n'
               f'                Ok(snapshot) => Ok(Emit::mutations(vec![{s["Mutation"]}::SetSnapshot {{ snapshot }}])),\n'
               f'                Err(_) => Ok(Emit::default()),\n'
               f'            }},')
    elif kit == "text":
        cmd_decl = "    ReplaceText { text: String },"
        arm = (f'            {s["X"]}EditCommand::ReplaceText {{ text }} => match <{s["Snapshot"]} as store::ArtifactDsl>::parse_dsl(text) {{\n'
               f'                Ok(snapshot) => Ok(Emit::mutations(vec![{s["Mutation"]}::SetSnapshot {{ snapshot }}])),\n'
               f'                Err(_) => Ok(Emit::default()),\n'
               f'            }},')
    elif kit == "media":
        cmd_decl = "    SeekMedia { position_ms: u64 },"
        arm = f'            {s["X"]}EditCommand::SeekMedia {{ position_ms: _ }} => Ok(Emit::default()),'
    else:
        raise ValueError(kit)

    doc_note = {
        "image": "Emits the frozen `set-pixel-region` action onto the artifact's own whole-raster replace mutation.",
        "image_indexed": "Emits the frozen `set-pixel-region` action onto the artifact's own frame/image pixel-index replace mutation (index fixed at 0 — a genuine per-region patch is not declared in this format's schema).",
        "image_svg": "SVG has no pixel buffer: `set-pixel-region` replaces the whole vector snapshot via the artifact's own DSL text round-trip (`parse_dsl`/`SetSnapshot`), the closest real mutation this format declares — not a pixel edit.",
        "text": "Emits the frozen `replace-text` action: the incoming text is the artifact's own DSL text envelope (`print_dsl`/`parse_dsl`), round-tripped into a whole-document `SetSnapshot`.",
        "media": "`seek-media` is declared (the frozen `MediaWindowKit` editable action) but intentionally performs no document mutation — playback position is host-side ephemeral transport state, not persisted document content this format's schema models.",
    }[kit]

    kit_ty_name = {"image": "ImageWindowKit", "image_indexed": "ImageWindowKit", "image_svg": "ImageWindowKit", "text": "TextWindowKit", "media": "MediaWindowKit"}[kit]
    return f'''//! ✏️ `{s["kind"]}` editor ({s["subset_mod"]}) — `ArtifactEditor` surface built on the frozen
//! `{kit_ty_name}` window kit (ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.6).
//! {doc_note}
//! MUST NOT be reached by the sibling `viewer` module (`policyViewerPurityBreaches`).

use crate::artifacts::{s["kind"]}::{{{s["dialect_const"]}, {s["doc_schema_const"]}}};
use {s["artifact_mod"]}::schema::mutations::{s["Mutation"]};
use {s["artifact_mod"]}::schema::snapshot::{s["Snapshot"]};
use crate::editor::{s["subset_key"]}::modes::edit;
use crate::editor::{s["subset_key"]}::modes::edit::windows::main;
use semio_framework_plugin::{{ArtifactEditor, ArtifactView, ConfigView, Dialect, DraftView, Editor, Emit, Fault, Label, NoConfig, NoConfigMutation, NoDraft, NoDraftMutation, NoPresence, NoPresenceMutation, NoTransient, NoTransientMutation, UiNode}};
use serde::{{Deserialize, Serialize}};
use store::EngineHandles;

//#region 🔖️Command
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum {s["X"]}EditCommand {{
{cmd_decl}
}}

impl protocol::OpBinary for {s["X"]}EditCommand {{
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {{
        serde_json::to_vec(self).map_err(|error| protocol::ProtocolError::Malformed {{ what: "{s["subset_key"]}-edit-command", offset: 0, detail: error.to_string() }})
    }}
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {{
        serde_json::from_slice(bytes).map_err(|error| protocol::ProtocolError::Malformed {{ what: "{s["subset_key"]}-edit-command", offset: 0, detail: error.to_string() }})
    }}
}}
//#endregion 🔖️Command

//#region 🔖️Editor
#[derive(Default, Clone, Copy)]
pub struct {s["X"]}Editor;

impl ArtifactEditor for {s["X"]}Editor {{
    type Snapshot = {s["Snapshot"]};
    type Mutation = {s["Mutation"]};
    type Config = NoConfig;
    type ConfigMutation = NoConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = NoPresence;
    type PresenceMutation = NoPresenceMutation;
    type Transient = NoTransient;
    type TransientMutation = NoTransientMutation;
    type Command = {s["X"]}EditCommand;

    const DIALECT: Dialect = {s["dialect_const"]};
    const DOCUMENT_SCHEMA: &'static str = {s["doc_schema_const"]};

    fn initial_snapshot() -> Self::Snapshot {{
        {s["Snapshot"]}::default()
    }}

    fn handle(
        command: &Self::Command,
        _doc: &ArtifactView<'_, Self::Snapshot>,
        _cfg: &ConfigView<'_, Self::Config>,
        _interaction: &semio_framework_plugin::app::InteractionView<'_>,
        _draft: &DraftView<'_, Self::Draft>,
        _engines: &EngineHandles,
    ) -> Result<Emit<Self::Mutation, Self::ConfigMutation, Self::DraftMutation>, Fault> {{
        match command {{
{arm}
        }}
    }}

    fn render(body_key: &str, doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>) -> UiNode {{
        match body_key {{
            main::BODY_KEY => main::render(doc.snapshot),
            _ => semio_framework_plugin::ui_text(Label::data(format!("Unknown body: {{body_key}}"))),
        }}
    }}
}}
//#endregion 🔖️Editor

//#region 🔖️Manifest
pub fn create_{s["subset_key"]}_editor() -> semio_framework_plugin::AppDefinition {{
    Editor::builder({s["dialect_const"]})
        .document(["semio", "{s["kind"]}"])
        .icon_id("{ICONS[kit]}")
        .mode_def(edit::definition())
        .default_mode_id(edit::MODE_ID)
        .window_kind_def(main::definition())
        .default_layout(edit::layout())
        .build_definition()
}}
//#endregion 🔖️Manifest

//#region 🧪️Tests
#[cfg(test)]
mod tests {{
    use super::*;

    #[test]
    fn create_editor_builds_a_definition_for_the_editor_role() {{
        let def = create_{s["subset_key"]}_editor();
        assert_eq!(def.role, semio_framework::AppRole::Editor);
        assert_eq!(def.dialect, {s["dialect_const"]}.into());
    }}

    #[test]
    fn editor_dialect_matches_the_artifact_coordinate() {{
        assert_eq!(<{s["X"]}Editor as ArtifactEditor>::DIALECT, {s["dialect_const"]});
    }}
}}
//#endregion 🧪️Tests
'''

def editor_root_ts(s):
    return f'''/** ✏️ `{s["kind"]}` editor ({s["subset_mod"]}) — subset-level typed twin. Mirrors the editor
 * manifest's mode/window vocabulary; no mutation-payload types beyond the window twin re-exported
 * below (this surface's whole command set is the single frozen action its window kit declares). */

export const {s["subset_key"].upper()}_EDITOR_DIALECT = {{ artifactKind: "{s["artifact_kind"]}", standard: "{s["standard"]}", subset: "{s["dsubset"]}" }} as const;

export const {s["subset_key"].upper()}_EDIT_MODE_ID = "edit" as const;

export * from "./🎭️modes/✏️edit/🪟️windows/🪟️main/🟦️component";
'''

def mode_rs(s, role):
    if role == "edit":
        mode_id, label_en, label_de, icon = "edit", "Edit", "Bearbeiten", "pencil"
        mod_path = f'crate::editor::{s["subset_key"]}::modes::edit::windows::main'
    else:
        mode_id, label_en, label_de, icon = "view", "View", "Ansicht", "eye"
        mod_path = f'crate::viewer::{s["subset_key"]}::modes::view::windows::main'
    return f'''//! {"✏️" if role == "edit" else "👁️"} `{s["kind"]}` {role} ({s["subset_mod"]}) — the `{mode_id}` mode: a single
//! full-pane Main window, the only mode this thin surface declares.

use {mod_path} as main;
use semio_framework_plugin::{{LocalizedLabel, ModeDefinition, WindowLayout, WindowLayoutRoot, WindowLayoutStackNode, WindowLayoutWindowNode}};

pub const MODE_ID: &str = "{mode_id}";

//#region 🔖️Definition
pub fn definition() -> ModeDefinition {{
    ModeDefinition {{ id: MODE_ID.into(), label: LocalizedLabel::native("{label_en}", "{label_de}"), icon_id: "{icon}".into(), tools: Vec::new(), layout_id: None, commands: Vec::new() }}
}}

pub fn layout() -> WindowLayout {{
    WindowLayout {{
        root: WindowLayoutRoot::Stack(WindowLayoutStackNode {{
            kind: "stack".into(),
            size: None,
            active_window_kind_id: None,
            children: vec![WindowLayoutWindowNode {{ kind: "window".into(), window_kind_id: main::WINDOW_KIND_ID.into(), title: Some("Main".into()), instance_id: None, template_id: None }}],
        }}),
    }}
}}
//#endregion 🔖️Definition
'''

def window_rs(s, role):
    kit = s["kit"]
    editable = role == "edit"
    if kit in ("image", "image_indexed"):
        kit_ty, view_ty = "ImageWindowKit", "ImageView"
        def_call = "editable_window_kind" if editable else "window_kind"
        if s["has_wh"]:
            w_expr, h_expr = "snapshot.width", "snapshot.height"
        else:
            w_expr, h_expr = "0", "0"
        body = f'''use base64::Engine as _;
use {s["io_mod"]}::io::{s["encode_fn"]};
use {s["artifact_mod"]}::schema::snapshot::{s["Snapshot"]};
use semio_framework_plugin::app::{{{view_ty}, {kit_ty}}};
use semio_framework_plugin::{{UiNode, WindowKindDefinition, WindowKit}};

pub const WINDOW_KIND_ID: &str = {kit_ty}::KIND_ID;
pub const BODY_KEY: &str = {kit_ty}::KIND_ID;

pub fn definition() -> WindowKindDefinition {{
    {kit_ty}::{def_call}()
}}

pub fn render(snapshot: &{s["Snapshot"]}) -> UiNode {{
    {kit_ty}::render(&image_view(snapshot))
}}

fn image_view(snapshot: &{s["Snapshot"]}) -> {view_ty} {{
    let bytes = {s["encode_fn"]}(snapshot).ok().unwrap_or_default();
    {view_ty} {{ width: {w_expr}, height: {h_expr}, mime: "{s["mime"]}".into(), base64: base64::engine::general_purpose::STANDARD.encode(bytes) }}
}}
'''
    elif kit == "image_svg":
        kit_ty, view_ty = "ImageWindowKit", "ImageView"
        def_call = "editable_window_kind" if editable else "window_kind"
        body = f'''use base64::Engine as _;
use {s["artifact_mod"]}::schema::snapshot::write_svg_xml;
use {s["artifact_mod"]}::schema::snapshot::{s["Snapshot"]};
use semio_framework_plugin::app::{{{view_ty}, {kit_ty}}};
use semio_framework_plugin::{{UiNode, WindowKindDefinition, WindowKit}};

pub const WINDOW_KIND_ID: &str = {kit_ty}::KIND_ID;
pub const BODY_KEY: &str = {kit_ty}::KIND_ID;

pub fn definition() -> WindowKindDefinition {{
    {kit_ty}::{def_call}()
}}

pub fn render(snapshot: &{s["Snapshot"]}) -> UiNode {{
    {kit_ty}::render(&image_view(snapshot))
}}

/// 🖼️ SVG has no pixel buffer — the "image" IS its own XML source, base64-wrapped as an
/// `image/svg+xml` data URI so `ImageWindowKit::render` displays it like any other raster.
fn image_view(snapshot: &{s["Snapshot"]}) -> {view_ty} {{
    let xml = write_svg_xml(&snapshot.doc);
    {view_ty} {{ width: 300, height: 150, mime: "{s["mime"]}".into(), base64: base64::engine::general_purpose::STANDARD.encode(xml.as_bytes()) }}
}}
'''
    elif kit == "text":
        kit_ty, view_ty = "TextWindowKit", "TextView"
        def_call = "editable_window_kind" if editable else "window_kind"
        read_only = "false" if editable else "true"
        body = f'''use {s["artifact_mod"]}::schema::snapshot::{s["Snapshot"]};
use semio_framework_plugin::app::{{{view_ty}, {kit_ty}}};
use semio_framework_plugin::{{UiNode, WindowKindDefinition, WindowKit}};
use store::ArtifactDsl;

pub const WINDOW_KIND_ID: &str = {kit_ty}::KIND_ID;
pub const BODY_KEY: &str = {kit_ty}::KIND_ID;

pub fn definition() -> WindowKindDefinition {{
    {kit_ty}::{def_call}()
}}

/// 📝️ The editable text buffer is the artifact's own DSL text envelope (`print_dsl`), not literal
/// markup — the same textual form `parse_dsl` accepts back on `replace-text` (see the sibling root
/// `handle`). Round-trips exactly for any document this format's own grammar can already print.
pub fn render(snapshot: &{s["Snapshot"]}) -> UiNode {{
    {kit_ty}::render(&{view_ty} {{ text: snapshot.print_dsl(), language: Some("{s["kind"]}".into()), read_only: {read_only} }})
}}
'''
    elif kit == "media":
        kit_ty, view_ty = "MediaWindowKit", "MediaView"
        def_call = "editable_window_kind" if editable else "window_kind"
        body = f'''use {s["artifact_mod"]}::schema::snapshot::{s["Snapshot"]};
use semio_framework_plugin::app::{{MediaKind, {view_ty}, {kit_ty}}};
use semio_framework_plugin::{{UiNode, WindowKindDefinition, WindowKit}};

pub const WINDOW_KIND_ID: &str = {kit_ty}::KIND_ID;
pub const BODY_KEY: &str = {kit_ty}::KIND_ID;

pub fn definition() -> WindowKindDefinition {{
    {kit_ty}::{def_call}()
}}

/// 🎬️ Duration/position stay at the kit's zero defaults — this format's decoded snapshot does not
/// model a playable transport position yet (thin v1: the kit's own transport chrome is real, the
/// per-document duration/position feed is a documented follow-up, not invented here).
pub fn render(_snapshot: &{s["Snapshot"]}) -> UiNode {{
    {kit_ty}::render(&{view_ty} {{ duration_ms: 0, position_ms: 0, kind: MediaKind::{s["media_kind"]} }})
}}
'''
    else:
        raise ValueError(kit)

    header = f'''//! {"✏️" if editable else "👁️"} `{s["kind"]}` {role} ({s["subset_mod"]}) — Main window: real `{kit_ty}`
//! render of the current document{" (editable variant)" if editable else " (read-only)"}.
'''
    tests = f'''
#[cfg(test)]
mod tests {{
    use super::*;

    #[test]
    fn definition_uses_the_frozen_window_kit_kind_id() {{
        let def = definition();
        assert_eq!(def.id, WINDOW_KIND_ID);
    }}

    #[test]
    fn render_produces_a_scene_node_for_the_default_document() {{
        let document = {s["Snapshot"]}::default();
        let _node = render(&document);
    }}
}}
'''
    return header + "\n" + body + tests

def window_ts(s, role):
    kit_id = {"image": "framework.window.image", "image_indexed": "framework.window.image", "image_svg": "framework.window.image", "text": "framework.window.text", "media": "framework.window.media"}[s["kit"]]
    tag = "EDIT" if role == "edit" else "VIEW"
    return f'''/** {"✏️" if role == "edit" else "👁️"} `{s["kind"]}` {role} ({s["subset_mod"]}) — Main window: typed twin of
 * `🦀️component.rs`'s view-model, mirroring the frozen `{kit_id}` window kit's own payload shape. */

export const {s["X"].upper()}_{tag}_WINDOW_KIND_ID = "{kit_id}" as const;
export const {s["X"].upper()}_{tag}_BODY_KEY = "{kit_id}" as const;
'''

def viewer_root_rs(s):
    return f'''//! 👁️ `{s["kind"]}` viewer ({s["subset_mod"]}) — the read-only counterpart of `✏️editor` for this
//! subset (ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.2). `{s["X"]}Viewer`
//! implements `ArtifactViewer`, never `ArtifactEditor`/`ArtifactApp` — `ViewerApp<{s["X"]}Viewer>` is
//! the sole runtime adapter, so this file can never structurally emit an artifact or draft mutation.
//! MUST NOT import anything from the sibling `editor` module (`policyViewerPurityBreaches`).

use crate::artifacts::{s["kind"]}::{{{s["dialect_const"]}, {s["doc_schema_const"]}}};
use {s["artifact_mod"]}::schema::mutations::{s["Mutation"]};
use {s["artifact_mod"]}::schema::snapshot::{s["Snapshot"]};
use crate::viewer::{s["subset_key"]}::modes::view;
use crate::viewer::{s["subset_key"]}::modes::view::windows::main;
use semio_framework_plugin::{{ArtifactView, ArtifactViewer, ConfigView, Dialect, Fault, Label, NoConfig, NoConfigMutation, NoPresence, NoPresenceMutation, NoTransient, NoTransientMutation, UiNode, ViewEmit, Viewer}};
use store::EngineHandles;

//#region 🔖️Command
/// 👁️ The viewer declares no actions, so its typed command channel has exactly one inert variant.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum {s["X"]}ViewCommand {{
    Noop,
}}

impl protocol::OpBinary for {s["X"]}ViewCommand {{
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {{
        Ok(Vec::new())
    }}
    fn decode_op(_bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {{
        Ok({s["X"]}ViewCommand::Noop)
    }}
}}
//#endregion 🔖️Command

//#region 🔖️Viewer
#[derive(Default, Clone, Copy)]
pub struct {s["X"]}Viewer;

impl ArtifactViewer for {s["X"]}Viewer {{
    type Snapshot = {s["Snapshot"]};
    type Mutation = {s["Mutation"]};
    type Config = NoConfig;
    type ConfigMutation = NoConfigMutation;
    type Presence = NoPresence;
    type PresenceMutation = NoPresenceMutation;
    type Transient = NoTransient;
    type TransientMutation = NoTransientMutation;
    type Command = {s["X"]}ViewCommand;

    const DIALECT: Dialect = {s["dialect_const"]};
    const DOCUMENT_SCHEMA: &'static str = {s["doc_schema_const"]};

    fn initial_snapshot() -> Self::Snapshot {{
        {s["Snapshot"]}::default()
    }}

    fn handle(_command: &Self::Command, _doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>, _interaction: &semio_framework_plugin::app::InteractionView<'_>, _engines: &EngineHandles) -> Result<ViewEmit<Self::ConfigMutation>, Fault> {{
        Ok(ViewEmit::default())
    }}

    fn render(body_key: &str, doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>) -> UiNode {{
        match body_key {{
            main::BODY_KEY => main::render(doc.snapshot),
            _ => semio_framework_plugin::ui_text(Label::data(format!("Unknown body: {{body_key}}"))),
        }}
    }}
}}
//#endregion 🔖️Viewer

//#region 🔖️Manifest
pub fn create_{s["subset_key"]}_viewer() -> semio_framework_plugin::AppDefinition {{
    Viewer::builder({s["dialect_const"]})
        .document(["semio", "{s["kind"]}"])
        .icon_id("{ICONS[s["kit"]]}")
        .mode_def(view::definition())
        .default_mode_id(view::MODE_ID)
        .window_kind_def(main::definition())
        .default_layout(view::layout())
        .build_definition()
}}
//#endregion 🔖️Manifest

//#region 🧪️Tests
#[cfg(test)]
mod tests {{
    use super::*;

    #[test]
    fn create_viewer_builds_a_definition_for_the_viewer_role() {{
        let def = create_{s["subset_key"]}_viewer();
        assert_eq!(def.role, semio_framework::AppRole::Viewer);
        assert_eq!(def.dialect, {s["dialect_const"]}.into());
    }}

    #[test]
    fn viewer_dialect_matches_the_artifact_coordinate() {{
        assert_eq!(<{s["X"]}Viewer as ArtifactViewer>::DIALECT, {s["dialect_const"]});
    }}
}}
//#endregion 🧪️Tests
'''

def viewer_root_ts(s):
    return f'''/** 👁️ `{s["kind"]}` viewer ({s["subset_mod"]}) — read-only counterpart of `✏️editor/🟦️component.ts`:
 * mirrors the viewer manifest's mode/window vocabulary, no mutation-shaped exports. */

export const {s["subset_key"].upper()}_VIEWER_DIALECT = {{ artifactKind: "{s["artifact_kind"]}", standard: "{s["standard"]}", subset: "{s["dsubset"]}" }} as const;

export const {s["subset_key"].upper()}_VIEW_MODE_ID = "view" as const;

export * from "./🎭️modes/👁️view/🪟️windows/🪟️main/🟦️component";
'''

# ------------------------------------------------------------------------------------------------
def emit_all():
    for s in SUBSETS:
        base = s["base"]
        # editor
        ed = p(base, "✏️editor")
        write(p(ed, "🦀️component.rs"), editor_root_rs(s))
        write(p(ed, "🟦️component.ts"), editor_root_ts(s))
        write(p(ed, "🎭️modes", "✏️edit", "🦀️component.rs"), mode_rs(s, "edit"))
        write(p(ed, "🎭️modes", "✏️edit", "🪟️windows", "🪟️main", "🦀️component.rs"), window_rs(s, "edit"))
        write(p(ed, "🎭️modes", "✏️edit", "🪟️windows", "🪟️main", "🟦️component.ts"), window_ts(s, "edit"))
        # viewer
        vw = p(base, "👁️viewer")
        write(p(vw, "🦀️component.rs"), viewer_root_rs(s))
        write(p(vw, "🟦️component.ts"), viewer_root_ts(s))
        write(p(vw, "🎭️modes", "👁️view", "🦀️component.rs"), mode_rs(s, "view"))
        write(p(vw, "🎭️modes", "👁️view", "🪟️windows", "🪟️main", "🦀️component.rs"), window_rs(s, "view"))
        write(p(vw, "🎭️modes", "👁️view", "🪟️windows", "🪟️main", "🟦️component.ts"), window_ts(s, "view"))
    print(f"Emitted {len(SUBSETS) * 10} files across {len(SUBSETS)} subsets.")

if __name__ == "__main__":
    emit_all()
