# 🧭️ terra-final-verification — end-to-end status (2026-08-20)

Measurer only. Every number below was actually run this turn; exit codes and counts are pasted, not
recalled. Raw logs live alongside this file (`terra-final-*.txt`) and in the scratchpad census script
`terra-final-census.py`. `CARGO_TARGET_DIR` used the ticket's shared warm dirs in scratchpad
(`target-host` / `target-wu` / `target-wasm`), never the ticket folder.

## Headline: three NEW blockers found this pass, on top of the RED/GREEN state below

1. **`semio-framework-plugin-host` cannot compile at all** — reproducible rustc ICE (internal compiler
   panic), 3/3 runs, not a code-level error.
2. **`semio-framework-plugin` is RED**, 650 errors native / 172 errors wasip2 — a live, uncommitted,
   half-landed `ui_contract` integration (naming mismatch, not this ticket's edit).
3. **`cargo check --workspace --all-targets` is RED** and stops at the very first crate
   (`semio-framework-os-kernel`, 133 errors) — but ONLY under workspace-level feature unification;
   the same crate alone is clean. Because cargo aborts on first failure, **none of the other ~130
   first-party crates were reached by this run** — the workspace check currently carries zero
   information past os-kernel.

None of these three are fixed here — none is "trivial breakage provably safe to fix": (1) is a compiler
bug with unclear root cause, (2) sits in files with live `M`/`MM` git status (someone is mid-edit right
now), (3) is a large multi-hundred-error residue explicitly out of scope for a measurer to repair.

---

## 1. Full test ladder

| crate | command | result | vs baseline |
|---|---|---|---|
| `semio-framework-os-kernel` | `cargo test -p ... --lib` | **EXIT 0 — 779 passed / 0 failed** | ✅ matches 779/0 exactly |
| `semio-framework-os-kernel-db` | `cargo test -p ... --lib` | **EXIT 0 — 424 passed / 0 failed** | ✅ matches 424/0 exactly |
| `semio-framework-plugin-host` | `cargo test -p ... --lib` | **EXIT 101 — cannot compile** (rustc ICE, see §4b) | ❌ was 125/0/1 — now unmeasurable |
| `semio-framework-plugin` | `cargo test -p ... --lib` | **EXIT 101 — 650 errors, cannot compile** | ❌ was 263/5-known — now unmeasurable |

**Confirmed passing total this pass: 1,203** (779 + 424). The other two crates' baselines
(125 for plugin-host, 263 for plugin) cannot be re-measured — both are currently RED, for two
unrelated reasons (§4a, §4c). No test count exists for them right now, not even a degraded one.

Also ran, since `plugin-host --lib` failed before reaching the test binary:
`cargo check -p semio-framework-plugin-host --lib` → same ICE, 3/3 reproductions
(`terra-final-pluginhost-check.txt` / `-check2.txt` / `-check3.txt`).

## 2. Triple-target gate (R14) — `semio-framework-os-kernel`, the framework spine crate

| target | command | result |
|---|---|---|
| native | `cargo check -p semio-framework-os-kernel --lib` | **EXIT 0**, 57 warnings |
| `wasm32-unknown-unknown` | `cargo check -p ... --lib --target wasm32-unknown-unknown` | **EXIT 0**, 55 warnings |
| `wasm32-wasip2` | `cargo check -p ... --lib --target wasm32-wasip2` | **EXIT 0**, 55 warnings |

All three green — matches the briefed "CURRENT VERIFIED STATE." **Caveat:** this is the spine crate,
not the crate that actually carries the WIT guest export surface world-collapse changed.
`semio-framework-plugin --lib --target wasm32-wasip2 --features component-guest` — the crate R14's
own precedent names explicitly — is **RED, 172 errors** (§1). So the triple-target claim holds for
os-kernel but the guest-side crate that world-collapse actually touches cannot currently be gated on
wasip2 at all, for a reason unrelated to the async-runtime work itself.

## 3. Schema gate — CONFIRMED, by direct read of the WIT file

`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧬️schema/📜️component.wit:1094-1101`:

```wit
world actor {
  import pure;
  import host-async;
  export reactor;
  export jobs;
  export checkpoint;
  export describe;
}
```

All 7 exported functions confirmed `async func` by reading each interface body:
`reactor.poll` (1) + `jobs.{start-job, step-job, cancel-job}` (3) +
`checkpoint.{checkpoint, restore}` (2) + `describe.describe` (1) = **7/7 async func**.

`interface runner` and `world actor-async`: **zero declarations** in any `.wit` file repo-wide
(`grep -n '^interface runner\|^world actor-async'` across every tracked `.wit` file) — confirmed
deleted, not merely unreferenced.

## 4. Workspace check — RED, and the failure hides the rest of the workspace

`cargo check --workspace --all-targets` → **EXIT 101**.

**(a) The one crate that failed:** `semio-framework-os-kernel`, **133 errors**, almost entirely
`error[E0277]`/`E0308`/`E0053`/`E0599` of the shape "expected X, found future" — i.e. functions made
`async` by a codemod whose call sites were never given `.await`. Breakdown by file (from the workspace
run's own diagnostics, `terra-final-workspace-check.txt`):

| file | error count |
|---|---:|
| `🏪️store/🔄️sync/🦀️component.rs` | ~127 |
| `📇️directory/🔌️client/🦀️component.rs` | ~6 |

**(b) This is feature-unification-only** — the exact R14/R26 pattern this ticket has hit before.
Standalone `cargo check -p semio-framework-os-kernel --all-targets` (no workspace unification) is
**clean: EXIT 0, 0 errors, 77 warnings** (`terra-final-oskernel-alltargets-standalone.txt`). Whatever
feature some other workspace member enables that pulls these two files into scope was not isolated in
this pass — flagging the mechanism, not the specific culprit crate, for whoever picks this up.

**(c) `semio-framework-plugin-host`'s ICE, separately.** `cargo check -p semio-framework-plugin-host
--lib` panics the compiler **3/3 times**, identically:

```
query stack during panic:
#0 [trait_def] computing trait definition for `protocol::mutation::MutationDiff::apply::{opaque#0}`
panicked at .../rustc_metadata/src/rmeta/decoder/cstore_impl.rs:222:1
error: could not compile `semio-framework` (lib)
```

`cargo check -p semio-framework --lib` **alone** (no plugin-host in the invocation) is clean:
**EXIT 0, 27 warnings** — the documented baseline. The only structural difference found:
plugin-host's `Cargo.toml` pulls `ui_wgpu = {..., features = ["wgpu"]}`, which unifies the `wgpu`
feature into `semio-framework`'s own `ui_wgpu` dependency graph when built alongside plugin-host —
plausible trigger for a different codepath reaching the ICE, **not confirmed** (ran out of budget to
isolate further; flagging the correlation, not claiming causation).

**Consequence:** because `cargo check --workspace` has no `-k`/`--keep-going`, it aborts at the very
first failing crate (os-kernel, topologically early) and **never reaches any of the other ~130
first-party crates in this run** — including plugin-host's separate ICE, which was only found because
this packet also ran it standalone. **The workspace check gives zero information past the first
failure**; do not read "EXIT 101, one crate named" as "only one crate is broken."

## 5. Dropped-future census (R12/R17: `cargo clean -p X` then check, grep `unused implementer of` alone)

| crate | forced-rebuild result | dropped futures |
|---|---|---:|
| `semio-framework-os-kernel` | EXIT 0 | **0** |
| `semio-framework-os-kernel-db` | EXIT 0 | **0** |
| `semio-framework-async` | EXIT 0 | **0** |
| `semio-framework-actor` | EXIT 0 | **0** |
| `semio-framework-os-services` | EXIT 0 | **0** |
| `semio-framework` (bare glue crate) | EXIT 0 | **0** |
| `semio-framework-plugin-host` | **RED (ICE)** | unmeasurable — R17 forbids trusting a census on a red crate |
| `semio-framework-plugin` | **RED (650 errors)** | unmeasurable — R17 |

All 6 crates that could actually be censused this pass are clean of the silent-dropped-future class —
the defect that has produced every real production bug found on this ticket to date. No new instance
found. The two crates most likely to matter for the "does the runtime actually run" question
(plugin, plugin-host) are exactly the two that cannot be censused right now, because they're red.

## 6. Adoption census (static, over 10,868 first-party `.rs` files — method below)

Grep-based (word-boundary regex over file text, not an AST query — the workspace doesn't compile
end-to-end right now so no AST-level count is obtainable). Script: `terra-final-census.py` in
scratchpad, file list from `find … -name '*.rs' -print0` to survive emoji paths safely.

| metric | this pass | sol's baseline | delta |
|---|---:|---:|---:|
| `async fn` | 72,028 | — | — |
| total `fn` | 90,766 | — | — |
| **async ratio** | **79.4%** | **87%** | **−7.6pp** |
| `block_on(` — fleet (`✏️s/🔌️plugins/**`) | **134** | **134** | **exact match** |
| `block_on(` — framework (`🧰️framework/**`) | 826 | (implied 757) | +69 |
| `block_on(` — other paths | 93 | (not split out) | new bucket |
| `block_on(` — **total** | **1,053** | **891** | **+162** |
| `pending_effects` refs | 42 | 29 | +13 |
| `register_job_kind` refs | **13** | **13** | **exact match** |

**Honest caveat on the ratio and totals that moved:** I do not have sol's exact script to diff
methodology against, so I cannot attribute the −7.6pp async-ratio gap or the +162 block_on delta to a
specific cause with confidence. Plausible, non-exclusive explanations: (a) legitimate R9 sync-reversions
landing since the baseline was measured (e.g. `semio-framework-number`'s 384-fn reversion, and possibly
others like it, would show up as exactly this kind of ratio drop — that is R9 working as intended, not
regression); (b) a scope difference between my file list and sol's. Two figures matched **exactly**
(fleet block_on, register_job_kind) which is reassuring that the counting mechanism itself is sound;
the ones that moved should be re-measured against sol's own script before treating the delta as signal.

## 7. Banned symbols

- **`PluginWorkerClient`**: 24 hits repo-wide in tracked, non-generated source. **All 24 are inside
  comments/docstrings** explicitly narrating its own deletion (`kernel/🟦️component.ts`,
  `🎭️actor/🧵️shard-client.ts`, `PluginRuntime/🟦️component.tsx`, `plugin-bridge.ts`, `glue.ts`,
  `index.test.ts`) — permitted under this ticket's own rule ("doc-comment prose naming today's
  concrete choice is fine; identifiers are not"). **Zero live code references.**
- **`exchange`** (TS callable/member): **zero live occurrences** in tracked, non-generated first-party
  source. ~90 hits are `export function exchange(...)` inside **gitignored, auto-generated**
  `.d.ts` interface stubs under `🔌️plugin-modules/*/interfaces/` and `🔌️extension-modules/*/interfaces/`
  (`.gitignore:89: **/🔌️plugin-modules/`) — stale bindings from before world-collapse, not yet
  regenerated for ~70+ dev plugin modules. Signal that the fleet's checked dev artifacts haven't been
  rebuilt against the no-exchange world, not a source-level violation. A handful more hits are inside a
  **closed ticket's** `original-component.ts` "before" reference snapshot (not live code).
- **`interface runner` / `world actor-async`**: zero declarations anywhere (§3).

---

## Honest summary — what is measured, what merely compiles, what was never run

**Actually executed and asserted this pass (real runtime behavior, at least at the unit-test level):**
os-kernel's 779 tests and kernel-db's 424 tests. Both green, both match baseline exactly.

**Compile-checked only (proves absence of a compile error; says nothing about behavior):**
os-kernel's triple-target gate; the schema-gate WIT read (a text read, not even a build); the six
dropped-future censuses in §5.

**Static text census only (no compilation involved at all):** the adoption numbers in §6, the banned
symbol grep in §7 — these can be wrong if a file is dead code, `#[cfg(false)]`'d out, or otherwise
unreachable; they were not cross-checked against what the compiler actually includes.

**Never verified at runtime, in this pass or as far as I can tell in any prior one on this thread:**
whether a real built wasip2 `world actor` component actually completes an end-to-end turn through
wasmtime/jco with the new async-lift ABI; whether any of the 63 fleet plugin crates actually load and
run (their SDK dependency is currently RED, so this is not merely unverified — it is currently
**impossible** to verify); whether plugin-host's `⚡️effects` dispatch fixes from the prior
`host-dropped-futures` packet actually deliver messages under load rather than just compiling and
passing its own mocked unit tests (per this ticket's own rule 7 — a green mock-backed test is not
evidence of the real runtime).

**Bottom line:** the pure-Rust kernel/db spine (os-kernel, kernel-db, and the five other crates
censused in §5) is genuinely solid — real tests pass, real forced-rebuild census is clean, the schema
is exactly as designed. But the plugin runtime itself — the actual subject of this ticket
("MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME") — is **currently not buildable at all**, blocked by three
independent, unrelated issues (a compiler ICE, a live half-landed UI-contract rename, and an
unisolated feature-unification break), none of which existed when the last "CURRENT VERIFIED STATE"
snapshot was taken. Whatever async-runtime correctness work has landed on plugin/plugin-host since
then cannot be re-verified — compiled, tested, or run — until at least one of those three is resolved.

## Raw logs in this ticket folder

`terra-final-oskernel-test.txt` · `terra-final-kerneldb-test.txt` ·
`terra-final-pluginhost-test.txt` · `terra-final-plugin-test.txt` ·
`terra-final-oskernel-native.txt` / `-wu.txt` / `-wasip2.txt` ·
`terra-final-plugin-wasip2.txt` ·
`terra-final-pluginhost-check.txt` / `-check2.txt` / `-check3.txt` · `terra-final-frameworkonly-check.txt` ·
`terra-final-workspace-check.txt` · `terra-final-oskernel-alltargets-standalone.txt` ·
`terra-final-oskernel-forced-census.txt` · `terra-final-kerneldb-forced-census.txt` ·
`terra-final-census-semio-framework-async.txt` / `-actor.txt` / `-os-services.txt` / `-semio-framework.txt`
