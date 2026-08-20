# 📓️ terra-e2e-gate — Program Scorecard (measurement only, no source touched)

All commands run **fresh**, in the foreground, this session, on 2026-08-19/20, with
`CARGO_TARGET_DIR=/private/tmp/claude-501/-Users-ueli-Documents-semio/e6a44461-bab7-421f-8a53-65123a5e9482/scratchpad/target-e2egate`
(per rule 24 — never the ticket folder). Raw logs are in the same scratchpad dir
(`oskernel-lib.txt`, `oskernel-alltargets.txt`, `framework-lib.txt`, `framework-alltargets.txt`,
`sdk-lib.txt`, `ui-wgpu.txt`, `schema.txt`, `test-*.txt`) and are referenced below with exit codes
pasted directly from the terminal (never `| tail`).

---

## 1. Compile state

| target | command | exit | errors | warnings |
|---|---|---:|---:|---:|
| `semio-framework-os-kernel` `--lib` | `cargo check -p semio-framework-os-kernel --lib` | **0** | 0 | 79 |
| `semio-framework-os-kernel` `--all-targets` | `cargo check -p semio-framework-os-kernel --all-targets` | **101** | **1553** | 20 (lib test) |
| `semio-framework` `--lib` | `cargo check -p semio-framework --lib` | **0** | 0 | 109 |
| `semio-framework` `--all-targets` | `cargo check -p semio-framework --all-targets` | **0** | 0 | (109, no new) |
| **`semio-framework-plugin` `--lib`** (SDK, headline) | `cargo check -p semio-framework-plugin --lib` | **101** | **156** | 7 |
| `semio-framework-ui` `--lib --features wgpu` | `cargo check -p semio-framework-ui --lib --features wgpu` | **0** | 0 | — |
| `semio-framework-schema` | `cargo check -p semio-framework-schema` | **0** | 0 | — |
| `semio-framework-replication` test | `cargo test -p semio-framework-replication` | **0** | — | 184 passed / 0 failed |
| `semio-framework-pack` test | `cargo test -p semio-framework-pack` | **0** | — | 44 passed / 0 failed |
| `semio-framework-geometry` test | `cargo test -p semio-framework-geometry` | **0** | — | 57 passed / 0 failed |
| `semio-framework-math` test | `cargo test -p semio-framework-math` | **0** | — | 191 passed / 0 failed |
| `semio-framework-async` test | `cargo test -p semio-framework-async` | **0** | — | 17 passed / 0 failed |
| `semio-framework-dispatch-macros` test | `cargo test -p semio-framework-dispatch-macros` | **0** | — | 22+3+1+1+1 passed / 0 failed (5 test binaries) |

🚨 **`os-kernel --lib` and `semio-framework --lib`+`--all-targets` are BOTH confirmed still `EXIT 0` — no regression.** This matches the mission-statement claim.

