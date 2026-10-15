# Result: carve `semio-framework-print` (assets) and `semio-framework-server` (producers)

Executor: Sonnet 5.5, foreground, no sub-agents. Rules obeyed: `📓️EXECUTOR-RULES.md`, `📓️CARVING-RULES.md`, `📓️RESOLUTION-ORACLE.md`, `PRODUCER-CRATE-RECIPE.md`. Paths below are relative to `🧰️framework/🛍️products/` unless they start with `/` or `.`. Nothing was committed, staged, stashed or checked out. Tools and logs are in `🗑️generated/carvepr-*`.

## 0. Verdict

| Crate | Before (files / bytes / active lines) | After | Gate (4000 lines, 32 files, 262144 bytes; target 3500 / 28 / 230000) |
| --- | --- | --- | --- |
| `semio-framework-print` | 31 / 1,847,153 / 1,173 (11 embedded, 1,719,946 bytes) | 22 / 139,419 / 1,212 (2 embedded, 10,461 bytes) | passes, inside the target |
| `semio-framework-server` | 8 / 280,385 / 5,752 | 3 / 163,180 / 3,361 | passes, inside the target |
| `semio-framework-server-contract` (new) | | 1 / 18,113 / 428 | passes |
| `semio-framework-server-storage` (new) | | 1 / 24,247 / 397 | passes |
| `semio-framework-server-policy` (new) | | 1 / 15,464 / 329 | passes |
| `semio-framework-server-authority` (new) | | 1 / 39,452 / 768 | passes |
| `semio-framework-server-throttle` (new) | | 1 / 19,967 / 460 | passes, `closureCompilable: true` |

Rows come from the canonical counter (`readRustSourceClosureV1` through `🗑️generated/census.ts` with `ONLY=...`, log `🗑️generated/carvepr-census-after.log`; the before rows are the census rows plus `carvepr-print-closure-before.tsv`). The new server children report `oversizedDeps: 1` and `closureCompilable: false` only because their dependency `semio-framework-replication` is one of the fleet's oversized crates (a tier 0 carve target, not mine). `print` keeps `oversizedDeps: 8` (`os-kernel`, `plugin` and others, all fleet-active crates).

**Not done / limits (details in section 5):** no cargo build or test could run. `bun repo cargo ...` is refused before any compile (first `Live Original process issuance required`, a later retry `Finite original maintenance authority required`). Verification is the patched full-resolution differential plus static checks. `Cargo.lock` files were not refreshed (the launcher is the only sanctioned route and it is refused), so the next lock refresh must pick up five new server packages. Two print embeds remain (section 1.3).

## 1. Part A: `semio-framework-print`

### 1.1 What the "11 embedded assets" actually are

Not fonts. `include_str!` of the product's own authored schema documents (the compiler's fonts are a different crate; print embeds none, so there is nothing to coordinate with the font port, and `RESULT-CARVE-COMPILER.md` does not exist yet). The closure listed 11 embedded files: `🧬️schema/🔣️.json` (1,637,091 bytes, included twice), the `🔣️.json`/`🟦️.ts` documents of `snapshot`, `diff`, `mutations`, `inferences`, `🚪️io/📝️text/💡️inferences/🔣️.json` and `🚪️io/🪶️sqlite/📸️snapshot/🗄️.sql`. The five `include_str!` of Rust sources (the facet "rust" leaves) point at files that are already source in the closure.

### 1.2 The new asset channel (carving rule 3)

The repository has no generic runtime asset loader for schema documents; every registration site uses `include_str!` because `ArtifactSchemaDescriptor`, `ArtifactInferenceDescriptor`, `ArtifactInferencePayloadContract` and `register_referenced_schema_documents` all take `&'static str`. So the channel is an explicit parameter that carries exactly those leaves, in `🦀️.rs` (the lib root, so no new taxonomy folder was needed):

