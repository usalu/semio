# XLSX browser open: the real failure, root causes and fixes

Date 2026-11-26. Pane lane: standalone `stdio-xlsx` (journey: `standalone-pane-journey.ts`; capture: `claude-xlsx-browser-open.ts`).

## 1. The real failure (captured)

`claude-xlsx-browser-open.ts` records every console message, page error, worker lifecycle and `weberror`, and takes the notice text without truncation. The visible notice (`shell.documentTransfer.file-open-failed`, "could not be opened; the current document is unchanged") carries no cause. The cause is only in the console warning of the shell:

```
[os-shell] shell.documentTransfer.file-open-failed file open failed Error: natural-file.no-data-folder
    at keepArtifactOpeningV1 (…/ShellHost/🗂️local-catalog/📥️artifact/💾️opening/🟦️.ts:18)
```

The `[DEBUG] cft0 …` prints (80,000 lines per open) are another agent's retirement trace in the published wasm and are noise, not the error.

So decode, the work limit and the retirement of the import all succeed in the browser now. The open fails afterwards, in the shell, when it keeps the imported document.

## 2. Root causes (four, found in sequence)

1. **No data folder on the pane serve (`natural-file.no-data-folder`).** An opened file is kept as a local document before its editor is shown (`keepProgramDocument` → `localArtifactAdmissionV1`). That needs `VITE_S_DATA_DIR`, which the vite config bakes from `S_DATA_DIR` at serve time; the browser writes the folder through the dev server's `/semio-backbone` route. `claude-fulls-serve.sh` (the native pane recipe) never set `S_DATA_DIR`, so every open was refused. The refusal is also invisible: the notice does not name it.
2. **The opened document was blank (new, hidden behind 1).** Native tests pass because they run the guest in-process. In the browser, `XlsxSnapshot` has no `native_pack_source` (its pack is a sqlite snapshot, not a record spec; only PDF and a few others have one), so the natural-file job ends with the `Effect::LoadDocument` route, not the in-guest genesis adoption. The effect is parked per instance in `pendingTurnEffects`, and the open flow imports into a fresh instance that nobody refreshes or invokes, so the effect was never applied: the fresh editor stayed the blank birth workbook (1,746-byte save, `A1` empty), which the shell then kept and showed. A later command on that instance would have applied the stale load.
3. **The "Strict" fixtures are not Strict.** `➡️after.xlsx` (set-main-namespace) is a mutation-test artefact with a Strict main namespace but Transitional `r:`; the Strict decoder correctly refuses it (`relationship id attributes use the namespace …, expected the Strict officeDocument relationships namespace`). The strict subset's own `📕️reuse-marketplaces.xlsx` is a byte copy of the Transitional file.
4. **German decimal entry stayed text.** The browser reduces `set-cell` through the bounded native path (`bounded_native_editing_editor!`), which has no view state and therefore no locale, so `1,5` was parsed with the invariant notation and stored as an inline string. Native tests exercised the `handle` path, which has the locale.

Why the browser differs from the native tests: the native suites call the guest in-process (effects are returned to the test, the editor handles `set-cell` with `view_state`), while the browser drives a fresh instance through `importAppMedia` and the bounded native reducer, and needs a data folder from the serve.

## 3. Fixes

1. `claude-fulls-serve.sh` exports `S_DATA_DIR` (default `$R/.cache/pane-data/<variant>-<port>`, short enough for the 256-character storage suffix) and a per-port vite cache directory for extra ports. A second pane on 6273 was started with it (6272 is untouched, as ordered). Test: `claude-xlsx-browser-open.test.ts`.
2. `🔌️PluginRuntime/🔗️channel-adapter/🟦️.ts`: `importAppMedia` now takes the parked `load-document` effect of the fresh instance (`takeParkedEffectsV1`, new in `🏁️operation-completion/🟦️.ts`) and loads it with `loadDocumentArchive` before it resolves; other parked effects stay for their owner, more than one load is refused. Test: `🧪️tests/📦️document-pack-wire-adapter/🟦️.tsx` ("loads the whole-document LoadDocument effect a natural-file import parks…"); the whole PluginRuntime suite passes (150 tests).
3. Genuine Strict workbook `🔒️strict/🧪️tests/🧫️fixtures/🔒️reuse-marketplaces-strict.xlsx`, derived by `claude-xlsx-strict-fixture.py` (namespaces, relationship types, `conformance="strict"`). The journey uses it and expects `A1 = ID` for both dialects.
4. Locale for the native path. The grid's cell action carries a hidden optional `locale` argument; `command_from_action` rewrites the typed text into canonical invariant spelling with `xlsx_canonical_cell_text` (German `1,5` → `1.5`, German `1.5` → literal text, English `1,5` → text, no locale → unchanged), so both the native reducer and the `handle` path (now locale-free) agree. Files: `🪟️window-kit/🦀️.rs` (contract), base/strict/transitional `✏️editor/🦀️.rs`, base main window. Tests: fixture `🧫️fixtures/🌐️canonical-cell-text/🔣️.json` (cross-checked with a Python float oracle) and two unit tests in the base editor test module.

