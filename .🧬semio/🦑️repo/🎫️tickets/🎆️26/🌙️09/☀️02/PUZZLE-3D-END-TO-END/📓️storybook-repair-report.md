# Storybook repair — World.stories.tsx import fixes — agent U, 2026-09-05

## Verdict

I repaired all four dead imports in `.storybook/stories/puzzle/3d/World.stories.tsx` that agent T's
`📓️storybook-render-probe.md` found. Each fix is verified **statically** (the target paths exist
on disk, resolved from the story file's own directory, byte-for-byte). I could **not** get a real
`storybook build` (or the Playwright spec) to run to completion, because a *different, unrelated*
build step — `.storybook/main.ts`'s taxonomy schema validation — is currently broken repo-wide by
another session's in-flight `stdio/step` migration. This is not caused by my changes and not fixable
from inside my write-lock. Full evidence below.

## 1. Imports fixed in `World.stories.tsx`

### 1a. `concreteForestFixtureDsl` (line 15)

- Before: `"../../../../✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/📚️examples/🌲️concrete-forest/🖼️assets/🗣️forest.dsl.semio?raw"`
- After: `"../../../../✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🌲️concrete-forest/🖼️assets/🧪️forest/🗣️.dsl.semio?raw"`
- Evidence: `find "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d" -iname "*forest*"` lists exactly this path;
  the shallow `🗿️artifacts/🧊️3d/📚️examples/...` path does not exist (`ls` on it fails — no such
  directory at all, confirmed independently of agent T's probe). Also confirmed by a Python
  `os.path.normpath(os.path.join(story_dir, rel_import))` + `os.path.exists()` check from the
  story file's actual directory (`.storybook/stories/puzzle/3d`) — resolves to `True`.

### 1b. `nakaginCapsuleTowerFixtureDsl` (line 16)

- Before: `"../../../../✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/📚️examples/🏗️nakagin-capsule-tower/🖼️assets/🗣️tower.dsl.semio?raw"`
- After: `"../../../../✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🏗️nakagin-capsule-tower/🖼️assets/🧪️tower/🗣️.dsl.semio?raw"`
- Evidence: same method as 1a — `find`-confirmed on disk, and `os.path.exists()` resolves `True`
  from the story file's directory.

### 1c. `capsuleDreamFixtureDsl` / `CapsuleDream` story — **removed, not fixed**

- Before: `import capsuleDreamFixtureDsl from "../../../../.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️09/PUZZLE-DESIGN-PARITY/🌙️capsule-dream-out/🗣️dream.3d.dsl.semio?raw";`
  plus the `export const CapsuleDream: Story = { args: { fixtureDsl: capsuleDreamFixtureDsl } }` block.
- After: import and story both deleted.
- Evidence the fixture genuinely does not exist anywhere (not just at the wrong path):
  - `.🦑️repo` at repo root is a build-cache dir, not the ticket store — confirmed no such tree exists.
  - `find . -iname "*capsule-dream*" -not -path "*/node_modules/*"` returns: two `temp/compose/...`
    JSON/GH fixtures (unrelated plugin), a **`🖐️5d`** puzzle-artifact example dir
    (`✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🌙️capsule-dream`
    — 5d, not 3d, a different artifact/schema entirely), and a report markdown in
    `PUZZLE-DESIGN-PARITY`. None of these is a `3d` DSL fixture.
  - `find . -iname "*dream*dsl*"` (repo-wide) returns **nothing**.
  - Listing `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️09/PUZZLE-DESIGN-PARITY/` (the real ticket store
    path) shows no `🌙️capsule-dream-out` directory and no `dream.3d.dsl.semio` file anywhere in it.
  - Per the task's explicit instruction ("if it does not [exist], delete that story variant rather
    than inventing a substitute fixture"), I deleted the `CapsuleDream` export and its import rather
    than fabricate a fixture or repoint it at the unrelated 5d example. `puzzle-3d-5d-infinite.spec.ts`
    never references a `CapsuleDream` story id, so nothing downstream depends on it.
  - Note: the sibling `puzzle/2d` story (`Fixtures.stories.tsx`) has the exact same dead
    `.🦑️repo/.../🌙️capsule-dream-out/🗣️dream.2d.dsl.semio` import and an equally-stale shallow
    `◻️2d/📚️examples/...` path for its forest/tower fixtures — same rot, different file. That file
    is **not** in my write-lock (only `World.stories.tsx` is), so I left it untouched and am
    flagging it here rather than fixing it silently.

### 1d. Wasm module import (line ~110, inside `loadPuzzle3dWasm`)

- Before: `import("@semio-tech/puzzle-3d-ui-rs/pkg/puzzle_3d.js")`
- After: `import("../../../../✏️s/🔌️plugins/🧩️puzzle/📦️packages/🦀️rust/pkg/semio_puzzle.js")`
- Why a relative path instead of a bare package specifier: I checked whether `@semio-tech/puzzle-wasm`
  (the real package name, confirmed via
  `✏️s/🔌️plugins/🧩️puzzle/📦️packages/🦀️rust/pkg/package.json`: `"name": "@semio-tech/puzzle-wasm"`)
  is reachable as a bare specifier at all:
  - Not a `package.json` workspace entry (`grep -n workspaces -A20 package.json` — the puzzle
    workspaces listed are `🎛️apps`/`📦️packages/🟦️typescript` targets only, no `📦️packages/🦀️rust`).
  - Not symlinked into `node_modules/@semio-tech/` (`ls -la node_modules/@semio-tech/` shows
    `puzzle-5d-react` and `puzzle-js` symlinks only — no `puzzle-wasm`).
  - Not aliased in `.storybook/main.ts`'s baseline `aliasRecord`, and `.storybook/scopes.ts`'s
    `puzzle/3d` scope entry (lines ~94-98) has no `aliases` key — confirmed by reading it directly.
  - The convention that **does** work today is the one the real production code already uses: the
    puzzle 2d editor's own wasm loader
    (`✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🟦️.ts`)
    imports the exact same physical file via a **relative path**:
    `import("../../../../../../../../📦️packages/🦀️rust/pkg/semio_puzzle.js")`. I followed that
    convention rather than inventing an alias in a shared config file I was told to avoid touching.
  - Verified the free export exists in the actual built file: `grep -n "^export"` on
    `✏️s/🔌️plugins/🧩️puzzle/📦️packages/🦀️rust/pkg/semio_puzzle.js` shows
    `export function puzzle3dParseDslJson(dsl_text) {` (line 1638) and the module's default export
    is `export { initSync, __wbg_init as default }` — an async init function taking an optional
    `module_or_path`, matching the story's `Puzzle3dWasmModule` type (`default: (input?) => Promise<unknown>`)
    exactly. No shape mismatch.
  - Path-resolution check: `os.path.normpath(os.path.join(".storybook/stories/puzzle/3d", rel_import))`
    → `✏️s/🔌️plugins/🧩️puzzle/📦️packages/🦀️rust/pkg/semio_puzzle.js`, `os.path.exists()` → `True`.
  - Also updated the two doc comments (header summary, `loadPuzzle3dWasm` docstring) that named the
    old, nonexistent `@semio-tech/puzzle-3d-ui-rs` package, so the prose matches the real import.

Full diff (`git diff -- .storybook/stories/puzzle/3d/World.stories.tsx`) is on disk in the working
tree; nothing else in the file was touched.

## 2. The real build attempts (verbatim, not paraphrased)

Command:
```
cd /Users/ueli/Documents/semio
STORYBOOK_SCOPE=puzzle/3d bunx storybook build -c .storybook --output-dir ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️02/PUZZLE-3D-END-TO-END/🗑️generated/storybook-static-puzzle3d"
```

Ran this **three times**, spaced ~15s / ~45s / ~90s apart (in case it was a transient mid-write
race from a concurrent session). All three failed identically, before Vite ever starts bundling
anything — the failure is in `.storybook/main.ts`'s own preset evaluation, which happens before
`STORYBOOK_SCOPE` even filters which stories get bundled:

```
┌  Building storybook v10.5.6
│
◇  Cleaning outputDir: .../🗑️generated/storybook-static-puzzle3d
│
■  SB_CORE-SERVER_0007 (MainFileEvaluationError): Storybook couldn't evaluate your .storybook/main.ts file.

│  Original error:
│  Error: Invalid taxonomy schema:
│  - generatorContracts["external-step-assets"] tracked output
│  "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/1️⃣cc1/🧫️fixtures/🧪️hexagonal-cut-concrete-forest-left-ap214/📐️.stp"
│  is missing.
│  [... 6 more identical "is missing" lines for subsets 2️⃣cc2 through 6️⃣cc6 and 🧱️base ...]
│  at loadTaxonomy (file://.../🧰️framework/.../🔍️discovery/🟦️.ts:309:34)
│  at buildGeneratedScopes (file://./.storybook/scopes.ts:119:20)
│  at file://./.storybook/scopes.ts:141:26
│
▲  Failed to load preset: "/Users/ueli/Documents/semio/.storybook/main.ts"
│
■  Error: Invalid taxonomy schema: [same list]
└  Storybook exited with an error
```

Identical on run 2 and run 3 (same 7 missing `.stp` outputs, same stack trace) — this is a stable
failure, not flaky.

## 3. Root cause of the build failure — confirmed NOT my change, NOT puzzle3d

`.storybook/scopes.ts`'s `buildGeneratedScopes` calls `loadTaxonomy()` unconditionally while
evaluating `main.ts` — this runs for *every* Storybook invocation regardless of `STORYBOOK_SCOPE`,
before any scope-specific story bundling starts. `loadTaxonomy()`
(`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts:1283-1291`) validates
`generatorContracts` against the on-disk workspace and throws on any mismatch — there is no
env-var or flag to skip this (checked the surrounding source for one; none exists).

The specific mismatch is in the **`stdio` plugin's `step`/`ap214` artifact**, completely unrelated
to puzzle/3d:

```
$ git diff --stat HEAD -- "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step"
... 71 files changed, 290 insertions(+), 17013 deletions(-)

$ git show HEAD:"✏️s/.../ap214/🪆️subsets/1️⃣cc1/🧫️fixtures/🧪️hexagonal-cut-concrete-forest-left-ap214/📐️.stp"
EXISTS AT HEAD (committed)

$ ls "✏️s/.../ap214/🪆️subsets/1️⃣cc1/🧫️fixtures/🧪️hexagonal-cut-concrete-forest-left-ap214/"
No such file or directory   (deleted in the working tree, uncommitted)
```

So: the `.stp` files the taxonomy's `generatorContracts["external-step-assets"]` still declares as
tracked outputs are present in the last commit but have been deleted, uncommitted, in the live
working tree — a **massive in-progress edit by another concurrent session** (17,013 deleted lines
across 71 files under `stdio/step/ap214`), not something I touched, not in my write-lock, and not
something CLAUDE.md's "ignore unrelated recent changes, keep focusing on your own task" guidance
lets me route around — it's a hard, repo-wide `main.ts` load failure, not a scoped one. I did not
touch anything under `stdio/` or the taxonomy discovery library.

## 4. Playwright spec — not run

Since the Storybook build itself cannot complete right now (for the reason in §3, unrelated to
puzzle3d), there is no `storybook-static` build to serve, so I did not attempt
`bunx playwright test .storybook/puzzle-3d-5d-infinite.spec.ts --config .storybook/playwright.config.ts`
— it would have nothing to connect to. No screenshots were produced by anything I ran.

## 5. What this does and does not prove

**Proven:**
- The four dead references agent T found in `World.stories.tsx` are real and are now pointed at
  paths that exist on disk today (verified independently via `find`, `ls`, and Python
  `os.path.exists()` resolution from the story file's actual directory — not just trusting the
  probe doc).
- The wasm loader's expected shape (`default(): Promise<unknown>`, `puzzle3dParseDslJson(text): string`)
  matches what `semio_puzzle.js` actually exports, read directly from the built file.
- The edited file is syntactically balanced (brace/paren count check) and the removed
  `CapsuleDream` block left no dangling references (`grep -n "capsuleDream\|CapsuleDream\|puzzle-3d-ui-rs"`
  on the file after edits returns nothing).
- The one story ID this affects that's referenced elsewhere (`puzzle-3d-5d-infinite.spec.ts`'s
  `🧩️puzzle🧊️3d--concrete-forest` / `--nakagin-capsule-tower`) still exists — `CapsuleDream` was
  never referenced by that spec.

**Not proven — and I am not claiming it:**
- That the story actually renders in a browser. I could not get a `storybook build` to complete at
  all right now, for reasons entirely outside `World.stories.tsx` (see §3). The import-path fix is
  necessary but its sufficiency (does Vite/Rollup actually bundle it, does `World3dHost` mount,
  does the wasm module actually parse the DSL text at runtime, do the Playwright assertions on
  `objectCount`/`selection` pass) is **unverified**.
- Nothing about the interactive-job/action migration other agents are doing on the Rust editor is
  touched or verified by this — this is purely a TypeScript/Storybook import-path repair.
- The prebuilt `semio_puzzle.js`/`.wasm` (built 2026-09-01) predates this session's interactive-job
  migration entirely — even once rendering, this only proves the example DSL fixtures parse and
  draw against last-Tuesday's wasm, not that the current Rust source's migrated actions work.

## 6. Recommendation

Re-run the exact build command in §2 once the `stdio/step/ap214` migration (whoever owns it) either
commits its taxonomy-contract update or restores the `.stp` fixture outputs, then run the
Playwright spec as originally planned. My `World.stories.tsx` fix should not need any further
change for that — the remaining work is entirely outside this ticket's write-lock.

## Files touched

- `.storybook/stories/puzzle/3d/World.stories.tsx` — the four import fixes described in §1 (edited).
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️02/PUZZLE-3D-END-TO-END/🗑️generated/storybook-static-puzzle3d/`
  — created (empty/partial; the build never completed, see §2). Should be deleted at ticket close
  per the generated-folder rule.
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️02/PUZZLE-3D-END-TO-END/📓️storybook-repair-report.md` — this report.

Nothing under `stdio/`, `main.ts`, `scopes.ts`, or any Rust file was touched.
