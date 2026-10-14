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
