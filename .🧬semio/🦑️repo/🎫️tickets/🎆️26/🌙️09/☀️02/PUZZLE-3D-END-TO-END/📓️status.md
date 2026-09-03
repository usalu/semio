# 🧩️ Puzzle 3d end to end — status

## Done and verified

Repo-level breakage that stopped `bun dev:puzzle:3d` from building at all. Each was verified by
compiling, not by inspection.

| Fix | Evidence |
|---|---|
| `semio-s-plugin-stdio` mutation-leaf ownership (43 + 120 leaf dirs moved back under their aggregates, descriptors' `owner` rewritten, mounts repointed, 4 deleted gltf facet leaves restored, 43 manifest `payloadSchema` refs repointed) | `cargo check -p semio-s-plugin-stdio` 11 errors → 0, native **and** `--target wasm32-wasip2`; downstream plugins consume its rmeta again |
| `semio-framework-graph`: stale `generated::draw_layers` → `drawing_layers` (18 refs) and 6 × `use super::dsl_core` → `use crate::dsl_core` | `cargo check -p semio-framework-graph` 26 errors → 0 |
| `semio-s-plugin-puzzle`: 15 types had `#[value(...)]` but no `value_derive::ToValue/FromValue` (half-applied migration) | `cargo check -p semio-s-plugin-puzzle` 201 errors → 0 |
| `.vscode/🧩️launch.seed.jsonc` stale launcher keys `trinity-rewrite`→`trinity-rewriting`, `procedural2d/3d`→`generation2d/3d` | `nx run @semio-tech/plugin-registry:generate` succeeds; seed↔catalog check reports 0 stale keys |
| Taxonomy `generatorContracts["external-step-assets"]` pointed at a step fixture the subset split had moved | graph build script's "Invalid taxonomy schema" gone |

Non-regression: `repo-test-domain:test-contract` reports `unsplit-artifact-subset` 0,
`wildcard-subset-owner` 0, `duplicate-mutation-owner` 0 — the peer ticket's gate wins survived the
leaf move-back. A repo-wide sweep for other half-applied `#[value(...)]` sites found none.

## The real reason puzzle3d is not end-to-end

See `📓️why-puzzle3d-is-not-end-to-end.md` and `📓️interactive-job-migration-recipe.md`.

`validate_ui_dispatch_classification` admits only `InteractiveJobClassification::Migrated`. Puzzle3d
has 6 migrated actions and 61 `BatchOnlyPendingRewrite` — including `setActiveExample` (example
switching) and `setFillCount` / `fillBuildTick` (the fill tool). Those are hard-dead at the UI.

Puzzle3d is the least-migrated app in the repo (8%); repo-wide it is 427 migrated vs 414 not, with
23 apps fully migrated — `💠️lowpoly` 48/48 and `✒️writer` are working reference implementations.

The per-route blockers are recorded in the plugin's own fixture
`✏️s/🔌️plugins/🧩️puzzle/🧪️publication-authority/🔣️.json`:

| Lanes | Routes | Blocker |
|---|---|---|
| Config | 36 (incl. `setFillCount`) | no app-owned retained preparation **and root-retirement** factory |
| Artifact+Config | 18 (incl. `setActiveExample`) | same, for the artifact lane |
| HostOnly | 6 (incl. `fillBuildTick`) | the current empty/effect-only completion does not represent the real effect — needs real reducer work |

Crucially, the job-shaped logic already exists: ~50 of the 60 actions have fully implemented, staged,
budget-checked `Work` types (`Puzzle3dSetActiveExampleWork` at editor `🦀️.rs:4582` and siblings)
that are dead code behind the 4-element `PUZZLE3D_RETAINED_TOOL_IDS` gate. So the bulk of the work is
one shared piece of app infrastructure plus mechanical per-route wiring — **not** rewriting 60
handlers.

## Likely shared root cause with the runtime fault

Every actor turn also faults with `runtime live cleanup faulted for instance 1`, traced to
`ArtifactStore::take_returned_snapshot_read_retirement` (`🏪️store/🦀️.rs:14534`) returning
`Err("snapshot read retirement factory is not installed")`; `from_new` (`:13803`) leaves it `None`,
`from_initialized_runtime_with_owners` (`:13845`) sets it. The fixture's blocker text for both lanes
names a missing **root-retirement** factory. These are plausibly the same gap, and one fix may close
both — being verified.

## Open question for the dev

"All windows with different examples": puzzle3d's edit mode opens two instances of one window kind,
`puzzle3d-main-top` and `puzzle3d-main-perspective`. The active example is GLOBAL —
`Puzzle3dScene.fixture` (`✏️editor/🦀️.rs:320`), mutated app-wide by `setActiveExample` (`:4662`).
puzzle2d and puzzle5d are the same. So:

- "every window renders correctly and both examples work" — reachable via the migration above;
- "each window shows a DIFFERENT example at the same time" — needs new per-window example state; it
  is not expressible in the current data model.

I am building toward the first reading. Say the word if you meant the second.

## Build verification — blocked on live framework churn (2026-09-03 ~01:15)

The puzzle3d plugin and stdio both compile (native and `wasm32-wasip2`). What `dev 3d` still cannot
finish is the **engine wasm** step, and the blocker is not in the engine crates themselves — it moves
every few minutes as two other sessions push the `RUNTIME-DEPENDENCY-ELIMINATION` (serde →
`ToValue`/`FromValue`) migration through shared framework crates. Observed in sequence, each
confirmed against a file edited within the previous ~20 minutes:

| Time | Blocker | Owner |
|---|---|---|
| ~23:5x | `semio-framework-surface` 28 errs / `semio-framework-editor` 17 errs — serde trait resolution on `ActorInstance*`/`PropertyValue` | settled on its own; no longer reproduces |
| ~00:5x | `E0283 type annotations needed` ×3 in `♾️infinite/🎲️board/…/🕸️dag/🦀️.rs` (`Value::from(<ambiguous numeric>)`) — reduced surface/editor/flow-core to 2-5 errors, all transitive | peer fixed `:5343` while I watched |
| ~01:1x | `Effect: serde::Deserialize<'de> is not satisfied` ×3 at `🧰️framework/🔨️modules/🎠️kernel/🦀️.rs:1818,1822` (`TurnResult` still derives `Deserialize`, `Effect` no longer does) | file mtime 01:08, edited 9 min before the check |

Not fixing these: each is inside another session's in-flight edit, and the previous one resolved
itself within the hour. Per the repo's own guidance, poll rather than chase.

Consequence for this ticket: **puzzle3d has not yet been observed running from a fresh build.** The
only runtime observation so far is against the Sep 1 prebuilt wasm, which showed the app shell,
example picker, both window instances and the tour rendering, and every actor turn faulting with
`runtime live cleanup faulted for instance 1`. That fault's first two hypotheses were investigated
and **disproved**:

- the monotonic clock is installed (`clock=true` in the cooperative-maintenance probe);
- the snapshot-retirement-factory chain is intact for puzzle3d — `Puzzle3dPlayApp` does override
  `build_document_store_owners()`, forwarded through `VcsArtifactApp::with_registry_on_bus`'s
  `install_member_store_owners_exact` (`🔌️plugin/🦀️.rs:27134`), so all three stores that
  `begin_local_interaction_query` draws leases from have `snapshot_retirement_factory = Some(...)`.

`build_artifact_store_one_item_preparation_factory` IS genuinely missing on `Puzzle3dPlayApp`, but it
is currently inert (the one registered tool factory declares only `HostOnly`/`Config` lanes), so it
does not explain the fault either — though it is exactly the infrastructure the interactive-job
migration needs.

So the live-cleanup fault's cause is still open, and re-measuring it against a fresh build is the
next step once the framework tree compiles.

## Where this stops (2026-09-03 ~01:40)

`semio-s-plugin-puzzle` compiled clean earlier tonight (201 errors → 0 after the value-derive fix).
It does not now: it fails with ~80 `dsl::DslValue` vs `serde_json::Value` / `ToValue` / `FromValue`
mismatches spread **identically across puzzle2d, puzzle3d and puzzle5d** (`Puzzle*Command`, their
configs, `MeshData`, the diff types). That is ticket
`26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS` landing in the puzzle plugin
right now, from another session. Verified not ours: `git diff -U0` hunk headers show none of the
failing puzzle3d editor lines (185, 224, 1130, 1705-1889, 6715, 7333, 7530, 7632, 9012) fall inside
any hunk this ticket touched (2480, 6155, 6203, 6377-6558, 6587, 6607, 7228, 7581, 7913).

Sequence of blockers hit while trying to get one verified run, each in a file another session had
edited minutes earlier, each resolving on its own and being replaced by the next:

surface/editor serde (settled) → `♾️infinite` `E0283` (peer fixed) → framework kernel
`Effect: Deserialize` (peer fixed) → puzzle plugin serde→dsl (in flight now).

**So: puzzle3d has not been observed running from a fresh build, and I am not claiming it works.**
The one runtime observation remains the Sep 1 prebuilt wasm — shell, example picker, both window
instances and tour rendering; every actor turn faulting `runtime live cleanup faulted for instance 1`,
cause still open after two hypotheses were disproved.

## Recommended next step for the dev

Wait for `RUNTIME-DEPENDENCY-ELIMINATION` to finish landing in `✏️s/🔌️plugins/🧩️puzzle`, then:

1. `RUSTC_WRAPPER="" cargo check -p semio-s-plugin-puzzle` — should be clean again; this also gives
   Wave 1 (`setActiveExample` + `Puzzle3dArtifactStorePreparationFactory`) its first real rustc pass.
   Wave 1 is **written but not rustc-verified** — treat it as unproven until that runs.
2. `bun dev:puzzle:3d`, and re-measure the live-cleanup fault against the fresh wasm. It may be gone:
   `🔌️plugin/🦀️.rs` was rewritten twice on Sep 2, after the Sep 1 binary that showed it.
3. If it persists, the discriminator is already identified — log whether
   `snapshot_read_leases.has_returned()` is true at fault time (`🏪️store/🦀️.rs:14538`); the factory
   itself is confirmed installed, so the fault is elsewhere in `EditorApp::maintenance_step`.
4. Then continue the interactive-job migration wave by wave using the per-lane route lists above.

Note for whoever runs the build: a `dev 3d` invocation longer than ~9.5 minutes cannot be completed
from an agent session — the child is killed when the tool call is backgrounded. Run it from a real
terminal, or drive `cargo rustc -p semio-s-plugin-puzzle --target wasm32-wasip2 --profile dev --
-C link-arg=-zstack-size=8388608` in repeated passes first (cargo is incremental, so each pass
advances) and only then start the dev server.

## Correction: the puzzle plugin failures are framework cascade, not a plugin-side migration

The section above attributed the ~80 `semio-s-plugin-puzzle` errors to `RUNTIME-DEPENDENCY-ELIMINATION`
landing *in the puzzle plugin*. That was wrong, and the owning session (which does own the ticket)
independently said it could not attribute them to itself either — no agent of theirs had touched
`✏️s/🔌️plugins/🧩️puzzle` this session.

Settled by grouping every error line of `cargo check -p semio-s-plugin-puzzle` by file:
**100% are under `🧰️framework/`, zero under `✏️s/🔌️plugins/🧩️puzzle`.**

- `🧰️framework/🔨️modules/📡️replication/📡️wire/🦀️.rs:1517,1528` — `E0425: cannot find function
  default_true in this scope` (a `#[serde(default = "default_true")]` helper left referenced after
  the helper moved out of the production cfg)
- `…/📡️wire/🦀️.rs:1574` — `frames::SelectionMode`, `SelectionMethod`, `MergeMode` : `serde::Serialize`
  not satisfied
- `🧰️framework/🔨️modules/🕹️interaction/🦀️.rs:38,49`, `🧰️framework/🔨️modules/🎠️kernel/🦀️.rs:227,1034,1820`
  — same `E0277` shape

One root cause, not two: an in-progress `#[cfg_attr(test, derive(Serialize, Deserialize))]` strip on
`semio-framework-actor` makes serde test-only while production consumers still require it — the same
edit that produces the `dsl::ActorId: serde::Deserialize` errors in `semio-framework`. So the puzzle
plugin itself is not broken; it simply cannot compile its dependency.

The owning session has a known-good clean revert path for that strip (already exercised once today)
and has agreed to take it, which should return both `semio-framework` and the puzzle plugin to green
in one step.

## Verification standard raised

That session also reported four defects today that compiled clean and passed `cargo check` yet were
functionally wrong: a DEFLATE path succeeding on 0 of 513 inputs, a u64 rendering as `3600.0`, the
value-derive silently dropping `serialize_with` on enum-variant fields, and a `deserialize_with`
removal caught only by `cargo test` (3/183 fixture failures).

Consequence for this ticket: a green `cargo check` is NOT sufficient evidence for anything here. Both
the puzzle plugin generally and Wave 1 (`setActiveExample` + `Puzzle3dArtifactStorePreparationFactory`,
still never rustc-verified) must be validated with `cargo test`, and the app observed running, before
any claim that puzzle3d works.

## Hard limit: long builds cannot be completed from an agent session

Established by test, not assumption. A child process started from a tool call is killed when that
call ends, under every launch method available here:

- harness background (`run_in_background`) — killed; five `dev 3d` attempts died this way, each with
  an empty log and no error
- foreground call that exceeds the ~9.5 min tool timeout — the call is "moved to the background" and
  the child is killed with it; `cargo check -p semio-framework-surface` died twice this way
- `nohup … & disown` — same; the log got exactly one line ("Blocking waiting for file lock on build
  directory") before the process vanished
- `setsid` — not available on macOS

The only builds that complete are those that finish inside a single foreground call. After the
framework revert the workspace needs a large rebuild, and other sessions hold the cargo lock much of
the time, so `cargo check -p semio-framework-surface` alone does not fit in 9.5 minutes right now,
let alone `bun dev:puzzle:3d`.

Cargo is incremental, so repeated passes do advance — but this is not a reliable way to reach a
running dev server.

**Therefore the final verification of this ticket has to be run from a real terminal.** Exact
sequence, once `semio-s-plugin-puzzle` is green (the owning session will confirm, and has committed
to running its tests first):

```bash
cd /Users/ueli/Documents/semio
RUSTC_WRAPPER="" cargo test -p semio-s-plugin-puzzle          # also Wave 1's first real verification
SEMIO_BUILD_BUDGET_MS=3600000 S_OS_PORT=6081 bun run dev:puzzle:3d
```

Then in the browser at `http://127.0.0.1:6081/`: confirm both window instances (Top, Perspective)
render; switch the example picker between "Nakagin Capsule Tower" and "Concrete Forest" (this is the
`setActiveExample` route Wave 1 migrated — before Wave 1 it was rejected outright with
`interactive-job.not-ui-safe`); then select the fill tool and drag its count slider.

Expect the fill tool to still be dead: `setFillCount` and `fillBuildTick` remain
`BatchOnlyPendingRewrite`. `setFillCount` is in the Config-lane group (36 routes) and `fillBuildTick`
is in the HostOnly group (6 routes) whose blocker is that its completion does not represent its real
effect — that one needs reducer work, not wiring. Those are waves 2 and 3.

## Engine set closed; puzzle plugin is the sole remaining blocker (2026-09-03 ~04:30)

All three engine wasm crates that `bun dev:puzzle:3d` builds are green:

| Crate | Result | Verified by |
|---|---|---|
| `semio-framework-surface` | 0 | owning session, after I reported `DagCamera: serde::Serialize` at `🗺️surface/🕸️node-graph/🦀️.rs:494` |
| `semio-framework-editor` | 0 | owning session |
| `semio-framework-os-flow` @ `wasm32-wasip2` | 0 | this session, two independent completed runs (a third timed out mid-rebuild and was discarded, not counted) |

The flow-core zero is the load-bearing one: that crate pulls in `semio-s-plugin-stdio` plus ~10 flow
extension plugins, so it is the first evidence tonight that a real slice of the plugin graph builds
clean at `wasm32-wasip2` — not a zero sitting behind a red dependency.

### The error counts were never measuring what they appeared to

Puzzle read 91 errors, then 133. That was NOT a regression: the newest mtime anywhere under
`✏️s/🔌️plugins/🧩️puzzle` was 23:55, and no agent of either session wrote there for four hours. As the
framework chain went green, more of puzzle's own code actually reached the type-checker and
previously-invisible errors became visible. 91 was never a measurement of puzzle's health — it was
how far the compiler got before aborting.

Three separate misattributions tonight all trace to the same illusion: mine (blaming a plugin-side
migration for framework cascade), an agent's (blaming a phantom concurrent session), and the owning
session's (blaming its own agents for edits they never made). The narrow rule that would have
prevented all three: **check whether anything was actually written — mtimes — before reasoning about
who wrote it.** And: the first error in a rustc dump is the least informative line in it, because
compilation aborts at the first failing crate and everything downstream is invisible rather than fine.

### What the 133 actually are

One root cause, now analysed rather than delegated: `command_from_action`'s trait signature moved to
`Option<&dsl::DslValue>` in all three puzzle editors, but the ~90 `🎮️commands/*` handlers those
editors dispatch to still take `serde_json::Value`. That single mismatch is 89 of the E0308s; the
remainder (32 trait bounds, 8 closure signatures, 7 missing methods, 1 unresolved import) is its
tail. Mechanical, not exploratory.

Everything this ticket still needs — Wave 1 verification, waves 2-3 for the fill tool, and
re-measuring the live-cleanup fault — is downstream of that crate compiling.

## Retraction: "puzzle never regressed" is NOT established (2026-09-03 ~05:15)

The section above states, as fact, that the 91 → 133 error move was purely the compiler reaching more
code and that nothing was written under `🧩️puzzle` for four hours. **That rested on a measurement now
known to be broken**, and I am retracting it rather than leaving a false certainty in this ticket.

The owning session established that its mtime instruments fail *silently toward clean-looking
answers* on this machine: `find -newermt '30 minutes ago'` returns zero matches for a file just
touched, and `ls -lT | sort -k3` sorts the wrong column. Its "newest mtime under 🧩️puzzle is 23:55"
came from those, and is false — `stat -f '%m %N'` shows writes at 04:56, 04:59, 05:00, 05:01, and its
agents had been running since 02:30. So writes were happening during the window in which the count
rose, and the rise may have been agent edits, the wavefront, or both. **Unknown, not resolved.**

What survives: the wavefront effect is real and independently demonstrated elsewhere tonight (puzzle's
errors were genuinely invisible while the framework chain was red, and `semio-framework-surface` was
never checked at all until its dependencies went green). What does not survive is using it to explain
this particular count movement.

The instrument that IS reliable here, verified against `ls -la` on the same files and agreeing exactly:

```bash
stat -f '%m %Sm %N' -t '%H:%M:%S' <file>
find <dir> -name '*.rs' -type f -exec stat -f '%m %Sm %N' -t '%H:%M:%S' {} + | sort -rn | head
```

Meta-pattern worth carrying: across both sessions tonight, every failed measurement failed toward a
falsely clean result — a broken grep returning 0, an anchored `^` undercounting 1,676 to 7, an empty
output file from a missing `timeout` binary reading as "0 errors", `find -newermt` reading as "nothing
written", and a crate reading 0 errors purely because its dependency never compiled. **A clean number
in this repo deserves more suspicion than a dirty one.**

## Blocker cleared; verification handed to a terminal (2026-09-03 ~07:50)

**`semio-s-plugin-puzzle` compiles: 0 errors.** Verified twice, independently — my own run (isolated
`CARGO_TARGET_DIR`, exit 0, "Finished dev profile", zero error lines) and the owning session's
detached run, different target dirs, same result. That was the last blocker to `bun dev:puzzle:3d`.

Engine set also green: `semio-framework-surface` 0, `semio-framework-editor` 0,
`semio-framework-os-flow` @ `wasm32-wasip2` 0.

### What is NOT verified, and must not be assumed

1. **The wasm target of the puzzle plugin.** Everything above is a NATIVE check. `#[cfg(target_arch =
   "wasm32")]` code never compiles natively, and `dev 3d` builds `wasm32-wasip2`. Four attempts
   (two in the shared dir, two isolated) each exhausted a full 9.5-minute window without completing —
   the shared dir loses to continuous peer lock contention and the isolated dir needs a cold build of
   the whole dependency tree for that target. **Native 0 does not imply wasm 0.**
2. **The puzzle plugin's tests.** Never ran; every attempt was killed. So Wave 1 (`setActiveExample`)
   is still diff-verified only — no compiler and no test has ever exercised it.
3. **The app itself.** Never observed running from a fresh build.

### The 5d conversion is incomplete, despite compiling

`serde_json` refs in `🖐️5d/…/✏️editor/🦀️.rs`: 184 → 135. Of those, 29 are the other owner's `json!`
sites; **106 are real serde touches that remain**. By the agreed acceptance criterion — reference
count, not compile — this is unfinished. The file compiles and still links serde_json.

**Held deliberately, ready to resume:**
- the 9 struct derives (`Puzzle5dDocument`, `Puzzle5dPart{,2d,3d}`, `Puzzle5dFastener`,
  `Puzzle5dGrip{,2d,3d}`, `Puzzle5dPartAnchor`) → `dsl::ToValue, dsl::FromValue`;
- gated behind a byte-for-byte differential test (serialize a populated document through both paths
  while the derives coexist, assert identical bytes) covering the 10 `skip_serializing_if` sites, a
  `None`, an empty `Vec`, a `default`-reliant field, nested part/grip/fastener, and an integer field
  asserted not to widen to Float. Precedent: `value-derive`'s
  `flatten_nested_struct_matches_serde_json_byte_for_byte`.
- `Puzzle5dImportJob.fragment` can now use the real recursive descent: the framework gap I reported
  was closed — `Object::remove` and `Object::iter_mut` added at `🎒️pack/🔤️json/🦀️.rs:210,216`. The
  flat single-step disposal substituted meanwhile is safe but not byte-exact, and should be replaced.

### Why this session cannot finish it

No command longer than ~9.5 minutes can complete here: the harness kills the child when the tool call
ends, under every launch method tried (harness background, foreground overrun, `nohup`+`disown`;
`setsid` does not exist on macOS). Three or more peer sessions compile continuously, so the shared
cargo lock is rarely free. A 97 MB wasm component build cannot be driven through in 9.5-minute
slices, and repeated passes produced no new artifacts — they spent their windows waiting on the lock.

### Run this from a terminal

```bash
cd /Users/ueli/Documents/semio
RUSTC_WRAPPER="" cargo test -p semio-s-plugin-puzzle     # first real check of Wave 1
SEMIO_BUILD_BUDGET_MS=3600000 S_OS_PORT=6081 bun run dev:puzzle:3d
```

At `http://127.0.0.1:6081/`: both windows (Top, Perspective) should render; the example picker should
switch between "Nakagin Capsule Tower" and "Concrete Forest" — that is the `setActiveExample` route
Wave 1 migrated, and before Wave 1 it was rejected outright with `interactive-job.not-ui-safe`.

**Expect the fill tool to still be dead.** `setFillCount` is in the Config-lane group (36 routes) and
`fillBuildTick` in the HostOnly group (6 routes, blocker: "the current empty/effect-only completion
does not represent the real effect" — needs reducer work, not wiring). Those are waves 2 and 3.

Also re-measure the `runtime live cleanup faulted for instance 1` fault against the fresh build; it
may be gone, since `🔌️plugin/🦀️.rs` was rewritten twice after the Sep 1 binary that showed it. Two
hypotheses are already disproved (the clock is installed; the snapshot-retirement chain is intact).

### Instrument caveats found tonight — every one failed toward a falsely clean answer

`find -newermt` (silent zero) · `ls -lT | sort -k3` (wrong column) · anchored `^` grep undercounting
1,676 to 7 · empty output file from a missing `timeout` binary reading as "0 errors" · a crate reading
0 purely because its dependency never compiled · **`pgrep -c cargo` reporting 0 while `ps` showed
cargo and rustc running** · a per-session scratchpad path shared between sessions ("no such file"
reads as fine). Use `stat -f '%m %N'` for mtimes and `ps | grep` for liveness.

## Wave 2 — `setFillCount` migrated (the fill tool), written not yet verified

The recorded blocker for the Config lane ("no app-owned retained preparation and root-retirement
factory", 36 routes) is **stale**. Both already exist on `Puzzle3dPlayApp`:
`build_config_store_one_item_preparation_factory` → `Puzzle3dConfigStorePreparationFactory`
(editor `🦀️.rs:6627`) and `build_config_store_owners` (`:6623`), alongside `build_document_store_owners`
(`:6619`).

And the fill tool's handlers were never missing: `setFillCount` **and** `fillBuildTick` both already
resolve in `build_tool_job` (`:~6684`) via
`"cycleBrushCandidate" | "cycleBrushCandidateBack" | "fillBuildTick" | "registerBrushMesh" | "setFillCount" | "suggestionsTick" => Box::new(Puzzle3dPrecomputeCommandWork::new(tool_id))`.
They were gated only by `PUZZLE3D_RETAINED_TOOL_IDS`. So the fill tool was wiring, not machinery.

Four sync points moved together — verified on disk, not taken from an agent's report:

| # | Site | State |
|---|---|---|
| 1 | `PUZZLE3D_RETAINED_TOOL_IDS` (`:2530`) | now 6 ids, includes `"setFillCount"` |
| 2 | `ArtifactToolPublicationContract` (`:6208`) | `tool_id: "setFillCount", lanes: [Config]` — lanes read from the fixture's own group, not assumed |
| 3 | `bounded_first_step_tool_proofs!` `tools:` (`:6652`) | includes `"setFillCount"` |
| 4 | `.action_interactive_job("setFillCount", …)` (`:7286`) | `Migrated` |

Plus `🧪️publication-authority/🔣️.json` updated. All four must agree or the app fails to CONSTRUCT with
`interactive-job.catalog-incomplete` / `catalog-authority` — they do.

`fillBuildTick` deliberately NOT migrated: its blocker is semantic ("the current empty/effect-only
completion does not represent the real effect"), which needs reducer work rather than wiring.

**Not verified.** Six `cargo check` attempts across two target dirs each exhausted a full 9.5-minute
window without completing. A peer's `world3d` framework signature conversion invalidated the warm
isolated cache mid-session, so even that — previously the one reliable long command — now needs a
full rebuild. 17 concurrent peer `cargo`/`rustc` processes were running at the time of the last
attempt.

### Known incoming interference, not ours

That same `world3d` conversion deliberately left puzzle's call sites un-updated (puzzle was
do-not-touch for its agent). Confirmed present in `🧊️3d`: editor `🦀️.rs:~37, ~2635, ~2636, ~2648`
plus several `✏️editor/🎮️commands/*` files, all passing `serde_json::Value` into
`apply_world3d_sun_action` / `apply_world3d_projection_action` / `world3d_projection_action_moves_pose`.
**Errors naming those three functions belong to that session, which has said it will own them.**
Anyone measuring puzzle must exclude them before attributing a regression.

Correct attribution rule for the next measurement:
```bash
grep "🧊️3d" errors.txt | grep -vcE "world3d"     # ours
grep -cE "world3d" errors.txt                      # theirs
```

## Correction: `🧊️3d` production serde is 23, not 670

The Wave 2 section above quotes "670 `serde_json` references" for `🧊️3d`. That was a raw `grep -rn`
and it is wrong — it counted comment lines, the `🧪️`/`🏭️`/`🔬️` third-party oracle and generator
evidence base, and `#[cfg(test)]`-gated code, i.e. exactly the code this ticket must NOT remove.

Measured properly (strip comments, oracle/generator dirs, `#[cfg(test)]` blocks and single items):

| area | production `serde_json` |
|---|---|
| `🚪️io` | **0** — already clean, not "not clean" as I claimed |
| `🧬️schema` | 4 |
| `✏️editor` | 13 |
| **total** | **23** |

So 3d's remaining surface is small and the io layer is already done. My figure would have had someone
plan a conversion that mostly does not exist.

Notably this is the first measurement error of the night that ran **falsely dirty**. Every other one
ran falsely clean.

## The instrument list, and the asymmetry that is the actual finding

Twelve measurement failures across two sessions tonight, every one of which produced a confident
wrong answer:

`find -newermt` (silent zero) · `ls -lT | sort -k3` (wrong column) · anchored `^` grep undercounting
1,676 → 7 · empty output file from a missing `timeout` binary reading as "0 errors" · a crate reading
0 behind a red dependency (never compiled) · a crate reading 0 on the **wrong target** (native ≠
wasm) · `pgrep -c cargo` reading 0 against 15 live processes · a per-session scratchpad path reading
"no such file" · a build that did not finish · a build against a concurrently-written tree · an agent
report of edits it never made · **`/tmp/prodserde.py` returning 0 for a file path** (125 for the
containing directory; a file with 117 non-comment refs reported 0, no error, exit clean).

**Eleven of the twelve failed toward a falsely CLEAN answer.** That asymmetry is the durable lesson:
in this repo a clean number is the one that needs a second measurement; a dirty number is
comparatively safe. The one exception was mine, above.

Corollaries worth keeping:
- The first error in a rustc dump is its least informative line — compilation aborts at the first
  failing crate and everything downstream is invisible rather than fine.
- Responsibility partitions the work; only an explicit write-lock partitions the file.
- An agent's report of what it changed is not evidence that it changed it — diff the files.
- For conversion work the acceptance criterion is a reference count moving the right way, never a
  compile. A green build that keeps the dependency is a false pass.

### `🧊️3d` = 23 confirmed against the FIXED counter

The 23 above was measured with a version of the counter that had two bugs. Re-measured after both
were fixed: **still 23** (io 0, schema 4, editor 13) — neither bug touched 3d, so the corrected
figure stands. The 5d editor file, which the broken version reported as 0, now reads 102, consistent
with the ~106 measured independently before the derives were held.

Two further instrument failures, taking the night's count to fourteen:

- **`prodserde.py` on a file path → silent 0** (mine to find): `os.walk` yields nothing for a file, so
  it fell through to `return 0`. A file with 117 non-comment refs reported clean, no error, exit 0.
  This was being used to certify plugins manifest-clean across 32 crates. Now handles files and
  hard-errors (exit 2) on a nonexistent path — both verified here.
- **A comment between `#[cfg(test)]` and its `mod` broke block detection** (found in the audit my
  report triggered): the pending-state logic only checked the immediately-next line, saw
  `//#region …`, and cleared — so every ref inside that test module counted as production. `📕️norm`
  read 8 production refs and was about to have a correct manifest reverted as a miscertification. It
  is genuinely clean; those 8 are a deliberate third-party oracle inside the test module. Now 0.
- **`$?` after a pipeline reports the LAST command, not the one under test** (mine, committed live):
  checking the counter's exit status with `python3 … | tail -1; echo $?` reported 0 for a path that
  actually exits 2 — `tail` succeeded. Use `${PIPESTATUS[0]}`, or redirect and check directly.

Both of the falsely-**dirty** failures (my 670, norm's 8) came from over-counting the deliberate
oracle/test evidence base — the code this ticket must preserve. So the sharpened rule is:
**a clean number needs a second measurement; a dirty number needs checking against whether it is
counting the evidence base.**

### Known limitation of the counter — sound for certification, not for sizing

A file doing `use serde_json::{json, Value}` and then using bare `json!`/`Value` counts as **one**
reference, not one per use. `🌍️gis` read 152 both before and after five real call-site conversions.
The import is always caught, so "is this plugin manifest-clean?" is answered correctly; "how much
work remains?" is understated, sometimes badly. Do not size work from this number.

## `fillBuildTick`: blocker VERIFIED accurate — and wave 2 verified sound

Having caught one stale blocker in `🧪️publication-authority/🔣️.json` (the Config-lane one), I checked
this one against the code rather than inheriting it. It holds up, and the check also confirms the
`setFillCount` migration is doing real work.

`Puzzle3dPrecomputeCommandWork`'s completion (editor `🦀️.rs:6098-6124`) builds one `Emit` and
branches on tool id:

| tool id | what the completion emits |
|---|---|
| `setFillCount` | `Puzzle3dConfigMutation::SetFillRequest { count, generation }` + fill build scope — **substantive** |
| `cycleBrushCandidate` / `…Back` | `SetBrushCandidateIndex` + `UiDirtyScope::Full` — substantive |
| `fillBuildTick` | **`ui_scope` only. No mutations, no effects.** |
| `suggestionsTick` | `ui_scope` only |
| `registerBrushMesh` | `UiDirtyScope::None` |

The legacy `fill_build_tick` command (`🎮️commands/🪣️fill-build-tick/🦀️.rs`) does the actual work:

```rust
ctx.effects.push(Effect::SpawnJob { job, kind: FILL_JOB_KIND.into(), input, placement: JobPlacement::Isolated });
```

`Emit` **can** carry that — it has `pub effects: Vec<Effect>` (`🔌️plugin/🦀️.rs:10123`); I initially
reasoned it could not and was wrong. The capability exists; the Work simply does not use it. So
migrating `fillBuildTick` as it stands would yield a tool job that runs, sets a UI scope, and never
spawns the fill planning job — the slider's `ready` extent would never advance and the failure would
be silent. Exactly the "compiles, dispatches, does nothing" class this ticket keeps meeting.

**Conclusions**

1. **Wave 2 (`setFillCount`) is sound.** Its migrated path emits the real `SetFillRequest`, not a
   UI-scope stub. The slider's committed value will reach the document.
2. **`fillBuildTick` genuinely cannot be migrated by wiring.** Its Work arm must first push the
   `Effect::SpawnJob { kind: FILL_JOB_KIND, placement: Isolated }` that the legacy command pushes.
   That is a real, bounded reducer change — not the four-line sync-point move waves 1 and 2 were —
   and it is the last piece of "the fill tool works end to end".
3. `suggestionsTick` and `registerBrushMesh` have the same shape and the same gap.

So of the two blocker strings in that fixture I have now checked against source, **one was stale and
one was accurate.** Neither can be trusted without reading the code; both must be.

## Session close — `framework-surface` fixed; the wasm plugin profile is the next real gate

### Fixed and verified this pass

`semio-framework-surface` was failing the `dev 3d` engine build with 2 errors — the same
"type migrated, call site didn't" shape as the earlier `DagCamera` one:

```
🗺️surface/🕸️node-graph/🦀️.rs:903  DagNodeSpec: serde::Deserialize not satisfied
🗺️surface/🕸️node-graph/🦀️.rs:989  DagLayoutOptions: serde::Deserialize not satisfied
```

Both call sites still used `serde_json::from_str`. Converted to the established convention,
`dsl::os_pack::json::from_json_str`, after confirming both types actually carry `FromValue`
(`DagLayoutOptions` derives it; `DagNodeSpec`'s is hand-written because its `kind` is
`#[serde(flatten)]`, which the derive cannot express). **`cargo check -p semio-framework-surface`
→ 0 errors, verified.**

### The stdio wasm profile gate — documented, not a new breakage

Driving the plugin wasm build directly produced:

```
error: linking with `wasm-component-ld` failed
  failed to encode component → module was not valid
  functions count exceeds limit of 1000000
```

This looked like a hard structural blocker. It is not new, and Cargo.toml says so explicitly at
`[profile.dev.package.semio-s-plugin-stdio]`:

> "even at one unit that monolith crosses the component parser's one-million-function ceiling.
> Owned component publication therefore uses the optimized `[profile.wasm-release]`; debug builds
> remain compile/check inputs, never catalog bytes."

So a `dev`-profile stdio component is expected to be inadmissible. The documented path is
`SEMIO_PLUGIN_PROFILE=wasm-release` (or `SEMIO_BUILD_MODE=ship`), selected in
`🧑️‍💻️dev/…/📜️script.ts:94`. **I nearly reported this as a discovered structural blocker; reading the
manifest before claiming it is what caught it.** Fifteenth measurement trap, and the first where the
answer was already written down.

### Where it stops

Re-running with `SEMIO_PLUGIN_PROFILE=wasm-release` got past stdio and into
`semio-framework-os-kernel`, which now fails with:

```
E0599: no method named `set_token` found for reference `&DirectoryClient<T>`
E0599: no method named `mint_session` found for reference `&DirectoryClient<T>`
   at 🔨️modules/📇️directory/🪪️identity/🦀️.rs:176,189,192
```

`🪪️identity/🦀️.rs` has not been touched since Sep 1 23:11. `🔌️client/🦀️.rs`, where
`DirectoryClient` is defined, was modified **22 seconds before I measured** (16:20:53). Another
session is mid-refactor on that type right now, having removed or renamed those methods while the
consumer still calls them. Not mine, and not safe to fix underneath them.

### For the next run

```bash
SEMIO_PLUGIN_PROFILE=wasm-release S_OS_PORT=6081 bun dev:puzzle:3d
```

Plain `bun dev:puzzle:3d` uses `--profile dev`, which cannot produce an admissible stdio component.
Run it from a real terminal: a `dev 3d` invocation longer than ~9.5 minutes cannot complete from an
agent session — the child is killed when the tool call backgrounds, which cost five attempts here
even though the native plugin build did complete once (15m 42s).

## Why `fillBuildTick` cannot be migrated by wiring — the precise reason

Earlier this note said the blocker was that `Puzzle3dPrecomputeCommandWork`'s completion emits only a
`ui_scope`. That is true but is a symptom. The structural cause, from the shared trait at
`✏️s/🔌️plugins/🧩️puzzle/🎮️commands/🧵️retained/🦀️.rs:38-56`:

```rust
pub trait PuzzleCommandWork<A: ArtifactApp>: Send {
    fn step(&mut self, command: &A::Command, snapshot: &A::Snapshot, config: &A::Config,
            interaction: &protocol::InteractionState, hover: &InteractionHoverState)
        -> Result<PuzzleCommandWorkStep<A>, Fault>;
    …
}
```

A `Work` receives command, snapshot, config, interaction and hover. **It never receives the app
instance.** But the legacy `fill_build_tick` (`🎮️commands/🪣️fill-build-tick/🦀️.rs:18-32`) needs exactly
that:

```rust
let mut precompute = ctx.app.precompute.borrow_mut();
let changed = precompute.poll_fill_job();
let spawn = precompute.enqueue_fill_job();
…
ctx.effects.push(Effect::SpawnJob { job, kind: FILL_JOB_KIND.into(), input, placement: JobPlacement::Isolated });
```

`poll_fill_job` / `enqueue_fill_job` live on `app.precompute`. So the Work cannot produce the
`SpawnJob` effect because **it cannot reach the state that decides whether to spawn one** — not
merely because its `Emit` happens to be empty. (`Emit` does carry `effects: Vec<Effect>`, so the
carrier exists; the input does not.)

Migrating it therefore requires extending `PuzzleCommandWork::step` to pass the app — a signature
change on a trait shared by **all three** puzzle artifacts, touching every `Work` impl in 2d, 3d and
5d, in files other sessions are actively editing. That is a real design change, correctly recorded in
the fixture as semantic rather than wiring, and it is not something to write without a compiler.

**Consequence for the goal:** the fill tool's *control* is migrated and functional
(`setFillCount` emits a real `SetFillRequest`), but its *background planning tick* still needs the
legacy dispatch path or the trait change above. Until then the slider commits a value while the
planner does not advance under the migrated path.

## Chain state at 17:10 — one link remaining

Every crate `bun dev:puzzle:3d` needs, measured (not inferred), after this session's fixes:

| crate | target | result |
|---|---|---|
| `semio-framework-surface` | native | **0** (fixed here: `DagNodeSpec`/`DagLayoutOptions` → `from_json_str`) |
| `semio-framework-editor` | native | **0** |
| `semio-framework-os-flow` | wasm32-wasip2 | **0** (two independent runs) |
| `semio-framework-os-kernel` **--lib** | wasm32-wasip2 | **0** |
| `semio-s-plugin-puzzle` | native | **0** |
| `semio-s-plugin-stdio` | wasm32-wasip2 | **18** — ticket 26/09/03/BREP-KERNEL-DEPENDENCY-FREE-RUNTIME, another session's live fleet refactor |

### `--lib` matters on os-kernel

`cargo check -p semio-framework-os-kernel --target wasm32-wasip2` reports 2 errors that are pure
noise for this path: `cannot find cli in os_spr` / `os_pack`, from the `spr` and `pack` **binary**
targets. A plugin build never builds those bins. Checking that crate without `--lib` sends you
chasing failures that cannot affect the app.

### The `DirectoryClient` episode — and a false attribution stopped

`os-kernel` was blocked by 3 × E0599 (`set_token`, `mint_session` removed from `DirectoryClient`
while `🪪️identity/🦀️.rs` still called them). Three peers each confirmed it was not theirs, which
pointed at an orphaned uncommitted edit — HEAD had both methods, the worktree had neither. I said I
would restore them from HEAD **if** that held, and asked all three first rather than acting.

It did not hold: the owner was alive and drove it 3 → 1 while we watched. The last error was not
mid-write either — `LocalHubCredential::read_inherited` is `#[cfg(not(target_arch = "wasm32"))]`
(it reads inherited fd 3) and the consumer called it unconditionally, i.e. a cfg mismatch. A peer
gated the consumer to match; verified clean afterwards.

Two things worth keeping from it:
- **"Nobody claims it" is not evidence of abandonment.** Three independent "not mine" answers plus an
  uncommitted diff still pointed the wrong way. Only mtimes and a falling error count settled it.
- A peer relayed my *conditional* intent as a plan, and another session began waiting on me to fix a
  file I had never opened. Corrected immediately. **A wrong owner is worse than an unknown one,
  because everyone else stops looking.**

### Remaining work for the goal, unchanged and specific

1. stdio's brep arms land (owner's fleet, gated on `cargo check -p semio-s-plugin-stdio --lib`).
2. `SEMIO_PLUGIN_PROFILE=wasm-release S_OS_PORT=6081 bun dev:puzzle:3d` — from a real terminal.
3. `fillBuildTick` still needs `PuzzleCommandWork::step` to receive the app instance; until then the
   fill slider commits real values while the planner does not advance under the migrated path.
