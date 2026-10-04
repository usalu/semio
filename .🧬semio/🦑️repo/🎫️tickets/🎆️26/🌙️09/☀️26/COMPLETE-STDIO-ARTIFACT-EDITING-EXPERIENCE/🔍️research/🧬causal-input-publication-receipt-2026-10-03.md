# Causal Input Publication Receipt — 2026-10-03

## Native Authority

The existing terminal carrier is `AppFrame::OperationCompleted { operation, revision, ui_scope, history_patch }`. `take_typed_operation_completion` reads `ArtifactStore::content_revision_now()` only after the typed command settles and projects its first eight bytes into the existing u64 revision lane. The renderer already receives that revision as a bigint. A second completion protocol would duplicate authority and create an ordering gap.

`CursorRevisionAccumulator::revision` hashes the artifact identity, the effective digest and identity of every applied edit, the redo stack, their lengths, and the current checkpoint. A foreign remove/reinsert that restores equal visible bytes still changes the cursor revision because the new edit identities and operations change the applied digest. This makes the committed revision suitable for rejecting same-value positional ABA publications.

## Contract

1. Preserve both terminal u64 fields without JavaScript number narrowing. The channel and plugin runtime carry `operation` and `revision` as bigint. JSON-visible receipts use canonical decimal strings.
2. An applied input outcome may carry `commit: { operation, revision }`. ShellHost creates it only from the exact `OperationCompleted` frame matching the operation id returned by the admitting invocation. A timeout or capacity fallback produces no commit receipt.
3. `InputProps.publicationRevision` is an optional producer-authored canonical decimal-u64 string. It identifies the native document revision from which that retained input was rendered. It is part of the retained component, so equal visible values at different revisions still publish different node content.
4. Windowed editable table cells accept the publication revision separately from optimistic action arguments. Stdio converts the first eight bytes of its already captured canonical 32-byte revision to the same decimal u64. CSV authors it for each editable cell; the generic renderer never guesses a field named `revision` from arbitrary action arguments.
5. The input queue may accept its awaited value only after an applied outcome receipt and a retained publication have the same revision and semantic `draftTarget`. Either may arrive first. A same-value foreign publication cannot release the queue. An own publication followed by a later foreign publication also cannot release a follow-up against the foreign guard.

## Interleaving Law

The neutral fixture covers A/r1 → queued B,C; foreign B/rForeign; local B completion own/r2. C remains blocked at rForeign and becomes eligible only while the mounted publication is r2 and matches the applied receipt. It also carries an operation and revision above 2^53 so TypeScript must use bigint internally and decimal strings at the outcome/component boundary.

## Validation Boundary

Implementation and receipts will be appended after the schema, native renderer, channel, ShellHost and TypeScript contract gates run. The existing single-user rapid20 browser result remains valid but does not prove this concurrent interleaving until the root-owned input hook mounts the receipt comparison and the browser witness is rerun.

## Implemented Pipeline

- `InputCommitReceiptV1` carries canonical decimal-u64 `operation` and `revision` fields on applied input outcomes. ShellHost emits the receipt only after the exact matching `OperationCompleted` frame; timeouts and capacity fallbacks do not synthesize a receipt.
- The TypeScript channel retains both completion fields as bigint. The shared Rust/TypeScript wire fixture covers the ordinary vector, a revision above 2^53, and `u64::MAX` operation without JavaScript-number conversion.
- Native `InputProps` carries the compact `UiPublicationRevision(u64)` value and serializes it as a canonical decimal string. The Store projection uses `AppRenderOperationContext.canonical_base_revision` when hosted, so the rendered component and terminal completion refer to the same revision authority. Snapshot hashing remains only for explicitly unhosted test rendering.
- CSV table Inputs, the shared Details tree, WAV's custom Details provider, and every PDF inspector Input author the render publication revision. The Details convenience API now takes `ArtifactView` so ordinary artifact consumers cannot accidentally publish a snapshot-only guess. PNG uses the same revisioned custom-provider entry point in its separately owned editor integration.
- The root-owned queue compares the applied receipt, retained publication revision, normalized value and semantic draft target. Publication may arrive before or after completion. A same-value publication at another revision enters the explicit conflict recovery state and cannot release the follow-up edit.

