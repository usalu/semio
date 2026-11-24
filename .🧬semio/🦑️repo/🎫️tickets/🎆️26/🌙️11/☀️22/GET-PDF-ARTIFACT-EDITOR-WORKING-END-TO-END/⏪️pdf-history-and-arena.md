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

## Pass 3: JSON original receiving for 1.4 and 1.7, concrete role diff, drain probe

Build lane: `PATH=<bun dir>:$PATH RUST_MIN_STACK=67108864 sh native-cargo.sh test -p semio-s-artifact-stdio-pdf-pdf-snapshot-value-tests -- pdf14::tests::history_edits_end_to_end pdf14::tests::conflict_history_edits_end_to_end` (`repo cargo` rejects `--manifest-path`; run it from the pdf workspace). `native-cargo.sh` falls back to `~/.rustup/toolchains/nightly-2026-07-20-aarch64-apple-darwin` when the repo toolchain cache is gone. Disk: 29 GB free after the reboot; `native-target/lane-0` holds 34 GB (prune it first if space is needed).

### Done (tree compiles: `semio-s-artifact-stdio-pdf-model` check clean, 1.4 and 1.7 test crate compiles and runs)
1. **Both wires switched** to `with_original_receiving()` (1.4 and 1.7 `document_store_owners`).
2. **1.7 `NativeReceive` derives**: all 145 mutation leaves, aggregate and snapshot model types in `base/schema/{mutations,snapshot}` carry the derive. Framework derive gaps closed schema-first:
   - `with = "pack::value::bytes"` and `bytes::optional` fields are accepted by the native derive (`FieldAttrs::native_octets`; the native wire of octets is a plain `u8` sequence). Needed by `PdfStreamFilter::Jbig2.globals` and the embedded-file `data`.
   - Internally tagged enums used a synthetic field named `tag`, colliding with user variant fields named `tag` (`PdfOp::MarkedContentPoint`): renamed to `__semio_tag`.
   - New variant attribute `#[value(content = "value")]` for newtype variants of an internally tagged enum (payload under a named key instead of flattened). `PdfObject` now declares its `kind`/`value` wire in the schema (`tag = "kind"`, `receive_recursive`, `deny_unknown_fields`) so the derived receiver matches the hand-written ToValue/FromValue wire.
   - `PdfCidFont.system_info` lost `#[value(default)]` (no native default for a heap struct); the field is now required on the wire.
3. **Generic diff**: the derive refuses generics, so `PdfIndexedDiff<PdfAdmittedStreamRole>` in `InsertObject`/`RemoveObject`/`SetObjectValue` became the concrete `PdfRoleDiff`/`PdfRoleItem` (same camelCase wire as the generic; `PdfRoleDiff::to_indexed()` and `From<PdfIndexedDiff<_>>` bridge to the diff algebra). The three leaves, the binary receiver (`roles_target`, `role_rows`, another agent finished those) and four stream-role tests were updated.

### Results (this tree, filtered runs)
| Test | Result | Cause |
| --- | --- | --- |
| 1.4 `history_edits_end_to_end` | fails in 17 s | another agent's `[DEBUG]` probe: `DEBUG_TAGS[index]` in `⏱️trace/🧮️memory/🦀️.rs:232` indexes 73 into a 64-slot array; before that the log shows about 1,890 lines `[DEBUG] work quote denied observed=0 additional=1048577 fuel~999000` from `OriginalJobWorkDriver::admits` (`🧵️job/🛬️receiving/🎟️work`): one receiver asks for 2^20+1 work units against a 1,000,000 fuel turn, so it is denied on every turn. That quote (1 MiB plus one) is the next thing to attribute: it is not a PDF literal, it comes from a framework receiver sizing by an input limit. |
| 1.4 `conflict_history_edits_end_to_end` | fails | exact terminal-empty witness after 4159 turns |
| 1.7 | not run in this pass | |

### Release drain (task 3), evidence only, not fixed
The 1.4 conflict law (not 1.7) already shows it: spent `copied_items 400,922, copied_bytes 60.4 MB, retained_capacity 67.8 MB, released_bytes 470,077,940`; Source release stock left 23,880 against a close quote of 27,136. Released bytes are 7.8x copied bytes and about 1,170 per copied item, so releases are charged for something larger than what was copied (a whole retained document or replay history per item, not per-item frames). The Source grant is 15.3 M items, 451 MB copy and 115 MB capacity, so only the release axis is exhausted. The cause was not located: the build lane was held up by other agents' framework edits (`NativeReceivingFailure.primary`, `OriginalReceivingPrimary` iterator) and a reboot for most of the session, and a second agent is profiling the same drain with `DEBUG_TAGS`/`debug_turn` in `memory.rs`. Next step: the tags histogram at turn 3000, 20000 and 90000 that probe prints identifies the releasing call site; coordinate with that agent instead of adding a third probe.

