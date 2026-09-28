"""📥️ S20 window-3 prepared patch (phase 2 of the framework chunk staging, lands in the SAME window-3 step right after
`s20-patch-chunk-staging.py`): every app that staged a picked file's chunks itself now reads the whole file the framework
hands it (`semio_framework::kernel::ImportStaging`, admitted in the SDK's `dispatch_action`).

- procedural generation3d: `Generation3dImportStaging` + envelope + chunker gone; `ImportDocument {name, payload}`; the
  one size rule is one Artifact-lane edit (`generation3d-import-capacity`); the document-IO route carries one whole
  import; surface fixture/TS twin lose the chunk ledger (the kernel's `stagingCases` own it now); laws rewritten.
- puzzle 2d / 3d / 5d: the three process-global stagings, envelopes, chunkers and paged parsers gone; the action decodes
  one whole `payload` (≤ `PUZZLE_IMPORT_TOTAL_BYTES`, the export budget) with `Capacity`/`Payload` refusals (localized
  notices; 2d gains `import_too_large`, 3d/5d drop the dead `import_incomplete`); the retained routes admit one whole
  import (`PUZZLE_IMPORT_RAW_BYTES` = escaped JSON string ≤ 2× + one command envelope); laws rewritten, incl. an
  SDK-lane law per 3d/5d (chunks through `handle_action` → one applied edit; a gap is the typed `file-import.gap`).
- architect: CSV file picker `importRegistersCsvRequest` (RequestFileOpen → `importRegistersCsv`), whose file body is the
  framework's `payload` argument (was a single-line `csv` text field no file could reach); catalogue shortcut + laws.
- io-matrix pin: architect CSV import through the picker.

Usage: python3 s20-patch-chunk-staging-2.py [--dry-run]   (idempotent; CONFLICT → nothing written)
"""
import hashlib
import sys
from pathlib import Path

ROOT = Path("/Users/ueli/Documents/semio")
HERE = Path(__file__).resolve().parent / "s20-chunk-staging-2"
DRY = "--dry-run" in sys.argv

G3 = "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any"
G3E = f"{G3}/✏️editor"
PZ = "✏️s/🔌️plugins/🧩️puzzle"
P2 = f"{PZ}/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor"
P3 = f"{PZ}/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor"
P5 = f"{PZ}/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor"
AR = "✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor"
PINS = "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧫️fixtures/🚪️io-matrix.json"
KERNEL = "🧰️framework/🔨️modules/🎠️kernel/🦀️.rs"
IMPORT = "🎮️commands/📥️import-fixture/🦀️.rs"


def new(name: str) -> str:
    return (HERE / "new" / name).read_text()


def old(name: str) -> str:
    return (HERE / "old" / name).read_text()


WHOLE = [
    (f"{P2}/{IMPORT}", "691797c154faf146052037212fe02d5d8b20023f73c95cdd5c0f01462b11163c", "puzzle2d-import-fixture.rs"),
    (f"{P3}/{IMPORT}", "86312dd5a3ba491f801a6d890bc9c74608a578b695432a96687ec76154d5e37e", "puzzle3d-import-fixture.rs"),
    (f"{P5}/{IMPORT}", "c9fa69936486bc0760816c040567f651311f075d63b3d8d8a696c474395cd4fb", "puzzle5d-import-fixture.rs"),
    (f"{G3E}/🎮️commands/📥️import-document/🦀️.rs", "93a7c9709333ae021716425f7f63bd91e969f2b0505a2e75ffad4ad16861f1d0", "generation3d-import-document.rs"),
    (f"{G3E}/🎮️commands/📥️import-document/🧪️tests/🔬️unit/🦀️.rs", "1b72499f8fd2c85bf2583d75f8a255159a2253f3c12976e5085c6659c9b3d597", "generation3d-import-document-tests.rs"),
]


