# XLSX History Draft

The committed insert-cell witness seeds successfully on its own workbook. The prior native run also confirmed that the original mutation applies, matches the committed output, and exports B2 as the value 42 through Calamine. The subsequent stale error belongs to the fallback case, which tries the committed worksheet address against a different initial workbook. The address is revision-bound, so the fallback refusal preserves the intended guard.

The acceptance runner now records the first refusal per attempted path when every draft is skipped, including the exact editor refusal or blocked replay severity. It keeps at most four paths, reserving a slot for the row draft. This makes it possible to distinguish schema validation, payload rebuilding, and replay from the existing general skip text. The targeted managed Cargo run covers both Base history tests.

An independent Ajv check accepts the existing operation and the row-3 draft against the authored leaf and reference schemas. The neutral before and after snapshots show that B2 is inserted and that B3 is vacant in the before document. The Rust regression uses that same fixture and holds the original worksheet identity through generic payload reconstruction, then checks B3 through Calamine. The added Rust assertions passed syntax parsing.

## Compiler capture

The first owner-scoped test compile exited 101 before it reached the two history tests. Its `semio-plugin-decl-app` library was published at 11:06:20.677 UTC on 21 November. The live `ArtifactApp` and `EditorApp` sources gained matching `build_original_picker_job` signatures at 11:06:27.073 and 11:06:27.076 UTC. The surface compile had already consumed the earlier declaration artifact and failed with E0407. Both live signatures agree. I left the picker sources as they were and started one fresh managed build, using the same exact XLSX Base selector and a new log named `xlsx-history-draft-refusal-picker-retry`.

The retry cleared E0407. The next compile stopped while the reactor lifecycle used `semio_plugin_decl_media` before its matching dependency was in the captured package metadata. The live lifecycle manifest lists that dependency at 11:28:23.011 UTC. The managed retry recognized that the XLSX lockfile lagged its live manifests and refreshed it before the next capture within the same process.

The refreshed capture advanced through reactor lifecycle, context, and the acceptance harness. The harness rejected the newly added diagnostic because its submodule did not import the standard BTreeMap. I qualified the local reference and restarted the same two-test selector in a fresh managed log, xlsx-history-draft-refusal-map-retry.

## Mounted preview replacement

The native diagnostic build compiled in 30 minutes 33 seconds. An exact regular-test invocation of that executable reached both `/address/row` and `/address/column`; each returned `value.refusal` with zero retained effects from `historyEditInput`, before replay. The mounted editor handles schema or payload reconstruction refusals as ordinary rejected values. The direct fault arose while executing the reducer's ShowPreview effect. Opening the editor already selects a preview prefix, and Store preview selection treated that active owner as a WorkLimit fault after marking it for cancellation. The reducer had already changed its pending replacement by this point.

The mounted Store now defers a new preview while old preview owners retire. The time-travel ledger remembers that a preview is owed and selects the latest replacement already owned by the session after the existing original-grant retirement turns complete. Rapid edits update the same session draft. A replaced prefix or derived root retires through its original grants, and the render path keeps the last completed snapshot until the replacement publishes. Accept and Exit clear deferred preview work when the session leaves Editing. Instance close clears it before retiring the session. The per-turn driver also keeps the selection runnable between closing its completed prefix cursor and admitting the selected operation.

The focused Base native law reads the existing insert-cell wire witness and a rapid-input scenario in the existing neutral sheet-grid JSON. It drafts row 3 and then column 2 through mounted ingress without pumping between inputs. After draining, Calamine must read 42 at C3 and no intermediate value at B3. A second session exits while its prefix is active and must close to the empty original-owner witness. The independent Ajv check accepts both wire shapes, checks the original worksheet identity and vacancy, and confirms the resulting coordinate and value against the fixture. All changed Rust files passed the pinned `rustc -Z parse-crate-root-only --edition=2024` check. The parent runs native verification together with archive diagnostics, with two test threads so that conflict reload cannot hide the regular results.

A follow-up inspection confirmed the real mounted close ladder first hands history owners off at its TimeTravel rung. It does not dispatch Exit first. Terminal close now selects and quotes one original Store handoff at a time, including any shown or intermediate snapshot alias, the active prefix cancellation, replay stop, or the original typed mutation. The original editor and undelivered patch reserve and use the existing retirement queue. The fixture separately verifies Exit while the opening prefix is active, then defers another actual input and closes the mounted instance directly while its session is still Editing. No preview executes during direct close. A closure wait with a zero receipt allows other retirement work to free queue capacity before yielding, without stepping replacement or replay work.

