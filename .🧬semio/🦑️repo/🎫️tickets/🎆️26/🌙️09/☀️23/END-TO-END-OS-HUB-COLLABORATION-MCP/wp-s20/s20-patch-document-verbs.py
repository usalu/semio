"""📤️ S20 window-3 prepared patch: Export/Import Document become framework-reserved verbs of every app (decision b).

The host half (Export → `.semio-archive`, Import → NEW instance + cancellable `loadDocumentArchive`) already runs in the
React shell behind two os palette commands (`os.exportDocument`/`os.importDocument`). This patch declares the pair
schema-first in the manifest (Rust + TS ids, `document_transfer_action_definitions`: Shell kind, `transfer` category,
Chrome audience, en + de), injects it into every app window through the SDK's framework action chain (beside
`cancellation_action_definition`), lets the shell intercept it like `startIntroduction`, and removes the os commands in the
same edit (one implementation). Laws/lists that enumerate framework verbs learn the two ids (SDK bridge law skip list,
MCP catalog dedup, React + wgpu `FRAMEWORK_RESERVED_ACTION_IDS`); the io-matrix harness presses the rail rows.

Spans agreed with P9 (relay 28 12:4x); `🛂️manifest/🦀️.rs` hunk sits after `start_introduction_action_definition`, outside
P9's `ArgSchema::entity_kinds` / `ActionArgDef::entity_ids` spans.

Usage: python3 s20-patch-document-verbs.py [--dry-run]   (idempotent; every hunk reports applied / would apply / conflict)
"""
import re, sys
from pathlib import Path

ROOT = Path("/Users/ueli/Documents/semio")
DRY = "--dry-run" in sys.argv
MANIFEST_RS = "🧰️framework/🔨️modules/🛂️manifest/🦀️.rs"
MANIFEST_TS = "🧰️framework/🔨️modules/🛂️manifest/🟦️.ts"
SDK = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs"
MCP = "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🗂️catalog/🦀️.rs"
WGPU = "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs"
HELPERS = "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🛠️ShellHelpers/🟦️.tsx"
HOST = "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx"
REACT = "🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/🟦️.tsx"
I18N = "🧰️framework/🔨️modules/🖱️ui/🧱️elements/📚️I18n/🟦️.tsx"
HARNESS = "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🚪️io-matrix/🟦️.ts"

INTRO_RS = '''pub fn start_introduction_action_definition() -> ActionDefinition {
    ActionDefinition { in_palette: false, ..ActionDefinition::resumable_framework(START_INTRODUCTION_ACTION_ID, LocalizedLabel::native("Introduce App", "App vorstellen"), ActionKind::View, "graduation-cap") }
}
'''
PAIR_RS = '''
/// @emoji 📤️ The framework-owned action id of Export Document: injected into every app window and fully
/// shell-intercepted (never forwarded to the program) — the shell writes the document's canonical archive (root
/// envelope, its op log, every owned member) as one `.semio-archive` file, for every artifact kind alike.
pub const EXPORT_ARTIFACT_DOCUMENT_ACTION_ID: &str = "exportArtifactDocument";

/// @emoji 📥️ The framework-owned action id of Import Document, injected beside [`EXPORT_ARTIFACT_DOCUMENT_ACTION_ID`]
/// and shell-intercepted: the shell opens a picked archive as a NEW document of the same program (a cancellable,
/// progress-reporting load of its op log), never overwriting the focused one.
pub const IMPORT_ARTIFACT_DOCUMENT_ACTION_ID: &str = "importArtifactDocument";

/// @emoji 🗃️ The framework-injected Export/Import Document pair: `Shell` verbs in the palette and the `transfer` ribbon
/// category, shell chrome ([`CapabilityAudience::Chrome`]) — agents export through the MCP's own artifact export.
pub fn document_transfer_action_definitions() -> [ActionDefinition; 2] {
    [
        ActionDefinition::resumable_framework(EXPORT_ARTIFACT_DOCUMENT_ACTION_ID, LocalizedLabel::native("Export Document", "Dokument exportieren"), ActionKind::Shell, "export").with_category("transfer").audience(CapabilityAudience::Chrome),
        ActionDefinition::resumable_framework(IMPORT_ARTIFACT_DOCUMENT_ACTION_ID, LocalizedLabel::native("Import Document…", "Dokument importieren…"), ActionKind::Shell, "import").with_category("transfer").audience(CapabilityAudience::Chrome),
    ]
}
'''
INTRO_TS = 'export const START_INTRODUCTION_ACTION_ID = "startIntroduction";\n'
PAIR_TS = '''
/** 📤️ The framework-owned Export Document action id, shell-intercepted in every app — mirrors Rust
 * `EXPORT_ARTIFACT_DOCUMENT_ACTION_ID`. */
export const EXPORT_ARTIFACT_DOCUMENT_ACTION_ID = "exportArtifactDocument";

/** 📥️ The framework-owned Import Document action id, shell-intercepted in every app — mirrors Rust
 * `IMPORT_ARTIFACT_DOCUMENT_ACTION_ID`. */
export const IMPORT_ARTIFACT_DOCUMENT_ACTION_ID = "importArtifactDocument";
'''