### Changed files (this pass)
- framework derive: `🧰️framework/🔨️modules/🌱️value/✨️derive/⚙️expansion/🦀️.rs`, `…/🛬️receive/🦀️.rs`, `…/🛬️receive/🔀️enum/🦀️.rs`
- 1.4 and 1.7 `…/🧱️base/🧬️schema/🧬️mutations/🦀️.rs` (wire switch; 1.7 derive)
- 1.7 `🧬️schema/🧬️mutations/*/🦀️.rs` and `🧬️schema/📸️snapshot/📐️model/*/🦀️.rs` (derives), `🔗️object-model`, `🔤️font-model`, `🔺️diff/🔀️tri-state-indexed-and-keyed`, `🧬️mutations/{📦️insert-object,🧹️remove-object,🔧️set-object-value}`, `🚪️io/💾️binary/🧬️mutations/🦀️.rs`, `🪪️stream-roles/🧪️tests/🦀️.rs`
- `native-cargo.sh` (toolchain fallback)

## Pass 4: why the JSON wire is wrong for PDF, and what the 1,048,577 work quote is

1. **The 1,048,577-unit quote is not a stuck loop.** A backtrace of the denial (temporary probe, removed) shows `semio_os_store_replay_operation::native::probe_effect` (`🏪️store/🔁️replay/🎮️operation/🧬️native/🦀️.rs:84`) opening the semantic-preparation effect at `maximum = 1<<20` and backing off by quarters (`maximum/4`, floor 8) until the job admits it. Each preparation turn therefore logs one or more denials at 2^20+1 before the admitted ceiling; the `[DEBUG] work quote denied` lines are the designed probe, not a failure. The cost is a few wasted quotes per turn; a cheap root improvement is to start at the turn's remaining fuel instead of 2^20 (not done).
2. **The 1.4 history test failed after the JSON switch with `original JSON operation wire header differs`.** `OriginalJsonReceiving` requires every persisted operation row to start with the bytes `op` followed by canonical JSON (`with_original_receiving` hard-codes `b"op"`). The PDF vocabulary persists `OP_BINARY_FORMAT | tag | fixed-field payload` through its hand-written `OpBinary` (`🚪️io/💾️binary/🧬️mutations`), so JSON receiving can never read PDF rows. The earlier `funded final payload extent` failure of the binary receiver has the same root: the replacement comparison re-encodes through `ArtifactPreparedOperationSource::CanonicalJson` (JSON text, longer than the binary row). XLSX avoids both because its OpBinary is `format | ordinal | Pack record body`, which `with_original_ordinal_binary_receiving` / `with_original_pack_value_receiving` re-encode exactly (`ArtifactPreparedOperationSource::Pack`).
3. **Consequence and the clean fix.** Both wires are back on `with_original_binary_receiving()` (the JSON switch cannot pass, it only moved the failure). The root fix is schema-first: persist PDF operations as `format | ordinal | Pack record body` (derive the OpBinary body from the already derived `ToValue`/`NativeReceive` schema, drop the hand-written leaf encoders in `🚪️io/💾️binary/🧬️mutations/*`, the `.ksy`, `.abnf` and TS twins) and install `with_original_ordinal_binary_receiving()`. Alternative at the framework: add an `ArtifactPreparedOperationSource::Binary` variant that streams `OpBinary::encode_op` into the funded comparison. Either is a cross-language change that touches 5 leaves in 1.4 and about 60 in 1.7; the derives added in pass 3 (`NativeReceive` on every leaf and snapshot type, concrete `PdfRoleDiff`) are the prerequisite and stay.
4. **Release drain: still unattributed.** The tags histogram (`debug_turn` at turns 3000, 20000, 90000) did not print in my runs because the 1.4 history test dies earlier and the conflict test did not reach those turns. No fix attempted; do not raise the 512 MiB stock.
5. **Not run in this pass**: the four-test matrix, `pdf-io-export` and `mut-family-12`: the shared framework was uncompilable (`semio-framework-pack-json`: `OriginalReceivingPrimary::primary` method/field churn by another agent) when I ended.

## Pass 5: PDF 1.4 persisted as `format | Pack document` (XLSX route), 1.4 history test green

Decision implemented for 1.4 only: the persisted operation is `encode_pack_value(PdfMutation::to_value())` (a Pack document, exactly like XLSX), received by `dsl::original_pack_value_operation_receiving!(PdfMutation)` and installed with `with_original_pack_value_receiving()`. No dual format.

