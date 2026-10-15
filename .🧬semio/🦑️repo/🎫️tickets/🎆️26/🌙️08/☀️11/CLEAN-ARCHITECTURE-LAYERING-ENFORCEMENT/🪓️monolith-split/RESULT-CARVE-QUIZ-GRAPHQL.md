# Result: carve `semio-framework-quiz` and `semio-framework-repo-graphql` under the source boundary

Executor: Sonnet 5.5, foreground, no sub-agents. Rules followed: `📓️EXECUTOR-RULES.md`, `📓️CARVING-RULES.md`, `📓️RESOLUTION-ORACLE.md`, `PRODUCER-CRATE-RECIPE.md`. Nothing was committed, staged, stashed or checked out. Nothing was compiled around the gate.

## 0. Verdict

Both tier-0 leaves now pass the native source boundary with headroom, by carving one cohesive, dependency-free module each into its own producer crate. No comment or blank line was trimmed.

| Crate | Before (files / bytes / gate lines) | After (files / bytes / gate lines) |
| --- | --- | --- |
| `semio-framework-quiz` | 12 / 190,352 / **4,019** | 11 / 137,241 / **2,648** (0.662) |
| `semio-framework-quiz-schema` (new) | | 1 / 53,956 / 1,382 (0.345) |
| `semio-framework-repo-graphql` | 7 / 179,150 / **4,012** | 6 / 127,780 / **2,770** (0.693) |
| `semio-framework-repo-graphql-schema` (new) | | 1 / 52,411 / 1,263 (0.316) |

"Gate lines" are the numbers of `readRustSourceClosureV1` through the census tool (`🗑️generated/census.ts`, logs `cqg-census-quiz-after.log` and `cqg-census-gql-after.log`; the before rows reproduce the census table: quiz 4,019, graphql 4,012). Test-cfg closures (`cqg-tools/closure.ts`, logs `cqg-closure-before.log` and `cqg-closure-after.log`):

| Crate | Native before / after | `test` cfg before / after |
| --- | --- | --- |
| quiz | 4,019 / 2,648 | 8,368 (22 files) / **6,695 (20 files, 408,357 bytes)** |
| quiz-schema | / 1,382 | / 1,696 (3 files) |
| repo-graphql | 4,012 / 2,770 | 4,173 / **2,926 (7 files)** |
| repo-graphql-schema | / 1,263 | / 1,301 (3 files) |

Open limit: the quiz `test` closure is still far over 4,000 because the nine module unit-test files (4,029 lines) stay in the `quiz` crate. Carving them away needs more than one cut (the unit tests share helpers across modules: `sheet::tests`, `lifecycle::tests`), so I did not attempt it. The native closure, which the census and the compiler-ingress gate measure, passes. The CLI crate (`semio-framework-repo-cli`, 4,362) is a different task and is unchanged.

The compile route is not usable (section 6), so no type-check, no `cargo test` and no `--help` byte-identity run was possible. Everything below that says "verified" is static analysis or the rustc analysis oracle.

## 1. Choices and why

### 1.1 Quiz: carve `🧬️schema/🦀️.rs` (1,373 lines) into `semio-framework-quiz-schema`