## Validation Receipts

- `🗑️generated/causal-input-publication-native-1.log`: compact native component and canonical decimal-u64 contract, 9/9 passed.
- `🗑️generated/causal-input-publication-ts-3.log`: neutral A/r1 → foreign B/rForeign → local B/r2 → queued C interleaving, 2/2 passed.
- `🗑️generated/causal-input-publication-typecheck-2.log`: aggregate renderer typecheck passed before the later root-owned recovery UI changes; the root coordinator owns the final aggregate rerun.
- `🗑️generated/causal-input-details-native-2.log`: revisioned shared Details producer, 1/1 passed with 93 skipped.
- `🗑️generated/causal-input-store-revision-native-2.log`: real Store render context projects the exact canonical revision and decimal-u64 serialization, 1/1 passed with 97 skipped.
- `🗑️generated/causal-input-exact-app-frame-ts-3.log`: TypeScript encodes and decodes all three shared AppFrame vectors exactly, 1/1 passed with 253 skipped.
- `🗑️generated/retained-input-recovery-green-2.log`: root-owned React queue and bilingual conflict recovery, 63/63 passed, including the equality-only foreign interleaving and Escape between completion and publication.

The first Store witness invocation selected zero tests because `--lib` excluded its integration-test binary (`causal-input-store-revision-native-1.log`); it is inconclusive and superseded by the 1/1 receipt. The first two exact TypeScript AppFrame attempts stopped in Nx graph/plugin recovery (`causal-input-exact-app-frame-ts-1.log`, `causal-input-exact-app-frame-ts-2.log`) and are superseded by the fresh 1/1 result.

## Remaining Runtime Boundary

Revisions are hash-derived equality tokens, not monotonic counters. Consumers must never use ordering comparisons. The source and focused laws establish causal correlation, including both settlement/publication orders, but they do not by themselves prove a complete multi-user session. The root-owned live browser rerun remains the runtime acceptance boundary after the aggregate component build is coherent.

## Explicit Draft Publication and Local Lifetime

Every live explicit TextEditor producer now carries the same render-pinned publication authority as retained Inputs. The framework `TextDraftView`, `TextEditView`, document window kit, table overflow editor, Stdio Source/Details helpers, JSON/XML, whole-text, ZIP/Office, CSV/TSV, WAV, BCF and XLSX consumers require `UiPublicationRevision`; the old snapshot-hash fallback has been removed from hosted rendering. The strict settings schema and parser accept only canonical decimal u64 values through `u64::MAX`.

Transient Source drafts are owned by one shell-local exact document identity: plugin, app, session instance, runtime key and client instance. `InterpretedUiNode` passes the stable owner and window lane to every semantic and scene component. A bounded weak owner store retains only dirty, conflicted, failed or pending TextEditor drafts across disclosure unmount. Exact success or explicit Discard removes the entry. Document close, role replacement, spawned-program removal and shell teardown synchronously retire the owner and clear its state. A replacement client instance cannot inherit an earlier draft. Capacity exhaustion refuses the new owner visibly and never evicts an active document.

An Apply promise writes its typed outcome to the retained owner even when the Source disclosure is unmounted. A later remount reconciles that settlement against the newly rendered publication, preserving both completion-first and publication-first ordering without treating equal foreign text as acknowledgment.

Validation:

