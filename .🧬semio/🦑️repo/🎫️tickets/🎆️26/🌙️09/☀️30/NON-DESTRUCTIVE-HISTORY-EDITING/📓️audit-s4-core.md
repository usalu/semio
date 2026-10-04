# S4-AUDIT-CORE: read-only audit of the session-4 core changes (2026-10-04, 11:4x-12:2x)

Auditor: S4-AUDIT-CORE (Sonnet, read-only; no cargo/bun/nx run, nothing edited except this report).
Scope: channel handshake (S4-BUMP), store (S4-STORE), runtime (S4-RUNTIME), child-target ToolRun (S4-WIRES-MATH), graph vocabulary (S4-GRAPHS),
registry degrade (design 21.4, S4-INFRA), AGENTS.md compliance. Evidence is the tree on disk at audit time; "git grep" does not see untracked files,
the untracked framework files were listed separately (`git status -uall -- 🧰️framework`). Paths below are relative to the repo root; `FW` =
`🧰️framework`, `OSM` = `🧰️framework/🛍️products/💻️os/🔨️modules`, `PLG` = `OSM/🔌️plugin`.

## 0. Verdicts

| # | Area | Verdict | One line |
|---|---|---|---|
| 1 | Channel handshake | accept-with-changes | No frame precedes admission on wasmtime, pooled, owned-actor and jco paths; the owned codec origin is never admitted (F2), the structured refusal is dropped on every native host (F1), no host-path law (F7). |
| 2 | Store: effective forwards, bump drain, D22, archive-load actor | accept-with-changes | Every FOLD site reads effective forwards; the 8 plugin initializers still seed the revision chain from ORIGINAL forwards (F3); bump drain and D22 sound with bounded caveats (F11, F12); local-actor adoption correct but fallible after commit (F10). |
| 3 | Runtime: filter vocabulary, raw codes, paused rerun, history patch | accept | One filter vocabulary, declared = dispatched; no raw-code fallback left in the history body; paused remote change shows Replay again; patch logic sound. Two minors (F13, F14). |
| 3b | K3 macros `transient_root!` / `window_transient_owners!` | accept-with-changes | Hygienic and complete on paper, zero adopters, law untracked and not run, no compile-time inline-size guard (F8). |
| 4 | Child-target ToolRun, `close_streamed_transaction_unit` | accept-with-changes | Mechanism coherent and matches 21.1; the promised runtime laws do not exist (F4); finalize is not stepped (F5); `close_streamed_transaction_unit` correct. |
| 5 | Graph vocabulary `at`, edge-property trio | accept | delete-node/delete-edge inverses byte-exact, trio inverses exact; one design caveat on positional remove (F15). |
| 6 | Registry 21.4 (stale-channel degrade) | reject (not implemented) | Zero code on disk (F6). |
| 7 | AGENTS.md compliance of the core changes | accept-with-changes | No inline comments added, no `[DEBUG]` in core source, no compat shims; docstring-emoji reuse (F16), one hand-edited generated file (F17). |

## 1. Findings

Severity: critical = wrong result / data loss / safety property broken today; major = design contract or goal clause not met, or hazard on the
critical path; minor = hardening, consistency, convention. No critical found.

### Major

**F1. major. The structured `plugin.channel-mismatch` fault is dropped on every native host; only the browser (jco) path shows the localized notice.**
Evidence: `PluginHostError::Refused(Box<Fault>)` is raised at `PLG/🖥️host/🦀️.rs:1631`, `:3063`, `PLG/🖥️host/⏳️runtime/🦀️.rs:359`, but its only
consumer is `Display` (`🖥️host/🦀️.rs:116` = `fault.describe()` = `plugin.channel-mismatch: the guest speaks app channel 20, the host app channel 21`).
`activation::install_actor` stringifies it (`PLG/🖥️host/🎠️activation/🦀️.rs:60`, returns `Result<ActorId, String>` `:46`), the os host `activate` repeats
the `String` (`OSM/🖥️host/🎠️activation/🦀️.rs:82`), the wgpu kernel `create_app` returns `Result<u32, String>`
(`OSM/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs:6460`, `:8236-8276`) and the native arm of the ProgramBridge maps it with
`ProgramFault::from(String)` = `fault: None` (`OSM/📺️renderer/🧑‍🎨engine/🧱️elements/🌉️ProgramBridge/🎯️targets/🧊️wgpu/🦀️.rs:769`). Only the JS arm keeps
the fault (`js_program_fault`, `:1131-1135`). The wgpu law `a_refused_guest_channel_is_told_as_its_localized_notice`
(`🐚️Shell/🧪️tests/🧪️wgpu-fault-notices/🦀️.rs:~75-101`) builds `ProgramFault { fault: Some(..) }` by hand, so it does not cover the native path. Result:
wgpu desktop, os-hub, MCP and `🏃️run` show the English `code: message` string, not the en/de notice with both channels.
Fix: `install_actor`/`activate` return a typed refusal `{ reason: String, fault: Option<Fault> }` built from `PluginHostError::Refused`; the wgpu
`KernelOutcome::Created` carries `Result<u32, ProgramFault>`; add one law per native path (wgpu kernel, os host activation, MCP `ensure_instance`) that
opens a mismatched guest and asserts the `plugin.channel-mismatch` fault with `guest`/`host` params reaches the notice funnel. Owner: S4-BUMP (host) + S4-WGPU (renderer) + S4-LOAD (MCP/run).

