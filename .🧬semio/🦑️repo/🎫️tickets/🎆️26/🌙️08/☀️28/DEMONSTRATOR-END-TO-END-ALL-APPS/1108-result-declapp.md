# Lane declapp result (semio-plugin-decl-app 99 errs, semio-plugin-host-configuration 13 errs)

## State (verified: wasm32-wasip2 decl-app W-declapp-2 rc0, host-configuration W-declapp-3 rc0; native both W-declapp-4 rc0; hub W-declapp-hub-1 rc1 only downstream)
- decl-app lifted: completion retirement (`RetireOwned` seed form, `cursor_turn` helper, `error.into()`), birth (resident `OriginalConstructionAdmission`, `with_native_receiving(out,..)` + `receive_original_receiving_work`, `close_original_effect_step` for the shared admission, decoder identity `(NativeAllocationIdentity,u64)`), refusal (tuple cursor seed), context handle + fields (`pub struct Cursor`, effect `step`/`close_step`, `ContextResidualLeases` holds `ArcLease` / `SharedOriginalStorage`, `ContextToolRunSources` holds `SharedOriginalStorage`).
- decl-app job: request retirement (owned + reserved via `retirement_fields_type!/seed!`), bounded-proof registrations (tuple seeds), reserved-outcome-witnesses (`ReservedJobOutcomes::close_step(effect)`, `retire` via `plugin_effect_close`), op-binary producer (resident `construction` admission, `admit_original_construction_settled`, effect close), reserved work (`admit_reserved_work` over `admit_original_construction_settled`, effect close).
- `ArtifactApp::DraftMutation` now requires `::replication::OpBinaryReceiving` (like `ConfigMutation`): `DraftStore` needs it.
- host-configuration (`👷️job/📨️emission/🏛️host`): resident `construction` admission, local `settled` (copy of action-bus `admit_original_construction_settled`, host crate has no semio-framework dep), `receive` helper (flow pattern) for the parser phase, effect-shaped `close_step`.

## Interface changes (callers outside)
- `ReservedJobOutcomes::close_step(effect)` (was grant); fixtures `🔬️plugin-runtime-plugin-builder-contract/📋️clipboard-and-retained-command-fixtures` and `🤖️agent-lane-preview` still call it with a grant.
- `ArtifactOwnedContextHandle` / `ArtifactOwnedToolJobContext` cursor structs are `pub`; `ContextToolRunSources` fields are `SharedOriginalStorage`.
- `admit_op_binary_producer`, `OriginalHostConfiguration::admit`, `admit_reserved_work` birth through a default admission per call (precedent: instance reserved preparation).

## Debts
- cfg(test) code in lifted/🧬️context + 📸️fields (`drain` uses grant step), host-configuration tests/testkit unchanged.
- `settled` duplicated in host-configuration: move `admit_original_construction_settled` into semio-framework-job and re-export from action-bus.
Rerun: `chk.sh W-declapp-<n> 🌎️hub/Cargo.toml wasm -p semio-plugin-decl-app -p semio-plugin-host-configuration`

## Also changed (found by the compile)
- `ArtifactOwnedContextHandle::admit(original,effect)` / `try_duplicate(effect)` / `duplicate_original(effect)` take the effect (were grant); `provisional_source_admission` via a new `OriginalSourceProjection` impl for the handle. Callers: instance staging typed-command:187, command-routes actions:542.
- `ArtifactMediaExportRequestRetirement::close_step(effect) -> OriginalRetirementPoll` (was grant -> Outcome); caller instance docs staging instance-root:77 still on `close_original_quoted_retained_step` with a grant.

## Hub
W-declapp-hub-1 rc1: decl-app and host-configuration no longer red. Remaining reds are downstream and not caused by these interfaces: semio-plugin-composition-publication (41: group/owner, group/child, lib 170-320 `into_message`, grant-shaped steps, VcsError::OriginalRetirement unmatched), semio-plugin-job-child-vcs (5: checkpoint/cascade `RetainedCloneStep` import + double `effect` borrow), semio-plugin-document-media (6: missing crates `semio_framework_os_kernel`, `semio_framework_plugin_composition_emission`, a manifest issue).

STATUS: FIXED-verified W-declapp-2 (decl-app wasm), W-declapp-3 (host-configuration wasm), W-declapp-4 (decl-app + host-configuration native)