- `🗑️generated/source-draft-retention-react-2.log`: registered pure + mounted TextEditor suites passed 39/39. The mounted laws collapse and re-expand the real host, settle Apply while hidden, require exact publication acknowledgment, isolate replacement owners, and clear on retirement. Ajv independently validates the neutral retention and exact-u64 schemas.
- `🗑️generated/source-draft-retention-renderer-typecheck-1.log`: aggregate React renderer typecheck passed with the shell owner registry, Interpreter context and TextEditor retention store mounted together.
- `🗑️generated/source-draft-retention-react-1.log`: 38/39 passed; the sole failure used a query helper absent from this repository's render wrapper. It is a test-harness defect superseded by the fresh 39/39 run.
- `🗑️generated/explicit-draft-publication-window-kits-native-1.log`: initial framework native run compiled and passed 15/17 selected laws, exposing a stale expected action id and an optional document revision argument. Both source defects are repaired; a fresh sequential rerun remains required.
- `🗑️generated/explicit-draft-publication-editor-component-check-1.log`: reached the TXT consumer and exposed its missing direct UI-contract dependency; the consumer now uses the framework plugin's public revision reexport.
- `🗑️generated/explicit-draft-publication-editor-component-check-2.log`: stopped before Stdio on the then-current shared DSL `DslValue::Byte` drift. That foreign blocker was removed by its owner.
- `🗑️generated/explicit-draft-publication-editor-component-check-3.log`: active 60-minute finite all-editor component gate with private ticket output. Its final result is not yet claimed.

## Typed Source Refusal and Exact Cancellation

Snapshot Source bounded preflight now admits syntactically invalid but bounded UTF-8 into the reducer so the semantic parser remains the authority. A rejected replacement carries one canonical bounded `Fault` object across retained command output and the shell ledger. `Fault.code`, `Fault.span`, and bounded string params remain typed fields; no UI parses an encoded message. The TextEditor maps stable syntax/schema classes and their one-based line, column and path through required English/German producer labels while preserving the local draft.

An expensive Apply now receives an optional admission lifecycle from the existing scene action funnel. The shell exposes a control only after parsing both decimal-u64 `operationId` and `generation` from the actual typed-operation admission. Its Cancel closure dispatches the existing framework `cancelTypedOperation` verb with the original controller and window provenance and exact fixed-width 16-hex authority. It never calls the actor-wide extension abort route and never guesses an operation from refreshed state. Browser-actor dispatches that do not expose an admission handle do not fabricate a control.

The TextEditor retains the exact operation control with the pending Apply owner. Cancel becomes enabled only after admission, is single-flight while its cancellation request settles, and preserves the local draft. An accepted cancellation waits for the original operation's terminal settlement before retiring the Apply owner; a refused or thrown cancellation re-enables Cancel without clearing the draft. Completion-before-cancel-response and cancel-response-before-completion share one owner token, so neither a retired Apply nor another document can consume the result. A cancellation that races a completed publication conservatively preserves the draft and lets the canonical publication produce the existing conflict state instead of silently acknowledging a value-equal foreign or late commit.

The language-neutral lifecycle fixture pins operation `9007199254740993`, generation `18446744073709551613`, and their independent fixed-width hex cancellation arguments. Ajv validates the schema; Testing Library observes the real accessible Cancel button and retained draft; the pure law verifies exact parsing and action construction.

Validation:

- `🗑️generated/source-cancellation-react-red-1.log`: 42/46 passed and four new cancellation witnesses failed before the lifecycle was implemented.
- `🗑️generated/source-cancellation-react-green-1.log`: focused Source pure and mounted DOM suites passed 46/46.
- `🗑️generated/source-cancellation-renderer-typecheck-1.log`: first aggregate typecheck exposed four test-only contract defects (synchronous mock controls and one unsupported query option); production types were coherent.
- `🗑️generated/source-cancellation-renderer-typecheck-2.log`: aggregate renderer typecheck passed after repairing those test contracts.
- `🗑️generated/source-fault-diagnostic-native-1.log`: canonical bounded Fault wire and independent serde_json oracle passed 12/12.
- `🗑️generated/source-fault-plugin-native-2.log`: retained command canonical typed-fault carrier passed 3/3 with 977 unrelated tests skipped.
- `🗑️generated/source-fault-csv-native-1.log`: inconclusive. The focused CSV job was stopped after fifteen minutes with no rustc process while shared Cargo fine-grain locks were stalled; it is not a pass or failure. The next native run must reuse the ticket-isolated build directory after the current root-owned build finishes.

