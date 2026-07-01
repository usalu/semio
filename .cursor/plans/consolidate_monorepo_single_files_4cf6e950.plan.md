---
name: Consolidate Monorepo Single Files
overview: Continue the existing open ticket "Consolidate Monorepo Into Single Files" (goal `🎯aioptimizedrepo🎯singlefilerepo`) to flatten stray `src/` folders, remove the one extra non-`script.ts` script file, merge remaining scattered same-bundle source files into their single entry file, fill in missing `package.json` script blocks, and normalize the remaining `project.json` targets that bypass `script.ts` by calling `cargo`/`go`/`dotnet`/`vite`/`uv` directly.
todos:
  - id: reopen-ticket
    content: Reopen CONSOLIDATE-MONOREPO-INTO-SINGLE-FILES ticket via ticket_reopen
    status: completed
  - id: flatten-repo-lib-js-src
    content: Flatten repo/lib/js/src/ to repo/lib/js/ and rewrite all ~150 import references repo-wide
    status: completed
  - id: flatten-ui-styling-rs-src
    content: Flatten ui/styling/rs/src/generated.rs to ui/styling/rs/generated.rs and update lib.rs path + generator
    status: in_progress
  - id: merge-yak-script
    content: Merge Compose.Grasshopper/yak/script.ts commands into parent script.ts, delete yak/script.ts, fix project.json
    status: pending
  - id: merge-scattered-files
    content: Merge remaining scattered same-bundle files into single entry files (ts/tsx/rs/py/go list in Phase 3)
    status: pending
  - id: investigate-graph-manifest-generated
    content: Investigate mathematical/graph/manifest/generated/ for a generator before consolidating the 11 manifest documents
    status: pending
  - id: add-missing-package-scripts
    content: Add missing package.json scripts blocks to sequence/core, sequence/react, imperative/core, imperative/react, framework/core
    status: pending
  - id: route-project-json-bypasses
    content: Wrap remaining cargo/go/dotnet/vite/uv/bun-test project.json bypasses through script.ts using existing repo/lib/js helpers
    status: pending
  - id: validate-and-close
    content: Run typecheck/build/test across all touched bundles and update/close the ticket
    status: pending
isProject: false
---

# Consolidate Monorepo Into Single Files

## Context

This request maps directly onto an **already-open** ticket: [`.repo/🎫/26/05/30/CONSOLIDATE-MONOREPO-INTO-SINGLE-FILES/ticket.json`](.repo/🎫/26/05/30/CONSOLIDATE-MONOREPO-INTO-SINGLE-FILES/ticket.json) under goal `🎯aioptimizedrepo🎯singlefilerepo`. Its description: *"Consolidate monorepo source into single files where frameworks allow. Keep required separate entry points... Update imports and package exports. Run tests."* Prior sessions already merged `geometry-brep-js`, `repo/lib/js` (internal files only, not the `src/` wrapper itself), `repo/client/vscode` codegen, `mathematical/graph` Rust modules, `infinite/cavas` vello, and `compose/client/lib/py` tests. Per the repo workflow rule, this work continues by **reopening that ticket** rather than creating a new one.

Only **2** real `src/` folders exist in the whole repo (`repo/lib/js/src/`, `ui/styling/rs/src/`) — both are documented as not-yet-flattened follow-ups in the ticket's own history.

## Phase 0 — Ticket

- Reopen `.repo/🎫/26/05/30/CONSOLIDATE-MONOREPO-INTO-SINGLE-FILES` via `ticket_reopen` (goal already `🎯aioptimizedrepo🎯singlefilerepo`, no goal changes needed).
- Add any new temp/rewrite scripts for this session under that same ticket folder.

## Phase 1 — Flatten the 2 stray `src/` folders (highest blast radius)

- `repo/lib/js/src/{index.ts,index.test.ts}` → `repo/lib/js/{index.ts,index.test.ts}`.
  - Update [`repo/lib/js/package.json`](repo/lib/js/package.json) (`exports["."]`: `./index.ts` → `./index.ts`; `"test"` script: `bun test ./index.test.ts` → `./index.test.ts`).
  - Update [`repo/lib/js/project.json`](repo/lib/js/project.json) `sourceRoot`, and `tsconfig.json`/`nx-plugin.mjs` if they reference `src/`.
  - Rewrite every import of `repo/lib/js/index.ts` (relative variants like `../../repo/lib/js/index.ts`) across the ~150 files that reference it, using a one-off rewrite script under the ticket folder (same approach as the existing `rewrite-lib-js-imports.ts`).
- `ui/styling/rs/src/generated.rs` → `ui/styling/rs/generated.rs`.
  - Update `lib.rs`'s `#[path = "src/generated.rs"]` → `#[path = "generated.rs"]`.
  - Update the styling token generator (`ui/styling/script.ts` / `vite-elements-assets.ts`) if it writes to the old `src/` path.
