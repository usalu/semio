"""🔢️ S18 §15 (C13 relay): draw's canvas engagement speaks the human's locale — the layer-count status ("N layers · 0 selected")
and the input placeholder ("Layer name") come from `DrawingPlayLabels` (en + de, CLDR one/other), the manifest's declared
engagement carries no English text, a language-neutral fixture pins every (locale, count) → text, a Rust law pins the label set
and `layer_status` to it, a unit law pins the live engagement, and a TS twin checks each row's plural category against
`Intl.PluralRules` (ICU). Guest-linked (draw crate) → PREPARED for T6/T7; the draw descriptor (`🖍️draw/🔣️.json`) needs its
describe regen in the landing train (the manifest engagement changes).
usage: python3 s18-15-draw-layer-status.py [--dry-run | --revert] [--root <tree>]
(backups of every written file under `.🧬semio/🌐hub/s14-s18-backup/draw-layer-status/`; `--revert` restores them and removes the
created files)"""
import json
import shutil
import sys
from pathlib import Path

ROOT = Path(sys.argv[sys.argv.index("--root") + 1]) if "--root" in sys.argv else Path("/Users/ueli/Documents/semio")
BACKUP = Path("/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-s18-backup/draw-layer-status")
PLUGIN = ROOT / "✏️s/🔌️plugins/🖍️draw"
EDITOR = PLUGIN / "🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor"
TERMS = EDITOR / "🗣️terminology"
SOURCE = EDITOR / "🦀️.rs"
UNIT = EDITOR / "🧪️tests/🔬️unit/🦀️.rs"
LABELS = TERMS / "🦀️.rs"
FIXTURE = TERMS / "🧫️fixtures/🔢️layer-status/🔣️.json"
SCHEMA = TERMS / "🧫️fixtures/🔢️layer-status/🧬️schema/🔣️.json"
RUST_LAW = TERMS / "🧪️tests/🔢️layer-status/🦀️.rs"
TS_LAW = TERMS / "🧪️tests/🔢️layer-status/🟦️.ts"
SCRIPT = PLUGIN / "📦️packages/🟦️typescript/📜️script.ts"
DRY = "--dry-run" in sys.argv
if "--revert" in sys.argv:
    manifest = json.loads((BACKUP / "manifest.json").read_text(encoding="utf-8"))
    for relative in manifest["edited"]:
        shutil.copyfile(BACKUP / "files" / relative, PLUGIN / relative)
        print("restored", relative)
    for relative in manifest["created"]:
        (PLUGIN / relative).unlink(missing_ok=True)
        print("removed", relative)
    raise SystemExit(0)

edits: dict[Path, str] = {}
problems: list[str] = []


def swap(path: Path, old: str, new: str, done: str) -> None:
    text = edits.get(path) or path.read_text(encoding="utf-8")
    if done in text:
        print("already", path.name, done[:60].replace("\n", " "))
        return
    if text.count(old) != 1:
        problems.append(f"anchor count {text.count(old)} in {path.name}: {old[:90]!r}")
        return
    edits[path] = text.replace(old, new)


def create(path: Path, text: str) -> None:
    if path.exists():
        if path.read_text(encoding="utf-8") != text:
            problems.append(f"{path} exists with other content")
        else:
            print("already", path.name)
        return
    edits[path] = text


TEMPLATES = {"en": {"one": "{count} layer · {selected} selected", "other": "{count} layers · {selected} selected"}, "de": {"one": "{count} Ebene · {selected} ausgewählt", "other": "{count} Ebenen · {selected} ausgewählt"}}
PLACEHOLDER = {"en": "Layer name", "de": "Ebenenname"}
rows = []
for locale in ("en", "de"):
    for layers in (0, 1, 2, 21, 101):
        plural = "one" if layers == 1 else "other"
        rows.append({"locale": locale, "layers": layers, "plural": plural, "text": TEMPLATES[locale][plural].replace("{count}", str(layers)).replace("{selected}", "0")})
