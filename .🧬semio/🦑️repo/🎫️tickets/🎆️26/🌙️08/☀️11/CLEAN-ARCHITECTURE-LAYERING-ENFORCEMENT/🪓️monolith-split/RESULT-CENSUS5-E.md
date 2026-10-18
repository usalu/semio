# Result CENSUS5-E (prefix `c5e`)

Executor: Sonnet 5.5, foreground, no sub-agents, no modifying git command, no ticket tool. `F` = this folder. Tools in `F/c5e-tools/`, raw outputs in `F/🗑️generated/c5e-*`.

## 0. Verdict

**DONE (static verification only; nothing compiled).** All three items landed.

| Item | Before | After | Children |
|---|---:|---:|---|
| 13 `🔌️plugin/🧬️schema/📜️.wit` | 1,662 | `📜️.wit` 24 (package + `world actor`), largest sibling `📜️effects.wit` 430 | 16 sibling files `📜️<interface>.wit` in the same package directory (types, capabilities, documents, effects, instance-lifetime, events, ui, pure, byte-page, return-page, reactor, host-async, jobs, checkpoint, describe, codec) |
| 14 architect `🔺️diff/🟦️.ts` | 1,012 | 872 | `🧾️program-diff/🟦️.ts` (147): the root `ProgramDiff` interface; the generated `parse<X>Patch` / `parse<X>PatchEntry` parsers (135 `parse…` functions) stay in the leaf (see 3.2) |
| B `proctorSourceDirectories` | raw `cargo metadata` | manifest reader, 0 child processes | `🏗️bootstrap/🕸️manifest-closure/🟦️.ts` (244) |

## 1. Package 13: WIT directory

- Why siblings: `wit-parser` (and so wit-bindgen, wasmtime bindgen, jco, wasm-tools) reads the direct `*.wit` children of a package directory and nothing deeper, and every file repeats `package semio:framework@1.0.0;`. Every Rust consumer already passed the directory (`path: "…/🧬️schema"` in `🧬️schema/🦀️.rs`, host `🔗️wit-bindings`, describe `actor-bindings`, scale fixture, reactor `🌉️call/📥️input` test), so no Rust consumer changed.
- Tool: `F/c5e-tools/witsplit.py` cuts at the doc-comment start of each `interface`/`world`, asserts every non-blank line is kept in order, and writes `package …;` + chunk per file. Original saved to `F/🗑️generated/c5e-wit-original.wit`.
- Oracles:
  - Multiset of non-blank non-package lines of the 17 files equals the original (identical).
  - `F/c5e-tools/witdiff.mjs`: `generateHostTypes` of `@bytecodealliance/jco-transpile` (the same wit-parser `jco types` uses; `wasm-tools` is not installed) over the original single file and over the new directory (`world actor`, async jspi): 14 generated files, **0 differences**.

### Text readers re-pointed

