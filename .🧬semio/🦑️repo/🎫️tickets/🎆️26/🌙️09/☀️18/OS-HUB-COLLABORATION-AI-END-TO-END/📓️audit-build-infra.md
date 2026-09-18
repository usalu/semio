# Build / Workspace / Concurrency Infra Audit — 2026-09-18

Read-only audit. No source files edited, no modifying git commands run, no `🗑️generated` sweeps.
Captures: `🗑️generated/infra-metadata.json`, `infra-processes.txt`, `infra-recent-tickets.txt`,
`infra-recent-files.txt`, `infra-verify-subcommands.txt`, `infra-root-nx-targets.txt`,
`infra-percrate-check.txt`.

## 1. Cargo workspace health

**Members: 264 total** (`cargo metadata --no-deps`), grouped by top-level dir:

| dir | members |
|---|---|
| `✏️s` | 165 |
| `🧰️framework` | 98 |
| `🌎️hub` | 1 |

`.cargo/config.toml`: shared build-dir confirmed — `build-dir`/`target-dir` both under
`.🧬semio/🦑️repo/⚡️cache/cargo`, `fine-grain-locking = true`, `build-dir-new-layout = true`,
`checksum-freshness = true`, `no-embed-metadata = true`, global `rustflags = ["-Z","threads=8"]`.
Per-target overrides: `wasm32-unknown-unknown` gets a 16 MiB shadow stack (wgpu renderer debug
overflow history), `wasm32-wasip2` gets `--max-memory=536870912` (512 MiB, uniform across plugins
to avoid fingerprint churn), Linux targets use `mold`. `[env] RUST_MIN_STACK=67108864` (64 MiB) for
native test threads (deep `block_on` state machines in artifact-app boots). `rust-toolchain.toml`:
`nightly-2026-07-07`, targets `wasm32-unknown-unknown` + `wasm32-wasip2`, components
`rust-src`+`llvm-tools-preview`.

**Concurrent cargo/rustc at audit start**: yes — `pgrep -fl 'cargo|rustc'` showed 6 live processes
before the audit began, and a second sample mid-audit showed 12. Observed (via `ps`, not acted on):
a `cargo test -p semio-s-artifact-remodel-remodeling` run (test binary at 99% CPU, ~2 min elapsed —
matches ticket `26/09/06/REMODEL-PLUGIN-END-TO-END`, a synthetic-orbit-geometry diagnostic with a
Python codemod ahead of it) and a `bun … cargo dev` chain building `semio-s-plugin-procedural` /
`semio-s-artifact-stdio-semio` for `wasm32-wasip2` (matches `26/09/17/WGPU-RENDERER-REACT-PARITY`
or a stdio consumer). A later sample showed 8 parallel `rustc` invocations compiling most of the
`stdio` artifact family (`step`, `pdf`, `json`, `xml`, `obj`, `gif`, `las`, `csv`) at 12–87% CPU each
— a bulk stdio rebuild, not this ticket's work.

Per the task's own gate, the **whole-workspace `cargo check --workspace` was skipped** (processes
were live both times it was checked). Instead ran **`cargo check -p <pkg>` on 5 small representative
crates**, one per top-level dir plus two extras, sequentially (not in parallel, to avoid adding to
fine-grain-lock contention):

| package | dir | result | elapsed |
|---|---|---|---|
| `semio-framework-hash` | 🧰️framework | ✅ clean (cached) | 0s |
| `semio-s-artifact-note-note` | ✏️s | ✅ clean, 40 pre-existing warnings in `semio-framework-plugin` | 66s |
| `semio-s-artifact-mathematical-equation` | ✏️s | ❌ **2 errors** | 19s |
| `semio-hub` | 🌎️hub | ❌ **2 errors** | 12s |
| `semio-s-artifact-sequence-sequence` | ✏️s | ✅ clean | 19s |

**Two of five sampled crates fail to compile at current HEAD — both are real, committed breakage,
not transient peer-edit noise (git status is clean in both areas):**

