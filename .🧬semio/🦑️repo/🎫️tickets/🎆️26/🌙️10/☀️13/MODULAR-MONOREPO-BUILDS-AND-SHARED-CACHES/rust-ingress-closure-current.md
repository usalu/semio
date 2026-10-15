# Current Rust Ingress Closure

Static read-only route audit; no commands/builds/setup or Source mutation. This inventories reachable Repo developer routes, not a blanket whole-monorepo ingress qualification. Shared Storage changes are in progress.

## Route Map and Findings

| Reachable developer route | Actual owner | Current boundary |
|---|---|---|
| Canonical inferred native Cargo build target | library `🏗️build/🕸️graph/🦀️rust/🟦️.ts:20–24` → selected units | Per-unit Nx applies only exact canonical command, native execution kind, quoted manifest, and supported adapters. |
| Repo package build targets listed below | package `📜️script.ts:13` → runCmd → CargoRelayScript | Real bounded compiler/dependency storage and authored Source admission, but ordinary Cargo compilation bypasses per-unit Nx artifact reuse. |
| Coordinator Rust run launch | `.vscode/launch.json` name `🛠️dev🧰️repo🖥️coordinator🦀️rust` → `@semio-tech/repo-coordinator-rs:run` → package script line 27 | Calls Cargo run via relay; bounded compiler path, no per-unit Nx graft. |
| CLI build/run/terminal owner | `⌨️cli/📦️packages/🦀️rust/📜️script.ts:20,36,48` | Calls buildRepositoryCargoArtifacts directly; finite compiler and dependency owner present, dedicated per-unit binary adapter unauthored. |
| Dashboard build/launch owner | `🎛️dashboard/📦️packages/🦀️rust/📜️script.ts:130` | Same actual bounded artifact builder, bypasses selected per-unit graft; binary deliverable frontier. |
| workspace:deps-cargo | root project target line 2718 → bootstrap dependencies native script `sync cargo` | Real prepared update/fetch pair under withRepositoryCargoPreparationV1; dependency orchestration is correctly uncached. |
| workspace:deps-cargo-lock | root project target line 3437 → same script `sync cargo-lock` | Actual all-workspace Cargo update; guard admits storage overrides before execution, normal bounded preparation scope. |
| repo:cargo-prepare / workspace:cargo-prepare / repo:cargo-synchronize | normal launch entries → preparation script | Recipe compilation/dependency update/fetch keeps Original recipe Source and bounded preparation store; not compiler artifact tasks. |
| dependency tooling targets | bootstrap native script `cargo install`, `rustup target add` | Tool provisioning frontier; cargo install is compiler-producing but classified as dependency-tool execution. Storage owns that qualification. |
| registered PNG locked-metadata launch | native cargo metadata --manifest | Calls runRepositoryCommand, not compiler pipeline; metadata dependency acquisition needs own bounded dependency owner or explicit offline read-only contract. |

Root canonical selection matcher `.../🏗️build/🕸️graph/🦀️rust/🟦️.ts:20–24` only accepts `bun "<canonical cargo script>" native cargo build --manifest "<manifest>"`. The following 21 actual declared Repo builds instead call `bun ./📜️script.ts build`, whose body calls `runCmd("cargo",["build","-p",...])`; they are normal published project targets and cannot match this selection predicate. They do not bypass finite compiler storage: library `🏃️process/🟦️.ts:149–158` relays every cargo runCmd into `CargoRelayScript` (`⚡️caching/📦️artifacts/📋️native-orchestration/🟦️.ts:38–44`), which acquires the issued compiler store and bounded dependency scope. The defect is missing per-unit integration/qualification, not proof of global unbounded writes.

## Accepted Relay Classification Gap

Library `🏃️process/🟦️.ts:85–89` recognizes compile operations only from args[0] in build/rustc/check/run/clippy/test/bench/doc. The registered relay accepts arbitrary args. `cargo +nightly build` therefore avoids compile classification, Source census, Cargo artifact argument projection and build lease; `cargo install` also bypasses those compiler-specific steps. New `admitRepositoryCargoStorageArgumentsV1` still blocks target/config storage overrides in the dependency wrapper, so this is not a demonstrated arbitrary target path escape. A toolchain-prefix parser/refusal and explicit install tool owner must run before fetch/preparation, not after compiler dispatch. No normal launch with +toolchain was found; this is an accepted relay-input gap, lower priority than the actual package launch/target routes.

