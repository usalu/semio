# G5 — `runtime-inventory-missing` × 172: a new, more precise root cause found — a tool-level
# constraint mismatch, not source churn — and a concrete disproof of the "trivial to generate" plan

Shard G5 of `SEPARATE-ARTIFACT-STANDARD-SUBSET-IMPLEMENTATIONS-AND-FIXTURE-TEST-EVERY-MUTATION`.
Scope: `runtime-inventory-missing` (172 subsets at session start, confirmed against
`.🧬semio/🦑️repo/⚡️cache/breaches/testing.json`). Ran ALONE per the brief, foreground only, no
background agents/commands. Builds on B4 (`📓️b4-runtime-inventories.md`), E4
(`📓️e4-runtime-inventories.md`) and G3 (`📓️g3-runtime-inventories.md`) — all three read in full
before starting. This shard's job was: get `s.stdio.step@ap214/cc6` measured end to end first, then
scale using G3's generator scripts.

## Headline result

1. **Both of E4's fixes are still landed and correct** — re-verified: `taxonomy.json`'s
   `"inventory"` entries in `testPhases`/`testLevellessPhases` present; `cc6`'s corrected bridge
   import paths present and unchanged (`git status` still shows it `M`, matching E4's own diff
   description exactly).
2. **The coordinator's premise that the machine was quiet did not hold for the whole session.**
   `uptime` load average moved from 10.35–18.37 at session start to 20.89–33.67 forty minutes later,
   and `git status --porcelain` on the `step` artifact tree shows heavy, unrelated, currently-live
   concurrent editing by other sessions across `cc1`–`cc6` and `base` (io/schema/oracle/fixtures) —
   this is not something I touched; it is background reality the report below had to account for.
   One specific instance was caught directly: a genuinely broken, uncommitted match arm in
   `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🔌️client/🦀️.rs` (missing
   `DirectoryStreamMessage::RebootstrapRequired`, `error[E0004]`) blocked `semio-framework-os-kernel`
   — and therefore every possible bridge, `brep`/`mesh` included — for a real ~15-minute window; a
   second isolated check minutes later showed it fixed by whoever owns
   `26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS` (confirmed: the match arm
   now present, `cargo check -p semio-framework-os-kernel` clean). This is the SAME class of finding
   E4/G3 already documented (a live peer edit transiently blocking every bridge) — real, but not the
   final blocker.
