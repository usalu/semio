# 🔀️ terra-dyn-http-tail — report

Packet scope: `HttpBody` (7), `HttpTransport` (5), `RouterEffectHandler` (5), `EnvelopeInjector` (2),
`Operator` (4), `OsBackbonePort` (4), plus stragglers. Owned writable paths: `🧰️framework/🛍️products/💻️os/🔨️modules/**`
(named families only), `🌎️hub/**`.

## ✅️ Fixed — `EnvelopeInjector`: generic `EnvelopeCompletionSink<I>` / `AsyncEffectExecutor<I>`

File: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/⚡️effects/🦀️component.rs`.

Census, verified fresh (not inherited):
- `impl EnvelopeInjector for` → **exactly ONE** implementor repo-wide: `RecordingEnvelopeInjector`, a
  test double. The module's own doc already says so: *"No implementation ships in this packet ...
  [`RecordingEnvelopeInjector`] below exists for tests only."*
- `AsyncEffectExecutor::new` / `EnvelopeCompletionSink` → used **only inside this one file**, confirmed
  by a repo-wide grep (`🧰️framework/`, `🌎️hub/`, `✏️s/`) restricted to `struct AsyncEffectExecutor`,
  `AsyncEffectExecutor::new`, `EnvelopeCompletionSink::new`, `EnvelopeCompletionSink>` — every hit is in
  this file; the two other files that mention `AsyncEffectExecutor` by name do so only in doc comments.

This is R11(a)'s trivial case, not R11(b): `inject(&self, envelope: Envelope)` never returns a
runtime-chosen implementation, it just consumes an owned value — and the blast radius is a single file
(well under the ~10-public-type stop-and-report threshold). Applied:

- `EnvelopeCompletionSink<I: EnvelopeInjector>` (field `injector: Arc<I>`, was `Arc<dyn EnvelopeInjector>`).
- `impl<I: EnvelopeInjector> EnvelopeCompletionSink<I>` / `impl<I: EnvelopeInjector> CompletionSink for
  EnvelopeCompletionSink<I>`.
- `emit_completed_ok<I: EnvelopeInjector>` / `emit_completed_err<I: EnvelopeInjector>` (took
  `&Arc<EnvelopeCompletionSink<I>>`).
- `AsyncEffectExecutor<I: EnvelopeInjector>` (field `sink: Arc<EnvelopeCompletionSink<I>>`); its
  `router_handler`/`metrics` fields stay `Arc<dyn RouterEffectHandler>` / `Arc<dyn EffectMetricsRecorder>`
  unchanged — mixed generic + dyn fields on one struct is fine, no rule against it.
- Test helpers `executor()` / `activate()` now name the concrete `AsyncEffectExecutor<RecordingEnvelopeInjector>`;
  every constructor call site (`EnvelopeCompletionSink::new(...)`, `AsyncEffectExecutor::new(...)`, 5 sites
  across the test module) needed **zero** changes — the argument `Arc::new(injector.clone())` where
  `injector: RecordingEnvelopeInjector` already lets rustc infer `I` at each call.
- `dyn EnvelopeInjector` code occurrences: **2 → 0** (repo-wide grep of `🧰️framework/`; the one remaining
  hit is this packet's own doc comment referencing the OLD signature it replaced).

Diff is a single file, `git diff HEAD` confirms nothing else touched:
```
$ git diff --stat HEAD -- "🧰️framework/…/⚡️effects/🦀️component.rs"
 .../⚡️effects/🦀️component.rs | 47 +++++++++++++++-------
 1 file changed, 33 insertions(+), 14 deletions(-)
```

## ⚠️ Verified, unchanged — `HttpBody` / `HttpTransport` / `AsyncHttpTransport` / `StorageBackend`: DELIBERATELY left `dyn`, decision already made and consistent

All four traits are **fully sync** (no `async fn` anywhere on them — confirmed by reading every method
signature) — `dyn` on a sync trait is not an E0038 violation and is R1-legal today. A prior packet
(tag `dedyn-fw-os-misc`) already made and documented the "leave both, deliberately" call for `HttpBody`
(`🛎️services/🦀️component.rs:648-660`) and `HttpTransport` (`🛎️services/🦀️component.rs:681-691`), with
concrete, still-accurate reasons:
- `HttpBody`: real set is one production impl (`BufferedHttpBody`) + one `#[cfg(test)]`-only impl
  (`LocalSocketBody`) implementing the SAME `AsyncHttpTransport::start` trait method — `dyn_enum_close!`
  has no per-variant `#[cfg]` (confirmed empirically elsewhere in the same file family for `VersionGraph`),
  so a test-only variant can't be expressed in one enum declaration.
