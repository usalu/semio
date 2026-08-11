# w5a — Relocate `puzzle-5d-react` renderer target into the puzzle plugin

## Goal
Move the `@semio-tech/puzzle-5d-react` TS renderer target out of the generic
framework renderer-engine tree and into the `🧩️puzzle` plugin it renders for.

## Old location
`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🟦️typescript/🎯️targets/⚛️puzzle-5d-react/`
(sibling of the generic `⚛️react` target — was the only plugin-specific
renderer target living in the generic framework tree.)

## New location
`✏️s/🔌️plugins/🧩️puzzle/📦️packages/🟦️typescript/🎯️targets/⚛️5d-react/`

Chosen by mirroring the framework's own `📦️packages/🟦️typescript/🎯️targets/<tech>`
convention (seen at `🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react`
and the sibling `🦀️rust/🎯️targets/⌨️tui` / `🧊️wgpu`), nested one level under the
puzzle plugin's existing `📦️packages/🟦️typescript` package (home of
`@semio-tech/puzzle-js`). Neither `📐️cad` nor `🌊️flow` (the two other plugins
checked for precedent) have any custom renderer-target subfolder — they only
have a single flat `📦️packages/🟦️typescript` — so there was no existing
plugin-level convention to match beyond the framework one. Dropped the
`puzzle-` prefix from the directory name (redundant once nested inside the
puzzle plugin, matching how the plugin's own `🎛️apps/🖐️5d` is named without a
`puzzle-` prefix) but **kept the npm package name
`@semio-tech/puzzle-5d-react` unchanged** — it's an external import specifier
(`compose/client/lib/sketchpad/js/index.ts:17021` does
`await import(/* @vite-ignore */ "@semio-tech/puzzle-5d-react")`), and
renaming it was out of scope (task only asked to fix paths/names that encode
the *old location*, not the logical package identity).

## What moved (via `git mv`, node_modules left behind — gitignored build cache, removed)
- `package.json`
- `📋️project.json`
- `📜️script.ts`
- `📦️index.tsx`
- `🧪️vitest.config.ts`

Old now-empty framework target dir was removed entirely (only contained the
gitignored `node_modules/` cache dir, nothing tracked left over).

## Fixes inside the moved package
- **`package.json`**: `$schema` relative depth 10→7 `../`; `repository.directory`
  updated from the stale ASCII mirror path
  `framework/product/os/module/renderer/js/react/puzzle-5d` to the real
  new path `✏️s/🔌️plugins/🧩️puzzle/📦️packages/🟦️typescript/🎯️targets/⚛️5d-react`
  (following the `🌊️flow` TS package's convention of using the literal path,
  since ASCII-mirror conventions across plugins were already inconsistent —
  `📐️cad` uses `s/plugin/cad/js`, `🎞️animate` uses
  `s/plugin/animate/package/typescript` — picked the one that's actually
  correct/unambiguous rather than inventing a fourth variant).
- **`📋️project.json`**: `$schema` depth 10→7 `../`; `sourceRoot` and both
  targets' `cwd` repointed to the new path; `tags` changed
  `"scope:framework"` → `"scope:plugin"` (confirmed `scope:plugin` is an
  existing tag value used elsewhere in the repo, so this isn't a new
  vocabulary word).
- **`📜️script.ts`**: relative import back to the shared repo library
  (`🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts`) — depth
  10→7 `../`, matching the depth used by the sibling `@semio-tech/puzzle-js`
  package's own `📜️script.ts` (5 `../` at depth 5) scaled for the 2 extra
  `🎯️targets/⚛️5d-react` segments (5+2=7).
- **`🧪️vitest.config.ts`** and **`📦️index.tsx`**: no changes needed — neither
  had any relative-path references to the old location (config derives its
  root from `import.meta.url`; the index is a pure compose/flatten module
  with zero external relative imports).

## External references fixed
- **`package.json`** (repo root) — `workspaces` array: removed the old
  framework-tree entry, added the new plugin-tree entry (placed alongside
  the other `✏️s/🔌️plugins/📐️cad/...` block rather than mid-cad-extensions
  where it first landed — fixed placement to keep cad's own subentries
  contiguous).