| Reader | Now reads |
|---|---|
| `🔌️plugin/🌐️browser-bundle/🛂️descriptor/🟦️.ts` (`ACTOR_COMPONENT_EXPORTS`) | new `readActorComponentWit(directory)` (every `.wit`, name order) over the directory |
| `…/🛂️descriptor/🧪️tests/🟦️.ts` | directory for `jco types`; `readActorComponentWit` for the text parse |
| `🖥️host/📦️packages/🦀️rust/📜️script.ts` (service-operation check) | `📜️effects.wit` |
| `🖥️host/🪞️schema-parity/🧪️tests/🔬️unit/🦀️.rs` | whole directory, sorted, comments stripped (`fixture_path!("../../../🧬️schema")` + `read_dir`) |
| `⚛️reactor/🎟️context/🧪️tests`, `🌉️call/🧪️tests`, `🌉️call/📥️input/🧪️tests` | `📜️reactor.wit` |
| `🧠️interpreter/🧪️tests/🔌️exports/🟦️.ts` | `📜️jobs.wit` |
| `🧬️schema/🪶️sqlite/⚠️refusal/🧪️tests/🟦️.ts` (2 reads) | `📜️codec.wit` |
| `🌎️hub/🧪️tests/🪪️check-in-process/🟦️.ts` | `📜️codec.wit` |
| `🎭️actor/🧪️tests/🎁️effect-params-nesting` test + fixture `schema` | `📜️effects.wit` |
| `📚️library/⚖️laws/🎯️tool-job-coverage/🏁️run/🟦️.ts` | new `policyReadWitPackageSource(repoRoot, relDir)` in `🔍️discovery/📖️source-access/🟦️.ts` (symlink-refusing policy read, name order) |
| `🎠️kernel/📤️return/📦️content/🔌️wire/🔣️.json` + `🧬️schema/🔣️.json` (`declarationSource` const) | the package directory (the doc names `effects.*` and `reactor.*`) |
| nx inputs `🌐️browser-bundle/📋️project.json`, `📦️packages/🟦️typescript/📋️project.json` | `🧬️schema/**/*.wit` (as `⚖️scale` already did) |
| comments: `🏗️materialization/🟦️.ts`, plugin `Cargo.toml` | updated |

False positives checked and left: `✏️s/🔌️plugins/🎬️sequence/…/🧬️schema/📜️.wit` is the sequence plugin's own package; jcoprobe `📜️world.wit` is a separate single-file world.

## 2. Package 14: architect `🔺️diff`

- Generator: `📚️library/🎛️commands/🧬️schema/🪞️parser-projection` (freshness gate `checkParsers`). It projects `parse<Export>` into the document's one TypeScript leaf and audits only parsers whose text sits in that leaf below the `//#region 🚪️Parsers` block (`parserRange(source, name)` on the leaf; the guard helpers must be in the leaf too, found by `existingHelper`). Types may live in imported children (`SourceStore.declarations` follows the module tree), as the four recent `📋️`/`🗃️`/`🧱️`/`🩹️` type children already do.
- Split taken: the hand-written root interface `ProgramDiff` (143 lines) moved to `🧾️program-diff/🟦️.ts`; the leaf imports it and `export type { ProgramDiff }` keeps the public name (importer `🚪️io/📝️text/🔺️diff/🔣️json/🟦️.ts` untouched). Imports were partitioned by use (69 names to the child, 137 stay), 0 unused.
- Not split: the generated parsers (135 `parse…` functions, about 800 lines). Moving them would make the audit skip them (`range === undefined` is `continue`), i.e. blind the gate; splitting the generator output needs the audit and `declaresParser` to walk the module tree, which is a generator change outside this slice. Reported, not rewritten.
- Oracles: `F/c5e-tools/diffsplit.py` (every original non-import line present exactly once across leaf + child; extra: the re-export line and the child header); `bun build --no-bundle` parses both; `check-scope.ts ✏️s/🔌️plugins/🏛️architect` (run with `--tsconfig-override` because `node_modules` is currently a dangling symlink): `fresh: pending=0 dangling=0 helperless=0 drift=0 orphans=0 unowned=72`, identical to the baseline log; `listModuleTree(leaf)` lists the new child.
- Hash pin: `📚️library/🔣️schema-catalog.json` stores the sha256 of this leaf (`🔺️diff/🟦️.ts`); updated `0b4e277e…` to `42e796bd…` (one line). The child is not hashed (no format leaf).

## 3. Part B: proctor source directories

