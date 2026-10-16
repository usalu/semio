# Result: TSC-UNBLIND (prefix `tsu`)

Executor: Sonnet, 2026-10-16 15:48 to 16:23. No sub-agents, no modifying git command, no ticket tool.

## Verdict

DONE. `tsc -p` is unblinded. The 905 syntax errors in three files are gone and type checking now runs on both reference projects. The three files fell into three different classes, so I did not apply one repair to all of them.

| Project | Before | After |
| --- | --- | --- |
| `psc` (`psc-tsconfig-after2.json`, 322 files) | 905 errors, all syntax, 0 semantic | 0 syntax errors, 2722 semantic diagnostics (880 files). 15 of them are TS1205 `isolatedModules` re-export diagnostics. |
| `sdk2` (`sdk2-tsconfig-after.json`, 256 files) | 905 errors, all syntax | 0 syntax errors, 52 semantic diagnostics |

The `psc` count is much larger than the `sdk2` count because its closure includes the whole repo library, `docklayoutstore`, the stdio and draw plugins and the mit-bestand deck. Neither project's code changed, only the roots differ.

Two environmental findings, both needed to run `tsc` at all:

1. **Heap.** With a semantic phase, `tsc -p` on the psc project exhausts Node's default 4 GB heap (exit 134, `tsu-psc-after.err`). It needs `node --max-old-space-size=12288`. The run took about 22 min under load 50 and finished with exit 2 (diagnostics present), no crash.
2. **Stale root list.** A stale `files` entry makes `tsc` stop before semantics (TS6053 is a program option diagnostic). The psc list names `🧰️framework/🛍️products/💻️os/🔨️modules/🧵️backbone/🟦️.ts`, which no longer exists (peer carve of the os kernel). The sdk2 list names `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/💡️inference/🧪️tests/🎚️config/🟦️.ts`, which no longer exists either. I dropped both entries in `tsu-tsconfig-*-live.json` only. The original tsconfig files are untouched.

## The three files

