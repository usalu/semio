# W5a — launch.json / package.json generation cleanup

## Scope given
Make `.vscode/launch.json` and root `package.json`'s `dev:*`/`build:*` scripts fully generated
from registry metadata instead of hand-maintained duplicates. Files in scope: `🖥️launch.ts`,
`.vscode/🧩️launch.seed.jsonc`, root `package.json` (scripts section only), registry `📜️script.ts`
(only if needed).

## What I found before changing anything

Read `🖥️launch.ts` and `.vscode/🧩️launch.seed.jsonc` fully (3035 lines). Current split:

- **Already generated, single-sourced from the registry**: dev-server **ports** for every
  playground variant. `generateLaunchJson()` takes the `PlaygroundEntry[]` catalog (parsed from
  each plugin crate's `[[package.metadata.semio.playground]]` Cargo.toml block) and substitutes
  `{PORT}` tokens into `env`/`serverReadyAction` templates. This part was not duplicated anywhere.
- **Hand-authored in the seed's `devLaunchers` table** (42 keys): `namePrefix` (the bespoke emoji
  display name, e.g. `🏛️architect`), VS Code `presentation.order`, the launch `command`, the
  `reactEnv`/`wgpuEnv` templates (which env var name carries the port — not uniform, e.g.
  `ARCHITECT_PLAY_PORT` vs `S_OS_PORT` vs `CAD_JS_RENDERER_PLAY_PORT`), and the
  `serverReadyAction` regex/uriFormat templates.
- **Hand-authored directly in the skeleton** (not devLauncher-templated at all): fixture-variant
  launchers (concrete-forest, hexagonal-column, capsule-dream, base-icon), all `🧊️wgpu🖥️native`
  entries, non-playground tooling (storybook, compose, repo CLI/MCP, gates, build/publish groups).

## Why I did not attempt full (steps 2–3) generation

`PlaygroundEntry` (in registry `📜️script.ts`) currently carries `variant`, `pluginId`,
`cratePath`, `app`, `brand`, `aliases`, `ports`, `examples`, `engines`, `assets` — no label, order,
command, or env-var-name fields. Making every playground-derived launcher fully generated would
require one of:

1. Adding `label`/`order`/`command`/`envVar`/`serverReadyAction` fields to
   `[[package.metadata.semio.playground]]` and editing all **58** playground crates' `Cargo.toml`
   files (`bun nx run @semio-tech/plugin-registry:generate` reports 58 playgrounds today) — the
   task brief explicitly calls this out as likely too large for this pass.
2. Heuristically deriving those fields from the variant slug — not reliable here: the emoji
   display names (`namePrefix`) are genuinely bespoke branding with no mechanical relationship to
   the variant id; the launch `command` mechanism varies per plugin (`bun ./📜️script.ts dev X`
   vs `bun nx run <pkg>:dev` vs `bun run dev:X`); and the port env var name is not a deterministic
   function of the variant (`S_OS_PORT` is shared by every OS-shell-hosted plugin — `draw`, `s`,
   `note`, `writer`, `remodel`, `vcs`, `aggregator`, `aussuchen`, `bearbeiten` — while
   free-standing playgrounds each get their own `<NAME>_PLAY_PORT`; guessing wrong here silently
   breaks that dev server's env wiring for every developer).

Guessing wrong on any of these fields is a correctness regression for the entire team's dev
loop, not a cosmetic one — so I took the task's documented fallback: **keep the seed's structure,
eliminate real literal duplication, and document what's left for a future pass** (that future pass
being: add `label`/`order` — and possibly `command`/`envVar` — to the playground TOML metadata
across all 58 crates, a dedicated ticket in its own right).

## What real duplication I found and removed (step 5, and its seed-side counterpart)

Root `package.json` has ~40 `dev:*` scripts of the exact shape
`"dev:X": "bun ./📜️script.ts dev <variant-args>"` — pure aliases with zero logic beyond naming.
The seed's `devLaunchers` table then re-referenced most of them one level removed, as
`"command": "bun run dev:X"`. That's a real, mechanical, safe-to-collapse duplication: two names
for the same command, chained through `bun run`.