- **`semio-hub`** (directly in this ticket's own area): `E0560` ×2 — `SpaceArtifactCreationReadyV1`
  (defined at `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🌱️space-artifact-creation-v1/🦀️.rs:161`,
  fields are `artifact_id`/`kind_id`/`artifact_schema`/`parent_dialect` — no `document_id`) is
  constructed with a stale `document_id` field at two call sites: `🌎️hub/…/🗿️artifact-authority/🌱️creation/🦀️.rs:64`
  and `…/🌱️creation/🧬️schema/🦀️.rs:249`. Looks like a caller that wasn't updated when the struct
  dropped `document_id` in favor of `artifact_id`.
- **`semio-s-artifact-mathematical-equation`**: `E0422` ×2 — `EquationCamera` unresolved at
  `➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:535,1318`, even though the type is defined at
  `…/🕸️graph/🎚️config/🧬️schema/🦀️.rs:6`. Looks like a missing `use` import in the editor file, not a
  missing type — cheap fix.

Both are P0 for any hub- or equation-touching wave: **`semio-hub` cannot build at all right now**,
which blocks this ticket's own "OS-HUB-COLLABORATION-AI" surface until fixed.

**Disk**: `df -h /` reports a misleadingly small "Used" (13 GiB) because of APFS container
accounting; the real shared number is `/System/Volumes/Data`: **113 GiB free of 926 GiB (88% used)**.
Shared cargo `build-dir` alone is **205 GiB**, `target-dir` **4.3 GiB** (cache root **217 GiB** total).
`.nx` is 340 MiB, `node_modules` 4.3 GiB. 113 GiB headroom is not huge relative to the hazard history
below (ENOSPC incidents, 40 GB/20 min incremental regrowth) — treat disk as the tightest shared
resource, not memory or CPU.

**Memory**: `hw.memsize` = 32 GiB (34359738368 bytes). `vm_stat`: ~45k pages free (16 KiB pages ≈
720 MiB), 597k active + 590k inactive, 1.03M pages compressed (≈16 GiB), heavy compression/pagein
activity (559M decompressions, 187M pageins cumulative) — the machine has been swapping/compressing
under load for a long time, consistent with ticket reports of 15–26 GB swap under fleet load.

**CPU**: 10 logical cores (`hw.ncpu`).

## 2. Nx

`bun nx show projects`: **969 projects** (dominated by generated `test-s-plugins-*` leaf projects,
one per `component.feature` case, per the new test-domain discovery model — see §5).

Root `📋️project.json` has **289 targets**. Relevant families: `dev`, `verify` + ~100
`verify-policy-breach-*`/`verify-*-ownership`/`*-document-contract` targets, `test`/`test-quick`/
`test-long`/`test-exhaustive`, `deps-*` (cargo/cpp/dotnet/go/javascript/python/tools/trunk/wasm),
`clean-taxonomy-*`, `dev-storybook-*` (8 scoped storybook dev targets), `schema-*`.
`framework-os-dev`'s own `📋️project.json`
(`🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/📋️project.json`) carries the
actual `dev`/`test`/`test-quick`/`test-exhaustive`/`test-long`/`verify`/`bench*`/`collab-e2e`/
`generate-playground-session`/`check-playground-session`/`parity` targets — no separate
`serve-*`/`activate-*` targets live there; those are generated per plugin/profile elsewhere in the
catalog (`@semio-tech/plugin-registry:generate`).

**`bun nx run workspace:dev -- s` trace** (root `📜️script.ts`):
1. `📋️project.json` `dev` target → `bun ./📜️script.ts dev` (forwards all args, `cache:false`).
2. `DevScript.run()` (line 402): segment `"s"` is not `storybook`/`storybook-static`/`multi`/`mcp`,
   and isn't a registered playground app id via `resolvePlaygroundDevApp`, so it falls through to
   `runFrameworkOsPlaygroundDev("s")` (default, line 441) — `"s"` **is** itself a playground app id
   (see the DRAW/REMODEL/etc. tickets), so in practice `resolvePlaygroundDevApp(["s"])` matches and
   dispatches there directly (line 428-431).
3. `runFrameworkOsPlaygroundDev("s", [])` (line 266): runs
   `bun nx run @semio-tech/framework-os-dev:dev -- s`, env from `frameworkOsPlaygroundDevEnv(catalog, "s", {})`.
   Per the docblock: a bare `dev s` (no `served` sub-arg) runs the **whole Nx activation chain** for
   the `s` playground variant — every selected plugin's `component-<profile>`/`materialize-<profile>`
   targets, the browser support bundle, guest fonts, engine `wasm` producers, the generated
   playground session, then `prepare` and `activate` — before Vite serves the receipt. `SEMIO_RENDERER`
   defaults to `wgpu` unless `served` is passed (which forces `react` and skips the whole chain,
   serving whatever `dist/<profile>/🔌️plugin-modules/` already holds — used to avoid contending for
   the shared Cargo lock).
4. Port: `S_OS_PORT` (react/wgpu env templates both use `{PORT}`; default port pool documented
   elsewhere in the catalog as 6012–6205, with 6300 reserved for the MCP gateway HTTP transport and
   7300+ for the bench pool).

`.nx` cache: **340 MiB**.

## 3. Verification gates (`bun ./📜️script.ts verify …`)

There is **no `--help` listing** — an unrecognized subcommand falls through to the `default` branch,
which (confirmed by running it) unconditionally executes **`runGate()`** first (dependency-cruiser
boundary scan across `🧰️framework ✏️s 🌎️hub ♻️mit-bestand`, catalog freshness, several `nx run
*:lint`/`*:check` orchestrator calls, layering, storybook-scope freshness, indexed-generated-output
policy, …) — i.e. `verify --help`, or any typo, silently runs the **expensive** full gate, not a
help screen. Confirmed this ran to completion cleanly (no lingering process afterward) — worth
knowing before any coordinator scripts pipe an unrecognized arg into `verify`.

`VerifyScript` (lines 6885–8779) dispatches ~100 subcommands. Cheap, standalone ones worth
running ad hoc (all &lt;2 min, no full-workspace cargo):

| subcommand | purpose |
|---|---|
| `verify taxonomy report\|enforce [--scope]` | walks the repo, classifies every path against the taxonomy, reports/enforces clean=bool |
| `verify taxonomy implementation report\|enforce` | filesystem walk checking implementation-leaf basenames match taxonomy expectation |
| `verify mutation-outcome-law` | 7-rule mutation-outcome/merge-policy breach scan (source-only) |
| `verify semantic-vocabulary` | scans owned repo roots for banned-vocabulary breaches |
| `verify package-purity` | Shape-V2 package-folder language purity rule |
| `verify layering [write-baseline]` | dependency-direction gate: repo-wide/framework code must not reference an "implementation area"; shrink-only baseline |
| `verify dependencies [summary\|list\|self-test\|literal-external]` | the authoritative dependency-freeze/truth audit (per project memory, `literal-external` is the canonical dep audit, red-until-zero) |
| `verify rust-warnings [args]` | validates the rust-warning target-scope vectors against `Cargo metadata` |
| ~90 × `verify <feature>-document-contract` / `*-ownership` / `*-window-config-contract` / `policy-breach-*` | per-plugin/per-subsystem: an in-process TS oracle test + a scoped `tsc --noEmit`, optionally (`… native`) a **filtered** `cargo test -p <crate> --lib <name-filter>` — each targeted at one crate/test name, so individually seconds-to-low-tens-of-seconds even though there are ~100 of them |

`verify gate` itself and `verify interactivity` (full audit, no args) are the **expensive** ones —
budget minutes, not seconds, and don't run them concurrently with active builds.

## 4. Concurrency — who else is working right now

**Ticket status.md files touched in the last 6h** (besides this one):
`26/09/05/DRAW-PLUGIN-END-TO-END`, `26/09/17/WGPU-RENDERER-REACT-PARITY`,
`26/09/18/EXTRACT-WFC-PLUGIN`, `26/09/18/LAYOUT-PDF-EXPORT-END-TO-END`,
`26/09/06/REMODEL-PLUGIN-END-TO-END`.

**Non-ticket files touched in the last 3h** cluster into clear ownership areas:
- **WGPU renderer / React parity** (`26/09/17/WGPU-RENDERER-REACT-PARITY`): all of
  `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/*` (`🖍️draw`, `⚙️engine`, `🎬️action`, `📐️flex`,
  `📌️mounted_layout`, `🧊️gpu`), `♾️infinite/🌍️world/🦀️.rs` + its journal/pointer-gesture tests,
  `📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/*` (`🧊️renderer`, `📐️surface-lane`, `🌐️browser-worker`,
  `🏠️os-host`), `🖱️ui/🧱️elements/🪟️Window/🟦️.tsx` + story, `ShellHost`, `World3dHost` fixtures.
- **WFC plugin extraction** (`26/09/18/EXTRACT-WFC-PLUGIN`): `✏️s/🔌️plugins/🀄️wfc/*` (grid2d/grid3d/
  bitmap/2d artifacts, idle-turns test), its `🔣️.json`/`.descriptor.semio`.
- **Registry/catalog regeneration** (likely a side effect of the WFC extraction):
  `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🤖️generated/*` (framework/playgrounds/
  plugins/trusted-stdio-catalog JSON + generated TS/Rust), plus rebuilt `dist/component-dev/*.wasm`
  for `flow`, `procedural`, `puzzle`.
- **Remodel** (`26/09/06/REMODEL-PLUGIN-END-TO-END`): the running `cargo test` observed in §1.
- A stdio-wide `rustc` batch (step/pdf/json/xml/obj/gif/las/csv) not obviously tied to one open
  ticket's recent-file list — likely a dependency of one of the above (stdio is a shared leaf every
  plugin links against).

