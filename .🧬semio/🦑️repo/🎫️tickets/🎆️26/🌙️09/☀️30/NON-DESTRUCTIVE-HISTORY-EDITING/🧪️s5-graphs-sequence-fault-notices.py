"""📢️ S5-GRAPHS-WIRES — names every refusal of the sequence editor and declares its localized notice (design §20.12).

The fault-notice gate lists 88 findings in the sequence plugin: 85 anonymous `Fault::from(text)` refusals (code
`app.message`, English only), two coded refusals without a notice row and one two-segment code. This wave routes every
refusal through ONE helper, `sequence_fault(code, detail)`, declares `sequence_fault_notices()` (code → en + de) and
publishes it through `ArtifactEditor::fault_notices`. The English slug each site carried stays as the fault's detail
message, so logs keep their precision while the person reads the notice.

Codes group refusals by what the person can do about them, not by which internal buffer refused: sites that differ only
in the retained step that noticed the same condition share one code. A retained step that receives a command of another
route raises the framework's `app.command.tool-mismatch`, like the dispatcher already does.

    python3 🧪️s5-graphs-sequence-fault-notices.py --check   # report, write nothing
    python3 🧪️s5-graphs-sequence-fault-notices.py --apply

Fails closed: six explicit files, every literal `Fault::from("…")` must be in the slug table, every variable-text site
is an exact string with an expected count, the per-file site counts are pinned, and nothing is written unless all six
files convert and no `Fault::from(` survives outside `#[cfg(test)]` mounts.
"""

import pathlib
import re
import sys

ROOT = pathlib.Path(__file__).resolve().parents[7]
SUBSET = ROOT / "✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any"
EDITOR = SUBSET / "✏️editor/🦀️.rs"
VIEWER = SUBSET / "👁️viewer/🦀️.rs"
CONFIG = SUBSET / "✏️editor/🎭️modes/✏️edit/🪟️windows/📽️main/🎚️config/🦀️.rs"
TRANSIENT = SUBSET / "✏️editor/🎭️modes/✏️edit/🪟️windows/📜️script/🫧️transient/🦀️.rs"
EXAMPLE = SUBSET / "✏️editor/🎮️commands/📚️example/🦀️.rs"
NODE_GRAPH = SUBSET / "✏️editor/🎮️commands/🕸️node-graph/🦀️.rs"
HELPER = "crate::editor::sequence::sequence_fault"
TOOL_MISMATCH = "app.command.tool-mismatch"