## 4. Journey verdicts (pane on 6273, the wasm of 2026-11-25 23:48 plus the TypeScript fixes)

| Step | Verdict | Evidence |
| --- | --- | --- |
| en: open Transitional, edit A1, save, reopen | pass | A1 `ID` after open, saved file 11,045 bytes, edited cell read back |
| de: open, edit A1, save, reopen | pass | German notices, `Geändert auf Deutsch` round trip |
| de: `1,5` stored as number 1.5, shown as `1,5` | fail, fix written, not built | saved as inline string `1,5`; needs fix 4, which is Rust and needs the rebuilt wasm |
| open Transitional workbook | pass | `A1 = ID` |
| open Strict workbook | pass | genuine Strict fixture, `A1 = ID` |
| en and de: refuse text file and foreign ZIP named `.xlsx` | pass (4 of 4) | localized `content-mismatch` notices |

TypeScript-only fixes (1 to 3) are verified in the browser. The Rust fix (4) is verified only by its written tests, which have not been compiled or run.

## 5. Rebuild status and blocker

The wasm could not be rebuilt for fix 4. Sequence of causes, all outside the change:

1. Other agents' in-flight breaks (about a dozen distinct ones, each fixed by its owner within an attempt or two).
2. Warden cancellations of IFC shards over their carving budgets (budgeted by the coordinator), then of the `semio_hub_stdio_component` thin-LTO link at 6.0 to 6.4 GB (memory pressure, stalled-too-long, compressed memory), four times.
3. Final blocker: with LTO off the stdio component's core module declares more than 1,000,000 functions (wasmparser limit). The 2026-11-25 23:36 build declared 934,477, so the code has grown past the cap. `-Z share-generics=y` did not bring it under. The coordinator is splitting the stdio component (large artifact families into their own extension component); the rebuild resumes after that lands.

Build scripts (ticket folder): `claude-pane-cargo.sh` (now with `--keep-going`, `lto`, a 60% free-memory gate), `claude-pane-cargo-retry.sh`, `claude-pane-cargo-forever.sh`. My loops are stopped.

## 6. Not done

- Rebuild, publish, support, activate and the pane journey for the German decimal step.
- The full S lane (6070/6071): session, publish with space, stdio and stdio-office, activate, serve with `S_DATA_DIR`, journeys en and de. `claude-fulls-journey.ts` already writes `conformance="strict"` into its generated Strict workbook; it still needs to use the genuine Strict fixture.
- Native run of the new Rust tests (base editor: `canonical_cell_text_fixture_agrees_with_the_python_float_oracle`, `a_german_cell_draft_reduces_to_the_canonical_number_on_the_locale_free_native_path`).

## 7. Files changed

Shell and runtime: `🔌️PluginRuntime/🔗️channel-adapter/🟦️.ts`, `🔌️PluginRuntime/🏁️operation-completion/🟦️.ts`, `🧪️tests/📦️document-pack-wire-adapter/🟦️.tsx`.
Rust (unbuilt): `✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/🪟️window-kit/🦀️.rs`; xlsx base, strict and transitional `✏️editor/🦀️.rs`; xlsx base main window `🦀️.rs`; base editor unit tests; fixture `🧫️fixtures/🌐️canonical-cell-text/🔣️.json`.
Fixture: strict subset `🧪️tests/🧫️fixtures/🔒️reuse-marketplaces-strict.xlsx`.
Ticket folder: `claude-xlsx-browser-open.ts`, `.test.ts`, `.md`, `claude-xlsx-strict-fixture.py`, `claude-fulls-serve.sh`, `standalone-pane-journey.ts`, `claude-fulls-journey.ts`, the three build scripts above.