The old map-diagnostic executable finished after 2004.20 seconds across both tests. Its conflict initializer reported `original read birth inputs or actual Source changed` and close stopped at the OwnedStores document rung. The regular failure came from the same pre-change executable as the targeted input diagnostic. The combined replay captured the preview changes and runs the regular and conflict cases with two test threads under the archive progress trace.

The selected retirement uses the resident caller's original Source in two cases. A captured history slot may have to resume or return an interrupted read birth. A selected mutation Source may have to settle its pending Hold action before retirement. Both check the existing Source's identity. Other retirement turns retain their existing effect path. The lower Store suite already covers partial raw history births. It now also contains a neutral-corpus test for every partial funded mutation Source frontier. That new test verifies physical retirement receipts and that the captured issuer alias is returned. Its Rust syntax was checked with the pinned parser and its managed native execution is pending.

## Handoff paths

### Production

- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⏪️time-travel/📒️time-travel-ledger/🦀️.rs`
- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⏪️time-travel/🗄️store-state/🦀️.rs`
- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⏪️time-travel/♻️retirement/🦀️.rs`
- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⏪️time-travel/🧱️store-owners-and-member-visitors/🦀️.rs`
- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⏪️time-travel/🧵️instance/🔎️preview-replay-and-session-start/🦀️.rs`
- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⏪️time-travel/🧵️instance/🏃️turn-driving-and-replay-steps/🦀️.rs`
- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⏪️time-travel/🧵️instance/🧹️owners-and-retirement/🦀️.rs`
- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⏪️time-travel/🧵️instance/🎬️action-dispatch-and-effects/🦀️.rs`
- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⏪️time-travel/🧵️instance/📖️ref-reads/🦀️.rs`

### Test code and neutral fixtures

- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧪️history-edit-acceptance/🦀️.rs`
- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧪️history-edit-acceptance/🎛️acceptance-policy-pump-and-drive/🦀️.rs`
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/✏️editor/🧪️tests/🔬️unit/🦀️.rs`
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/✏️editor/🧪️tests/🧫️fixtures/📊️sheet-grid/🔣️.json`
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/🧪️tests/🔬️unit/🦀️.rs`
- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔁️replay/🎮️operation/🌱️derived/🧬️schema/🔣️.json`
- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔁️replay/🎮️operation/🌱️derived/🧪️tests/🧫️fixtures/🔣️.json`
- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔁️replay/🎮️operation/🧬️native/🧪️tests/🦀️.rs`
- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔁️replay/🎮️operation/🧬️native/🧪️tests/🟦️.ts`
- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔁️replay/🎮️operation/🧬️native/🔗️history/🧬️schema/🔣️.json`
- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔁️replay/🎮️operation/🧬️native/🔗️history/🧪️tests/🧫️fixtures/🔣️.json`
- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔁️replay/🎮️operation/🧬️native/🔗️history/🧪️tests/🦀️.rs`
- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔁️replay/🎮️operation/🧬️native/🔗️history/🧪️tests/🟦️.ts`

### Ticket inputs and results

- `/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️20/COMPLETE-THE-WELL-KNOWN-XLSX-ARTIFACT-AND-EDITOR/audit-xlsx-history.ts`
- `/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️20/COMPLETE-THE-WELL-KNOWN-XLSX-ARTIFACT-AND-EDITOR/verify-history-source-returns.test.ts`
- `/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️20/COMPLETE-THE-WELL-KNOWN-XLSX-ARTIFACT-AND-EDITOR/xlsx-history-draft.md`
- `/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️20/COMPLETE-THE-WELL-KNOWN-XLSX-ARTIFACT-AND-EDITOR/native-verification.md`

The combined run opened its log at 12:48:39.055 UTC and had reached the managed `source.served` phase by 12:59:30 UTC. Cargo selected store `p12ee82`. The latest production edits above were written at 13:12:43.212 UTC. The Rust lower fixtures were last written at 13:16:20.052 UTC and their independent verifier at 13:17:34.492 UTC. The compiler has not reported reaching the changed history packages yet, so their captured revision is unverified.
