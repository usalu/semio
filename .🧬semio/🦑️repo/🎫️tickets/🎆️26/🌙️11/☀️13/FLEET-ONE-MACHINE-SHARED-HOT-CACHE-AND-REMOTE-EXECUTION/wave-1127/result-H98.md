# H98: `repo preflight <targets>`: seconds-long static checks before admission

Stage only (full build-system freeze). Nothing landed, nothing wired.

## What exists (staged)
`T/🧰️scripts/H98/stage/tree/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🩺️preflight/🎯️targets/` (mirrors the live path; new member under the existing preflight member, node-only imports, no graph, no cargo, no process start):
- `🟦️.ts`: `preflightTargetsV1({root,targets,files?,peaks?,ceilingMegabytes?,cacheFile?})` returns `{ok,findings,schedule,units,millis,filesRead,truncated}`; `formatPreflightV1(report,"en"|"de")` (language required, none assumed); `unitPeaksFromWardensV1(store)`.
- `📖️texts/🔣️.json`: en+de detail and fix per finding code (placeholders must match in both languages, tested).
- `🧪️tests/🟦️.ts`: 10 tests, 3330 expectations, pass (run from a scratch copy, see below).
- Kept measure script: `T/🧰️scripts/H98/measure.ts`.

## The four checks
a. **Generated files and producers.** `generatorContracts` of the taxonomy (38 contracts) give output root to producer target (own `producer` of an output, else the contract target). A relative import / `mod` / `include!` that resolves to nothing inside an output root does not refuse: the producer target goes into `schedule` (run first), after verifying the producer owner is registered in `📋️build-owners.json` and the manifest has that target. Refusals: `generated-no-producer` (external contract), `generated-producer-unregistered`, `generated-producer-target-missing`. The same registry/manifest facts `validateGeneratorOutputProducersV1` uses, so no second truth; consuming `schedule` in `repo run` is the wiring below.
b. **Imports / `use` / `mod`.** TS/JS: extraction regexes (comment-stripped, `?raw`/`#` suffix stripped, `.semio-*` virtual modules skipped), relative resolution with extension variants, dir index (`🟦️.ts`), `.js`->`.ts`, closure of reached files (cap 20000). Rust: `mod x;` (+ `#[path]` among other attributes, `macro_rules!` bodies ignored), `include!/include_str!/include_bytes!`, and `use` of a workspace crate that the crate does not declare (extern name = dependency `[lib].name`, `extern crate .. as`, `use` aliases and names introduced anywhere in the crate count as declared). Codes: `import-unresolved`, `rust-mod-unresolved`, `rust-include-unresolved`, `rust-crate-undeclared`. A persistent file cache (`cacheFile`, mtime+size) makes warm runs ~3-5x faster.
c. **Cargo manifests and lock, offline.** No cargo process (the toolchain shim refuses outside the launcher): members from `workspace.members` (globs of one level), `exclude`, `package.workspace`; per member: manifest exists/parses, lock has the member with same version (workspace.package.version inherited), every declared dependency (dev/build/target, `workspace=true` inherited) is in the member's lock dependency list, nothing locked is undeclared, registry requirement satisfied by some locked version (caret/tilde/exact/range), path dependency inside the workspace dir must be a member. Codes: `lock-missing`, `lock-stale` (subject = the lock path, `{crate, why, manifest}`, command `repo cargo generate-lockfile --manifest-path <ws>/Cargo.toml`), `manifest-missing`, `manifest-invalid`, `manifest-no-workspace`, `manifest-not-member`. The hub lock of today is not stale any more (rewritten 17:34), so the real-tree run reports none; every drift class is a test case. Not covered: exact transitive resolution (cargo's resolver stays the authority at build time; `--locked` there is the backstop).
d. **Size/memory plan.** `unitPlan`: files, bytes, largest source per target root; `peaks` (crate name with `_`) from the wardens' `memory.json lastAlarms` (`unitPeaksFromWardensV1`), > ceiling (1536 MB default) = `memory-over-ceiling` refusal (fix: single thread/carving list/bigger host); unmeasured with a source > 1 MiB = `source-large` warning. H90 has no result file yet: its per-unit table plugs into `peaks` (any `Record<unit,MB>`) without changing the report; nothing duplicated.