NOTICES = [
    ("sequence.content.dialect", "The sequence's step graph is not a Semio flow graph.", "Der Schrittgraph der Sequenz ist kein Semio-Flussgraph."),
    ("sequence.content.unavailable", "The sequence's step graph is not loaded yet.", "Der Schrittgraph der Sequenz ist noch nicht geladen."),
    ("sequence.window.unavailable", "This action needs an open sequence window.", "Diese Aktion braucht ein geöffnetes Sequenzfenster."),
    ("sequence.editor.capacity", "This sequence is too large for this action.", "Diese Sequenz ist für diese Aktion zu groß."),
    ("sequence.run.capacity", "The run produced more than the editor can hold; reduce repeats or effects.", "Der Lauf hat mehr erzeugt, als der Editor fassen kann; Wiederholungen oder Effekte reduzieren."),
    ("sequence.run.step-missing", "The run reached a step that no longer exists.", "Der Lauf hat einen Schritt erreicht, der nicht mehr existiert."),
    ("sequence.resume.invalid", "The interrupted action could not be resumed; start it again.", "Die unterbrochene Aktion konnte nicht fortgesetzt werden; bitte erneut starten."),
    ("sequence.publication.lane", "This change could not be recorded in the sequence's step graph.", "Diese Änderung konnte nicht im Schrittgraph der Sequenz aufgezeichnet werden."),
    ("sequence.retained.artifact-command", "This edit does not fit the sequence editor's current step.", "Diese Bearbeitung passt nicht zum aktuellen Schritt des Sequenzeditors."),
    ("sequence.retained.config-command", "This view change does not fit the sequence editor's current step.", "Diese Ansichtsänderung passt nicht zum aktuellen Schritt des Sequenzeditors."),
    ("sequence.retained.example-command", "This example could not be loaded into the sequence editor.", "Dieses Beispiel konnte nicht in den Sequenzeditor geladen werden."),
    ("sequence.retained.close", "The sequence editor could not finish closing this action.", "Der Sequenzeditor konnte das Schließen dieser Aktion nicht abschließen."),
    ("sequence.node-graph.malformed", "The node graph edit is malformed.", "Die Knotengraph-Bearbeitung ist fehlerhaft."),
    ("sequence.node-graph.unsupported", "A sequence has no sliders and no variadic ports.", "Eine Sequenz hat keine Schieberegler und keine variadischen Anschlüsse."),
    ("sequence.import-media.missing", "Choose a media file to import.", "Eine Mediendatei zum Importieren auswählen."),
    ("sequence.import-media.undecoded", "The media could not be decoded for import.", "Das Medium konnte für den Import nicht dekodiert werden."),
    ("sequence.viewport.camera", "The viewport change needs a valid camera.", "Die Ansichtsänderung braucht eine gültige Kamera."),
    ("sequence.action.unhandled", "This action is not available in the sequence editor.", "Diese Aktion ist im Sequenzeditor nicht verfügbar."),
    ("sequence.example.unparsable", "The bundled sequence example could not be read.", "Das mitgelieferte Sequenzbeispiel konnte nicht gelesen werden."),
    ("sequence.child.projection", "The sequence document's step graph could not be restored.", "Der Schrittgraph des Sequenzdokuments konnte nicht wiederhergestellt werden."),
]

GROUPS = {
    "sequence.content.dialect": ["sequence-content-child-dialect-required", "sequence-content-child-dialect-mismatch"],
    "sequence.content.unavailable": ["sequence-content-child-context-required", "sequence-retained-scene-required", "sequence-node-graph-target", "sequence-node-graph-base", "sequence-run-registry"],
    "sequence.window.unavailable": [
        "sequence-window-context-required", "sequence-window-view-required", "sequence-script-window-view-required", "sequence-main-window-required",
        "sequence-main-window-kind-required", "sequence-script-window-required", "sequence-script-window-kind-required", "sequence-window-stale",
    ],
    "sequence.editor.capacity": [
        "sequence-retained-scene-capacity", "sequence-retained-selection-capacity", "sequence-retained-artifact-output-capacity", "sequence-retained-artifact-output-bytes",
        "sequence-retained-artifact-checkpoint-capacity", "sequence-retained-checkpoint-capacity", "sequence-reorganize-capacity", "sequence-node-graph-delete-capacity",
        "sequence-node-graph-output-items", "sequence-persistent-progress-capacity", "sequence-persistent-output-bytes", "sequence-persistent-checkpoint-capacity",
    ],
    "sequence.run.capacity": ["sequence-run-scene-capacity", "sequence-run-result-capacity", "sequence-run-while-capacity", "sequence-run-effect-capacity", "sequence-run-repeat-capacity"],
    "sequence.run.step-missing": ["sequence-run-step-index"],
    "sequence.resume.invalid": [
        "sequence-retained-artifact-checkpoint-invalid", "sequence-retained-artifact-checkpoint-cursor", "sequence-retained-artifact-checkpoint-identity",
        "sequence-retained-artifact-checkpoint-owner-mismatch", "sequence-retained-checkpoint-invalid", "sequence-retained-checkpoint-cursor", "sequence-retained-checkpoint-identity",
        "sequence-retained-checkpoint-owner-mismatch", "sequence-persistent-checkpoint-invalid", "sequence-persistent-checkpoint-cursor", "sequence-persistent-checkpoint-identity",
        "sequence-persistent-checkpoint-owner", "sequence-persistent-restore-live-workspace", "sequence-persistent-replay-overrun", "sequence-run-replay-overrun",
    ],
    "sequence.publication.lane": ["sequence-retained-artifact-publication-lane", "sequence-persistent-publication-lane"],
    "sequence.retained.artifact-command": ["sequence-retained-artifact-envelope"],
    TOOL_MISMATCH: ["sequence-retained-artifact-route-mismatch", "sequence-node-graph-route", "sequence-window-route-rejected", "sequence-example-route-rejected"],
    "sequence.node-graph.malformed": ["sequence-node-graph-bytes", "sequence-node-graph-json", "sequence-node-graph-items", "sequence nodeGraphEdit operations must be a JSON array"],
    "sequence.node-graph.unsupported": ["sequence-node-graph-row-unsupported", "sequence nodeGraphEdit refusal: a sequence has no sliders and no variadic ports"],
    "sequence.import-media.missing": ["sequence import-media requires media input"],
    "sequence.import-media.undecoded": ["sequence import-media admits a decoded media value, never a wire payload"],
    "sequence.viewport.camera": ["sequence setViewport requires a camera"],
}
SLUGS = {slug: code for code, slugs in GROUPS.items() for slug in slugs}