**Process snapshot** (`ps aux`, filtered): ~24 separate `bun … vite --configLoader bundle`
processes are currently running (dev-preview servers accumulated across sessions, several hours of
CPU time each — consistent with project memory "Preview Servers Vanish, Use nohup": these survive
because they were started with `nohup`, not because anyone is actively watching all of them), a
handful of live `claude` CLI processes (concurrent agent sessions), one `codex` app-server, plus the
rustc/cargo/nx-daemon activity above. This is a genuinely busy shared host — treat any new dev/build
launch as landing on top of double-digit existing processes, not a clean machine.

## 5. Test infra

- `🧪️tests/🎚️config/🟦️.ts` — the **root** Vitest config, deliberately *not* an aggregator (post
  `26/08/23/END-TO-END-TESTING-REFACTOR`). Discovery is owned by the testing domain
  (`🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test`): every `**/🧪️tests/*/component.feature` becomes
  one cacheable Nx project running through its own native host. This is why `nx show projects`
  returns 969 entries — most are these generated leaf test projects, not hand-authored ones.
- `🧪️tests/🗿️artifact-runner/🟦️.ts` — thin `describe`/`it`/`expect` wrapper over Vitest for
  artifact-test suites (the per-plugin test DSL).
- `🧪️tests/🦀️rust-warnings/🟦️.ts` — validates language-neutral rust-warning target-scope vectors
  against a fixture + live `Cargo metadata` cross-check (what `verify rust-warnings` runs).