**SDK (`semio-framework-plugin --lib`) is STILL RED — 156 errors, not 0.** This is real, measured
progress (the mission statement's own snapshot was 1,845 → 339; I now measure **339 → 156**, a further
54% cut since that snapshot), but it is not the green the brief conditions the fleet-unblock check on.
Per the brief's own instruction ("If the SDK is green, also run `cargo check -p semio-s-plugin-note
--lib`"), **that fleet-crate check was correctly skipped — it is still gated.** The remaining 156 errors
are dominated by `Send`-bound futures being awaited across a `Pin<Box<dyn Future<...> + Send>>` cast
(sample, `sdk-lib.txt` tail: `component.rs:625` — `bytes.await` inside a non-`Send` future being coerced
to a `Send`-bound boxed future), i.e. the R3 Send-boundary work is not finished in this crate.

`os-kernel --all-targets`: **1553 errors**, dominated by the same `E0733` self/mutual-recursion-needs-
`Box::pin` shape the `alltargets-hard` sibling was fixing by hand (e.g. `🏪️store/🦀️component.rs:3998`
`encode_op` calling itself recursively without indirection — confirmed in the raw log tail).

---

## 2. Censuses (python3, absolute paths, comments/strings stripped — see §5 for method notes)

Scripts: `/private/tmp/claude-501/.../scratchpad/e2egate-dyn_census.py` (adapted from the ticket's own
`terra-sdk-gate-census-dyn_census.py`, output re-pointed to my scratchpad),
`e2egate-banned-census.py` + `e2egate-banned-refine.py` (new, this packet).

### 2a. First-party `dyn`

| | was (mission statement) | measured now |
|---|---:|---:|
| **first-party `dyn` total** | 576 → 84 | **57** |
| framework | 79 | **52** |
| fleet (`✏️s`) | 5 | **5** (unchanged) |
| std/lang `dyn` (`Fn`/`FnMut`/`FnOnce`/`Future`/`Any`/`Error`, R1-legal) | — | **135** (`Fn` 50, `Future` 27, `FnMut` 23, `Any` 14, `Error` 9, `FnOnce` 12) |

By-trait (all 26 remaining first-party traits with `dyn` uses, framework+fleet):
`HttpBody` 7 · `HttpTransport` 5 · `RouterEffectHandler` 5 · `Operator` 4 · `OsBackbonePort` 4 ·
`BackboneTransport` 2 · `CapabilityChecker` 2 · `StorageBackend` 2 · `EffectMetricsRecorder` 2 ·
`ThreadSpawner` 2 · `QuerySource` 2 · `BlobStore` 2 · `AsyncHttpTransport` 2 · `ToolRegistry` 2 ·
`ResourceRegistry` 2 · `PromptRegistry` 2 · `MediaCache` 1 · `ConflictOracle` 1 · `Signer` 1 ·
`SignatureVerifier` 1 · `CompletionSink` 1 · `DynEngine` 1 · `Backbone` 1 · `MeshExporter` 1 ·
`MeshImporter` 1 · `SpaceBackbonePort` 1.

This **corroborates** two siblings independently: `dyn-http-tail`'s stop-and-report on `Operator` (4
sites, public-field, ~138 impls, outside its writable scope) and its note on 5 residual
`OsBackbonePort` sites in `✏️s/🔌️plugins/🪐️space/🦀️component.rs` — both traits still show up at exactly
the counts their report implied are outstanding.

### 2b. Async ratio

| | was | measured now |
|---|---:|---:|
| `async fn` | — | **67,034** |
| plain `fn` | — | **9,736** |
| **ratio** | 86.7% | **87.32%** |
| `🚫️async:` tags | ~337+ | **413** (E1 209, E4 125, E5 19, E3 60) |

Both figures moved in the expected direction (more async, more tags) since the mission-statement
snapshot — consistent with, not contradicting, the eleven siblings' claimed work.

---

## 3. Banned symbols — source vs generated, comments/strings excluded

Two-pass method: pass 1 (`e2egate-banned-census.py`) does a naive word-boundary scan (catches
everything, including doc-comment prose); pass 2 (`e2egate-banned-refine.py`) strips `//` line
comments and string literals before matching, isolating **real code identifiers**. Per rule 21, every
non-zero finding below was spot-checked against the raw file a second way (direct line read, not
grep-over-emoji-paths).

| symbol | raw hits (source) | **code-only** (source) | **code-only** (generated) | verdict |
|---|---:|---:|---:|---|
| `exchange` | 234 | **119** | 26 | 🚨 **STILL LIVE — see below** |
| `PluginWorkerClient` | 8 | 0 | 0 | clean (comment-only mentions) |
| `LeasePool` | 10 | 2 | 0 | clean — both hits are the **sanctioned** relocation at `📦️packages/🟦️typescript/🟦️glue.ts` |
| `PluginModuleLease` | 6 | 0 | 0 | clean (comment-only) |
| `createLeasePool` | 6 | 4 | 0 | clean — all 4 at the sanctioned `🟦️glue.ts` location (`important.md`: "relocates to `📦️packages/🟦️typescript/🟦️glue.ts` for its 3 non-plugin users") |
| `WasmPluginRuntime` | 37 | 0 | 0 | clean (comment-only — mostly doc-comments in `🏃️run/component.rs`/`plugin/host/component.rs` narrating the migration) |
| `ExtensionRuntime` | 2 | 0 | 0 | clean |
| `ProgramSupervisorState` | 1 | 0 | 0 | clean |
| `PLUGIN_FUEL_BUDGET` | 3 (+1 generated) | 0 | 0 | clean |
| `PLUGIN_WORKER_UNRESPONSIVE_MS` | 0 | 0 | 0 | clean |
| `INSTANCE_GUARD` / `clear-instance-guard` | 1 / 1 | 0 | 0 | clean |
| `host_port` | 9 | 1 | 0 | **false positive** — the 1 hit (`Shell/🧊️component.rs:2293-2295`) is an unrelated local variable from `str::split_once` parsing a `"host:port/space"` URL string, not the banned global. Confirmed by direct read. |
| `install_io_fallback_dispatcher` | 0 | 0 | 0 | clean |
| `set_host_backbone_channel` | 6 | 0 | 0 | clean (comment-only) |
| `runSerialized` | 3 | 0 | 0 | clean (comment-only) |
| `loadPluginModuleUncached` | 2 | 0 | 0 | clean (comment-only) |
| `component::host_*` (pattern) | 0 / 0 | 0 | 0 | clean |