fixture = {
    "schema": "semio.s.draw.layer-status/v1",
    "description": "The drawing canvas engagement's layer-count status and input placeholder per locale (ticket 26/09/23 S18 §15, C13 relay: the status was English-only). `templates` are the label set's one/other templates (`{count}` = top-level layers, `{selected}` = selected layers, always 0 in this row: selection is framework-owned); every row names the CLDR plural category of its count and the exact text the canvas window shows.",
    "templates": TEMPLATES,
    "placeholder": PLACEHOLDER,
    "rows": rows,
}
schema = {
    "$schema": "http://json-schema.org/draft-07/schema#",
    "$id": "https://json.schemas.assets.semio-tech.com/s/draw/drawing/editor/layer-status.json",
    "$ref": "#/definitions/DrawingLayerStatusV1",
    "definitions": {
        "DrawingLayerStatusV1": {
            "type": "object",
            "additionalProperties": False,
            "required": ["schema", "description", "templates", "placeholder", "rows"],
            "properties": {
                "schema": {"const": "semio.s.draw.layer-status/v1"},
                "description": {"type": "string", "minLength": 1},
                "templates": {"type": "object", "additionalProperties": False, "required": ["en", "de"], "properties": {"en": {"$ref": "#/definitions/templates"}, "de": {"$ref": "#/definitions/templates"}}},
                "placeholder": {"type": "object", "additionalProperties": False, "required": ["en", "de"], "properties": {"en": {"$ref": "#/definitions/text"}, "de": {"$ref": "#/definitions/text"}}},
                "rows": {"type": "array", "minItems": 1, "items": {"$ref": "#/definitions/row"}},
            },
        },
        "text": {"type": "string", "minLength": 1, "maxLength": 128},
        "template": {"type": "string", "minLength": 1, "maxLength": 128, "pattern": "^(?=.*\\{count\\})(?=.*\\{selected\\}).*$"},
        "templates": {"type": "object", "additionalProperties": False, "required": ["one", "other"], "properties": {"one": {"$ref": "#/definitions/template"}, "other": {"$ref": "#/definitions/template"}}},
        "row": {
            "type": "object",
            "additionalProperties": False,
            "required": ["locale", "layers", "plural", "text"],
            "properties": {"locale": {"enum": ["en", "de"]}, "layers": {"type": "integer", "minimum": 0}, "plural": {"enum": ["one", "other"]}, "text": {"$ref": "#/definitions/text"}},
        },
    },
}

swap(LABELS, '''        rotation: native_en "Rotation", native_de "Rotation", reuse_en "Rotation", reuse_de "Rotation";
    }
}
''', '''        rotation: native_en "Rotation", native_de "Rotation", reuse_en "Rotation", reuse_de "Rotation";
        layer_status_one: native_en "{count} layer · {selected} selected", native_de "{count} Ebene · {selected} ausgewählt", reuse_en "{count} layer · {selected} selected", reuse_de "{count} Ebene · {selected} ausgewählt";
        layer_status_other: native_en "{count} layers · {selected} selected", native_de "{count} Ebenen · {selected} ausgewählt", reuse_en "{count} layers · {selected} selected", reuse_de "{count} Ebenen · {selected} ausgewählt";
        layer_name_placeholder: native_en "Layer name", native_de "Ebenenname", reuse_en "Layer name", reuse_de "Ebenenname";
    }
}

impl DrawingPlayLabels {
    /// 🔢️ The canvas window's layer-count status in this label set's locale: the CLDR `one` template for exactly one
    /// top-level layer (en and de), `other` for every other count; selection is framework-owned, so `{selected}` is 0.
    /// Pinned by `🧫️fixtures/🔢️layer-status/🔣️.json`.
    pub fn layer_status(&self, layer_count: usize) -> String {
        let template = if layer_count == 1 { self.layer_status_one } else { self.layer_status_other };
        let count = layer_count.to_string();
        template.fill(&[("count", count.as_str()), ("selected", "0")]).into_string()
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔢️layer-status/🦀️.rs"]
mod layer_status_tests;
''', "fn layer_status(&self, layer_count: usize)")

swap(SOURCE, '''    fn window_engagements(doc: &ArtifactView<'_, DrawingSnapshot>, _cfg: &ConfigView<'_, NoConfig>, view_state: &semio_framework_plugin::ViewModel) -> HashMap<String, WindowEngagement> {
        let Some(window_id) = view_state.window_id.clone() else { return HashMap::new() };
        // 🧮️ The status row reports the LIVE top-level layer count (selection is framework-owned and not
        // visible from this view, so it stays at the framework's interaction domain); it used to be a
        // hard-coded "0 layers · 0 selected" that no load or edit ever moved.
        let layer_count = doc.snapshot.layers.len();
        let engagement = WindowEngagement {
            session_active: Some(false),
            options: None,
            input: Some(WindowEngagementInput {
                id: Some("drawing-canvas-engagement".into()),
                value: Some(String::new()),
                placeholder: Some("Layer name".into()),''', '''    /// 🧮️ The canvas window's engagement in the view's locale: its status row reports the LIVE top-level layer count
    /// (`DrawingPlayLabels::layer_status`; selection is framework-owned and not visible from this view, so it stays at the
    /// framework's interaction domain) — it used to be a hard-coded English "0 layers · 0 selected" that no load or edit
    /// ever moved, then an English-only count (C13 relay, ticket 26/09/23 S18 §15).
    fn window_engagements(doc: &ArtifactView<'_, DrawingSnapshot>, _cfg: &ConfigView<'_, NoConfig>, view_state: &semio_framework_plugin::ViewModel) -> HashMap<String, WindowEngagement> {
        let Some(window_id) = view_state.window_id.clone() else { return HashMap::new() };
        let labels = semio_framework_plugin::resolve_labels::<DrawingPlayLabels>(view_state);
        let layer_count = doc.snapshot.layers.len();
        let engagement = WindowEngagement {
            session_active: Some(false),
            options: None,
            input: Some(WindowEngagementInput {
                id: Some("drawing-canvas-engagement".into()),
                value: Some(String::new()),
                placeholder: Some(labels.layer_name_placeholder.into()),''', "placeholder: Some(labels.layer_name_placeholder.into()),")
