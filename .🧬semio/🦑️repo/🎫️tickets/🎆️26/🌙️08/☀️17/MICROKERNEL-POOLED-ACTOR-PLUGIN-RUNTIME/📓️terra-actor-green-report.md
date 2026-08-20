# 📓️ terra-actor-green — report

Packet: `actor-green`. Scope: `🧰️framework/🔨️modules/🎭️actor/**` (only). Goal: take
`semio-framework-actor` from 98 compile errors to green and unblock
`semio-framework-plugin-host`.

## Result

`semio-framework-actor` is **green**: `--lib` and `--all-targets` both exit 0 with **zero
warnings**, and `cargo test` is **70 passed / 0 failed** — an exact match to the ticket's recorded
baseline (`60 → 69 shard-grants → 70 interactive-isolation`). Downstream, `plugin-host` is still
blocked, but by a **pre-existing, live, uncommitted break in a file outside this packet's scope**
(`💻️os/🔨️modules/🗣️dsl/🧬️schema/🦀️component.rs`), not by anything in the actor crate — see
"Downstream unblock attempt" below.

## Acceptance — commands, output, exit codes

All runs: `CARGO_TARGET_DIR=/private/tmp/claude-501/-Users-ueli-Documents-semio/e6a44461-bab7-421f-8a53-65123a5e9482/scratchpad/target-actorgreen`, foreground, single turn.

### 1. `cargo check -p semio-framework-actor --lib`
```
    Finished `dev` profile [unoptimized] target(s) in 0.15s
```
**EXIT 0.** Zero warnings. Full log: `terra-actorgreen-final-lib-check.txt`.

### 2. `cargo check -p semio-framework-actor --all-targets`
```
    Finished `dev` profile [unoptimized] target(s) in 0.15s
```
**EXIT 0.** Zero warnings. Full log: `terra-actorgreen-final-alltargets-check.txt`.

### 3. `cargo test -p semio-framework-actor`
```
test result: ok. 70 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.03s
   Doc-tests semio_framework_actor
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```
**EXIT 0.** Named set matches the ticket's baseline table exactly (70/0). Full log:
`terra-actorgreen-final-test-run.txt`.

### 4. `cargo check -p semio-framework-plugin-host --lib` — BLOCKED, not by this packet
**EXIT 101** — but the 6 errors are 100% inside
`🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🧬️schema/🦀️component.rs` (a product-tier DSL
schema test file, `plugin-host → semio-framework-os-kernel` dependency), e.g.:
```
error: mismatched closing delimiter: `}`
   --> …/🗣️dsl/🧬️schema/🦀️component.rs:2496:26
2493 |     async fn quantity_and_angle_round_trip_in_their_declared_unit() {
2496 |         assert_round_trip("material e=210GPa rho=7850kg/m3 rotation=30deg", &spec     assert_document_inline_agree("material e=210...
     |                          ^ unclosed delimiter
```
Six sites, all the same shape: a statement separator (`;` + newline) between two consecutive
`assert_*` calls has been deleted, fusing them into one unparseable expression. Full log:
`terra-actorgreen-pluginhost-BLOCKED-oskernel.txt`.

### 5. `cargo check -p semio-framework-os-kernel --lib` and `-p semio-framework --lib`
Per the ticket's own binding rule ("re-run both at the end and paste the codes — os-kernel has
regressed once already today from a concurrent edit"):
- `semio-framework-os-kernel --lib` → **EXIT 101**, same 6 errors as above.
  Full log: `terra-actorgreen-oskernel-RED-peer-break.txt`.
- `semio-framework --lib` → **EXIT 101**, same root cause (`semio-framework` depends on
  `semio-framework-os-kernel` per `Cargo.lock`). Full log: `terra-actorgreen-framework-RED-peer-break.txt`.