EXACT = {
    EDITOR: [
        ('Fault::from(format!("invalid sequence setViewport camera: {error}"))', 'sequence_fault("sequence.viewport.camera", format!("invalid sequence setViewport camera: {error}"))', 1),
        ('Fault::from(format!("sequence: unhandled action id {other}"))', 'sequence_fault("sequence.action.unhandled", format!("sequence: unhandled action id {other}"))', 1),
        ("Fault::from(error.message)", 'sequence_fault("sequence.retained.close", error.message)', 1),
        ('FaultCode::new("sequence.child-projection")', 'FaultCode::new("sequence.child.projection")', 1),
    ],
    VIEWER: [('FaultCode::new("sequence.child-projection")', 'FaultCode::new("sequence.child.projection")', 1)],
    CONFIG: [],
    TRANSIENT: [],
    EXAMPLE: [('Fault::from(format!("sequence example {} is not parsable: {error:?}", crate::examples::demo::ID))', HELPER + '("sequence.example.unparsable", format!("sequence example {} is not parsable: {error:?}", crate::examples::demo::ID))', 1)],
    NODE_GRAPH: [('Fault::from(format!("sequence nodeGraphEdit refusal: {reason}"))', HELPER + '("sequence.node-graph.malformed", format!("sequence nodeGraphEdit refusal: {reason}"))', 1)],
}
LITERAL_SITES = {EDITOR: 73, VIEWER: 0, CONFIG: 3, TRANSIENT: 3, EXAMPLE: 0, NODE_GRAPH: 2}
LITERAL = re.compile(r'(?:semio_framework_plugin::)?Fault::from\("([^"\\]*)"\)')

ERRORS_END = "//#endregion ⚠️ Errors\n"
PROJECTION_FN = """    fn child_restore_projection(snapshot: &Self::Snapshot) -> Result<store::ChildRestoreProjection<'_>, Fault> {
        store::ChildRestoreProjection::from_snapshot(snapshot).map_err(|error| Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("sequence.child.projection"), error.to_string()))
    }
"""
NOTICES_FN = """
    /// 🔔️ The localized notices of the editor's own refusal codes (design §20.12).
    fn fault_notices() -> &'static [(&'static str, LocalizedLabel)] {
        sequence_fault_notices()
    }
"""


def declarations() -> str:
    rows = "\n".join('                ("%s", LocalizedLabel::native("%s", "%s")),' % row for row in NOTICES)
    return (
        "/// 🚧️ One named refusal of the sequence editor: `code` is a row of [`sequence_fault_notices`] (or a framework code), `detail`\n"
        "/// the English diagnostic the logs keep; the person reads the notice.\n"
        "pub(crate) fn sequence_fault(code: &'static str, detail: impl Into<String>) -> Fault {\n"
        "    Fault::new(semio_framework_plugin::FaultOrigin::App, code, detail)\n"
        "}\n"
        "\n"
        "/// 📢️ The app fault notices of the sequence editor (`code → {en, de}`): every refusal its retained steps, windows and\n"
        "/// commands raise through [`sequence_fault`].\n"
        "pub fn sequence_fault_notices() -> &'static [(&'static str, LocalizedLabel)] {\n"
        "    static NOTICES: std::sync::OnceLock<Vec<(&'static str, LocalizedLabel)>> = std::sync::OnceLock::new();\n"
        "    NOTICES\n"
        "        .get_or_init(|| {\n"
        "            vec![\n" + rows + "\n"
        "            ]\n"
        "        })\n"
        "        .as_slice()\n"
        "}\n"
        "\n"
    )