swap(SOURCE, '''            status: Some(vec![WindowEngagementStatus { id: "drawing-layer-count".into(), text: format!("{layer_count} layer{} · 0 selected", if layer_count == 1 { "" } else { "s" }) }]),''',
     '''            status: Some(vec![WindowEngagementStatus { id: "drawing-layer-count".into(), text: labels.layer_status(layer_count) }]),''',
     "text: labels.layer_status(layer_count) }]),")
swap(SOURCE, '''            placeholder: Some("Layer name".into()),
            on_change: Some(drawing_manifest_action("engagementInput")),''', '''            placeholder: None,
            on_change: Some(drawing_manifest_action("engagementInput")),''', "            placeholder: None,\n            on_change: Some(drawing_manifest_action(\"engagementInput\")),")
swap(SOURCE, '''        status: Some(vec![WindowEngagementStatus { id: "drawing-layer-count".into(), text: "0 layers · 0 selected".into() }]),''',
     '''        status: None,''', "        control: None,\n        controls: None,\n        status: None,\n        possible_engagements: None,\n    };\n    Editor::builder(crate::DRAWING_DIALECT)")

swap(UNIT, '''        assert_eq!(selected_strokes(&app).await,vec!["path".to_owned()]);assert_eq!(app.snapshot().unwrap(),before);
    }
}
''', '''        assert_eq!(selected_strokes(&app).await,vec!["path".to_owned()]);assert_eq!(app.snapshot().unwrap(),before);
    }
}

/// 🔢️ The canvas engagement a human sees speaks the view's locale: its layer-count status and input placeholder are the
/// resolved label set's (`🗣️terminology/🧫️fixtures/🔢️layer-status`), never English for a German human (C13 relay, ticket
/// 26/09/23 S18 §15).
#[semio_framework_async_macros::async_test]
async fn canvas_engagement_status_and_placeholder_speak_the_view_locale() {
    let mut app = drawing_app().await;
    let layers = app.snapshot().expect("materialize projection").layers.len();
    for locale in [semio_framework_plugin::Locale::En, semio_framework_plugin::Locale::De] {
        let view = ViewModel { locale, window_id: Some(DRAWING_PLAY_WINDOW_CANVAS.into()), window_instances: vec![semio_framework_plugin::ViewWindowInstance { id: DRAWING_PLAY_WINDOW_CANVAS.into(), window_kind_id: DRAWING_PLAY_WINDOW_CANVAS.into() }], ..Default::default() };
        let labels = semio_framework_plugin::resolve_labels::<DrawingPlayLabels>(&view);
        let engagements = app.window_engagements(&view).await;
        let engagement = engagements.get(DRAWING_PLAY_WINDOW_CANVAS).expect("canvas engagement");
        let status: Vec<&str> = engagement.status.iter().flatten().map(|status| status.text.as_str()).collect();
        assert_eq!(status, [labels.layer_status(layers).as_str()], "{}", locale.as_str());
        assert_eq!(engagement.input.as_ref().and_then(|input| input.placeholder.as_deref()), Some(labels.layer_name_placeholder.as_str()), "{}", locale.as_str());
    }
}
''', "async fn canvas_engagement_status_and_placeholder_speak_the_view_locale()")