- No `pytest.ini`/dedicated pytest config found at root (only `pyproject.toml`); no
  `playwright.config*` at root. Storybook: `.storybook/` present, with 8 scoped
  `dev-storybook-*` nx targets (cad/framework/framework-hosts/framework-os/infinite/puzzle/
  puzzle-2d/puzzle-3d/styling/ui/animate).

**Quickest whole-repo smoke**: `bun nx run workspace:test-quick` (root `📋️project.json` target
`test-quick`, `cache:true`, fans out `test-quick` across every project except `workspace`/
`@semio-tech/repo-test-domain` plus that domain's own `:test-contract`). This is the closest thing
to a single "prove nothing is broken" command: it's the fast tier (no wasm/native builds implied by
the target name and the domain-generated per-`component.feature` projects), and Nx caches every leaf
project, so a **warm** run should be low-single-digit minutes; a **cold** run across 969 projects has
no observed duration in this audit (not run — would contend with the live builds in §4) and per the
hazard history in §6 should be assumed to be materially slower and disk-hungrier than "quick" implies
the first time. Did not execute it in this audit to avoid adding a 969-project fan-out on top of an
already-loaded host; recommend a coordinator run it once on a quiet host to get a real baseline.

## 6. Known infra hazards (tickets `26/09/15`–`26/09/18`)

