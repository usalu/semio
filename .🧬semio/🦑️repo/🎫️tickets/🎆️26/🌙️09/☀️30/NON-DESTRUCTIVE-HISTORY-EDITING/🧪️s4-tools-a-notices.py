#!/usr/bin/env python3
"""📢️ S4-TOOLS-A: the history-editing-scope fault notices of draw, note, fem and shooting (design §20.12, S4-GATES routing
table of 03:50) — idempotent. Tool-mismatch refusals move onto the framework code `app.command.tool-mismatch`; every other
scoped code is named (`FaultCode`) and declared with en/de labels in the editor's `fault_notices()`. `--apply` writes.
"""
import sys

REPO = "/Users/ueli/Documents/semio"
APPLY = "--apply" in sys.argv
DRAW = f"{REPO}/✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any"
NOTE = f"{REPO}/✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any"
SHOOT = f"{REPO}/✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any"
FEM2 = f"{REPO}/✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any"
FEM3 = f"{REPO}/✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any"
TOOL = 'FaultCode::new("app.command.tool-mismatch")'
APP = "semio_framework_plugin::FaultOrigin::App"
CODE = "semio_framework_plugin::FaultCode::new"


def table(name, doc, rows):
    entries = "\n".join(f'            ("{code}", semio_framework_ui_locale::LocalizedLabel::native("{en}", "{de}")),' for code, en, de in rows)
    return f'''
/// {doc}
pub fn {name}() -> &'static [(&'static str, semio_framework_ui_locale::LocalizedLabel)] {{
    static NOTICES: std::sync::LazyLock<[(&str, semio_framework_ui_locale::LocalizedLabel); {len(rows)}]> = std::sync::LazyLock::new(|| {{
        [
{entries}
        ]
    }});
    &*NOTICES
}}
'''


def method(doc, call):
    return f'''    /// {doc}
    fn fault_notices() -> &'static [(&'static str, semio_framework_ui_locale::LocalizedLabel)] {{
        {call}
    }}

'''


DRAW_ROWS = [
    ("drawing.gesture.retained-route", "This drawing gesture can only run in an open drawing window.", "Diese Zeichengeste läuft nur in einem geöffneten Zeichenfenster."),
    ("drawing.gesture.closing", "The drawing is closing; the gesture was not applied.", "Die Zeichnung wird geschlossen; die Geste wurde nicht angewendet."),
    ("drawing.gesture.saturated", "Too many drawing gestures are running at once.", "Zu viele Zeichengesten laufen gleichzeitig."),
    ("drawing.gesture.owner", "The drawing gesture ended before it could continue.", "Die Zeichengeste endete, bevor sie fortgesetzt werden konnte."),
    ("drawing.gesture.point-capacity", "The stroke has too many points.", "Der Strich hat zu viele Punkte."),
    ("drawing.gesture.query-owner", "Another drawing gesture owns this query.", "Eine andere Zeichengeste besitzt diese Abfrage."),
    ("drawing.gesture.query-capacity", "Too many shapes match this point.", "Zu viele Formen treffen diesen Punkt."),
    ("drawing.gesture.query-output-capacity", "The interaction produced too many results.", "Die Interaktion erzeugte zu viele Ergebnisse."),
    ("drawing.gesture.command", "This action cannot run during a drawing gesture.", "Diese Aktion kann während einer Zeichengeste nicht ausgeführt werden."),
]
NOTE_ROWS = [
    ("note.ink-tool.provisional", "The ink stroke could not be previewed.", "Der Tintenstrich konnte nicht in der Vorschau angezeigt werden."),
    ("note.ink-gesture.invalid", "The ink gesture is not valid.", "Die Tintengeste ist ungültig."),
    ("note.ink-events.invalid", "The ink input could not be read.", "Die Tinteneingabe konnte nicht gelesen werden."),
    ("note.ink-phase.invalid", "The ink gesture step is not known.", "Der Schritt der Tintengeste ist unbekannt."),
    ("note.retained.transaction", "This change must be made as one step.", "Diese Änderung muss in einem Schritt erfolgen."),
]
FEM_ROWS = [
    ("fem.gumball.phase-unknown", "The transform step is not known.", "Der Transformationsschritt ist unbekannt."),
    ("fem.gumball.transient-context-required", "Dragging the gumball needs an open FEM window.", "Das Ziehen des Gumballs braucht ein geöffnetes FEM-Fenster."),
    ("fem.gumball-flag.window-context-required", "Gumball handles can only be toggled in an open FEM window.", "Gumball-Griffe lassen sich nur in einem geöffneten FEM-Fenster umschalten."),
    ("fem.gumball-flag.window-required", "Focus a model window to toggle gumball handles.", "Ein Modellfenster fokussieren, um Gumball-Griffe umzuschalten."),
    ("fem.gumball-flag.window-stale", "The window of this gumball is no longer open.", "Das Fenster dieses Gumballs ist nicht mehr geöffnet."),
    ("fem.gumball-flag.window-kind", "The transform gumball lives on the model window.", "Der Transformations-Gumball gehört zum Modellfenster."),
    ("fem.canvas.window-required", "This canvas action needs a focused window.", "Diese Zeichenflächenaktion braucht ein fokussiertes Fenster."),
    ("fem.canvas.window-kind", "This canvas action is not available in this window.", "Diese Zeichenflächenaktion ist in diesem Fenster nicht verfügbar."),
]


def named(code, message):
    return f'Fault::new({APP}, {CODE}("{code}"), "{message}")'


