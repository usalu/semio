STATUS: IN PROGRESS, UNVERIFIED (lane ownercallers; crates original-job-ownership, operation, decl-tool-contract, interaction)

Baseline: W-ownercallers-1 (native, 4 crates): 8 / 32 / 14 / 20 errors, same as W-gateway-119. Log: 🗑️generated/1105b/W-ownercallers-1.txt. Lister: 1105b-tools/ownercallers/errs.py.

## original-job-ownership
- lease/command: `retirement()` -> `type RetirementCursor` + `retirement_seed` (EDITED).
- birth: local `built` removed; `ArtifactReservedToolJob::admit_mounted(source, &mut ResidentConstruction<ErasedToolJob>, operation, cx) -> Result<Option<ErasedToolJob>,_>` uses `construction.drive` (action-bus `admit_original_construction_settled`). Caller retained-command full-operation got field `reserved_construction` (frontier x2, tests/publication-retirement-authority x3 literals) and producer.rs maps Option to candidate; `original_reserved_producer_pending` includes a non-empty resident construction.
- Gap: ResidentConstruction has no close step, so a cancel mid-construction does not retire its admission; unit tests in birth/🧪️tests (old exact-quote assertions) NOT ported.
- W-ownercallers-3 rc=0: semio-plugin-original-job-ownership lib VERIFIED native (log 🗑️generated/1105b/W-ownercallers-3.txt); after fixing one E0283 (explicit RetainedWorkOutcome generics in producer close).
