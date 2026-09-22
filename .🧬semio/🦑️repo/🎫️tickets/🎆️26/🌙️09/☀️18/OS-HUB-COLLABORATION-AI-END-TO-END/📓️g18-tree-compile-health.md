# G18 — Tree compile health (native crates + os product TypeScript/vitest)

Auditor G18, 2026-09-22, ~11:00–12:15 CEST. Scope: measure compile health of the tree the four
outcomes depend on, while several peers edit concurrently. No source edits, no servers, no
sub-agents, one cargo process at a time, `CARGO_INCREMENTAL=0` +
`CARGO_TARGET_DIR=.🧬semio/🦑️repo/⚡️cache/cargo/target-g18`, captures in `🗑️generated/g18-*.txt`
(tail ≤300 lines, `grep -c '^error'` recorded per file). At session start `git status --short` was
435 lines (per the harness's stale snapshot); by the time this report was written auto-commit had
swept most of it (see §Auto-commit recency) and live churn was 136 lines — both counts are real,
just taken at different instants, which is itself evidence of how fast this tree moves under G18.

## Crate results

| # | crate | cmd | errors | warnings (own crate) | reached? | first error | likely owner |
|---|---|---|---|---|---|---|---|
| 1 | `semio-hub` | `check -p semio-hub` | 0 | 41 (31 lib + 10 bin) | yes (`Checking semio-hub` at line 1430, `.txt`) | — | green |
| 2 | `semio-framework-plugin` | `check -p semio-framework-plugin` (no `component-app-assembly`: package doesn't declare it) | 0 | 48 | yes | — | green, **but see §Ranked — this default-feature check does not reach the `component-guest` code path that every real plugin forces on; it missed a live break (below)** |
| 3 | `semio-framework-plugin-host` | `check -p semio-framework-plugin-host` | 0 | 0 (own) | fresh/cached from #1's dependency graph (no new `Checking` line for it this run — already type-checked when hub pulled it in at 11:03–11:05, 0 errors then too) | — | green |
| 4 | `semio-framework-os-kernel` | `check -p semio-framework-os-kernel` | 0 | 4 | yes | — | green |
| 5 | `semio-framework-os-mcp` | `check -p semio-framework-os-mcp` | 0 | 15 | yes | — | green |
| 6 | `semio-framework-server` | `check -p semio-framework-server` | 0 | 0 (own; dep `semio-framework-trace` 1) | yes | — | green (matches H2's "server crate 73/73" claim in status.md) |
| 7 | `semio-framework-replication` | `check -p semio-framework-replication` | 0 | 0 | yes | — | green (matches H2's "274/274") |
| 8a | `semio-s-plugin-space` | `check -p semio-s-plugin-space` (feature flag doesn't exist on the plugin crate itself — see note) | **7→0** | 2 | yes | see §Ranked #1 (transient, self-resolved) | peer editing `🔌️plugin/⚛️reactor/🧵️executor/🦀️.rs`, 11:08–11:24 |
| 8b | `semio-s-plugin-note` | `check -p semio-s-plugin-note` | **7→0** | 3 | yes | same as 8a | same |
| 8c | `semio-s-plugin-gis` | `check -p semio-s-plugin-gis` | **6→0** | — | yes | same as 8a | same |
| 8d | `semio-s-plugin-draw` | `check -p semio-s-plugin-draw` | **7→0** | 5 | yes | same as 8a | same |
| 8e | `semio-s-plugin-layout` | `check -p semio-s-plugin-layout` | 0 | 1 | yes | — | green (ran after the 11:24 fix) |
| 8f | `semio-s-plugin-playbook` | `check -p semio-s-plugin-playbook` | 0 | — | yes | — | green |
| 8g | `semio-s-plugin-norm` | `check -p semio-s-plugin-norm` | 0 | — | yes | — | green |
| 8h | `semio-s-plugin-stdio` | `check -p semio-s-plugin-stdio --features component-app-assembly` (this one DOES declare the feature) | 0 | 4 | yes | — | green |
| 8i | `semio-s-plugin-wfc` | `check -p semio-s-plugin-wfc` | 0 | 4+1 (2 artifact crates) | yes | — | green |
| 8j | `semio-s-plugin-puzzle` | `check -p semio-s-plugin-puzzle` | 0 | 5+3 | yes | — | green |

**Feature-flag note**: `--features component-app-assembly` only exists as a real Cargo feature on
the *artifact* crates (`semio-s-artifact-*`) and on `semio-s-plugin-stdio` itself. For the other
nine plugin crates the feature doesn't exist on the plugin package — passing it to `cargo check -p
semio-s-plugin-<x> --features component-app-assembly` errors `does not contain this feature`
(confirmed for `space`; same would happen for the rest). It doesn't need to be passed: every
plugin's `[dependencies]` already pins its artifact crates with
`features = ["component-app-assembly"]` unconditionally, so a plain `cargo check -p
semio-s-plugin-<x>` already exercises that code path. Items 8a–8g, 8i, 8j above used the plain
form for this reason; 8h used the explicit flag since `stdio` is the one crate that actually
declares it.

**All ten plugin crates are green as of the last check in each row** (space/note/gis/draw were
each re-checked a second time after 11:24 and confirmed 0 errors; see `g18-*-recheck.txt`
captures).

## TypeScript

Verb: `📋️project.json` target `typecheck` on `@semio-tech/framework-os`
(`🧰️framework/🛍️products/💻️os/📦️packages/🟦️typescript/📋️project.json`) → `bun ./📜️script.ts
typecheck` → `bunx tsc --noEmit -p ../../tsconfig.json` (i.e. `🧰️framework/🛍️products/💻️os/tsconfig.json`).
Ran directly (`cd 🧰️framework/🛍️products/💻️os && bun x tsc -p tsconfig.json --noEmit`), same effect.

Captured `🗑️generated/g18-os-typecheck.txt`. **76 `error TS…` diagnostics, exit 1.**

Top clusters (file → count):
- `🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🎨️world3d-scene-shading/📜️script.ts` — 13 (mtime 2026-09-21 04:04, not part of today's churn — stale test-helper debt, not a live peer edit)
- `🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/📐️Canvas2dHost/🧪️tests/🖱️input-contract/🟦️.tsx` — 10 (mtime 2026-09-21 14:30, same category)
- `🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🟦️.tsx` — 5
- `🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🎨️settings-theme-publication/🟦️.tsx` — 5
- `🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/⚙️settings-general-layout/🟦️.ts` — 5
- `🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/⌨️browser-keyboard-scope/🟦️.ts` — 5
- **`🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx` — 2** (mtime 2026-09-22 12:01, `git status` shows ` M` — a peer is actively editing ShellHost right now; both errors were still present when the typecheck finished at 12:08)
- remainder: 1 each, scattered across renderer elements/tests, `🔨️modules/🌉️mcp/🟦️.ts`, `🔨️modules/🛂️manifest/🟦️.ts`, `🧰️framework/🔨️modules/🖱️ui` (`Tree`, `PresenceBar`, `🖥️host/…ordered-scroll`), one `✏️s/🔌️plugins/🏛️architect` schema-diff file (`Cannot find name 'KnowledgeRecord'/'BenchmarkRecord'`), one puzzle-2d wasm editor bridge (`Board2dWasmSession` missing `pointerCancelScreen`), and `🔨️modules/🧑‍💻dev/🏗️builder/🌐️vite/🟦️.ts` (vite config type overload mismatch).

First error in the raw run:
`♻️mit-bestand/🧺️demonstrator/🧪️tests/🧪️scheduledemonstratoridle/🟦️.ts(16,22): error TS7006:
Parameter 'callback' implicitly has an 'any' type.` — cosmetic (implicit-any in a demonstrator test
helper), not load-bearing.

The two **ShellHost** errors are the ones nearest the outcome-1 critical path (`dev s` hosting
every plugin):
- `(5139,92) TS2339: Property 'consumes' does not exist on type '{ readonly pluginId: string;
  readonly moduleUrl: string; }'`
- `(8136,73) TS2345: Argument of type 'PluginWasmHandle' is not assignable to parameter of type
  '{ readonly ephemeralSnapshot?: … }'`

Given the file is mid-edit by a peer as of report time, both are likely to move or resolve shortly
— reported as observed at 12:08, not claimed as a stable defect.

## Vitest

The os product does not wire its per-module suites through a single `vitest.workspace` +
`--project` flag; each module has its own vitest config and its own `nx`/`📜️script.ts test`
target, each invoking `vitest run --config <that module's config>` directly (see
`runVitest`/`vitestRunArguments` in `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟦️.ts`).
Registered os-product vitest projects found (`defineConfig({ test: { name: … } })`):

| project name | config |
|---|---|
| `@semio-tech/framework-os` | `🧰️framework/🛍️products/💻️os/🧪️tests/🎚️config/🟦️.ts` |
| `@semio-tech/framework-os-dev` | `🔨️modules/🧑‍💻dev/🧪️tests/🎚️config/🟦️.ts` |
| `@semio-tech/framework-os-mcp` | `🔨️modules/🌉️mcp/🧪️tests/🎚️config/🟦️.ts` |
| `@semio-tech/framework-os-shell` | `🔨️modules/🖥️shell/🧪️tests/🎚️config/🟦️.ts` |
| `@semio-tech/framework-renderer-react` | `🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/⚛️react/🧪️tests/🎚️config/🟦️.ts` |
| `@semio-tech/framework-renderer-wgpu` | `🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧪️tests/🎚️config/🟦️.ts` |
| `@semio-tech/plugin-registry` | `🔨️modules/🔌️plugin/📇️registry/🧪️tests/🎚️config/🟦️.ts` |

`🖥️host` is not a literal project name here. `ShellHost` (the React component) lives under
`@semio-tech/framework-renderer-react`; the document/session host (backbone worker,
`hubSessionFetch`, actor reservation) lives under `@semio-tech/framework-os`. Ran the latter as the
smallest set actually covering host/session behavior (`bun ./📜️script.ts test quick` from
`📦️packages/🟦️typescript`, captured `🗑️generated/g18-os-vitest-framework-os.txt`):

**6 test files, 363 tests: 5 files / 360 tests passed, 1 file / 3 tests failed.**

Failures, all in `../../🔨️modules/🏪️store/👷️worker/🟦️.ts > backbone-worker offline resilience`:
1. `browser document first open rejects hostile assets and retired owners before socket
   authority` — `AssertionError: foreign-plan-scope: expected [] to deeply equal [ 'open-plan',
   'manifest' ]`
2. `browser document actor reservation activates only after an exact current socket Session` —
   `Error: session activation test deadline`
3. `browser document actor transfers one verified cold pair only after lifecycle ACK and exact
   page receipts` — `Error: hub session rebootstrap required` (thrown from `hubSessionFetch`)

All three assert directly against outcome-3 machinery (browser session/document hosting over the
hub). Not independently re-run against `HEAD` to separate "real defect" from "flaky under fleet
load ≈ 100"; flagged in §Ranked as unverified-cause.

`@semio-tech/framework-renderer-react` (owns `ShellHost`, the two TS errors above) was **not**
vitest-run — out of budget; listed only, per the task's "list, then run the smallest set" framing.

## Uncommitted churn buckets

`git status --short | awk '{print $2}' | cut -d/ -f1-3 | sort | uniq -c | sort -rn | head -30`,
taken **after** the checks above (11:20 auto-commit already landed, live count 136 lines):

```
  42 🧰️framework/🛍️products/💻️os
  26 .🧬semio/🦑️repo/🎫️tickets
  13 ✏️s/🔌️plugins/🗄️stdio
   9 ✏️s/🔌️plugins/🗒️note
   7 🧰️framework/🛍️products/🦑️repo
   6 ✏️s/🔌️plugins/➗️mathematical
   5 ✏️s/🔌️plugins/🀄️wfc
   3 ✏️s/🔌️plugins/🧩️puzzle
   3 ✏️s/🔌️plugins/🕸️dag
   3 ✏️s/🔌️plugins/📜️imperative
   2 ✏️s/🔌️plugins/💡️reasoning
   2 ✏️s/🔌️plugins/🌍️gis
   1 🏢️semio-tech/🎡️play/🧪️tests
   1 🌎️hub/🧫️fixtures/🧱️foundation-source
   1 🌎️hub/🧪️tests/🧱️foundation-source
   1 🌎️hub/🏗️bootstrap/🦀️.rs
   1 🌎️hub/README.md
   1 ✏️s/🔌️plugins/🏛️architect
   1 ✏️s/🔌️plugins/🎞️animate
```
plus one line each for `.windsurf/`, `.vscode/`, `.semio/events.semio`, `.mcp.json`, `.kiro/`,
`.cursor/`, `.codex/config.toml` (editor/agent config churn, not source).

At session start (harness's `git status` snapshot, 435 lines) the two biggest buckets were
`.🧬semio/🦑️repo/🎫️tickets` (89) and `✏️s/🔌️plugins/🀄️wfc` (81), followed by
`🧰️framework/🛍️products/💻️os` (69) and `🧰️framework/🔨️modules/🖱️ui` (55) — wfc and the ticket
folders were swept hardest by the 11:20 auto-commit; `🧰️framework/🛍️products/💻️os` is still the
largest live bucket, consistent with the peer edits caught mid-flight above (ShellHost, the
`🔌️plugin/⚛️reactor/🧵️executor` break).

## Auto-commit recency

`git log -1 --format=%cd`: **`Tue Sep 22 11:20:08 2026 +0200`** (commit `f2585a4fdb`). Previous:
`Mon Sep 21 21:41:07 2026 +0200` (`ef2210a418`, was HEAD at session start ~13.5 h earlier) and
`Mon Sep 21 14:38:46 2026 +0200` (`50c97b2051`). Cadence is irregular (13.5 h gap then landed
mid-session) — per memory `feedback-auto-commit-message-date-is-fake`, the commit *message* date is
frozen/unreliable; `%cd` (committer date) used here instead and is trustworthy.

## Ranked breaks blocking a running slice

1. **[RESOLVED DURING THIS RUN, 11:08→11:24, ~16 min window] `semio-framework-plugin`'s
   `component-guest`-gated code path was broken for every real plugin.** File
   `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🧵️executor/🦀️.rs`: a peer's
   uncommitted, staged edit (`git diff --cached`) removed `#[cfg(test)]` from `TaskReservation`,
   its `Drop` impl, `reserve()` and `detach()` (un-gating them for production use — most likely
   related to the ongoing instance-close task-cancellation work status.md's FP9/FP10 slices
   describe: "instance-close cancellation DROPS") but **left `#[cfg(test)] reserved: bool,` on the
   `TaskSlot` struct field itself** (line ~351) while the now-ungated code unconditionally reads
   `.reserved` in 5 places (lines 475–584). Result: any crate reached through `component-guest`
   (every one of the ten `s` plugin crates — `space`/`note`/`gis`/`draw` were checked in this
   window and hit `E0609`/`E0560`/`E0027` × 6–7 each) failed to compile. `layout` onward (checked
   after 11:24) never saw it. Re-checking `space`/`note`/`gis`/`draw` after 11:24 confirmed 0
   errors — **the peer landed the missing half of their own edit and it is fixed as of this
   report.** Flagged this high because it demonstrates a live blind spot: `cargo check -p
   semio-framework-plugin` with default features (item 2 in the crate table) **never exercises
   this path and would have reported green the whole time** — the only checks that would have
   caught it are the ones that build a real plugin. Owner: whoever is mid-edit on
   `⚛️reactor/🧵️executor/🦀️.rs` (git-blame/author unavailable — uncommitted staged change, no
   commit author to read).

2. **[LIVE, unresolved as of 12:08] Two `ShellHost` TypeScript errors** on outcome-1's critical
   path (`dev s` hosting every plugin) in
   `🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx` — `Property 'consumes' does
   not exist on type '{ readonly pluginId: string; readonly moduleUrl: string; }'` (5139,92) and a
   `PluginWasmHandle` vs `{ ephemeralSnapshot? }` mismatch (8136,73). File is mid-edit by a peer
   (`git status`: ` M`, mtime 12:01, 7 min before this typecheck captured them) — likely the same
   plugin-loading work as #1 given both touch plugin-handle/module shapes. Not re-verified after
   report-writing; may already be fixed.

3. **[LIVE, unresolved, cause unverified] 3/363 vitest failures in `@semio-tech/framework-os`**,
   all in `backbone-worker offline resilience` (browser document open / actor reservation / cold
   pair transfer) — directly exercises outcome-3 (browser collaboration over the hub): a
   foreign-plan-scope assertion returning `[]` instead of `['open-plan','manifest']`, a "session
   activation test deadline", and a "hub session rebootstrap required" thrown from
   `hubSessionFetch`. Could be a real regression or load-sensitivity (fleet load was ≈90–120
   during this run per `📓️status.md`'s own load readings) — not re-run to disambiguate, flagged
   for whoever owns `🏪️store/👷️worker` or the hub session bootstrap path to re-run in isolation.

4. **74 other TypeScript diagnostics**, none touching the four outcomes' hot paths directly — two
   file clusters (`world3d-scene-shading/📜️script.ts` ×13, `Canvas2dHost/…/input-contract/🟦️.tsx`
   ×10) are >1 day stale test-helper debt, not live churn; the rest are single-diagnostic
   scattered pre-existing debt (consistent with the ticket's known ~833-diagnostic TypeScript
   backlog per `📓️audit-typescript-debt.md`). Lowest priority of the four.

## Native crates: clean bill

All 7 non-plugin crates in scope (`semio-hub`, `semio-framework-plugin`,
`semio-framework-plugin-host`, `semio-framework-os-kernel`, `semio-framework-os-mcp`,
`semio-framework-server`, `semio-framework-replication`) and all 10 `s` plugin crates
(`space`/`note`/`gis`/`draw`/`layout`/`playbook`/`norm`/`stdio`/`wfc`/`puzzle`) compile with **0
errors** as of the last check on each (some needed a recheck after item #1 above self-resolved).
Warning counts are non-zero everywhere (4–48 per crate) but none block a build.
