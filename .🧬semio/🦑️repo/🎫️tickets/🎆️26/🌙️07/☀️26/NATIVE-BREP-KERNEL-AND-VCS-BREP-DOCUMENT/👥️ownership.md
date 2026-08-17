# Ownership — Native Brep Kernel and Vcs Brep Document

**Ticket:** `26/07/26/NATIVE-BREP-KERNEL-AND-VCS-BREP-DOCUMENT`
**Wave 0:** baseline (this file + stubs). Later waves MUST stay inside disjoint globs below.

## Integrator (single owner)

Exclusive edit rights for repo-wide glue; merges `📥️integration-requests.md` entries.

| Glob / path | Notes |
|---|---|
| `Cargo.toml` | workspace members, `[workspace.dependencies]` |
| `Cargo.lock` | lockfile after dep churn (Wave 6 drops brepkit-*) |
| `✏️s/🔨️modules/🧊️3d/📦️packages/🦀️rust/📦️glue.rs` | module registry |
| `✏️s/🔨️modules/🧊️3d/📦️packages/🦀️rust/Cargo.toml` | crate deps/features |
| `✏️s/🔨️modules/🧊️3d/📦️packages/🦀️rust/📋️project.json` | nx targets |
| `✏️s/🔨️modules/🧊️3d/📦️packages/🦀️rust/📜️script.ts` | test/lint/bench router |
| `.vscode/launch.json` | launch entries |
| `✏️s/🔨️modules/🧊️3d/📐️brep/⚙️engine/**` | frozen `BrepKernel` trait until Wave 6 |
| `✏️s/🔨️modules/🧊️3d/📐️brep/🧰️kernel/**` | Wave 6 flip only |

**Rule:** Every other agent that needs an integrator-only change MUST append to `📥️integration-requests.md` (append-only).

## Frozen contract

- `BrepKernel` trait + transfer types in `⚙️engine/🦀️component.rs` stay frozen until Wave 6.
- Lane public APIs freeze in `📐️module-contracts.md` (status: DRAFT → FROZEN).
- Cannot start a lane whose upstream contract row is still DRAFT.
- Cannot delete brepkit deps before Wave 6.

## Wave 1 — Foundations (parallel)

| Lane | Glob | Model |
|---|---|---|
| 1-bvh | `📐️brep/🌳bvh/**` | composer-2.5 |
| 1-primitives | `📐️brep/🧱primitives/**` | cursor-grok-4.5-high |
| 1-measure | `📐️brep/📏measure/**` | composer-2.5 |
| 1-tessellate | `📐️brep/🧩tessellate/**` | cursor-grok-4.5-high |
| 1-oracle | `📐️brep/🔮️oracle/**` + ticket differential harness | composer-2.5 |
| 1-int-cc | `📐️brep/✂️int-cc/**` | cursor-grok-4.5-high |

Shared read-only: existing foundation modules (vec/mat/tolerance/predicates/poly/bezier/bspline/curve/curve-ops/surface/surface-ops/arena/history/topo/euler/validate/error).

## Wave 2 — Intersect + IO (parallel)

| Lane | Glob | Model |
|---|---|---|
| 2-int-cs | `📐️brep/✂️int-cs/**` | cursor-grok-4.5-high |
| 2-int-ss | `📐️brep/✂️int-ss/**` | cursor-grok-4.5-high |
| 2-sweep | `📐️brep/➡️sweep/**` | cursor-grok-4.5-high |
| 2-sew | `📐️brep/🧵sew/**` | composer-2.5 |
| 2-step | `📐️brep/📄step/**` | composer-2.5 |
| 2-mesh-io | `📐️brep/📦mesh-io/**` | composer-2.5 |

## Wave 3 — Classify / Imprint / Heal (parallel)

| Lane | Glob | Model |
|---|---|---|
| 3-classify | `📐️brep/🏷️classify/**` | composer-2.5 |
| 3-imprint | `📐️brep/🖋️imprint/**` | cursor-grok-4.5-high |
| 3-heal | `📐️brep/🩹heal/**` | composer-2.5 |

## Wave 4 — Boolean (serial flagship)

| Lane | Glob | Model |
|---|---|---|
| 4-boolean | `📐️brep/🔀boolean/**` | cursor-grok-4.5-high |

## Wave 5 — Offset / Blend (parallel)

| Lane | Glob | Model |
|---|---|---|
| 5-offset | `📐️brep/↔️offset/**` | cursor-grok-4.5-high |
| 5-blend | `📐️brep/🎨️blend/**` | cursor-grok-4.5-high |

## Wave 6 — Flip (integrator serial)

Kernel rewrite + drop brepkit-* + rename `BrepkitKernel` → `Brep` across consumers + rewrite benches.

## Wave 7 — Hardening (parallel)

Exhaustive fuzz, consumer/wasm verification, runtime e2e (procedural-3d + CAD).
