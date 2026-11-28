# MediaIn stall of the xlsx editor (2026-11-27)

## Verdict
No native reproduction on the current source: the real-runtime `AppCommand::MediaIn` of an xlsx completes. Root cause in the guest is not demonstrable from source; the stalled behaviour is attributed to the stdio-office wasm of 03:48, which predates a large amount of job/retirement/reactor churn (dozens of framework `job`, `retirement`, `retained-turn`, `reactor/turn`, `typed-operation` Rust files changed after 03:48). A wasm rebuild of stdio-office is required to confirm in the browser (not done here, per instructions).

## Native twin of the pdf law
New file `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/🌉️transitional/✏️editor/🧪️tests/🔬️media-in/🦀️.rs` (included from `.../✏️editor/🦀️.rs` next to `mod tests`), harness copied from the pdf `shard_media_in_open` law (real `PluginRuntime`, browser shard fuel 50 M, typed-operation page acknowledgement by the simulated shell, command pages one per turn, 512-turn bound, instance close). Fixture `🧱️base/🧪️tests/🧫️fixtures/💰️budget.xlsx` is the 3 042-byte `budgetWorkbook()` of the browser probe (shared strings, styles, merge, formula, bool, inlineStr). Dev-dependency `semio-framework-trace` added to the transitional crate for the wall-clock case.

Tests (crate `semio-s-artifact-stdio-xlsx-transitional --lib media_in_of_`), exit 0, `5 passed`:
- `media_in_of_the_marketplaces_workbook_completes_under_the_shard_budget`
- `media_in_of_the_small_budget_workbook_completes_under_the_shard_budget` (159 turns, 1 `LoadDocument` effect in the first run)
- `media_in_of_the_small_budget_workbook_completes_under_the_interactive_grant`
- `media_in_of_the_small_budget_workbook_completes_beside_a_resident_editor_instance` (instance 4021 open, import into 4022, as editor #2 / import #3 in the browser)
- `media_in_of_the_small_budget_workbook_completes_under_a_two_millisecond_turn_wall` (real clock deadline per turn; also passed at 100 ms)

Each asserts: no fault, `CommandComplete` within 512 turns, at most one parked `LoadDocument`. Run: `claude-media-in-xlsx-test.sh` (guarded, private target dir, deleted afterwards). Native debug census of one run: 1 PageAccepted x2, 151 CommandPending turns with sources `typed_operation, command_ingress`, 5 with `command_ingress`, then CommandComplete; never the browser's endless `command-pending` with nothing observable.

## Other fix found on the way
`positioned_worksheet_children` (`📕️xlsx/.../🧱️base/🧬️schema/🗣️vocabulary/🦀️.rs`) tied the XML children lifetime to the scope and namespace borrows, so `worksheet_cell_refs` (`💡️inferences/📘️workbook`) failed with E0515 and nothing downstream (xlsx base-model, transitional) compiled. It now borrows only `children` for `'a` and owns clones of scope and namespace.

## Remaining
- Rebuild stdio-office wasm from the current tree and re-run `e2e-probe-open-long.ts`; if the stall persists, the difference is browser-only (wasm clock, shell turn pacing), and the next probe is the shell's drive of the `load-document` effect and typed-operation acks, which the native twin only simulates.
- The twin does not apply the parked `LoadDocument` to a fresh editor (the shell's `importAppMedia` does that in TS).

## Browser outcome after stdio-office rebuild (2026-11-27 19:41)

Built stdio-office wasm (47 min cold, exit 0, 224 MB), published and activated into the dev modules dir served on 6072. The `e2e-probe-open-long.ts` smoke produced `shell.documentTransfer.file-opened` ("Opened Budget.xlsx") after 31 s. Log: `🗑️generated/claude-pane/smoke.log`.
