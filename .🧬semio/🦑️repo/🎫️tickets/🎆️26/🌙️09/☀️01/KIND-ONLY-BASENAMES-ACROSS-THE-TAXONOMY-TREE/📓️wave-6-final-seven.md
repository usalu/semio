# 🌳️ Wave 6 — the 7 package-root/module-root TS+HTML breaches

Picks up exactly where wave-1.md's final tally left off: "6 package-root TS modules whose `🟦️.ts`
slot is now taken by the migrated entry file and which therefore need the directory remedy,
`🌐️multi.html`". The 8 `🧬️component.<domain>.<facet>.semio` packed artifacts and the 1 dotted-slug
(`.model.json`) from that same tally are OUT of this wave's scope.

Constraint carried in from the assignment (verified against the live `🔣️taxonomy.json`, not
re-derived): `packageBoundaryRules["🟦️typescript"].allowedDirectoryKindIds` is exactly
`["targets","fixtures","apps"]`, all closed/inappropriate for these files, so every package-root
file had to be HOISTED out of the package root into its owner tree, using only vocabulary already
registered in `semanticDirectoryKinds` / `semanticDirectoryMemberKinds` — nothing invented.

## Disposition of each of the 7

| # | file | disposition | evidence |
|---|---|---|---|
| 1 | `✏️s/🔌️plugins/📐️cad/📦️packages/🟦️typescript/🟦️brep-implementation.ts` | **hoisted** → `✏️s/🔌️plugins/📐️cad/⚙️engine/🧱️brepjs/🟦️.ts` | `engine` (⚙️) is a registered, parent-unrestricted kind already used directly at OTHER plugin roots (`✏️s/🔌️plugins/🏗️fem/⚙️engine`, `✏️s/🔌️plugins/🪐️space/⚙️engine`). `🧱️brepjs` is already a registered `members-of-engine` name AND already the exact facet name the file's sole external consumer (`✏️s/🔨️modules/🌐️spatial-kernel/⚙️engine/🧱️brepjs/🟦️.ts`) uses for the very same concept. Zero registry edits. |
| 2 | `✏️s/🔌️plugins/🧩️puzzle/📦️packages/🟦️typescript/🟦️board-session.ts` | **deleted** — merged into the package's own compliant `🟦️.ts` | The file was a 3-line pure re-export (`export { createPuzzleBoardSession, PUZZLE_BOARD_SESSION_FACTORIES } from ".../✏️editor/🌉️wasm/🟦️.ts"`) that existed ONLY to back `package.json`'s `"./board-session"` subpath export. Verified empirically with a throwaway `bun` package (`scratchpad/exports-test/`) that Node/Bun's `exports` resolver REJECTS a target outside the package directory — so the shim could never be deleted in favor of pointing `exports` straight at the real file. Instead the same named export was added directly to the package's existing kind-only `🟦️.ts`, the `"./board-session"` export entry was removed, and every live consumer was repointed from the subpath to the bare package specifier `@semio-tech/puzzle-js`. |
| 3 | `✏️s/🔌️plugins/🎞️animate/📦️packages/🟦️typescript/🟦️vitest.setup.ts` | **hoisted** → `✏️s/🔌️plugins/🎞️animate/🪨️tests/🟦️.ts` | `members-of-members-of-plugins` (children of an individual plugin) already registers `🪨️tests` — unused anywhere else on disk, but lexically and structurally exact for "this plugin's own test-support code," and rejecting the alternative `🔧️setup` (also registered there) because every real usage of that name is a plugin's `.setup(...)` REGISTRATION hook, a different concept from vitest jsdom polyfills. **Found the config already broken**: `🧪️vitest.config.ts`'s `setupFiles` pointed at `"./🟦️.ts"` — the package's own WASM-facade barrel, not the setup script at all (an orphaned pointer from an earlier rename). Fixed to `"../../🪨️tests/🟦️.ts"` as part of this move. |
| 4 | `🧰️framework/🛍️products/💻️os/🔨️modules/🧑️‍💻️dev/📦️packages/🟦️typescript/🟦️config-tooling.ts` | **hoisted** → `🧑️‍💻️dev/⚙️engine/🎛️config/🟦️.ts` | Same `engine` kind as #1, this time at a MODULE root (already precedented: `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine`). `🎛️config` is an already-registered `members-of-engine` name with a real existing precedent (`✏️s/🔌️plugins/🎞️animate/…/⚙️engine/🎛️config/🦀️.rs`). Zero live consumers were found repo-wide before the move (confirmed again after) — the file is "owned tooling" per its own ticket record (`OWNED-UI-AND-TOOLING-STACK/📓️p10j-…`), not dead code, so it was relocated rather than deleted. Its one relative import (`../../../../../../🔨️modules/🖱️ui/…`) resolves to the identical target from the new location too — both the old and new containing directories are 7 path segments deep from repo root, computed and verified with `os.path.normpath`, not assumed. |
| 5 | `🧰️framework/🛍️products/💻️os/📦️packages/🟦️typescript/🟦️glue.backbone-worker.ts` | **deleted** — genuinely dead | The repo's OWN captured policy audit (`.🧬semio/…/🧪️w1e-before-taxonomy-enforce.txt:10661`) already flagged this exact file as a `packaging-violation`: "language-neutral assets belong at the owner root." Repo-wide search for the package specifier `@semio-tech/framework-os/backbone-worker` found zero import statements — only comments. The one real consumer (`ShellHost/🟦️.tsx`) already imports the real owner-root file (`🧰️framework/🛍️products/💻️os/🟦️backbone-worker.ts`) by relative path, with its own docstring explaining it deliberately bypasses the subpath export. Deleted the shim, removed `package.json`'s `"./backbone-worker"` export entry, rewrote the now-stale `ShellHost` docstring. |
| 6 | `🌎️hub/🔨️modules/🛡️admin/📦️packages/🟦️typescript/🟦️vitest.setup.ts` | **already resolved** by a concurrent session before this wave started | Verified on disk: file is gone, package root now has a compliant `🟦️.ts` holding the jsdom polyfills, `🧪️vitest.config.ts`'s `setupFiles` already points at it correctly. This worked because, unlike #3, this package's `package.json` `exports["."]` uses `🟦️.tsx` (a different extension), so the `.ts` slot was never occupied. Confirmed zero residual references to the old filename. No action needed beyond verification. |
| 7 | `🧰️framework/🛍️products/💻️os/🔨️modules/🧑️‍💻️dev/🌐️multi.html` + co-located `🧩️multi.tsx` | **hoisted together** → `🧑️‍💻️dev/🧪️tests/🧪️multi-shell-harness/🌐️.html` + `…/🟦️.tsx` | `🧑️‍💻️dev` is a MODULE root, not a package root, so `packageBoundaryRules` never applied here — the module root already has an unrelated precedent for exactly this shape in the frozen fixture's planner output (`🧪️bench-web-harness.ts` → `🧪️tests/🧪️bench-web-harness/🟦️.ts`). `tests` (🧪️) is a registered, parent-unrestricted kind; `test-case` (also 🧪️) is a registered OPEN-slug kind whose `parentKindIds` includes `tests`, so `multi-shell-harness` needed no new registration — it matches `test-case`'s pattern `^[a-z0-9]+(?:-[a-z0-9]+)*$` directly. The pair moved together because the `.html` loads the `.tsx` as its module script. |

