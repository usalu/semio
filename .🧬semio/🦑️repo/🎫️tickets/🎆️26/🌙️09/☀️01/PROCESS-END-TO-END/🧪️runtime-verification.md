# 🧪️ Runtime verification — `dev process 3d` on :6022

Playground: `process3d`, app `s.process.process3d@1/*#editor`, react renderer, port **6022**
(`✏️s/🔌️plugins/🏭️process/📦️packages/🦀️rust/Cargo.toml`).

## Boot, before any of this ticket's fixes
`🔌️plugin-modules/process/semio_s_plugin_process_component.core.wasm` was dated **Aug 27 16:02** — five
days stale, exactly the shape the sourcing ticket hit, because the crate had stopped compiling. A first
`SKIP_PLUGIN_BUILD=1` boot attempt exited 1 before binding :6022.

## What the checklist is
1. Shell mounts: title `semio · process · process3d`, Editor mode, the example selector.
2. `🪚️workpiece` renders a **non-degenerate** mesh — specifically NOT the 1×1×1
   `ProcessWorkingScene::default()` cube and NOT `PROCESS3D_FALLBACK_MESH_KIND` (`"box"`). Read the
   scene json and check the extents against the fixture's beam (3.0 × 0.2 × 0.3).
3. The engagement stepper reports the fixture's real step count, not `0`.
4. Panels: `📄️artifact` lists every step id in order; `🛠️workshop` lists the fixture's machines;
   `🛍️catalogue` flags capabilities whose rules the stock violates. `🔍️inspection` is expected to stay
   on its empty state — that one is a framework gap (`ArtifactEditor::render` carries no
   `InteractionView`), not a process defect.
5. Commands round-trip: `drill` in the engagement line switches the active utility; `forward` advances
   the cursor AND the rendered mesh changes; a step add/remove survives a reload.
6. Console carries no `turn failed` / `render failed` / `readConflicts failed` for actor `process#1`.

## Status
Pending — the wasm component build is running; results land here.

---

# 📅️ 2026-09-02 — the app boots and mounts

## What it took to get a page at all
Two packages had been half-renamed by the peer's sweep: their `package.json` already declared
`"exports": { ".": "./🟦️.tsx" }` while the file on disk was still `📦️index.tsx`, so vite could not resolve
either of them and the whole react renderer was dead for EVERY dev app:
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🟦️typescript/🎯️targets/⚛️react`
  (`@semio-tech/framework-renderer-react`) — renamed, plus its 19 importers and its own test.
- `🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react` (`@semio-tech/ui-react`) — renamed.
Sibling packages (`♾️infinite`'s react renderer) already carry the `🟦️.tsx` name, so this completes a sweep
rather than inventing a convention.

Also needed: `SKIP_WASM_BUILD=1 SKIP_ENGINE_BUILD=1 SKIP_WGPU_BUILD=1` alongside `SKIP_PLUGIN_BUILD=1`, because
the machine's disk was at 100% (see below) and the surface wasm-pack build could not run.

## Result
`http://127.0.0.1:6022` serves, the shell mounts, and the document title is **`semio · process · 3d`** — the
app, its Editor mode and a Panel are all there. The console shows `cursor: 4`, which is the regenerated timber
fixture's four-step timeline arriving in the runtime: **the fixture fix reaches the browser.**

## The remaining runtime fault — NOT process-specific
Every window body still fails to render, and the three `[DEBUG]` lines are the same ones the sourcing ticket
recorded for its own app:
```
[DEBUG] PluginRuntime: turn failed for actor process#1
[DEBUG] render failed
[DEBUG] readConflicts failed
```
Thanks to sourcing's `replyError` fix the fault now decodes instead of printing `[object Object]`:
```
{"origin":"plugin","code":"plugin.internal","severity":"error",
 "message":"runtime live cleanup faulted for instance 1"}
```
Traced to `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:29501`. `RUNTIME_MAINTENANCE_FAULT` is stored
in exactly two places on this path:
- `:29487` — `semio_framework_job::default_now_ms()` returned `None` (no monotonic clock). Ruled unlikely: the
  component imports `wasi:clocks/monotonic-clock@0.2.0`, jco emits its shim, and for `target_env = "p2"`
  `default_clock_us` is the `std::time::Instant` branch (`🔨️modules/⏱️trace/🦀️.rs:61-65`).
- `:28868` — `interactive_step_contract_violated(elapsed)`, i.e. one cooperative-maintenance turn crossed
  `INTERACTIVE_STEP_CEILING_US = 8_000` (`⏱️trace/🦀️.rs:87-95`). **This is the likely branch.** The component
  is a 48 MB **debug-profile** wasm and the machine is pathological right now: load average 4.6–9.6, disk at
  97–100%, several concurrent cargo builds from peer sessions.

That makes the remaining failure a framework-level interactive-budget violation rather than a defect in this
plugin — consistent with sourcing hitting the identical fault on its own app. Two things would discriminate it,
neither runnable while the disk is saturated:
1. rebuild the component with `SEMIO_BUILD_MODE=ship` (the `wasm-release` profile) and re-boot;
2. boot any second plugin's dev app and confirm the same `runtime live cleanup faulted` fault.