**This is not my packet's file.** `💻️os/🔨️modules/🗣️dsl/🧬️schema/🦀️component.rs` is outside my
owned path (`🧰️framework/🔨️modules/🎭️actor/**`), so per the binding rules I did not touch it.
Evidence this is a live peer edit, not something I broke or something stale: `git diff HEAD --stat`
on that exact path shows **208 lines of uncommitted working-tree changes right now**
(100 insertions / 108 deletions), and `git log --date=iso -3` on it shows its last *committed*
change was `cb9bcce7a4` — the break is in the **uncommitted** delta on top of that, i.e. a session
mid-edit. **Escalating to sol**: `os-kernel` and `semio-framework` are both red repo-wide right
now for a reason unrelated to `actor-green`, `plugin-host`'s own acceptance gate cannot be run
until it's fixed, and I have no path_scope to fix it myself.

## What was actually wrong (98 → 0)

Baseline measured fresh at start: `cargo check -p semio-framework-actor --lib` → **EXIT 101, 98
errors** (E0308 33 · E0271 23 · E0277 20 · E0605 4 · tail). Full baseline log:
`terra-actorgreen-baseline-98errors.txt`. This confirmed the packet brief's own count.

The 98 errors were the *visible* fraction of one systemic defect, not 98 independent bugs: the
crate's own async-codemod pass had converted every fn to `async fn` (including trivial,
I/O-free ones), and essentially **every call site of every such fn was still written as if it were
sync** — some of those omissions produced compile errors (the 98), most did not, because a
statement-position call to a `()`-returning `async fn` (`pack::write_u8(out, 0);`,
`scheduler.register_actor(...);`) silently compiles and silently does nothing. This is exactly the
class flagged in the packet brief: *"an encoder where 14 of 17 byte-writer calls lacked `.await`
so it wrote almost nothing."* Here it was closer to *all* of them, across the whole 3139-line file
— every `pack_encode` body, and a long tail of `Kernel`/`Scheduler`/`ShardTable` mutators
(`register_actor`, `set_active`, `set_shard`, `release_exclusive`, `unpin`, `quarantine_package`,
`on_clean_turn`, …) called as bare statements. None of these show up as compiler errors; they show
up as **behavior that silently never happens**. I found them with a purpose-built diagnostic
scanner (`terra-actorgreen-scan-dropped-futures.py` — enumerates statement-position calls to
locally-defined unit-returning `async fn`s; never edits, only lists candidates for review) and
fixed the codec-only, zero-collision-risk subset with a span-keyed patcher
(`terra-actorgreen-fix-dropped-encode-awaits.py` — a fixed, hand-reviewed list of exact
`(line, exact-current-text)` pairs, asserts the text still matches before touching anything). The
rest (Kernel/Scheduler mutators, test-module bodies) were fixed by hand, reading each call site for
what it does. **These two scripts are diagnostic/span-keyed tools, not the name-keyed bulk `.await`
inserter R10 prohibits** — see each script's own docstring for why.

### Fix categories, mapped to the ticket's residue-shape taxonomy

1. **Struct-literal field shorthand `.await`** (residue shape 7) — 2 parse errors:
   `Kernel::activate` (`budget.await,` as a bare field) and `Kernel::actor_record`
   (`mailbox.await,`). Fixed by naming the field (`budget: budget.await,` → after also fixing the
   underlying repeated-await bug below, just `budget,`).
2. **Awaiting one future repeatedly** (residue shape 2) — `Kernel::activate`, `ShardTable::pin`/
   `pin_avoiding` all awaited the same local (`id`, `shard`) at every use instead of once at
   binding; `Kernel::submit` had the same shape for `backpressure`. Fixed per the shape's own
   prescription: hoist the single `.await` onto the `let`, then every later use is the plain
   (`Copy`) value. The exact same bug, at far greater volume, filled `mod tests` — ~24 test
   functions, 219 edits — handled by a bounded, diagnostic-scoped script
   (`terra-actorgreen-hoist-repeated-await.py`) that brace-matches each
   `#[async_test]`-tagged fn body and only ever touches identifiers local to that one function; not
   a repo-wide name-keyed pass.