HARNESS_OLD_START = "/** 📤️ The framework's Export/Import Document pair as the palette lists it in every editor"
HARNESS_OLD_END = "/** 📤️ Export Document from the palette: the focused program's archive must download and decode. */"
HARNESS_EXPORT_OLD = '''async function exportDocumentArchive(page: Page, recorders: Recorders, locale: "en" | "de", dir: string, prefix: string) {
  const cursor = recorders.downloads.length;
  const noticeCursor = (await recorders.notices()).length;
  const pressed = await pressPaletteCommand(page, DOCUMENT_TRANSFER_COMMANDS.export.item, DOCUMENT_TRANSFER_COMMANDS.export.query[locale]);
'''
HARNESS_EXPORT_NEW = '''async function exportDocumentArchive(page: Page, recorders: Recorders, dir: string, prefix: string) {
  const cursor = recorders.downloads.length;
  const noticeCursor = (await recorders.notices()).length;
  const pressed = (await pressVerb(page, recorders, EXPORT_ARTIFACT_DOCUMENT_ACTION_ID, {}, "", 500)).row;
'''


def hunks() -> list[tuple[str, str, str, str]]:
    return [
        ("manifest rs: document verb ids + definitions", MANIFEST_RS, INTRO_RS, INTRO_RS + PAIR_RS),
        ("manifest ts: document verb ids", MANIFEST_TS, INTRO_TS, INTRO_TS + PAIR_TS),
        ("sdk: inject into every window beside cancellation",
         SDK,
         ".chain([crate::app::operation_progress::cancellation_action_definition()]) {",
         ".chain([crate::app::operation_progress::cancellation_action_definition()]).chain(semio_framework::document_transfer_action_definitions()) {"),
        ("sdk: bridge law skips the shell-intercepted pair",
         SDK,
         '                "setActiveTool",\n                "interactionSelect",\n',
         '                "setActiveTool",\n                semio_framework::EXPORT_ARTIFACT_DOCUMENT_ACTION_ID,\n                semio_framework::IMPORT_ARTIFACT_DOCUMENT_ACTION_ID,\n                "interactionSelect",\n'),
        ("sdk: reserved-id predicate (probe + kind follow, P9 relay 13:0x)",
         SDK,
         "            || matches!(action, REVERT_TO_COMMAND_ACTION_ID | SET_HISTORY_COMMAND_FILTER_ACTION_ID | NOTE_SHELL_COMMAND_ACTION_ID | RECORD_TUTORIAL_ACTION_ID)\n    }\n",
         "            || matches!(action, REVERT_TO_COMMAND_ACTION_ID | SET_HISTORY_COMMAND_FILTER_ACTION_ID | NOTE_SHELL_COMMAND_ACTION_ID | RECORD_TUTORIAL_ACTION_ID)\n            || matches!(action, semio_framework::EXPORT_ARTIFACT_DOCUMENT_ACTION_ID | semio_framework::IMPORT_ARTIFACT_DOCUMENT_ACTION_ID)\n    }\n"),
        ("sdk: reserved kind Shell",
         SDK,
         "        if action == NOTE_SHELL_COMMAND_ACTION_ID {\n            return Some(ActionKind::Shell);\n        }\n",
         "        if action == NOTE_SHELL_COMMAND_ACTION_ID || matches!(action, semio_framework::EXPORT_ARTIFACT_DOCUMENT_ACTION_ID | semio_framework::IMPORT_ARTIFACT_DOCUMENT_ACTION_ID) {\n            return Some(ActionKind::Shell);\n        }\n"),
        ("sdk: guest refuses the shell-owned pair",
         SDK,
         "            if action==CANCEL_TYPED_OPERATION_ACTION_ID {return self.dispatch_operation_cancellation(args,meta).await;}\n",
         "            if action==CANCEL_TYPED_OPERATION_ACTION_ID {return self.dispatch_operation_cancellation(args,meta).await;}\n"
         "            if matches!(action, semio_framework::EXPORT_ARTIFACT_DOCUMENT_ACTION_ID | semio_framework::IMPORT_ARTIFACT_DOCUMENT_ACTION_ID) {\n"
         "                return Err(Fault::new(FaultOrigin::Framework, FaultCode::new(\"framework.document-transfer.shell-owned\"), format!(\"{action} is performed by the shell, never by the program\")));\n"
         "            }\n"),
        ("mcp: framework dedup ids",
         MCP,
         'const FRAMEWORK_VIEW_SHELL_ACTION_IDS: [&str; 5] = ["setActiveUtility", "setActiveTool", "startIntroduction", "setHistoryCommandFilter", "noteShellCommand"];',
         'const FRAMEWORK_VIEW_SHELL_ACTION_IDS: [&str; 7] = ["setActiveUtility", "setActiveTool", "startIntroduction", "setHistoryCommandFilter", "noteShellCommand", manifest::EXPORT_ARTIFACT_DOCUMENT_ACTION_ID, manifest::IMPORT_ARTIFACT_DOCUMENT_ACTION_ID];'),
        ("mcp: dedup doc count", MCP, "/// 🕹️ The 5 framework-injected action ids that carry", "/// 🕹️ The 7 framework-injected action ids that carry"),
        ("mcp: framework action definitions",
         MCP,
         "    actions.push(manifest::note_shell_command_action_definition());\n",
         "    actions.push(manifest::note_shell_command_action_definition());\n    actions.extend(manifest::document_transfer_action_definitions());\n"),
        ("wgpu: reserved ids",
         WGPU,
         'const FRAMEWORK_RESERVED_ACTION_IDS: [&str; 17] = [',
         'const FRAMEWORK_RESERVED_ACTION_IDS: [&str; 19] = ['),
        ("wgpu: reserved ids tail",
         WGPU,
         '    "startTutorial",\n    "setActiveUtility",\n    "setActiveTool",\n];\n',
         '    "startTutorial",\n    "setActiveUtility",\n    "setActiveTool",\n    semio_framework::EXPORT_ARTIFACT_DOCUMENT_ACTION_ID,\n    semio_framework::IMPORT_ARTIFACT_DOCUMENT_ACTION_ID,\n];\n'),
        ("helpers: import ids",
         HELPERS,
         '    type ViewTreeWindowRequest,\n} from "@semio-tech/framework";',
         '    type ViewTreeWindowRequest,\n    EXPORT_ARTIFACT_DOCUMENT_ACTION_ID,\n    IMPORT_ARTIFACT_DOCUMENT_ACTION_ID,\n} from "@semio-tech/framework";'),
        ("helpers: reserved ids",
         HELPERS,
         '  "setActiveUtility",\n  "setActiveTool",\n]);\n',
         '  "setActiveUtility",\n  "setActiveTool",\n  EXPORT_ARTIFACT_DOCUMENT_ACTION_ID,\n  IMPORT_ARTIFACT_DOCUMENT_ACTION_ID,\n]);\n'),
        ("helpers: drop os command ids",
         HELPERS,
         '''//#region 📤️DocumentTransfer
/** 📤️ The framework's Export Document: the focused program's whole document archive — root envelope, its op log and
 * every owned member (`DocumentArchivePack`, `encodeDocumentArchiveBytes`) — as one file, for every artifact kind alike. */
export const EXPORT_DOCUMENT_COMMAND_ID = "os.exportDocument";

/** 📥️ The framework's Import Document: opens a file {@link EXPORT_DOCUMENT_COMMAND_ID} wrote as a NEW document of the same
 * program (`createApp` → `loadDocumentArchive`, progress + cancellation in the Tasks window); the focused document is never
 * overwritten. */
export const IMPORT_DOCUMENT_COMMAND_ID = "os.importDocument";

/** 🗃️ File extension of an exported document archive. */''',
         '''//#region 📤️DocumentTransfer
/** 🗃️ File extension of the document archive the framework's Export Document writes and Import Document reads
 * (`EXPORT_ARTIFACT_DOCUMENT_ACTION_ID`/`IMPORT_ARTIFACT_DOCUMENT_ACTION_ID`, shell-intercepted in `🏛️ShellHost`): the
 * program's root envelope, its op log and every owned member (`DocumentArchivePack`, `encodeDocumentArchiveBytes`). */'''),
        ("helpers: buildOsCommands param",
         HELPERS,
         '''  hasOpenArtifactSurfaces = false,
  /** 📤️ Whether a document program (not the Home or the space host) is focused — gates Export/Import Document. */
  hasDocumentProgram = false,
): CommandDefinition[] {''',
         '''  hasOpenArtifactSurfaces = false,
): CommandDefinition[] {'''),
        ("host: import ids, drop os command ids",
         HOST,
         "  EXPORT_DOCUMENT_COMMAND_ID,\n  IMPORT_DOCUMENT_COMMAND_ID,\n",
         ""),
        ("host: manifest ids",
         HOST,
         "  START_INTRODUCTION_ACTION_ID,\n  START_TUTORIAL_ACTION_ID,\n",
         "  EXPORT_ARTIFACT_DOCUMENT_ACTION_ID,\n  IMPORT_ARTIFACT_DOCUMENT_ACTION_ID,\n  START_INTRODUCTION_ACTION_ID,\n  START_TUTORIAL_ACTION_ID,\n"),
        ("host: drop palette gating",
         HOST,
         '''  /** 📤️ Whether the focused program holds a document of its own (not the Home landing, not the space host) — gates the
   * framework's Export/Import Document. */
  const documentProgramFocused = focusedApp !== null && focusedApp.id !== hostAppId && focusedApp.id !== landingAppId;
''',
         ""),
        ("host: buildOsCommands args", HOST, ", hasOpenArtifactSurfaces, documentProgramFocused),", ", hasOpenArtifactSurfaces),"),
        ("host: buildOsCommands deps", HOST, ", hasOpenArtifactSurfaces, documentProgramFocused],", ", hasOpenArtifactSurfaces],"),
        ("host: drop os command dispatch",
         HOST,
         "      if (isOsCommandAddress(address) && commandId === EXPORT_DOCUMENT_COMMAND_ID) void documentTransferRef.current.exportDocumentArchive();\n"
         "      if (isOsCommandAddress(address) && commandId === IMPORT_DOCUMENT_COMMAND_ID) void documentTransferRef.current.importDocumentArchive();\n",
         ""),
        ("host: intercept the verbs",
         HOST,
         "        // 🎓️ First-run walkthrough (mirrors setActiveUtility below): fully shell-intercepted, resets\n",
         "        if (action.action === EXPORT_ARTIFACT_DOCUMENT_ACTION_ID) {\n"
         "          void documentTransferRef.current.exportDocumentArchive();\n"
         "          return applied();\n"
         "        }\n"
         "        if (action.action === IMPORT_ARTIFACT_DOCUMENT_ACTION_ID) {\n"
         "          void documentTransferRef.current.importDocumentArchive();\n"
         "          return applied();\n"
         "        }\n"
         "        // 🎓️ First-run walkthrough (mirrors setActiveUtility below): fully shell-intercepted, resets\n"),
        ("react: drop de command labels",
         REACT,
         '          exportDocument: { label: { normal: "Dokument exportieren", beginner: "Dokument als Datei sichern" } },\n'
         '          importDocument: { label: { normal: "Dokument importieren…", beginner: "Dokument aus Datei öffnen…" } },\n',
         ""),
        ("react: drop en command labels",
         REACT,
         '          exportDocument: { label: { normal: "Export Document", beginner: "Save Document as File" } },\n'
         '          importDocument: { label: { normal: "Import Document…", beginner: "Open Document from File…" } },\n',
         ""),
        ("i18n: drop command label keys", I18N, "      readonly exportDocument: UiLabelValue;\n      readonly importDocument: UiLabelValue;\n", ""),
        ("harness: rail verb for export", HARNESS, HARNESS_EXPORT_OLD, HARNESS_EXPORT_NEW),
        ("harness: rail verb for import",
         HARNESS,
         "  const pressed = await pressPaletteCommand(page, DOCUMENT_TRANSFER_COMMANDS.import.item, DOCUMENT_TRANSFER_COMMANDS.import.query[locale]);\n",
         "  const pressed = (await pressVerb(page, recorders, IMPORT_ARTIFACT_DOCUMENT_ACTION_ID, {}, \"\", 500)).row;\n"),
        ("harness: export call sites 1",
         HARNESS,
         'const exported = await exportDocumentArchive(page, recorders, locale, dir, "document");',
         'const exported = await exportDocumentArchive(page, recorders, dir, "document");'),
        ("harness: export call sites 2",
         HARNESS,
         'await exportDocumentArchive(page, recorders, locale, dir, "document-reimported")',
         'await exportDocumentArchive(page, recorders, dir, "document-reimported")'),
        ("harness: round trip signature",
         HARNESS,
         'async function runDocumentTransfer(page: Page, recorders: Recorders, locale: "en" | "de", dir: string) {',
         "async function runDocumentTransfer(page: Page, recorders: Recorders, dir: string) {"),
        ("harness: round trip call", HARNESS, "const document = await runDocumentTransfer(page, recorders, locale, dir);", "const document = await runDocumentTransfer(page, recorders, dir);"),
        ("harness: import ids",
         HARNESS,
         'import { decodeDocumentArchiveBytes } from "../../../../🟦️.ts";\n',
         'import { decodeDocumentArchiveBytes } from "../../../../🟦️.ts";\nimport { EXPORT_ARTIFACT_DOCUMENT_ACTION_ID, IMPORT_ARTIFACT_DOCUMENT_ACTION_ID } from "../../../../../../🔨️modules/🛂️manifest/🟦️.ts";\n'),
        ("harness: driveProgram signature",
         HARNESS,
         'matrixPins: MatrixPins, locale: "en" | "de", dir: string) {',
         "matrixPins: MatrixPins, dir: string) {"),
        ("harness: module doc",
         HARNESS,
         "Home landing through the command palette, its first non-empty example seated from the navbar. Every row drives the\n * framework's document pair from the palette — Export Document,",
         "Home landing through the command palette, its first non-empty example seated from the navbar. Every row drives the\n * framework's document pair from its Actions rail rows — Export Document,"),
    ]