create(FIXTURE, json.dumps(fixture, indent=2, ensure_ascii=False) + "\n")
create(SCHEMA, json.dumps(schema, indent=2, ensure_ascii=False) + "\n")
create(RUST_LAW, '''//! 🔢️ Draw's layer-count status in every locale (C13 relay, ticket 26/09/23 S18 §15): the label set carries exactly the
//! language-neutral fixture's templates and placeholder on both terminology axes (`🧫️fixtures/🔢️layer-status/🔣️.json`), and
//! `DrawingPlayLabels::layer_status` fills them to every fixture row's text. The TS twin beside this file checks each row's
//! plural category against the platform's CLDR plural rules (`Intl.PluralRules`).

use super::*;

fn fixture() -> serde_json::Value {
    serde_json::from_str(include_str!("../../🧫️fixtures/🔢️layer-status/🔣️.json")).expect("layer-status fixture")
}

/// 🗣️ Both terminology axes of both locales carry the fixture's templates and placeholder verbatim.
#[test]
fn every_axis_carries_the_fixture_templates_and_placeholder() {
    let fixture = fixture();
    for (labels, locale) in [(&DrawingPlayLabels::NATIVE_EN, "en"), (&DrawingPlayLabels::REUSE_EN, "en"), (&DrawingPlayLabels::NATIVE_DE, "de"), (&DrawingPlayLabels::REUSE_DE, "de")] {
        assert_eq!(labels.layer_status_one.as_str(), fixture["templates"][locale]["one"].as_str().expect("one template"), "{locale}");
        assert_eq!(labels.layer_status_other.as_str(), fixture["templates"][locale]["other"].as_str().expect("other template"), "{locale}");
        assert_eq!(labels.layer_name_placeholder.as_str(), fixture["placeholder"][locale].as_str().expect("placeholder"), "{locale}");
    }
}

/// 🔢️ Every fixture row's text is what `layer_status` fills for its locale and layer count.
#[test]
fn layer_status_fills_every_fixture_row_in_its_locale() {
    let fixture = fixture();
    let rows = fixture["rows"].as_array().expect("rows");
    assert!(!rows.is_empty(), "the fixture pins rows");
    for row in rows {
        let locale = row["locale"].as_str().expect("locale");
        let layers = usize::try_from(row["layers"].as_u64().expect("layers")).expect("layer count fits usize");
        let labels = semio_framework_plugin::resolve_labels_for_locale::<DrawingPlayLabels>(locale);
        assert_eq!(labels.layer_status(layers), row["text"].as_str().expect("text"), "{locale} {layers}");
    }
}
''')
create(TS_LAW, '''/** 🔢️ Draw's layer-count status fixture against an independent plural oracle (C13 relay, ticket 26/09/23 S18 §15): Ajv admits
 * the fixture against its schema, and every row's text is its locale's template for the CLDR plural category the platform's
 * `Intl.PluralRules` (ICU) selects for its layer count — so the Rust `DrawingPlayLabels::layer_status`, pinned to the same rows
 * (`🦀️.rs` beside this file), can never pick a wrong plural form. */
import {expect,test} from "bun:test";
import Ajv from "ajv";
import fixture from "../../🧫️fixtures/🔢️layer-status/🔣️.json";
import schema from "../../🧫️fixtures/🔢️layer-status/🧬️schema/🔣️.json";

test("the layer-status fixture is schema-valid (Ajv)",()=>{
  const validate=new Ajv({strict:true}).compile(schema);
  expect(validate(fixture)?null:validate.errors).toBeNull();
});

test("every row's text is its locale's template for the CLDR plural category of its count",()=>{
  for(const row of fixture.rows) {
    const category=new Intl.PluralRules(row.locale).select(row.layers);
    const template=fixture.templates[row.locale as "en"|"de"][category as "one"|"other"];
    expect({row:`${row.locale} ${row.layers}`,category,text:template.replace("{count}",String(row.layers)).replace("{selected}","0")}).toEqual({row:`${row.locale} ${row.layers}`,category:row.plural,text:row.text});
  }
});
''')
swap(SCRIPT, 'join(subset, "✏️editor/📚️examples/🎬️demo-session/🧪️tests/🧩️example/🟦️.ts")]);',
     'join(subset, "✏️editor/📚️examples/🎬️demo-session/🧪️tests/🧩️example/🟦️.ts"), join(subset, "✏️editor/🗣️terminology/🧪️tests/🔢️layer-status/🟦️.ts")]);',
     '"✏️editor/🗣️terminology/🧪️tests/🔢️layer-status/🟦️.ts")')

if problems:
    for problem in problems:
        print("PROBLEM", problem)
    raise SystemExit(1)
if not DRY and edits:
    if BACKUP.exists():
        raise SystemExit(f"backup {BACKUP} exists: revert or remove it first")
    edited = [str(path.relative_to(PLUGIN)) for path in edits if path.exists()]
    for relative in edited:
        (BACKUP / "files" / relative).parent.mkdir(parents=True, exist_ok=True)
        shutil.copyfile(PLUGIN / relative, BACKUP / "files" / relative)
    (BACKUP / "manifest.json").write_text(json.dumps({"edited": edited, "created": [str(path.relative_to(PLUGIN)) for path in edits if not path.exists()]}, ensure_ascii=False, indent=2), encoding="utf-8")
for path, text in edits.items():
    if DRY:
        print("dry", path.relative_to(PLUGIN), len(text))
    else:
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_text(text, encoding="utf-8")
        print("ok", path.relative_to(PLUGIN))
print(f"{len(edits)} changes, 0 problems")
