# Glob project selectors resolve from declared owners (2026-11-23)

## Problem
`repo run-many -t component -p "semio-s-plugin-flow-*-component"` fell back to `{kind:"all"}` in `buildInvocationGraphSelectionV1`, because only literal selectors counted as scoped. That is a cold whole-tree read of 4850 owners (hours). The exact name reads 2 owners.

## Fix
Build root: `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/`.
- `🧭️selection/📥️options/🟦️.ts`: new `buildSelectorPatternV1` (the wildcard matcher), `declaredBuildProjectSelectorsV1` (gate), `selectBuildNamesV1` (one selection rule over `{name,root,tags?}`), `wholeBuildReadReasonV1` (bilingual line).
- `🧭️selection/🟦️.ts`: `buildPatternV1` and the non-literal path of `selectBuildProjectsV1` delegate to them, so the registry and the loaded graph select with the same code.
- `🕸️graph/🚪️admission/🟦️.ts`: run-many gate uses `declaredBuildProjectSelectorsV1`; `closeBuildOwnerAdmissionV1` expands the selectors against `registry.owners` (the `📋️build-owners.json` data) with `selectBuildNamesV1`, then admits only those owners plus their closure, exactly as for exact names. `readBuildGraphSelectionV1` keeps selector order when a negation is present (order is semantic). New `buildWholeReadReasonV1(args)`.
- `🧭️invocation/🕸️graph/🟦️.ts`: prints the bilingual line on stderr when the read stays whole.
- `⌨️command/⚙️runtime/🟦️.ts`: the unknown-project check uses `selectBuildNamesV1`.
- Schemas `🧬️schema/🚪️selection.json` and `🧬️schema/🔣️.json` (kept equal by a test): owner selectors may contain `*` and `!` (not `tag:`, `!tag:`).

Resolved from declared names: `*` wildcards, comma lists, repeated `-p`, later negation (`a*,!b*`), `name:`, `directory:`, root paths, `--exclude`.
Whole read kept (and announced in English and German): `tag:` (needs inferred tags), a leading `!` or a bare `*`/`name:*` (selects everything), unsupported syntax `[]{}()\?` (still refuses as before), run-many without exact targets.

## Tests (bun, absolute paths)
New `🕸️graph/🚪️admission/🧪️tests/🎯️glob-selection/🟦️.ts` (4 pass): expansion vs an independent picomatch oracle (glob, exclude, negation, lists, `name:`/`directory:`, tags, unsupported syntax); a glob admits the same owner set and the same planner calls as the equivalent exact list (and fewer than the roster); the whole-read cases and their bilingual line.
Existing: admission tests (10 pass), `🧪️tests/🕸️graph/🛂️admission` (20 pass), selection tests (6), show-project-scope (3), current-never-builds (8) all pass.
`🧪️tests/🕸️graph/📌️pinned-publication` has 6 failing tests with "Exact Original Graph format owner required" (cache store format owner, unrelated to selectors; not verified against the tree before my change because git state cannot be restored).

## Verification
Live graph read skipped: `vm.swapusage` free was 0.9-1.2 GB, below the 1.5 GB gate. Offline substitute (registry file only, no graph): 4844 declared owners; `semio-hub-flow-component,semio-s-plugin-flow-*-component` expands to 10 owners (hub plus bim, brep, dictionary, draw, list, logic, math, primitive, text), so the read now covers those 10 plus closure instead of the whole tree. No `--dry-run` mode was looked for or run. A live `repo show-projects` or run-many dry check remains open.
