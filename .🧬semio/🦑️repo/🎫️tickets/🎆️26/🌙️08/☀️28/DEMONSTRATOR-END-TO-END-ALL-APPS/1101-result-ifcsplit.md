STATUS: PARTIAL. GREEN (wasm32-wasip2 --lib check, ifc workspace manifest, `C/🗑️generated/1101/ifcsplit-16.txt` rc 0, `Finished`): semio-s-artifact-stdio-ifc-4 with its whole dependency chain (ifc-4-model, -mutations, -declarations, ifc-domain, ifc-details-profile/-wire/-type/-edit/-provider/-guidance, ifc-details). NOT done: hub wasm checks (blocked by ifc-4x3, lane ifc4x3), unit tests (testkit feature + dev-deps added, first `cargo test` run `ifcsplit-t1` in progress).

## Split of semio-s-artifact-stdio-ifc-4 (885535 B / 63 files / 13607 lines)
Shared `🔎️details` tree (mounted by ifc-4 and ifc-4x3) became a crate chain, lowest first (bytes / lines, limit 262144 / 4000):
- `semio-s-artifact-stdio-ifc-domain` `📚️catalog/⚖️domain` (cursor.rs, core, membership, ifc4 values; 50991 / 343). Now also holds `impl DomainPopulation for IfcSnapshot` (orphan rule).
- `-ifc-details-profile` `🔎️details/🧭️profile` (IfcDetailsProfile, type_metadata, candidate_rules, key; 11547 / 203). `IfcTypeCandidate` no longer names the roster: it borrows an `IfcCandidateCheck` object (breaks the profile<->roster cycle).
- `-ifc-details-wire` `📨️wire` (pointer, receive, wire, preimage, inverse, source_loan; 99661 / 2016)
- `-ifc-details-type` `🔤️type` (type_input*, type_remap, constituent_roster, requested_type; 103451 / 1656). `RetainedTypeAdmission::candidate::<C>` builds the check.
- `-ifc-details-edit` `✏️edit` (reference, value_edit, contextual*, domain_edit, domain_work; 143190 / 2903)
- `-ifc-details-provider` `👁️provider` (IfcDetailsProvider; 41605 / 720)
- `-ifc-details-guidance` `🧶️guidance` (type-guidance and its copy modules, guidance candidate/request; 210660 / 2349)
- `semio-s-artifact-stdio-ifc-details` `🔎️details` (work, admission, command_wire, entity_remove, plan/edit entry points, public re-exports; 136146 / 2053)
- ifc-4 itself now 94509 B / 23 files / 1455 lines (uses the details crate as `shared`); ifc-4x3 `editor/details.rs` switched to the same crate.
Registered: `ifc/Cargo.toml` members + workspace.dependencies, `📋️build-owners.json` (8 entries), manifests under `**/📦️packages/🦀️rust`. Cargo.lock untouched.
Tools: `1101-tools/ifcsplit/{split,manifests,gate.ts,deps2,rootgraph}`; originals snapshot `🗑️generated/1101/ifcsplit-orig`.

## Open
- Lockfiles (ifc workspace and hub): coordinator.
- First compile will also surface framework API drift in this never-compiled code; cross-crate visibility fixes expected.
- Unit tests (`*_tests.rs`) stay in `🔎️details/`, mounts re-pointed with `../`, but they use private fields and the ifc-4 test profile: not ported, not run.
- ifc-4-model: 33 files (1 over). ifc-4x3: 13.8 MB, generated tables unpacked (see 1030-result-ifcdata.md). Not done.
- Pending wasm checks: stdio-bim, stdio-semio, stdio-step-geometry, part21.

## Update (after lock)
- ifc-4-model 33 -> 32 files: dropped the 112 B `🚪️io/🪶️sqlite/🦀️.rs` shim, `snapshot` is mounted inline in the model root.
- Fixed latent drift found by the first compile: `semio-s-artifact-stdio-ifc-declarations` lacked its `semio-framework-deflate` dependency (Cargo.toml edited; may need another lock refresh); staged-form `close_step` (4 files, `RetirementStep::{Complete(0),Progress(..,1),Failure(..,1)}`); ifc-4-model sqlite `owned.rs`/`owned-graph.rs`/`pack-index.rs` (`RetireOwned` trait vs derive import, `close_step -> RetainedWorkOutcome`, `RetirementStep` work arguments, `terminal_is_empty` ambiguity).
- Logs: `C/🗑️generated/1101/ifcsplit-{1..6}.txt`. ifcsplit-1: wrong package id (`stdio-step-geometry` is not a hub package; step is reached via `semio-hub-stdio-cad`).
- ifc-4x3 is what blocks hub checks. Closure: program.rs 5.68 MB, metadata-data.rs 3.07 MB, templates/data.rs 2.45 MB, mvd/data.rs 0.39 MB, program-index-data.rs 0.18 MB, plus ~1 MB authored code in 98 files. Each table is a typed `&'static` Rust literal tree (ExpressProgram AST, StaticEntity/StaticType metadata, IfcTemplateCatalog, IfcMvdEntry, OwnerIndex) consumed zero-copy by the evaluator; packing needs a neutral JSON + pack per table and a leaked-static decoder per type, then a domain split of the authored code. Not started.

