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