def hunks() -> list[tuple[str, str, str]]:
    return [
        # ── shared puzzle bounds ──
        (f"{PZ}/🎮️commands/🧵️retained/🦀️.rs",
         "pub const PUZZLE_COMMAND_CHECKPOINT_BYTES: usize = 120;\n",
         "pub const PUZZLE_COMMAND_CHECKPOINT_BYTES: usize = 120;\n\n"
         "/// 📥️ Largest file ONE puzzle import may carry — the budget one export may stream ([`PUZZLE_COMMAND_OUTPUT_BYTES`]), so\n"
         "/// a file a puzzle app wrote is always a file it can read back.\n"
         "pub const PUZZLE_IMPORT_TOTAL_BYTES: usize = PUZZLE_COMMAND_OUTPUT_BYTES;\n\n"
         "/// 📏️ Raw wire bytes of the largest command a puzzle route carries: one WHOLE import (the framework reassembles the\n"
         "/// picked file before `importFixture` runs, `semio_framework::kernel::ImportStaging`) as a JSON string — at most two\n"
         "/// wire bytes per text byte once escaped (`\"`, `\\` and the JSON whitespace escapes) — plus one command envelope.\n"
         "pub const PUZZLE_IMPORT_RAW_BYTES: usize = 2 * PUZZLE_IMPORT_TOTAL_BYTES + PUZZLE_COMMAND_RAW_BYTES;\n"),
        # ── puzzle 2d ──
        (f"{P2}/🦀️.rs",
         "/// 📏️ Raw wire bytes ONE retained 2d command may carry — one 32 KiB import chunk plus its escaped\n/// envelope, or a whole-board `applyBoardEvents` select over Nakagin's 180 ids; the same figure the 3d\n/// factory admits, so a file this app wrote is always a file this app can read back.\nconst PUZZLE2D_COMMAND_RAW_BYTES: usize = 262_144;\n",
         "/// 📏️ Raw wire bytes ONE retained 2d command may carry — one whole `importFixture` file, escaped, plus its envelope\n/// (`PUZZLE_IMPORT_RAW_BYTES`, the widest command this route admits; a whole-board `applyBoardEvents` select over\n/// Nakagin's 180 ids is far below it), the same figure the 3d factory admits.\nconst PUZZLE2D_COMMAND_RAW_BYTES: usize = crate::retained_command::PUZZLE_IMPORT_RAW_BYTES;\n"),
        (f"{P2}/🦀️.rs",
         '        factory: "Puzzle2dRetainedCommandJobFactory",\n        factory_type: Puzzle2dRetainedCommandJobFactory,\n        contract: semio_framework::ToolExecutionContract::resumable(262_144, 16_384, 1, 262_144, 7_500, 1, 1),\n',
         '        factory: "Puzzle2dRetainedCommandJobFactory",\n        factory_type: Puzzle2dRetainedCommandJobFactory,\n        contract: semio_framework::ToolExecutionContract::resumable(PUZZLE2D_COMMAND_RAW_BYTES, PUZZLE2D_COMMAND_DECODED_ITEMS, 1, crate::retained_command::PUZZLE_COMMAND_OUTPUT_BYTES, crate::retained_command::PUZZLE_COMMAND_STEP_MICROS, 1, 1),\n'),
        (f"{P2}/🦀️.rs",
         'LocalizedLabel::native("Replaces the whole 2D puzzle with one read from imported JSON, delivered in chunks; the previous puzzle is discarded.", "Ersetzt das gesamte 2D-Puzzle durch eines aus importiertem JSON, das in Teilen geliefert wird; das bisherige Puzzle wird verworfen.")',
         'LocalizedLabel::native("Replaces the whole 2D puzzle with one read from an imported JSON file; the previous puzzle is discarded.", "Ersetzt das gesamte 2D-Puzzle durch eines aus einer importierten JSON-Datei; das bisherige Puzzle wird verworfen.")'),
        (f"{P2}/🗣️terminology/🦀️.rs",
         '        import_invalid: native_en "The file is not a puzzle 2d fixture", native_de "Die Datei ist kein Puzzle-2d-Fixture", reuse_en "The file is not a puzzle 2d fixture", reuse_de "Die Datei ist kein Puzzle-2d-Fixture";\n',
         '        import_invalid: native_en "The file is not a puzzle 2d fixture", native_de "Die Datei ist kein Puzzle-2d-Fixture", reuse_en "The file is not a puzzle 2d fixture", reuse_de "Die Datei ist kein Puzzle-2d-Fixture";\n'
         '        import_too_large: native_en "That file is larger than one import may carry", native_de "Diese Datei ist größer als ein Import tragen kann", reuse_en "That file is larger than one import may carry", reuse_de "Diese Datei ist größer als ein Import tragen kann";\n'),
        # ── puzzle 3d ──
        (f"{P3}/🦀️.rs",
         "        crate::editor::puzzle3d::precompute::retire_abandoned_brush_mesh_uploads();\n        crate::editor::puzzle3d::commands::import_fixture::retire_abandoned_import_runs();\n",
         "        crate::editor::puzzle3d::precompute::retire_abandoned_brush_mesh_uploads();\n"),
        (f"{P3}/🦀️.rs",
         "const PUZZLE3D_IMPORT_RAW_BYTES: usize = 262_144;\nconst PUZZLE3D_IMPORT_DECODED_ITEMS: usize = 16_384;\n",
         "/// 📏️ Raw wire bytes ONE retained 3d command may carry — one whole `importFixture` file, escaped, plus its envelope.\nconst PUZZLE3D_IMPORT_RAW_BYTES: usize = crate::retained_command::PUZZLE_IMPORT_RAW_BYTES;\nconst PUZZLE3D_IMPORT_DECODED_ITEMS: usize = 16_384;\n"),
        (f"{P3}/🦀️.rs",
         '        factory: "Puzzle3dRetainedCommandJobFactory",\n        factory_type: Puzzle3dRetainedCommandJobFactory,\n        contract: semio_framework::ToolExecutionContract::resumable(262_144, 16_384, 1, 262_144, 7_500, 1, 1),\n',
         '        factory: "Puzzle3dRetainedCommandJobFactory",\n        factory_type: Puzzle3dRetainedCommandJobFactory,\n        contract: semio_framework::ToolExecutionContract::resumable(PUZZLE3D_IMPORT_RAW_BYTES, PUZZLE3D_IMPORT_DECODED_ITEMS, 1, crate::retained_command::PUZZLE_COMMAND_OUTPUT_BYTES, crate::retained_command::PUZZLE_COMMAND_STEP_MICROS, 1, 1),\n'),
        (f"{P3}/🦀️.rs",
         'LocalizedLabel::native("Replaces the whole 3D puzzle with one read from imported JSON, delivered in chunks; the previous puzzle is discarded.", "Ersetzt das gesamte 3D-Puzzle durch eines aus importiertem JSON, das in Teilen geliefert wird; das bisherige Puzzle wird verworfen.")',
         'LocalizedLabel::native("Replaces the whole 3D puzzle with one read from an imported JSON file; the previous puzzle is discarded.", "Ersetzt das gesamte 3D-Puzzle durch eines aus einer importierten JSON-Datei; das bisherige Puzzle wird verworfen.")'),
        (f"{P3}/🗣️terminology/🦀️.rs",
         '        import_incomplete: native_en "That import arrived incomplete", native_de "Dieser Import ist unvollständig angekommen", reuse_en "That import arrived incomplete", reuse_de "Dieser Import ist unvollständig angekommen";\n',
         ""),
        (f"{P3}/🧪️tests/🔬️unit/🦀️.rs", old("puzzle3d-unit-laws.txt"), new("puzzle3d-unit-laws.rs")),
        (f"{P3}/🧪️tests/🔬️unit/🦀️.rs",
         "    pub async fn dispatch_reporting(app: &mut Puzzle3dApp, action: &str, args: Option<&Value>, window_id: Option<&str>) -> (Result<InvocationResult, Fault>, Puzzle3dSettled) {\n        dispatch_reporting_with_items(app, action, args, window_id, 1).await\n    }\n",
         "    pub async fn dispatch_reporting(app: &mut Puzzle3dApp, action: &str, args: Option<&Value>, window_id: Option<&str>) -> (Result<InvocationResult, Fault>, Puzzle3dSettled) {\n        dispatch_reporting_with_items(app, action, args, window_id, 1).await\n    }\n\n"
         "    /// 📥️ [`dispatch_reporting`] through the SDK's own action dispatch (`handle_action`) instead of the typed channel —\n"
         "    /// the lane a shell's picked-file chunks take, where the framework's import staging admits them first.\n"
         "    pub async fn dispatch_action_reporting(app: &mut Puzzle3dApp, action: &str, args: &dsl::DslValue) -> (Result<InvocationResult, Fault>, Puzzle3dSettled) {\n"
         "        let window_id = main::WINDOW_KIND_ID;\n"
         "        app.ensure_window(window_id);\n"
         "        let action_meta = ActionMeta { view_state: Some(app.window_view(window_id)), ..meta(\"local\") };\n"
         "        let answered = app.handle_action(action, Some(args), &action_meta).await;\n"
         "        settle_into_reporting_with_items(app, answered, 1).await\n"
         "    }\n"),
        # ── puzzle 5d ──
        (f"{P5}/🦀️.rs",
         "/// 📏️ The retained command route's own wire-admission band, widened off the shared 8 KiB/512 puzzle\n/// default to puzzle 3d's (`PUZZLE3D_IMPORT_RAW_BYTES`/`PUZZLE3D_IMPORT_DECODED_ITEMS`): a Nakagin-sized\n/// document's camera/grid/sun publications and the import/fixture routes both carry wire owners that the\n/// narrow band rejects before the job ever admits.\nconst PUZZLE5D_RETAINED_RAW_BYTES: usize = 262_144;\n",
         "/// 📏️ The retained command route's own wire-admission band, widened off the shared 8 KiB/512 puzzle\n/// default to puzzle 3d's (`PUZZLE3D_IMPORT_RAW_BYTES`/`PUZZLE3D_IMPORT_DECODED_ITEMS`): one whole `importFixture`\n/// file, escaped, plus its envelope (`PUZZLE_IMPORT_RAW_BYTES`) — and a Nakagin-sized document's camera/grid/sun\n/// publications, which the narrow band rejects before the job ever admits.\nconst PUZZLE5D_RETAINED_RAW_BYTES: usize = crate::retained_command::PUZZLE_IMPORT_RAW_BYTES;\n"),
        (f"{P5}/🦀️.rs",
         'LocalizedLabel::native("Replaces the whole 5D puzzle with one read from imported JSON, delivered in chunks; the previous puzzle is discarded.", "Ersetzt das gesamte 5D-Puzzle durch eines aus importiertem JSON, das in Teilen geliefert wird; das bisherige Puzzle wird verworfen.")',
         'LocalizedLabel::native("Replaces the whole 5D puzzle with one read from an imported JSON file; the previous puzzle is discarded.", "Ersetzt das gesamte 5D-Puzzle durch eines aus einer importierten JSON-Datei; das bisherige Puzzle wird verworfen.")'),
        (f"{P5}/🗣️terminology/🦀️.rs",
         '        import_incomplete: native_en "That import arrived incomplete", native_de "Dieser Import ist unvollständig angekommen", reuse_en "That import arrived incomplete", reuse_de "Dieser Import ist unvollständig angekommen";\n',
         ""),
        (f"{P5}/🎮️commands/🗂️open-import-fixture/🦀️.rs",
         "/// 🗂 Opens a file picker that re-dispatches `importFixture` once per\n/// `semio_framework::kernel::IMPORT_CHUNK_BYTES` page of the picked text.\n",
         "/// 🗂 Opens a file picker whose picked text the shell dispatches to `importFixture` in\n/// `semio_framework::kernel::IMPORT_CHUNK_BYTES` pages, reassembled by the framework before the action runs.\n"),
        (f"{P5}/🧪️tests/🔬️unit/🦀️.rs",
         "    assert_eq!(PUZZLE5D_RETAINED_RAW_BYTES, 262_144);\n",
         "    assert_eq!(PUZZLE5D_RETAINED_RAW_BYTES, crate::retained_command::PUZZLE_IMPORT_RAW_BYTES, \"the retained route admits one whole import\");\n"),
        (f"{P5}/🧪️tests/🔬️unit/🦀️.rs", old("puzzle5d-unit-roundtrip.txt"), new("puzzle5d-unit-roundtrip.rs")),
        (f"{P5}/🧪️tests/🔬️unit/🦀️.rs", old("puzzle5d-unit-import-laws.txt"), new("puzzle5d-unit-import-laws.rs")),
        (f"{P5}/🧪️tests/🔬️unit/🦀️.rs",
         "        let result = block_on(app.dispatch_typed(Puzzle5dCommand::from_action(action, args.cloned(), window_id.map(str::to_string)), &action_meta));\n        settle(app, result)\n    }\n",
         "        let result = block_on(app.dispatch_typed(Puzzle5dCommand::from_action(action, args.cloned(), window_id.map(str::to_string)), &action_meta));\n        settle(app, result)\n    }\n\n"
         "    /// 📥️ [`dispatch`] through the SDK's own action dispatch (`handle_action`) instead of the typed channel — the lane a\n"
         "    /// shell's picked-file chunks take, where the framework's import staging admits them first.\n"
         "    pub fn dispatch_through_action(app: &mut Puzzle5dApp, action: &str, args: &dsl::DslValue) -> Result<InvocationResult, Fault> {\n"
         "        let action_meta = action_meta(action, None, None);\n"
         "        let result = block_on(app.handle_action(action, Some(args), &action_meta));\n"
         "        settle(app, result)\n"
         "    }\n"),
        # ── procedural generation3d ──
        (f"{G3E}/🦀️.rs",
         "    eval_session: Option<FlowEvalSession>,\n    /// 📥️ The chunk runs this INSTANCE has open. Instance-scoped, never process-global: two users\n    /// importing into two documents of the same component must never see each other's staged bytes,\n    /// and an instance that closes takes its runs with it\n    /// (`🎮️commands/📥️import-document/🦀️.rs`).\n    import_staging: import_document::Generation3dImportStaging,\n",
         "    eval_session: Option<FlowEvalSession>,\n"),
        (f"{G3E}/🦀️.rs",
         "Self { eval_session: Some(FlowEvalSession::new()), import_staging: import_document::Generation3dImportStaging::default(), run_link:",
         "Self { eval_session: Some(FlowEvalSession::new()), run_link:"),
        (f"{G3E}/🦀️.rs",
         "/// gesture. `importDocument` hands over one CHUNK of a picked file, and a chunk is filled to (and\n/// never past) `semio_framework::PUBLIC_INVOCATION_STRING_BYTES`, the bound\n/// `validate_public_json_envelope` applies BEFORE any tool contract is consulted. Widening the\n/// 8 KiB gesture quota to admit it would widen 23 unrelated interactive routes with it\n/// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, io-surface lane).\n",
         "/// gesture. `importDocument` hands over one WHOLE picked file — reassembled by the framework\n/// (`semio_framework::kernel::ImportStaging`) and bounded by one Artifact-lane edit. Widening the\n/// 8 KiB gesture quota to admit it would widen 23 unrelated interactive routes with it\n/// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, io-surface lane).\n"),
        (f"{G3E}/🦀️.rs",
         "/// 🎒️ Real bound for one document-IO hop's wire payload: one import chunk\n/// ([`import_document::GENERATION3D_IMPORT_CHUNK_BYTES`]) plus the addressed envelope, the chunk\n/// counters and the command id — the whole public-invocation body the host may send, pinned to\n",
         "/// 🎒️ Real bound for one document-IO hop's wire payload: one whole import\n/// ([`import_document::GENERATION3D_IMPORT_TOTAL_BYTES`]) plus the addressed envelope and the command\n/// id — the whole public-invocation body the host may send, pinned to\n"),
        (f"{G3E}/🦀️.rs",
         "/// 📄️ Runs one import/export hop against the app instance's RETAINED owner — the import's chunk\n/// staging lives there, so chunk `n` of a run finds the pages chunk `n-1` left, and a completed\n/// import re-arms every attached preview through the same per-window latch an example switch uses.\n",
         "/// 📄️ Runs one import/export hop against the app instance's RETAINED owner — the export reads its\n/// retained preview, and a completed import re-arms every attached preview through the same\n/// per-window latch an example switch uses.\n"),
        (f"{G3E}/🦀️.rs",
         "                    let mut emit = import_document::emit(payload, &doc, &cfg, &mut owner.import_staging)?;\n                    // 🔁️ Only a chunk that CLOSED its run moved the graph; a staged one authored no\n                    // mutation and owes the previews nothing.\n",
         "                    let mut emit = import_document::emit(payload, &doc, &cfg)?;\n                    // 🔁️ An import that changed nothing owes the previews nothing.\n"),
        (f"{G3E}/🦀️.rs",
         '            // 📥️ `chunk`/`chunkCount` are what `dispatchOpenedFiles` (`🛠️ShellHelpers/🟦️.tsx`) adds\n            // to every picked file\'s `{payload, name}`; an invocation that declares neither is one\n            // whole-payload chunk, the shape every file below one public invocation string takes.\n            "importDocument" => Ok(Generation3dCommand::ImportDocument(import_document::ImportDocument {\n                name: str_arg(&["name"]).unwrap_or_default(),\n                payload: str_arg(&["payload", "contents"]).unwrap_or_default(),\n                chunk: u64_arg(&["chunk"]).unwrap_or_default() as u32,\n                chunk_count: u64_arg(&["chunkCount", "chunk_count"]).unwrap_or(1) as u32,\n            })),\n',
         '            // 📥️ The framework hands the whole picked file (`semio_framework::kernel::ImportStaging`).\n            "importDocument" => Ok(Generation3dCommand::ImportDocument(import_document::ImportDocument {\n                name: str_arg(&["name"]).unwrap_or_default(),\n                payload: str_arg(&["payload", "contents"]).unwrap_or_default(),\n            })),\n'),
        (f"{G3E}/🦀️.rs",
         "            // 📥️ Not palette-worthy: the shell re-dispatches it once per CHUNK of the picked file,\n            // with args no human types (`dispatchOpenedFiles`, `🛠️ShellHelpers/🟦️.tsx`).\n",
         "            // 📥️ Not palette-worthy: the shell dispatches it with the picked file, args no human\n            // types (`dispatchOpenedFiles`, `🛠️ShellHelpers/🟦️.tsx`).\n"),
        (f"{G3E}/🦀️.rs",
         'LocalizedLabel::native("Replaces the generator document with one read from an imported 3D artifact file, delivered in chunks.", "Ersetzt das Generatordokument durch eines aus einer importierten 3D-Artefaktdatei, die in Teilen geliefert wird.")',
         'LocalizedLabel::native("Replaces the generator document with one read from an imported 3D artifact file.", "Ersetzt das Generatordokument durch eines aus einer importierten 3D-Artefaktdatei.")'),
        (f"{G3E}/🧪️tests/🔬️unit/🦀️.rs",
         'import_document::ImportDocument { name: "cube.stl".into(), payload: "data:model/stl;base64,aGVsbG8=".into(), chunk: 0, chunk_count: 1 }',
         'import_document::ImportDocument { name: "cube.stl".into(), payload: "data:model/stl;base64,aGVsbG8=".into() }'),
        (f"{G3E}/🧪️tests/🔬️unit/🦀️.rs", old("generation3d-unit-wire-law.txt"), new("generation3d-unit-wire-law.rs")),
        (f"{G3E}/🧪️tests/🔬️unit/🦀️.rs", old("generation3d-unit-chunk-laws.txt"), ""),
        (f"{G3}/🧫️fixtures/🚪️io/🗿️artifact-surface.json", old("generation3d-fixture-chunking.txt"), '  "faultCodes": {\n    "capacity": "generation3d-import-capacity"\n  },\n'),
        (f"{G3}/🧫️fixtures/🚪️io/🗿️artifact-surface.json",
         "which leaves are deliberately withheld from import and why, and how one picked file is cut into the chunks the public-invocation envelope admits. Language-agnostic: an implementation in any language must publish the same roster, build the same download envelope and cut the same chunks.",
         "which leaves are deliberately withheld from import and why, and the one size refusal an import answers with (the framework reassembles a picked file's chunks before the import runs). Language-agnostic: an implementation in any language must publish the same roster, build the same download envelope and refuse the same sizes."),
        (f"{G3}/🚪️io/🧪️tests/🗿️artifact-surface/🟦️.ts", old("generation3d-ts-ledger.txt"), ""),
        (f"{G3}/🚪️io/🧪️tests/🗿️artifact-surface/🟦️.ts", old("generation3d-ts-chunk-asserts.txt"),
         '  // 📏️ An import refuses by size only, with one stable code: the framework hands the whole picked file.\n'
         '  assert.deepEqual(Object.keys(fixture.faultCodes), ["capacity"], "the import\'s one refusal is its size budget");\n'
         '  assert.match(fixture.faultCodes.capacity ?? "", /^generation3d-import-/u, "the capacity code is this artifact\'s own");\n\n'),
        (f"{G3}/🚪️io/🧪️tests/🗿️artifact-surface/🟦️.ts",
         " * 🧩️ The chunk ledger is re-derived from the fixture's own prose rather than called: an import\n * arrives one chunk at a time, so a run that staged, retransmitted or gapped differently in one\n * implementation would answer a different `nextChunk` here.\n",
         " * 📦️ The framework reassembles a picked file's chunks before the import runs\n * (`semio_framework::kernel::ImportStaging`, whose `stagingCases` live in the kernel's own\n * `🧫️fixtures/📤️file-open-import/🔣️.json`), so this surface states only the one size refusal.\n"),
        (f"{G3}/🚪️io/🧪️tests/🗿️artifact-surface/🟦️.ts",
         "interface ChunkCase {\n  readonly id: string;\n  readonly payloadBytes: number;\n  readonly chunks: number;\n}\n\ninterface StagingCase {\n  readonly id: string;\n  readonly events: readonly { readonly chunk: number; readonly chunkCount: number }[];\n  readonly outcome: string;\n  readonly nextChunk?: number;\n}\n\n",
         ""),
        (f"{G3}/🚪️io/🧪️tests/🗿️artifact-surface/🟦️.ts",
         "  readonly chunking: { readonly chunkBytes: number; readonly cases: readonly ChunkCase[]; readonly staging: readonly StagingCase[] };\n",
         ""),
        (f"{G3}/🚪️io/🧪️tests/🗿️artifact-surface/🟦️.ts",
         '"the editor offers the picker, the export and the chunk sink"',
         '"the editor offers the picker, the export and the import sink"'),
        (f"{G3}/🚪️io/🧪️tests/🗿️artifact-surface/🟦️.ts",
         "`editorActions=${fixture.editorActions.length} viewerActions=${fixture.viewerActions.length} chunkBytes=${fixture.chunking.chunkBytes} ` +\n      `chunkCases=${fixture.chunking.cases.length} stagingCases=${fixture.chunking.staging.length} accept=${fixture.acceptFilter}`,",
         "`editorActions=${fixture.editorActions.length} viewerActions=${fixture.viewerActions.length} faultCodes=${Object.keys(fixture.faultCodes).join(\",\")} ` +\n      `accept=${fixture.acceptFilter}`,"),
        (f"{G3}/🚪️io/🧪️tests/🗿️artifact-surface/🦀️.rs",
         "//! carries, the import roster equal to the declared import dialects, and the chunk envelope one\n//! picked file arrives in.\n",
         "//! carries, and the import roster equal to the declared import dialects.\n"),
        # ── kernel doc: the staging, not the app, reads the chunk counters ──
        (KERNEL,
         "/// multi-file pick. Integers are minted as [`Number::UInt`] carriers, never floats: the guest decodes\n/// `chunk`/`chunk_count` as `u32` and `FromValue`'s unsigned arm refuses a `Float` by design.\n",
         "/// multi-file pick. Integers are minted as [`Number::UInt`] carriers, never floats: the guest's\n/// [`ImportStaging`] reads `chunk`/`chunkCount` with `DslValue::as_u64`, which a `Float` never answers.\n"),
        # ── architect CSV picker ──
        (f"{AR}/🎮️commands/📤️exchange/🦀️.rs",
         "    pub struct ImportRegistersCsv {\n        pub csv: String,\n        pub strategy: String,\n    }\n",
         "    pub struct ImportRegistersCsv {\n        pub payload: String,\n        pub strategy: String,\n    }\n"),
        (f"{AR}/🎮️commands/📤️exchange/🦀️.rs",
         '        import_registers_csv(&mut next_program, &payload.csv, strategy).map_err(|error| Fault::new(FaultOrigin::App, FaultCode::new("architect.import-csv-invalid"), format!("importRegistersCsv cannot read the CSV starting {:?}: {error:?}", payload.csv.chars().take(48).collect::<String>())))?;\n',
         '        import_registers_csv(&mut next_program, &payload.payload, strategy).map_err(|error| Fault::new(FaultOrigin::App, FaultCode::new("architect.import-csv-invalid"), format!("importRegistersCsv cannot read the CSV starting {:?}: {error:?}", payload.payload.chars().take(48).collect::<String>())))?;\n'),
        (f"{AR}/🎮️commands/📤️exchange/🦀️.rs",
         "pub mod export_program {\n",
         "pub mod import_registers_csv_request {\n"
         "    use crate::editor::architect::config::{ArchitectConfig, ArchitectConfigMutation};\n"
         "    use crate::op::ProgramMutation;\n"
         "    use crate::ProgramSnapshot;\n"
         "    use dsl::{FromValue, ToValue};\n"
         "    use semio_framework_plugin::{ArtifactView, ConfigView, Effect, Emit, Fault};\n\n"
         "    /// 🪪️ This app's CSV file-open request id — distinct from its program picker (110) and every other plugin's.\n"
         "    pub const ARCHITECT_IMPORT_CSV_REQUEST_ID: u64 = 132;\n\n"
         "    #[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]\n"
         "    #[dsl(keyword = \"import-registers-csv-request\")]\n"
         "    pub struct ImportRegistersCsvRequest {}\n\n"
         "    /// 📂️ Asks the shell for one CSV file; the framework reassembles the picked file and `importRegistersCsv`\n"
         "    /// receives it whole as its `payload` (merge strategy `upsert`).\n"
         "    pub fn handle(_payload: &ImportRegistersCsvRequest, _doc: &ArtifactView<'_, ProgramSnapshot>, _cfg: &ConfigView<'_, ArchitectConfig>) -> Result<Emit<ProgramMutation, ArchitectConfigMutation>, Fault> {\n"
         "        Ok(Emit::effect(Effect::RequestFileOpen {\n"
         "            req: semio_framework_plugin::RequestId(ARCHITECT_IMPORT_CSV_REQUEST_ID),\n"
         "            accept: \".csv,text/csv\".into(),\n"
         "            read_as: Some(\"text\".into()),\n"
         "            import_action: \"importRegistersCsv\".into(),\n"
         "            multiple: false,\n"
         "        }))\n"
         "    }\n"
         "}\n\n"
         "pub mod export_program {\n"),
        (f"{AR}/🦀️.rs",
         "use crate::editor::architect::commands::exchange::{export_program, export_registers_csv, import_program, import_program_request, import_registers_csv};\n",
         "use crate::editor::architect::commands::exchange::{export_program, export_registers_csv, import_program, import_program_request, import_registers_csv, import_registers_csv_request};\n"),
        (f"{AR}/🦀️.rs",
         '        "setActiveExample" as "set-active-example" => set_active_example::SetActiveExample,\n    }\n}\n//#endregion 🔖️Commands\n',
         '        "setActiveExample" as "set-active-example" => set_active_example::SetActiveExample,\n        "importRegistersCsvRequest" as "import-registers-csv-request" => import_registers_csv_request::ImportRegistersCsvRequest,\n    }\n}\n//#endregion 🔖️Commands\n'),
        (f"{AR}/🦀️.rs",
         'pub(crate) const ARCHITECT_EXCHANGE_TOOL_IDS: &[&str] = &["runValidation", "runAnalysis", "runReport", "search", "exportProgram", "exportRegistersCsv", "importProgramRequest", "importProgram", "importRegistersCsv"];\n',
         'pub(crate) const ARCHITECT_EXCHANGE_TOOL_IDS: &[&str] = &["runValidation", "runAnalysis", "runReport", "search", "exportProgram", "exportRegistersCsv", "importProgramRequest", "importProgram", "importRegistersCsvRequest", "importRegistersCsv"];\n'),
        (f"{AR}/🦀️.rs",
         "        | ArchitectCommand::ImportProgramRequest(_) => Some(1),\n",
         "        | ArchitectCommand::ImportProgramRequest(_)\n        | ArchitectCommand::ImportRegistersCsvRequest(_) => Some(1),\n"),
        (f"{AR}/🦀️.rs",
         "        ArchitectCommand::ImportRegistersCsv(payload) if payload.csv.len() <= ARCHITECT_EXCHANGE_RAW_BYTES => Some(1),\n",
         "        ArchitectCommand::ImportRegistersCsv(payload) if payload.payload.len() <= ARCHITECT_EXCHANGE_RAW_BYTES => Some(1),\n"),
        (f"{AR}/🦀️.rs",
         '        ArtifactToolPublicationContract { tool_id: "importProgram", lanes: &[ArtifactToolPublicationLane::HostOnly] },\n        ArtifactToolPublicationContract { tool_id: "importRegistersCsv", lanes: &[ArtifactToolPublicationLane::HostOnly] },\n',
         '        ArtifactToolPublicationContract { tool_id: "importProgram", lanes: &[ArtifactToolPublicationLane::HostOnly] },\n        ArtifactToolPublicationContract { tool_id: "importRegistersCsvRequest", lanes: &[ArtifactToolPublicationLane::HostOnly] },\n        ArtifactToolPublicationContract { tool_id: "importRegistersCsv", lanes: &[ArtifactToolPublicationLane::HostOnly] },\n'),
        (f"{AR}/🦀️.rs",
         '        tools: ["runValidation", "runAnalysis", "runReport", "search", "exportProgram", "exportRegistersCsv", "importProgramRequest", "importProgram", "importRegistersCsv"]\n',
         '        tools: ["runValidation", "runAnalysis", "runReport", "search", "exportProgram", "exportRegistersCsv", "importProgramRequest", "importProgram", "importRegistersCsvRequest", "importRegistersCsv"]\n'),
        (f"{AR}/🦀️.rs",
         '            "importRegistersCsv" => Ok(ArchitectCommand::ImportRegistersCsv(import_registers_csv::ImportRegistersCsv { csv: str_field("csv").unwrap_or_default(), strategy: str_field("strategy").unwrap_or_else(|| "upsert".into()) })),\n',
         '            "importRegistersCsv" => Ok(ArchitectCommand::ImportRegistersCsv(import_registers_csv::ImportRegistersCsv { payload: str_field("payload").unwrap_or_default(), strategy: str_field("strategy").unwrap_or_else(|| "upsert".into()) })),\n            "importRegistersCsvRequest" => Ok(ArchitectCommand::ImportRegistersCsvRequest(import_registers_csv_request::ImportRegistersCsvRequest {})),\n'),
        (f"{AR}/🦀️.rs",
         '            .action_with(ActionDefinition::new("importProgramRequest", LocalizedLabel::native("Import Program File", "Programmdatei importieren"), ActionKind::Shell, "upload"))\n',
         '            .action_with(ActionDefinition::new("importProgramRequest", LocalizedLabel::native("Import Program File", "Programmdatei importieren"), ActionKind::Shell, "upload"))\n            .action_with(ActionDefinition::new("importRegistersCsvRequest", LocalizedLabel::native("Import Registers CSV File", "Register-CSV-Datei importieren"), ActionKind::Shell, "upload"))\n'),
        (f"{AR}/🦀️.rs",
         '            .action_interactive_job("importProgramRequest", InteractiveJobClassification::Migrated)\n',
         '            .action_interactive_job("importProgramRequest", InteractiveJobClassification::Migrated)\n            .action_interactive_job("importRegistersCsvRequest", InteractiveJobClassification::Migrated)\n'),
        (f"{AR}/🦀️.rs",
         '                    ActionArgDef::text("csv", LocalizedLabel::native("CSV", "CSV")),\n',
         '                    ActionArgDef::text("payload", LocalizedLabel::native("CSV Text", "CSV-Text")),\n'),
        (f"{AR}/🦀️.rs",
         '            .action_describe("exportRegistersCsv",',
         '            .action_describe("importRegistersCsvRequest", LocalizedLabel::native("Asks the user to pick a CSV file of register rows and then imports it with Import Registers CSV by upsert.", "Lässt den Nutzer eine CSV-Datei mit Registerzeilen wählen und importiert sie dann mit Register CSV importieren per Upsert."))\n            .action_describe("exportRegistersCsv",'),
        (f"{AR}/📌️panels/📚️catalogue/🦀️.rs",
         '    Shortcut { id: "architect-catalogue.import-csv", label: "Import Registers CSV", action: "importRegistersCsv", args: &[("csv", ShortcutArg::Text("")), ("strategy", ShortcutArg::Text("upsert"))] },\n',
         '    Shortcut { id: "architect-catalogue.import-csv", label: "Import Registers CSV", action: "importRegistersCsvRequest", args: &[] },\n'),
        (f"{AR}/🧪️tests/🔬️unit/🦀️.rs",
         '        ArchitectCommand::ImportRegistersCsv(import_registers_csv::ImportRegistersCsv { csv: "a,b".into(), strategy: "upsert".into() }),\n',
         '        ArchitectCommand::ImportRegistersCsv(import_registers_csv::ImportRegistersCsv { payload: "a,b".into(), strategy: "upsert".into() }),\n'),
        (f"{AR}/🧪️tests/🔬️unit/🦀️.rs",
         "        ArchitectCommand::SetAdjacencyFilter(set_adjacency_filter::SetAdjacencyFilter { kind: None }),\n    ]\n}\n",
         "        ArchitectCommand::SetAdjacencyFilter(set_adjacency_filter::SetAdjacencyFilter { kind: None }),\n        ArchitectCommand::ImportRegistersCsvRequest(import_registers_csv_request::ImportRegistersCsvRequest {}),\n    ]\n}\n"),
        (f"{AR}/🧪️tests/🔬️unit/🦀️.rs",
         "    let emit = context::drive(&ArchitectCommand::ImportRegistersCsv(import_registers_csv::ImportRegistersCsv { csv, strategy: \"upsert\".into() }), &program);\n",
         "    let emit = context::drive(&ArchitectCommand::ImportRegistersCsv(import_registers_csv::ImportRegistersCsv { payload: csv, strategy: \"upsert\".into() }), &program);\n"),
        (f"{AR}/🧪️tests/🔬️unit/🦀️.rs",
         "#[semio_framework_async_macros::async_test]\nasync fn undo_redo_round_trips_through_the_wrapper() {\n",
         "/// 📂️ LAW: the CSV import is reachable from a file: its request verb asks the shell for one CSV file and names\n"
         "/// `importRegistersCsv` as the verb the picked file is dispatched to — no document edit of its own.\n"
         "#[semio_framework_async_macros::async_test]\n"
         "async fn import_registers_csv_request_opens_a_csv_file_picker() {\n"
         "    let program = sample_plugin();\n"
         "    let emit = context::drive(&ArchitectCommand::ImportRegistersCsvRequest(import_registers_csv_request::ImportRegistersCsvRequest {}), &program);\n"
         "    assert!(emit.artifact_mutations.is_empty(), \"a file request edits nothing\");\n"
         "    let Some(semio_framework_plugin::Effect::RequestFileOpen { accept, read_as, import_action, multiple, .. }) = emit.effects.first() else { panic!(\"importRegistersCsvRequest must request a file: {:?}\", emit.effects) };\n"
         "    assert_eq!((accept.as_str(), read_as.as_deref(), import_action.as_str(), *multiple), (\".csv,text/csv\", Some(\"text\"), \"importRegistersCsv\", false));\n"
         "}\n\n"
         "#[semio_framework_async_macros::async_test]\nasync fn undo_redo_round_trips_through_the_wrapper() {\n"),
        (f"{AR}/🧪️tests/⚖️declared-verbs/🦀️.rs",
         "/// ⚖️ LAW: all 22 declared verbs honour their declarations. The agent lane runs the same retained job the shell runs\n/// and refuses by name (`interactive-job.agent-lane-uncarried`) the four whose result an agent transaction cannot carry\n/// yet: the example switch (a host `LoadDocument`), the two exports (a file download) and the import picker (a host file\n/// request) — routed to the MCP gateway (ticket 26/09/23 `wp-p8.md` § routed). The pin turns red when a carrier lands.\n",
         "/// ⚖️ LAW: all 23 declared verbs honour their declarations. The agent lane runs the same retained job the shell runs\n/// and refuses by name (`interactive-job.agent-lane-uncarried`) the five whose result an agent transaction cannot carry\n/// yet: the example switch (a host `LoadDocument`), the two exports (a file download) and the two import pickers (a\n/// host file request) — routed to the MCP gateway (ticket 26/09/23 `wp-p8.md` § routed). The pin turns red when a\n/// carrier lands.\n"),
        (f"{AR}/🧪️tests/⚖️declared-verbs/🦀️.rs",
         '    assert_eq!(probes.len(), 22, "the architect editor declares twenty-two verbs");\n',
         '    assert_eq!(probes.len(), 23, "the architect editor declares twenty-three verbs");\n'),
        (f"{AR}/🧪️tests/⚖️declared-verbs/🦀️.rs",
         '["setActiveExample", "exportProgram", "exportRegistersCsv", "importProgramRequest"], "agent-lane divergences");\n',
         '["setActiveExample", "exportProgram", "exportRegistersCsv", "importProgramRequest", "importRegistersCsvRequest"], "agent-lane divergences");\n'),
        # ── io-matrix pin: the CSV import goes through its picker ──
        (PINS,
         '{ "id": "csv", "verb": "importRegistersCsv", "via": "argument", "argument": "csv", "args": { "strategy": "replace" }, "from": "csv" }',
         '{ "id": "csv", "verb": "importRegistersCsvRequest", "via": "picker", "from": "csv" }'),
    ]


