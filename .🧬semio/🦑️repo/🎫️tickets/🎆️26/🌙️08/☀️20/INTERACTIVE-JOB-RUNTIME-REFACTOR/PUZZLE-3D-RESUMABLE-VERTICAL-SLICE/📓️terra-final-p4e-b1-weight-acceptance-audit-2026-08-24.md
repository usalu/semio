# Terra Final P4e B1 Weight Acceptance Audit — 2026-08-24

## Verdict

**GREEN for P4e-B1/B2 and retained P4e/P4d static acceptance.** The prior B1 coverage gap for `ObjectWeights` and `VortexWeights` is remediated. The scoped Puzzle static gate reaches and accepts the P4d/P4e baselines and all P4e mutation self-tests. Its final DENY is exclusively the unrelated active P1q DB findings described below.

P5b was not started.

## Evidence Read

- Root and Puzzle `AGENTS.md` instructions.
- Prior RED audit: `📓️terra-independent-p4e-b1-b2-remediation-acceptance-audit-2026-08-24.md`.
- Updated implementation report: `📓️sol-p4e-constructor-spatial-checkpoint-preview-implementation-2026-08-24.md`.
- Current P4e fill, geometry, mounted precompute, schema, transport, renderer, and interactivity verifier sources.

## B1 — Ten-Root Capacity Contract

`PreparationCapacityBranch`, `preparation_capacity_refusal`, and the constructor hostile-root fixture now enumerate all ten roots:

1. fixture objects
2. fixture attractions
3. fixture target volumes
4. meshes
5. catalog objects
6. catalog vortices
7. catalog cables
8. kind compatibility
9. object weights
10. vortex weights

At the fixed cap, each branch cooperatively reaches `PrepareTargets` within the fixture's bounded-turn guard. At cap plus one, the preflight happens before preparation mutation and retains the first failed branch and omitted index `32`. The fixture reads the still-owned exact source item. For both weight roots it additionally requires the precise source key and f64 value:

- `object-weight-32`, `32.25`
- `vortex-weight-32`, `32.5`

Before and after the rejection/fault path, it asserts all ten retained destinations are empty: the three fixture collections, three catalog collections, compatibility, meshes, object weights, and vortex weights. This verifies no partial destination mutation on every hostile plus-one case.

## B2 — Qualified Rejection Before Fault

On a refusal's first current-owner grant, `InteractiveJob::step` marks the refusal published, removes any candidate ghost, sets `preparation-capacity:<branch>`, and calls `publish_preview`. That publication advances the preview sequence and carries the operation/revision/generation identity; the dedicated fixture sets and asserts the registry generation as well. Only the following grant returns `fill-preparation-capacity`.

The transport emits `fillBuildPreview` even without a ghost, and the renderer validates the five-part monotonic identity, renders the diagnostic independently, and admits a fill ghost only when the accepted diagnostic owns one. This preserves the qualified no-ghost diagnostic contract.

## Verifier Mutation Defense

The P4e predicate's branch, source-length, and fixed-owner arrays include both weight cases. The B1/B2 mutation block has **21 faithful mutations**:

- ten missing-preflight mutations, including `missing-object-weight-preflight` and `missing-vortex-weight-preflight`;
- eight dynamic-owner mutations;
- catalog-cap-fixture removal;
- fault-before-diagnostic inversion; and
- no-ghost transport omission.

The self-test applies every mutation and rejects it; the baseline is then accepted. Thus the two new weight preflight mutations are not merely declared—they are killed.

## Retained Contracts

- P4e still requires the one generation-owned mounted preparation path, fixed resumable spatial ownership/query/replacement, no dormant whole-state checkpoint/clone/rebuild-clear escape, and bounded canonical preview/ghost-independent renderer transport.
- P4d's registry-exclusive admitted owner, reclaimable `Closing`, checked nonzero revision/generation allocation, exact aggregate credit, and R7/R8/R9–R11 fixture predicates remain in the same scoped static route and were not rejected.

## Scoped Gates Run

- `rustfmt --edition 2021 --check` on the six P4e Rust files: **PASS**.
- Scoped `git diff --check` on the P4e Rust/renderer/verifier files: **PASS**.
- `bun 📜️script.ts verify interactivity --self-test`: **Puzzle P4d/P4e PASS**; the command ultimately exits DENY solely for active P1q DB blocking-bridge findings in `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️component.rs`:
  - missing fixed-page construction/exact rejected-owner handback for DB input ownership;
  - missing 16-KiB-page, per-operation, process-byte, or item I/O caps.

No Cargo, Nx, Wasm, browser, network, broad build, or runtime gate was run. Runtime acceptance is not claimed.