## Remaining work, in order (nothing below is verified)
1. details port to the work-accounting API (the details tree was never ported; compiler output is the todo list). Known sites, all `Result<RetainedCloneStep,ValueError>` -> `RetainedWorkOutcome`: `close_step` of IfcValueReceiver (done in receive.rs, callers not yet: value_edit.rs:484-492, type_input.rs:149,313-318), pointer.rs:528, contextual.rs:361, contextual_edit.rs:217, domain_work.rs:94, reference.rs:692, constituent_roster.rs:400,927, admission.rs:218, command_wire.rs:58, entity_remove.rs:98, candidate rules trait (`close_step`), work.rs:749-818. `source_loan.rs` needs the staged `RetainedCloneSource::admission_borrowed` (advance / take_output / return_inputs) instead of `admit_borrowed`; same for any other `admit_owned/admit_borrowed` (about 70 hits, `rg -c 'admit_owned|admit_borrowed|\.receiver\(\)' 🔎️details`). Done mechanically already: all `RetirementStep::{Complete,Progress,Failure}` literals in 🧶️guidance (work argument 0/1).
2. Port the unit tests: lower-crate tests need a `DetailsTestProfile` (plan: `testkit` feature in profile crate over the ifc4 catalog) and dev-dependencies (serde_json, trace) which change the lock; work tests need the ifc-4 editor, so mount them from ifc-4. Test mounts already point at `../x_tests.rs`; `DetailsTestProfile` path depth already adjusted in the 5 tests that import it.
3. ifc-4x3: per table a neutral JSON + IFCP-style pack + leaked-static decoder (ExpressProgram AST 5.68 MB, StaticEntity/StaticType 3.07 MB, IfcTemplateCatalog 2.45 MB, IfcMvdEntry 0.39 MB, OwnerIndex 0.18 MB; evaluator consumes `&'static` zero-copy, metadata.rs documents "no request-time allocator"), then split the ~1 MB authored code (validation machine, mvd concepts, templates, io/native) into crates. Not started.

## Also changed in this pass
- ifc-4-model: sqlite snapshot `native/owned*.rs`, `pack-index.rs` ported (RetainedWorkOutcome, RetireOwned trait/derive import, RetirementStep work argument); ifc-4-mutations `set-entity-complex` split into MutationKind + ColdMutationKind like its siblings; staged-form 4 files; `ifc-declarations/Cargo.toml` gained `semio-framework-deflate` (lock refresh may be needed).
- Wrong package ids wasted two queue slots: use `semio-hub-stdio-bim`, `-stdio-semio`, `-stdio-cad`; `semio-s-artifact-stdio-step-geometry` is not a hub package.

## Update (ifcsplit-9..14)
- Ported to the work-accounting API by hand: receive, pointer, source_loan (staged `admission_borrowed`, manual retirement), wire publication close, constituent_roster and contextual_edit (cursor `output_ready`/`take_demand`/`take(grant)`), value_edit, domain_work, reference, contextual*, admission, command_wire, entity_remove, work.rs (staged source admission field `source_admission`, `take`/`advance` outcomes, closed through `unborn`), guidance (RequestCopy staged `admission_owned`, close_owned/cursor_step, 25 `admit_original_construction` closures through `costed`). Helpers: `retained.rs` in the profile crate (`owned_step`, `cursor_step`, `idle_step`, `refused`, `begun`, `costed`). `IfcCandidateRules::close_step` now returns `RetainedWorkOutcome` (ifc-4x3 `NativeCandidateRules` must follow; lane ifc4x3).
- Work values are 1 per bounded action and 0 for refusals/idle turns; no framework crate was edited in this pass (the earlier staged-form edit stays).
- Fixes found by the compiler: `pub fn key`, `IfcEnumerationQuery: Copy`, `match &*event` borrow in value_edit, declarations path in reference.rs, ifc-4 imports (`io::{binary,text}`, `ifc-4-inferences`, `TreeWindowKit`, `app::SurfaceKind`, `window_kinds.iter_mut()`).
- Remaining: (1) re-run `chk.sh ifcsplit-15` for ifc-4 once os-store-replay compiles; (2) unused-import warnings (about 100 in guidance, 50 in details) to prune; (3) tests: `*_tests.rs` still unmounted-compatible only under `cargo test`, need `DetailsTestProfile` testkit feature and dev-deps (lock change); (4) hub checks after ifc4x3 lands.

## Coordination with lane ifc4x3 (NativeCandidateRules)
`IfcCandidateRules` (profile crate `🔎️details/🧭️profile/candidate_rules.rs`) changed: `close_step(&mut self, grant) -> RetainedWorkOutcome` (was `Result<RetainedCloneStep,ValueError>`) and `IfcTypeCandidate::new(owner:&dyn IfcCandidateCheck, snapshot, mutations)` (public accessors `check/snapshot/mutations/position` unchanged). `ifc-4x3` `🧬️schema/⚖️validation/local.rs` `impl IfcCandidateRules for NativeCandidateRules` must return `RetainedWorkOutcome{result,work}` from `close_step`; import it from `semio_framework_value::RetainedWorkOutcome`. `editor/details.rs` already uses the `semio-s-artifact-stdio-ifc-details` crate (`use ... as shared`).
