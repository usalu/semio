# Result lw8: WP8 OS (non-renderer) to framework modules, ui, other

Executor `lw8`. No `package.json`/`bun.lock` edit, nothing under the frozen storage/revision paths, no cargo, no git-modifying command, renderer (`📺️renderer`) untouched, only import specifier text changed (no os->repo edges touched, no compat re-exports).

## 1. Totals (violations from non-renderer OS files to targets outside the OS product)

Scoped dependency-cruiser run (lay0 policy config; `lw8-tools`): `dependency-cruise <os> --exclude '^<os>/…/📺️renderer/' --include-only '^(<os>|🧰️framework/🔨️modules)/' --max-depth 1`, about 2.4 min.

| rule family (target outside OS) | before | after |
|---|---:|---:|
| `no-cross-package-relative-*` (per package rule, `-rs` twin counted separately) | 301 | 121 |
| unique edges | 172 | 69 |

108 edges (180 violations) fixed in 73 files. Remaining 69 edges all need new `exports` entries (listed 1:1 in `lw8-tools/exports-needed.json`; verified: 0 remaining edges unlisted). Same run: whole non-renderer-OS-sourced total 464 -> 285 (the rest is os->os WP6/lay6 and `no-core-path` 7 for `flow/core`, not WP8).

## 2. What changed

`lw8-tools/ncpr-fix.ts` (lay6 method: TypeScript checker proves every imported symbol is exported with the identical declaration by an EXISTING package export entry, namespace/dynamic uses need a full superset) rewrote deep relative specifiers to bare package entries, e.g. `@semio-tech/framework-compiler-ts/syntax/module-tree` (28), `@semio-tech/process/compiler-capacity` (8), `@semio-tech/framework/identity/installation` (6), `@semio-tech/framework/browser-bundle/child/schema` (6), `@semio-tech/framework/config/mutations` (4), `@semio-tech/framework-kernel` (3), `@semio-tech/value-ts/testing/fixtures`, `@semio-tech/framework-actor/retained-turn/bootstrap/…`. Targets: compiler, actor, process, kernel, browser-bundle, identity, config, plugin-module, manifest, packs, io, filesystem, job, hash, assets, ui-styling, replication, value, backbone, app-channel.

File list: `lw8-tools/ncpr-changed.txt` (73 files; 22 production sources and 51 tests, all under `🧰️framework/🛍️products/💻️os/🔨️modules/**` and `…/🧪️tests/📡️scene-wire`).

## 3. Open: exports needed (manifest batch)

`lw8-tools/exports-needed.json` (lay6 format: per package target file, proposed subpath, symbols, importers). 69 edges, 35 new entries: `@semio-tech/framework` 12 (23 edges: ui/identity/job/trace test fixtures, test adapter, shell, browser-bundle action-handoff, ui retained resident/pages, component-receipt), `@semio-tech/framework-actor` 7 (21: testing fixtures 13, return, shard-client/runtime, mailbox, activation output), `@semio-tech/process` 6 (9: publication sources, native-build storage/component-pass/receipt), `@semio-tech/framework-kernel` 2 (5), `@semio-tech/framework-compiler-ts` 1 (3, namespace use of module-tree), `@semio-tech/framework-tool-run-ts` 2 (3), `@semio-tech/assets` 2, `@semio-tech/framework-canvas-ts` 2, `@semio-tech/framework-async` 1. Proposed subpaths for `@semio-tech/framework` already omit the `modules/` segment (existing convention). Run `ncpr-fix.ts apply` again after the entries exist.

Manifest note (same as lay6): replaced specifiers are bare workspace imports from files whose nearest `package.json` may not list the target; the cruiser does not flag it, resolution works, add dependencies when the freeze lifts.

## 4. Verification

- All 108 replacement specifiers: `Bun.resolveSync` from each importing directory resolved 108/108.
- tsc `NODE_OPTIONS=--max-old-space-size=8192 node node_modules/typescript/bin/tsc --noEmit -p 🧰️framework/🛍️products/💻️os/tsconfig.json --pretty false`: 35 errors, none in a touched file (missing `@dnd-kit`, jsdom/pngjs/@webgpu types, ui, renderer, peer work).
- `bun test` on the 51 touched test files: all pass except environment/unrelated failures (`tree-sitter-wasms/package.json` missing, "original caller artifact directory is required", native-oracle artifact required, one Rust-source text assertion in `plugin-runtime-runtime-close-budget`); zero-test files are fixtures that load without error.
- OS vitest config (`lw8-tools/runvt.py` on `os/🧪️tests/🎚️config`): 749 passed, 2 files fail (`retry`, `cockatiel` missing): identical to lay6 before/after.
- Re-cruise after: numbers in section 1.

Generated output (`🗑️generated/lw8-run`) deleted. Tools kept in `lw8-tools/`.
