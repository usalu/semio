# 📓️ terra-interactive-isolation report

**Status: mechanism landed and compiles; only 2 of 5 acceptance commands genuinely completed. The
other 3 (plugin-host test, wgpu-renderer check, bench) are UNRUN — see §5. No p95 is estimated
anywhere in this report.**

## 1. mechanism chosen + why

**Signal**: a new pure method, [`ActorMetrics::is_saturating(&self, budget: &Budget) -> bool`]
(`🎭️actor/🦀️component.rs:1909-1918`). An actor is judged CPU-saturating once it has completed at
least `SATURATION_MIN_TURNS = 2` turns (line 1837) AND its `wall_us_p95()` — already-tracked, from
the existing 64-sample ring `record_turn` fills — reaches `SATURATION_THRESHOLD_PERCENT = 70` percent
of *that actor's own* `Budget::wall_ms` ceiling (line 1842). Two turns, not one, so a single slow
sample (cold cache, one GC pause) can never flip an actor "hot"; p95 rather than the latest sample, so
one fast turn can't mask a sustained pattern either.

**Decision point**: `Kernel::activate` (`:2258-2272`). Every OTHER lane pins exactly as before this
packet (`ShardTable::pin`, unmodified count-based least-loaded). Only `Lane::Interactive` actors take
a different path: `Kernel::saturated_shards()` (`:2279-2281`) walks the kernel's own already-tracked
`self.actors` map, keeps every actor whose `metrics.is_saturating(&meta.budget)` is true, and maps
each to its current shard via `ShardTable::shard_of` — a `BTreeSet<ShardId>`. That set is passed to
the new `ShardTable::pin_avoiding(actor, avoid)` (`:1438-1445`), which places the actor on the
least-loaded shard **excluding** every shard in `avoid`, falling back to the ordinary unrestricted
least-loaded shard only if literally every general-pool shard is saturated (a saturated pool must
still admit the actor — `Backpressure::Rejected` owns "no room," not this method).

`pin` and `pin_avoiding` now share one private helper, `ShardTable::least_loaded(&self, avoid)`
(`:1411-1427`), which is the ORIGINAL `pin` body with an added `.filter(|(index,_)| !avoid.contains(..))`
before the same `min_by_key((count, index))` tie-break, and an `.or_else(..)` fallback to the
unfiltered version. Called with an empty set (what `pin` now does), it is byte-for-byte the same
arithmetic as before this packet — same iteration order, same comparator, same tie-break.

**How this stops queueing**: in production (`kernel_runtime` in `📦️glue.rs`, NOT the bench — see §2),
every user-opened plugin app activates with `Lane::Interactive` (`glue.rs:335`), progressively over a
live session. If a background/extension actor has, by the time a NEW app opens, shown sustained
near-budget turns, `saturated_shards()` already reflects that (built from `Kernel::complete`'s
already-running bookkeeping — no new tracking added), and the freshly-opened interactive app is pinned
to a DIFFERENT shard. It is placement-time avoidance, not runtime migration: an actor already pinned
before it turned saturating is not moved (see §6, gap 1).

