# Native Admission Memo Scope

Root measured the actual native dependency vectors separately in `build-performance.md`; this regression makes no timing claim or performance assertion. Core now creates one `nativeCache` / `nativeClosures` pair inside each original inventory Source capture and passes that pair among the capture's owner callbacks. Independent calls retain their own default maps. No whole-owner result is memoized.

The small language-neutral graph fixture `nativeAdmissionMemo` contains six Cargo crates, a normal dependency chain, and distinct root/child development dependencies. Four root/test-mode cases compare actual `nativeDependencyRoots` calls using fresh maps, shared maps and repeat memo hits. Independent @iarna TOML parsing feeds a SQLite recursive graph query; its reachability agrees with all four declared outcomes. Only the selected root's development dependencies are admitted.

The test proves actual memo hits by retaining the same cache entry and result identity. Each receiving Source capture contains exactly the same complete observations as its fresh counterpart, including physical Cargo manifests. It also calls the real `discoverBuildOwnerAdmissionFactsV1` with the production context fields and compares fresh, shared and repeated owner facts and observations.

A physical edit redirects a dependency from crate `c` to crate `d`. Existing shared maps then produce the current result matching both fresh maps and the SQLite oracle. Captured observations contain the new physical SHA and exclude the old SHA; the actual owner helper's dependency facts also update. A TypeScript AST witness checks that production allocates both maps within the `captureObservedSourceInputsV1` callback and passes those exact fields to the owner helper, so the maps do not survive a fresh inventory attempt.

The focused law passed **1 test / 69 assertions / 0.502 seconds** (`🗑️generated/native-admission-memo-green.txt`). Core was notified immediately that test and fixture edits were settled for its single coherent gate and next selected publication. This fixture does not claim a graph publication or final all-workspace acceptance.