**Before deleting anything** I grepped the whole repo (not just my 4 assigned files) for every
candidate script name, because `package.json` scripts are consumed by more than the seed. I found
a second, out-of-scope consumer: **`.claude/launch.json`** (a hand-maintained `runtimeExecutable:
"bun", runtimeArgs: ["run", "dev:X"]` launch config for the Claude Code harness, unrelated schema
to VS Code's `.vscode/launch.json`) references 29 of these same script names directly. That file
is not in my file assignment and deleting scripts it depends on would silently break it — a real
regression outside my scope, so I left every script `.claude/launch.json` touches untouched.

Cross-referencing the two consumer lists left **15 scripts** referenced only by `package.json`
itself, `.vscode/🧩️launch.seed.jsonc`, the generated `.vscode/launch.json` output, and inert
historical ticket/plan markdown (no live consumer besides the seed):

- `dev:animate`, `dev:architect`, `dev:mit-bestand:aussuchen`, `dev:mit-bestand:bearbeiten`,
  `dev:dag`, `dev:mit-bestand:generator`, `dev:gis:2d`, `dev:gis:3d`, `dev:imperative`,
  `dev:mit-bestand:koordinator`, `dev:mathematical`, `dev:procedural:2d`, `dev:reasoning:wires`,
  `dev:sequence`, `dev:mit-bestand:verfolgen`

For these 15 I:
1. Changed the seed's `devLaunchers[key].command` to call `bun ./📜️script.ts dev <args>` directly
   (the exact command the deleted package.json script used to wrap), removing the indirection.
2. Deleted the now-unused script from root `package.json`'s `scripts` block.

No `build:*` script matched the "wraps the generic catalog-resolving command for a playground
variant" pattern — every `build:*` entry calls `bun nx run <pkg>:build -- <variant>` or a
dedicated nx target directly, not a `bun ./📜️script.ts build <variant>` passthrough, so step 5
had nothing to remove there.

The remaining ~24 `dev:*` package.json scripts (`dev:block:2d/3d/5d`, `dev:fem:2d/3d`, `dev:flow`,
`dev:forms`, `dev:layout`, `dev:lowpoly`, `dev:note`, `dev:procedural:3d`, `dev:process:3d`,
`dev:puzzle:2d/3d/5d`, `dev:raster`, `dev:remodel`, `dev:shooting`, `dev:sourcing`,
`dev:trinity:jack/rewrite`, `dev:vcs`, `dev:writer`, `dev:cad`, `dev:draw`, `dev:s`,
`dev:os:multi`) were kept — they're either directly consumed by `.claude/launch.json`, or (for
`cad`/`draw`/`s`) already invoked without indirection by the seed via a different mechanism
(`bun nx run @semio-tech/framework-os-dev:dev`).

## Verification

- Ran `bun nx run @semio-tech/plugin-registry:generate` to regenerate `.vscode/launch.json` from
  the edited seed + registry catalog.
- `diff` against the pre-change `.vscode/launch.json`: the only lines that changed are the 15
  `command` fields (`bun run dev:X` → `bun ./📜️script.ts dev <args>`), each producing the
  byte-identical command the deleted script used to run. Every launcher name, order, env,
  ports, and `serverReadyAction` is unchanged — no dev-facing launcher disappeared or was
  renamed.
- Re-derived `generateLaunchJson()` in-process from the current seed + fresh
  `generatePlaygroundRegistry()` catalog and byte-compared it against the written
  `.vscode/launch.json`: **match**.
- `bun -e 'JSON.parse(...)'` confirms `package.json` is still valid JSON (91 scripts remain).
- `git status --porcelain` on my 3 touched paths shows exactly `.vscode/launch.json`,
  `.vscode/🧩️launch.seed.jsonc`, `package.json` — nothing else moved.
- Ran `bun nx run @semio-tech/plugin-registry:check` (the freshness/policy gate for this bundle).
  It exits non-zero, but the failures are pre-existing, unrelated Rust wiring/schema violations
  for other plugins (`component.rs is not declared by any #[path] in glue.rs`, `artifact "🗒️note"
  is missing 🧬️schema/`, etc.) — I touched no `.rs` files, and the in-process byte-compare above
  independently confirms the launch.json-specific piece of that check (`generateLaunchJson`
  freshness) passes. Per the shared-live-tree rule, I left these unrelated failures alone.

## Files touched
- `/Users/ueli/Documents/semio/.vscode/🧩️launch.seed.jsonc` — 15 `devLaunchers[*].command` values
  now call `bun ./📜️script.ts dev …` directly instead of routing through a `bun run dev:X`
  package.json alias.
- `/Users/ueli/Documents/semio/.vscode/launch.json` — regenerated (generated output, not hand-edited).
- `/Users/ueli/Documents/semio/package.json` — 15 redundant `dev:*` scripts deleted from `scripts`.
- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🟦️typescript/📇️registry/🖥️launch.ts` — **not modified** (see rationale above; kept the existing seed+registry-ports split).
- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🟦️typescript/📇️registry/📜️script.ts` — **not modified** (no `PlaygroundEntry` schema change made; see rationale).

## Left for a future, dedicated pass
Full elimination of the `devLaunchers` seed table would need `label`/`order` (and ideally
`command`/`envVar`) fields added to `[[package.metadata.semio.playground]]` across all 58
playground crates' `Cargo.toml` — real, per-crate authoring work, not a mechanical refactor, so
it deserves its own ticket rather than a rushed derivation here.

Also out of scope but worth a future ticket: `.claude/launch.json` is itself a second
hand-maintained duplicate of the same dev-launcher catalog, in a different schema, for the Claude
Code harness. It currently pins 29 of the `dev:*` package.json scripts as load-bearing, which is
exactly why this pass could only remove 15 of the ~40 total. Regenerating `.claude/launch.json`
from the same registry catalog (or from `.vscode/launch.json` itself) would let the remaining
`dev:*` package.json aliases be collapsed too.