- `ChartSchemaAssets { artifact, snapshot, diff, mutations, inferences: FacetLeaves, inference_output_schema: &'static str }` (`Copy`). A host may build it from any source.
- `ChartSchemaAssets::read(root: &Path) -> Result<Self, ChartSchemaAssetError>` reads the fifteen documents below the product directory with `std::fs` and leaks each once (the registries keep `'static` leaves for the process lifetime, documented on the type).
- `ChartSchemaAssetError { path, cause }` names the unreadable document (`Display` + `Error`).
- Every function that used a document now takes `&ChartSchemaAssets`: `chart_artifact_schema_descriptor`, `chart_artifact_declaration`, `chart_inference_service`, `register_chart_inference`, `register_chart_artifact`, `print_plugin` (the private inference descriptor too). No default, no fallback, no alias for the old zero-argument signatures.
- Callers updated at once: the only callers in the repository are the crate's own tests (`🧪️tests/🧬️chart-mutations/🦀️.rs`: `chart_inference_service`, `register_chart_artifact` x2, and `🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs`: `print_plugin`). No other crate depends on `semio-framework-print` (`git grep` over every manifest, Rust and TypeScript file found only its own `Cargo.toml` and `📜️script.ts`).
- Tests: the shared `crate::tests::assets()` loads the assets once from `CARGO_MANIFEST_DIR/../..`. Two new tests: `runtime_schema_assets_equal_the_documents_once_compiled_into_the_crate` asserts every leaf is byte-identical to the former `include_str!` document (the old embedding is the oracle), parses each JSON leaf with independent `serde_json`, and checks the schema descriptor built from the assets; `missing_schema_assets_name_the_unreadable_document` checks the error names the first document and carries `NotFound`.

### 1.3 Remaining embeds (2 files, 10,461 bytes) and why

`🧬️schema/🧬️mutations/🦀️.rs` builds `pub const CHANGE_CHART_VALUE: MutationLeafDescriptor { payload_schema: include_str!("🔣️.json"), ... }` (445 bytes), reached through the `protocol::Mutation` trait const `DESCRIPTORS`. `🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs` has `const SQL: &str = include_str!("🗄️.sql")` (10,016 bytes) for the trait const `ArtifactSqliteSnapshot::SQLITE_SCHEMA`, implemented by about 40 crates and read verbatim by the TypeScript twin test (`🧪️tests/🟦️.ts`). Both are compile-time trait constants of `os-kernel` / `pack` (fleet-active crates), so a runtime channel needs a trait change outside this carve; inlining them as string literals would only hide the bytes. The crate passes the gate with them; moving them is a follow-up that belongs to the trait owners.

### 1.4 Print verification

| Check | Result |
| --- | --- |
| Full-resolution differential, native and `--test`, `rustc +nightly-2026-07-20 --emit=metadata -o /dev/null`, extern crates removed and external globs neutralised identically in baseline and edited tree | native 187 vs 194 errors, test 379 vs 393. Every difference is an unresolved external crate: the baseline reported one `E0432 unresolved import semio_framework_schema_registry` at its `use`, the edited tree reports eight `E0433` for the same missing crate where the type is now written in signatures; plus `CARGO_MANIFEST_DIR` (cargo-only env), `serde_json` and the schema-registry crate in the new tests. No new E0412/E0425/E0422/E0423/E0603/E0432 for an in-crate name. |
| rustfmt parse (`rustfmt +nightly-2026-07-20 --emit stdout`) | `🦀️.rs`, `🧪️tests/🧬️chart-mutations/🦀️.rs`, the sqlite test file: exit 0 |
| Census rows | section 0 |
| Existing TS runner for the shared SQLite chart law (reads the same schema JSON and `🗄️.sql`) | `bun test "../../🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts"` from `📓️print/📦️packages/🟦️typescript`: 8 pass, 0 fail, 173 expects. The second TS file (`🚪️io/🧪️tests/🏛️ownership/🟦️.ts`) registers no tests. |
| Cargo | not possible, section 5 |

## 2. Part B: `semio-framework-server`

### 2.1 Carve