### 🚨 `exchange` is NOT closed — confirmed, real, in production code

`🧰️framework/🛍️products/💻️os/🔨️modules/🏃️run/🦀️component.rs` still declares and calls a live
`exchange` method:

```
133:  async fn exchange(&mut self, ctx: &OperationContext, node: u32, commands: Vec<AppCommand>) -> Result<Vec<AppFrame>, RunError>;
1086:     let frames = self.host.exchange(&ctx, handle, commands).await?;
1940:  async fn exchange(&mut self, _ctx: &OperationContext, node: u32, commands: Vec<AppCommand>) -> Result<Vec<AppFrame>, RunError> {
2105:      async fn exchange(&mut self, _ctx: &OperationContext, node: u32, commands: Vec<AppCommand>) -> Result<Vec<AppFrame>, RunError> {
```

This directly contradicts `important.md`'s "Replace, never wrap" list: `exchange` (WIT + all callers)
"must not exist at exit." `exec:run-kernel-wiring`'s own summary is consistent with this — it describes
giving `exchange()`'s passthrough branch "a real body," i.e. it explicitly worked *inside* the still-
extant `exchange` method rather than replacing it. **Not a contradiction between siblings — a genuinely
unfinished item that no packet in this wave claimed to close**, but it should be flagged loudly since
the binding rule says it must not exist at exit and it demonstrably does, in the crate that is the
native run path. (`🔌️plugin/🖥️host/🦀️component.rs:3531,3609,3639` also has a local closure parameter
literally named `exchange` — different thing, a `FnMut` callback param, not the banned method, but same
name and worth a rename for grep-ability once the real one is gone.)

---

## 4. Flagged discrepancies between sibling claims and my measurement

1. **`alltargets-hard` claimed `os-kernel --all-targets` at 1545 errors (its "final, fresh" number,
   down from 2746).** I measure **1553** — 8 more. Within the noise this ticket has repeatedly
   documented as concurrent-churn on `🏪️store`/`🗣️dsl`/`📡️spr` (multiple other packets this same wave —
   `dyn-emit-runtime`, `run-kernel-wiring`, `extension-activation` — independently reported the identical
   crates fluctuating under them from unrelated live edits). Not a regression I can attribute to any
   named packet; flagging the delta rather than asserting either number is "right."
2. **`exec:parity-rebaseline`'s claimed blocking regression — independently reproduced.** I traced the
   full import chain by hand (not trusting the sibling's claim): `ShellHost/🟦️component.tsx:137-138`
   imports `mutationEnvelopeFromWire`/`mutationEnvelopeToWire` from `@semio-tech/framework-os`, whose
   package.json `exports["."]` points at `📦️packages/🟦️typescript/🟦️glue.ts`, which does
   `export * from "../../🟦️component.ts"`. `🟦️component.ts` imports those two names from
   `@semio-tech/framework-replication` for its own internal use (lines 24, 1285, 1431) but never
   re-exports them — a plain `import` has no effect on `export *`'s surface. **Confirmed: the react-side
   package's public surface is genuinely missing these two names; this blocks 100% of `ShellHost`
   consumers, i.e. every react-side dev boot,** exactly as the sibling reported. This is a real,
   live, unresolved blocker, not a stale claim — and it sits in `🧰️framework/🛍️products/💻️os/🟦️component.ts`,
   which is **not** one of the registrar-only files (`Shell/🧊️component.rs` and
   `ShellHost/🟦️component.tsx` are registrar-only; the plain `💻️os/🟦️component.ts` is not), so it is
   fixable by any packet without a lease-request once someone is assigned it.
3. **Mission-statement dyn figure (84) vs measured (57)** and **SDK-error figure (339) vs measured
   (156)**: both are real forward progress since the snapshot, not discrepancies — noted so the next
   packet doesn't re-measure from a stale baseline.
4. **`luna-exit-audit`'s "TS Rust must-not-exist symbols: CLEAN"** — my independent, comment-stripped,
   source-vs-generated banned-symbol census **agrees** for every symbol except `exchange`, which that
   audit's own summary (`luna-exit-audit.md`, not re-read in full here) is not quoted as having flagged
   by name in the excerpt available to me. Recommend the coordinator confirm whether `luna-exit-audit`
   checked `exchange` specifically — my measurement says it is not clean.

---

## 5. Method notes (so a future packet trusts or distrusts this correctly)

- Dyn/async census reused the ticket's existing, previously-verified `dyn_census.py` verbatim (only the
  two output paths were repointed into my own scratchpad) rather than writing a new one — it already
  handles raw strings, block comments, escaped quotes and line-continuation correctly (see its own
  in-file docstrings recording three prior corruption bugs it fixed). I did not re-invent this wheel.
