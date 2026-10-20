# Result fixups6 (executor fixups6): actor test gate, graph test split (staged), owner rows of the three workspace roots

Sonnet 5.5, foreground, no sub-agents, no modifying git command. `C` = this ticket, `LIB` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library`, `A` = `🧰️framework/🔨️modules/🎭️actor`. HARD FREEZE respected: nothing under `LIB/`, no `package.json`, no taxonomy file edited. Tools in `C/fixups6-tools/`. Generated output (`🗑️generated/fixups6`, scratch overlays under `/private/tmp/claude-501/fixups6-*`) deleted; the 4 disk images the suite mounted in the overlays were detached first.

## 1. `semio-framework-actor` test cfg 1.073 -> 0.781 (applied, not compiled)

Gate (`C/rg-test-tools/gate-full.ts`, canonical `readRustSourceClosureV1`): before test 27f / 281,283 B / 3,579 L = 1.073 (bytes); after test 25f / 195,210 B / 2,452 L = **0.781**, prod 0.584 -> 0.546.

Method (rg-test): the 955-line unit file and the cfg(test) typegen table leave the lib closure and become `[[test]]` targets over the public API. No visibility change.

- `A/🧪️tests/🔬️unit/🦀️.rs` (61.7 KB): now `[[test]] kernel-unit`. Only change: `use crate::*;` -> `use semio_framework_actor::*;`; the typegen tail moved out. Public-API proof: `crate::*` in the old mount is the package root (`pub use component::*`), so it already saw only public items; a scan of every identifier, struct field and method in the file against `pub(super)`/`pub(crate)` definitions found none used (the only restricted items, `ScheduledActor`, `ActorMeta`, `Kernel.scheduler/...`, `JobPublicationOwner`, `lifetime::valid_request`, are not referenced).
- `A/🧪️tests/🧬️typegen/🦀️.rs` (new): `schema_metadata` (15.5 KB, std only, moved verbatim from `A/🦀️.rs`, no longer `pub`/`cfg(test)`) plus `exports_typescript_bindings` (still `async_test`, reads `🤖️generated/🎭️actor/🟦️.ts` by `include_str!`, 8.8 KB leaves the lib closure) = `[[test]] typegen`.
- `A/🦀️.rs`: schema region and `mod tests` mount removed; `retained_turn_tests` and the global allocator stay (the ownership law `🧬️ownership/🧪️tests/🟦️.ts` requires the runtime laws in `🧪️tests/🎟️retained-turn/🦀️.rs` inside the lib closure; run, 4 pass).
- `A/📦️packages/🦀️rust/Cargo.toml`: `[[test]]` `kernel-unit` and `typegen`.
- `A/🧬️typegen/🏃️execution/🟦️.ts`: the exporter now runs `cargo test ... --test typegen exports_typescript_bindings` (builds only the exporter target, not the whole suite).
- `A/🧪️tests/🧬️typegen/🟦️.ts`: the source assertion reads the new Rust file.

Verification: `gate-full.ts` after (numbers above); `rustfmt --edition 2021 --check --config disable_all_formatting=true,skip_children=true` (parse only) ok on the 3 Rust files; `Bun.TOML.parse` of the manifest lists 4 `[[test]]`; `bun test ./🧰️framework/🔨️modules/🎭️actor/🧪️tests/🧬️typegen/🟦️.ts ./...🧭️fixture-placement/🟦️.ts ./...🏛️owner-boundary/🟦️.ts` 8 pass; `bun test ./...🧬️ownership/🧪️tests/🟦️.ts` 4 pass. **Cargo: not run** (no compiler evidence; name resolution of the moved files verified by scan only). Cargo check to run later: `-p semio-framework-actor --tests`.

## 2. `LIB/🏗️build/🧪️tests/🕸️graph/🟦️.ts` split (STAGED, not applied)

Original: 1,055 lines now (a peer added tests while I worked; the freeze is not holding for this file), 104 test registrations (`test`, `test.each` x2, `test.skipIf`, `registerBuildTestLawV1`). Staged in `C/fixups6-tools/staged/` (mirrors the repo path), anchored by `manifest.tsv` (sha256 of the live base file).

Result (all hand-authored files < 200 lines; sizes 3.9-61 KB):

| File (under `LIB/🏗️build/🧪️tests/🕸️graph/`) | Tests | Behaviour |
| --- | ---: | --- |
| `🟦️.ts` (aggregator, 20 lines) | 0 | imports the owned suites (candidate, settlement, caching, inputs, tools, verification) and the 10 groups in order |
| `🧰️support/🟦️.ts` | 0 | shared `graph()`, `withGraphTestInvocationV1`, `withCommandSourceControlV1`, Nx oracle helpers and the one-time `beforeAll` warm-up |
| `🛂️admission/` | 20 | selected graph admission / source inference |
| `🧭️portable-cli/` | 10 | CLI, selections, native caller, watch |
| `📐️portable-model/` | 7 | schema, bundler, projection oracles |
| `📌️pinned-publication/` | 12 | pins, nested consumers, reset |
| `🔭️observed-facts/` | 7 | facts, retry, observed replay |
| `🔌️nx-engine/` | 13 | Nx projection and private engine references |
| `🚀️dependency-bootstrap/` | 3 | bootstrap and dependency CLI |
| `🧊️optional-cache/` | 9 | optional graph cache (the contender subprocess test stays beside the test it re-runs via `import.meta.path`) |
| `🌐️source-discovery/` | 6 | portable source and manifest discovery |
| `🎯️source-demand/` | 17 | matched inputs, runtime modules, demanded entries |

The aggregator keeps the original path, so `🏗️build/📜️script.ts:75` and the dashboard-control `readerFiles` row need no edit. Module-relative literals in bodies (`new URL(.., import.meta.url)`, `require`/`import()`/`resolve(import.meta.dir, ..)`, `import(..)` types, static imports) are rewritten by AST for the extra directory depth; fixture-like literals (`"."`, `".."`, `"../"`, `"../../"`, `../library/api.ts`, ...) were reviewed by hand and stay.

Tools: `restage.sh` (split.ts + oracle.ts; the group table is `groups.json`, keyed by test title: a new peer test makes `split.ts` stop with "no group for ..."; one such title was added this run), `apply.sh [--dry-run|--check|apply]` (python, pattern of walkperf: refuses unless the live file hash equals the staged anchor, refuses to overwrite foreign files, atomic replace; exercised in a throw-away root: apply, idempotent re-apply, drift refusal), `oracle.ts`, `overlay.sh`/`runov.sh`/`compare.ts`/`detach.sh` (verification).

Verification (exact):
- Lossless line oracle `bun C/fixups6-tools/oracle.ts`: 854 non-import original lines vs 854 in the 11 new files as a multiset (leading `./`/`../` stripped; blank, import, doc header, the replicated `const require=createRequire(import.meta.url)` line and the `export ` prefix excluded), 104 registrations by independent regex; every one of 163 relative literals either equals the original or resolves to the same absolute path from the new directory (unrewritten ones must not exist on disk, except the 4 reviewed). Negative controls (deleted line, wrong depth) both fail it.
- Scratch overlay (symlink farm of the real tree with only the `🕸️graph` directory physical, `C/fixups6-tools/overlay.sh`; `package.json`, `📋️project.json`, `tsconfig.json` copied because `findWorkspaceRoot` rejects symlinks), run through `bun C/bzl-act-tools/wrap.ts ./<file> --reporter=junit` with the owned side-effect suites removed from both variants: original 110 tests = 71 pass, 38 fail, 1 skip; split 110 tests = 72 pass, 37 fail, 1 skip. Per-test compare (`compare.ts`): 1 difference, `ordinary immutable graph readers do not inventory optional accelerator caches`, fails in the original in both runs (first test of the file, `Original resource loan deadline exhausted` after 10 s) and passes in the split (and passes alone as first test of its group file). No test changed from pass to fail.
- The 38 shared failures are baseline failures of this overlay (e.g. `Original bounded Nx namespace required`, Ajv `strictRequired`, `@iarna/toml` externalDependency); they fail identically before and after. Including the owned imports the whole aggregated file cascades into `Original process admission closed` after the first (candidate) failure in the original too.
- `tsc --noEmit --strict ... ` over the overlay aggregator: split has no TS2304/TS2307/TS2552 (no missing import) and a strict subset of the original's diagnostics.
- Emoji directory prefixes (🛂 🧭 📐 📌 🔭 🔌 🚀 🧊 🌐 🎯 🧰) each already occur as `<emoji>️<slug>` directories elsewhere in the repo.
- NOT run: `verify taxonomy`/name laws on the new directories (the repo scan currently crashes in `package-implementation-admission` from peer edits: `bun ./📜️script.ts verify build-owners` failed with `undefined is not an object (evaluating 'admission.form')`), the real `workspace:test-build-contract` runner, the staged files in the real tree.

Apply after the freeze lifts: `C/fixups6-tools/apply.sh --dry-run` then `apply.sh`; if it reports drift, `restage.sh` first.

## 3. The three workspace roots: no owner rows needed, nothing removed

`✏️s/🔌️plugins/🏗️fem/⚙️engine/📍node`, `.../🧱️material` and `✏️s/🔌️plugins/🔋️energy/🧩️extensions/🌦️epw` are not stubs or leftovers. Each `Cargo.toml` there is the standalone `[workspace]` root of the real crate in `📦️packages/🦀️rust` (`semio-s-fem-node`, `semio-s-fem-material`, `semio-s-plugin-energy-epw`): the crates reference it with `workspace = "../.."` and inherit `version.workspace`/`rust-version`; all three came in with their crates (commits `...🚩️695`/`696`). Removing them would break those crates.
Rows are keyed by owner packages, not by virtual manifests (`cargoPackageName` returns undefined for a manifest without `[package]`, so the sync writer never adds a row for it). The three crates have rows now (`📋️build-owners.json` lines 700, 706, 22488). Read-only evidence: `bun C/fixups6-tools/owners-scope.ts <the three directories>` (`checkBuildOwnerRegistry` scoped): 3,729 owners, 0 findings; `SYNC=1` dry-run `syncOwnerRegistry`: added 0, removed 0, renamed/relocated/repackaged none. The registry file was not touched.

## Open items
- Cargo check of the actor crate (`-p semio-framework-actor --tests`, then `cargo test --test typegen exports_typescript_bindings` once): not run; new test targets add build time to the actor test run.
- Apply the staged graph split after the freeze lifts; re-run the oracle and `compare.ts` after apply.
- Peers keep editing the original graph test (`semanticValueImports` test added during this session); restage if `apply.sh --check` reports drift.
- 135 test-cfg crates remain in 0.90-1.00 (audit census 10 section 3); not touched.