- The schema module depends on nothing but `serde` and `BTreeMap` (the only `crate::` mention was a doc link to `crate::validation`). Every other module imports from it; it imports from none. So the new crate is a true leaf and `quiz` depends on it, never the reverse.
- Layout (recipe section 2): the lib root stays where it was, `🧬️schema/🦀️.rs`, so no source file moved and no text reader of the path changes (path-tail `git grep` over the index: only historical tickets mention `❓️quiz/🧬️schema/🦀️.rs`). New package dir `🧬️schema/📦️packages/🦀️rust/` holds `Cargo.toml` and `📋️project.json`; `[lib] path = "../../🦀️.rs"`.
- Lib name `quiz_schema`, following the quiz convention (`semio-framework-quiz` is lib `quiz`). Package `semio-framework-quiz-schema`, id `quiz-schema`, Nx project `@semio-tech/quiz-schema-rs`, role `product` (same as `quiz`), `workspace` key six ups.
- **Test helpers** (`fixture`, `typed`, `json`, `entries`, `assert_close`, 40 lines) lived inside the schema's private-to-crate `tests` module and were imported by nine module test files as `crate::schema::tests::…`. After a crate boundary that cannot work, so they moved to a new `🧬️schema/🧪️tests/🧰️testkit/🦀️.rs`, mounted as `#[cfg(any(test, feature = "testkit"))] pub mod testkit;`. The feature is `testkit = ["dep:serde_json"]`; `quiz` enables it as a dev-dependency. This is the recipe's `testkit` pattern (same shape as `pack-json-retained-schema`).
- **Stale fixture path fixed on the way.** The old `fixture()` read `CARGO_MANIFEST_DIR/../../🧫️fixtures`, which is an empty leftover directory `❓️quiz/🧫️fixtures` (the real fixtures are under `🧪️tests/🧫️fixtures`; the empty directory is untracked and not mine, I left it). In the new crate the path is `../../../🧪️tests/🧫️fixtures`, which exists; all 14 case names used by the unit tests resolve to a directory with a `🔣️.json` (script check). Before my change those tests could not have found a fixture.
- **Consumers.** All 17 quiz module and unit-test files had `crate::schema::` rewritten to `quiz_schema::` (and `crate::schema::tests::` to `quiz_schema::testkit::`), i.e. they depend on the new crate directly. Import blocks were re-ordered where `rustfmt` wants `quiz_schema` after the `crate::` group (11 blocks). The package glue lost its `pub mod schema;`.
- **Façade re-export** (`❓️quiz/🦀️.rs`): `pub use crate::schema::*;` became an explicit `pub use quiz_schema::{…}` of the 54 names (47 types, 7 constants) that external consumers use. The set was computed from every `quiz::…` use in `🎓️teaching` and in the quiz test adapters (`cqg-tools/consumers.py`, re-checked after the edit by `consumers2.py`: 125 consumer names, 0 missing; 48 schema items are no longer re-exported and none is used). The consumers (the proctor and the quiz test adapters) need no edit. `quiz::serde_json` is unchanged.
- **Test route kept whole.** `@semio-tech/quiz-rs:test` (`📜️script.ts`) now runs `["semio-framework-quiz-schema", "semio-framework-quiz"]`, so the schema's unit tests do not silently drop out of the launch route.
- Docs: the quiz `README.md` table gained the new package row; the schema doc comment no longer links `crate::validation`.

### 1.2 GraphQL: carve `🏗️repo-schema` (1,242 lines) into `semio-framework-repo-graphql-schema`

Choice between `🏗️repo-schema` and `🔤️grammar` by dependency direction, from an identifier cross-reference of the six children (`cqg-tools/gqlrefs.py`, `gqlimports.py`, `gqlschemause.py`):

- `🔤️grammar` (621 lines) is also a leaf, but it would only take the core to about 3,390 lines.
- `🏗️repo-schema` uses nothing from the other five children and nothing from the model or identity crates; it needs only `serde_json` (`json!`, `Value`) and `BTreeMap`. Only `🖼️sources` (`Schema`) and `⚡️execution` (`Schema`, `NamedType`, `ObjectType`, `TypeRef`, `build_schema`) use it. Carving it gives the larger relief (core 2,770) and an acyclic edge: core depends on the new crate, never back.

Result:

- New node `🔗️graphql/🏗️repo-schema/` (emoji is free among its siblings): lib root `🏗️repo-schema/🦀️.rs` (the child file, moved with plain `mv`, header region and imports rewritten), tests mounted from `🧪️tests/🦀️.rs`, package dir `📦️packages/🦀️rust/` with `Cargo.toml` (`workspace` eight ups, role `library`, id `repo-graphql-schema`, exact three boundary keys, only `serde_json = { workspace = true }`) and `📋️project.json` (`@semio-tech/repo-graphql-schema-rs`, `scope:framework`, `language:rust`, as the sibling `repo-graphql-fs-rs`).
- The crate has `pub use serde_json;` with the same explicit-re-export docstring the core uses, because `Argument::default` and `schema_inventory` expose `serde_json::Value`.
- **No visibility had to be widened.** Everything the core uses (`Schema`, `NamedType`, `ObjectType`, `TypeRef`, `build_schema`, all their fields) was already `pub`; the two `pub(crate)` items (`ty`, `Field::new`) are used only inside the file. Verified by a use scan (no other child references `ty(`, `Field::`, `.arg(`, `.resolved(`, `reachable(`, `render_field(`, `SCALARS`, `schema_*()`), by a definition scan (no private field, no non-`pub` associated fn used outside) and by the oracle with `pub(crate)` stripped (section 5.2). No orphan-rule hazard: no impl on a moved type exists outside it.
- Core root: `mod repo_schema; pub use repo_schema::*;` removed; added `use semio_framework_repo_graphql_schema::{NamedType, ObjectType, TypeRef};` (children get them through `use super::*`, as with the model types) and an explicit `pub use semio_framework_repo_graphql_schema::{build_schema, schema_inventory, Schema};`. `Schema` must stay public because `Executor::schema()` returns `&Schema`; `build_schema` and `schema_inventory` are what the `📜️sdl-dump` adapter imports (`semio_framework_repo_graphql::{build_schema, schema_inventory}`), so that adapter needs no edit. `render_sdl` is no longer exported by the core; its only user was the unit test (moved). The CLI and `repo-graphql-fs` import only `Executor`, `RepoContext`, `RecordingContext`, `ContextError`, `parse_milestone_number` and need no edit.
- `semio-framework-repo-graphql/Cargo.toml` gains the path dependency `../../🏗️repo-schema/📦️packages/🦀️rust`; it keeps `serde`, `serde_json`, identity, model.
- Tests: `serves_the_committed_sdl` moved from the core's unit tests into the new crate (`include_str!("../../🧪️tests/🧫️fixtures/📜️served-schema/🔗️.graphql")`, the SDL shared with the Go twin and `graphql-js`). One new test, `inventories_exactly_the_declarations_of_the_committed_sdl`, compares the kind and name of every declaration found by scanning the committed SDL text with `schema_inventory(&build_schema())` (an independent text scan, not the renderer). The language-agnostic differential `📜️sdl-dump` (Gherkin, `graphql-js` oracle, Go, TS, Rust adapters) is unchanged and still covers the feature. `@semio-tech/repo-graphql-rs:test` (`📜️script.ts`) now runs `[schema crate, core]`.
- Docs: `🔗️graphql/README.md` package list and Rust layout updated.

## 2. Recipe checklist per new crate

| Step | Quiz schema | GraphQL schema |
| --- | --- | --- |
| Manifest | `🧬️schema/📦️packages/🦀️rust/Cargo.toml` | `🏗️repo-schema/📦️packages/🦀️rust/Cargo.toml` |
| Lib root beside package dir, `cfg(test)` mount | `🧬️schema/🦀️.rs` (+ `testkit` mount, `🧪️tests/🔬️unit/🦀️.rs`) | `🏗️repo-schema/🦀️.rs`, `🧪️tests/🦀️.rs` |
| Workspace member (root `Cargo.toml`, byte-sorted; checked the whole list stays byte-sorted) | after `❓️quiz/📦️packages/🦀️rust` | before `🔗️graphql/📦️packages/🦀️rust` (U+1F3D7 sorts before U+1F4E6) |
| Nx project | `@semio-tech/quiz-schema-rs` | `@semio-tech/repo-graphql-schema-rs` |
| Owner row `📋️build-owners.json` | after `quiz-rs` | after `repo-graphql-rs` (the first insertion before `-go` was lexically wrong and was corrected; name unique, manifest exists, project name equals row name) |
| Six launch routes in `.vscode/launch.json` | orders 900.01085/86, 922.04185/86, 903.58285/86 | 900.01087/88, 922.04187/88, 903.58287/88 |
| Lock | root `Cargo.lock` holds both packages and the changed dependency lists (written during my official-check attempts by the launcher's lock step, or by a peer's cargo run; the entries match my manifests) | same |

Notes on the recipe:

- The seed `.vscode/🧩️launch.seed.jsonc` holds no producer routes at all (0 of 76 `🦀️ semio-framework-… 🏗️build` routes), exactly as the recipe says, so no seed edit was made. It parses with `Bun.JSONC`. The 12 new route names, key order, groups and command shape were compared field by field against the `repo-graphql-fs` routes (all equal apart from crate and id); the new orders are unique in their groups.
- Boundary-corpus row (step 11) is UI-only; not applicable. Taxonomy registration (step 1) was not done and no taxonomy validator was run: the new folders `🏗️repo-schema` and `🧰️testkit` use emoji that exist as folder prefixes in the repo and are RGI (script check), without a sibling collision.
- I did not edit `🔒️dependencies.json` (generated external-dependency usage lock; the `repo-graphql-fs` precedent did not either) and I could not run `verify dependencies literal-external` (it started a long `build:playground-native` and was killed at the 110 s cap; log `cqg-verify-dependencies.log`). The `users` lists for `serde`, `serde_json` do not list the two new manifests yet.
- `🎓️teaching/Cargo.lock` (a separate workspace that reaches `quiz` by path) was already stale before my change (its `semio-framework-quiz` entry lists only `serde`) and is not refreshed.

## 3. Files touched

New: `❓️quiz/🧬️schema/📦️packages/🦀️rust/{Cargo.toml,📋️project.json}`, `❓️quiz/🧬️schema/🧪️tests/🧰️testkit/🦀️.rs`; `🔗️graphql/🏗️repo-schema/{🦀️.rs,🧪️tests/🦀️.rs,📦️packages/🦀️rust/Cargo.toml,📦️packages/🦀️rust/📋️project.json}`.

Edited (paths relative to `🧰️framework/🛍️products/`): `❓️quiz/🦀️.rs`, `❓️quiz/README.md`, `❓️quiz/📦️packages/🦀️rust/{Cargo.toml,📜️script.ts,🦀️.rs}`, `❓️quiz/🧬️schema/🦀️.rs`, `❓️quiz/🧬️schema/🧪️tests/🔬️unit/🦀️.rs`, nine quiz modules `🔨️modules/*/🦀️.rs` and their `🧪️tests/🔬️unit/🦀️.rs` (imports and qualified paths only); `🦑️repo/🔨️modules/🔗️graphql/{README.md,📦️packages/🦀️rust/{🦀️.rs,Cargo.toml,📜️script.ts},🧪️tests/🔬️unit/🦀️.rs}`. Repo root: `Cargo.toml` (two member lines), `Cargo.lock` (launcher-written entries), `📋️build-owners.json` (two rows), `.vscode/launch.json` (12 routes).

Removed: `🔗️graphql/📦️packages/🦀️rust/🏗️repo-schema/🦀️.rs` (moved to `🔗️graphql/🏗️repo-schema/🦀️.rs`).

## 4. Static verification run

| Check | Result |
| --- | --- |
| Census before/after | section 0 (`census.ts` with `ONLY=`; before rows reproduced) |
| Lossless line multiset (`cqg-tools/lossless.py`) | quiz schema family: missing only the doc link line, the old helper bodies' `pub(crate)` forms and the old path literal, extra only the testkit header, `pub` forms, mounts and the new path; modules: only the `crate::schema::` to `quiz_schema::` rewrites; graphql: missing only the old mount lines, the old doc header and the moved unit test, extra only header, imports, re-exports, mounts and the new tests |
| Full-resolution differential (`cqg-tools/orc.py`, `rustc +nightly-2026-07-20 --edition 2021 --crate-type lib --emit=metadata -o /dev/null --error-format=json`, extern/glob patches applied to both sandboxes) | quiz: native 193 vs 193, `--test` 376 vs 378; graphql: native 10 vs 12, `--test` 10 vs 12. Non-external errors: quiz 0/0 native, 1/1 test (the same `env!("CARGO_MANIFEST_DIR")` error in both); graphql 0/0. The only differences are `E0432`/`E0433` for `serde_json`/`serde` in the moved files and the abort count |
| How the split is simulated | the new crate is mounted as module `crate::quiz_schema` / `crate::repo_graphql_schema`, every `quiz_schema::` / `semio_framework_repo_graphql_schema::` path is rewritten to `crate::<module>::`, and all `pub(crate)`, `pub(super)` and `pub(in …)` in the new crate's files are stripped so a cross-crate use of a crate-visible item becomes an error |
| Negative controls (not vacuous) | quiz: making `SheetItem` private yields 2 new `E0603`. graphql: making `Schema` private yields 1 `E0603`; removing `TypeRef` from the core's import yields `E0425` x2 and `E0433` x6; breaking the `build_schema` call yields `E0425` |
| Definition scan | quiz-schema: 0 private fields, 0 `pub(crate)`; graphql-schema: 0 private fields, 2 `pub(crate)` (`ty`, `Field::new`), neither used outside the file; no impl or inherent impl on a moved type outside its crate (orphan rule) |
| Consumer check (quiz) | all names used through `quiz::` in `🎓️teaching` and the quiz tests are exported by the façade (`consumers2.py`: 0 missing) |
| rustfmt | `rustfmt --check` (repo `rustfmt.toml`) hunks: quiz crate 64 (baseline 67, identical hunks after normalising the path rewrite; two context-only differences from wrapped imports), quiz schema 3 (all pre-existing), graphql core 2 and schema 22 (identical counts to the baseline run with the same config; all pre-existing). No formatting finding in a line I wrote. All files parse |
| `#[path]` / `include_str!` resolution | 0 missing in all 5 new `.rs` files (`cqg-tools/verify.ts`) |
| Regions, docstrings, emoji, path length | regions balanced, every new file starts with an emoji docstring, new folder emoji RGI with U+FE0F and no sibling collision, all new paths at most 240 bytes |
| TOML | both new manifests, both edited manifests and root `Cargo.toml` parse with Bun's TOML; `workspace` key resolves to a directory with `Cargo.toml`; every dependency path and `[lib] path` exists; every workspace member for quiz or graphql exists |
| JSON | both `📋️project.json`, `📋️build-owners.json`, `.vscode/launch.json` parse; seed parses with `Bun.JSONC`; 12 of 12 routes present exactly once |
| `[DEBUG]` logs | none added |