- The banned-symbol refine pass (`e2egate-banned-refine.py`) is new this packet and uses a **cruder**
  block-comment strip (`re.sub(r'/\*.*?\*/', ' ', src, flags=re.S)`) that can desync line numbers across
  files containing multi-line block comments — confirmed this bug directly (the `host_port` false-
  positive's reported line number, 1911, did not match its real line, 2293, when re-read raw). **The
  aggregate counts are unaffected** (the mismatch is only in which line number gets attributed to a
  hit), but every non-zero hit above was re-verified against a **direct, unstripped read of the real
  file** before being reported as real or false — never trusted from the refine pass alone.

---

## 6. Plain-language end-to-end readiness verdict

**What works, measured, right now:**
- The framework core is solid: `semio-framework` (`--lib` and `--all-targets`) and
  `semio-framework-os-kernel --lib` are all green, with zero errors. Six independent test crates
  (`replication`, `pack`, `geometry`, `math`, `async`, `dispatch-macros`) all pass, zero failures.
- The de-dyn program is genuinely near its finish line: 57 first-party `dyn` sites left (was 576 at
  program start), concentrated in ~10 traits with known, already-diagnosed dispositions (public-field
  `Operator`, stale-blocker `OsBackbonePort`, R11 open-set `HttpBody`/`HttpTransport`/
  `RouterEffectHandler`/`AsyncHttpTransport`). Async-literal adoption is at 87.3% with 413 documented
  exceptions.
- wgpu-feature UI (`semio-framework-ui --lib --features wgpu`) and `semio-framework-schema` both check
  clean under the consumer's actual feature set (not just the bare crate).

**What is blocked, in priority order:**
1. **The guest SDK (`semio-framework-plugin --lib`) is still red at 156 errors**, almost entirely a
   Send-boundary mismatch (a guest-side `?Send` future being coerced into a host-side `Pin<Box<dyn
   Future + Send>>`). Until this is 0, none of the 63 fleet crates can build, so **no real guest wasm
   exists to load into the pool on either renderer** — this is still the single highest-leverage blocker
   in the whole program, exactly as the mission statement says, just measurably closer (339→156 this
   session).
2. **React-side dev boot is 100% blocked by a missing re-export** (§4.2 above): two names imported by
   `ShellHost` are absent from `@semio-tech/framework-os`'s public surface because `🟦️component.ts`
   imports-but-doesn't-export them and `🟦️glue.ts`'s `export *` can't surface what was never exported.
   This is a **one-line-class fix** (add the two names to `🟦️component.ts`'s own export surface, or
   re-point the two imports in `ShellHost` straight at `@semio-tech/framework-replication`) but it sits
   outside this packet's scope (measurement-only) and is not yet claimed by any packet.
3. **`exchange` (the banned pre-collapse ABI method) is still live and called** in
   `🏃️run/🦀️component.rs` — the native run path has not actually completed its migration off the old
   `exchange`-shaped API despite `run-kernel-wiring`'s work landing real bodies *inside* it.
4. `os-kernel --all-targets` carries 1553 errors, dominated by the same "recursive async fn needs
   `Box::pin`" shape (R10 residue-class 3) the `alltargets-hard` packet was already chipping at by hand
   in `store`/`dsl`/`spr` — this blocks `cargo test` for the kernel crate and therefore any native
   integration test of a real turn.

**Ordered shortest path to a first real plugin turn:**
- **Browser:** (a) close the SDK's remaining 156 Send-boundary errors → unblocks all 63 fleet crates
  including `stdio` (the universal Wave-0 dependency per `luna-fleet-readiness`); (b) rebuild at least
  one plugin (`stdio` or the smallest fixture) to real wasm; (c) fix the `mutationEnvelopeFromWire`/
  `mutationEnvelopeToWire` export gap so `ShellHost` boots at all; (d) exercise the already-wired
  `ShardClient`/`ActivationRegistry` path (`luna-e2e-path` confirms this is 70% wired) against the real
  wasm instead of a protocol stub (closing `bench-web-rows`'s two `pass-stub-worker` budgets).
- **Native:** (a) same SDK closure as above; (b) finish removing `exchange` from `🏃️run/🦀️component.rs`
  so the native run path is actually on the new ABI, not just wrapping the old one; (c) clear the 1553
  `os-kernel --all-targets` errors (mechanical `Box::pin` residue, per `luna-runtime-audit`'s Phase 1/2
  characterization of the async world as still fundamentally unmounted); (d) wire a genuine
  `execute_turn` async path per `luna-runtime-audit`'s ~3-week Phase 0–2 estimate.

Neither path reaches a first real plugin turn without the SDK going green first — that remains the
single serializing bottleneck for the whole program.