## Minimum Ownership Edits

1. Hand-update the 21 declared library build targets to the canonical native Cargo build owner, or add schema-owned explicit selected build command metadata consumed by selectedCargoBuildOwnerV1. Do not regex arbitrary scripts or grant cache:true to aggregate Cargo. Preserve package selectors and explicit mode; executable package kinds still require their actual binary adapter frontier.
2. Keep run/test/check as their separate normal compiler owners until genuine corresponding unit adapters exist. Route libraries to shared finite compiler/dependency custody and qualify their source/config/execution inputs. Do not claim per-unit production reuse for these operations from library build proof.
3. Add a closed relay command grammar that handles/rejects +toolchain and compiler-producing install before operation classification. Tool installations/rustup target writes need the existing tool-capacity owner; retain update/fetch writer leases and Original dependency network grant.
4. Locked metadata route currently bypasses the bounded dependency callback; project its normal read-only metadata through that owner or use offline locked admission. Existing normal metadata launch makes this reachable, not speculative library use.
5. Preserve editor route ordering and both-mode qualification routes. Existing launch covers coordinator run, dependency sync/lock, prepare/synchronize and targeted ingress laws. Package build integration can retain current project names; no new launch entry is needed where inferred/normal project build routing already owns it.

## Declared Package Build Inventory

- `@semio-tech/repo-yaml-rs:build` — `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧾️yaml/📦️packages/🦀️rust/📜️script.ts:13`
- `@semio-tech/repo-goals-rs:build` — `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎯️goals/📦️packages/🦀️rust/📜️script.ts:13`
- `@semio-tech/repo-codebase-rs:build` — `🧰️framework/🛍️products/🦑️repo/🔨️modules/🗂️codebase/📦️packages/🦀️rust/📜️script.ts:13`
- `@semio-tech/repo-metrics-rs:build` — `🧰️framework/🛍️products/🦑️repo/🔨️modules/📊️metrics/📦️packages/🦀️rust/📜️script.ts:13`
- `@semio-tech/repo-hooks-rs:build` — `🧰️framework/🛍️products/🦑️repo/🔨️modules/🪝️hooks/📦️packages/🦀️rust/📜️script.ts:13`
- `@semio-tech/repo-tree-rs:build` — `🧰️framework/🛍️products/🦑️repo/🔨️modules/🌳️tree/📦️packages/🦀️rust/📜️script.ts:13`
- `@semio-tech/repo-test-runner-rs:build` — `🧰️framework/🛍️products/🦑️repo/🔨️modules/🏃️test-runner/📦️packages/🦀️rust/📜️script.ts:13`
- `@semio-tech/repo-contributors-rs:build` — `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧑️contributors/📦️packages/🦀️rust/📜️script.ts:13`
- `@semio-tech/repo-move-rs:build` — `🧰️framework/🛍️products/🦑️repo/🔨️modules/🚚️move/📦️packages/🦀️rust/📜️script.ts:13`
- `@semio-tech/repo-todos-rs:build` — `🧰️framework/🛍️products/🦑️repo/🔨️modules/📝️todos/📦️packages/🦀️rust/📜️script.ts:13`
- `@semio-tech/repo-coordinator-rs:build` — `🧰️framework/🛍️products/🦑️repo/🔨️modules/🖥️server/🎛️coordinator/📦️packages/🦀️rust/📜️script.ts:13`
- `@semio-tech/repo-tickets-rs:build` — `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎫️tickets/📦️packages/🦀️rust/📜️script.ts:13`
- `@semio-tech/repo-languages-rs:build` — `🧰️framework/🛍️products/🦑️repo/🔨️modules/🗣️languages/📦️packages/🦀️rust/📜️script.ts:13`
- `@semio-tech/repo-model-rs:build` — `🧰️framework/🛍️products/🦑️repo/🔨️modules/📐️model/📦️packages/🦀️rust/📜️script.ts:13`
- `@semio-tech/repo-graphql-rs:build` — `🧰️framework/🛍️products/🦑️repo/🔨️modules/🔗️graphql/📦️packages/🦀️rust/📜️script.ts:13`
- `@semio-tech/repo-events-rs:build` — `🧰️framework/🛍️products/🦑️repo/🔨️modules/📡️events/📦️packages/🦀️rust/📜️script.ts:13`
- `@semio-tech/repo-providers-rs:build` — `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧩️providers/📦️packages/🦀️rust/📜️script.ts:13`
- `@semio-tech/repo-identity-rs:build` — `🧰️framework/🛍️products/🦑️repo/🔨️modules/🪪️identity/📦️packages/🦀️rust/📜️script.ts:13`
- `@semio-tech/repo-search-rs:build` — `🧰️framework/🛍️products/🦑️repo/🔨️modules/🔎️search/📦️packages/🦀️rust/📜️script.ts:13`
- `@semio-tech/repo-workspace-rs:build` — `🧰️framework/🛍️products/🦑️repo/🔨️modules/🏠️workspace/📦️packages/🦀️rust/📜️script.ts:13`
- `@semio-tech/repo-statutes-rs:build` — `🧰️framework/🛍️products/🦑️repo/🔨️modules/📜️statutes/📦️packages/🦀️rust/📜️script.ts:13`