Every finding: `code`, `severity`, `owner` (registry owner of the file, `(no owner)` for build library files), `subject`, `detail{en,de}`, `fix{en,de}`, optional `command`.

## Measured (this Mac, load 20-40, `nice -n 10`, bun, wall incl. process start)
| target | cold | warm |
|---|---|---|
| `@semio-tech/framework-actor-rs` (Rust, 1716 files, 5 workspaces crates closure) | 2.8 s | 0.9 s |
| `@semio-tech/framework-renderer-wgpu` (TS, 2235 files) | 1.7 s | 0.5 s |
| `semio-hub-demonstrator-playground-editor-app` (hub workspace, lock of 2.7 MB) | 0.8 s | 0.4 s |
All < 10 s. Real findings on the tree today: `rust-mod-unresolved` `mod funding;` in `🧰️framework/🔨️modules/🗣️dsl/🧬️schema/🛬️receive/🧪️tests/🦀️.rs` (the `📥️bank` folder is empty: genuine, breaks the dsl-record test build); `memory-over-ceiling` for `semio-s-artifact-stdio-binary-mutations` (2032 MB from the warden). Earlier false positives found and fixed by measuring: `?raw` suffix, `//` inside strings, `import{x}from"y"` without spaces, `#[path]` after `#[cfg]`, `[lib].name` differing from the package name, `extern crate .. as`, `macro_rules!` bodies.

## Validation (CLAUDE.md: language-agnostic + third-party oracle)
Tests build scratch worlds (owners, taxonomy, manifests, locks) and assert codes/schedule/commands; import extraction is compared with TypeScript's own `ts.preProcessFile` on 400 real files of the build library (every literal specifier the compiler sees, we see: 3330 expectations). Lock check has no independent oracle in this environment: `cargo metadata --locked` is unreachable (the cargo and rustc of the toolchain are the launcher shim). Python oracle for TOML was not possible (no `tomllib`, Python < 3.11). Run: copy the `🎯️targets` folder to scratch, replace `from "typescript"` by the absolute path of `node_modules/typescript/lib/typescript.js`, `H98_CORPUS=<dir> bun test ./🧪️tests/🟦️.ts`.

## Wiring to land after the seal (via `repo land`, all slot-bound)
1. Taxonomy: confirm the new `🎯️targets` member name/emoji and its `📖️texts` + `🧬️schema` are admitted (not run: freeze); add the JSON schema for the report (schema-first) next to the texts.
2. `🩺️preflight/🟦️.ts` `readBuildPreflightArgumentsV1`: accept `preflight <project[:target]...> [--files=...] [--lang=en|de]` besides the bare form; with targets call `preflightTargetsV1` (and keep the host checks); exit 1 on `!ok`.
3. `📥️caller/📤️child/🟦️.ts` line 16 (the `preflight` branch) and `⌨️command/🧭️selection/🔣️.json` (+ its schema list): route `preflight` with targets to the same kernel.
4. `repo run` / `repo cargo` / `repo test`: call `preflightTargetsV1` before graph admission (after argument parsing, before lease/queue); on `!ok` print `formatPreflightV1` and refuse; on `schedule` non-empty add the producer targets as prerequisites of the run instead of the late `Generator output has no Build producer` refusal.
5. MCP: extend the `preflight` tool with `targets` (array of strings), `files`; typed result = the report.
6. Feed H90's per-unit peak table as `peaks`; pass `cacheFile` under the machine cache.
Open risk: `use` of undeclared workspace crates is heuristic (source-level, cfg-blind); it flags only crates that exist in the workspace and are used with no declaration/alias anywhere in the crate. Treat as refusal after one fleet day without false positives, else downgrade to warning.

## Unverified
Taxonomy/emoji admission of the new member; the wiring (not written, freeze); cold numbers on a worker (16 GB Mac, Windows paths); the stale-hub-lock case on the real hub (lock already fresh); Windows path separators (code uses `sep`/`resolve`, not run).