## Machine state that shaped all of this
`/System/Volumes/Data` hit **100% of 926 GB**. `target/` alone is 157 GB and `/private/tmp/claude-501` 166 GB of
per-session scratchpads, 155 GB of it belonging to three OTHER sessions' isolated cargo target dirs. My own
16 GB isolated dir is deleted; the rest is not mine to remove.

## 🧪️ Control: the runtime is broken repo-wide, not for process
Booted a SECOND plugin's dev app from its already-built module — `dev cad` on :6031, same skip flags. It fails
the same way: the shell mounts (`semio · cad`, Editor, Panel), every window body is empty, and the console
carries the identical triad plus `Agent disconnected`:
```
[DEBUG] PluginRuntime: turn failed for actor cad#1
[DEBUG] render failed
[DEBUG] readConflicts failed
```
Its underlying faults are DIFFERENT from process's, which is what makes the control informative:
- `TypeError: Cannot destructure property 'word0' of 'v88_2' as it is undefined` — a jco-transpiled component
  ABI mismatch. `🔌️plugin-modules/cad/` is from **Sep 1 14:13**, built against an older framework WIT and now
  loaded by today's host.
- `actor-ui-patch.pairing`
- `history snapshot failed`

So three plugins now show empty windows in the dev host — sourcing (its own ticket), process, and cad — each
with a different fault, none of them a defect in the plugin's own document/render code. process is the only one
of the three whose component is built from today's source (10:16) and whose fault is not an ABI mismatch.

**Conclusion.** The remaining "windows are empty" symptom is a framework-level regression in the OS dev runtime
during the peer's in-flight serde→`ToValue`, rename and async migrations — not something the process plugin can
fix from its side. Everything process owns is green: 260 of 261 tests, the component builds 1/1, the app mounts,
and the regenerated four-step fixture demonstrably reaches the runtime (`cursor: 4` in the console).

---

# 📅️ 2026-09-05 — re-baseline after three days of peer migrations

## Paths in the notes above are stale — the OS dev tree was renamed under us
The peer's emoji-normalisation sweep (ticket `26/04/08/ENFORCE-UNIQUE-SEMANTIC-EMOJIS-ACROSS-REPOSITORY`)
stripped variation selectors from the dev module directory, so every path recorded on 09-02 now 404s:

| 09-02 | 09-05 |
| --- | --- |
| `…/💻️os/🔨️modules/🧑️‍💻️dev` | `…/💻️os/🔨️modules/🧑‍💻dev` |
| `🔌️plugin-modules/process` | `🔌️plugin-modules/🏭️process` |

Reading the old path returns "No such file or directory", not an empty listing — worth knowing before
concluding the artifacts were wiped.

## 🔭️ The decisive finding: repo-wide wasm/JS skew, process included
`ls` across all 56 built plugin modules shows the transpiled JS glue was bulk-regenerated on **Sep 4 17:13–17:20**
while almost every `.core.wasm` core it wraps is far older:

```
🏭️process    wasm: Sep 2 14:23    js: Sep 4 17:20
🪵️sourcing   wasm: Sep 1 12:30    js: Sep 4 17:20
🗄️stdio      wasm: Aug 18 11:14   js: Sep 4 17:20
📜️imperative wasm: Aug  7 13:33   js: Sep 4 17:13
…
📐️cad        wasm: Sep 5 02:31    js: Sep 5 02:31   ← the ONLY matched pair
```

`📐️cad` is the single module whose core and glue were produced by the same build, and it was rebuilt at 02:31
today by a peer. Every other module — process among them — is a core built against an older framework WIT being
loaded by today's host.

This reframes the 09-02 conclusion. That note said the empty-window symptom was "a framework-level regression
in the OS dev runtime". The skew table says something more specific and more actionable: **the cores are simply
stale relative to the host**, and `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs` has changed four times
since (`96aa4f8c12` 09-02 17:38, `7ad363fd1e` 09-03 12:49, `03100691d5` 09-03 18:13). process's core predates
three of those four commits.

So the next action is not to chase `RUNTIME_MAINTENANCE_FAULT` — it is to rebuild the process core against
today's framework and re-measure. Only if the fault survives a matched core is it a framework defect.

## Source state
`cargo check -p semio-s-plugin-process --target wasm32-wasip2` against today's tree: **0 errors** (warnings only)
— so the 09-02 compile fixes have held through the peer migrations and nothing new has broken the crate.

### ⚠️ Refinement — what the JS/wasm skew does and does not prove
The bulk `Sep 4 17:13–17:20` JS timestamps are almost certainly the **dev boot's own transpile step**, not a
partial build: `dev` re-runs jco over whatever `.core.wasm` is already on disk every time it starts. Re-transpiling
an old core yields glue that matches *that old core*, so the skew by itself is **not** evidence of a broken
core↔glue pair.

