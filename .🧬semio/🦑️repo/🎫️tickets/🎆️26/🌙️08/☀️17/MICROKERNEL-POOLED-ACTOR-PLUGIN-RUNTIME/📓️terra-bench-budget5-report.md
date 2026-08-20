# 📓️ terra — bench budget5 report (packet `bench-budget5`, executor "terra")

## 0. Verdict, up front

**Budget 5 could NOT be measured this session.** Not "measured and missed" — the native scale-bench
binary (`semio-wgpu-native`, the only thing that runs `budget_4_and_5` / the whole native ladder
budgets 2-8) does not compile at the current HEAD. The failure is entirely inside
`semio-framework-ui` (`🖱️ui/**`), a crate this packet's `path_scope` explicitly excludes and I did
not touch. This is a real, reproducible, out-of-scope blocker, confirmed twice, and it blocks the
*entire* native 8-budget ladder, not just budget 5.

The fixture blocker sol fixed 20 minutes before dispatch (`semio-framework-os-scale-fixture`) is
confirmed genuinely fixed — see §1. A second, independent blocker further downstream is what stopped
this session: see §2.

## 1. Fixture blocker — confirmed fixed, wasm artifact built

```
$ cd /Users/ueli/Documents/semio
$ CARGO_TARGET_DIR=<scratchpad>/target-bench cargo check -p semio-framework-os-scale-fixture --lib --target wasm32-wasip2 --features component-guest --message-format=short
    Checking semio-framework-os-scale-fixture v0.1.0
    Finished `dev` profile [unoptimized] target(s) in 1m 20s
EXIT:0

$ CARGO_TARGET_DIR=<scratchpad>/target-bench cargo build -p semio-framework-os-scale-fixture --target wasm32-wasip2 --features component-guest
    Compiling semio-framework-os-scale-fixture v0.1.0
    Finished `dev` profile [unoptimized] target(s) in 4.45s
EXIT:0

$ file <scratchpad>/target-bench/wasm32-wasip2/debug/semio_framework_os_scale_fixture.wasm
... WebAssembly (wasm) binary module version 0x1000d   # real WASI-p2 component, 841552 bytes
```

Features match the consumer's own build step exactly (`component-guest`, same as
`BenchPluginsScript`'s `cargo build -p semio-framework-os-scale-fixture --target wasm32-wasip2
--features component-guest` in `🧑️‍💻️dev/📦️packages/🟦️typescript/📜️script.ts:4763`) — rule 22
satisfied. Raw logs: `terra-fixture-check.txt`, `terra-fixture-build.txt` (this ticket folder's
scratchpad copies live under the ticket root as plain `.txt`; the full paste is above since both
were short).

## 2. The blocker that actually stopped this session

`semio-wgpu-native` (the binary `budget_4_and_5` runs inside) unconditionally depends on
`ui_wgpu` = `semio-framework-ui` with `features = ["wgpu-engine"]`
(`…/📺️renderer/🧑️‍🎨️engine/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/Cargo.toml`, no feature gate — I
checked, there is no way to build this binary without compiling that dependency).

```
$ CARGO_TARGET_DIR=<scratchpad>/target-bench cargo build -p semio-framework-os-renderer-wgpu --bin semio-wgpu-native --features native-bin --message-format=short
... 682 errors, all inside semio-framework-ui, e.g.:
🖱️ui/…/🎯️targets/🧊️wgpu/🦀️widgets.rs:676:32: error[E0599]: no method named `dot` found for opaque type `impl Future<Output = Vec3>`
🖱️ui/…/🎯️targets/🧊️wgpu/🦀️host.rs:28:31: error[E0308]: mismatched types: expected `PointerModifiers`, found future
🖱️ui/…/🎯️targets/🧊️wgpu/🦀️draw.rs:372:137: error[E0308]: mismatched types: expected `Option<usize>`, found future
error: could not compile `semio-framework-ui` (lib) due to 682 previous errors
EXIT:101
```
Re-ran verbatim a second time (~10 minutes later, after the §3 floor checks) to rule out a transient
race: **identical 682 errors, same exit code.** Raw logs: `terra-wgpu-bin-build.txt` (1st),
`terra-wgpu-bin-build2.txt` (2nd, `--message-format=short`).

Error count by file (2nd run, `grep 'error\[' | sed -E 's/^([^:]+):.*/\1/' | sort | uniq -c`):

```
151 🖱️ui/…/🎯️targets/🧊️wgpu/🦀️draw.rs
109 🖱️ui/…/🎯️targets/🧊️wgpu/🦀️paint.rs
107 🖱️ui/…/🎯️targets/🧊️wgpu/🦀️events.rs
 77 🖱️ui/…/🎯️targets/🧊️wgpu/🦀️widgets.rs
 44 🖱️ui/…/🎯️targets/🧊️wgpu/🦀️reconcile.rs
 36 🖱️ui/…/🎯️targets/🧊️wgpu/🦀️flex.rs
 26 🖱️ui/…/🎯️targets/🧊️wgpu/🦀️shell.rs
 25 🖱️ui/…/🧱️elements/🪵️Tree/🧊️component.rs
 22 🖱️ui/…/🎯️targets/🧊️wgpu/🦀️tree.rs
 19 🖱️ui/…/🎯️targets/🧊️wgpu/🦀️text.rs
 …