- **`compose/client/lib/sketchpad/js/vitest.config.ts`** — the
  `@semio-tech/puzzle-5d-react` alias now resolves to
  `../../../../../✏️s/🔌️plugins/🧩️puzzle/📦️packages/🟦️typescript/🎯️targets/⚛️5d-react/📦️index.tsx`
  (same 5-`../` depth as before, since the sketchpad package's own depth
  didn't change).

## Checked, found nothing to fix
- **Plugin-registry generated playground catalog**
  (`🧰️framework/.../🔌️plugin/📦️packages/🟦️typescript/📇️registry/🟦️catalog.ts`
  and `🤖️generated/🔣️playgrounds.json`): the puzzle5d playground row is keyed
  by `variant`/`app` (from `✏️s/🔌️plugins/🧩️puzzle/📦️packages/🦀️rust/Cargo.toml`'s
  `[[package.metadata.semio.playground]]` block) — no hardcoded path to the TS
  renderer target anywhere in the registry source-of-truth. Nothing to
  change there.
- **`.claude/launch.json`**: the `"puzzle-5d-react"` entry just runs the npm
  script `dev:puzzle:5d` (→ `bun ./📜️script.ts dev 5d`), which doesn't
  reference the TS target's filesystem path directly. No change needed.
- **`.vscode/launch.json`**: only unrelated `wgpu` native-target hits for the
  `puzzle5d` *variant* filter argument, not this TS package. No change
  needed.
- **`.cursor/fix-playhost-extraction.ts`** and **`.cursor/plans/*.md`**:
  contain stale/historical references to `@semio-tech/puzzle-5d-react`
  (including an already-broken ASCII mirror path `puzzle/5d/react/index.tsx`
  that never existed). Out of my assigned file scope (one-off historical
  migration scripts / plan docs, not live source-of-truth config) — left
  untouched.
- **`bun.lock`**, **`.nx/workspace-data*/*.json`**: generated/cache files
  that reference the old path — both regenerate automatically on next
  `bun install` / `nx` run. Not hand-edited.

## Verification
- `bun ✏️s/🔌️plugins/🧩️puzzle/📦️packages/🟦️typescript/🎯️targets/⚛️5d-react/📜️script.ts test`
  from the new location: **1 test file, 1 test, passed** (237ms). Confirms
  the moved package's own vitest config, its `$schema`/import paths, and its
  relative import into the shared repo library all resolve correctly.
- Manually resolved the new `compose/client/lib/sketchpad/js/vitest.config.ts`
  alias path on disk — `📦️index.tsx` exists at the resolved location.
- Tried running the specific compose-sketchpad test that imports
  `@semio-tech/puzzle-5d-react`
  (`bunx vitest run --config vitest.config.ts -t "emits relative fixtures flattened through puzzle 5d"`
  inside `compose/client/lib/sketchpad/js`): the suite fails to load, but
  for a **pre-existing, unrelated** reason — `index.ts` fails at its very
  first import, `Cannot find package '@semio-tech/framework-platform-core'`.
  That alias (and several siblings: `@semio-tech/framework`,
  `@semio-tech/compose-js`, `@semio-tech/reasoning-mindmap-*`,
  `@semio-tech/infinite-*`, `@semio-tech/puzzle-2d-react`) all point at an
  ASCII `framework/...` tree that doesn't exist in this repo (only the emoji
  tree does) — confirmed with `ls`, no such directory. This whole alias
  block in this vitest config was already broken before my change; my one
  edited alias (`puzzle-5d-react`) is the only one verified correct by
  filesystem inspection since the suite can't get far enough to prove it at
  runtime. Not fixing the sibling aliases — out of scope for this move and
  a pre-existing issue unrelated to what I touched.
- `bunx nx show project @semio-tech/puzzle-5d-react` fails, but for a
  **pre-existing, unrelated** reason: Nx reports a duplicate-project-name
  conflict for `@semio-tech/infinite-world-r3f` (defined at two mojibake'd
  paths differing only in a corrupted directory-name byte), which looks like
  another concurrent session's in-progress rename mid-refactor. Confirmed no
  other `📋️project.json` in the repo declares
  `"name": "@semio-tech/puzzle-5d-react"` besides the one at the new
  location — the move itself introduced no naming conflict.

## Files touched
- Moved (git mv): `✏️s/🔌️plugins/🧩️puzzle/📦️packages/🟦️typescript/🎯️targets/⚛️5d-react/{package.json,📋️project.json,📜️script.ts,📦️index.tsx,🧪️vitest.config.ts}`
  (from `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🟦️typescript/🎯️targets/⚛️puzzle-5d-react/`)
- Edited: `package.json` (repo root, workspaces array)
- Edited: `compose/client/lib/sketchpad/js/vitest.config.ts` (alias path)
- Removed: old empty framework target directory (only contained gitignored
  `node_modules/` cache, nothing tracked)

## Not touched (out of scope / pre-existing / other sessions)
- `.cursor/fix-playhost-extraction.ts`, `.cursor/plans/*.md` — historical,
  already-stale references, outside assigned file list.
- `bun.lock`, `.nx/workspace-data*/*.json` — generated, regenerate on next
  install/graph build.
- `🧰️framework/.../🧱️elements/Shell/🧊️component.rs` — showed as modified in
  `git status` during this session but was not touched by me; presumably
  another concurrent dev's in-flight change.
- The broken `@semio-tech/framework-platform-core` et al. aliases in
  `compose/client/lib/sketchpad/js/vitest.config.ts` — pre-existing breakage
  unrelated to this move.
- The Nx duplicate-project-name conflict on `@semio-tech/infinite-world-r3f`
  — pre-existing, unrelated to this move, looks like another session's
  in-progress rename.