- Replaced the `cargo metadata` child process by `cargoPackageClosure(workspaceManifest, packageName, { signal, onManifest })` in `🎓️teaching/🛂️proctor/🏗️bootstrap/🕸️manifest-closure/🟦️.ts`, which reads manifests with the library parser (`📚️library/🗂️workspaces/🦀️cargo/📖️toml/🟦️.ts`, `parseCargoToml`). `proctorSourceDirectories(repoRoot, options)` keeps its owner-root mapping and nested-directory pruning; the unused `env` parameter is gone (caller `🧱️stack/🟦️.ts` passes its abort signal).
- Semantics kept equal to `cargo metadata`: all workspace members resolve with default features; feature requests unify across the whole graph including members' dev-dependencies; `dep:x`, implicit `x`, `x/f`, `x?/f` (weak, only once `x` is enabled); `default-features = false`; every platform and `[build-dependencies]` table; `workspace = true` inheritance (paths relative to the inheriting workspace root, features additive); renamed `package = …`; reachability over enabled normal/build edges only; dev-dependencies of non-members ignored; registry/git dependencies skipped. A member glob is refused (not silently mis-expanded).
- Real tree: 62 crates, 18 owner roots, 0.4 to 1.6 s. Compared to the naive all-optional closure of the predecessor (63) the only difference is `semio-framework-deflate`, which nothing enables; `semio-framework-test-fixtures` is included because the proctor's dev-dependency enables `teaching-proctor-base/testkit`, as `cargo metadata` reports.
- Tests (`🏛️architecture/❓️quiz/🧪️tests/🧱️local-stack/🟦️.ts`): the real-tree test lost its cargo oracle wording and its 120 s timeout; new `describe("the crates the proctor is built from")` with a synthetic repository (member + second member, workspace inheritance, rename, dev-only crate, unified dev feature, other-member request, strong and weak feature forms, `default-features = false`, build and platform tables, nested owner pruning) with 4 tests: exact crate set, exact owner list, each manifest read once + reported + cancellable, refusals. The same test bodies run green under `bun test` in a scratch copy (4 pass, 8 expects; vitest itself cannot start because `node_modules` is dangling). `README.md` of the quiz site no longer says "cargo's own resolution".

## 4. Open risks

1. No third-party oracle ran: `cargo metadata` is refused by the Cargo ingress and `smol-toml`/`vitest` are not installed right now. `🎓️teaching/Cargo.lock` is stale (19 local packages vs 62) and unusable as a bound. One-off differential for the coordinator: `cargo metadata --format-version 1 --offline` through the sanctioned launcher in `🎓️teaching`, then compare its non-dev reachable path packages from `teaching-proctor` with `cargoPackageClosure(…).map(c => c.name)`; expected 62.
2. About 40 doc comments still say `🔌️plugin/🧬️schema/📜️.wit` for a record that now lives in a sibling file; harmless, not mass-edited in files peers are changing.
3. `📜️.wit` keeps its path and holds only the package line and `world actor`; the readers that need an interface name the sibling.
4. Architect leaf stays at 872 lines because its parsers are generator-owned (2).
5. Resolver difference to Cargo not modelled: resolver-2 host/target feature separation (build-dependency features are unified into the same set here), which can only add crates, never drop one.

## 5. Coordinator compile list

- `cargo check -p semio-framework-plugin-host --lib --tests` (mounts the schema-parity test, which now reads the WIT directory)
- `cargo check -p semio-framework-plugin-host-wit --lib --tests` (wasmtime `bindgen!` on the directory)
- `cargo check -p semio-framework-plugin --lib --tests` (host-native `bindgen!` in `⚛️reactor/🎟️context/🌉️call/📥️input/🧪️tests`); the wasm32-p2 `generate!` in `🧬️schema/🦀️.rs` compiles only for `wasm32-wasip2`
- `cargo check -p semio-framework-plugin-describe --lib --tests`
- `cargo check -p semio-framework-os-scale-fixture --lib` (wasm-gated `generate!`)
- TypeScript: `bun test` for `🛂️descriptor/🧪️tests`, `🎭️actor/🧪️tests/🎁️effect-params-nesting`, `🧠️interpreter/🧪️tests/🔌️exports`, `🧬️schema/🪶️sqlite/⚠️refusal/🧪️tests`, reactor `🎟️context` tests, vitest of `@teaching/architecture-quiz` (local-stack), hub `🪪️check-in-process`, `verify interactivity tool-jobs`, `schema parsers check`.