def convert(path: pathlib.Path, source: str, refusals: list) -> str:
    name = path.relative_to(SUBSET).as_posix()
    helper = "sequence_fault" if path == EDITOR else HELPER
    found = LITERAL.findall(source)
    if len(found) != LITERAL_SITES[path]:
        refusals.append("%s: %d literal Fault::from sites, expected %d" % (name, len(found), LITERAL_SITES[path]))
    for slug in found:
        if slug not in SLUGS:
            refusals.append("%s: unmapped refusal %r" % (name, slug))
    text = LITERAL.sub(lambda match: '%s("%s", "%s")' % (helper, SLUGS.get(match.group(1), "?"), match.group(1)), source)
    for before, after, count in EXACT[path]:
        if text.count(before) != count:
            refusals.append("%s: %r occurs %d times, expected %d" % (name, before[:60], text.count(before), count))
        text = text.replace(before, after)
    if path == EDITOR:
        for anchor, what in ((ERRORS_END, "errors region end"), (PROJECTION_FN, "child_restore_projection")):
            if text.count(anchor) != 1:
                refusals.append("%s: anchor %s occurs %d times, expected 1" % (name, what, text.count(anchor)))
        if "fn sequence_fault" in source or "fn fault_notices" in source:
            refusals.append("%s: already carries the helper or the notices" % name)
        text = text.replace(ERRORS_END, declarations() + ERRORS_END).replace(PROJECTION_FN, PROJECTION_FN + NOTICES_FN)
    if "Fault::from(" in text:
        refusals.append("%s: %d Fault::from( site(s) survive" % (name, text.count("Fault::from(")))
    return text


def main() -> None:
    mode = sys.argv[1] if len(sys.argv) == 2 else ""
    if mode not in ("--check", "--apply"):
        raise SystemExit("usage: 🧪️s5-graphs-sequence-fault-notices.py --check | --apply")
    declared = {code for code, _, _ in NOTICES}
    raised = set(GROUPS) | {"sequence.action.unhandled", "sequence.retained.close", "sequence.example.unparsable", "sequence.child.projection", "sequence.retained.config-command", "sequence.retained.example-command"}
    if raised - declared != {TOOL_MISMATCH} or declared - raised or len(declared) != len(NOTICES):
        raise SystemExit("[sequence-fault-notices] REFUSED: table and raise sites disagree: unlabelled %s, unraised %s" % (sorted(raised - declared - {TOOL_MISMATCH}), sorted(declared - raised)))
    if not SUBSET.is_dir():
        raise SystemExit("[sequence-fault-notices] REFUSED: sequence subset root is missing")
    refusals, converted = [], {}
    for path in EXACT:
        if not path.is_file():
            refusals.append("%s: missing" % path)
            continue
        converted[path] = convert(path, path.read_text(), refusals)
    if refusals:
        raise SystemExit("[sequence-fault-notices] REFUSED, nothing written:\n  " + "\n  ".join(refusals))
    if mode == "--apply":
        for path, text in converted.items():
            path.write_text(text)
    sites = sum(LITERAL_SITES.values()) + sum(count for rows in EXACT.values() for _, _, count in rows)
    print("[sequence-fault-notices] %s: %d site(s) in %d file(s), %d notice row(s), %d slug(s) mapped" % ("applied" if mode == "--apply" else "would apply", sites, len(converted), len(NOTICES), len(SLUGS)))


main()