Grepped every `📓️status.md` under `26/09/1[5-8]/*` for build-lock/OOM/swap/incremental/nx-cache
mentions; ten tickets had hits. Recurring, corroborating patterns:

- **Fine-grain-locking deadlock is real and has happened at least twice this week.**
  `26/09/18/EXTRACT-WFC-PLUGIN` §191: "~15 minutes every cargo in the machine (12 processes …) sat
  at `Blocking waiting for file lock on artifact directory` with zero rustc alive" — confirmed via
  `sample` as `prebuild_lock_exclusive`/`flock` on `⚡️cache/cargo/target/debug/.cargo-lock`, not a
  slow build. **Recovery recipe recorded there: kill your own queued `cargo`, not anyone else's —
  killing one participant broke the cycle and rustc resumed within seconds.** Diagnostic:
  `ps aux | grep "[c]argo "` + confirm `ps aux | grep -c "[r]ustc"` is 0 before touching anything.
  `26/09/16/FORMS-PLUGIN-END-TO-END` §9 hit the same thing restaging three wasm plugins at once (9–15
  min idle in `prebuild_lock_exclusive`, no rustc child) and had to kill and relaunch.
  `26/09/17/WGPU-RENDERER-REACT-PARITY` also reports the build-dir deadlocked twice under an 18-wave
  fleet, with 4 of 7 runs SIGKILLed by peer sweeps.
- **Disk (ENOSPC) is the most frequently-tripped limit, not memory.**
  `26/09/16/FEM-3D-INTERACTIVE-FEATURE-COMPLETE` §22: disk filled mid-build (135 GB debug + 147 GB
  wasm32-wasip2 caches). `26/09/16/INPUT-CAUSALITY-LEDGER` §33: shared build dir hit "No space left
  on device" mid-materialize; pruned ~42 GB of *idle* debug/wasm32-unknown-unknown/wasm32-wasip2
  incremental caches, explicitly left the peer's *live* `wasm-dev` incremental alone.
  `26/09/16/FEM-2D-INTERACTIVE-FEATURE-COMPLETE` §20: hit ENOSPC at 102 GB incremental cache, deleted
  the whole `debug/incremental` dir; later re-ran with `CARGO_INCREMENTAL=0` and watched it **regrow
  to 40 GB in 20 minutes** anyway. `26/09/17/PUZZLE-2D-5D-FEATURE-PARITY-WITH-3D` §21: disk hit **0
  GiB free** when the nohup disk guard died; pruned 132 GB of unlocked incremental sessions to
  recover 98 GiB, restarted the guard with a 40 GiB floor. `26/09/18/EXTRACT-WFC-PLUGIN` §98: volume
  hit 100% (4.8 GiB free, shared build dir at 323 GB) but self-recovered to 24 GiB without pruning
  (every incremental session was &lt;45 min old / live fleet work, correctly left alone).
- **Host OOM/swap under fleet load** is real but secondary to disk.
  `26/09/16/FEM-3D-INTERACTIVE-FEATURE-COMPLETE` §25: two headless probe runs lost the page to host
  OOM at 20–26 GB swap under peer fleet load. `26/09/17/PUZZLE-2D-5D-FEATURE-PARITY-WITH-3D` §12:
  load 75, swap 15.2/16 GB. `26/09/17/WGPU-RENDERER-REACT-PARITY` §22: load ~35, swap 4.3/5.1 GB,
  several cargo runs SIGKILLed.