### Cancellation Owner and Terminal-Race Hardening

The retained Cancel closure also pins the exact admitting effect owner and dialog origin. A replacement window may reuse the same operation id and generation, so operation identity alone is insufficient. Recursive cancellation dispatch now refuses before guest dispatch unless the captured plugin handle, plugin/app/controller/instance, mounted document runtime/client owner, and resolved target program are still current. `lifecycle.started` is not published after the action owner retires. The neutral old-owner law covers both a replacement program and a replacement document owner with coincident operation authority.

Native operation cancellation no longer reports an applied no-op for a missing operation, a terminal operation, or an operation without a live cancellation lease. Only a present, non-terminal, cancellable target sets `user_cancel_requested` and cancels the lease. This prevents a cancellation response that arrives after successful Apply from bypassing the original publication reconciliation. A neutral target-admission matrix is implemented in Rust and TypeScript and validated by Ajv.

Latest receipts:

- `🗑️generated/source-cancellation-react-green-3.log`: focused Source owner, disclosure, causal receipt, diagnostic and cancellation laws passed 48/48.
- `🗑️generated/source-cancellation-renderer-typecheck-4.log`: aggregate renderer typecheck passed with the pinned owner guard.
- `🗑️generated/source-cancellation-target-ts-4.log`: registered operation cancellation identity/outcome/target suite passed 25/25.
- `🗑️generated/source-cancellation-target-ts-1.log`: no test selected before the operation-progress suite was registered in the OS Vitest config; inconclusive.
- `🗑️generated/source-cancellation-target-ts-2.log`: registered suite exposed its stale `bun:test` import under the repository Vitest owner; superseded.
- `🗑️generated/source-cancellation-target-ts-3.log`: registered suite exposed a draft mismatch between the new 2020-12 schema and the existing draft-07 Ajv owner; superseded by the aligned schema and green run.

The focused Rust target is registered in both launch seed and generated launch. The first registered invocation repeated `--lib`, which the repository test runner already supplies; `source-cancellation-target-native-1.log` is therefore an inconclusive launch-contract failure. Both launch files now carry the zero-touch runner form without the duplicate flag.

- `🗑️generated/source-cancellation-target-native-2.log`: the corrected registered Rust target passed 1/1 selected cancellation law with 981 unrelated tests skipped. The run exited zero after 9m40s in the ticket-isolated native build root.

## Source Diagnostic Integration Checkpoint

The semantic Source parser now preserves duplicate-member specificity: `JsonError::DuplicateMember` maps to `snapshot-edit.ambiguous-object`, while all other JSON syntax failures remain `snapshot-edit.invalid-source`. Both paths retain the typed span. Bounded preflight admits duplicate-member source because only the semantic reducer owns this classification.

- `🗑️generated/source-contract-green-4.log`: the full Stdio artifact contract passed 96/96, including the bounded-preflight admission law, duplicate-member semantic code and Source width witness.
- `🗑️generated/source-fault-csv-native-2.log`: the first focused CSV retry stopped in a concurrent shared SQLite snapshot before the typed `ValueRefusalKind` codec was present; current source had already repaired that boundary.
- `🗑️generated/source-fault-csv-native-3.log`: the next retry surpassed SQLite and stopped in a concurrent old stdio-binary typed-error snapshot; current source had already repaired those sites.
- `🗑️generated/source-fault-csv-native-4.log`: the latest focused CSV retry surpassed both earlier frontiers, then stopped before CSV at the in-flight stdio TXT DSL extraction with 25 compiler errors: private `dsl::DslValue`/`FromValue` paths and missing generated `__dsl_spec`, `__dsl_diff_spec`, `__dsl_to_record`, and `__dsl_from_record` methods. This is the current authoritative integration blocker; no CSV runtime result is claimed.