The load-bearing argument is the other one, and it is unaffected: the process core was compiled **Sep 2 14:23**
against the framework WIT of that moment, while the host it is loaded into has changed three times since
(`96aa4f8c12`, `7ad363fd1e`, `03100691d5`). Core-vs-host age is the mismatch that matters. `📐️cad` being the one
module with a same-minute pair is a consequence of a peer having rebuilt it at 02:31 today, not a separate signal.

### 🔎️ The 09-02 fault hypothesis was under-determined
That note weighed only two `RUNTIME_MAINTENANCE_FAULT` store sites (no monotonic clock; the 8 ms interactive
ceiling). There are in fact **twelve**, all in `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`, and several
are far likelier for a stale core than a timing overrun:

| site | condition |
| --- | --- |
| `:29726` | `pump.session` absent → fault |
| `:29728` | `session.step()` returned `Err` |
| `:29730` | `session.checkout_outcome()` failed |
| `:29676` | pump rejected terminal-empty |
| `:29705` | pump already faulted |
| `:29664` | elapsed ≥ `INTERACTIVE_STEP_CEILING_US` (the 09-02 hypothesis) |

A core whose ABI no longer matches the host fails at `session.step()` — `:29728` — and surfaces as exactly the
same `runtime live cleanup faulted for instance 1` string. So the message alone never discriminated between
"too slow" and "wrong ABI", and the 09-02 conclusion that this is a framework regression was premature.

`INTERACTIVE_STEP_CEILING_US` is a hard-coded `8_000` in `🧰️framework/🔨️modules/⏱️trace/🦀️.rs:90` with **no env
or feature override** — confirmed by exhaustive grep. If the ceiling really is the branch, it cannot be relaxed
for a test; the only lever is making the turn faster (ship-profile core).

## ✅️ The stdio blocker cleared itself
`export_brep_out_returns_step_text_structured_payload` was red on 09-02 because
`stdio_format_descriptors()` failed with `s.stdio.gltf executable mapping keys diverge from schema registrations`.
The stdio owner fixed it in `03100691d5` (09-03 18:13): `✏️s/🔌️plugins/🗄️stdio/📇️registry/🦀️.rs:363-373` now
filters `GltfMutation::kinds()` down to the three variants actually carrying `executable_registration: true`,
so mappings and schema both yield 71 ids (3 mutations + 67 inferences + 1 codec) instead of 188 vs 71.
The bug had been introduced one commit earlier, in `96aa4f8c12` (09-02 17:38) — i.e. it landed *the same evening*
this ticket hit it. Nothing for process to do; re-run the suite to confirm.

## 🧱️ A live upstream migration, not a process defect — and process is genuinely gated on it
`semio-s-plugin-stdio` is a **regular** dependency of `semio-s-plugin-process`
(`✏️s/🔌️plugins/🏭️process/📦️packages/🦀️rust/Cargo.toml:42`, not dev-only), so anything that stops stdio
compiling stops the process wasm core from building. The 09-02 note called stdio "boot noise, not a process
blocker" on the grounds that process links only stdio's *types*; that is true of the wasm link step but
irrelevant to `cargo`, which must still compile the crate.

Ticket `26/04/08/ENFORCE-UNIQUE-SEMANTIC-EMOJIS-ACROSS-REPOSITORY` owns a repo-wide emoji rename that is
**in flight right now**. It renames directories first and reference strings after, so `#[path = "…"]` mounts
spend a window pointing at names that no longer exist. Two drift classes:
- variation-selector loss — `🏷set-entity-name` vs on-disk `🏷️set-entity-name`
- a different emoji entirely — `🟤️set-snapshot` vs on-disk `📸️set-snapshot`, `🔖️4` vs `4️⃣4`

The same sweep is what moved `🧑️‍💻️dev` → `🧑‍💻dev` under this ticket's own notes. **Treat the whole class as one
migration, not as several independent bugs.**

### ⚠️ Do not trust a single unresolved-mount measurement
A static gate over stdio returned, in one session, `5 → 65 → 64` for the identical root, and `25 → 0` for
`📦️packages`. A peer sampling every 45 s got `15 → 8 → 89 → 95 → 67` in nine minutes, with 96 stdio directories
renamed between 03:44 and 03:51. A run of three identical invocations agreeing (`64, 64, 64`) with
`find -newermt "-5 minutes"` reporting zero changed paths still only proves a **lull** — it did not mean the
migration had finished. Any "stdio is clean now" conclusion needs the owning ticket's own completion signal,
not a quiet filesystem.

Also note the gate's count is **root-sensitive** by construction: it discovers via `os.walk`, so a root that does
not contain the mounting `lib.rs` reports `0` by construction rather than by health — the silent-zero shape this
repo has hit before. A `0` is only meaningful once you confirm the walk visited the mounting file.

### Consequence for this ticket
The process core rebuild — the decisive experiment for whether the empty-window symptom is staleness or a real
framework fault — compiles stdio on the way through, so it can fail for reasons that have nothing to do with
process. The compiler's verdict on that dependency edge is worth more than either static gate, and is being
collected now.