Tools and logs live in `🗑️generated/` with the prefix `cqg-`: `cqg-tools/{orc.py,control.py,lossless.py,verify.ts,closure.ts,closure-before.ts,consumers.py,consumers2.py,reorder.py,addroutes.py,addowner.py,gql*.py}`, `cqg-census-*.log`, `cqg-closure-*.log`, `cqg-baseline-quiz-check.log`, `cqg-official-check-*.log`, `cqg-verify-dependencies.log`. Original snapshots and sandboxes are in the session scratchpad (outside the repo).

## 5. Official compile route

- Baseline `bun repo cargo --store=shared --configuration=prototype -- check -p semio-framework-quiz --message-format=short` (before my edit): refused with `Live Original process issuance required` (`cqg-baseline-quiz-check.log`).
- After my edit, `… check -p semio-framework-quiz-schema` and `… check -p semio-framework-repo-graphql-schema`: no source-boundary refusal (neither the new crates nor their closures are over the limit), but the launcher dies in cache maintenance with `Finite original maintenance authority required` (`cqg-official-check-*.log`). It got past the lock step first; that is the likely source of the root `Cargo.lock` entries (a peer's run is the alternative).
- So no `cargo check` or `cargo test` ran for any of the four crates, and I do not claim they compile or that the tests pass. The static evidence is the differential above, whose limit is that rustc stops before type checking because serde is not linked (field and method privacy and trait resolution are not exercised; the definition scan and orphan scan cover the gap by hand).
- The CLI `--help` byte-identity run of the earlier split needs a compiled `semio-repo` binary; it cannot be done without compiling around the gate, so it was skipped. The CLI source is untouched and imports no moved name.

## 6. Residual risks and follow-ups

1. First real compile should check three things the oracle cannot: (a) `[lib] name = "quiz_schema"` together with the dependency key `semio-framework-quiz-schema` (extern name is the lib name, as for `quiz`); (b) the `testkit` feature unification (normal dependency without the feature, dev-dependency with it, optional `serde_json` listed in both tables); (c) that the callers of `semio-framework-quiz` (the proctor and the quiz test adapters) still resolve all 54 re-exported names.
2. Quiz `test` closure is 6,695 lines. A second cut would have to take the unit tests with their modules; start with `🎲️randomness` (leaf, 96 + 137 lines) and `✅️validation`, then break the `sheet::tests` and `lifecycle::tests` helper sharing with a testkit crate, as done here for the schema.
3. Refresh `🎓️teaching/Cargo.lock` and `🔒️dependencies.json` once the launcher works; run `verify dependencies literal-external` and the taxonomy validators.
4. The empty untracked `❓️quiz/🧫️fixtures/` directory tree is a leftover of an earlier move and can be deleted by its owner.