def regex_hunks(text_by_file: dict[str, str]) -> list[str]:
    notes = []
    calls = text_by_file[HARNESS].count('matrixPins, options.locale === "de" ? "de" : "en", join(')
    if calls == 2:
        text_by_file[HARNESS] = text_by_file[HARNESS].replace('matrixPins, options.locale === "de" ? "de" : "en", join(', "matrixPins, join(")
        notes.append("apply     harness: driveProgram call sites")
    elif calls == 0:
        notes.append("applied   harness: driveProgram call sites")
    else:
        notes.append(f"CONFLICT  harness: {calls} driveProgram call sites")
    helpers = text_by_file[HELPERS]
    block = re.compile(r"    \.\.\.\(hasDocumentProgram\n.*?\n      : \[\]\),\n", re.S)
    found = block.findall(helpers)
    if len(found) == 1:
        text_by_file[HELPERS] = block.sub("", helpers)
        notes.append("apply     helpers: drop Export/Import os command rows")
    elif "hasDocumentProgram" not in helpers:
        notes.append("applied   helpers: drop Export/Import os command rows")
    else:
        notes.append(f"CONFLICT  helpers: {len(found)} os command blocks")
    harness = text_by_file[HARNESS]
    start, end = harness.find(HARNESS_OLD_START), harness.find(HARNESS_OLD_END)
    if start >= 0 and end > start:
        text_by_file[HARNESS] = harness[:start] + "/** 📤️ Export Document from its rail row: the focused program's archive must download and decode. */" + harness[end + len(HARNESS_OLD_END):]
        notes.append("apply     harness: palette helpers replaced by the rail rows")
    elif start < 0 and "pressPaletteCommand" not in harness:
        notes.append("applied   harness: palette helpers replaced by the rail rows")
    else:
        notes.append("CONFLICT  harness: palette helper section not found intact")
    return notes