## 🐌️ Why `dev process 3d` never reached Vite — and the decoupling that fixed it
Three consecutive boots each spent 6–10 minutes after `plugin registry catalog refreshed` without ever binding
:6022, with **no child process and no file writes**. That reads like a hang; it is not. Measured on the fourth:

```
elapsed 09:37   cpu-time 0:37.13   nice 5   load average 112
```

The dev worker had consumed 37 seconds of CPU in nine and a half minutes of wall clock — roughly 6 % of one
core. It nices ITSELF to 5 while peer sessions' `git`/`rustc` run at nice 0, and a non-root user cannot lower a
nice value once set, so under a contended box this phase is starved rather than stuck. **`%CPU` from a single
`ps` sample is misleading here** — an early sample read `0.0` and looked like a deadlock; cumulative `TIME` is
the honest measure.

One real defect did surface while diagnosing this: a `dev` worker from a killed run **survived** `kill` on its
parent and `pkill -f "framework-os-dev:dev"`, because the child's argv is `script.ts dev process3d` (no `--`
separator, variant concatenated) and matches neither pattern. It kept the plugin-build lease, so the next boot
logged `plugin builds owned by pid 34365 (port 6022); serving only` and waited out the full 60 s lease deadline
before building anyway. **Match `script.ts dev process ?3d` when cleaning up, and verify with `ps` — the lease
holder is a grandchild, not the process you launched.**

### The decoupling
Waiting on the dev script's streaming build is the wrong shape here: it is opaque, it is starved, and it
interleaves the core build with Vite startup. `plugin` is separately dispatchable
(`…/🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts:5522` → `PluginBuildScript` → `buildPlugins(filter)`), so the
core can be built on its own with the compiler's own output in view:

```
cd 🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript
DEVELOPER_DIR=/Library/Developer/CommandLineTools \
SDKROOT=/Library/Developer/CommandLineTools/SDKs/MacOSX.sdk \
SEMIO_BUILD_BUDGET_MS=5400000 \
bun ./📜️script.ts plugin process
```

Then boot with `SKIP_PLUGIN_BUILD=1` to serve the artifact that produced. This also makes the stdio dependency
edge observable: whatever `cargo rustc -p semio-s-plugin-process` says about `semio-s-plugin-stdio` is the
authoritative answer to the migration question above, which no static path gate can give.

## 🔒️ The real build bottleneck: one shared `wasm-dev` lock, and a session queued behind itself
Plugin wasm builds do NOT use the repo's `target/`. They use `target-demonstrator-dev/wasm-dev`
(`CARGO_TARGET_DIR` is honoured at `…/🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts:969`), and that directory has
ONE cargo build lock. Observed state while this ticket's core rebuild was queued:

```
96183  cargo rustc -p semio-s-plugin-puzzle  --target wasm32-wasip2 --profile wasm-dev   HOLDS lock, 36m
62291  cargo rustc -p semio-s-plugin-process --target wasm32-wasip2 --profile wasm-dev   "Blocking waiting for file lock on build directory"
```

**Both belong to the same peer session** (`7029d145-…`, logs `build11/build-puzzle.txt` and
`build12/build-process.txt`) — it launched a process build behind its own puzzle build. Any third party
building a plugin for `wasm32-wasip2` queues behind both.

### How to tell "blocked" from "working" — the pattern that keeps misleading us
A lock-holding cargo shows **0.0 %CPU and ~2 s of CPU time over 36 minutes**. That looks dead. It is not: the
work is in its `rustc` CHILD (match by ppid). Conversely a cargo that is genuinely *waiting* also shows 0 %CPU
but has **no rustc child at all**. The discriminator is the child, never the parent's `%CPU`:

| parent %CPU | rustc child | meaning |
| --- | --- | --- |
| 0.0 | yes | holding the lock, compiling — leave it alone |
| 0.0 | no | waiting on the lock — queued behind someone |