- `HttpTransport`: its two non-test impls (`UnwiredHttpTransport` here, `UreqHttpTransport` in
  `📇️directory/🔌️client`) span a ONE-DIRECTIONAL crate dependency (`os-kernel` depends on `os-services`,
  not the reverse) — `dyn_enum_close!` needs every implementor nameable from one site, impossible here.
  Generics would force the 60-reference, widely-shared `HttpPool`/`HttpPoolTransport` generic for a trait
  that costs nothing left as `dyn`.

I independently re-verified (not inherited on faith) both traits are still fully sync and both exception
comments still match the code exactly — no drift, not "half done" (neither trait has gained an `async fn`
anywhere; every method still returns a plain `Result`/`HostFuture` from a sync fn signature).

I additionally found and verified two **stragglers** in the same file family, same shape, same decision,
not previously named in this packet's brief:
- `AsyncHttpTransport` (`🛎️services/🦀️component.rs`): sync trait (`start` returns `HostFuture<...>` from a
  plain, non-`async` fn — same shape as `HttpBody::next_chunk`), one production impl
  (`BlockingHttpTransport`), one `#[cfg(test)]`-only impl (`LocalSocketTransport`) — the SAME
  cfg(test)-blocks-`dyn_enum_close!` situation, and it's the trait `HttpBody`'s own exception comment
  already names as the reason `HttpBody` can't collapse to one concrete type. Left `dyn` — consistent.
- `StorageBackend` (`🔌️plugin/🖥️host/⚡️effects/🦀️component.rs`): sync trait (`read`/`write`/`delete` all
  return `Result<...>` directly), doc explicitly says *"same seam discipline as `HttpTransport`"*, one
  production impl (`UnwiredStorageBackend`) + one `#[cfg(test)]`-only impl (`RecordingStorageBackend`).
  Same shape, same call. Left `dyn` — consistent.

**Do NOT read this as "nothing happened here."** I confirmed the decision by rebuilding the census from
scratch (grep every `dyn <Name>` + every `impl <Name> for` occurrence in `🧰️framework/`), not by trusting
the existing comments — they turned out to be accurate.

## ⚠️ Verified, unchanged — `RouterEffectHandler`: DELIBERATELY left `dyn`, consistent with the above

Sync trait (`handle` returns `Result<Vec<u8>, RouterEffectError>` from a plain fn). Real set: one
production impl (`UnwiredRouterEffectHandler`) + two `#[cfg(test)]`-only impls (`RecordingRouterHandler`,
`AlwaysOkRouterHandler`) — same `dyn_enum_close!`-blocked-by-test-only-variants shape. Additionally,
`AsyncEffectExecutor::new` (the only place a `RouterEffectHandler` is consumed) has **zero production call
sites repo-wide** (verified this same census run), so there is no live seam to generic-ize either. Comment
at `⚡️effects/🦀️component.rs:379-388` already states this and is accurate; I did not need to change it.

## 🛑 STOP AND REPORT — `Operator`: left `dyn` deliberately, fix is out of this packet's writable scope