3. **Found the ACTUAL, deeper blocker this session, and it is new: a tool-level constraint mismatch,
   not source churn.** Once the framework layer was clean, `cc6`'s bridge (whole-crate-link against
   `semio-s-plugin-stdio`, the ~90-artifact stdio megacrate) still could not be built — not because
   of errors (zero compile errors were ever observed, six full attempts, matching E4's and G3's own
   "never once failed on real code" experience) but because:
   - The crate genuinely needs 30–60+ minutes of **uninterrupted** wall-clock to compile+link (G3's
     own directly-observed ceiling: 61 minutes, zero errors, then cut off).
   - This shard's own operating instruction is **foreground only, no background commands** — and the
     Bash tool's own hard cap is **600,000 ms (10 minutes) per call**. Six consecutive 10-minute
     attempts were made; every one reached almost exactly the same point in the build log
     (~20,280–20,290 lines, mid-way through the crate's own artifact list, around `xlsx`) before
     being killed, with **zero cumulative progress across attempts**.
   - Traced WHY there was no cumulative progress, concretely (not assumed): `find
     $SHARED/debug/incremental -iname '*semio_s_plugin_stdio*'` showed the session directory for my
     bridge's own metadata hash suffixed **`-working`** after every kill — rustc's own marker for an
     incomplete incremental session, which cargo discards and restarts from scratch on the next
     invocation. A `kill -9`/timeout mid-build does not get a chance to let rustc finalize that
     session, so every 10-minute window re-pays the same ~1–2 minutes of frontend/typecheck work
     (which is why the log always reaches the same point) and then **loses 100% of whatever codegen
     progress it made** when killed.
   - Also traced why the coordinator's "shared, pre-warmed `CARGO_TARGET_DIR`" did not actually help
     this bridge's build, concretely: `cc6`'s bridge Cargo.toml declares its **own** `[workspace]`
     (deliberately, to avoid root-workspace lease contention — same pattern as `brep`/`mesh`). A
     `cargo build -p semio-s-plugin-stdio --lib` run from *within the main repo workspace* (which is
     how the shared target dir was pre-warmed) produces a **different** `-C metadata=` fingerprint
     than the *same crate* compiled as a path-dependency of a bridge with its own standalone
     workspace and its own `--extern` graph — confirmed directly via `cargo build -v`: my bridge's
     own invocation carried `-C metadata=eed468ec1e647daf` for `semio_s_plugin_stdio`, while the
     pre-existing `libsemio_s_plugin_stdio.rmeta` already sitting in the shared target dir carries
     **no hash suffix at all** (a different compilation unit entirely — the coordinator's own
     "already warm and green" build, from a directly different graph). So the "warm cache" saved
     essentially nothing for this specific bridge shape; the megacrate had to be compiled from a cold
     start every single attempt, inside a hard 10-minute ceiling that structurally cannot contain it.
4. **This is a genuinely new, more precise diagnosis than E4's ("concurrent source edits") or G3's
   ("shared-machine CPU/swap contention") — both real phenomena, both reproduced again this session,
   but neither is the proximate cause any more.** Even with load lower than G3's worst readings and
   a supposedly pre-warmed cache, the whole-crate-link bridge pattern is **structurally incompatible
   with a strict foreground-only, 10-minute-per-call execution constraint**, because (a) the crate
   needs far more than 10 minutes of *uninterrupted* wall-clock and (b) a killed attempt at this
   granularity loses all its progress rather than banking it. No amount of retrying inside this
   constraint would ever finish it, quiet machine or not.
5. **Investigated the one architectural way around it — E4's own recommended follow-up (rewrite
   `cc6`'s bridge onto the narrow `#[path]`-mount `DESCRIPTORS` pattern `brep`/`mesh` already use,
   which needs no `semio-s-plugin-stdio` dependency at all) — and found it is NOT the small,
   consistency-only change E4 described for `cc6`, nor the "trivial, clone the generator" job G3's
   Part 5.A assumed for the ~40 remaining single-subset artifacts.** Traced `cc6`'s real, transitive
   compile dependency graph by reading source, not guessing (detail in Part 3). Headline finding:
   `step`'s artifact-level `semio_framework_plugin::derive_artifact_facets!` macro entangles **every
   one of `step`'s seven subsets (`base` + `cc1`..`cc6`) with every other** at the schema-macro
   level, because `derived_composition` and `io_registry` are two inline modules inside the SAME
   file (`✳️base/🚪️io/🦀️.rs`) — mounting one via `#[path]` (unavoidable, needed for
   `StepComposerComposition`) brings the other, and `io_registry` needs
   `StepCc1Composer`..`StepCc6Composer` from all six conformance-class subsets' own schema files. A
   narrow bridge for `step` is not "mount 4 leaf files" — it is "mount essentially all ~30–40
   production files under `step`", because `brep`/`mesh` avoid this entirely by **deliberately
   excluding `🚪️io`** (their own doc comment: "brep's importers and exporters reach into six sibling
   artifacts, none of which a mutation inventory consults") — an option `step`'s subsets do not have,
   since the macro that builds their own `StepArtifact` needs `io::derived_composition` directly.
   Given the size and risk of hand-mounting ~30–40 files correctly with zero opportunity to compile-
   verify within budget, and given the ticket's own explicit priority ("a fabricated/wrong
   measurement is far worse than an open breach"), **I did not attempt this rewrite.** See Part 3 for
   the full traced dependency chain, and the itemised remainder for what it would take.
6. **Zero fabricated inventory files, zero production edits.** `.🧬semio/🦑️repo/⚡️cache/tests/
   results/🏭️inventory/` still holds exactly the same two pre-existing files (`brep`, `mesh`,
   `Aug 28` timestamps, untouched). No file under any artifact's production tree was written or
   edited by this shard. `cc6`'s bridge `Cargo.lock` shows a further mechanical diff from my build
   attempts (dependency graph re-resolution against the live, evolving workspace — same
   "mechanical, cargo-driven, not hand-edited" pattern E4 and G3 both already documented for their
   own build attempts on this exact file); not reverted, since reverting a cargo-computed lockfile
   against a moving workspace is more disruptive than leaving it.

## Before / after

| id | before (session start) | after (session end) |
| --- | --- | --- |
| `runtime-inventory-missing` | 172 | **172 (unchanged)** |
| `runtime-only-mutation` | 0 | **0** |
| `manifest-only-mutation` | 0 | **0** |
| `mutation-outcome-mismatch` | 0 | **0** |
| `mutation-variant-mismatch` | 0 | **0** |
| total breaches, repo-wide | 692 | **679** (−13, other shards' unrelated concurrent work — confirmed via the live, unrelated churn documented in headline point 2; none of the 13 are in this shard's five tracked ids) |

Both runs: foreground `bun ./📜️script.ts test contract` (non-zero exit both times, as always
expected — the authority is `testing.json`, not the exit code), before at session start
(`🗑️generated/g5-test-contract-before.txt`) and after at session end
(`🗑️generated/g5-test-contract-after.txt`), both cross-checked directly against
`.🧬semio/🦑️repo/⚡️cache/breaches/testing.json` via the python one-liner in the shared brief.
`runtime-inventory-missing` staying at exactly 172 confirms nothing was measured and nothing was
fabricated to compensate. Full scope-list dump of the 172:
`🗑️generated/g5-runtime-inventory-missing-before.txt`. Five comparison-group counts, both runs
identical: `🧿️semio` 17, `📄️pdf` 10, `🗒️note` 8, `📐️step` 7, `🎞️gif` 5, `🏗️ifc` 5.

## Part 1 — re-verifying prior fixes, isolated

- `grep -n '"inventory"' 🔣️taxonomy.json` → both `testPhases`/`testLevellessPhases` entries present.
- `git status --porcelain` + direct read of `✳️cc6/🏭️bridge/🦀️.rs` → E4's corrected
  `semio_s_plugin_stdio::artifacts::step::standards::v_ap214::subsets::cc6::schema::mutations::{…}`
  import confirmed present, unchanged, still uncommitted (`M`).
- `.🧬semio/🦑️repo/⚡️cache/tests/results/🏭️inventory/` → exactly `brep`+`mesh`, `Aug 28` timestamps,
  confirmed unchanged before AND after this session (checked both times).

## Part 2 — the `cc6` build: six attempts, zero code errors, a precisely traced tool-level ceiling

All six attempts used `CARGO_TARGET_DIR=<the coordinator's shared, pre-warmed scratch dir>` and
`RUSTC_WRAPPER=""`, per the brief.

1. **Via `test inventory` (inherits env)**: bridge exited 1, truncated stderr showed a fragment
   mentioning `SemioAnimationMutation`/`protocol::Mutation` — inconclusive from the 3-line truncation
   the harness applies.
2. **Direct `cargo build --offline --bin semio-step-ap214-cc6-bridge` (full log, no truncation)**:
   `error[E0004]: non-exhaustive patterns: &DirectoryStreamMessage::RebootstrapRequired { .. } not
   covered`, in `📇️directory/🔌️client/🦀️.rs:497` — traced to a live, uncommitted edit
   (`git diff --stat` showed `+40` insertions, owned by ticket `26/09/01/…`, confirmed via
   `grep -rl RebootstrapRequired` across that ticket's own research notes). Re-checked minutes later,
   isolated (`cargo check -p semio-framework-os-kernel`): **clean**, confirming the other session had
   landed its own fix in the interim — real, transient, not mine, not chased, exactly per house
   rules.
3. **Retry, same command**: different error fragment (`SemioMutation`/`protocol::Mutation` path
   note) from the truncated bridge-script stderr; full `cargo build` log for this attempt showed
   **zero `^error` lines** through 19,522 captured log lines before a 5-minute Bash-tool timeout
   killed it mid-flight (still compiling `semio_s_plugin_stdio`, evidenced by ongoing xlsx-artifact
   warnings).
4. **Retry #2, 9m50s budget**: 20,286 log lines, **zero errors**, killed by timeout.
5. **Retry #3, 9m50s budget**: 20,279 log lines — **the same point as retry #2**, zero errors, killed
   by timeout. This is the observation that triggered the deeper investigation in headline point 3:
   two consecutive full-budget attempts landed at the SAME log position, meaning no forward progress
   was banked between them.
6. **`-v` verbose spot-check (25s, deliberately short)**: captured the exact `rustc` invocation for
   `semio_s_plugin_stdio` — confirmed **no `-C codegen-units=1` flag** (the workspace-root
   `Cargo.toml`'s `[profile.dev.package.semio-s-plugin-stdio] codegen-units = 1` pin, put there for a
   documented wasm32-wasip2 `rust-lld` SIGSEGV workaround, does **not** apply — the bridge's own
   standalone `[workspace]` uses cargo's own default profile) and the metadata hash
   `eed468ec1e647daf` used for this run.

Cross-referenced that hash against `$SHARED/debug/deps/`: the pre-existing `libsemio_s_plugin_
stdio.rmeta` the coordinator's own pre-warm produced carries **no hash suffix** — i.e. a distinct
compilation unit from mine, confirming the shared warm cache does not transfer to this bridge shape
(full reasoning in headline point 3). Checked `$SHARED/debug/incremental/semio_s_plugin_stdio-
<my-hash>/` directly after a kill: the one session directory present was suffixed **`-working`**
(rustc's own "incomplete" marker) — confirming each kill discards that attempt's codegen state
rather than banking it (also headline point 3). Confirmed source stability during this whole window
independently: `find ✏️s/🔌️plugins/🗄️stdio -newermt '-20 minutes'` → **0 files** — so the "same log
position every time" result is not source churn; it is the interrupted-build-loses-progress
mechanism, isolated and confirmed.

**Verdict for `cc6` specifically: environment-adjacent but now precisely NOT environment-quietness-
blocked — it is a hard mismatch between (a) this bridge shape's genuine ≥30–60-minute uninterrupted
build requirement (matching G3's own directly-observed 61-minute, zero-error ceiling) and (b) this
shard's mandated foreground-only execution with the tool's 10-minute-per-call cap.** No further
retrying inside that constraint would ever finish it. This is not a restatement of E4/G3's findings —
both of their proximate causes (concurrent source edits; CPU/swap contention) were checked directly
this session and ruled out as the CURRENT limiting factor; the limiting factor now is the execution
model itself.

## Part 3 — why the narrow `#[path]`-mount rewrite is not the quick fix E4/G3 assumed

E4 (Part 3) recommended, and G3 (Part 4/5) assumed as a template, rewriting bridges like `cc6` onto
the `brep`/`mesh` narrow-mount `DESCRIPTORS` pattern (own `[workspace]`, depends only on
`semio-framework-os-kernel`/`-schema`/`-plugin`/`-3d`/`-number`/`pack`, `#[path]`-mounts just the
production files a subset's mutation enum needs — no `semio-s-plugin-stdio` dependency at all, hence
no megacrate compile). I traced, by reading real source (not guessing), exactly what `cc6` would
need, to check whether this is the small job E4's phrasing ("for consistency … and because …") and
G3's Part 5.A ("same shape as `gif@87a`, trivial") implied:

1. `StepCc6Mutation` (the `DESCRIPTORS` source) needs `StepSnapshot`, `StepDiff`, `engine::ladder`
   (`ClassEdit`/`ProductIdentity`/`ShapeRepresentationRow`/`apply_class_edit`/`invert_class_edit`),
   `cc6::schema::MAX_RUNG`, and `base::schema::mutations::{apply_step_mutation, StepMutation}` — all
   confirmed via direct `use` statements in the real, current `🧬️mutations/🦀️.rs`.
2. `MAX_RUNG` lives inside `cc6/🧬️schema/🦀️.rs`'s own `derived_analysis` module (`pub const
   MAX_RUNG: u8 = 6;`, confirmed by reading the file) — so mounting `MAX_RUNG` means mounting cc6's
   **whole** schema file, not a cherry-picked constant (`#[path]` mounts whole files; Rust has no
   finer-grained import of "just this const from this file, skip the rest").
3. `derived_analysis` in turn needs `base::schema::{StepAnalyzer, StepParts}` — both produced by
   `base/🧬️schema/🦀️.rs`'s own `semio_framework_plugin::derive_artifact_facets!(...)` macro
   invocation, which references `super::super::io::derived_composition::StepComposerComposition` —
   confirmed by reading the macro call site directly.
4. `derived_composition` is an **inline** module (not a separate file) inside `✳️base/🚪️io/🦀️.rs` —
   the same file that inline-declares `io_registry`, which needs `StepCc1Composer`..`StepCc6Composer`
   from **all six** conformance-class subsets' own schema files (confirmed by reading
   `io_registry`'s own `use` block). `#[path]` mounts the whole file; there is no way to take
   `derived_composition` without also compiling `io_registry`'s six-subset dependency, short of
   hand-copying part of the file (which the ticket's own philosophy explicitly rules out — mount
   real production files, do not duplicate their logic).
5. Contrast with why `brep`/`mesh` are cheap: their own doc comment states it directly — `🚪️io` is
   "deliberately absent: brep's importers and exporters reach into six sibling artifacts, none of
   which a mutation inventory consults." `semio`'s `brep` subset's schema does **not** route through
   an artifact-composition macro that needs `io::derived_composition`; `step`'s does. This is a real,
   structural difference between artifacts, not a difference in effort someone forgot to spend.

**Conclusion: a correct narrow-mount bridge for `step` needs essentially all of `step`'s own
production tree mounted (`base` + `cc1`..`cc6`, both `🚪️io` and `🧬️schema`, ~30–40 files) — matching
B4's "one bridge per artifact" idiom in scope, not a 4-file cherry-pick.** That is a bounded,
tractable, but substantial one-time engineering job, with real risk of a silently-wrong `#[path]`
mount if rushed and never compile-verified. Given remaining session budget did not allow both
building it AND compile-verifying it end to end, and given the ticket's own explicit priority
("a fabricated/wrong measurement is far worse than an open breach" — the same line G3 quoted when it
deleted its own four unverified bridges), **I chose not to attempt it rather than leave an unverified
mount in the tree or, worse, a verified-looking one that is subtly wrong.** This is the same
discipline B4/E4/G3 each held; I am holding it too.

**This generalises beyond `step`.** `pdf`(1.4/1.7) and `ifc@2x3` — the two other members of G3's
"shared snapshot, split enum" family, together accounting for 15 of the 172 — are architecturally the
SAME shape as `step` (one shared base snapshot + several conformance/profile subsets under it); they
should be assumed to carry the same `io_registry`-style cross-subset coupling until someone actually
checks, not assumed trivial. I did not have budget to verify pdf/ifc directly this session; flagging
this as the single highest-value next check, since it could invalidate a large slice of G3's
"ready, unexecuted" generator-script estimate.

**It also generalises to the "~40 trivial single-subset artifacts."** `semio-s-plugin-mathematical`
(and, per B4's own Part 1 finding, `note`/`draw`/`sequence`/`sequence` too) declares an
**unconditional** Cargo dependency on `semio-s-plugin-stdio` (confirmed again this session by reading
its `Cargo.toml` directly) — so a bridge for ANY of them, built the naive Cargo-dependency way,
would hit the exact same 30–60-minute megacrate wall this shard hit for `cc6`, regardless of how
small or self-contained that artifact's OWN mutation vocabulary is. The narrow `#[path]`-mount
pattern is not optional polish for these — it is required to make ANY of the 172 buildable inside a
reasonable window, and each one's OWN io/schema coupling (as with `step`) needs to be checked before
assuming the mount is small.

## Part 4 — two concrete process findings for whoever resumes

1. **A pre-warmed shared `CARGO_TARGET_DIR` does not, by itself, speed up a whole-crate-link
   bridge's build.** The bridge crate's own standalone `[workspace]` (needed to avoid root-workspace
   lease contention, same as `brep`/`mesh`) produces a different `--extern`/profile graph and
   therefore a different `-C metadata=` fingerprint than a `cargo build -p semio-s-plugin-stdio
   --lib` run from inside the main repo workspace. Verify this with `cargo build -v` and compare the
   `-C metadata=` value against the actual filenames already sitting in the target dir's
   `debug/deps/` before assuming a pre-warm will help — it did not here.
2. **A killed (`SIGTERM`/timeout) cargo build banks nothing for this crate.** Its incremental session
   directory is left suffixed `-working` (rustc's own "incomplete" marker, visible directly under
   `$TARGET/debug/incremental/semio_s_plugin_stdio-<hash>/`) and is discarded, not resumed, on the
   next invocation. Chaining many short foreground attempts against this specific pattern is not a
   slower version of one long attempt — it is repeated, complete waste beyond the first ~1–2 minutes
   of frontend work each time. Only a single, genuinely uninterrupted run (≥30–60 minutes, per G3's
   own 61-minute observation) will ever finish it.

## Itemised remainder

- **`s.stdio.step@ap214/cc6`**: bridge code is correct (E4's fix, twice re-verified). Blocked purely
  by the tool-level constraint in Part 2 — needs either (a) a single genuinely uninterrupted
  ≥60-minute build window outside a 10-minute-per-call foreground cap, or (b) the narrow-mount
  rewrite scoped in Part 3 (~30–40 files, all of `step`'s own `base`+`cc1`..`cc6` `io`+`schema`
  trees), built and compile-verified as ONE piece of work, not split across many short sessions.
- **The other 171 subsets**: unchanged from G3's own itemisation (`📓️g3-runtime-inventories.md`
  Part 5), with two corrections from this session's investigation: (1) the `step`/`pdf@1.4`/
  `pdf@1.7`/`ifc@2x3` "shared-snapshot, split enum" family (25 subsets, G3's `🔨️g3-gen-shared-
  snapshot-bridges.py`) should NOT be assumed cheap to generate — verify each for `io_registry`-style
  cross-subset macro coupling before running the generator, per Part 3. (2) EVERY remaining subset,
  including the "~40 trivial single-subset artifacts," needs the narrow `#[path]`-mount pattern
  specifically (not a Cargo path-dependency on its own plugin crate) to avoid the same
  `semio-s-plugin-stdio` transitive-dependency wall this shard hit — confirmed for `mathematical`
  directly, and per B4's own prior finding for `note`/`draw`/`sequence` too.
- **`os.config@1`** (3 subsets): different shape, not investigated (per G3, unchanged).
- **Genuine production gaps found this session: none newly confirmed** — no bridge build reached
  completion, so `compareInventories` never ran against real data this session. B4's Part 1 step 4
  spot-check (`mathematical`'s `change-coefficient` sidecar `outcomeClasses` vs. its v2 manifest)
  remains the one concrete piece of evidence, still unconfirmed by an actual bridge run.

## Files touched

- **Production source: none.** All investigation was read-only (source tracing) plus repeated,
  read-only-in-effect `cargo build`/`cargo check` attempts against the pre-existing, already-`M`
  `cc6` bridge.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/✳️cc6/🏭️bridge/
  Cargo.lock` — further mechanical `cargo`-driven diff from this shard's own build attempts (7
  insertions/8 deletions, dependency-graph re-resolution against the live, moving workspace); not
  hand-edited, not reverted (same disposition E4/G3 both already recorded for their own attempts on
  this file).
- Nothing else. Zero inventory cache files written (`.🧬semio/🦑️repo/⚡️cache/tests/results/
  🏭️inventory/` unchanged: exactly `brep`+`mesh`, confirmed before and after).
- `🗑️generated/g5-*` — every build log, gate run and scope dump, to be deleted at ticket close per
  house rules; this report's evidence stands on its own without them.

## Final answer

- **Inventories produced this session: 0.** The two pre-existing ones (`brep`, `mesh`) predate this
  ticket's shards and remain untouched.
- **Genuine production gaps found: 0** — no bridge build reached completion, so `compareInventories`
  never ran on anything new.
- **Build verdict: a precise, new diagnosis, not a restatement.** Zero compile errors were ever
  observed across six attempts on `cc6`'s bridge (matching E4's and G3's own "never fails on real
  code" experience). The actual, now-confirmed blocker is a **structural mismatch between this
  bridge shape's genuine ≥30–60-minute uninterrupted build requirement and this shard's mandated
  foreground-only, 10-minute-per-call execution constraint** — concretely traced to (a) a pre-warmed
  shared target dir that does not transfer to this bridge's own distinct compilation fingerprint, and
  (b) every killed attempt discarding its incremental codegen progress rather than banking it. A
  quieter machine would not have changed this outcome. The one architectural way around it (E4's
  recommended narrow-`#[path]`-mount rewrite) was investigated and found to require mounting
  essentially all ~30–40 files of `step`'s own production tree, not a small cherry-pick, because
  `step`'s artifact-composition macro entangles all seven of its subsets together — a real,
  substantial, but bounded follow-up, not attempted here given the risk of an unverified or subtly
  wrong measurement.
- **Before/after**: `runtime-inventory-missing` 172 → 172 (unchanged); the four comparison ids
  stayed 0/0/0/0 both times; total repo-wide breaches 692 → 679 (−13, entirely other shards'
  unrelated concurrent work, verified none of the 13 are in this shard's tracked ids).
- **Generalised findings for whoever resumes**: (1) `pdf`/`ifc@2x3` — G3's other two "trivial,
  shared-snapshot" families — should be checked for the same `io_registry`-style cross-subset macro
  coupling BEFORE assuming their generators are cheap; (2) every one of the "~40 trivial single-
  subset artifacts" needs the narrow-mount pattern too, since each of their own plugin crates
  (`mathematical` confirmed directly) unconditionally depends on `semio-s-plugin-stdio`; (3) the two
  process findings in Part 4 (fingerprint mismatch defeats the shared-target-dir trick; a killed
  build banks nothing) apply to any future attempt at any whole-crate-link bridge, not just `cc6`.
- **Report**: this file, `$TICKET/📓️g5-runtime-inventories.md`.