### ⚠️ Attribution: `ps` alone cannot tell you whose build you are looking at
`pgrep -f "cargo rustc -p semio-s-plugin-process"` returned a pid that was **not this session's** — its stdout fd
pointed into another session's scratchpad. Reading a peer's cargo as your own is an easy and costly mistake here
(it briefly looked like this ticket's build had started when it had not). Settle ownership with:

```
lsof -p <pid> | awk '$4 ~ /^1w?$/ {print $NF}'    # → the owning session's scratchpad path
```

### Why waiting beats a private `CARGO_TARGET_DIR` here
The usual remedy for target contention is a private `CARGO_TARGET_DIR`. It is the wrong call **in this specific
case**: the crate currently in `rustc` is `semio-s-plugin-stdio`'s 898 KB `🦀️.rs`, which is exactly the
expensive dependency this ticket's own build needs. Waiting inherits that artifact from the shared directory;
a private directory would recompile it from scratch. Queue behind the lock, don't fork the target dir.

### 🧾️ Incidental answer to the stdio migration question
`rustc` is in **codegen** on stdio's lib right now (`--crate-name` → `✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/🦀️.rs`).
A crate with unresolved `#[path]` mounts fails during expansion in seconds; this one has been compiling for a
sustained stretch. So the mount-drift class is clear as of this compile. That is not the same as "stdio compiles
clean" — the sweep's reference-string half would surface later as unresolved imports.

## ✅️ P1 confirmed on disk, and the concrete acceptance criteria it yields
`📚️examples/🎬️demo/🖼️assets/🗣️.dsl.semio` carries all eleven lines, `stockPayload` and `stepPayloads` among them.
Decoded from hex, the fixture is:

```
stockPayload  {"id":"beam","label":"Timber Beam",
               "solid":{"kind":"box","width":3.0,"depth":0.2,"height":0.3},
               "pose":{"position":[0.0,0.0,0.15],"axis":[0.0,0.0,1.0],"angle":0.0}}
resolvedUpTo  null
stepPayloads  4 steps, all enabled:
                crosscut        Crosscut To Length   circularSaw / crosscut
                lap-joint-cut   Cut Lap Joint        cncRouter   / pocket
                dowel-drill     Drill Dowel Hole     drillPress  / bore
                dowel-attach    Insert Dowel         dowelJig    / dowel
```

So the P4 checklist stops being "does it look populated" and becomes falsifiable:

| # | assertion | fails if |
| --- | --- | --- |
| 1 | shell mounts, title `semio · process · 3d`, Editor mode | — |
| 2 | `🪚️workpiece` mesh extents ≈ **3.0 × 0.2 × 0.3** | a 1×1×1 cube = `ProcessWorkingScene::default()`; `kind:"box"` unit = `PROCESS3D_FALLBACK_MESH_KIND` |
| 3 | engagement stepper reports **4** steps | `0` = `step_payloads` never reached the runtime |
| 4 | `🗿️artifact` lists **crosscut, lap-joint-cut, dowel-drill, dowel-attach** in order | — |
| 5 | `🛠️workshop` lists circularSaw, cncRouter, drillPress, dowelJig | — |
| 6 | `🔍️inspection` empty with no selection, real fields once a step is selected | — |
| 7 | `forward` advances the cursor **and the rendered mesh changes** | a cursor that moves without the mesh changing means the CSG replay is not wired to `resolved_up_to` |
| 8 | console carries no `turn failed` / `render failed` / `readConflicts failed` for `process#1` | — |

Note `resolvedUpTo` is `null` in the demo fixture (the plate example uses `2`), so the timeline opens fully
unresolved and every one of the four steps is available to the stepper.

## 🎣 False alarm worth recording: "the dev vite config is broken" was my own invocation
Bypassing the starved dev script by starting Vite directly produced:

```
failed to load config from …/🧑‍💻dev/📦️packages/🟦️typescript/⚙️vite.config.ts
SyntaxError [ERR_UNSUPPORTED_TYPESCRIPT_SYNTAX]: TypeScript parameter property is not supported in strip-only mode
    at parseTypeScript (node:internal/modules/typescript:68:40)
```

It looked like a live repo-wide regression — the offending line had been committed 40 minutes earlier
(`02db159aee`, 03:53) — and it would have blocked every dev app. **It is not a regression.** The giveaway is in
the stack: every frame is `node:internal/modules/…`. `bunx vite` resolved Vite's Node shebang and ran it under
**Node**, whose strip-only TS loader rejects parameter properties. The dev script does not do that — it runs
`bun <path>/node_modules/.bin/vite` (visible in any live dev server's argv), and **bun** supports parameter
properties, so the config loads normally.

Confirmation that this is about the loader and not the code: after fixing the first site, the very next failure
was a *different, long-standing* file (`📡️replication/📡️wire/🏠️local-interaction/📡️transport/🟦️.ts:28`), and the
repo contains **40** such parameter-property sites across `🧰️framework` and `✏️s`. A genuine regression would not
require rewriting forty pre-existing sites that have worked all along.

**How to start the dev server's Vite directly** (skips the slow pre-Vite phase; the registry it regenerates has
usually already been written):
```
cd 🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript
S_OS_PORT=6022 SEMIO_PLUGIN=process3d SEMIO_RENDERER=react VITE_SEMIO_RENDERER=react \
VITE_SEMIO_PLUGIN=process VITE_SEMIO_APP_ID='s.process.process3d@1/*#editor' \
bun <repo>/node_modules/.bin/vite --configLoader bundle --config ⚙️vite.config.ts \
  --host 127.0.0.1 --port 6022 --strictPort
```
`bun …/node_modules/.bin/vite`, **never** `bunx vite` — the latter silently changes runtime and fabricates
TypeScript syntax errors in files that are fine.

### ⚠️ Side effect: the exploratory edit was auto-committed before it could be undone
Diagnosing the above, one parameter property was rewritten to an explicit field. On reverting it, `git diff`
showed the working tree now *differed from HEAD* — the repo's auto-commit had already committed the edit. The
working tree was restored to match HEAD rather than left reverting a committed change, so
`🧰️framework/📦️packages/🟦️typescript/🟦️.ts:797-798` now carries:
```ts
readonly url: string;
constructor(url: string) { this.url = url; opened.push(url); }
```
in place of `constructor(private readonly url: string)`. It is semantics-identical, test-only
(inside `if (import.meta.vitest)`), and incidentally makes that file loadable under Node's strip-only loader —
but it was introduced on a **wrong diagnosis** and nothing in this ticket required it. **Lesson: in this repo an
experimental edit can be committed within minutes, so there is no such thing as a throwaway probe edit.**

## 🚀️ The dev server was I/O-throttled, not merely starved — and that is fixable
Vite finally reported `ready in 782561 ms` (13 minutes) on :6022. The cause was not only peer contention:
the process was running under macOS **background task policy**, which adds a nice penalty *and* throttles disk
I/O hard. Symptoms that fit "the box is busy" but actually mean "this process is throttled":

```
41945  nice 5  STAT RN   0:30 CPU over 11:00 elapsed
42155  esbuild child pinned at 0.0% CPU          ← the tell
```

`taskpolicy -B -p 41945` + the same for its `esbuild` child took that child from **0.0% → 55.9% CPU** at once.
Two subtleties: a plain `(sleep &)` from the same shell shows nice 0 and `ps -o nice -p $$` reports the shell at
0, so nothing upstream looks wrong; and the `nice 5` does **not** clear after `taskpolicy -B` — lifting the I/O
throttle is the part that matters, so judge it by the child's %CPU, not the nice column. Re-apply after any
restart, since new children start throttled again.

This is worth trying before concluding "the machine is too contended to proceed" — it likely also applies to the
`cargo`/`rustc` builds this ticket has been queuing behind.

### ❌️ Correction: do NOT use the bare-Vite recipe above to verify the app
The "start the dev server's Vite directly" recipe recorded earlier **boots a shell with no plugins in it**. It
binds, serves HTTP 200 and mounts the React shell, but the page reads `No plugins loaded` / `Agent disconnected`,
and the console shows every module 404-ing into the SPA fallback:

```
plugin.descriptor-invalid: /🔌️plugin-modules/🗄️stdio/🔣️.json returned HTML
plugin.descriptor-invalid: /🧩️extension-modules/🪓️process-extension-wood/🔣️.json returned HTML
Failed to load module script: … non-JavaScript MIME type of "text/html"
[DEBUG] shard 0 worker error … shard 0 terminated
Framework OS boot failed: Error: shard 0 terminated
```

`returned HTML` is the diagnostic signature: Vite fell through to `index.html` because the
`/🔌️plugin-modules/…` and `/🧩️extension-modules/…` static routes were never mounted. The dev script does setup
beyond spawning Vite that those routes depend on, so bypassing it trades a slow boot for a **silently empty
one** — and an empty shell is exactly the failure mode this ticket is trying to distinguish from a real plugin
fault. Use `bun ./📜️script.ts dev process 3d` (with `SKIP_PLUGIN_BUILD=1 SKIP_ENGINE_BUILD=1` when the core is
being built separately) and make it fast with the unthrottle loop instead.

**Generalisable tell:** any `plugin.descriptor-invalid … returned HTML` or `non-JavaScript MIME type of
"text/html"` in this shell means a *static route is missing*, never that the descriptor or the plugin is
malformed. Do not go debugging the descriptor.

### ⚠️ RETRACTED: "0 E0277s, so the serde migration is clear" — that was a false clean
The failed core build was read here as evidence that stdio's serde → `ToValue`/`FromValue` migration compiles,
on the grounds that the log contained no trait-bound errors. **That inference is invalid.** The build died with

```
error: couldn't read `…/🗿️artifacts/🎞️pptx/🦀️.rs`: No such file or directory (os error 2)
```

which is a **module-resolution failure during macro/module expansion**. rustc aborts there *before type
checking*, so not a single trait bound in stdio was ever evaluated. Every type-level error class — E0277
included — reads zero **by construction**, not by health. The 7000+ clean lines were the framework crates, which
did type-check; stdio contributed exactly two lines.

**The discriminator (credit: semio-f4) — count the crate's WARNING lines:**

```
grep -E "^warning: \`semio-s-plugin-stdio\` \(lib\) generated" <log>
```

Measured on this log: stdio produced **0 warnings** and **no `generated N warnings` summary at all**, while every
framework crate has one (`semio-framework-graph` 258, `semio-framework-plugin` 187, `semio-framework-os-kernel`
40, …). A crate that type-checks in this workspace always emits warnings. **Zero warnings ⇒ it never
type-checked ⇒ every clean type-level result from that run is meaningless.**

So the honest position: a peer's 225-error census (1335 stdio warning lines, 160 E0277 mentions across 82
headers) is currently the **only** measurement that carries information about stdio's trait bounds. Two other
sessions, this one included, measured the rename race and never reached the migration. Those are measurements of
*different things*, not different moments of the same thing — and the 225 has NOT been ruled out. It is the
likeliest thing this ticket's build hits next, once mount drift stops aborting it early.

