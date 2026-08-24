# Terra Independent P4e B1/B2 Remediation Acceptance Audit — 2026-08-24

## Verdict

**RED — one bounded, exact P4e-B1 acceptance obligation remains unproven.** The implementation repairs the prior B1/B2 source defects for the eight exercised fixture/catalog roots and has a sound source path for both weight roots. However, the required max/+1 exact-owner/no-partial-mutation proof and its faithful permanent mutations omit `ObjectWeights` and `VortexWeights`. The requested B1 contract explicitly includes weights, so P4e cannot be accepted until those two roots are included in the fixture and verifier mutations.

This is an acceptance-coverage blocker, not a claim that the current weight preflight is functionally absent.

## Evidence Read

- Root and applicable Puzzle3d `AGENTS.md` instructions.
- The prior RED audit: `📓️terra-independent-p4e-acceptance-audit-2026-08-24.md`.
- The remediation report: `📓️sol-p4e-constructor-spatial-checkpoint-preview-implementation-2026-08-24.md`.
- Current seven P4e source/verifier inputs: geometry, brush, fill, precompute, schema, scene transport, `World3dHost`, and `📜️script.ts`.
- The retained P4d R9/R11 independent acceptance audit and its current production/verifier predicates.

## B1 — Fixed Input Owners And Preflight

### Proven Source Properties

`FixedOwnerVec<T>` has one boxed fixed `MaybeUninit` slot page, enforces `FIXED_OWNER_SLOTS == 32`, returns the rejected exact owner from `try_push`, supports one-owner `pop`, and retires its backing only after semantic owners are gone (`geometry/🦀️component.rs:16-83`).

The retained fill roots are fixed owners, not ordinary destination vectors:

- `FixedFixtureOwner`: objects, attractions, target volumes.
- `FixedCatalogOwner`: objects, vortices, cables.
- `kind_compatibility`: `FixedOwnerVec`.
- meshes and both retained weight maps: `FixedOwnerMap`.

`preparation_capacity_refusal` checks all ten source collections before any `prepare_*` stage can mutate a retained destination: the three fixture roots, meshes, three catalog roots, compatibility, object weights, and vortex weights (`fill/🦀️component.rs:251-267`). It records the first branch and omitted index `32`; its test-only attribution helper resolves that same branch/index to the still-owned source item, including map-key sources for meshes and weights (`:2088-2104`). A refusal starts with all exercised destination roots empty and its first step publishes the diagnostic; it does not run preparation.

The eight-root cap/+1 fixture covers fixture objects/attractions/target volumes, meshes, catalog objects/vortices/cables, and compatibility. For each, cap reaches `PrepareTargets` in bounded turns; plus-one produces the expected branch and source-owner witness, publishes a no-ghost rejection, faults only on its following grant, and keeps every checked retained destination empty (`:3417-3526`). The generic fixed-map/vector boundary fixture also independently proves fixed backing identity and exact plus-one handback (`:3591-3660`).

No ordinary `Vec` destination remains for the retained fixture/catalog/compatibility roots. The only `Fixture` vector reconstruction is the unused explicit `FixedFixtureOwner::snapshot` projection boundary; the search path uses `FillFixtureView` and `BrushCatalogView` directly.

### Blocking Gap

The above max/+1 fixture's `HostileRoot` enum and `branches` table omit `ObjectWeights` and `VortexWeights`. It consequently never constructs a 32/33-entry weight map, never asserts their refusal branch/index/source key, and never proves those two refusals leave all destinations empty.

The claimed 19 B1/B2 verifier mutations have the same omission: eight missing-preflight mutations and eight dynamic-owner mutations cover only the eight fixture/catalog roots, followed by catalog-fixture, diagnostic-order, and no-ghost mutations. `interactivityPuzzleFillP4eFailures` likewise constructs its `preparationBranches`, `preparationLengths`, and hostile-root requirements without either weight branch. Replacing either weight length in `preparation_capacity_refusal` with `0` is therefore not rejected by this P4e predicate/self-test set.

The production source does preflight both maps and retains them in fixed maps, but that does not satisfy the requested complete cap/+1 proof or mutation defense for weights.

## B2 — First-Grant Diagnostic, Then Fault

**PASS.** A live capacity refusal enters `InteractiveJob::step` before ordinary preparation. On its first current-owner grant it marks the refusal published, forces `candidate_ghost = None`, supplies `preparation-capacity:<branch>`, and returns `publish_preview(context)`; only the later grant returns `fill-preparation-capacity` fault (`fill/🦀️component.rs:3268-3280`). `FillBuildPreview` already contains operation, base revision, registry generation, generation, and a sequence advanced by preview publication. The dedicated fixture asserts those identity fields, rejection text, no ghost, increasing sequence, and then the second-grant fault (`:3529-3557`).

The transport reads the active fill preview, refuses zero/stale/complete identity, installs `fillBuildPreview` before the optional ghost branch, and serializes a diagnostic-only object when no ghost exists (`main/🦀️component.rs:404-420`). `World3dHost` strictly validates the bounded record, requires monotonic five-part identity, displays `FillDiagnosticOverlay` independently, and renders a fill ghost only when the accepted diagnostic owns one. A missing terminal/cancelled/stale diagnostic resolves local `fillDiagnostic` to `null`, clears the overlay and prevents a ghost (`World3dHost/🟦️component.tsx:1090-1156,3819-3855,4961`).

The verifier's `fault-before-rejection-diagnostic` and `omit-no-ghost-rejection` mutations are present and were exercised by the scoped self-test.

## Retained P4e/P4d Acceptance

- The fixed, generation-owned spatial index and its resumable replacement/query machinery remain present; the four P4e spatial fixtures are still required by the verifier.
- The P4e mounted preparation path, bounded canonical preview, and no-whole-checkpoint/clone/rebuild-clear predicates remain present.
- The current P4d predicate still requires registry-exclusive post-admission access, reclaimable `Closing`, checked nonzero revision/generation allocation, exact aggregate credit, R7/R8 terminal behavior, and R9–R11 binding/exhaustion fixtures. The current source contains these required structures and the scoped self-test did not raise a Puzzle P4d/P4e failure.

## Scoped Gates

- `rustfmt --edition 2021 --check` on the six P4e Rust files: **PASS** (silent).
- Scoped `git diff --check` on the seven P4e files and verifier: **PASS** (silent).
- `bun 📜️script.ts verify interactivity --self-test`: Puzzle P4d/P4e baselines and all registered self-tests completed without a Puzzle failure. The overall command is **DENY** solely on the active unrelated P1q DB verifier findings:
  - DB input ownership lacks fixed-page construction/exact rejected-owner handback.
  - DB I/O admission lacks 16-KiB page, per-operation, process-byte, or item caps.

No Cargo, Nx, Wasm, browser, network, broad build, or runtime command was run. P5b was not started.

## Required Remediation

Extend the constructor max/+1 fixture with object-weight and vortex-weight `HostileRoot` branches. At cap and plus-one, assert exact branch/index/source key, first-preview/no-ghost then later fault, and no retained-destination mutation. Add corresponding `missing-object-weight-preflight` and `missing-vortex-weight-preflight` verifier mutations, include both branches/length inputs in `interactivityPuzzleFillP4eFailures`, and make all 21 mutations self-test. Re-audit after that change.
