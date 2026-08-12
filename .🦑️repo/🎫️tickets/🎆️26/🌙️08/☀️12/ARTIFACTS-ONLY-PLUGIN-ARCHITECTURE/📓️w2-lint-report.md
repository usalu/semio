# W2 — Lint layer (capability lint, layering lint, dependency-cruiser)

Scope: `PluginCapabilityLintScript` extension, `CapabilityLayeringLintScript` gate-wiring,
`.dependency-cruiser.cjs` plugin-SDK-only rule. Files disjoint from the policy agent's
repo-root `📜️script.ts`, which was **not opened or edited**.

## Correction to the census

`📓️w0-census.md`/scout reporting a nonexistent `.dependency-cruiser.cjs` was wrong — it globbed
`*dep-cruiser*`, which doesn't match the real filename `dependency-cruiser` (no hyphen-split).
The file exists at repo root, 369 lines pre-edit, ~394 post-edit.

## Task A — `PluginCapabilityLintScript` extension

File: `🧰️framework/🛍️products/💻️os/🔨️modules/🧑️‍💻️dev/📦️packages/🟦️typescript/📜️script.ts`

1. **`semio-framework-os` forbidden dependency** — added to `depRules` (~:1442, anchor
   `"semio-framework-os": "forbidden",`). Per §3.2 of `📓️w0-d-sdk-surface.md`, 17 plugin crates
   depend on the HOST crate today. Re-verified live (not just trusting the census) with
   `find ✏️s/🔌️plugins -iname Cargo.toml -exec grep -l '^semio-framework-os[[:space:]]*=' {} \; | wc -l`
   → **17**, identical set: `✒️writer, 🌀️procedural, 🌍️gis, 🎪️demonstrator, 🏭️process, 📏️layout,
   📐️cad, 🎥️shooting, 🎞️animate, 💠️lowpoly, 📸️remodel, 🗒️note, 🔱️trinity, 🖍️draw, 🖨️raster,
   🧩️puzzle, 🪐️space`. All 17 seeded into `KNOWN_CAPABILITY_VIOLATIONS` (~:1425), each with an
   inline `// 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE` annotation, in the exact
   `"<pkg>: forbidden dependency semio-framework-os"` string the lint emits.
2. **`std::env`/`std::process`** joined `std::fs`/`std::net` on the same footing (~:1499, same
   `localBackboneStorage`-gated check, single combined regex + message). This surfaced exactly one
   real hit not already covered by the pre-existing `std::fs` check:
   `semio-s-plugin-puzzle: … (✏️s/🔌️plugins/🧩️puzzle/📦️packages/🦀️rust/build.rs)` — this file
   already used `std::fs::{read_dir,copy,write}` undeclared *before* this wave (verified: puzzle's
   `Cargo.toml` has no `localBackboneStorage` capability declared), so it was already a live gate
   failure the old check should have caught; adding `std::env`/`std::process` just folds
   `build.rs`'s `std::env::var(CARGO_MANIFEST_DIR/OUT_DIR)` into the same message. Seeded as an
   18th `KNOWN_CAPABILITY_VIOLATIONS` entry with a comment explaining it predates this wave.

**`KNOWN_CAPABILITY_VIOLATIONS` semantics preserved**: still a `Set<string>` matched against the
exact failure message; grandfathered entries print as `WARN`, everything else still hard-fails.

## Task B — `CapabilityLayeringLintScript` wired into the gate

File: same `📜️script.ts`.