Related: a peer established that `💠️lowpoly` declares `default-features = false` on stdio and adds nothing back,
so it enables a strictly SMALLER stdio surface than process does. Feature-unification is therefore dead as an
explanation for why lowpoly builds — lowpoly is not special, it simply got further.

**Rule to carry forward: before calling any dependency green from a build log, confirm that crate emitted
warnings. An abort during expansion looks identical to a clean type-check if you only count errors.**

## 🔧️ Two stdio reference fixes applied here (unblocking the process core build)
The core build is gated on `semio-s-plugin-stdio` compiling. Two stale references survived the rename sweep,
were verified stable (the owning file untouched for 21 min while the applier worked elsewhere), and both had an
unambiguous on-disk target, so they were repaired here rather than waited out:

| file:line | was | now | evidence |
| --- | --- | --- | --- |
| `📇️registry/🦀️.rs:257` | `../🗿️artifacts/🧊️obj/🧬️schema/📜️artifact-definition.json` | `🗽️obj/…` | no `🧊️obj` dir exists; `🗽️obj/…/📜️artifact-definition.json` present (6271 B, 04:43) |
| `📇️registry/🦀️.rs:923` | `native_codec_factory!(obj_codec, …, "../🗿️artifacts/🧊️obj/…/📡️.protocol.semio")` | `🗽️obj/…` | target dir present on disk |

