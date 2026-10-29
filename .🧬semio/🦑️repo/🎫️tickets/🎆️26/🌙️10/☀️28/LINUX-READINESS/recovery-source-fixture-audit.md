# Native Recovery Physical Source Fixtures

The focused recovery argument test reproduced the actual whole-workspace failure: one test passed and the argument-vector test failed after 65 assertions with "Bounded physical Source revision required".

## Exact Cause

The test used the real checkout as its fixture root and read live owner manifests through the genuine physical Source reader. The frontend owner manifest was an ordinary single-link 3515-byte file. The caching owner manifest was an ordinary single-link 105717-byte file, exceeding the recovery protocol's 65536-byte bound.

No symlink or multiple-link Source violation was found in these actual files. The global physical Source reader's retained-text limit is also 65536 bytes. Neither that limit nor descriptor, ancestry, identity or single-link checks was changed.

## Private Fixture Repair

The language-neutral argument corpus now includes handcrafted minimal frontend and caching owner declarations. They contain the actual required owner/target facts, not copied Source files or a snapshot of the growing live project manifest. Ajv validates them against the actual Owner, MaintenanceOwner and ResetOwner contracts.

The test requires its caller-owned SEMIO_TEST_ARTIFACT_DIR, creates an ordinary private fixture root there, materializes only those small owner declarations and removes its private root afterward. The actual recovery selector runs unchanged against this physical root. All existing argument vectors and the independent Python reading remain in force.

Two neutral physical refusal cases strengthen the fixture check. The real Source reader refuses a multiply linked owner file and an oversized ordinary owner file. After restoration to the ordinary bounded single-link file, genuine recovery selection succeeds again. No SourceOrigin mock or permissive read port was introduced.

## Verification

Focused handle86385 completed0: 3 tests, 187 assertions. A final combined run73069 completed0: 8 tests, 269 assertions across the recovery and lease-status suites. Full directly streamed output is temporarily retained at `🗑️generated/workspace-fixture-repairs.log`.

Only the recovery test runner, adjacent neutral corpus and corpus schema changed for this repair. Production recovery code and physical Source readers remain unchanged.

## Separate Production Limitation

This unit repair does not establish successful maintenance recovery against the current live caching owner manifest. That manifest still exceeds the existing recovery reader's bound; an actual `repo run repo:cache-reset-nx` recovery selection can encounter the same refusal. Root was informed explicitly. A production correction must retain physical Source custody and a finite authored data bound; the unit fixture repair is not evidence that this separate live-data issue disappeared.

The Nx watcher investigation was paused before any production edit, as Root requested. Its API constraints are appended to nx-daemon-watch-audit.md. No full workspace, native build or setup retry was launched by this agent.