3. **R9 (E1-transitive pure accessors)** — 14 functions made sync + tagged
   `// 🚫️async: E1[-adjacent] …`, each because their only real consumer is either an external-trait
   impl (`Debug::fmt`, `Default::default`) or a sync std combinator (`Iterator::map`/`filter_map`,
   `Option::map_or`, `max_by_key`) that cannot itself be async: `ActorId::{plugin_ordinal,
   kind_tag, ordinal, generation}`, `Lane::weight`, `Mailbox::{len, is_empty, pressure,
   earliest_deadline}`, `Scheduler::{lane_of, actor_weight}`, `FailureState::new`,
   `SceneStore::new`, `ActorMetrics::wall_us_p95`. Every one verified I/O-free before the change
   (pure struct/bit-math or a `BTreeMap`/slice lookup) — both halves recorded per R9's own
   requirement.
4. **Closure/`Fn` return-type mismatch (E0271)** — the four generic pack combinators
   (`write_opt`/`read_opt`/`write_vec`/`read_vec`) took plain sync closures but were being handed
   async callees. First attempt used `AsyncFnOnce`/`AsyncFnMut` bounds (stable since 1.85); this
   hit a real rustc rough edge ("implementation of `AsyncFnOnce` is not general enough" — HRTB
   inference over a closure-captured, per-call future) that made the error count go **up** (98→141)
   before I reverted it. Final fix: deleted `write_opt`/`read_opt` in favor of two *concrete*,
   non-generic helpers (`write_opt_bytes`/`read_opt_bytes`, `write_opt_u64`/`read_opt_u64` — every
   `Option<T>` field in this wire format is one of exactly those two `T`s), and reordered
   `write_vec`'s callback to `(item, out)` so `T::pack_encode`'s own `(&self, out)` signature lines
   up and can be passed as a bare fn item (fn items don't hit the HRTB bug, only closures do) — the
   two-field `ShardTable::assignment`/`exclusive_leases` cases that still needed real closures were
   inlined as plain `for` loops instead.
5. **Recursive/self-referential and dropped-borrow bugs** — `ShardTable::pin`/`pin_avoiding`
   (shape 2, above); `Kernel::commit_frame`/`scene_of` (`.map()` over an async method — rewritten
   as an explicit loop / match); `Scheduler::tick`'s deadline-preemption scan (`.await` inside a
   sync `filter_map` closure, E0728 — the `earliest_deadline` R9 fix above removed the need for the
   await entirely).
6. **`async_fn_in_trait` warnings (R7)** — `ShardTransport`'s 4 methods triggered the standard
   "discouraged" lint. Added `#![allow(async_fn_in_trait)]` at the crate root
   (`📦️glue.rs`) with a comment citing R3/R7, and did **not** take rustc's `-> impl Future + Send`
   suggestion (would wrongly impose `Send` on this crate's `?Send` guest-side futures).
7. **`#[test]` on `async fn`** — this crate's ~37 test fns were already written `async fn` (correct
   per the universal-async convention) but still tagged bare `#[test]`, which cannot run an async
   fn. Used the shared `async-test-attr.py` tool (`--apply` on this packet's own path) to rewrite
   every site to `#[semio_framework_async_macros::async_test]`, and added the
   `semio-framework-async-macros` dev-dependency to this crate's own `Cargo.toml` (a packet-owned
   file, not a registrar-only one). The two `macro_rules!`-generated round-trip harnesses
   (`round_trip!`, `serde_round_trip!`) needed the same treatment applied inside the macro
   template itself.

### A defect this exercise actually found (not just async residue)