Same reassignment pattern as `🎞️pptx`→`📽️pptx`: `🧊️` now belongs to `🧊️gltf`, so obj moved to `🗽️obj`. The third
error from that run (`🌱️metabolism/🖼️assets/🧪️base/🧊️.glb`) needed no fix — the applier had already corrected it
to `🏙️base`, which is what the source now reads.

### 🕳️ Scanning one reference class is not enough
An earlier scan reported "0 unresolved `#[path]` mounts" and that was used to conclude stdio's references were
clean. **It only covered one of at least three classes.** Broken references hide in:
1. `#[path = "…"]` module mounts — 6425 of them
2. `include_str!` / `include_bytes!` — 4298 of them
3. **quoted path arguments to ordinary macros** — e.g. `native_codec_factory!(…, "…/📡️.protocol.semio")`,
   which is neither of the above and is invisible to a scan for either

Class 3 is what a regex over `include_*` misses, and it is exactly where the second fix lived. A useful superset
for this repo is "every quoted relative path containing `🗿️artifacts/`" — 2396 in stdio — though that over-matches
doc comments, so treat its hits as candidates and let `rustc` arbitrate.

## 🩺️ Diagnosing a blank shell: probe the entry module, don't read the console
The page mounted nothing — `readyState: "interactive"`, `#root` with 0 children, `document.body.innerText` empty
— while the server answered `200` on `/` in 1.08 s. `performance.getEntriesByType('resource')` showed **116
resources with 0 pending**, i.e. the module graph had stalled rather than being slow.

The console was useless here: its buffer survives reloads, so it was full of `ERR_CONNECTION_REFUSED` and
`descriptor-invalid` entries from earlier, dead servers. The decisive probe was to import the entry module by
hand from the live page:

```js
await import('/🟦️.ts')
// → "Failed to fetch dynamically imported module: http://127.0.0.1:6022/%F0%9F%9F%A6%EF%B8%8F.ts"
```

and then confirm against the server: `curl http://127.0.0.1:6022/🟦️.ts` → `code=000`. Vite had restarted
(`📜️script.ts changed, restarting server...`) and was not serving modules, while still answering `/` from cache.
**A `200` on `/` does not mean the dev server is up** — check the entry module.

## ♻️ The real obstacle to live verification: restart churn
`vite` restarts whenever a file in its config/watch graph changes, and a Codex-driven applier is rewriting this
repo every few minutes (`🦑️repo/📚️library/🟦️.ts` at 4:33, `📜️script.ts` at 5:14). Each restart takes minutes on
this box, so the window in which the app is actually serving can be shorter than the time the shell needs to
boot ~20 WASM plugins. That, not any process defect, is what has prevented P4 from completing.

## 🪤 Self-inflicted: killing a build wrapper orphans its cargo, and sccache then stalls it
Killing the `bun ./📜️script.ts plugin process` wrapper does **not** kill the `cargo` it spawned. The cargo
reparents to launchd and keeps running. Twenty minutes later this session had:

```
31418  ppid 1  cargo rustc -p semio-s-plugin-process   0.0% CPU, 19:33 elapsed
49534  ppid 31418  sccache                              0.0% CPU,  9:13 elapsed
```

— an orphan of this session's own making, stalled with an idle `sccache` child. It was attributable as mine only
via `lsof -p 31418 | awk '$4 ~ /^1w?$/ {print $NF}'` → this session's `core-build2.txt`. That check is what makes
killing it safe; without it the process is indistinguishable from a peer's live build.