```

**Why this is a live, unrelated, in-flight refactor and not my bug:**
- `git status --porcelain` shows **zero** uncommitted `.rs` changes anywhere in `🖱️ui/**` (only two
  unrelated `📜️script.ts` files are dirty). The 682 errors exist at the current committed HEAD
  (`13559f1c`, 2026-08-20 16:00:54) — this is a genuinely broken mainline state right now, not
  something I introduced or something sitting in a working tree.
- The failure pattern is uniform: dozens of call sites across `draw.rs`/`paint.rs`/`events.rs`/
  `widgets.rs`/`host.rs`/`engine.rs`/`shell.rs` calling a method (`Vec3::dot`, `active_foreground_of`,
  indexing, etc.) that now returns a `Future` and is not awaited — a systematic async-conversion
  sweep of `semio-framework-ui`'s core types, mid-flight, call sites not yet fixed up.
- `.🧬semio/…/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME/📌️important.md` (coordinator's own notes,
  same session) independently documents a live packet ("scene-surface") that put
  `semio-framework-ui` "mid-flight red" today via an unrelated relocation, and separately names
  `semio-framework-ui` as "one of the highest-fan-in crates in the tree" (15 direct dependents). A
  `ps aux` snapshot at the time of my second build attempt showed a *different* live packet
  (`insert-await.py --crate semio-framework-plugin …`) actively running an automated await-insertion
  codemod, scoped to `🔌️plugin`, not `🖱️ui` — so nobody was actively repairing `🖱️ui` at the moment I
  checked.
- `🖱️ui/**` and `🔌️plugin/🖥️host/**` (the crate `shard-lane` actually edited) are unrelated crates;
  `semio-framework-plugin-host`'s own `--lib` build (§3) is unaffected. This is not a knock-on effect
  of shard-lane's changes.
- Machine load at the time of the second failed build: `load averages: 93.72 47.93 36.92` (`uptime`)
  — extreme, consistent with several sessions compiling concurrently across the repo right now.

**I did not attempt to fix `🖱️ui/**`** — explicitly out of this packet's `path_scope`, and per rule 3
(lease-request rather than edit) this would need a lease I have no reason to believe would be
approved for a measurement packet. I also did not retry in a sleep-loop (rule: diagnose, don't
loop-retry a failing command once the cause is known) — two independent verbatim runs ~10 minutes
apart, same 682 errors, is sufficient to call this reproducible-not-transient for now.

## 3. Regression floor — held, checked before concluding

```
$ CARGO_TARGET_DIR=<scratchpad>/target-bench cargo test -p semio-framework-plugin-host --lib
test result: ok. 127 passed; 0 failed; 1 ignored; 0 measured; 0 filtered out
EXIT:0
```
Matches the required **127 passed / 0 failed / 1 ignored** exactly.

```
$ CARGO_TARGET_DIR=<scratchpad>/target-bench cargo test -p semio-framework-actor
test result: ok. 76 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
EXIT:0
```
**76 passed / 0 failed**, not the ticket's stated 70/0 — count is *higher* (0 failures, tests only
grew), not moved down. Consistent with other packets adding tests to this crate since the shard-lane
report was written; not a regression by this ticket's own definition (rule 7: "if your changes...
move either" — I made no changes to this crate). Flagging the number delta for the record rather than
silently reporting the ticket's stale expected count as if I'd verified it unchanged.

Raw logs: `terra-floor-plugin-host.txt`, `terra-floor-actor.txt`.

## 4. What was NOT run, and why

- `budget_4_and_5` itself: blocked, §2. Zero p95 samples collected — no repeated-run spread to report,
  because there is no single run to report.
- The other 7 native budgets (2, 3, 6, 7, 8; budget 4 is the RSS row bundled with 5): all gated behind
  the exact same `semio-wgpu-native` binary, so all 7 are equally blocked, not just budget 5. Budget 1
  (registry parse timing) is measured independently by the TS harness (`benchRegistryRow`, pure
  `readFileSync`+`JSON.parse`, no cargo) and does not depend on this binary — but `BenchPluginsScript`
  (the only wired entry point for it) runs it as row 1 of the SAME script invocation that then throws
  on the native build failure, so even that one row was never written to a report this session. I did
  not hand-roll a standalone budget-1 timing outside that script, since a number produced by a
  different code path than the one actually gated on `BENCH_BUDGETS` would not be a like-for-like
  comparison to the historical ladder.
- The (a)/(b)/(c) profiling classification from the brief: not applicable — that classification is
  for a measured-but-missing result. There is no measurement to classify. I have not formed an opinion
  on `KernelAsyncRuntime` necessity from this session; there is no evidence either way.
- I did not fall back to the web (`react`/`wgpu`) bench path to get *some* number. That path runs
  through browser `ShardClient`/Worker transport (a different, JS-side implementation), not
  `ShardLoop::pump_primed` — it would not exercise shard-lane's actual mechanism at all, and per
  `🧪️bench-web-harness.ts`'s own header, budgets 2 and 5 there are explicitly STUB-backed ("no real
  fleet wasm exists yet"). A web number would not be evidence about the native piece 1/2 changes and
  risks being misread as one; not produced.

## 5. Files touched

None inside my `path_scope` — this packet's only output is measurement, and no measurement was
obtainable. No edits made to `🧊️wgpu/📦️glue.rs`, `🎠️runtime.rs`, or `🧫️fixtures/🔌️scale/**`.

## 6. Artifacts left in the ticket folder (scratch, per rule 6)

- `terra-fixture-check.txt` — §1 check log.
- `terra-fixture-build.txt` — §1 build log.
- `terra-wgpu-bin-build.txt` — §2 first native-bin build attempt (full output).
- `terra-wgpu-bin-build2.txt` — §2 second native-bin build attempt (`--message-format=short`).
- `terra-floor-plugin-host.txt`, `terra-floor-actor.txt` — §3 regression floor logs.
- `target-bench/` under the session scratchpad (`/private/tmp/.../scratchpad/target-bench`, **not**
  in this ticket folder — that is the `CARGO_TARGET_DIR` rule-4 requires; it holds the built
  `semio_framework_os_scale_fixture.wasm` and the partially-built (failed) renderer-wgpu target,
  reusable by whoever picks this back up once `🖱️ui/**` is green again).

## 7. Next step, for whoever runs this next

Once `semio-framework-ui` compiles again (unrelated live packet's async-conversion sweep needs to
finish and land), the very next command is:

```
CARGO_TARGET_DIR=<target> cargo build -p semio-framework-os-renderer-wgpu --bin semio-wgpu-native --features native-bin
CARGO_TARGET_DIR=<target> cargo run -p semio-framework-os-renderer-wgpu --bin semio-wgpu-native --features native-bin -- \
  --scale 🧰️framework/🛍️products/💻️os/🧫️fixtures/🔌️scale/🤖️generated/🔣️registry.json \
  --scale-wasm <target>/wasm32-wasip2/debug/semio_framework_os_scale_fixture.wasm \
  --shards 8 --report <out>.json
```
The committed registry fixture (2550 records, 354 `cpu`-profile, 387 `idle`-profile) already
comfortably covers the 40-cpu-actor + 1-interactive-actor shape `budget_4_and_5` needs — no
regeneration required. Run it 3+ times and report the spread, per this packet's own measurement
discipline.