**Why not the exclusive-reserve shards** (the mission's other suggested direction): I traced
`ShardTable::request_exclusive` and found it leases a WHOLE shard to exactly ONE actor at a time
(`for offset in 0..exclusive_reserve { if Vacant { insert; return } }`, `:1424+` in the pre-existing
code) — architecturally a single-heavy-actor isolation primitive, not a lane. With `exclusive_reserve`
capped at 2 (`ShardTable::new`'s `.min(2)`), routing 40 CPU actors through it is impossible; routing
the ONE interactive actor through it SHRINKS the general pool available to CPU actors (4→3 or 4→2),
concentrating more CPU work on fewer shards. I chose `pin_avoiding` within the SAME general pool
instead: it never reduces available parallelism for background work, only steers new interactive
placements away from shards that already look loaded.

## 2. why it cannot read fixture profile names

`is_saturating` takes only `&self` (the actor's own `ActorMetrics`) and `&Budget` (that actor's own
declared ceiling) — both are `🎭️actor`-crate-native types already flowing through `Kernel::complete`
before this packet. There is no `Profile`/`"cpu"`/`"hang"`/`"idle"` string anywhere in
`🎭️actor/🦀️component.rs` (confirmed by grep — the crate has no dependency capable of reading the
scale-fixture's `scale_fixture.profile` JSON field, which lives in a wholly different crate,
`🧫️fixtures/🔌️scale`). The mechanism reacts to what a turn actually cost in `wall_us`, nothing else;
it would behave identically against a real, non-bench actor that happened to run long, or against a
`cpu`-profile bench actor — the kernel cannot and does not tell them apart.

## 3. budget 3 non-regression argument

Budget 3's own bench path (`scale_bench::Env::activate` in `📦️glue.rs:775`) calls
`Kernel::activate(..., Lane::Background, ...)` for every one of its 100 actors — **always**
`Lane::Background`, never `Lane::Interactive` (confirmed by reading that call site directly; it is a
single hardcoded lane argument, not caller-configurable). My branch in `Kernel::activate` reads:

```rust
let shard = if lane == Lane::Interactive {
    let avoid = self.saturated_shards();
    self.shards.pin_avoiding(id, &avoid)
} else {
    self.shards.pin(id)
};
```

Every budget-3 actor takes the `else` arm — `self.shards.pin(id)`, the EXACT call that existed before
this packet, now routed through `least_loaded(&BTreeSet::new())`, which (per §1) performs the
identical count-based least-loaded computation the original inline body did. The code path budget 3
exercises is untouched by this packet, not merely "compatible with it." I additionally re-ran the
crate's own existing test proving the literal property budget 3 depends on —
`pin_spreads_actors_of_one_plugin_across_the_pool` (100 actors over 8 shards, asserts `max shard count
<= 100/8 + 1`) — unchanged, still green (§5). Real `perShardCounts {0:25,1:25,2:25,3:25}` against a
ceiling of 26 (the 40-shard-count-4 case the ticket's mission cites) is produced by the SAME
unmodified `least_loaded` arithmetic and is not something this packet's diff can perturb, because
`Lane::Background` never reaches `pin_avoiding`.

## 4. purity

No new state needing a clock was introduced. `is_saturating` reads only fields `record_turn` already
populates from a `Usage` the CALLER supplies (via `Kernel::complete(actor, result, now_ms)` — `now_ms`
itself is caller-injected and untouched by this packet); it performs no comparison against, and holds
no reference to, wall-clock time. `saturated_shards`/`pin_avoiding`/`least_loaded` are pure functions
of already-in-memory maps (`self.actors`, `self.shards.assignment`). Grepped the file for
`tokio|wasm_bindgen|web_sys|winit|std::thread|SystemTime|Instant::now` — the only 3 matches are the
pre-existing module-doc sentence naming the purity rule itself and two pre-existing doc-comments in
unrelated (untouched-by-me) regions discussing `ThreadTransport`; zero matches inside code I wrote.
`cargo check --target wasm32-unknown-unknown` against this exact diff **did complete** (see §5.2) and
passed, which is the actual gate for this claim, not just the grep.

## 5. exact line ranges edited, and command status

All edits are confined to one file: `🧰️framework/🔨️modules/🎭️actor/🦀️component.rs`. No other file
under `actor/`, `plugin-host/shard/`, or `wgpu/targets/` was touched — the placement/scheduling
mechanism did not require touching the wgpu dispatch loop or the plugin-host shard executor (see §6,
gap 1, for why: this packet is placement-time-only, no cross-shard instance migration).

| Region | Lines | What |
|---|---|---|
| import | `:17` | added `BTreeSet` to the existing `use std::collections::{...}` |
| `ShardTable::pin` | `:1396-1403` | body replaced with a call to the new `least_loaded` helper — same result, same doc comment (`:1380-1395`, untouched) |
| `ShardTable::least_loaded` | `:1405-1427` | **new**, private, shared by `pin`/`pin_avoiding` |
| `ShardTable::pin_avoiding` | `:1429-1446` | **new**, public |
| saturation consts | `:1834-1842` | **new**, `SATURATION_MIN_TURNS`, `SATURATION_THRESHOLD_PERCENT` |
| `ActorMetrics::is_saturating` | `:1903-1918` | **new** |
| `Kernel::activate` | `:2249-2272` | doc comment extended; body gains the `if lane == Lane::Interactive` branch |
| `Kernel::saturated_shards` | `:2274-2281` | **new**, private |
| style fix | `:1634`, `:2407` | two `unnecessary qualification` warnings my own `BTreeSet` import caused elsewhere in the file (pre-existing `std::collections::BTreeSet::new()` calls) — cleaned up, zero behavior change |
| new unit test | `:2602-2640` | `interactive_actor_avoids_a_shard_saturated_by_cpu_bound_actors`, the demanded pure placement-property test — see §5.1 |

**The file is NOT mid-refactor and does compile** — both claims below are backed by real, complete,
non-killed command output (not predictions):

### 5.1 `cargo test -p semio-framework-actor` — REAL, COMPLETE, ran to the end

```
$ CARGO_TARGET_DIR=".../🎯️target-ii" cargo test -p semio-framework-actor
...
running 70 tests
test component::tests::quick::interactive_actor_avoids_a_shard_saturated_by_cpu_bound_actors ... ok
test component::tests::quick::pin_spreads_actors_of_one_plugin_across_the_pool ... ok
[... all 70 listed ...]
test result: ok. 70 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.02s

Doc-tests semio_framework_actor
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
EXIT:0
```
Baseline was 69/0; this run is 70/0 (my one new test, everything else including
`pin_spreads_actors_of_one_plugin_across_the_pool` still green). This is genuinely captured terminal
output from a run that reached its own `test result:` line and a real `EXIT:0` for the `cargo test`
process itself (not a pipeline wrapper's exit code) — I read it back mid-run.

### 5.2 `cargo check -p semio-framework-actor --target wasm32-unknown-unknown` — REAL, COMPLETE

```
$ CARGO_TARGET_DIR=".../🎯️target-ii" cargo check -p semio-framework-actor --target wasm32-unknown-unknown
    Checking semio-framework-actor v0.1.0 (.../🎭️actor/📦️packages/🦀️rust)
warning: unnecessary qualification [x3, pre-fix — now cleaned up per §5 style-fix row]
warning: `semio-framework-actor` (lib) generated 3 warnings
    Finished `dev` profile [unoptimized] target(s) in 3m 42s
EXIT:0
```
Zero errors. (The 3 warnings shown are the ones §5's "style fix" row above already cleaned up in a
later edit — not re-verified with a third build, see below.)

### 5.3 `cargo test -p semio-framework-plugin-host --lib -- --skip schema_parity` — **UNRUN**

Started, but this machine is running ~35 concurrent cargo processes this wave (peer sessions plus
several of the coordinator's own packets sharing this repo). The Bash tool backgrounds any command
past ~120s, and — as the coordinator flagged — a backgrounded build cannot reliably report back across
a turn boundary. I killed the in-flight process on the coordinator's explicit instruction rather than
continue waiting. Its captured output stopped mid-dependency-compile (`wit-parser`, `crossbeam-deque`,
...) — **the crate itself was never reached, no test ran.** The `EXIT:0` in that output is the
`tail`/`echo` pipeline's own exit code, not cargo's (binding rule 6's own warning, hit exactly as
described) — I am not counting it as a pass. Genuinely UNRUN.

### 5.4 `cargo check -p semio-framework-os-renderer-wgpu --lib` — **UNRUN**

Started, never got past `Blocking waiting for file lock on build directory` (contending with the
plugin-host run above over the same `🎯️target-ii` dir, plus the wider 35-process sweep) before I
killed it on the same instruction. Zero compilation occurred. Genuinely UNRUN — I have no signal
either way on whether this crate (which imports `semio-framework-actor`) compiles against my diff.

### 5.5 `bun ./📜️script.ts bench plugins --renderer native --count 50 --extensions 50 --shards 4 --out <TICKET_DIR>/🔣️bench-interactive.json` — **UNRUN**

Never started. **No p95 is reported, estimated, or predicted anywhere in this report or file
`🔣️bench-interactive.json` does not exist.** §6 explains, as pre-registered reasoning (not a result),
why I expect this specific harness to show little to no movement even once run — that is an argument
about the harness's own `wait_for_outcomes` semantics, offered so the coordinator does not have to
re-derive it, not a substitute for the coordinator's own measurement.

## 6. honest gaps

1. **Placement-time only, no reactive migration.** `pin_avoiding` only ever decides where a NEW
   `Lane::Interactive` actor lands. An actor already pinned before it turns saturating is never moved
   off a shard an interactive actor already occupies — the reverse direction (protect an existing
   interactive actor from a LATER-arriving hot neighbor) is not handled. Doing so for real would need
   physically relocating a live `GuestInstance` across `ShardExecutor` OS threads (extract-from-old +
   re-register-on-new), which does not exist anywhere in `plugin-host/shard/` today; I judged building
   that novel subsystem, unverified, in the time available, riskier than shipping the placement-only
   half honestly flagged as partial.

2. **The bench harness itself never activates any actor with `Lane::Interactive`.** Traced directly
   (`glue.rs:775`, `scale_bench::Env::activate`): every one of the bench's actors — including the
   `profile == "idle"` one budget 5 calls "the interactive actor" — activates as `Lane::Background`.
   My mechanism's `if lane == Lane::Interactive` gate structurally cannot fire for ANY bench-driven
   activation, in budget 5 or elsewhere. This is a fact about the fixture/harness pairing, verified by
   reading the call site, not a guess.

3. **`scale_bench::Env::new` hardcodes `exclusive_reserve: 0`** (`glue.rs:753`), unlike the real
   `kernel_runtime` host which passes `2` (`glue.rs:288`'s own comment: "no caller in this file
   exercises it yet"). Neither of these facts required a code change from me — they're read-only
   findings about files outside my owned paths — but they matter for interpreting whatever the
   coordinator's own bench run reports: this packet's mechanism was reachable in NEITHER of the two
   ways described above for budget 5 specifically, so an unchanged number there would be consistent
   with the mechanism working as designed, not with it being absent.

4. **Reasoning, not a result, on why I would not expect budget 5 to move even if gaps 1-2 were
   closed**: `ParallelRuntime::wait_for_outcomes(expected, timeout)` (`🎠️runtime.rs:264-282`) blocks
   until `expected` outcomes arrive from ANY of the K shards, counted, not attributed to a specific
   actor — `env.pump()` (`📦️glue.rs`) loops calling this until every granted turn in the round is
   drained. Given 40 cpu + 1 interactive actor dispatch in one `Kernel::tick()` Decision (verified: DRR
   budget 64 comfortably covers 41 single-turn grants), the measured elapsed is bounded by whichever
   shard finishes LAST, regardless of which shard the interactive actor sits on. I could not identify
   a placement scheme, working within a fixed `shard_count=4`, that reduces the busiest shard's turn
   count below `ceil(40/4)=10` — moving the interactive actor around only changes WHICH shard carries
   10 vs 11, never the max. This is offered so the coordinator's own run is interpreted correctly if
   the number does not move — it is not a substitute for running it.

5. **The plugin-host and wgpu-renderer builds are genuinely unverified against this diff.** I have no
   evidence either way that `semio-framework-plugin-host` or `semio-framework-os-renderer-wgpu`
   compile cleanly against the new `pin_avoiding`/`is_saturating`/`saturated_shards` additions — only
   that `semio-framework-actor` itself does (§5.1-5.2), both natively and on `wasm32-unknown-unknown`.
   Both of those crates depend on `semio-framework-actor` and reference `ShardTable`/`Kernel::activate`
   directly, so a signature mismatch is possible in principle, though I added rather than changed any
   public signature `pin`/`activate` callers use (`pin`'s signature is unchanged; `activate`'s
   signature is unchanged — only its body branches internally).

## lease-requests

None. All edits stayed inside the owned path (`🎭️actor/🦀️component.rs`); no other file was touched.
