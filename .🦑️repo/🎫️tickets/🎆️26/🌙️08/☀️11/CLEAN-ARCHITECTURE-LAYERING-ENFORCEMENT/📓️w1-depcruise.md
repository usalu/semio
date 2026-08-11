# W1 — `.dependency-cruiser.cjs` layering rules

Scope: **only** `/Users/ueli/Documents/semio/.dependency-cruiser.cjs` (per file ownership).

## 1. Fixed `no-generated-edits-upstream`

Stale path `🧰️framework/…/🔌️plugin/⚡️implementations/🟦️typescript/📇️registry/` did not match reality
(confirmed via `find`: no `⚡️implementations` segment exists under `🔌️plugin` anymore). Real path is
`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🟦️typescript/📇️registry/`, confirmed to exist
and to contain `🤖️generated/🔣️plugins.json`. Repointed both `from.pathNot` and `to.path`.

Verified with regex tests against real paths:
- `from.pathNot` correctly excludes (allows) the registry's own files as importers.
- `from.pathNot` correctly does NOT exclude (i.e. flags) an outside importer (e.g. `🔌️plugin/🖥️host/…`).
- `to.path` correctly matches the real generated `🔣️plugins.json`.

## 2. Self-deriving infrastructure added

- `escapeRegex(literal)` — shared regex-escape helper (previously inlined ad hoc in a couple of places).
- `scanPackageJsonFiles(rootAbsDir)` — recursive directory walk (skipping `node_modules|dist|target|pkg|storybook-static|.git|.nx|🦑️repo|repo`,
  same exclusions as `options.doNotFollow` plus `pkg/` per `noCorePathRule`'s existing convention for
  wasm-pack output) returning `{dir, name}` for every `package.json` found.
- `S_PACKAGES = scanPackageJsonFiles(path.join(__dirname, "✏️s"))` — all 42 real `package.json` files
  under `✏️s` (verified via `find "✏️s" -name package.json`), used to derive `@semio-tech/*` name
  equivalents for the new path-based rules below. `PLUGINS` (already existing, `fs.readdirSync` on
  `✏️s/🔌️plugins`) was reused as-is, not re-hardcoded.

## 3. `framework-no-s` (new, severity `warn`)

`from: ^🧰️framework/` → `to:` `^✏️s/` plus every `@semio-tech/*` name found in `S_PACKAGES` (exact-anchored).
`warn` because real violations of this direction are known to exist elsewhere in the codebase (other
waves' job to clear); not promoted to `error` per task instructions.

## 4. `s-modules-no-plugins` (new, severity `error`)

`from: ^✏️s/🔨️modules/` → `to:` `^✏️s/🔌️plugins/` plus the `@semio-tech/*` names of every `S_PACKAGES`
entry whose dir is under `✏️s/🔌️plugins/` (covers plugin-core, extensions, and nested plugin-owned
modules like `🔱️trinity/🔨️modules/🔌️jack/🧠️lsp` — all live under the plugins tree). Set to `error`
immediately per task instructions (audit found zero real violations of this direction).

## 5. `no-plugin-to-extension-{plugin}` (new, one rule per plugin in `PLUGINS`, severity `warn`)

For each `p` in `PLUGINS` (33 plugins, derived — not hardcoded):
`from: ^✏️s/🔌️plugins/{p}/` excluding `pathNot: ^✏️s/🔌️plugins/{p}/🧩️extensions/` →
`to:` `^✏️s/🔌️plugins/[^/]+/🧩️extensions/` (any plugin's extensions dir) plus the `@semio-tech/*` names
of every `S_PACKAGES` entry whose dir contains a `🧩️extensions/` path segment (4 real extension packages
found today, all under `📐️cad/🧩️extensions/*`). `from.pathNot` exempts extension→extension imports
(including within the same plugin) as specified. `warn` per task instructions.

Verified via direct regex tests against real repo paths/names (see below) — all matched as intended:
- `✏️s/🔌️plugins/📐️cad/📦️packages/…` flagged as `from` (core code).
- `✏️s/🔌️plugins/📐️cad/🧩️extensions/…` excluded from `from` (own extensions exempt).
- `to` matches both the raw extensions-dir path and the derived extension package name
  `@semio-tech/cad-js-module-aec-building-energy`.
- `to` does NOT false-positive match the plugin's own core package name `@semio-tech/cad-js`.

## 6. Runtime verification

`node -e "require('./.dependency-cruiser.cjs')"` — loads cleanly, no syntax/reference errors.
`forbidden.length` = 1090 rules total; `no-plugin-to-extension-*` produced exactly 33 entries (one per
derived plugin).

Checked `📜️script.ts` (lines 655, 675) for the actual invocation this repo uses — no npm/nx script or
project.json target wraps it; both `sync`/`verify` call
`bunx dependency-cruiser@16 compose 🧰️framework ✏️s 🌎️hub ♻️mit-bestand --config .dependency-cruiser.cjs --output-type err`
directly. Ran the closest equivalent the task requested:

```
bunx dependency-cruiser@16 compose 🧰️framework ✏️s --config .dependency-cruiser.cjs --output-type err-long
```

Result: `✔ no dependency violations found (187 modules, 257 dependencies cruised)`. **No config error, no
crash** — confirms the config itself is valid and loads/executes successfully end-to-end through the
CLI, matching the task's pass bar ("only a crash/config error is a problem").

**Caveat found and worth flagging (not fixed — outside file ownership, pre-existing, not introduced by
this change):** `bunx dependency-cruiser@16 --info` shows `.ts`/`.tsx` support marked unavailable
(`x typescript`) in this environment — `bunx` fetches an ephemeral copy of dependency-cruiser that
cannot resolve the repo's local `typescript` package (dependency-cruiser dynamically
`tryImport("typescript", …)` relative to its own install location, which for a `bunx`-fetched package is
outside the repo's `node_modules` tree; confirmed no `dependency-cruiser` devDependency exists anywhere
in the repo's `package.json` files, and no global bun install directory provides it either). Practical
effect: this bunx invocation only actually parses `.js`/`.cjs`/`.mjs`/`.jsx` files (mostly wasm-pack
`pkg/` output and dev-shim bundles) and silently skips the ~5400 `.ts`/`.tsx` source files under
`🧰️framework` + `✏️s`, which is why the CLI run above found 0 violations across the board (both
pre-existing rules and my new ones) — it isn't meaningfully exercising most of the ruleset, including my
new rules, against real TS source. This is identical behavior for the exact invocation already baked
into `📜️script.ts` (not something my config change caused), so it's a systemic tooling gap, not a
regression. To compensate, I validated all new/fixed rules' regexes directly against real repo paths and
`package.json` names (see sections 1, 3, 4, 5 above) using `node -e` — every pattern matched/excluded
exactly as intended. Flagging this bunx/typescript resolution gap as worth a separate ticket for whoever
owns `package.json`/tooling setup, since it silently defeats most of this config's TS-path coverage today.

## Files touched

- `/Users/ueli/Documents/semio/.dependency-cruiser.cjs` (only file edited, per ownership)
