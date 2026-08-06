# 🧮️ Framework Math Family Crate Consolidation — Completion Summary

## What landed

One Shape V2 crate, `semio-framework-math`, at `🧰️framework/🔨️modules/🧮️math/📦️packages/🦀️rust`, plus its
npm sibling `@semio-tech/framework-math-js` at `…/📦️packages/🟦️typescript`. The owner tree now holds
**67 `🦀️component.rs`** files and **2 `🟦️component.ts`** files and **zero** `⚡️implementations`
directories (was 52).

### Architecture decisions

- **One flat domain tree, no sub-crates.** Every former crate became a `🦀️component.rs` in its own
  emoji folder; `📦️lib.rs` is pure `#[path = "../../…"] pub mod …` wiring with `graph` and `wfc` as
  nested inline modules mirroring their old crate hierarchies (`graph::normal::directed`,
  `graph::ports::directed::normal`, …). Import sites therefore map mechanically:
  `mathematical_graph_normal_directed::X` → `math::graph::normal::directed::X`.
- **`wfc` was exploded, not flattened.** Its 40 `📂️src/🦀️*.rs` files each became their own component
  folder (`🔦️beam`, `⛓️constraint`, `🀄️tiled`, …) so the tree stays Shape V2-pure instead of keeping a
  `📂️src` bag. Its public re-export block was preserved verbatim.
- **Generated manifest output moved to the owner root** (`🧮️math/🤖️generated/`), per Shape V2 —
  previously it lived inside the graph-manifest crate. `build.rs` and the TypeScript component both
  point at the new location, and `build.rs` bootstraps `bun ./📜️script.ts generate` when the output is
  missing.
- **The manifest generator moved to the package** (`📦️packages/🦀️rust/📜️script.ts`) and now also owns
  the `test` and `lint` gates for the whole crate. `generate` is a `dependsOn` of every test/lint
  target on both the Rust and TypeScript projects.
- **The two TypeScript pieces were merged**, not kept as separate npm packages: the graph Jack DSL
  (`🕸️graph/🗣️dsl/🟦️component.ts`) and the manifest catalog/validator
  (`🕸️graph/🛂️manifest/🟦️component.ts`) are re-exported from one `📦️index.ts`. Their old packaging was
  already broken (`@semio-tech/graph-dsl-core` pointed at `./📦️.ts` and `js/🧪️vitest.config.ts`, both
  nonexistent), so it was rebuilt rather than ported.

### Deletions

**23 stub crates** were deleted outright, not migrated: the NetworkX-parity graph placeholders
(`approximation`, `bipartite`, `centrality`, `cliques`, `clustering`, `coloring`, `community`,
`components`, `connectivity`, `cycles`, `dag`, `flow`, `generate`, `generate/random`, `io`,
`isomorphism`, `matching`, `paths`, `planarity`, `similarity`, `spectral`, `structure`, `trees`).

The plan said 25; the disk says 23. Each had a 5-line `📦️lib.rs` carrying only the
`🚧️ Implementation lands` marker comment — the "25 lines each" in the plan does not match any file in
the family.

### Dependent search

Both directions were grepped, as the master doc demands:

- **Cargo.toml path/package strings** — no manifest outside the math family named any of the 23
  stubs; the only references were preemptive declarations between math crates themselves.
- **`use`-line imports** — no Rust source anywhere imported a stub crate's alias. The stub `lib.rs`
  files export nothing, so there was nothing to import.

24 external manifests do consume the surviving math crates; they are enumerated with their aliases in
`📋️registrar-handoff.md` §3–§4 and `🧪️external-consumers.txt`.

## Verification

The crate is not a workspace member yet (root `Cargo.toml` is registrar territory), so everything was
run through `🔧️cargo.sh`, which appends a temporary `[workspace]` block plus the repo's workspace
lints, runs cargo, then restores the manifest and removes the stray `Cargo.lock`.

| gate | result |
| --- | --- |
| `cargo check` (native) | clean (`🧪️check-final.txt`) |
| `cargo check --target wasm32-wasip2` | clean (`🧪️check-wasm.txt`) |
| `cargo clippy --all-targets` | clean for `semio-framework-math` (`🧪️clippy-2.txt`) |
| `cargo test --no-fail-fast` | **1740 passed / 13 failed** (`🧪️test-final.txt`) |
| `bun ./📜️script.ts test` (TypeScript) | 8 passed / 2 files |
| `bunx tsc --noEmit` | clean |
| framework discovery (`role = "framework"`) | both packages found, `maturity: "clean"` |

The baseline (`🧪️baseline-tests.txt`, captured across the 51 separate crates before any change) was
also **1740 passed / 13 failed**, with the same 13 names. Zero regressions, zero tests lost.

The 13 failures are pre-existing `cas` (integrate/limits/ode/sums) and `polynomial`
(algebraic/finite/univariate) assertions, unchanged by this ticket.

Clippy took work: `--fix` cleared 20 sites automatically and 9 were fixed by hand (`🔧️lint-fixes.mjs`)
— digit grouping, two `needless_pass_by_value`, two `type_complexity` aliases, a
`needless_range_loop`, a `Copy` derive, a const assertion, and a `get().is_none()`. Three
`unused_qualifications` warnings introduced by the `crate::` path rewrite were also cleaned up. The
generator itself was fixed too: emitted manifest bundles now widen to `GraphManifestDocument` before
projecting, and the emitted type surface gained `direction`/`blockKinds`, which made the TypeScript
side typecheck for the first time.

## Blockers

- **Root `Cargo.toml` is untouched** by design — see `📋️registrar-handoff.md`. Until §1 lands, the
  crate cannot be built through the workspace and the 51 stale member lines point at deleted
  directories, so root `cargo metadata` will fail. This is the one hard sequencing dependency.
- **A concurrent process restored the deleted `⚡️implementations` directories twice** (~12:29 and
  ~12:45), from git, along with the temporary `[workspace]` block in the new `Cargo.toml`. Each time
  they were deleted again; the tree was verified clean and stayed clean for 90+ seconds at the end.
  The registrar should re-verify `find 🧰️framework/🔨️modules/🧮️math -type d -name '⚡️implementations'`
  returns nothing before editing root `Cargo.toml`.
- `.vscode/launch.json`, root `📜️script.ts`, the os TypeScript `package.json` and the ui-styling vite
  alias still name the retired nx/npm projects. All four are out of this ticket's scope and are
  itemized in `📋️registrar-handoff.md` §5.

## Files

**Created** — `🧮️math/📦️packages/🦀️rust/{Cargo.toml,build.rs,📋️project.json,📜️script.ts,📦️lib.rs}`,
`🧮️math/📦️packages/🟦️typescript/{package.json,tsconfig.json,📋️project.json,📜️script.ts,📦️index.ts,🧪️vitest.config.ts}`,
67 `🦀️component.rs`, 2 `🟦️component.ts`, `🧮️math/🤖️generated/**` (30 files).

**Deleted** — all 52 `⚡️implementations` directories under `🧮️math/**` (51 Rust, 1 TypeScript),
including the 23 stub crates and their npm mirrors.

**Ticket scratch** — `🔧️consolidate.mjs`, `🔧️emit-lib.mjs`, `🔧️lint-fixes.mjs`, `🔧️cargo.sh`,
`🔧️discovery-probe.mjs`, `📋️registrar-handoff.md`, and the `🧪️*` logs.