- Added the one real untriaged finding to `KNOWN_LAYERING_VIOLATIONS` (~:1565):
  `"semio-framework-os-renderer-wgpu: framework->plugin dependency on semio-s-plugin-puzzle"`,
  confirmed live at
  `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/Cargo.toml:30`
  (`puzzle = { path = "…", package = "semio-s-plugin-puzzle" }`). **Did not touch** the wgpu
  renderer or puzzle plugin (both outside this agent's boundary, puzzle held by another session)
  — the real fix (documented in the docstring) is deleting that unused Cargo dependency line from
  the renderer's `Cargo.toml`, once whoever owns that boundary confirms it's truly dead.
- **Wiring, without touching repo-root `📜️script.ts`**: the repo-root gate already calls
  `nx run @semio-tech/framework-os-dev:plugin lint` unconditionally (confirmed:
  `grep -n framework-os-dev 📜️script.ts` → line 681, the only such call besides `dev`/`build`).
  So the `"plugin"` router's `"lint"` subcommand (~:2657) was extended to run
  `CapabilityLayeringLintScript` immediately after `PluginCapabilityLintScript` — this folds
  layering into the SAME gate invocation the policy agent's file already makes, with zero edits to
  that file.
- The `layer-lint` nx target didn't actually exist as a runnable target — `📋️project.json` (the
  real, emoji-named project config; ASCII `project.json` doesn't exist for this package) declared
  `dev`/`build`/`test`/`verify`/`plugin`/`parity` only. Added a `layer-lint` target there mirroring
  the existing ones exactly (same `nx:run-commands` executor/cwd/env shape), so
  `bun nx run @semio-tech/framework-os-dev:layer-lint` and the router's already-correct docstring
  claim both become true.

## Task C — `.dependency-cruiser.cjs`

Added `pluginsFrameworkSdkOnlyRule()` (new function, wired into `module.exports.forbidden`,
followed the file's existing derive-don't-hardcode style with a new `FRAMEWORK_PACKAGES` const
mirroring `S_PACKAGES`'s own `scanPackageJsonFiles` pattern): `✏️s/🔌️plugins/**` may depend on the
plugin SDK (`@semio-tech/framework`, rooted at `🧰️framework/📦️packages/`) but not on any other
`🧰️framework` package (path-prefix `^🧰️framework/` plus package-name alternation for every
`@semio-tech/framework-*` name, excluding the SDK itself). **Severity `warn`**, matching the
instruction and this ticket's existing report-mode rules.

First dry run found 108 hits, all `<plugin>/📜️script.ts → …/repo-lib/📦️index.ts` — every plugin's
own build/dev script relative-importing repo-lib, the exact sanctioned pattern
`crossPackageRelativeRule` above already carves out for the identical reason. Added the same
`pathNot: "(^|/)📜️script\\.ts$"` exclusion to `from`. After that, **7 real hits** remain (not
silenced): `📐️cad`'s renderer/brepjs components reaching `…/♾️infinite/🌍️world/🎨️r3f`,
`…/🌊️flow/🫀️core/pkg` (a wasm build output), a `🧪️vitest.config.ts` reaching a UI vite-assets
helper, and three plugins' direct `🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react`
imports. Left as WARN backlog, documented in the rule's docstring — not this wave's job to fix.

## Verification — real output pasted

### `bun nx run @semio-tech/framework-os-dev:plugin lint`

**FAILS — but not because of anything in this wave's scope.** Full output:
`.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE/w2-lint-verify-pluginlint2.txt`.

```
error: plugin capability lint failed (69 issue(s), 59 plugin package(s) evaluated)
```

Breakdown of the 69 blocking issues: **59 `cross-plugin dependency on semio-s-plugin-stdio` (and a
handful of others: `process`→its 4 extensions, `sourcing`→its 3 extensions, `writer`→`trinity`,
`sequence`→4 `imperative` extensions)**, plus **10 pre-existing `forbidden dependency`** hits
(`web-sys`/`js-sys` in puzzle/sequence/trinity/layout, `wgpu`/`winit` in animate) — **zero** from
either check this wave added (confirmed by diffing against `git show HEAD:<path>` — the
cross-plugin-dependency loop and the `web-sys`/`js-sys`/`wgpu`/`winit`/`egui`/`eframe`/`libloading`/
`reqwest`/`wgpu-core` entries in `depRules` are byte-identical to HEAD; this wave touched neither).
**All 18 of this wave's own additions (17 `semio-framework-os` + 1 puzzle `build.rs`) print as
`WARN (grandfathered)` and contribute zero blocking issues** — verified by re-running immediately
after seeding and confirming the blocking count dropped from 70 → 69 (only the `std::env`/
`std::process` addition was net-new; `semio-framework-os` entries were pre-grandfathered in the same
edit before the first run).

**Root cause, per `📌️important.md`'s own cross-session protocol**: "stdio is UCAS's and is
transiently red mid-rename… no plugin-side cargo check passes for anyone right now" — UCAS's live
W2 rollout is repointing plugins at stdio subsets, which is exactly what trips
`cross-plugin dependency on semio-s-plugin-stdio` en masse. This is out of this wave's boundary
(Task A/B/C were `semio-framework-os` + `std::env`/`std::process` + the TS SDK-only rule, not the
pre-existing cross-plugin/forbidden-lib checks) and I did not attempt to grandfather someone else's
in-flight, still-changing refactor — that would be inventing unreviewed exceptions, the opposite of
what `KNOWN_CAPABILITY_VIOLATIONS`'s own docstring asks for ("do not add an entry … without the same
standard of evidence"). Reporting `blocked-churn` for this portion, per the protocol's own guidance.

### `bun nx run @semio-tech/framework-os-dev:layer-lint`

**PASSES.** Full output:
`.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE/w2-lint-verify-layerlint2.txt`.

```
[capability-layering-lint] WARN (grandfathered C2, see 📓️w5b-c2-verdict.md): semio-s-plugin-procedural: plugin->extension dependency on semio-s-plugin-flow-extension-{brep,dictionary,list,logic,math,primitive,text}
[capability-layering-lint] WARN (grandfathered C2, see 📓️w5b-c2-verdict.md): semio-framework-os-renderer-wgpu: framework->plugin dependency on semio-s-plugin-puzzle
[DEBUG] capability layering lint passed (231 cross-role edge(s) evaluated, 8 grandfathered warning(s))
 NX   Successfully ran target layer-lint for project @semio-tech/framework-os-dev
```

(Its first run failed with `Cannot find configuration for task …:layer-lint` — the `📋️project.json`
target didn't exist; added it, then it passed.)

### `bunx dependency-cruiser --config .dependency-cruiser.cjs --output-type err-long ✏️s`

**Runs; exit code 158 (unchanged by this wave's rule — see below).** Full output:
`.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE/w2-lint-verify-depcruise2.txt`.

```
x 197 dependency violations (158 errors, 39 warnings). 6675 modules, 2450 dependencies cruised.
```

dependency-cruiser's exit code equals the ERROR count only (warnings never affect it — confirmed:
the error count was 158 both before and after fixing the script.ts noise, only the warning count
changed 140→39). The 158 errors are 100% pre-existing (`not-to-unlisted` ×19, `no-circular` ×11,
`ui-no-framework-packages` ×4, `no-cross-technology-✏️s-to-🧰️framework` ×many, `no-state-outside-os`
×1, `no-core-path` ×1, etc.) — none from `plugins-framework-sdk-only` (severity `warn`, by design,
per the task's instruction not to shift the gate). `plugins-framework-sdk-only` itself contributes
exactly 7 warnings (`grep -c plugins-framework-sdk-only` on the output), all pre-existing runtime
code, none newly introduced.

## Files touched

- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑️‍💻️dev/📦️packages/🟦️typescript/📜️script.ts` — updated
  (`PluginCapabilityLintScript` depRules + std::env/std::process check + `KNOWN_CAPABILITY_VIOLATIONS`
  seeding; `KNOWN_LAYERING_VIOLATIONS` seeding + docstring; `"plugin"`/`"lint"` router entry now also
  runs `CapabilityLayeringLintScript`; `.register("layer-lint", …)` comment updated).
- `.dependency-cruiser.cjs` — updated (`FRAMEWORK_PACKAGES` const, `pluginsFrameworkSdkOnlyRule()`,
  wired into `module.exports.forbidden`).
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑️‍💻️dev/📦️packages/🟦️typescript/📋️project.json` — updated
  (added the missing `layer-lint` nx target; not in the original file list handed to this agent, but
  it's this same package's project config, the exact file CLAUDE.md says a `script.ts` command must
  be wired through, and was required for the `layer-lint` verification command to run at all).
- Nothing else. Repo-root `📜️script.ts` was **not opened**.

## `sharedFileRequests`

None — everything needed was inside this agent's assigned boundary.

## Concurrent-churn observations

- **`plugin lint`'s 69 remaining blocking issues are concurrent churn from UCAS's W2 stdio rollout**,
  not this wave's doing — see the Verification section above for the full grep-based proof
  (byte-identical pre-existing code paths, `git show HEAD:<path>` diffed).
- Re-ran `plugin lint` twice (before/after seeding the puzzle `build.rs` entry) roughly 2 minutes
  apart; the cross-plugin (59) and forbidden-lib (10) counts were IDENTICAL both times — no
  additional churn landed mid-verification.
- `git status --porcelain` on all 17 `✏️s/🔌️plugins/*/📦️packages/🦀️rust/Cargo.toml` files this wave's
  `KNOWN_CAPABILITY_VIOLATIONS` seeding depends on: clean at time of writing (re-verified right
  before this report).

## Honest pass/fail

- Task A (capability lint extension): **DONE**, verified — this wave's own additions produce zero
  blocking failures.
- Task B (layering lint gate-wiring): **DONE, PASSES** standalone (`layer-lint` target, newly added).
  Folded into `plugin lint` too, but `plugin lint` never reaches the layering step today because the
  capability step throws first on the unrelated stdio backlog above — once that backlog clears
  (UCAS's boundary, not APA's), the layering step will run as part of the same gate invocation with
  no further changes needed here.
- Task C (dependency-cruiser rule): **DONE**, runs, contributes 7 real WARN findings, zero error-count
  impact (verified before/after).
- Overall `plugin lint` (the actual `verify gate` target): **FAILS today**, but for reasons entirely
  outside this wave's three tasks and outside this agent's boundary (UCAS's in-flight stdio
  rollout + pre-existing `web-sys`/`js-sys`/`wgpu`/`winit` deps in `animate`/`layout`/`sequence`/
  `trinity`/`puzzle`, none introduced or touched by this wave). Reporting honestly rather than
  grandfathering someone else's unreviewed, still-moving backlog to force a green result.