## Current Integration Checkpoint — 2026-10-15

The earlier minimum-edit inventory is historical diagnosis. All 21 ordinary package targets now carry closed explicit Rust build intent and canonical Nx entry dispatch; actual target-guard/selector/declaration laws are both-mode five tests / 520 assertions. Selected graph consumption belongs to the Nx owner, including descriptor-directory roots and explicit uncached/no-output typed fallback for supported foreign/proc-macro/build-script/binary frontiers. Aggregate fallback is not per-unit reuse. Narrow imports and task-qualified catalog admission are also both-mode qualified. Ordinary YAML has repeatedly reached real selected preparation but has not yet compiled: latest failure was the actual Node workspace-root environment projection, now independently corrected by Nx and awaiting the protected SDK/pair checkpoint before retry. Details remain in rust-normal-yaml-runtime-current.md.

Current captured Cargo execution has a genuine retained consumer API with separate finite stdout/stderr and exact native status/signal; private toolchain, Source, registry, compiler capacity and build/target/store owners remain held through callback consumption. Run graph planning projects only run to exact selected build and excludes runtime argv; actual execution remains run. Generated capture now reads a private Rust-backed file through bounded no-follow descriptor custody and supports shell preview without system temporary targets. Shared output delivery honors cancellation and the unchanged Original deadline. Current tests are both-mode nine / 98, with actual Rust main/generated output/oversize refusal and independent native/status/schema/AST/sink oracles; expanded current strict closure is green. Details: rust-cargo-separated-capture-current.md and rust-cargo-generated-output-current.md.

All 36 existing plain Rust mutation bridge entries now declare their original binary to the shared async Original capture owner. Both-mode three tests / 237 assertions cover exact declarations/profiles and an actual tiny private Cargo bridge with awaited JSON delivery; actual full production bridge dependency graphs remain unqualified. Seven richer collector providers remain a distinct current protocol frontier. OS shell check/typegen/schema/preview are async current callers, while actual Shell/Kernel production compilation remains unqualified. Generic workspace preparation now has finite cadence and bounded terminal diagnostics, both-mode one / seven plus strict green; actual named native retry remains Root's separate obligation. The 123-match/90-script raw writer census is a review queue rather than a measured unsafe count, and the larger central caller inventory remains open in rust-writer-ingress-current.md. No all-writer cap or all-package per-unit cache claim is justified.

## Current Rich Collector Stable Checkpoint

Shared collector/envelope laws are both-mode green (5 tests / 275 assertions each), narrow strict exit 0. The first actual grid2d package retry is terminal at corrected physical Source admission for `semio-framework-3d` (before byte allocation), with guards unchanged. Owned lock maintenance had passed. All own handles terminal and shared Source released; six rich provider routes remain unchanged. See [the exact current evidence](rust-mutation-collector-current.md).

## Six Rich Entry Adoption Terminal

The remaining six entries now reuse the unchanged bounded collector with exact binaries/packages/default features and all eleven actual coordinates retained. BOTH4/349 + expanded actual-script strict0; real tiny Cargo proves selected mode and issued10/10 host capacity. All six actual production entries genuinely stop at actor physical Source admission before allocation; no compiler/inventory success. Central editor policy owns scheduling/profile/incremental; all own handles terminal and Source stable. Full evidence and current frontiers are in [the collector report](rust-mutation-collector-current.md).