Each of the five modules that sat behind `#[path]` in the package glue became a producer crate at the recipe position `🖥️server/🔨️modules/<m>/📦️packages/🦀️rust` with `[lib] path = "../../🦀️.rs"`; the module files did not move (so history, TS readers such as `🧪️tests/🔬️wire/🟦️.ts` which reads `📡️gateway/🦀️.rs`, and error locations stay stable). The gateway (3,306 lines, 159,593 bytes) stays in the parent with the reference instance and the conformance suite.

Dependency DAG, downward only (checked by the declared manifests and by the compiler's unresolved-crate list per child, `🗑️generated/carvepr-child-deps.py`):

```
contract <- storage, policy <- authority <- (parent: gateway + facade)
throttle (no first-party edge) <- (parent)
```

| Crate | Package name / Nx project / `semio.id` | Dependencies |
| --- | --- | --- |
| `🧬️contract` | `semio-framework-server-contract`, `@semio-tech/framework-server-contract-rs`, `server-contract` | replication (lib `protocol`), serde, serde_json |
| `🗄️storage` | `...-storage`, `...-storage-rs`, `server-storage` | contract, replication, serde, dispatch-macros |
| `🛡️policy` | `...-policy`, `...-policy-rs`, `server-policy` | contract, serde, dispatch-macros |
| `🎭️authority` | `...-authority`, `...-authority-rs`, `server-authority` | contract, storage, policy, dispatch-macros, tokio (`sync`) |
| `🚦️throttle` | `...-throttle`, `...-throttle-rs`, `server-throttle` | none; dev: governor, serde_json |

Manifests follow the recipe (`workspace = "../../../../../../.."` which resolves to the repo root, exactly three `rust-source-boundary` keys with the ceilings, `role = "product"` like the parent, `[lints] workspace = true`, dependency specs copied literally from the parent so the lock resolutions do not change). Longest new path is 117 bytes.

Parent `semio-framework-server` (lib name `server`):
- `📦️packages/🦀️rust/🦀️.rs` replaces the five `#[path] pub mod` mounts with `pub use semio_framework_server_<m> as <m>;`, so every consumer path `server::contract::`, `::storage::`, `::policy::`, `::authority::`, `::throttle::` (hub and the teaching proctor) is unchanged. The cross-crate half of the `dyn_enum_close!` recipe is kept: the nine port macros are re-exported explicitly (`__semio_dispatch_{AuthorityStore, BlobStore, ProjectionStore, SessionStore}`, `{PrincipalResolver}`, `{Decider, Saga}`), next to the gateway's own two, so `use server::{__semio_dispatch_Decider, __semio_dispatch_PrincipalResolver};` in hub and proctor and the whole `closed_ports` test import list still resolve. `#[macro_use]` stays only on `gateway`.
- The old root `pub use crate::contract::*;` glob and the product-root `pub use component::*;` were dropped (carving rule 4: explicit items, no globs). No consumer used a root-level contract item: every `server::` path in `🌎️hub` and `🎓️teaching/🛂️proctor` is module-qualified, `server::conformance` or one of the macro imports above.
- `Cargo.toml`: five path dependencies added, `governor` dev-dependency removed (only the throttle test used it). Features unchanged (`conformance`).
- `🦀️.rs` (product root) now holds only the façade docs; `crate::report` moved (below).
- `📜️script.ts` `test` now runs `cargo test` for the parent and the five children.

Behaviour moves forced by crate boundaries (no logic changed):
- `report` (one `eprintln!("[ERROR] ...")`) moved from the product root into `semio-framework-server-authority` as `pub fn report` (the lowest crate that needs it); the gateway imports it with its other `authority` items and calls it bare (3 sites).
- The rustdoc intra-doc links that pointed upward (`crate::gateway::ServerInstance`/`ServerState` from storage, policy and authority, `crate::authority::Decision` from contract) became plain code spans; links to lower crates became crate paths.
- `#![allow(async_fn_in_trait)]` (the parent's crate-level allow) was added to storage, policy and authority, whose public traits declare `async fn`.
- Unit tests. `contract` and `throttle` keep their `🧪️tests/🔬️unit/🦀️.rs` mounted by their own lib root (they only use their own module). The unit tests of `authority`, `policy` and `storage` need the reference instance (`crate::test_instance`: memory stores, deciders, sagas, resolver, which implements the gateway's `ServerInstance`), which can only live above them. Their files did not move; they are mounted by the parent glue as `#[cfg(test)] mod authority_unit/policy_unit/storage_unit` and their `use super::*;` became explicit imports (`crate::authority::*`, `crate::policy::*` plus the contract and storage names they used through the module's private imports). The authority test used two private helpers, `frame` and `unframe`; instead of widening them to `pub`, the test now carries its own re-statement of the snapshot framing (`[format.to_le_bytes(), state].concat()` and `strip_prefix`), which is a stronger oracle. The reference instance gained `use crate::{__semio_dispatch_Decider, __semio_dispatch_Saga};` (a closing site in another crate than the trait's needs the import, as the `dispatch` crate documents).

### 2.2 Registrations (recipe steps 2 to 10)

- Root `Cargo.toml` `[workspace] members`: five lines after `🖥️server/📦️packages/🦀️rust`, byte-sorted (`🎭️authority, 🗄️storage, 🚦️throttle, 🛡️policy, 🧬️contract`, checked by a byte comparison, and against the following member `🦑️repo/...`). `🧰️framework/Cargo.toml` does not list server, so it is untouched.
- `📦️packages/🦀️rust/📋️project.json` per child: name and the `scope:framework`, `language:rust` tags only (targets are inferred), same as the latest peer carve.
- `📋️build-owners.json`: five rows in name order around `@semio-tech/framework-server-rs` (1,728 owners, names unique, `root` and `manifest` checked per row).
- `.vscode/launch.json`: 30 routes `🦀️ semio-framework-server-<m> 🏗️build|🔎️check|🧪️test 🛠️prototype|🚀️release`, groups `4_build`, `4_gate`, `4_test`, orders `900.013..900.01309`, `922.043..922.04309`, `903.583..903.58309` (all checked unused in their group; the peers were at `...088` while I worked, so a gap was left on purpose), inserted after the `repo-graphql-fs` block. Command form copied from the peer routes (`bun repo run @semio-tech/framework-server-<m>-rs:<target> --configuration=<c> --parallel=1 -- --lib --offline`). `.vscode/🧩️launch.seed.jsonc` was not touched: it holds none of the 498 existing `🦀️ semio-framework-*` producer routes, so adding six-route blocks there would diverge from every precedent (the recipe says the same).
- Boundary corpus (recipe step 11): not applicable, it covers the UI cluster only.
- Taxonomy: no new folder emoji; `📦️packages/🦀️rust` below a product module has precedent (`💻️os/🔨️modules/🛢️db/📦️packages/🦀️rust`).

### 2.3 Server verification

| Check | Result |
| --- | --- |
| Patched full-resolution differential, union sandbox (the five crates simulated as modules of one crate: `semio_framework_server_<m>::` mapped to `crate::<m>::`, `#[macro_use]` restored, macro re-export lines dropped identically; `🗑️generated/carvepr-union-server.py`, `carvepr-run-split.py`) against a sandbox of the original tree, native / `--test` / `conformance` / `--test`+`conformance` | 105 vs 105, 318 vs 318, 105 vs 105, 318 vs 318 errors, **only-split 0, only-baseline 0** in every configuration (all remaining errors are the missing external crates `tokio`, `axum`, `serde`, `serde_json`, `futures`, `protocol`, `semio_framework_dispatch_macros`, `semio_framework_async_macros`, `governor`, `tower`, identical on both sides, plus the unexpanded `dyn_enum_close!` sets). |
| Oracle sensitivity | in a throwaway copy, removing the `Arc` and storage imports from the relocated authority test and renaming `frame` produced 32 new errors (E0425/E0433/E0405), so the relocated tests' name resolution is genuinely covered. |
| Per-child dependency completeness | each child compiled as its own crate root: the set of unresolved external crates equals the declared dependencies (no missing, no unused) for contract, storage, policy, authority and throttle (lib and test); `throttle` has no unresolved name at all. |
| Definition scan | no `pub(crate)`/`pub(super)` item existed in any module; the relocated tests use only `pub` API of `authority` and `policy` (private top-level items used: `frame`, `unframe`, handled above; private fields mentioned in the tests are locals or `storage::TurnCommit` fields). |
| rustfmt | all 11 touched `.rs` files parse (exit 0) |
| Paths | every `#[path]` and `include_*!` in the server tree resolves (0 missing); `🗑️generated/carvepr-verify.ts`: six manifests parse under Bun TOML, `workspace` resolves to the repo root, `[lib] path`, dependency paths and `[[test]]` paths exist, members byte-sorted and present (97 members), owner rows and project names match, boundary keys exact |
| JSON | `.vscode/launch.json` parses (9,040 configs; the 30 new names unique, the 30 commands present; three pre-existing duplicate names belong to peers), `.vscode/🧩️launch.seed.jsonc` parses as JSONC (6,074 configs), `📋️build-owners.json` parses |
| Census rows | section 0 |
| TypeScript runners | server's `🧪️tests/🔬️wire/🟦️.ts` and `🎚️config/🟦️.ts` are vitest registrars (0 tests under `bun test`) and were not run; the Rust files they read (`📡️gateway/🦀️.rs`) did not move. |

## 3. Residual risks (first real compile)

1. Macro paths. The nine re-exported `__semio_dispatch_*` macros and `use crate::{__semio_dispatch_Decider, __semio_dispatch_Saga}` are the only constructs the union oracle cannot exercise (the `dyn_enum` proc macros do not expand there). They follow the documented cross-crate recipe and the existing `closed_ports` test; if rustc rejects the re-export of a `#[macro_export]` macro from an external crate, the fallback is for each closing site to import the macro from the owning producer crate directly (`use semio_framework_server_storage::__semio_dispatch_AuthorityStore;`), never a glob.
2. Trait-in-scope and field/method privacy (E0599, E0616, E0624) are only covered by the scans above, as in the earlier hub and infinite hardening.
3. `Cargo.lock` (root, `🌎️hub`, `🎓️teaching`) must be refreshed (`bun repo cargo --store=shared --configuration=prototype -- update --workspace --offline`, plus the hub and teaching workspaces) once the launcher works; expect five new packages and `semio-framework-server`'s dependency list to grow by five and lose `governor`.
4. `governor` stays a dev-dependency of `server-throttle` only; the parent's `lib name = "server"` and every consumer path are unchanged.

## 4. Files

Print (edited): `📓️print/🦀️.rs`, `📓️print/🧪️tests/🧬️chart-mutations/🦀️.rs`, `📓️print/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs`.

Server (edited): `🖥️server/🦀️.rs`, `🖥️server/📦️packages/🦀️rust/🦀️.rs`, `.../Cargo.toml`, `.../📜️script.ts`, `🔨️modules/{🧬️contract,🗄️storage,🛡️policy,🎭️authority,📡️gateway}/🦀️.rs`, `🔨️modules/🎭️authority/🧪️tests/🔬️unit/🦀️.rs`, `🔨️modules/🛡️policy/🧪️tests/🔬️unit/🦀️.rs`, `🧪️tests/🧩️instance/🦀️.rs`.

Server (created): `🖥️server/🔨️modules/{🧬️contract,🗄️storage,🛡️policy,🎭️authority,🚦️throttle}/📦️packages/🦀️rust/{Cargo.toml,📋️project.json}`.

Repository registries (edited): `Cargo.toml` (5 members), `📋️build-owners.json` (5 rows), `.vscode/launch.json` (30 routes).

Tools and logs in `🪓️monolith-split/🗑️generated/`: `carvepr-oracle.py`, `carvepr-run-baseline.py`, `carvepr-run-split.py`, `carvepr-union-server.py`, `carvepr-child-deps.py`, `carvepr-verify.ts`, `carvepr-print-edit.py` (the exact print edit), `carvepr-print-closure-{before,after}.tsv`, `carvepr-census-after.log`, `carvepr-server-baseline-check.log` and `carvepr-server-child-check.log` (the two refused `bun repo cargo` attempts). The scratch sandboxes were deleted.
