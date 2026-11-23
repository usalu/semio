# PDF history, native encoder and UI arena

Crate `semio-s-artifact-stdio-pdf-pdf-snapshot-value-tests` (the real package name; it mounts the 1.7 and 1.4 base editor tests). Native runs through `native-cargo.sh`, `bun` on PATH, `RUST_MIN_STACK=67108864`.

## Before and after

| Run | Pass | Fail | Note |
| --- | --- | --- | --- |
| Handoff baseline | 111 | 33 | `history_edits_end_to_end` and `conflict_history_edits_end_to_end` never settle. |
| First run in this session (after the framework had moved on) | 96 | 48 | 36 failures are the history, `history_edit_inputs_resolve` and `closes_mid_reserved_commit` family, all the same catalog refusal below. |
| After the proof-catalog fix | history tests start | | They now fail or run for the real reasons below. |

The messages "native encoder requires its original construction recipient" and "UI arena has no accepting lifetime registration" did not appear in this crate's run at all. They are not present in the current output (0 hits); the `FixtureUiArena::admit()` guard that XLSX uses is already in the 1.7 base editor tests (three tests). The page-editor-authoring-tests crate was not run in this pass.

## Root causes found and fixed

1. **Tool-proof catalog refused every instance (36 tests).** `bounded_native_edit_bounded_first_step_proofs` (`✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/✏️editing/🛠️tool-job-factories/🦀️.rs`) had gained a working-tree filter that skipped every tool whose publication lane is `Transient`. The PDF view and presence tools (`zoom-in`, `fit-page`, `go-to-page`, `set-query`, `next-hit`, `presence-pointer`, ...) are transient native tools and are classified `Migrated`, so `validate_tool_job_rows` expects a proof for each and failed with `interactive-job.catalog-incomplete` before any history ran. Proofs of tools that already have an app registration are skipped by instance ingress, so they cost nothing at run time. The filter was removed. The refusal message in `👷️job/🏛️reserved/🦀️.rs` now names the missing and unexpected tool ids.
2. **Operation wire without original receiving (`installed operation wire has no original receiving capability`).** PDF 1.4 and 1.7 `document_store_owners` installed only `with_original_target`. XLSX installs `with_original_pack_value_receiving` because its operation vocabulary is a Pack Value; PDF operations are fixed-field binary frames with their own `OpBinaryReceiving` cursor, so both now install `with_original_binary_receiving()` (the first attempt with the Pack Value variant failed with "retained Pack final output exceeds caller ceiling", which proves the PDF frames are not Pack Value documents).
3. **PDF 1.4 receiving birth refused (`original mutation receiving birth was refused`).** `Pdf14OperationReceiving::admit` quoted and returned `size_of::<Self>()` copy bytes. The history hydration already charges the cursor box (capacity and its two `Option<Box<_>>` slots) as the construction demand, so the receiver must return one item and no copy bytes, as the 1.7 receiver and the framework macros do. It now quotes `fixed(0)`.
4. **Fixture caller release stock exhausted before the final close rung.** The conflict law ended with `registered fixture did not reach its exact terminal-empty witness` on rung `FinalBackings`: the last close step quotes 175,776 release bytes (the boxed `VcsArtifactApp`) while the fixture's whole original Source stock had 79 release bytes left after 470 MB of paid releases. A PDF document is far larger than the XLSX fixtures, and every history replay releases the cloned document. The plugin testing caller stock `maximumReleaseBytes` in `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️testing/🎟️caller/🧪️tests/🧫️fixtures/🎟️caller.json` was raised from 536870912 to 2147483648. Copy and capacity stock are untouched (30 MB of 528 MB and 22 MB of 268 MB were spent).
5. `begin_original_settlement` gained a `maximum_work` argument in the framework; the two callers in `🏪️store/📜️space-history/🚪️io/🪶️sqlite/📸️snapshot/🚦️native/🦀️.rs` passed the old arity and broke the whole dependency chain. Both now pass 1, the work the surrounding step already declares.

