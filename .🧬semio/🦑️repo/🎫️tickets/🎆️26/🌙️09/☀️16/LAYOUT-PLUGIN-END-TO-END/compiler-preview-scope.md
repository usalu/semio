# Compiler Preview Scope

The successful diagnostic preview12746 does not census the pooled Writer build lane. This is a defining-source scope mismatch, not evidence of aliases or a whole-image empty plan. No Source changes, new runtime request, cache deletion or peer action were performed for this investigation. Root released setup4 to the dependency owner; Source edit and other runtime HOLD remain in effect.

## Exact path derivation

Let R be the issued Rust backing root `.🧬semio/🦑️repo/⚡️cache/rust/fixed/m`.

- `caching/storage/enforcement/compiler/layout` (`📍️layout/🟦️.ts:17`) projects prototype to R/b/p and R/t/p using authored policy. It does not project a pooled lane.
- `caching/pruning/budget/script.ts` passes R/b/p as buildDirectory and R/t/p as the sole relatedDirectory to the inactive maintenance executor.
- `caching/pruning/budget/execution/🟦️.ts:46` requires exactly those canonical roots. At line48 both build and incremental scanners receive only R/b/p. The relatedDirectory participates in maintenance custody at line50, not scan enumeration.
- `caching/pruning/🟦️.ts:153,174,223` searches only descendants of the supplied root with maximum traversal depth8. It skips symbolic links during traversal/measurement and does not walk upward into siblings.
- `caching/cargo/unit-cache/launch/🟦️.ts:102` defines unitCacheRootV1(R) as R/u.
- `library/process/cargo/launcher/🟦️.ts:227` selects withRepositoryCargoLaneV1 for store=shared with that R/u root. Lines101–106 return the acquired lane build/target directly.
- `caching/cargo/unit-cache/lane/🟦️.ts:11–14` and its authored policy derive R/u/l/<lane>/b and R/u/l/<lane>/t. The actual Writer4 ENOSPC path R/u/l/p0f8a58/b/prototype is a descendant of that lane, not R/b/p.

The paths above use readable domain labels; their exact repository emoji paths are retained in the existing command/source packets. No physical image inventory or symlink-alias census was run. The canonical source path construction supplies no alias from the preview root to pooled lanes.

## Separate automatic owner already covers lanes

`caching/cargo/unit-cache/gc/🟦️.ts:118–130` operates from R/u, passes dirname(root)=R to incremental and build-unit collection, and records claimed collector outcomes. The build-unit schema policy bases explicitly include b/*, t/*, u/l/*/b, u/l/*/t, components/*/b and components/*/t. `unitProfilesV1` expands only these authored bases and bounded profile depth. The existing source test specifically expects both b/p/prototype and u/l/d/b/prototype. This is source evidence only, not a fresh execution or proof any particular lane is eligible.

## Meaning of the actual preview result

The retained preview plan had no selected units, zero selected bytes, unmetBytes0, remainingBytes2857923 and originalAvailableBytes28874031104. Its 4GiB retained/available targets did not require deletion of the observed shared-root bytes. It cannot establish zero pooled-lane bytes, absence of active exclusions, or eligibility of Writer or peer units. Free-space growth is not attributed to this preview.

## Required future correction

A declared whole-compiler inactive preview must use the existing compiler-image/lane ownership model and report the actual roots it covers. Merely substituting R for buildDirectory would violate the current executor root contract and would change lease, descriptor and relative-unit assumptions. Any future correction needs schema-first neutral tests, genuine bounded custody and active-lane exclusions, plus normal managed validation. It is not authorized by this read-only task and must wait for setup4 terminal and Root direction. No retirement is authorized.

## Current owner routes and intended scope

The declared `repo-cache-retirement:cache-retire-inactive-preview` dashboard description says current compiler store, with no lane-specific qualification. Its execution still intentionally enforces the canonical nonlane roots. Those roots remain current: compiler storage creates them and installs their CARGO_BUILD_BUILD_DIR/CARGO_TARGET_DIR defaults; isolated Cargo uses the current store layout. They must not be removed or described as obsolete.

The same lane-aware automatic GC is entered by the genuine shared Cargo launcher through `relieveUnitCachePressureV1`, headroom admission and out-of-space handling. It runs under the actual compiler store opened for that invocation. Its source also exposes internal `--semio-unit-cache` plan/gc commands, but these are not a substitute for a declared owning route and were not invoked. No declared target for that internal operator command was found in the caching project manifests inspected.

The collector takes a single-flight claim. Whole-lane removal takes the exact lane claim and rechecks live markers at effect time. Build-unit collection observes authored profile roots, probes their actual build leases, excludes held unit locks, keeps newest variants under a busy profile, uses the stronger busy-age threshold, and retakes profile/unit locks before deletion. Incremental collection retakes each session lock and skips held sessions. These are Source protections, not proof that a specific current unit is eligible. Authored age values are 30min idle-profile minimum, 3h busy-profile minimum, 24h superseded idle threshold and newest2 per crate for the build-unit planner. No capacity increase, policy override, peer stop, collector invocation or deletion is part of this investigation.

Root accepted the scope distinction and chose no new maintenance implementation: continue the actual goal after setup4 terminal, using the existing genuine Cargo owner and its automatic lane GC when separately authorized.

The final read-only source-reference search completed EXIT0 over5375 project manifests, excluding .git, node_modules, .cache, target, dist, generated output and ticket trees. It searched literal references to the internal operator/collector names and inactive target; only the existing budget project matched. This is a bounded textual declaration search, not runtime graph inference, a physical cache census, or proof against dynamically generated declarations. No additional declared lane-GC route was identified.