**F2. major. The owned codec origin is instantiated and exercised without the channel handshake.**
Evidence: `OwnedRuntime::codec_call` (`PLG/🖥️host/🦀️.rs:1749-1757`) -> `codec_origin` (`:1638`) -> `assemble_codec_origin` (`:1679-1695`) builds an
`OwnedInstanceState` from `owned.artifact.instantiate()` and runs `PackSchemaHash`, never `OwnedOperation::ChannelVersion` (compare
`instantiate_actor:1620-1632`); `codec_instance` (`:1697-1701`) then serves `Genesis`, `PrintMirror`, `ApplyOps`, `ReplayEnvelopes` (the hub Check In fold),
`SqliteSchema/Export/Import` and the wgpu `OwnedComponentDocumentCodec` (`PLG/🖥️host/🧬️component-codec/🦀️.rs:24-50`, registered before activation at
`wgpu 🧊️renderer/🦀️.rs:8246-8249`). These calls decode `.spr`/envelope/native bytes whose layout is channel-versioned (wave B changes `Edit`). Today only
pre-v21 components are stopped, and only because they lack the export at `OwnedSemioArtifact::from_component` (`🧠️interpreter/🦀️.rs:608-640`); a
v22-built component with all 18 exports passes. The Wasmtime codec path is fine (`:2736` goes through `instantiate`).
Fix: run the `ChannelVersion` op + `admit_guest_channel_version` once in `assemble_codec_origin` (before `PackSchemaHash`), refuse with `PluginHostError::Refused`
(map into `TurnFault::Host`); law: owned codec call against a guest reporting another version is refused. Owner: S4-BUMP.