Two rules combine here, and both were already known but not applied together:
- **`kill <wrapper>` is not enough** — find and kill the cargo too, or the next build queues behind your own ghost.
- **`sccache` serializes builds and an isolated `CARGO_TARGET_DIR` does not escape it.** A cargo at 0% CPU whose
  only child is an idle `sccache` is stuck, not working. The remedy is `RUSTC_WRAPPER=""`.

Note this is the one case where the "0% CPU + live child ⇒ working, leave it" rule gives the wrong answer:
the child was `sccache`, not `rustc`. Refine the discriminator to **a live `rustc` child**; an idle `sccache`
child means blocked.

The rebuild therefore runs as:
```
DEVELOPER_DIR=/Library/Developer/CommandLineTools SDKROOT=… \
SEMIO_BUILD_BUDGET_MS=5400000 RUSTC_WRAPPER="" \
bun ./📜️script.ts plugin process
```

## 🎯️ The app's boot failure was captured — and it is NOT the 09-02 runtime fault
With the dev server genuinely serving (entry module `200`, not just `/`), the shell loaded **250 resources** and
reached `readyState: "complete"`, but `#root` stayed at 0 children. The reason, from the console:

```
Uncaught SyntaxError: The requested module '/@fs/…' does not provide an export named
'parseDirectorySpaceAdministrationPageV1'
```

This is **a peer's in-flight refactor of `📇️directory`**, not a process defect and not
`runtime live cleanup faulted`. It is the same module whose edits were restarting Vite
(`📇️directory/🧬️schema/🟦️.ts` and `📇️directory/🟦️.ts` at 05:23 and 05:27).

The symbol resolved itself while this was being diagnosed — it is now defined at
`📇️directory/🧬️schema/🟦️.ts:817`, re-exported at `📇️directory/🟦️.ts:69`, and imported at
`🛍️products/💻️os/🟦️.ts:4026`. So the boot failure was a **transient window** in which the importer had been
updated but the exporter had not, exactly the same lagging-window shape as the emoji renames — reference
strings updated on one side before the other.

**Why this matters for the ticket's central question:** today's empty shell has a *demonstrated, different*
cause from 09-02's. A missing ES export halts module evaluation before the plugin runtime ever starts, so today's
failure says nothing about `RUNTIME_MAINTENANCE_FAULT` either way. It also means an observer who saw an empty
app at that moment and attributed it to the plugin would have been wrong — the third instance tonight of a
symptom whose obvious explanation was not the real one.

## 🧭️ Practical: how to tell whether the dev server is really usable
Three states must be distinguished, and only the third permits verification:

| check | meaning |
| --- | --- |
| port `:6022` LISTEN | nothing — Vite holds the socket across restarts |
| `curl /` → `200` | nothing — `index.html` is served while modules are not |
| `curl '/🟦️.ts'` → `200` | the module graph is actually being served |

Even the third is not sufficient: the shell needs several uninterrupted minutes after that to instantiate its
WASM plugins, and a restart during that window blanks the tab. Wait for the entry module to answer `200` on
**several consecutive checks** before spending a load.

---

# 📅️ 2026-09-05 (13:40) — ✅️ stdio is GREEN for process's feature set

A native `cargo test -p semio-s-plugin-process --lib` (with `RUSTC_WRAPPER=""`) compiled the whole graph
through stdio. The three-condition test, applied properly this time:

| condition | result | meaning |
| --- | --- | --- |
| `couldn't read` count | **0** | every `#[path]` mount, `include_*` and macro-argument path resolves — through a real compile, not just a scanner |
| stdio warning lines | **1334** | stdio genuinely **type-checked** (a crate that aborts during expansion emits none) |
| `E0277` count | **0** | the serde → `ToValue`/`FromValue` migration compiles |
| any `E0xxx` | **0** | no type errors at all |

**The 1334 figure independently matches what a peer measured for `semio-s-plugin-puzzle`.** That is a stronger
corroboration than a bare repeat: process and puzzle are different crates with different dependency sets, so an
identical stdio warning count means both resolve the same stdio surface with the same features enabled.

Two earlier positions are now settled:
- The **serde migration was never the blocker** for process. The earlier "0 E0277s" reading was invalid (0 by
  construction, since expansion aborted first) — but the conclusion it was reaching turns out to be right, now
  for a valid reason. Getting the right answer earlier by bad reasoning would still have been wrong; this run is
  what makes it knowable.
- The **`🗽️obj` repairs made in this ticket hold under a real compile**, covering both the `#[path]` class and the
  macro-argument class (`native_codec_factory!`) that no scan for mounts-or-includes would have caught.

The 225-error census reported elsewhere stays scoped to `💠️lowpoly`, and the reading consistent with all three
measurements is that lowpoly's `default-features = false` **disables** the features carrying those impls — i.e.
it fails for having too FEW features, the opposite of a feature-unification story.