Deleted: the 5 hand-written leaf codecs (`insert-page`, `remove-page`, `move-page`, `resize-page`, `replace-page-text` under `4️⃣1.4/…/🚪️io/💾️binary/🧬️mutations/`), `🧰️wire`, `📥️receiving` (`Pdf14OperationReceiving`, 265 lines), `🫳️borrowed`, the `REGISTRY`, and the hand `OpBinaryReceiving`. Rewritten to the XLSX shape: `🦀️.rs`, `📡️.protocol.semio` (Pack document, magic framing), `🔠️.abnf`, `🟦️.ts` (`PdfMutationsPack = Uint8Array`), `🥋️.ksy`, `🌶️.spicy`. Manifests and mirrors: removed the seven `io::binary::mutations::*` carve entries from `📦️packages/🦀️rust/Cargo.toml`, the `binary::mutations::wire` entry from the snapshot crate `Cargo.toml`, and the binary `mutations { wire }` block from the snapshot mirror. The fixture `🧪️tests/🧫️fixtures/🧬️mutations/🫳️operation-source/🔣️.json` lost its frame bytes (`frameHex`, `tag`, close/cancel limits; schema `pdf14.operation-source/v2`) and the unit test `paged_pdf14_original_source_preserves_five_frames…` became `pdf14_operations_round_trip_through_the_pack_document_with_exact_float_words` (five vector cases, raw float words compared bitwise).

### Results (native-cargo lane, debug, one run each)
| Test | Result | Time |
| --- | --- | --- |
| 1.4 `history_edits_end_to_end` | **ok** | 97 s (with the other agents' debug output) |
| 1.4 `conflict_history_edits_end_to_end` | FAILED: terminal-empty witness, release stock exhausted | 29 s |
| 1.7 history tests | not run: 1.7 still on `with_original_binary_receiving()` (unchanged) | |
| regression suites (lib 38, io-export 19, mut-family-11..15, round-trip 12, page-editor, TS) | not run | |

### Release drain attributed (1.4 conflict law)
A temporary histogram in `StepContext::post_retained` (removed) at 200 MB of cumulative release shows the releases are per-instance and per-step quotes of large inline types, not a copy leak:
- `27,136` bytes x 3,132 = 85.0 MB: `maintenance_ladder_unit_step` -> `LadderCtx` -> `original_lifecycle_step` -> `consume_retained`, called from `settle_document_archive_load` during `load_document` (every seeded acceptance scenario loads documents and each maintenance unit step retires a 27 KB step state).
- `136,496` x 256 = 34.9 MB and `175,776` x 128 = 22.5 MB: `close_registered_fixture_app` releases each fixture app (`VcsArtifactApp<EditorApp<Pdf14Editor>>` is 175,776 bytes by value, its stores 136,496) once per scenario instance; 128 instances existed at the cutoff.
- `3,072` x 3,328 = 10.2 MB: the same ladder path, small stage.
So ~470 MB = (instances per law, >=128) x (inline size of the app, its stores and ladder step state). XLSX passes because its app types are an order of magnitude smaller. Root fixes, in order of preference: (1) stop passing the app, its stores and the ladder step state by value: box or page the large members so the release quote is the real heap allocation, not `size_of` of the whole composed type; (2) retain one ladder step state across turns instead of retiring a fresh 27 KB state per unit step; (3) make the fixture caller stock a function of the quoted app size instead of the constant 512 MiB. Not changed in this pass (framework-wide; the stock is untouched).

### 1.7 plan (not started, scope measured)
Same deletion for `7️⃣1.7/…/🧱️base/🚪️io/💾️binary/🧬️mutations`: 1,584-line `🦀️.rs` (`BINARY_TAG_REGISTRY`, `receiving` with `metadata/object/leaves`, `OpBinary`), 61 leaf directories each re-exported through `pub use semio_s_artifact_stdio_pdf_mut_base_{4,5,6}` / `mut_family_5`, their per-leaf test mirrors and carve lists in ~15 package `Cargo.toml`/mirror files, plus the `📐️e`, `⚕️h`, `🖨️x`, `🧾️vt`, `♿️ua` subset wires and the base presence/transient editor wires (which share leaf modules by path). Because the build loop was 10+ minutes per iteration and the shared framework broke several times, I did not start a half conversion: 1.7 stays on the binary wire and compiles.

### Other
- `probe_effect` (`🏪️store/🔁️replay/🎮️operation/🧬️native/🦀️.rs:85`) still starts at `1<<20`; the denials are the designed back-off, not a failure.