EDITS = [
    (f"{DRAW}/✏️editor/🦀️.rs", [
        ('FaultCode::new("drawing.bounded.tool-mismatch")', TOOL),
        ('FaultCode::new("drawing.gesture.tool-mismatch")', TOOL),
        ("impl ArtifactEditor for DrawingPlayApp {\n", "impl ArtifactEditor for DrawingPlayApp {\n" + method("📢️ The localized notices of the retained gesture owner's refusals (design §20.12).", "drawing_fault_notices()")),
    ], table("drawing_fault_notices", "📣️ The en/de notices of every `drawing.gesture.*` refusal code (design §20.12).", DRAW_ROWS)),
    (f"{DRAW}/👁️viewer/🦀️.rs", [('FaultCode::new("drawing.viewer.retained.tool-mismatch")', TOOL)], None),
    (f"{NOTE}/✏️editor/🧵️retained/🦀️.rs", [('FaultCode::new("note.retained.tool-mismatch")', TOOL)], None),
    (f"{NOTE}/✏️editor/🦀️.rs", [
        ("impl ArtifactEditor for NotePlayApp {\n", "impl ArtifactEditor for NotePlayApp {\n" + method("📢️ The localized notices of the ink tool and retained route refusals (design §20.12).", "note_fault_notices()")),
    ], table("note_fault_notices", "📣️ The en/de notices of every `note.ink-*` / `note.retained.transaction` refusal code (design §20.12).", NOTE_ROWS)),
    (f"{SHOOT}/✏️editor/🦀️.rs", [('FaultCode::new("shooting.retained.tool-mismatch")', TOOL)], None),
    (f"{FEM2}/✏️editor/🎮️commands/🧭️gumball/🦀️.rs", [
        ('Fault::from("fem2d.gumball.phase-unknown")', named("fem.gumball.phase-unknown", "unknown gumball phase or abort reason")),
        ('Fault::from("fem2d.gumball.transient-context-required")', named("fem.gumball.transient-context-required", "a streamed gumball phase needs the retained route's transient")),
        ('Fault::from("fem2d.gumball-flag.window-context-required")', named("fem.gumball-flag.window-context-required", "the gumball flag toggle needs a window context")),
        ('Fault::from("fem2d.gumball-flag.window-required")', named("fem.gumball-flag.window-required", "the gumball flag toggle names no window")),
    ], table("fem_fault_notices", "📣️ The en/de notices of the FEM editors' gumball and canvas refusal codes (design §20.12) — fem 2d and fem 3d declare the same table.", FEM_ROWS)),
    (f"{FEM3}/✏️editor/🎮️commands/🧭️gumball/🦀️.rs", [
        ('Fault::from("fem3d.gumball.phase-unknown")', named("fem.gumball.phase-unknown", "unknown gumball phase or abort reason")),
        ('Fault::from("fem3d.gumball.transient-context-required")', named("fem.gumball.transient-context-required", "a streamed gumball phase needs the retained route's transient")),
        ('Fault::from("fem3d.gumball-flag.window-context-required")', named("fem.gumball-flag.window-context-required", "the gumball flag toggle needs a window context")),
        ('Fault::from("fem3d.gumball-flag.window-required")', named("fem.gumball-flag.window-required", "the gumball flag toggle names no window")),
        ('Fault::from("fem3d.gumball-flag.window-stale")', named("fem.gumball-flag.window-stale", "the gumball flag toggle names a closed window")),
        ('Fault::from("fem3d.gumball-flag.window-kind: the transform gumball lives on the model window")', named("fem.gumball-flag.window-kind", "the transform gumball lives on the model window")),
    ], None),
    (f"{FEM2}/✏️editor/🕹️interaction/🖱️canvas-gesture/🦀️.rs", [
        ('Fault::from(format!("{fault}.window-required"))', f'Fault::new({APP}, {CODE}("fem.canvas.window-required"), format!("{{fault}}: the gesture names no window"))'),
        ('Fault::from(format!("{fault}.window-kind"))', f'Fault::new({APP}, {CODE}("fem.canvas.window-kind"), format!("{{fault}}: the addressed window has the wrong kind"))'),
    ], None),
    (f"{FEM2}/✏️editor/🦀️.rs", [
        ("impl ArtifactEditor for Fem2dPlayApp {\n", "impl ArtifactEditor for Fem2dPlayApp {\n" + method("📢️ The localized notices of the gumball and canvas refusals (design §20.12).", "crate::editor::fem2d::commands::gumball::fem_fault_notices()")),
    ], None),
    (f"{FEM3}/✏️editor/🦀️.rs", [
        ("impl ArtifactEditor for Fem3dPlayApp {\n", "impl ArtifactEditor for Fem3dPlayApp {\n" + method("📢️ The localized notices of the gumball and canvas refusals (design §20.12), the fem 2d table.", "semio_s_artifact_fem_2d::editor::fem2d::commands::gumball::fem_fault_notices()")),
    ], None),
]


def main():
    pending = 0
    for path, replacements, appended in EDITS:
        text = open(path, encoding="utf-8").read()
        after = text
        for old, new in replacements:
            if old not in after or (old.startswith("impl ") and new in after):
                continue
            if after.count(old) != 1:
                raise SystemExit(f"[s4-tools-a] {path}: expected one {old!r}, found {after.count(old)}")
            after = after.replace(old, new)
        if appended and appended.strip().split("\n")[1] not in after:
            marker = "\n//#region 🧪️Tests"
            after = after.replace(marker, appended + marker, 1) if marker in after else after.rstrip("\n") + "\n" + appended
        if after != text:
            pending += 1
            print(f"[s4-tools-a] {'wrote' if APPLY else 'would write'} {path[len(REPO) + 1:]}")
            if APPLY:
                open(path, "w", encoding="utf-8").write(after)
    print(f"[s4-tools-a] {pending} files {'written' if APPLY else 'pending'}")


if __name__ == "__main__":
    main()