- Validate: `bun run build`/typecheck across the workspace, `cargo check -p elements-styling-rs` (or the crate's actual package name).

## Phase 2 — Remove the one extra script file

- `compose/client/ui/gh/Compose.Grasshopper/yak/script.ts` is a second `script.ts` nested beneath a bundle that already has its own `script.ts` — violates "only `script.ts` per bundle".
- Merge its `build`/`publish`/`setup`/`test` command classes into the parent [`Compose.Grasshopper/script.ts`](compose/client/ui/gh/Compose.Grasshopper/script.ts) router (rename subcommands as needed to avoid clashing with the existing `build`/`test` targets, e.g. fold yak packaging into `build`, keep yak push/search under `test`, login under `setup`).
- Delete `yak/script.ts` (keep other non-script files under `yak/`, e.g. `manifest.yml`).
- Update [`Compose.Grasshopper/project.json`](compose/client/ui/gh/Compose.Grasshopper/project.json) so `build`/`publish`/`setup`/`test` targets call only `bun ./script.ts <command>`.

## Phase 3 — Merge remaining scattered same-bundle files

Continuing the ticket's established pattern (fold sibling files into the bundle's single entry file via `#region` sections):

- `mathematical/graph/manifest/core/validate.ts` → `core/index.ts`
- `ui/styling/js/{sizing.ts,resolve.ts,icon-render-port.ts}` → `index.ts` (leave `tokens.generated.ts`, it's build-generated)
- `mit-bestand/präsentation/33.projektetage/spec.ts` → `index.ts`
- `semios/core/rust-studio.ts` → `index.ts`
- `vcs/play/demo.ts` → `index.ts`
- `framework/product/presentation/renderer/react/json.tsx` → `index.tsx`
- `infinite/cavas/rs/{icon_codec.rs,theme.rs}` → `lib.rs` (leave `build.rs`, required by cargo)
- `kernel/2d/rs/{booleans.rs,trace.rs}` → `lib.rs`
- `kernel/2d/engine/compute.rs` → `lib.rs`
- `kernel/3d/engine/compute.rs` → `lib.rs`
- `compose/client/lib/py/store.py` → `main.py`
- `coda/client/bin/assistant/reference.py` → `main.py`
- `compose/client/lib/go/kit_graph.go` → `main.go`
- Investigate `mathematical/graph/manifest/generated/` (11 paired `.ts`+`.rs` manifest documents, `registry.rs`, `types.ts`, `manifest.schema.json`): check `script.ts`/`build.rs` for a generator first; if hand-authored, fold the 11 `.ts` data modules into one file with `#region` sections re-exported from `index.ts` (leave the `.rs` side if `registry.rs` already serves as its aggregator).

**Explicitly NOT touched** (required-separate, matches the ticket's own exception policy): the 10× `fixture-slugs.ts` helpers (shared by Node `script.ts` and browser `index.ts` — can't merge without breaking the Node/browser boundary), worker/preload entry files (`kit-store.worker.ts`, `tessellate.worker.ts`, `preload.ts` ×2, `worker.ts` — separate bundler entry points), all `build.rs` cargo build scripts, and Vite/Vitest/Playwright config files.

## Phase 4 — Fill in missing `package.json` scripts blocks

`sequence/core`, `sequence/react`, `imperative/core`, `imperative/react`, `framework/core` are missing the `"scripts"` block that sibling packages like `raster/core`/`semios/core` already have (`"test": "bun nx run @semio-tech/<pkg>:test"`). Add matching blocks.

## Phase 5 — Route remaining `project.json` bypasses through `script.ts`

Using the generic helpers already in `repo/lib/js` (`runCargo`, `runCmd`, `runViteBuild`, `runVitest`, `runBunx`, `runPlaywright`), add missing command classes to each bundle's `script.ts` (creating one where absent) so `project.json` targets call only `bun ./script.ts <command>`:

- Cargo direct calls: `reasoning/mindmap`, `procedural/{2d,3d}/rs`, `puzzle/{3d,5d}/rs`, `gis/2d/rs`, `writer/rs`, `trinity/jack/lsp`, `mathematical/graph/manifest`, `compose/server/hub`, `compose/client/lib/{query,rs}`
- Go direct calls: `repo/client/cli`, `repo/lib/go`, `repo/server/coordinator`, `coda/client/lib/{programming,blnbo}/go`, `compose/client/lib/go`
- .NET direct calls: `compose/client/lib/net/Compose{,.Tests,.Benchmark}`, `compose/client/ui/3dm/Compose.Rhino{,.Tests}`, `compose/client/ui/gh/Compose.Grasshopper.Tests`
- Python/uv direct calls: `compose/client/bin/engine`, `compose/client/lib/py`
- Vite direct calls: `compose/client/ui/vscode`, `compose/client/ui/3dm/ui`, `compose/client/lib/sketchpad/{doc,js,play}`, `repo/client/vscode`
- `bun test`/`bunx tsc` direct calls: `kernel/2d/js`, `ui/styling/js`, `repo/lib/js`
- Electron/Storybook/eslint/vsce bypasses: `compose/client/ui/desktop`, `compose/dev/algorithm`, `repo/client/vscode`
- Stub no-op targets with no `script.ts` at all: `asset`, `asset/icon`, `puzzle/asset`, `compose/fixture`

This phase is the largest and riskiest (touches Rust/Go/.NET/Python/Electron toolchains); it is scoped last so Phases 1-4 land safely first.

## Phase 6 — Validate and close

- Run typecheck/build/tests for every touched bundle (`bun nx run <pkg>:test`, `cargo test`/`cargo check` for touched Rust crates, `go test ./...` for touched Go modules, `dotnet test` where feasible).
- Update the ticket summary with every file created/moved/deleted/modified, then leave it `closed` via `ticket_close` if all phases land, or keep `open` with an updated summary if Phase 5 is deferred.