## Latest measurements
- PDF 1.4 `conflict_history_edits_end_to_end` now passes (62 s). The other history tests in this crate were not re-counted; the full-crate after-count is not taken.
- `RetirementSupport` memo: `controlled_retirement_supported::<T>()` (`🧰️framework/🔨️modules/🌱️value/♻️retirement/🧭️support/🦀️.rs`) re-walked the whole PDF type graph on every birth (profile of PDF 1.7 `history_edits_end_to_end`: hot in `TypeId` equality inside `RetirementSupport::supports`). The outermost verdict is now memoized per thread. Not yet re-measured after the change.
- Diagnosis of the 1.4 `funded final payload extent` failure: `verify_replacement_wire` compares the replacement input row with the canonical re-encoding of the mutation (`OriginalOperationWireComparison`, output list sized to the row length). PDF's wire is binary-framed and the canonical encoding is longer, so the output list runs past its exact final extent. XLSX avoids it because its OpBinary is a Pack Value. The consistent fix is JSON receiving, `ArtifactOperationWire::with_original_receiving()`, which needs `NativeReceive<DslValue>` on `PdfMutation` (1.4 and 1.7). I tried it, it did not compile, and I reverted to `with_original_binary_receiving()` so the tree compiles (`cargo check -p semio-s-artifact-stdio-pdf-mut-base` passes).

## Still open

- PDF 1.4 `history_edits_end_to_end` fails with `funded final payload extent cannot extend or reallocate` (`🌱️value/📋️list`, a paged list whose capacity is not a multiple of its page). Not yet attributed to a PDF owner.
- PDF 1.7 `history_edits_end_to_end` and the conflict law keep running past 150 s per test after the stock bump. The release volume (470 MB for one law) shows that a replay releases the whole document per step; XLSX stays far below it. Needs a profile of which close or fold step releases the document.
- The framework was in flux during this work (`Turn::Yield`, `NativeVariantSpec.statements`, `heap_bytes` builds failed in other agents' crates for several minutes each), so the final after-count could not be taken.

## Changed files

- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📜️space-history/🚪️io/🪶️sqlite/📸️snapshot/🚦️native/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/👷️job/🏛️reserved/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️testing/🎟️caller/🧪️tests/🧫️fixtures/🎟️caller.json`
- `✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/✏️editing/🛠️tool-job-factories/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/4️⃣1.4/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/4️⃣1.4/🪆️subsets/🧱️base/🚪️io/💾️binary/🧬️mutations/📥️receiving/🦀️.rs`

## Follow-up pass (coordinator items 1 to 3)

State at the end of this pass: items 1 and 3 are not done, item 2 is half done. The shared native build dir was held by other agents' compiles for over 45 minutes, so nothing after the derive change was run.

1. **Release stock bump reverted.** `maximumReleaseBytes` is back to 536870912 in the plugin testing caller's `🎟️caller.json`. The accounting root cause is not found. The only evidence so far: one PDF 1.7 conflict law spent 469,964,449 release bytes over 341,929 copied items (about 1,374 bytes per item) before the 175,776-byte `FinalBackings` close, so the stock is drained by many small per-item releases, not by one large one. I added a temporary `[DEBUG] rel` probe in `StepContext::post_retained` (`🧰️framework/🔨️modules/🧵️job/⏱️context/🦀️.rs`) to histogram releases of 20,000 bytes or more, but the build never ran, and I removed it again. Next step: log every `post_retained` with `released_bytes` above a few hundred bytes during the PDF 1.7 conflict law and group by size, to find the repeated charge (candidates: per-turn re-release of the same frame, or a quote refunded on a refused turn but not on the Source).
2. **`NativeReceive<DslValue>` for 1.4: implemented and compiling.**
   - `semio_framework_value::NativeReceive` is derived on the 1.4 `PdfMutation` enum, on the five leaves (`InsertPage`, `RemovePage`, `MovePage`, `ResizePage`, `ReplacePageText`) and on `PageDoc`.
   - `InsertPage` used a hand-written `deserialize_with = "deserialize_page"` that rejected unknown page fields. The derive refuses custom field deserializers ("native fields require their explicit original receiving port"). I removed the hook and `PagePayload`, and gave `PageDoc` `deny_unknown_fields`, so the strictness is now declared in the schema.
   - `cargo check -p semio-s-artifact-stdio-pdf-mut-base` passed with these derives.
   - **Wire switch:** the switch to `with_original_receiving()` in 1.4 `document_store_owners` is written but not verified: the test build for it was queued behind other agents' compiles. To keep the tree compiling I left the wire on `with_original_binary_receiving()`. The one-line switch is in `🏅️standards/4️⃣1.4/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/🦀️.rs` (`owners.operation_wire.with_original_binary_receiving()` becomes `.with_original_receiving()`). 1.4 `history_edits_end_to_end` is therefore still failing with the paged-list extent refusal.
   - **1.7 not started:** it has about 60 mutation leaves and nested snapshot object types and needs the same derive treatment.
3. **1.7 timing with the memo: not measured.** The sampled hot spot was `TypeId` equality inside `RetirementSupport::supports`; the per-thread verdict memo in `controlled_retirement_supported` is in the tree but unmeasured.