`Scheduler::tick`'s deadline-preemption/DRR-round loops and `Kernel::activate`/`complete`/
`quarantine_package`/`suspend`/`resume`/`request_exclusive`/`release_exclusive` all called their
own mutators (`register_actor`, `submit`, `set_active`, `set_shard`, `unpin`,
`release_exclusive`, `on_clean_turn`, `quarantine_package`) as **bare statements with the return
value silently dropped**. Before this packet, that meant `register_actor` never ran,
`quiet_grants`/`busy_grants` in `drr_fairness_plugin_with_50_actors_does_not_starve_plugin_with_1`
were measuring an empty scheduler, and `interactive_actor_avoids_a_shard_saturated_by_cpu_bound_actors`
never actually saturated anything — every one of these tests was **passing vacuously** the way the
brief's own worked example (a graph traversal that never ran) describes. `cargo test` was green
throughout the whole 98-error period on `--lib` only because `--all-targets`/tests weren't compiling
at all; once tests compiled, rustc's own `unused_must_use` lint caught the last 4 survivors
(`mailbox.pack_encode(&mut bytes);`, `record.pack_encode(&mut bytes);`, two `mailbox.enqueue(…);`)
as warnings, which is how I know the sweep is now exhaustive rather than merely "no compile error."

## Purity constraint (re-verified, not assumed)

`grep -n "wasm_bindgen\|web_sys\|winit\|tokio\|rayon\|std::thread\|SystemTime\|Instant::now"
🦀️component.rs` matches only the module doc comment and two doc-comments inside
`recv_deadline_returns_none_on_timeout`'s own docstring explaining why it does NOT use
`std::thread::current()`. `mod thread_transport` (the one `std::sync::mpsc`-based exception) is
still correctly behind `#[cfg(not(target_arch = "wasm32"))]`. Every `wasm_bindgen` use is confined
to `📦️glue.rs` behind `#[cfg(target_arch = "wasm32")]`.

## Files touched (all inside owned scope)

- `🧰️framework/🔨️modules/🎭️actor/🦀️component.rs` — the fix (679 insertions / 583 deletions vs `HEAD`)
- `🧰️framework/🔨️modules/🎭️actor/📦️packages/🦀️rust/📦️glue.rs` — `#![allow(async_fn_in_trait)]` + R3/R7 doc comment
- `🧰️framework/🔨️modules/🎭️actor/📦️packages/🦀️rust/Cargo.toml` — added `semio-framework-async-macros` dev-dependency
- Ticket folder (scratch, kept per rule 5): `terra-actorgreen-scan-dropped-futures.py`,
  `terra-actorgreen-fix-dropped-encode-awaits.py`, `terra-actorgreen-hoist-repeated-await.py`,
  `terra-actorgreen-baseline-98errors.txt`, `terra-actorgreen-final-lib-check.txt`,
  `terra-actorgreen-final-alltargets-check.txt`, `terra-actorgreen-final-test-run.txt`,
  `terra-actorgreen-pluginhost-BLOCKED-oskernel.txt`, `terra-actorgreen-oskernel-RED-peer-break.txt`,
  `terra-actorgreen-framework-RED-peer-break.txt`

## Registrar-only file touched as an unavoidable tool side-effect

`Cargo.lock` gained **26 lines**, purely additive: the one real edge my new dev-dependency needed
(`semio-framework-actor → semio-framework-async-macros`), plus two packages
(`semio-framework-ui-contract`, `semio-framework-ui-render`) that a live peer session had already
added to some `Cargo.toml` but that hadn't been resolved into the lockfile yet — `cargo check`
resolves the whole workspace graph, not just my crate, so that resolution happened as a side effect
of my build, not a manual edit. I did not hand-edit `Cargo.lock`; flagging per the registrar-only
rule rather than silently letting it pass.

## Open item for sol

`plugin-host`'s own acceptance gate (item 3 of my brief) cannot be exercised until
`💻️os/🔨️modules/🗣️dsl/🧬️schema/🦀️component.rs` is fixed by whoever owns it — it is a live,
uncommitted, ~208-line in-progress edit, not something stale or something I can attribute to a
finished packet. I have not touched it and have no path_scope to.