Paths below are shortened. Prefix: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/<A>/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🟦️.ts`.

### 1. `puzzle/🧊️3d` diff (482 errors) and 2. `puzzle/🖐️5d` diff (422 errors)

- **Syntax error.** Template literals such as `` `${${at}.position}[${index}]` `` (TS1005/TS1128/TS1434/TS1443/TS1351 cascade from the first one). 7 sites in 3d (position 1, direction 1, origin 3, orientation 2) and 6 sites in 5d (position 1, direction 1, origin 2, orientation 2).
- **Evidence.** The file at `251fa724c4a` (2026-10-08 15:31) has no `${${` at all and writes `` `${at}.position[${index}]` ``. The corrupt form first appears in `da607013f32` (2026-10-09 01:21, one auto-commit); both files have mtime 2026-10-08 23:21. Every other `.map((item, index) =>` path in the same files (`inserted`, `modified`, `moved`, `removed`) is intact, so it was a regex sweep that wrapped `at.<field>` inside a second `${…}` and then split off `[${index}]`. `git grep '${${'` over `✏️s` finds no other tracked file with this corruption. No generator emits this text: nothing tracked contains the template, the files are hand-maintained. Verdict: **corrupted by a sweep, 7 days old, not touched by a peer.** `git diff HEAD` was empty, so it is committed that way.
- **Repair.** Handcrafted regex restore of exactly those spans: `${${at}.<f>}[${index}]` becomes `${at}.<f>[${index}]` (`<f>` in position, direction, origin, orientation). Asserted that the count of `${${` equals the count of matches (7 and 6) and that none remains. Line counts unchanged (532 and 536). Snapshots: `🗑️generated/tsu-snap/puzzle3d-diff.ts.pre`, `puzzle5d-diff.ts.pre`. `git diff --stat HEAD`: 6+6 and 7+7 lines.
- **After.** 5d: 0 diagnostics of any kind. 3d: 0 syntax errors, 43 semantic errors (family 2 below), because the file is a half-finished binary64 migration, not just broken strings.

### 3. `✏️s/🧪️tests/🏘️plugin-composition/📏️layout-drawing/🪪️document/🟦️.ts` (1 error)

- **Syntax error.** Line 36 col 180, TS1005 `';' expected`: `… as typeof invalidEntry.dataFields.entries.inserted[0].row;`. A `typeof` query cannot be followed by an index access.
- **Evidence.** Not corrupted. The identical line exists in the original at `0c3e78944af` (2026-10-14) and `f856dff0813`, in `…/📏️layout/…/🧬️schema/🧪️tests/🪪️document/🟦️.ts` (33), and came along with the move by `RESULT-SMALL-CROSS-EDGES.md` (`36fbe9da786`). It is a latent type syntax error that bun's transpiler tolerates. The file was last touched at 14:50:59 today by the package-specifier codemod, which only rewrote import lines (`git diff HEAD` shows just four import lines). `RESULT-PACKAGE-SPECIFIER-CODEMOD.md` is DONE and its last run ended 15:39. I waited until the mtime was older than one hour before editing (age 3672 s when edited).
- **Repair.** One anchored, unique replacement to the equivalent valid type: `as (typeof invalidEntry.dataFields.entries.inserted)[0]["row"];`. Snapshot: `tsu-snap/layout-document.ts.pre`. 69 lines before and after, 1 line changed. After: 0 diagnostics.

No file was left as peer-mid-edit; none was generated.

## After: the real type-error families (psc, 2722 diagnostics)

Log: `🗑️generated/tsu-psc-live.txt` (command below). Counts are diagnostics, not files.

| # | Family | Count | Where / owner | Notes |
| --- | --- | --- | --- | --- |
| 1 | Repo library (`semio-repo` `📚️library`) test and closure typing | 668 in `🧰️framework/🛍️products/🦑️repo/🔨️modules`, mostly `⚡️caching` 167, `🏗️build` 165, `🧪️tests` 137, `🔍️discovery` 51, `🖥️server/🎛️coordinator` 38, `🗂️workspaces` 33 | repo product owners (the build/cache/discovery slices) | (a) `.mjs` modules imported without declarations (TS7016, about 40, e.g. `../../🟨️.mjs`); (b) `{}` or `Record<string,string>` passed where `NodeJS.ProcessEnv` requires `NODE_ENV` (TS2741/TS2769, e.g. `📚️library/🟦️.ts` 19 diagnostics, `⚡️caching/🧪️tests/🎚️build-mode` 22); (c) `unknown` from `cacheInternals.*` (TS18046); (d) a duplicate identifier `scriptCompilerAdmissionControlV1` (TS2300) in `🔍️discovery/🕸️runtime/🧪️tests/🟦️.ts`; (e) fixture object literals no longer assignable to the library schema types (TS2345, TS2322, TS2353). |
| 2 | Binary64 transport migration half-done | 43 in puzzle 3d diff, 226 in `🖍️draw` (210 in `🖍️drawing/🏅️standards/🔖️1`, 16 in `🔨️modules/🏠️host`) | puzzle, draw | 3d diff: `puzzlePuzzle3dDiffGuardBinary64` was added in `da607013f32` but `Binary64`/`parseBinary64` are not imported (it is imported the right way in architect: `import { parseBinary64, type Binary64 } from "@semio-tech/framework/value/ieee754"`), `puzzlePuzzle3dDiffGuardNumber` was removed while about 34 call sites still use it, and the `Puzzle3d*Patch` interfaces still say `number` while `3d/…/🔣️.json` already refers to `Binary64Transport` and the 3d `🧬️schema/🟦️.ts` has no `Binary64` yet. Needs a coherent migration of the 3d schema and diff together, not a patch. Draw: `{ width: Binary64; … }` against `number` in fixtures and exporters, and `[x, y, z, w]` tuples requiring 4 elements. |
| 3 | Unresolvable packages | 153 TS2307, 67 TS7016 | root workspace linking, not source | `@/lib/persistence`, `@/lib/authentication`, `@/lib/event` (mit-bestand app alias, 24); `@semio-tech/cad-js*` (about 20), `@semio-tech/stdio-{binary,ifc,zip,ply,pdf,las,dwg}`, `@semio-tech/puzzle-{wasm,2d}`, `@semio-tech/lowpoly-lowpoly`, `@semio-tech/mit-bestand-praesentation-projektetage[/spec]` (12), `@xmldom/xmldom` (7, draw svg tests), `brepjs`, `tree-kill`; `graphlib` and `yargs-parser` and `jsdom` have no types. `node_modules/@semio-tech` holds 95 packages but not these, so they need the owner workspaces' `bun install` or a `paths` entry in the generated tsconfig. This inflates families 4 and 5 (TS2347, TS7006, TS18046 follow from `any`). |
| 4 | Implicit any and unknown cascades | 310 TS7006, 197 TS18046, 24 TS2315, 19 TS2347 | mostly consequences of families 1 and 3 | `docklayoutstore` test: 79 (`EventId` used as a type but it is a value, TS2749 plus TS2347 on a call with type arguments); mit-bestand `projektetage-deck` test: 137. |
| 5 | Test or implementation drift (property missing, arity) | 254 TS2339, 46 TS2554, 92 TS2741, 43 TS2353, 33 TS2305 | each file's owner | `workspace-contract` test: `parseExtensionCargoManifest` and others not exported (115, 586); `🪵️sourcing/🗂️curation` sqlite snapshot test: `curationSnapshotToSqliteDatabase` not exported (24); `build/🕸️graph` test: `alias`, target kind literals (77); `flow/🧩️extensions/🕸️wasm` unit test: `retainedRequest`, nullable `oracle.owner` (24); print product tests (`📓️print/🧪️tests`) 56, option objects with unknown properties (TS2353). |
| 6 | Re-export type without `export type` (TS1205) | 15 | stdio `🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🧬️schema/🟦️.ts` lines 4 to 6 | easy and local; the `isolatedModules` rule in the root tsconfig. |
| 7 | Module namespace and type export gaps | 37 TS2694 (36 in stdio) | stdio | namespace `X` has no exported member Y. |

Top areas by diagnostic count: repo library (668), stdio plugin (368), draw plugin (226), mit-bestand deck (157), docklayoutstore test (79), puzzle 3d diff (43), plugin-host process artifacts (70), os renderer engine (63), hub (62), print tests (56), `dev/🗄️stdio` (36).

The sdk2 closure (52 diagnostics, `tsu-sdk2-live.txt`): TS2345 15, TS2769 14, TS2339 14, TS18047 5, TS2542/TS2540 3, TS7016 1 (`graphlib`). 29 in the draw artifacts, 10 in the stdio registry, 3 each in stdio artifacts, architect and procedural. This matches the baseline categories of `RESULT-SDK-BOUNDARY-2.md` §8.2 (bun test typings of `toEqual` and `Vector3Tuple`, missing `graphlib` types, `SnapshotValue` narrowing).

## Commands run (all from `/Users/ueli/Documents/semio`)

1. Repro: `perl -e 'alarm 900; exec @ARGV' -- node node_modules/typescript/bin/tsc -p 🗑️generated/tsu-tsconfig-psc.json --noEmit --pretty false` (a copy of `psc-tsconfig-after2.json`): 905 error lines, 482 + 422 + 1, exit 2, 84 s. Log `tsu-psc-before.txt`.
2. Edits: python with asserted counts, as above. Pre-edit snapshots in `tsu-snap/`.
3. After, default heap: `tsc -p tsu-tsconfig-psc.json …`: crash, exit 134 (OOM at 4 GB). `tsu-psc-after.err`.
4. After, 12 GB heap, stale list: `node --max-old-space-size=12288 …`: one error TS6053 (stale file), no semantics. `tsu-psc-after8g.txt`.
5. After, live list: `perl -e 'alarm 2400; exec @ARGV' -- node --max-old-space-size=12288 node_modules/typescript/bin/tsc -p 🗑️generated/tsu-tsconfig-psc-live.json --noEmit --pretty false`: 2722 diagnostics, exit 2, 1322 s (15:56 to 16:18). Log `tsu-psc-live.txt`.
6. After, sdk2 project: `🗑️generated/tsu-run-sdk2.sh` (same command with `tsu-tsconfig-sdk2-live.json`, detached): 52 diagnostics, exit 2, 204 s. Log `tsu-sdk2-live.txt`. The sdk2 "before" was not re-run; its 905 baseline is in `sdk2-tsc-before.txt`.

Read-only git: `git log -3`, `git diff HEAD`, `git show <rev>:<path>`, `git grep`, `git status`. No debug logs were added.

## Hand-offs for follow-up slices

- **Binary64 owners (puzzle, draw):** finish the migration as one change per artifact (schema JSON, `🧬️schema/🟦️.ts`, diff guards, fixtures). Do not re-introduce `GuardNumber` for fields the JSON declares as `Binary64Transport`.
- **Workspace linking owner:** decide whether the generated tsconfig for `tsc` should add the missing workspace packages through `paths`, or whether `bun install` per workspace is the intended route (family 3), then re-measure families 4 and 5, which should shrink a lot.
- **Repo library owner:** `ProcessEnv` fixtures (`NODE_ENV`), `.mjs` declarations and the duplicate `scriptCompilerAdmissionControlV1`.
- **Stale tsconfig lists:** `psc-tsconfig-after2.json` (os `🧵️backbone/🟦️.ts`) and `sdk2-tsconfig-after.json` (gis `🧪️tests/🎚️config/🟦️.ts`). Regenerate before the next `tsc -p` comparison.
- **Foreign (not mine):** the 2722 diagnostics are not caused by my edits; the three files were the only syntax blockers. Compare future runs with `tsu-psc-live.txt` as the first honest semantic baseline.
