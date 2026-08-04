---
name: Pluralize kind folders
overview: Rename all taxonomy container folders from singular to plural across the monorepo (862 directories, e.g. ⚡️implementations→⚡️implementations, 🔨️modules→🔨️modules, ✏️s/🔌️plugin→✏️s/🔌️plugins) and rewrite every path reference in the ~1,200 active source and config files, plus the six npm package names derived from those folders.
todos:
  - id: ticket
    content: Open a ticket for the plural-folder-names refactor (repo MCP or repo CLI), associating it with the most appropriate goal from repo://goals
    status: completed
  - id: rename-dirs
    content: Write and run the ticket-local migration script that renames the 862 taxonomy container directories deepest-first using filesystem renames
    status: completed
  - id: hook-collision
    content: "Consolidate the 🪝️hooks / 🪝️hooks duplicate: newer 🪝️hooks content lands at 🪝️hooks, retarget the Go CLI paths"
    status: completed
  - id: rewrite-refs
    content: Rewrite path references across the 1,202 active files, covering forward-slash, backslash, regex-escaped, quoted-segment, and glob forms with the 🔌️plugin anchor and trailing-boundary guards
    status: completed
  - id: fix-regex-and-prose
    content: Hand-fix the regex literals in the plugin registry script and React engine test, plus prose mentions the mechanical pass mangles
    status: completed
  - id: rename-packages
    content: Pluralize the six folder-derived npm package names and every dependent, including compose/** aliases and root package.json workspaces
    status: completed
  - id: relock
    content: Regenerate bun.lock with bun install
    status: completed
  - id: verify
    content: Run verify gate, cargo metadata, go build, nx show projects, dependency-cruiser, full test suite, and one dev target for runtime asset resolution
    status: completed
  - id: close-ticket
    content: Close the ticket with summary and the full list of touched files
    status: completed
isProject: false
---

## Rule

A folder whose children are *instances of a kind* becomes plural. A folder that *is* an instance keeps its singular name. So `✏️s/🔌️plugins/` (holds 32 plugins) becomes `✏️s/🔌️plugins/`, while `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/` (one module named "plugin") stays singular.

## Directories to rename (862 total)

- `⚡️implementations` -> `⚡️implementations` (637)
- `🔨️modules` -> `🔨️modules` (72)
- `📚️examples` -> `📚️examples` (42)
- `🎛️apps` -> `🎛️apps` (30)
- `🖼️assets` -> `🖼️assets` (10)
- `🧩️extensions` -> `🧩️extensions` (7)
- `🗂️typologies` -> `🗂️typologies` (7)
- `🎬️actions` -> `🎬️actions` (7)
- `🎬️interactions` -> `🎬️interactions` (6)
- `🔀️transformations` -> `🔀️transformations` (6)
- `🔣️icons` -> `🔣️icons` (4)
- `🧫️fixtures` -> `🧫️fixtures` (3)
- `🪧️logos` -> `🪧️logos` (3)
- `🖼️images` -> `🖼️images` (3)
- `🏷️attributeDefinitions` -> `🏷️attributeDefinitions` (3)
- `📊️statDefinitions` -> `📊️statDefinitions` (3)
- `🔤️fonts` -> `🔤️fonts` (2)
- `🔌️ports` -> `🔌️ports` (2)
- `🛍️products` -> `🛍️products` (1)
- `🏗️modelDefinitions` -> `🏗️modelDefinitions` (1)
- `🔧️propertyDefinitions` -> `🔧️propertyDefinitions` (1)
- `🏷️propertyKinds` -> `🏷️propertyKinds` (1)
- `🪝️hooks` -> `🪝️hooks` (1, see collision below)
- `✏️s/🔌️plugin` -> `✏️s/🔌️plugins` (1, path-anchored only)

## Deliberately left singular

Instance names, not containers: `🖱️ui`, `⚙️engine`, `🧑️‍🎨️engine`, `🫀️core`, `🗣️dsl`, `🎒️pack`, `📡️protocol`, `🔧️op`, `🖥️host`, `👷️worker`, `💻️os`, `🦑️repo`, `📓️print`, and the `🔌️plugin` module under `💻️os/🔨️modules/`. Also `🛂️manifest`, `🗿️artifact`, `🧬️schema` — each wraps a single unit, per your scope choice.

Untouched trees: `.🦑️repo/🎫️tickets/**` (historical), `compose/**` and `♻️mit-bestand/**` folder names, plus `node_modules`, `target`, `dist`, `pkg`, `out`, `🤖️generated`, `storybook-static`, `temp`, `.nx`, `.venv`, and tool-owned dirs (`.cursor/plans`, `.agents/skills`, `.github/workflows`).

## Two pre-existing problems this surfaces

- **`🪝️hooks` vs `🪝️hooks` collision.** `🧰️framework/🛍️products/🦑️repo/` contains both. `🪝️hooks/` (Aug 3, 1733 bytes each) is what the Go CLI installs from in [main.go](🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/⌨️cli/⚡️implementations/🐹️go/main.go); `🪝️hooks/` (Jul 31, 1493 bytes) is stale but is what [📦️index.ts](🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️lib/⚡️implementations/🟦️typescript/📦️index.ts) line 3915 reads. Resolution: the newer `🪝️hooks/` content lands at `🪝️hooks/`, overwriting the stale copies, and the Go paths retarget to `🪝️hooks`.
- **Package-name scope narrower than the nine listed.** Only six have a folder that actually pluralizes: `@semio-tech/assets`->`assets`, `@semio-tech/icons`->`icons`, `@semio-tech/images`->`images`, `@semio-tech/logos`->`logos`, `@semio-tech/puzzle-assets`->`puzzle-assets`, `@semio-tech/remodel-images`->`remodel-images`. `@semio-tech/graph-manifest` stays (its `🛂️manifest` folder stays singular) and `@semio-tech/compose-fixture` stays (its `compose/fixture` folder is out of scope). `@semio-tech/ui-asset` turned out not to exist — only stale mentions in `.cursor/plans/`. Rust crate names (`semio-s-kernel-flow-extension-brep`, `semio-s-plugin-remodel-image`, ...) are semantic identifiers, not paths, and stay as-is.

## Reference rewrite: 1,202 files

Breakdown: 616 `.toml`, 248 `.ts`, 220 `.json`, 87 `.rs`, 10 `.css`, plus `go.work`, `.gitignore`, `.dependency-cruiser.cjs`, the five git hook scripts, and `.vscode/launch.json` (46 hits) / `settings.json`.

The rewrite must handle five syntactic forms, not just `a/b/c`:

- Forward-slash paths: `Cargo.toml` `path =` deps and workspace `members`, `📋️project.json` `cwd`, `package.json` `workspaces`, `nx.json` plugin paths, `go.work` `use`, [🧪️vitest.config.ts](🧪️vitest.config.ts) project list, Rust `include_str!` / `include_bytes!` / `#[path]`.
- Windows backslash paths, e.g. `🔨️modules\💻️client\🧩️` in [main_test.go](🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/⌨️cli/⚡️implementations/🐹️go/main_test.go).
- Regex literals with escaped separators in the plugin registry script and the React engine test:

```1:1:🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚡️implementations/🟦️typescript/📇️registry/📜️script.ts
const isExtensionCrate = /\/✏️s\/🔌️plugins\/[^/]+\/🧩️extensions\/[^/]+\/⚡️implementations\/🦀️rust\/Cargo\.toml$/.test(path);
```

- Standalone quoted segments in `join(...)` calls, e.g. `join("🧰️framework", "🛍️products", "🦑️repo", "🔨️modules", "💻️client")` in [📜️script.ts](📜️script.ts) line 69.
- Glob patterns in [.gitignore](.gitignore) and [.dependency-cruiser.cjs](.dependency-cruiser.cjs) template literals like `` `^✏️s/🔌️plugins/${from}/` ``.

Two guards are mandatory:

- `🔌️plugin` is replaced only when anchored to `✏️s` (`✏️s/🔌️plugin`, `✏️s\/🔌️plugin`, `✏️s", "🔌️plugin`, `✏️s\🔌️plugin`). The 147 `💻️os/🔨️modules/🔌️plugin/` hits and the `🔌️plugin-modules` gitignore glob must survive untouched.
- Every token needs a trailing boundary check (not `[A-Za-z0-9_-]` and not already `s`) so `🔣️icon_svgs` is not corrupted and `🧫️fixtures` / `🪝️hooks` are not double-pluralized. This also makes the migration idempotent.

## Execution

1. Open a ticket. The repo MCP server is not connected in this session, so I will need it enabled, or I will drive `ticket_open` through the repo CLI at `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/⌨️cli`. All temporary scripts and logs go in the ticket folder.
2. Write a one-off migration script in the ticket folder that renames deepest-first (so parent renames do not invalidate child paths) using plain filesystem renames — no `git mv` or any other index-modifying git command.
3. Run the reference rewrite over the 1,202 files, then hand-fix the five regex literals and the prose mentions the mechanical pass mangles (docstrings like "over every real shipped `📚️examples` fixture" read wrong as "`📚️examples` fixture").
4. Rename the six package directories' `"name"` fields and every dependent, including root [package.json](package.json) `workspaces` + `dependencies` and the `compose/**` vite/tsconfig aliases that import `@semio-tech/assets`.
5. Regenerate `bun.lock` via `bun install`.

## Verification

- `bun install` clean, then `bun ./📜️script.ts verify gate` (lint + generated-catalog freshness + region/host-contract lints).
- `cargo metadata --no-deps` to prove all 599 `Cargo.toml` path deps resolve; `go build ./...` across the four `go.work` modules.
- `bun nx show projects` to confirm no project vanished, and `bunx dependency-cruiser@16` (the rule set hard-codes `^✏️s/🔌️plugins/` prefixes).
- `bun ./📜️script.ts verify` for the full test suite, and launch one dev target (`🛠️dev📐️cad⚛️react`) to confirm runtime asset resolution after the `🖼️assets`/`🔤️fonts`/`🔣️icons` renames.
- Close the ticket with the summary and full file list.
