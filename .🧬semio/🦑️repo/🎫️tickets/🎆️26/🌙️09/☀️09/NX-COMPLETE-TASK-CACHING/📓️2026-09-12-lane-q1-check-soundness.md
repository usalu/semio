# Lane Q1 — Check/Generate Soundness + P1 Policy Leftovers (2026-09-12)

Scope: apply P1's three "Needs lane P4" leftovers; find and fix the generic soundness bug class where a
cached `check-*` target skips its own project's `generate-*` producer and can no longer see the producer's
declared bytes; update the wgpu boot-cache test to the new sound contract; run the full cache-contract suite
to completion.

## Task 1 — P1's three policy leftovers

Read `DiscoverScript`/`MetricsScript` myself (not just trusted P1's report) before touching policy:

- `DiscoverScript` (`🧪️test/📜️script.ts:867`) — `discoverTestCases(this.repoRoot)` is a pure filesystem walk
  (`walkDirectories`/`readdirSync`/`existsSync`, no writes, no randomness, no network) plus a `console.log`.
  Deterministic. `test-discover` already had precise `inputs`/`outputs:[]` staged in
  `🧪️test/📋️project.json` by P1; only the `policy.json` `uncached` exact-name entry was forcing `cache:false`.
- `MetricsScript` (`🧪️test/📜️script.ts:1013`) — reads `📈️metrics.json` (JSON.parse), calls
  `readImplementationCoverage`, formats and logs; `--enforce` runs a pure threshold check over the same
  in-memory data. No writes, no network, no randomness. Deterministic given that file's current bytes.

Changes:
- `⚡️caching/🔣️policy.json` `uncached`: removed `"test-discover"` and `"test-metrics"`. `matchesCommand`'s
  prefix rule (`name === command || name.startsWith(command + "-")`) meant the single `"test-metrics"` entry
  also covered `"test-metrics-enforce"` — confirmed no separate entry was needed.
- Root `📋️project.json`: added to `test-metrics` and `test-metrics-enforce` the same `{ runtime: <sha256
  digest> }` input pattern S11 built (`outputRootInputs`) and P1's `test-report` already used, pointed at
  `.🧬semio/🦑️repo/⚡️cache/tests/reports/latest/📈️metrics.json` (`outputs: []`, no side effects).
- `@semio-tech/repo-lib:test-inventory-artifact-shards` — already `"cache": true` in the live tree (a
  concurrent lane applied this before I got to it); verified via `git diff`, left untouched.

Caveat found, not fixed (out of Task 1's literal scope: the coordinator's brief named only the
`📈️metrics.json` digest): `MetricsScript` also calls `readImplementationCoverage`, which reads
`.🧬semio/🦑️repo/📊️metrics/coverage/{rust,js,go,py,dotnet}/**` (gitignored, confirmed via
`git check-ignore`). Those directories are written by tooling entirely outside the test-domain's own runner
(`cargo llvm-cov`/vitest-coverage elsewhere), so a cache hit on `test-metrics` today does not see a coverage
directory changing independently of `metrics.json`. Flagging for whoever next touches `test-metrics`'s inputs
— same class of gap, narrower scope than what I was asked to apply here.

Verification: `nx show project workspace --json` → `test-metrics`/`test-metrics-enforce`/`test-discover` all
`cache: true`, `test-discover` shows 42 precise inputs (P1's pre-staged glob list), `test-metrics`/
`test-metrics-enforce` show the new runtime digest as their last input.

## Task 2 — Soundness bug class: check targets blind to the same-project generator they verify

### The bug, generalized

`projectInputs` (🟨️.mjs) computes one `exclusions` array from **every** target's declared `outputs` in a
project, then appends it to **every** named input bucket for that whole project (`default`, `production`,
`nativeSources`, `nativeTestSources`, `artifactSources`) — not just the producing target's own inputs. A
`check-*` target that reads a sibling `generate-*` target's output via `readFileSync` (not a static import, so
no source-closure walker sees it) therefore has that exact file excluded from its own cache key, project-wide,
regardless of whether the `check-*` target itself declares that path anywhere.

### A load-bearing finding that overturns the brief's suggested fix

The brief suggested "tracked files as explicit positive globs outside `default`'s negations — prove Nx honours
them." I built an isolated, git-initialized Nx fixture (`🗑️generated/q1/nx-fixture`, pattern from
`⚡️caching/📜️script.ts` `CacheVerifyScript`) with a `generate` target producing one **tracked** output
(`tracked-out.txt`) and one **gitignored** output (`ignored-out.txt`), and three check-style targets:

- `checkDefaultOnly`: `inputs: ["default"]` (mirrors today's real `check-browser-worker`).
- `checkFixedTracked`: `inputs: ["default", "{projectRoot}/tracked-out.txt"]` — the brief's suggested fix.
- `checkTrackedGlobOnly`: `inputs: ["{projectRoot}/tracked-out.txt"]` only, no `default` — a control.
- `checkFixedRuntime`: `inputs: ["default", { runtime: "<sha256 digest> ignored-out.txt" }]`.

Result: editing `tracked-out.txt` directly (bypassing `generate`) does **not** invalidate `checkDefaultOnly`
(stays `[local cache]`, proving the bug) **and does not invalidate `checkFixedTracked` either** — the brief's
suggested "positive glob outside `default`'s negations" does **not** work once `default` is also referenced in
the same `inputs` array; Nx unions positive patterns first and then subtracts every negation globally,
regardless of array position, so `default`'s embedded `!tracked-out.txt` still wins over the extra literal
entry. The **same** literal glob alone (`checkTrackedGlobOnly`, no `default` in the mix) correctly picks up the
edit — confirming the interaction is with `default`'s negation specifically, not glob mechanics in general.
`checkFixedRuntime`'s `{ runtime: ... }` digest **does** correctly invalidate on the same edit (`ignored-out.txt`),
immune to the fileset inclusion/exclusion machinery entirely (S11's mechanism, `outputRootInputs`), and is
therefore the only mechanism proven to work uniformly for both tracked and gitignored outputs when mixed with
`default` — exactly the situation every real hand-authored `check-*` target is in.

Full run log (fixture kept at `🗑️generated/q1/nx-fixture`, `.nx`/`.git`/`node_modules`/state files removed
after capture):
```
checkDefaultOnly:      cold 0/1 hit -> warm 1/1 hit -> edit both outputs directly -> STILL 1/1 hit (bug)
checkFixedTracked:     cold 0/1 hit -> warm 1/1 hit -> edit tracked-out.txt again -> STILL 1/1 hit (brief's fix fails)
checkTrackedGlobOnly:  cold 0/1 hit -> warm 1/1 hit -> edit tracked-out.txt -> 0/1 hit (control: glob alone works)
checkFixedRuntime:     cold 0/1 hit -> warm 1/1 hit -> edit ignored-out.txt -> 0/1 hit (correct invalidation)
                                                     -> edit unrelated source.txt -> 0/1 hit (default's own glob correctly still fires)
```

### The generic fix actually applied

New in `🟨️.mjs`:
- `resolveOutputPath(output, root, workspaceRoot)` — factored out of `projectInputs`'s own exclusion loop
  (same resolution, now shared, no behavior change there).
- `generatorOutputCouplingInputs(name, target, targets, root, workspaceRoot)` — for a target whose name matches
  `^check(?:-|$)`, appends one `{ runtime: <sha256 digest over the absolute path> }` entry (S11's
  `OUTPUT_ROOT_DIGEST_SCRIPT`) for every declared output of every **other**, same-project target whose name
  matches `^generate(?:-|$)`, deduped against any digest already present (e.g. one a generator-contract
  `checkTarget` wiring already added). Domain-neutral — no project names in the function, just the two naming
  families the task named as the detection heuristic.
- Wired as a third pass in `projectWithDefaults`, after the existing contract-`checkTarget` loop, over the
  fully resolved `normalized` targets for the project, using `declared` (raw target definitions) as the
  source of truth for sibling outputs.
- Deliberately **not** gated on `dependsOn`: every real `dependsOn` entry in this repo names its target fully
  qualified (`@project:target`), even for a same-project reference (confirmed by reading `ui-styling-tokens`,
  `ui-rs`, `ui-contract-rs`'s authored `dependsOn`), and `dependsOn` alone never feeds a task's hash regardless
  — only a paired `{ dependentTasksOutputFiles }` input does, which none of these hand-authored pairs declare.
  Skipping based on `dependsOn` would have silently reintroduced the bug for exactly the targets that already
  order-depend on their generator. The cost is one extra cheap `node -e` digest spawn per generator output per
  check target; the earlier per-generator-output digest already established this cost is acceptable
  (S11 shipped the identical pattern for the generator-contract `checkTarget` guard).

### Every affected target found and fixed

Enumerated via `SEMIO_TICKET_DIR=<ticket> bunx nx run repo:audit --skip-nx-cache` →
`🗑️generated/nx/projects.json`, structurally: every project with a `generate*` target with declared `outputs`,
crossed against every `check*` target in the same project. 238 raw structural hits included many `test-*`
targets whose coupling I could not confirm reads the specific generator's file (out of scope for a blind
name-family match — noted below, not fixed); narrowing to the `check*` family (matching the task's headline
example and the one family where "reads a sibling's declared output" is directly confirmable from the command)
left 14 real project:target pairs, **all now fixed** (verified post-fix that every one hashes every relevant
generator's output — see `q1-verify-list` in this report's evidence, reproduced from the live
`repo:audit` inventory with `ensure_ascii=False` path matching):

| project:target | reads (generator) | fix |
| --- | --- | --- |
| `@semio-tech/framework-renderer-wgpu:check-browser-worker` | `generate-browser-boot` (🚀️boot.js, tracked), `generate-frame-worker` (🎞️frame-worker.js, gitignored) | digest both |
| `@semio-tech/framework-renderer-wgpu:check-frame-worker` | same two (over-couples to boot.js too — sound, slightly imprecise; see below) | digest both |
| `@semio-tech/framework-renderer-wgpu:check` (native `cargo check`) | same two | digest both |
| `@semio-tech/assets:check-generated` | `generate-logo` | digest |
| `@semio-tech/framework-graph:check` (native) | `generate` (`🤖️generated` dir) | digest |
| `@semio-tech/framework-os-dev:check-distribution` | `generate-scale-fixture` | digest |
| `@semio-tech/framework-os-kernel:check` (native) | `generate-jco-package-adapter` | digest |
| `@semio-tech/plugin-registry:check` | `generate` (`.vscode/launch.json` + `🤖️generated`) | digest both |
| `@semio-tech/ui-contract-rs:check-wasm` | `generate` (`📜️ui-contract.ts`) | digest |
| `@semio-tech/ui-rs:check-wgpu-engine-wasm` | `generate` (`🤖️generated.rs`, `🎚️ui-axes.ts`) | digest both |
| `@semio-tech/ui-rs:check-wasm` | same two | digest both |
| `@semio-tech/ui-styling-tokens:check` (native) | `generate` (5 outputs: css/py/cs/dir/rs) | digest all 5 |
| `@semio-tech/ui-styling-tokens:check-no-px` | same 5 | digest all 5 |
| `@semio-tech/ui-styling-tokens:check-no-raw-colors` | same 5 | digest all 5 |

Imprecision noted honestly: `check-frame-worker` only reads `🎞️frame-worker.js` in its own implementation
(`CheckFrameWorkerScript` → `checkFrameWorker` only), but the generic name-family rule also digests
`generate-browser-boot`'s `🚀️boot.js` for it (every `check-*` target couples to every same-project
`generate-*`, not the one it specifically calls). This is sound (never a false hit) but not maximally precise —
an edit to `boot.js` alone will now also bust `check-frame-worker`'s cache even though it doesn't read it. A
fully precise per-target mapping would need real per-target call-graph analysis (which function each
`check-*` script actually invokes) rather than a name-family match; out of this lane's remaining budget,
flagged rather than attempted.

Real-graph proof (structural, `nx show project`/`repo:audit` inventory): all 14 rows resolve `cache: true` with
every sibling generator output present as a `{ runtime: ... }` digest naming its resolved absolute path.
`framework-renderer-wgpu:check-browser-worker`/`check-frame-worker` could not be run live end-to-end (see
"Unrelated blocker" below) — the identical mechanism was proven live in the isolated fixture instead, per the
rule "never edit real generated files to test."

### Deeper root cause, found but not fixed (flagging, not spawning — same file family, bigger blast radius)

The blanket exclusion in `projectInputs` is architecturally too broad: it's meant to stop a **producer** from
hashing its own freshly-written output (a real, separate concern P3 already fixed for
`generate-frame-worker`'s self-loop), but it's applied to every named bucket for the **whole project**, not
scoped to the producing target's own inputs. This means `nativeSources`/`nativeTestSources` (used by `build`/
`test` native targets, not just `check`) have the identical blind spot whenever a same-project generator's
output is also part of the crate's own `mod`/`include!` closure (confirmed `rustSourceFiles` **does**
structurally trace `mod`/`include!` targets, including into files that don't exist yet — but the trailing
`exclusions` list, appended to the same bucket, wins anyway per the fixture proof above). Fixing this at the
root (moving the self-exclusion onto each producing target's own inputs instead of the shared buckets) is a
repo-wide architectural change touching every project with any declared output, well beyond this lane's
remaining budget and risk tolerance for a shared, actively-edited plugin file. The 14 `check-*`/`generate-*`
pairs above are fixed on top of the existing architecture without touching it. Left as a documented follow-up,
not spawned as a background task (not a self-contained, independently-actionable slice — it needs its own
focused review of every named-input bucket's producers/consumers repo-wide).

## Task 3 — wgpu boot-cache-inputs test updated

`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🧊️wgpu-browser-boot-cache-inputs/🟦️.ts`:
flipped the authored-`cache:false` assertion to `cache: true`, and added assertions (via
`cacheInternals.generatorOutputCouplingInputs`, called directly since the raw `📋️project.json`'s
`check-browser-worker`/`check-frame-worker` don't author `inputs` themselves — those come from the plugin at
graph-construction time) that `check-browser-worker` hashes both `🚀️boot.js` and `🎞️frame-worker.js`
directly, and `check-frame-worker` hashes `🎞️frame-worker.js`. Consolidated a duplicate `cacheInternals`
import that predated this change.

## Task 4 — full cache-contract suite, run to completion

Command: `SEMIO_TICKET_DIR="<ticket>" bun "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📜️script.ts" test`

**Final result: exit 0, 78 `PASS` lines, full suite completes** (was: 16 `PASS` then an `AssertionError` at the
wgpu boot-cache assertion this lane's Task 3 fixes). Tail of the final green run:
```
[DEBUG] Native Nx inventory covers all 700 projects and 6788 targets, preserving resolved settings and existing configuration sources PASS
[DEBUG] Project inventory collected: 700 projects
[DEBUG] Component and activation contracts passed; checking editor and playground contracts
[DEBUG] Editor and playground contracts passed; checking lifecycle and compiler contracts
Could not inspect Nx descendants; stopping owned launch processes: JSON Parse error: Expected '}'
Could not inspect Nx descendants; stopping owned launch processes: Snapshot unavailable
[DEBUG] Nx coordinator preserves explicit workspace data paths and stops owned launch processes after malformed or unavailable snapshots PASS
[DEBUG] Lifecycle and compiler contracts passed; checking source discovery and cancellation
[cache-contract] schema, graph ownership, source-byte discovery, native dependency oracles and materializer cancellation passed
```
(the two "Could not inspect Nx descendants" lines are the test's own fault-injection output, asserted on the
next line — not a failure.)

### Failures found while unblocking the run, none caused by this lane's Tasks 1–3, all documented here

Getting from "16 PASS then abort" to "78 PASS, exit 0" required passing through **four** more pre-existing,
unrelated breaks — each confirmed via `git diff`/`git log` to predate this session (or predate today's lanes
entirely) before I touched it:

1. **`♻️mit-bestand/🧺️demonstrator/📋️project.json` `test-e2e`**: authored `"cache": true`, silently forced
   `false` by `policy.json`'s exact `"test-e2e"` entry — the exact "dead authored value" class P1/P4 already
   cleaned up elsewhere in this same ticket, just never done for this file (owned by lane P1/P3's territory,
   not touched by either per their reports). `git diff` on this file was empty before my edit; the test
   assertion expecting `false` (`⚡️cache-contracts/🟦️.ts:189`) is also untouched by any diff in this session.
   Flipped `cache: true` → `false` to match reality (one line).
2. **`⚡️caching/🧫️fixtures/command-boundaries/🧫️cases.json`** — `"native Cargo producer"` (entry
   `⚡️caching/🦀️cargo/📜️script.ts`, export `buildCargoArtifacts`) fixture pinned `maximumInputs: 5`; real
   esbuild closure is 6 files (verified directly: `🗂️workspaces/🟦️.ts`, `🏃️process/🧭️routing/🟦️.ts`,
   `🏃️process/🟦️.ts`, `⚡️caching/📦️artifacts/🟦️.ts`, `⚡️caching/🟦️.ts`, and the entry itself). None of the
   6 files are part of any diff in this session — `⚡️caching/🟦️.ts` was added yesterday by the ticket's own
   coordinator (Wave A, `📓️2026-09-11-coordinator-log.md`: "New resolvers: … `⚡️caching/🟦️.ts`
   (`repoCacheDirectory`)") and this fixture was never updated to count it. Bumped `maximumInputs` to 6.
3. **`⚡️caching/🧫️fixtures/nx-bootstrap/🔣️.json` `eagerSources`** — same root cause as #2:
   `🏃️process/🌿️environment/🟦️.ts` imports `repoCacheDirectory` from `../../⚡️caching/🟦️.ts` (added in the
   same Wave A commit), and the bootstrap-closure fixture's `eagerSources` list was never updated to include
   it. Added the missing entry.
4. **`⚡️caching/🧪️tests/⚡️cache-contracts/🟦️.ts:840`** — read
   `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🧫️fixtures/🧱️binary-gate.json`, a path that no longer
   exists; `git log` shows the file was renamed `🧫️fixtures/` → `🎚️config/` in commit `9b605a4550` (already in
   `HEAD`, i.e., an already-committed, unrelated MCP-module rename that predates every lane in this ticket).
   Fixed the one path reference — genuinely unrelated to caching, but it sits inside the shared caching test
   file this ticket's lanes actively co-edit and was the only thing standing between "16 PASS" and seeing the
   rest of the suite at all; fixed in passing rather than reverted, following the same precedent P1 set for
   the `SEMIO_TEST_OUTPUT_SCOPE` regex bug ("genuinely unrelated, pre-existing bug that independently breaks
   every case … squarely in this lane's file ownership").

All four are one-line, mechanical, verified-correct-against-reality changes; none touch caching policy,
plugin logic, or cache flags beyond restoring an honest value. No background task spawned for any of them —
recorded here per the brief's instruction for unrelated failures.

### Unrelated blocker found, not fixed, no live real-target proof possible

`@semio-tech/framework-renderer-wgpu:check-browser-worker`/`check-frame-worker`, run for real
(`--skip-nx-cache`), both fail today with `🎞️frame-worker.js is stale; run the generate-frame-worker target` —
`🟦️typescript/🎞️frame-worker.js` is currently a genuinely stale, uncommitted, mid-flight file
(`git status` shows it modified) alongside `📜️script.ts` in the same directory (also modified) — almost
certainly a concurrent lane (P3's own report describes running `generate-frame-worker` live during its
verification) leaving the working tree in an inconsistent state relative to `checkFrameWorkerCarrierCensus`'s
marker requirements. Not caused by my Task 2 change (my diff never touches `frame-worker.js`,
`checkFrameWorker`, or `checkFrameWorkerCarrierCensus`); confirmed by running the identical command against
the pre-fix state too (same error). This is exactly why the live cache-hit proof for these two targets is
structural (`nx show project`/`repo:audit`) plus the isolated fixture, not a real `nx run` — Task 2's own rule
forbids editing real generated files to test, and I can't sensibly regenerate `frame-worker.js` mid-flight of
another lane's work without risking clobbering it.

## Files changed

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🔣️policy.json` — removed `test-discover`,
  `test-metrics` from `uncached`.
- `📋️project.json` (root) — `test-metrics`/`test-metrics-enforce` gained the runtime-digest input +
  `outputs: []`.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟨️.mjs` — `resolveOutputPath` (factored),
  `generatorOutputCouplingInputs` (new), wired as a third pass in `projectWithDefaults`; `cacheInternals`
  export list gained both.
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🧊️wgpu-browser-boot-cache-inputs/🟦️.ts`
  — `cache: false` → `true` assertion, new coupling-input assertions, de-duplicated `cacheInternals` import.
- `♻️mit-bestand/🧺️demonstrator/📋️project.json` — `test-e2e` `cache: true` → `false` (unrelated, fixed in
  passing, see Task 4 #1).
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧫️fixtures/command-boundaries/🧫️cases.json`
  — `maximumInputs` 5 → 6 (unrelated, Task 4 #2).
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧫️fixtures/nx-bootstrap/🔣️.json` — added the
  missing `⚡️caching/🟦️.ts` `eagerSources` entry (unrelated, Task 4 #3).
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/⚡️cache-contracts/🟦️.ts` — one stale
  fixture-path correction (unrelated, Task 4 #4).
- New (kept, scripts/config only): `🗑️generated/q1/nx-fixture/*` — the isolated Nx fixture proving the
  soundness mechanism (`.nx`/`.git`/`node_modules`/state files removed after evidence capture).

## Verification summary

- `NX_DAEMON=false bunx nx show projects` → exit 0, run after every plugin/policy edit (final run confirmed).
- `node --check 🟨️.mjs` → OK.
- `python3 -c "import json; json.load(...)"` → valid JSON for every edited `.json` file.
- Ajv validation of `policy.json` against its own `🧬️schema/🔣️.json` → valid.
- `nx show project workspace --json` → `test-metrics`/`test-metrics-enforce`/`test-discover` all `cache: true`
  with the expected inputs.
- `nx show project <p> --json` / `repo:audit` inventory (post-fix) → all 14 `check-*`/`generate-*` pairs in
  the decision table hash every relevant generator output.
- Isolated Nx fixture (`🗑️generated/q1/nx-fixture`): reproduces the bug, disproves the brief's suggested
  literal-glob fix, proves the runtime-digest fix, proves it doesn't false-invalidate on an unrelated edit.
- `SEMIO_TICKET_DIR=<ticket> bun ⚡️caching/📜️script.ts test` → **exit 0, 78 PASS**, full suite (was blocked at
  16 PASS by the wgpu assertion this lane fixes, then by four further pre-existing, unrelated breaks — all
  four fixed in passing and documented above).

## Left undone / flagged (not spawned — recorded per brief)

- `MetricsScript`'s `readImplementationCoverage` reads gitignored coverage directories not covered by the new
  `test-metrics` digest (narrower than what I was asked to apply; see Task 1 caveat).
- `check-frame-worker` over-couples to `generate-browser-boot`'s output too (sound, imprecise; see Task 2).
- The deeper architectural root cause (blanket per-project output exclusion instead of per-producer-target
  exclusion) affects native `build`/`test` targets too, repo-wide; not fixed (see Task 2 "Deeper root cause").
- Four pre-existing, unrelated suite blockers found and fixed in passing while unblocking the run (Task 4,
  items 1–4) — none are background-task-shaped (each was a one-line, immediately-obvious, already-verified
  correction), so none were spawned; listed here as the brief requests for "unrelated to caching" findings.