**F3. major. The 8 plugin store initializers fold effective inputs but seed the revision chain from ORIGINAL forwards, so a reloaded store names another content revision than a whole load.**
Evidence: writer ApplyForward hashes `entry.forwards.get(mutation).encode_op()` (`✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🧬️mutations/💾️binary/🦀️.rs:1091-1099`)
into a private `writer.edit` digest and commits it with `runtime.push_applied(id, digest)` (`:1140`) / `push_redo(id, digest)` (`:1213`); same shape in
generation2d (`...🧬️mutations/💾️binary/🦀️.rs:3628, 3664, 3695, 3716`), generation3d (`:3772, :3824`), gismap (`:1091, :1161`), process3d (`:3838, :3890`),
jack (`:1819, :1892`), drawing (`:5971, :6058`), raster (`:4368, :4450`). The framework initializer and hydration use the canonical
`push_applied_edit` / `push_redo_edit` (`PLG/🦀️.rs:17652, :17679`; `OSM/🏪️store/🧾️document/📜️history/💧️hydration/🦀️.rs:609-611`), which digest
`effective_edit_digest` (`OSM/🏪️store/🦀️.rs:15703-15721`). `CursorRevisionAccumulator::reconcile` keeps the common prefix by id without re-validating edit digests,
so the divergent records persist. Design 3.7 ("digests effective forwards") and the W1G-2 claim ("a retained reload names the same content revision as a whole
load") hold only for the framework initializer. Fix: replace the per-plugin Hash*/Commit* phases by `push_applied_edit(entry)` / `push_redo_edit(entry)` (also
deletes ~8 duplicated phase blocks); extend the G12 reload law with `reloaded.content_revision_now() == whole.content_revision_now()` for every plugin.
Owner: S4-STORE (+ the 8 plugin owners' regions).

**F4. major. The child-target ToolRun has no runtime law on disk.**
Evidence: `📓️s3-wires-report.md` 1.1 item 7 promises `PLG/🧪️tests/🧪️tool-run-member` (stepped progress + cancel, finalize = one child row with the run's
transaction, abort = zero trace, published child mutation editable). The directory does not exist; `PLG/🧪️tests/🔬️tool-run/🦀️.rs` only has `member: None`
(`:52`); the only member coverage is definition validation (`FW/🔨️modules/⏯️tool-run/🧪️tests/🔬️unit/🦀️.rs:418-425`, TS `🧩️conformance/🟦️.ts:57-58`).
`ToolRunMemberFold/Emit/Retire` (`PLG/⏯️tool-run/🦀️.rs:1251-1357`), the member refresh (`:2274-2326`) and `publish_tool_run_member` (`:2336-2362`) are compile-checked
only. Likewise there is no framework-level law for the D6 fix (child-only commit keeps the `TransactionRef`, abort strips it): only flow has one
(`✏️s/🔌️plugins/🌊️flow/...✏️node-graph-edit/🧪️tests/🔬️unit/🦀️.rs:295`). AGENTS: at least one language-agnostic test per feature.
Fix: add the four member-run laws plus the child-only-commit law in `PLG/🧪️tests/` with a two-store fixture app (one parent, one owned child), cross-checked against a
fresh fold of the child store. Owner: S4-WIRES-MATH (member), S4-RUNTIME (D6 law).

**F5. major. A member run publishes in one unstepped call.**
Evidence: `publish_tool_run_member` (`PLG/⏯️tool-run/🦀️.rs:2336-2362`) decodes up to `TOOL_RUN_PROVISIONAL_OPS_MAX` = 65 536 ops (`FW/🔨️modules/⏯️tool-run/🦀️.rs:31`)
in `ToolRunMemberEmit::visit` (`:1300-1316`) and hands them to a single `dispatch_emit_group`; the document-target path steps the publication
(`begin_outbound_apply_batch` + `advance_apply_batch` under the turn deadline, `:2160-2208`). Violates design 20.14 (wall-budget per turn) and the AGENTS
progress/cancel rule for expensive operations (a 65 k-op wires/dag Reorganize blocks the reactor and cannot be cancelled once Publishing starts).
Fix: bound the member publish by the same driver deadline (chunk the `ChildEmit` decode and publish through the child store's stepped batch, or refuse
above a declared op ceiling with a localized notice); keep ONE edit. Owner: S4-WIRES-MATH.

**F6. major. Design 21.4 (dev registry generate degrades per plugin, check/publish strict) is not implemented.**
Evidence: `git grep "stale-channel|staleChannel"` = 0 hits repo-wide; `PLG/📇️registry/🔎️discovery/🟦️.ts` last modified 2026-10-01 23:55; the strict gate
`validateCatalogExecutionProtocol` (`PLG/📇️registry/✅️catalog-verification/🟦️.ts:441-446`) throws through `validateCatalogDescriptorPair` and is recorded
as a `descriptor-invalid` issue (`:655-666`) that fails the whole source audit; `📽️projection/🟦️.ts:296` and `🛂️descriptor-verification/🟦️.ts:127` have no
per-plugin degrade. This is exactly the 04:04 activation failure (all committed descriptors at 20 vs 21) and it blocks "describe + activate puzzle alone".
Fix: in the generate path (`discoverCatalogPackages` consumer) classify a descriptor whose `executionProtocol.appChannelVersion !== APP_CHANNEL_VERSION` as a
`stale-channel` diagnostic (per plugin, in the generated catalog and the console summary) and drop it from the offered set; keep `plugin-registry:check`,
`verify-staged`, trusted-catalog preflight/publish refusing ANY stale descriptor; law: a fixture registry with one stale descriptor generates without it
(diagnostic present) and `check` fails on it. Owner: S4-INFRA.

**F18. major (process). Components built between channel-bump wave A and wave B report version 21 but may disagree on the persisted/wire `Edit` layout.**
Evidence: wave B (delete store `.description`) is not on disk: `OSM/🏪️store/🦀️.rs:15067, 16675, 16882, 17002, 26769`, `FW/🔨️modules/📡️replication/🔗️causal/🔀️transition/🦀️.rs:37, 591`,
`OSM/📡️spr/📜️history/🦀️.rs:124` still declare `description`, and `edit_digest` still hashes it (`OSM/🏪️store/🦀️.rs:15402-15405`). The handshake compares one integer, so a
guest compiled before wave B passes admission and then misreads `.spr`/archive bytes (the exact failure the handshake exists to prevent).
Fix: sequence the describe wave strictly after "BUMP DONE" (wave B landed), or bump to 22 with wave B; add the version-pin census as a precondition of
`rebuild-all --to check`. Owner: coordinator + S4-BUMP.

### Minor

**F7. minor. No host-path law for the handshake.** Corpus `📡️spr/🧵️channel/🧫️fixtures/🧫️channel-handshake/🔣️.json` and
`a_host_admits_only_a_guest_of_its_own_channel_version` (`📡️spr/🧵️channel/🧪️tests/🔬️unit/🦀️.rs:1047`) pin `admit_guest_channel_version` only; no test instantiates a
mismatched guest on wasmtime, the pooled runtime, `OwnedRuntime` or the jco bridge (`git grep Refused|channel_version` in `PLG/🖥️host/🧪️tests` and
`PLG/🌐️browser-bundle`: nothing). The "no frame before admission" property is therefore untested end to end. Fix: one fixture guest with a build-time channel
override (or a mock `GuestRuntime` returning a wrong version through the real `instantiate` wrapper) per host path. Owner: S4-BUMP.

**F8. minor (cheap, high-risk footgun). `transient_root!` lacks a compile-time inline-size guard.** `PLG/🪟️window/🫧️transient/🦀️.rs:436-587` wires
`$mutation::footprint` into `ArtifactEphemeralTransferPreparationFactory`, whose `preflight` refuses every publication when `size_of::<P>()` or `size_of::<M>()` exceeds
`ARTIFACT_EPHEMERAL_TRANSFER_MAXIMUM_INLINE_BYTES` (256) (`OSM/🏪️store/🫧️ephemeral/📢️publication/🔁️transfer/🦀️.rs:34-37`). A cad/fem/remodel scratch root with inline
arrays would compile and then refuse every publish at runtime. Also: zero adopters (`git grep "transient_root!"` = docs only), the law
`PLG/🪟️window/🫧️transient/🧪️tests/🧪️transient-root/🦀️.rs` is untracked and was never run (rule 43), and `footprint` re-serializes the whole root to JSON just to
measure it (O(root) per publish). Fix: `const _: () = assert!(size_of::<$state>() <= ARTIFACT_EPHEMERAL_TRANSFER_MAXIMUM_INLINE_BYTES && size_of::<$mutation>() <= ...)` inside the macro; measure with
a counting writer instead of `to_json_string(..).len()`. Owner: S4-RUNTIME.

**F9. minor. Member-run error paths drop un-retired composed reads.** `refresh_member_overlay` (`PLG/⏯️tool-run/🦀️.rs:2300-2308`) `take`s `member.stale` and returns on the first failing
`retire_tool_run_children(view)?` (`:2327-2332`, `admit_child_content_publication()?` fails before `ChildContentRetirement::new`), dropping the remaining views (and the failing one)
without retirement; `ChildContentView` has no Drop witness, so the snapshot-read lease leaks silently. Fix: on error push the unretired views back into `member.stale`. Owner: S4-WIRES-MATH.

**F10. minor. Local-actor adoption can fail after the document store was already swapped.** `PLG/🦀️.rs:26116-26121`: `local_actor` is read before
`publish_boxed_document_store_candidate_if_authoritative`, then `self.store.set_local_actor_id(local_actor).map_err(..)?` runs after the commit; a displaced-queue refusal there would return
`Err` with the store replaced but `children`/content/composition not yet swapped (lines 26122-26145). Practically unreachable (fresh store queue), but the commit boundary is documented as
non-failing. Fix: call `candidate.set_local_actor_id(local_actor)` on the still-separate candidate before publication. Owner: S4-RUNTIME.

**F11. minor. `relieve_displaced_pressure` is bounded in steps, not owners.** `OSM/🏪️store/🦀️.rs:22983-23020`: gated at 256 owners, at most 16 one-item steps per bump (`:1825`),
each step releasing at most one item. A displaced snapshot of N items needs N steps, so the claim "more than any single change displaces" holds only for owners of at most
5 steps; for large documents a burst of changes without maintenance turns still reaches the 1 024 cap and `reserve(3)` (`:22985`) refuses the change. O(change) and no starvation
(relief breaks on `Blocked`), but the relief error path (`?` at `:22998` after the generation was bumped) returns Err for an applied change. Fix: relieve in owner units with a
byte grant, and swallow/record a relief fault instead of failing the committed change. Owner: S4-STORE.

**F12. minor. Still O(history) per interior step / authoring.** `supersede_inputs` builds a map over every applied op (`OSM/🏪️store/🦀️.rs:21578-21592`), `fill_supersession_targets` over every edit
(`:12875-12880`), `superseded_positions` (`:23814-23838`) and `effective_forwards` allocate op ids for every edit once any supersession exists (`:23596-23603`). Acceptable for finalize/load
(rare), but not the 20.14 target; use `find_near` per target. Owner: S4-STORE (already listed in `📓️w1-g-report.md` as open).

**F13. minor. `history-filter.unknown` has no localized notice.** `PLG/🦀️.rs:30637` raises it (an undeclared value from an agent/MCP caller); it is in neither `HISTORY_NOTICE_LABELS` nor
`FRAMEWORK_FAULT_NOTICE_LABELS` (`FW/🔨️modules/🎠️kernel/🦀️.rs:2174-2189`), so shells fall back to the generic refusal and the NOTICES gate would flag it. `timeTravel.name-invalid` is in the same position.
Fix: add both to the framework table (en/de), TS twin and corpus. Owner: S4-GATES/S4-UI.

**F14. minor. Duplicate transaction ids when the host has no clock.** `FW/🔨️modules/🛠️tool-machine/🦀️.rs:992-994`: `physical_ms: default_now_ms().unwrap_or(0)`; with no clock and a run counter that restarts per instance, two
different tool runs mint the same `TransactionRef.id`, and the history groups rows by transaction id. Fix: refuse to mint without a clock, or mix a per-instance nonce. Owner: S4-RUNTIME.

**F15. minor. `remove-edge-property` is positional.** `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🕸️graph/🧬️schema/🧬️mutations/➖remove-edge-property/🔺️diff/🦀️.rs:7-17` removes by `index`,
with no key check; once a history edit changes an earlier property list, the replay silently removes another entry instead of reporting `target-missing`. Inverses are exact today; the hazard is replay under
editing. Fix: address by `key` (diff finds the key; inverse is `add-edge-property {index: base position}`), keep `index` only on add. Owner: S4-GRAPHS.

**F16. minor. Docstring-emoji uniqueness (AGENTS) is breached on added docstrings.** Per file, docstrings whose first emoji is reused with at least one added line (script over `git diff HEAD`): `OSM/🏪️store/🦀️.rs` 44 emojis
(e.g. 🔭️ x8: `:20370`, `:20618`, `:18561`, `:18566`; 🧭️ x15; 🌡️ x4: `:1824`, `:23001`), `PLG/⏯️tool-run/🦀️.rs` 10 (🧹️ x8, ♻️ x8, 🔁️ x7, 🧩️ x7, 🏁️ x4), `OSM/📡️spr/🧵️channel/🦀️.rs` 13 (🤝️ x4: `:30`, `:35`),
`PLG/🪟️window/🫧️transient/🦀️.rs` 2 (🫧️, 🪟️). Design 21.2 (per file for ticket-touched files). Owner: S4-GATES gate + owning WPs.

**F17. minor. Hand-edited generated file.** `FW/🔨️modules/🛂️manifest/🤖️generated/🪪️manifest/🟦️.ts:1037` carries `member?: string` added by S4-WIRES-MATH by hand (file mtime 02:38, gitignored via
`.gitignore:97`). The Rust type derives it, so the next `framework generate` reproduces it, but the rule is "never hand-edit generated files". Fix: run the generator, do not commit. Owner: coordinator.

**F19. minor. Stale host fixture descriptor.** `PLG/🖥️host/🧫️fixtures/🧩️component/🔣️.json:3346` (`appChannelVersion: 20`) and its `🛂️.descriptor.semio` (old `withoutOperations` filter options) are not in the channel-version census
(pin 21, 39 consumers, 0 findings), so a half-done bump passes while this fixture is stale. Fix: register it as a consumer or regenerate it in the describe wave. Owner: S4-BUMP.

## 2. What was verified and holds (no action)

- Handshake: admission strictly precedes the first frame on `WasmtimeRuntime::instantiate` (`PLG/🖥️host/🦀️.rs:3061-3064`), the pooled `AsyncActorTask::spawn` (`PLG/🖥️host/⏳️runtime/🦀️.rs:350-364`; `ready_tx.send(Ok(()))` only after admission), `OwnedRuntime::instantiate_actor` (`🖥️host/🦀️.rs:1626-1631`) and the jco bridge `createActorApi` (`PLG/🌐️browser-bundle/🏗️materialization/🟦️.ts:1017-1018`, thrown error carries `.fault`);
  `describe_observed` goes through `instantiate_actor`; the descriptor's `appChannelVersion` is the guest's own compiled constant (`PLG/🛂️describe/🦀️.rs:205, 313`); the TS twin and Rust function agree
  (`OSM/🟦️.ts:3735-3740` vs `OSM/📡️spr/🧵️channel/🦀️.rs:38-46`); WIT `channel-version: async func() -> u32` (`PLG/🧬️schema/📜️.wit:1234`); en/de notice in the framework table (`FW/🔨️modules/🎠️kernel/🦀️.rs:2187`).
- Effective forwards: every fold site folds effective inputs (`fold_history`, `fold_effective_edit`, `effective_forwards` `OSM/🏪️store/🦀️.rs:23460, 23596-23625`; hydration `loaded_history_replay` with the initializer's supersessions; framework initializer
  `fold_supersession_step`/`fold_forward` `:15630-15690`; the 8 plugin applier arms call `effective_forward(..)`/`fold_forward`). Remaining `.forwards` uses are codecs, validation, digests of the original op, row/printing reads and amend growth (`:12342-12678, 13350, 15247, 19126, 23644`); `config/📥️retained` folds original forwards but the config lane has no supersession (L4). TS side: no fold twin reads `.forwards`.
- D22: `tail_step`/`adopt_tail_step` (`:20299-20372`) guard on a live fold, line tip, no open transaction, exact id set and own actor; redo only while `tail_step_generation == generation`; retirement order documented; mirrors checked by the test oracle.
- History view patch: `history_store_change` (`PLG/⏪️time-travel/🦀️.rs:1925-1957`) rebuilds on any supersede/checkout/checkpoint/replay, patches only appended/amended/undone/redone tails; `history_view_patch` (`PLG/🦀️.rs:27936-27975`) is conservative.
- Filter vocabulary: one name set in the manifest (`FW/🔨️modules/🛂️manifest/🦀️.rs:2850-2874`), one reader `HistoryCommandFilter::from_value` (`PLG/🦀️.rs:12134-12151, 30635-30637`); no `withoutOperations` in any source (only in regenerable committed descriptors).
- `close_streamed_transaction_unit` (`PLG/🦀️.rs:28732-28756`): commit with child emits keeps the ref, abort strips it; `tool_transaction_shape_fault` (`:24486`) refuses an abort across children or with mutations, so an abort can never carry child emits.
- Graph vocabulary: `delete-node` inverse = `create-node` at the base index then each severed edge ascending at its base index (`🗑️delete-node/↩️inverse/🦀️.rs`), `delete-edge` inverse at the base index, `at` clamped and hidden in `x-semio-ui` (`🏗️create-node/🧬️schema/🔣️.json:164-176`); ascending re-insertion restores positions exactly; `add-edge-property`/`set-edge-property` inverses mirror the diff conditions exactly.
- No inline comments were added in any core diff (checked 10 files against `git diff HEAD`); no `[DEBUG]` in non-test core source (`SEMIO_DEBUG_STORE_UNITS` = 0); no compat shims or deprecations found in these changes.

## 3. Owner routing

S4-BUMP: F1 (host half), F2, F7, F18, F19. S4-WGPU: F1 (renderer half). S4-LOAD: F1 (MCP/run). S4-STORE: F3, F11, F12. S4-RUNTIME: F8, F10, F14, D6 law of F4. S4-WIRES-MATH: F4, F5, F9.
S4-INFRA: F6. S4-GRAPHS: F15. S4-GATES/S4-UI: F13, F16. Coordinator: F17, ordering of F18.