- **Session/usage-limit kills, not just infra kills**: `26/09/17/PUZZLE-2D-5D-FEATURE-PARITY-WITH-3D`
  §21 — all 14 Opus executors killed mid-slice by the session usage limit; resumed via SendMessage
  with an explicit build gate of **&lt;3 concurrent cargo per crate**.

This audit's current live snapshot (§1: 6→12 cargo/rustc processes, 113 GiB free of 926 GiB, ~16 GiB
of compressed memory) sits well inside "normal churn" for this repo, not yet in incident territory —
but the margin to the next ENOSPC/deadlock event is not large given the history above.

## 7. Recommendations

**Safe parallelism**
- Cap concurrent cargo invocations at **≤3 per crate / ≤4–5 total** system-wide, matching the
  `26/09/17` post-incident policy — the fine-grain-locking deadlock in §6 was triggered at 12
  simultaneous cargo processes with zero rustc progress.
- Cheap crates observed this audit for smoke checks between waves: `semio-framework-hash` (&lt;1s,
  cached), `semio-s-artifact-sequence-sequence` (~19s cold), `semio-s-artifact-note-note` (~66s,
  pulls in most of the stdio family as deps). Avoid touching the `stdio` family and anything under
  `wfc`/`wgpu-renderer`/`draw`/`layout`/`remodel`/`procedural` right now — those are the areas five
  other agents are actively editing per §4.
- Treat disk, not CPU/RAM, as the binding constraint: 113 GiB free against a 217 GiB build cache that
  has hit 0 GiB free twice in the last 3 days. Before starting a new wave of native+wasm builds,
  check `df -h /System/Volumes/Data` and prune only sessions `-mmin +120` and unlocked, per the
  existing "Prune Stale Incremental Sessions On Disk Full" playbook — never touch a live peer's
  incremental dir.

**Wave-plan skeleton for a coordinator**
1. **Wave 0 (fast, read-only gates)**: `verify taxonomy report`, `verify package-purity`,
   `verify semantic-vocabulary`, `verify mutation-outcome-law`, `verify layering`,
   `verify dependencies literal-external` — all &lt;2 min, safe to run anytime, no cargo contention.
2. **Wave 1 (P0 fix)**: fix the two broken crates found in §1 (`semio-hub`'s stale `document_id`
   field at the two call sites, `semio-s-artifact-mathematical-equation`'s missing `EquationCamera`
   import) — trivial, unblocks any hub/equation work and any `cargo check --workspace` a coordinator
   might later want to run.
3. **Wave 2 (targeted feature work)**: per-crate `cargo check`/`cargo test -p <crate> --lib <filter>`
   only, ≤3 concurrent, avoiding the five actively-owned areas in §4 unless this ticket's own work
   requires touching hub/collaboration code (in which case coordinate explicitly with the
   REMODEL/WGPU/WFC/DRAW/LAYOUT-PDF agents first — they're all live right now).
4. **Wave 3 (integration)**: `bun nx run workspace:test-quick` once, alone, on a quiet host, to get
   a real warm/cold baseline duration before relying on it as a gate.
5. **Never**, mid-wave: `cargo check --workspace`, `verify gate`/`verify interactivity` (no args), or
   an unrecognized `verify <typo>` (falls through to the full gate) while builds are live.

**P0 infra fixes needed before feature work**
- `semio-hub` does not compile at HEAD (`E0560` ×2) — this ticket's own product area is blocked
  until that's fixed.
- `semio-s-artifact-mathematical-equation` does not compile at HEAD (`E0422` ×2, missing import).
- No standing safeguard currently visible against the fine-grain-locking deadlock other than
  post-hoc `sample`+kill; a pre-emptive concurrent-cargo cap (per "safe parallelism" above) is the
  cheapest mitigation available without code changes.
- The disk-guard mentioned in `26/09/17/PUZZLE-2D-5D-FEATURE-PARITY-WITH-3D` (a nohup'd pruning
  daemon with a floor) has died at least once silently; if it's meant to be relied on repo-wide it
  needs a liveness check, not just a restart-after-the-fact.
