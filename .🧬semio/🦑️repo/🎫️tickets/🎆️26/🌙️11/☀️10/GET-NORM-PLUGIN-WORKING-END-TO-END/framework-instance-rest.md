# Instance leftovers (effect migration)

## Migrated
- docs `🚪️ingress`: `advance_snapshot_read_returns_one(stores,interaction,effect)`; child pump `drive` and the transient root step run on `effect.nested(child)` (child grant = grant minus rotation parent copy).
- 🪜️maintenance-ladder stage 8: moved before the fault-only ladder, `plugin_effect_close(cx,grant,..)`; the 3-iteration loop became one call (every call posts >=1 copied item).
- 🔒️close-ladder `SnapshotReturns`: `plugin_effect_close`.
- tool-run instance callers: `🧹️member-retirement-and-close` (`owner.close_step(&mut *effect)`), `📤️publication-and-member-refresh` (ingress close via `close_original_effect_step` + `RetirementPollExit::settle`). `ToolRunEntry.entities/.payload` needed no further change.
- peer test module: all close/step calls run through a new `retirement_fixture::run` (`EffectBank` + `driven`, genuine original producer invocation, heap observed inside). Dev-dependency `semio-framework-value-testing-admission` added.

## PEER_HEAP
It was a `#[global_allocator]` (`HeapWitness`) backing `observe_heap_allocations_on_this_thread` in the peer tests, not unused. `testing-admission` links `semio-framework-value-testing-allocator`, whose global allocator delegates to `HeapWitness` through `RequestedAllocator`; two global allocators conflict, so the peer one was removed. All allocation assertions are unchanged and still measured.

## Verification
- `check -p semio-plugin-instance-docs --lib` (log instance-rest-6): 18 errors, all in `🛠️tool-machine/🫧️host` (other agent: old `admit_original_construction` arity, decoder identity `u64` vs `NativeAllocationIdentity`, grant `step`). None in instance/ladder files reached yet.
- peer test: earlier run 3 had 3 failures ("native retirement requires its original producer invocation") fixed by the EffectBank helper; run 5 blocked: `semio-plugin-job-receiving` (other agent, mid-edit) has mismatched-delimiter syntax errors at `📥️pack:21` and `:71`; peer tests unverified, rerun once job-receiving compiles.
- Open: document-media `ActiveArtifactEnvelopeDecode::drive` (other agent).

## composition-publication callers (coordinator follow-up)
- New `plugin_controlled_effect_close(cx,grant,|effect,control|..)` beside `plugin_effect_close` in plugin ownership `⚠️failure`.
- 🧵️instance `🪆️child-group`: `advance_inputs(..,effect)` via `plugin_effect_close` (clock read before the closure).
- 🧵️instance `👶️child-operation-unit`: controlled child-group close via `plugin_controlled_effect_close`.
- 🧵️retained-command `🪟️mounted/♻️frontier`: artifact (controlled and plain) and child-group controlled closes via the two helpers.
- Not migrated: plugin `🧪️tests` (cancellation-publication-boundaries, publication-retirement-authority, ...) still grant-shaped.
- `check -p semio-plugin-retained-command --lib` (log instance-rest-7): 15 errors, all in job `📨️emission/🏛️host` (other agent: `built`/`built_with` arity). Not in my files.
- Reruns pending: instance-docs, retained-command, peer tests once tool-machine/scrub-host, job emission host and job-receiving compile.

## publication-mounted (coordinator follow-up)
`check -p semio-plugin-publication-mounted --lib`: 0 errors (log instance-rest-11). It was not a composition-publication caller but an unmigrated grant-shaped receipt chain; effect threaded through:
- payload `payload_close(bytes,effect)` (page release via `release_empty_page(effect)`), `MountedReceiptPayload::close_step(effect)`.
- receipt: prepared-operation bytes, operation-list bytes and kernel receipt `advance`/`close_step` take `effect`; author admission via `SharedUtf8::admit(..,effect.actual())`; child grants via `effect.nested(..)`.
- group receipt, member receipt and members `advance_receipt`/`advance_row`/`close_step` take `effect`.
- owner `receipts.close_step` runs through `plugin_effect_close::<ValueError>`; instance `🪆️child-group` `advance_receipt` through `plugin_effect_close`.
- crate tests under `🪟️mounted/🧪️tests` not migrated/verified.

## instance-docs rerun
`check -p semio-plugin-instance-docs --lib` (log instance-rest-12): 95 errors, all in `🎮️command` (`📥️admission`, `🎟️typed/*`, `🗝️latest`): other agent's crate, not instance code. Instance/ladder files not reached yet.

## retained-command and latest reruns
- `check -p semio-plugin-retained-command --lib`: 0 errors (log instance-rest-17). Migrated: job close paths (`publication`/`fault_publication` effect closes, `with_original_effect` retire_publication), mounted page close (`close_step(effect)` with `plugin_poll_outcome`), ui close, frontier closes (`plugin_effect_close`/`plugin_controlled_effect_close` pinned to `ValueError`), full-operation `close_step`, `lease_retirement_step(canonical,effect)`, `result_page_retirement_step(effect)`, producer `retire_original_producer(cx,grant)`, `view_state` through `artifact_retirement_admit_shared_lease`, `captured_child_content` as `Option<SharedOriginalStorage<ChildContentView>>` retired through `admit_shared_retirement_into` (new dependency `semio-plugin-job-receiving`), `original_emission_decoder` identity type. New ownership helpers `plugin_controlled_effect_close` and `plugin_poll_outcome` (exported). Instance callers (typed-ops retirement, maintenance-ladder full-operation) use `close_original_effect_step` (work quote dropped; no effect-shaped quoted step exists).
- retained-command tests: not migrated yet (blocked below).
- `check -p semio-plugin-instance-docs --lib` (log instance-rest-18): 57 errors, all in `⏪️time-travel` (assigned to a new agent).
- peer tests (log instance-rest-19): did not reach the tests; `semio-plugin-decl-app` fails to compile at `👷️job/🎮️command/🧬️op-binary:75` (`OpBinaryProducer` lacks `ArtifactOriginalCommandProducer` bounds for `terminal_is_empty`), another agent's in-flight edit.

## instance-docs rerun after time-travel
- time-travel now compiles. instance-stores ingress needed `close_original_retained_step::<ValueError>` (fixed).
- `check -p semio-plugin-instance-docs --lib` (log instance-rest-21): 8 errors, all in `semio-plugin-typing-edit` (`🛠️tool-machine/⌨️typing/✍️edit/🦀️.rs` lines 23, 33, 44, 48, 65, 108): old `admit_original_construction` arity (6 args), decoder identity `u64` vs `NativeAllocationIdentity`, grant-shaped `step`. Tool-machine territory; not edited. No errors in instance/ladder files.