#: 🏁️ Set-level landing markers `(repo path, text)` — `None` = the set deletes that file. All present → the set is
#: landed and nothing is applied (per-hunk checks alone cannot see an insert whose text a later codemod reworded).
LANDED = [('🧰️framework/🔨️modules/🛂️manifest/🦀️.rs', 'pub const IMPORT_ARTIFACT_DOCUMENT_ACTION_ID: &str = "importArtifactDocument";')]


def landed_guard() -> bool:
    """🏁️ True when every landing marker is in the tree; a partial landing is a conflict, never a second write."""
    tree = Path("/Users/ueli/Documents/semio")
    present = [(not (tree / rel).exists()) if marker is None else ((tree / rel).exists() and marker in (tree / rel).read_text()) for rel, marker in LANDED]
    if all(present):
        print("landed: every set marker is in the tree — nothing to apply")
        return True
    if any(present):
        raise SystemExit(f"CONFLICT: set partially landed (markers {present}) — nothing written")
    return False


def main() -> None:
    if landed_guard():
        return
    files = {rel for _, rel, _, _ in hunks()}
    text_by_file = {rel: (ROOT / rel).read_text() for rel in files}
    original = dict(text_by_file)
    notes = []
    for name, rel, old, new in hunks():
        text = text_by_file[rel]
        if (new and text.count(new) == 1 and (old not in text or old in new)) or (not new and old not in text):
            notes.append(f"applied   {name}")
        elif old and text.count(old) == 1:
            text_by_file[rel] = text.replace(old, new)
            notes.append(f"apply     {name}")
        else:
            notes.append(f"CONFLICT  {name}: anchor found {text.count(old)}×")
    notes += regex_hunks(text_by_file)
    print("\n".join(notes))
    conflicts = [line for line in notes if line.startswith("CONFLICT")]
    if conflicts:
        raise SystemExit(f"{len(conflicts)} conflict(s): nothing written")
    changed = [rel for rel in files if text_by_file[rel] != original[rel]]
    if not DRY:
        for rel in changed:
            (ROOT / rel).write_text(text_by_file[rel])
    print(f"{'dry-run' if DRY else 'applied'}: {len(changed)} files {'would change' if DRY else 'changed'}")


if __name__ == "__main__":
    main()