## What did NOT need inventing

No entry in `🔣️taxonomy.json` was touched. Every target reused vocabulary that was already registered
— `engine`/`members-of-engine` (#1, #4), `members-of-members-of-plugins`'s `🪨️tests` (#3), `tests` +
`test-case` (#7) — or eliminated the file outright once it was proven to be redundant packaging
plumbing with zero real consumers (#2, #5).

The `🧫️fixtures/🧪️remaining-package-purity-authority/🔣️.json` frozen fixture (never edited — a listed
HARD CONSTRAINT) turned out to be an oracle of the taxonomy tool's OWN planner output for several of
these exact files, and was read (not written) as evidence: it independently proposed the `⚙️engine`
hoist for #1 and named `⚙️config-tooling` for #4 (an UNREGISTERED slug — rejected in favor of the
already-registered `engine`/`🎛️config` pair instead, per the assignment's "do not invent a directory
kind" rule) and `configurable-tool-setup` → `🧪️tests/🧰️setup/🟦️.ts` for #3's vitest.setup shape (also
rejected: that target nests a `tests` directory INSIDE a package root, which the CURRENT
`packageBoundaryRules.allowedDirectoryKindIds` does not admit — the fixture is a historical snapshot
of the planner's output, not a live authority to satisfy over the assignment's own constraint).

## Node/Bun `exports` containment — the constraint that shaped #2 and #5

Verified empirically (not assumed) with a disposable package in the scratchpad
(`scratchpad/exports-test/`): a `package.json` `exports` target that resolves OUTSIDE the package's
own directory throws `Cannot find package` at resolution time. This is why #2 and #5 could NOT simply
have their subpath export repointed at the real out-of-package implementation file — the shim had to
either move (impossible, no compliant in-package slot exists) or be eliminated by merging its content
into the package's own already-compliant entry file (#2) or by proving it dead and deleting it
outright (#5).

## Excluded by design, not missed

`♻️mit-bestand/🧺️demonstrator/package.json` and `📦️index.tsx` still reference
`@semio-tech/puzzle-js/board-session` — per the assignment's own exclusion list (`♻️mit-bestand` is
excluded from the repointing sweep, treated as legacy). Left untouched deliberately.

## Verification

* `bun 📜️script.ts verify taxonomy` (bare) — still fails ONLY with `expected report or enforce, got
  undefined` (the defined "healthy" bar), before and after.
* All 8 old paths (7 files, #7 is 2 files) confirmed absent on disk; all 5 new paths (#2 and #5 have no
  new path, they were eliminated) confirmed present.
* Repo-wide grep (excluding node_modules/target/.nx/.🧬semio/.cursor and the frozen fixture) for every
  one of the 7 original basenames: zero residual hits outside expected historical/excluded locations.
* `bun 📜️script.ts test` run in the foreground for the 5 affected packages (`@semio-tech/puzzle-js`,
  `@semio-tech/animate-js`, `@semio-tech/framework-os-dev`, `@semio-tech/framework-os`,
  `@semio-tech/hub-admin`): every failure present is pre-existing and unrelated to these 7 files
  (a dangling `./🟦️component` import inside `🟦️backbone-worker.ts` untouched by this wave, a missing
  `🧫️fixtures/🏠️local-interaction/🔣️query.json`, a repo-wide `@semio-tech/ui-react` resolution failure
  traced to that package's OWN unrelated internal breaches, a `🧪️fixture`/`🧪️fixtures` singular/plural
  typo in `📜️script.ts`, and a cad-js WASM test that legitimately exceeds the 15s quick-tier budget).
  None of these failures mention any of the 7 files or their new locations. `@semio-tech/animate-js`
  (11 passed) and `@semio-tech/framework-os` (188 passed) both exercise the moved
  `vitest.setup.ts`/`backbone-worker` import chains without error.
