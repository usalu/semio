# Rust Physical Admission Independent Review

Read-only review, 2026-10-15. No edits, gates, Originals, native commands, deletion or physical measurements. Root supplied Rust16 fullness and root sizes; these are not independently remeasured. Coordinated exact lock findings with Nx execution. The chosen contract is hard global 16 GiB backing plus initial physical headroom under retained actual writer exclusion, not a per-mode 4 GiB physical quota.

Paths below are relative to `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/`.

## New check-to-writer lifetime is explicit

`💾️storage/🔒️enforcement/🏗️compiler/📏️capacity/🟦️.ts` 14–20 acquires one exact image-root writer EX, observes statfs under that guard and holds it through the awaited body. Compiler owner 37–38 places reservation recovery, directory creation and callback inside that scope. This closes the simple check-then-unlocked-writer race for participating store callbacks. It is initial headroom only: no claim that a 4 GiB ledger reservation enforces a per-mode physical quota, bounds every writer's actual growth, or reserves against unparticipating Nx/cache writers.

## P1: Concrete parent/child self-wait

`🦀️cargo/📦️units/🧪️tests/🟦️.ts` 110 already holds `withRepositoryCompilerStoreV1` while spawning and awaiting the sealed compiler `unit compile` child. Child `🦀️cargo/📦️units/📜️script.ts` 31 takes output EX, then 35 opens its own compiler store, which now requests the same global writer EX. Parent holds it until child returns; child cannot reach return until parent releases. AsyncLocalStorage at capacity 15 only refuses same-process reentry; it does not detect this distinct child process. Sent exact seam to Nx. Do not add an inherited-environment “writer held” bypass: another process needs actual authenticated ownership delegation or a rearranged scope with no enclosing parent writer held across child store acquisition.

## P1: Maintenance lock order must account for output-first producers

Current unit producer order is output EX (script 31) → global compiler writer EX (35) → dependency readers inside rustc compiler. A new retirement owner taking global writer then waiting for output EX can cycle with a producer already holding output and waiting for writer. Reclaimer must nonblockingly skip held output/readers or establish one consistent order before all paths run. Same-root recursive helper entry currently refuses immediately rather than silently reacquiring; preserve that honest behavior or explicitly route the same actual owned operation without a fresh lock.

Current ordinary production planning can use sequential store scopes safely, but the generic callback is broad enough to spawn children, tests, planning helpers or other stores. All such real callsites need review before broad writer wrapping is qualified. The guard currently drains only awaited callback work; arbitrary detached writers are not enrolled by this new helper.

## Reclaimability is narrower than root size

Layout owner `📍️layout/🟦️.ts` projects compact `b/p`, `b/r`, mode captures/tmp and shared Cargo home. It does not grant deletion authority over earlier `cargo/prototype/build` or arbitrary Nx material. Existing inactive maintenance `🧹️pruning/🏪️budget/🟦️.ts` 29–43 acquires cache-prune/store EX, refuses visible compiler processes, holds actual physical Cargo locks, checks complete descriptors, re-censuses and deletes exact selected units. Its current scanner at 35 selects only `scanCargoBuildUnits`, whose implementation in pruning 147–158 recognizes nested Cargo build/package/hash units. It is not a whole-root reclaimer for deps, new random scratch, unit publications, old unknown layouts or Nx cache. Root-size totals therefore do not prove enough eligible reclaimable bytes for a 4 GiB initial floor. Preserve typed unmet-headroom refusal when actual safe candidates cannot fund admission.

Old baseline/unknown roots remain charged unless their existing owner and current physical/open-descriptor exclusion are proved. New global writer alone excludes only newly participating callbacks; it does not revoke old Cargo descriptors or protect independent Nx artifact readers. Existing store/physical locks and exact unit reader guards remain necessary.

## Failure and publication

Unit compiler `🦀️cargo/📦️units/🏗️compiler/🟦️.ts` 19–20 retires scratch on compiler/receipt/input failure but swallows cleanup failure to preserve the primary error. Source review cannot treat that as proof failed bytes are removed after abort/full filesystem. Unit script 36–38 measures/copies artifact publication while still inside the cohort callback, uses bounded publication reservation, preserves prior output/charge on partial rollback, refuses unowned previous charge, and finally retires result scratch. Retain global writer until real publication and cleanup settle. An initial free-space check is not sufficient permission to publish an artifact if its exact byte reservation or physical copy fails; existing refusal must remain.

At present the physical floor executes before compiler charge recovery/directory writes, so a full image refuses ordinary store entry. Maintenance/recovery must have a separate narrowly owned real writer path that can reclaim without first satisfying the same growth floor, or it will be unable to repair the very condition being refused. That path must retain all existing reader/physical exclusions and cannot call the floor wrapper recursively.

## Qualification still open

Need actual nested-child completion/refusal law, opposite lock-order contention law, active-reader preservation, failure/cleanup publication refusal, and sufficient authenticated reclamation observed under the same writer through next accepted growth. No such runtime claim follows from this audit. The new helper and its initial floor are not yet runtime integrated proof.