def main() -> None:
    texts: dict[str, str] = {}
    notes: list[str] = []
    for rel, digest, name in WHOLE:
        current = (ROOT / rel).read_text()
        target = new(name)
        if current == target:
            notes.append(f"applied   whole {rel.split('/')[-2]}")
        elif hashlib.sha256(current.encode()).hexdigest() == digest:
            texts[rel] = target
            notes.append(f"apply     whole {rel.split('/')[-2]}")
        else:
            notes.append(f"CONFLICT  whole {rel}: content moved since preparation")
    for rel, before, after in hunks():
        text = texts.setdefault(rel, (ROOT / rel).read_text())
        head = before.strip().splitlines()[0][:80]
        if (after != "" and text.count(after) == 1) or (after == "" and before not in text):
            notes.append(f"applied   {rel.split('/')[-2]}: {head}")
        elif text.count(before) == 1:
            texts[rel] = text.replace(before, after)
            notes.append(f"apply     {rel.split('/')[-2]}: {head}")
        else:
            notes.append(f"CONFLICT  {rel}: anchor found {text.count(before)}× — {head}")
    print("\n".join(notes))
    if any(line.startswith("CONFLICT") for line in notes):
        raise SystemExit("conflict: nothing written")
    changed = [rel for rel, text in texts.items() if text != (ROOT / rel).read_text()]
    if not DRY:
        for rel in changed:
            (ROOT / rel).write_text(texts[rel])
    print(f"{'dry-run' if DRY else 'applied'}: {len(changed)} files {'would change' if DRY else 'changed'}")


if __name__ == "__main__":
    main()