`Operator` is sync (`fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError>`), ONE method,
but **~138 impls measured repo-wide** (`impl Operator for` grep), overwhelmingly in `✏️s/🔌️plugins/**`
(fleet plugin crates — NOT in this packet's owned paths) plus a handful in-scope
(`🌊️flow/📐️brep-geometry`, `🌊️flow/🧩️extensions/🕸️wasm`, `🧠️neural/⚙️engine` itself).

**Why this is not a same-file fix like `EnvelopeInjector`:** `OperatorImpl` (`🧠️neural/⚙️engine/🦀️component.rs`)
has a **public field** `pub operator: Box<dyn Operator>`, constructed via **direct struct-literal syntax**
(`OperatorImpl { schemas: …, operator: Box::new(ConcreteType) }`) at **60+ call sites across ~20 fleet
files**, plus `Registry.operators: HashMap<String, OperatorRecord>` where `OperatorRecord.implementations:
Vec<OperatorImpl>` — the HashMap genuinely needs heterogeneous storage (different operator ids hold
different concrete `Operator` types), so type erasure at the STORAGE layer is structurally required, not
optional. This is `Operator` failing R11(a)'s trivial case for a different reason than `EnvelopeInjector`
did — not "returns an implementation" (R11(b)), but "stored heterogeneously, in a public field, across
dozens of files this packet cannot edit."

**Recommended fix for whoever owns `✏️s/🔌️plugins/**` (or gets a lease granted):**
1. `register_untyped`/`register_typed`/`reg_geo` (`🌊️flow/📐️brep-geometry`) change from
   `operation: Box<dyn Operator>` to a generic `operation: Box<O>` where `O: Operator + 'static` — this
   needs **zero** fleet call-site edits, because every call already passes `Box::new(ConcreteType)`
   inline (verified: no fleet site assigns through an explicit `Box<dyn Operator>`-typed local first), so
   Rust infers `O` from the literal.
2. `OperatorImpl`'s field changes from `pub operator: Box<dyn Operator>` to a PRIVATE
   `operator: Box<dyn Fn(&Dictionary) -> Result<Dictionary, EvalError> + Send + Sync>` (permitted under
   R1 — `dyn Fn` stays legal), with a generic constructor `OperatorImpl::new<O: Operator + 'static>(schemas:
   Vec<String>, operator: O) -> Self` that captures `operator.evaluate(..)` in a closure. This DOES require
   editing every one of the 60+ `OperatorImpl { …, operator: Box::new(X) }` struct literals to
   `OperatorImpl::new(…, X)` — that edit is what needs the lease, since almost all sites are in
   `✏️s/🔌️plugins/**`.
3. `Registry`/`OperatorRecord`/`HashMap<String, OperatorRecord>` need no further change — they already
   only hold `OperatorImpl`, which after step 2 carries no first-party `dyn` at all.

This is a >10-public-type, ~20-file blast radius squarely outside this packet's owned paths — the R11
"stop and report" threshold, not a judgment call to make unilaterally. **Left `Operator` as `dyn`,
unchanged**, `dyn Operator` code count: **4 → 4** (all in-scope: `register_untyped`, `register_typed`,
`reg_geo` params, `OperatorImpl.operator` field).

## ℹ️ Already satisfied elsewhere — `OsBackbonePort`

**Zero occurrences in this packet's owned paths.** The trait lives at
`🧰️framework/🛍️products/💻️os/🖥️host/🦀️component.rs` — sibling to, but NOT under, `🔨️modules/` (out of my
writable scope either way) — and is **already fully closed**: sync trait, blanket `impl<T:
store::BackbonePort> OsBackbonePort for T`, and an `OsBackbonePorts` enum (`Store`/`Space` variants) that
**implements the trait itself**, exactly the `Backbones`/`BackbonePorts` shape `📓️terra-store-dedyn-report.md`
established, written up already in `📓️terra-os-backbone-report.md`. `dyn OsBackbonePort` count in
`🧰️framework/`: **0**.

**Residue found, outside this packet's scope, worth flagging to whoever owns `✏️s/🔌️plugins/🪐️space/`:**
5 `dyn OsBackbonePort` sites remain in `✏️s/🔌️plugins/🪐️space/🦀️component.rs` (`shared_studio_ports`,
`register_studio_port`, `register_studio_port_for_test`), with an existing comment explaining why:
*"`open_folder_space_backbone`/`open_file_space_backbone`... returning `Arc<dyn OsBackbonePort>` directly,
already type-erased before this file ever sees the value."* **That blocker is now stale** — I read
`🖥️host/🦀️component.rs:2008-2014` directly and both functions already return
`Result<Arc<OsBackbonePorts>, VcsError>`, the concrete enum, not `Arc<dyn OsBackbonePort>`. The fix the
fleet packet's own report asked for has already landed upstream; the 5 remaining fleet-side `dyn` sites
could now be closed by simply changing `Arc<dyn OsBackbonePort>` → `Arc<OsBackbonePorts>` in
`✏️s/🔌️plugins/🪐️space/🦀️component.rs` — a fleet-owned file this packet cannot edit.

## 🚨 Required gates

```
$ CARGO_TARGET_DIR=.../target-dyn-http-tail cargo check -p semio-framework-os-kernel --lib
warning: `semio-framework-os-kernel` (lib) generated 417 warnings (9 auto-fixable; all pre-existing —
unawaited-Future lints in files this packet never touched, e.g. 🏪️store, 📡️spr/🧵️channel)
    Finished `dev` profile [unoptimized] target(s) in 1m 28s
EXIT:0

$ CARGO_TARGET_DIR=.../target-dyn-http-tail cargo check -p semio-framework --lib
warning: `semio-framework` (lib) generated 28 warnings (pre-existing R7 async_fn_in_trait warnings)
    Finished `dev` profile [unoptimized] target(s) in 1m 05s
EXIT:0
```

Both required gates confirmed green, fresh measurement, full paste above (commands run from repo root,
`CARGO_TARGET_DIR` elided for width — full value:
`/private/tmp/claude-501/-Users-ueli-Documents-semio/e6a44461-bab7-421f-8a53-65123a5e9482/scratchpad/target-dyn-http-tail`).

## ⚠️ Could NOT get a clean build of my own edited crate — confirmed unrelated, not a regression

`semio-framework-plugin-host --lib` (the crate containing my one edited file) fails to compile, but the
failure is **entirely upstream and unrelated**:

```
$ CARGO_TARGET_DIR=.../target-dyn-http-tail cargo check -p semio-framework-plugin-host --lib
error: could not compile `semio-framework-actor` (lib) due to 266 previous errors
warning: build failed, waiting for other jobs to finish...
EXIT:101
```

Every one of the 266 errors is in `semio-framework-actor` (`🧰️framework/🔨️modules/🎭️actor/`) — NOT a
family in this packet's scope, NOT a file this packet touched. All 266 are the missing-`.await` shape
(`the ? operator can only be applied to values that implement Try`, `impl Future<Output=T>` where a `T`
is expected, etc.) — the signature of an in-progress, not-yet-await-fixed asyncification, matching this
ticket's own documented pattern for mid-flight async conversions. I checked whether this was caused by my
edit or by uncommitted local changes:

```
$ git diff --stat HEAD -- "🧰️framework/🔨️modules/🎭️actor/"
(empty)
```

`git diff` against `HEAD` is EMPTY for that whole crate — meaning the currently on-disk `semio-framework-actor`
is **exactly the last-committed state**, broken as committed, not a WIP edit sitting under a concurrent
agent's hands right now. This is very likely a mid-flight async-conversion packet that auto-committed
between its await-insertion passes (matches the documented "auto-commit + concurrent devs" hazard).
**Not my bug, not my file, not my scope** — re-verified by grepping the full error log for my own symbols
(`EnvelopeCompletionSink`, `AsyncEffectExecutor`, `EnvelopeInjector`, `⚡️effects`): **zero hits**, and only
one crate name appears in any `error: could not compile` line: `semio-framework-actor`.

I re-ran the same check a second time (a few minutes later) to see if it was mid-fix by another agent —
still EXIT 101, same 266-error shape, same single failing crate. Not chasing further: it's outside my
owned paths and the two REQUIRED gates (`os-kernel`, `semio-framework`) are unaffected and green, which is
the standing instruction. **Flagging for the coordinator / whoever owns `🎭️actor`**: `semio-framework-actor
--lib` is currently red at HEAD (266 errors, all missing-`.await`), which will block every downstream
crate — `semio-framework-plugin-host` and its dependents — from a clean `--lib` check until fixed.

I could not get a full `--lib` compile confirmation of my own single-file diff as a result, but: (a) the
diff is mechanical (add one type parameter, thread it through 8 signatures, update 2 test-helper return
types — no logic changed), (b) I hand-verified every call site of the touched types compiles by inspection
(all 5 test constructor calls infer `I = RecordingEnvelopeInjector` from `Arc::new(injector.clone())`
where `injector: RecordingEnvelopeInjector`, requiring no call-site edits), and (c) I will re-run
`cargo check -p semio-framework-plugin-host --lib` once `semio-framework-actor` is fixed and update this
report rather than claim a pass I did not measure.

## Files touched

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/⚡️effects/🦀️component.rs` — `EnvelopeCompletionSink`
  → `EnvelopeCompletionSink<I: EnvelopeInjector>`, `AsyncEffectExecutor` → `AsyncEffectExecutor<I: EnvelopeInjector>`,
  `emit_completed_ok`/`emit_completed_err` genericized, 2 test-helper signatures updated.

## Dyn count summary (code occurrences only, doc comments excluded)

| family | before | after | disposition |
|---|---:|---:|---|
| `EnvelopeInjector` | 2 | **0** | fixed — generic `<I: EnvelopeInjector>` |
| `HttpBody` | 7 | 7 | verified, deliberately left (sync trait, documented) |
| `HttpTransport` | 5 | 5 | verified, deliberately left (sync trait, documented) |
| `AsyncHttpTransport` (straggler) | 2 | 2 | verified, deliberately left (sync trait, same shape) |
| `StorageBackend` (straggler) | 2 | 2 | verified, deliberately left (sync trait, same shape) |
| `RouterEffectHandler` | 5 | 5 | verified, deliberately left (sync trait, documented) |
| `Operator` | 4 | 4 | stop-and-report — fix needs fleet-owned files, out of scope |
| `OsBackbonePort` | 0 (in-scope) | 0 | already closed elsewhere (`🖥️host`, out of scope) |
