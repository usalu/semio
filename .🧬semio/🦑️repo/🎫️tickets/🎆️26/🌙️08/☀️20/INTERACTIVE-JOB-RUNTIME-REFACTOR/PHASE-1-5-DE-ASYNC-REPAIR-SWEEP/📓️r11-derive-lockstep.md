# R11 — De-Async Repair: `semio-framework-machine-derive` Lockstep + Final Phase Gate

Packet R11 of Phase 1.5, the final repair packet. Ownership boundary:
`semio-framework-machine-derive` only (`🧰️framework/🔨️modules/🔄️machine/✨️derive/🦀️component.rs`).
Closes the cross-boundary hand-off R8 flagged and could not make from inside its own
boundary, then runs the phase-wide gate.

## 1. Trait/generated-method correspondence — verified, not trusted

R8's claim ("mechanical, no body changes needed") was checked against the actual current
trait declarations in `🧰️framework/🔨️modules/🔄️machine/🦀️component.rs` before touching
anything:

```rust
pub trait StatechartEvent: Clone {
    const EVENT_COUNT: u16;
    fn event_id(&self) -> EventId;                    // sync
    fn event_name(id: EventId) -> &'static str;        // sync
}
pub trait Machine: Sized + 'static {
    ...
    fn definition() -> &'static MachineDefinition<Self>;  // sync
}
```

Both traits are fully sync (§2117–2159 of that file, post-R8). Signatures (params,
return types, generics) match exactly what the derive crate's generated `impl` blocks
already produced, modulo `async`. `grep -n ".await" ✨️derive/🦀️component.rs` returned
zero matches both before and after the edit — confirming R8's "no nested `.await`"
claim independently rather than trusting it.

**Edit:** dropped `async` from all 5 generated methods:

- `emit()` (`impl machine::StatechartEvent for #event_name`): `fn event_id`,
  `fn event_name`.
- `emit()` (`impl machine::Machine for #marker_name`): `fn definition`.
- `expand_statechart_event()` (`impl machine::StatechartEvent for #name`):
  `fn event_id`, `fn event_name`.

Purely mechanical — no body changed, matching R8's prediction exactly.

## 2. `export_wasm_machine!` — three bugs, all confirmed against generated code first

**Bug 1, "`.await` on now-sync kernel calls":** did not exist as literal text. A repo
grep for `.await` in the derive crate's source returned nothing, before or after. The 7
generated inherent methods (`new`/`send_json`/`tick`/`snapshot_json`/`restore_json`/
`manifest_json`/`on_effect`) call sync kernel functions (`machine::init`,
`machine::macrostep`, `machine::route_command`, `machine::timer_elapsed`,
`machine::persist`, `machine::restore`, `Machine::definition()`) directly without
`.await` — that was never wrong *syntactically* (calling a sync fn from an `async fn`
body needs no `.await`), so it never produced a compiler error by itself. The 5-error
lib-level failure on wasm32 was the `StatechartEvent`/`Machine` E0053 trio (fixed by §1,
propagating through `wasm_smoke::toggle`'s macro expansion) plus the two independent bugs
below — not a missing-`.await` class at all in this block. Dropped `async` from all 7
methods anyway, since they wrap now-sync bodies and being `async fn` was itself the root
cause of bug 3.

**Bug 2, `restore::<M>(...)` missing its `Mg` generic (E0107):** confirmed
`machine::restore`'s real signature —
`pub fn restore<M: Machine, Mg: Migration>(persisted: &PersistedSnapshot, context: M::Context, migrations: &[&Mg]) -> Result<Snapshot<M>, RestoreError>`
— against the generated call site (`✨️derive/🦀️component.rs:1292`, inside
`restore_json`). Followed R8's suggested default (matching what the crate's own
`checkout_integration`/native tests already do): defaulted to `machine::NoMigrations`,
which is re-exported at the crate root the generated code already resolves `machine::`
through (`extern crate self as machine;` in `📦️glue.rs`). Fix:
`machine::restore::<#machine_path, machine::NoMigrations>(&persisted, context, &[])`.

**Bug 3, `wasm_bindgen_futures` (E0433):** the literal string `wasm_bindgen_futures`
does **not** appear anywhere in `✨️derive/🦀️component.rs`, nor in either Cargo.toml —
confirmed by grep before concluding anything. The dependency was implicit: `#[wasm_bindgen]`
attached to an `impl` block containing `pub async fn` methods (all 7 were `async fn`
before this edit) makes `wasm-bindgen`'s own proc-macro emit code that bridges the
`Future` to a JS `Promise` via `wasm_bindgen_futures::future_to_promise` internally —
never visible in this crate's own source, only in wasm-bindgen's macro output — which is
why the crate's `Cargo.toml` (`wasm-bindgen`/`js-sys` only under
`[target.'cfg(target_arch = "wasm32")'.dependencies]`) fails to resolve it. Per the
dependency-freeze ratchet, `wasm_bindgen_futures` was **not** added. Instead, dropping
`async` from all 7 methods (§ above, matching the direction "pure CPU wrapper, no genuine
suspension") means `#[wasm_bindgen]` no longer emits the futures-bridge at all — the
dependency was never architecturally necessary once the wrapped kernel calls are sync,
exactly as R8 predicted. Confirmed by rerunning `bun ./📜️script.ts verify dependencies`
after the fix: still 238/238, no new dependency introduced.

## 3. Fixture status

Neither `checkout_integration` (native, `#[cfg(test)]`) nor `wasm_smoke::toggle`
(`#[cfg(target_arch = "wasm32")]`) carried any `#[cfg(any())]` or other disabling
attribute at the start of this packet — grepped both files to confirm before starting.
With the macro fixed, both now compile and run for real, no exclusion left in place:

- `cargo test -p semio-framework-machine` (debug): **31/31 pass** (was 23/23 with
  `checkout_integration`'s 8 tests probe-excluded by R8; all 8 now included and passing).
- `cargo test -p semio-framework-machine --release`: **31/31 pass**.
- `wasm_smoke::toggle` compiles cleanly on both `wasm32-unknown-unknown` and
  `wasm32-wasip2` (verified via `--all-targets` on each; it is compile-only, not test-gated,
  so there is no runtime assertion beyond "expands and type-checks").

## 4. Blast radius — verified independently, not trusted

`cargo metadata` dependency graph: exactly one package (`semio-framework-machine`)
depends on `semio-framework-machine-derive`. Repo-wide grep for
`export_wasm_machine!`/`derive(StatechartEvent)`/`derive(StatechartSchema)`/
`statechart!` call sites (excluding `./compose` and `target/`) found only
`🧰️framework/🔨️modules/🔄️machine/🦀️component.rs` (self, both fixtures) and the draw-fsm
plugin's own **separate** macro crate (`✏️.../🔄️fsm/✨️macros/📦️packages/🦀️rust/📦️glue.rs`),
confirming R8's finding that draw-fsm is a sibling, not a reuse. Change fully contained.

## 5. Files touched

- `🧰️framework/🔨️modules/🔄️machine/✨️derive/🦀️component.rs` — dropped `async` from 5
  generated `StatechartEvent`/`Machine` methods (`emit()`, `expand_statechart_event()`)
  and 7 generated `export_wasm_machine!` inherent methods; added the missing `Mg`
  generic (`machine::NoMigrations`) to the generated `restore::<...>(...)` call.
  Reformatted with `rustfmt` (explicit path only). No other file touched.

## 6. Verification actually run

- `cargo check -p semio-framework-machine-derive --all-targets` — **0 errors**
  (was already 0; unaffected, confirmed still clean).
- `cargo check -p semio-framework-machine --all-targets` — **lib: 0 errors, lib test: 0
  errors** (was lib 0 / lib test 3).
- `cargo test -p semio-framework-machine-derive` — **10/10 pass** (unaffected).
- `cargo test -p semio-framework-machine` (debug) — **31/31 pass** (was 23/23 with
  `checkout_integration` probe-excluded by R8; now fully included).
- `cargo test -p semio-framework-machine --release` — **31/31 pass**.
- `cargo check -p semio-framework-machine --target wasm32-unknown-unknown --all-targets`
  — **0 errors** (was 5 lib / 8 lib-test).
- `cargo check -p semio-framework-machine --target wasm32-wasip2 --all-targets` —
  **0 errors** (was 5 lib / 8 lib-test; confirms the fix is target-arch-general, not
  `wasm32-unknown-unknown`-specific, matching R8's original observation about the bug).
- `bun ./📜️script.ts verify dependencies` — **238/238, clean**, no new third-party
  dependency (confirms `wasm_bindgen_futures` genuinely was not needed).

### Final phase gate

`cargo check --workspace --all-targets --exclude semio-compose-rs 2>&1 | grep "could not compile"`,
run twice to check for the known reachability non-determinism (identical both times):

```
error: could not compile `semio-framework-hash` (lib test) due to 29 previous errors
error: could not compile `semio-compose-rs` (lib) due to 18 previous errors; 89 warnings emitted
```

This is **not** empty, contrary to the phase's stated exit condition. Both lines are
explained, and neither is this packet's to fix:

- **`semio-compose-rs`**: explicitly out-of-scope per this packet's instructions ("its 18
  errors are expected and must be left alone"). It still appears in the `--exclude
  semio-compose-rs` output because `--exclude` only removes it from the top-level
  check-target selection, not from the dependency graph — `semio-compose-gql` and
  `semio-compose-query` (confirmed via `cargo metadata`) both depend on it as a path
  dependency, so `cargo` still has to compile it to build those two crates. There is no
  `--exclude`-only way to silence this; it would need those two dependents excluded too,
  which was not authorized. Expected, unchanged, matches the 18-error count stated in
  the task brief exactly.
- **`semio-framework-hash`** (29 errors, `lib test` target): **new to this run**, not
  present in the packet's stated starting state ("exactly ONE in-scope failure:
  `semio-framework-machine`"). Confirmed via `cargo metadata` that this crate has zero
  dependency relationship to `semio-framework-machine`/`-derive` (its only dependency is
  `blake3`) — so this packet's edits cannot have caused it. Confirmed via `cargo check -p
  semio-framework-hash --all-targets` in isolation: same error shape (`async functions
  cannot be used for tests`, `impl Future<Output = String>` mismatches) — the identical
  never-suspends `async fn` bug class this whole phase has been repairing, just in a
  crate no packet has owned yet. This matches the phase's own documented pattern (R3 §2:
  fixing one crate can newly unmask a previously-unreached one via build-reachability,
  observed reproducibly here across two consecutive runs with identical counts — stable,
  not the old multi-crate instability). **Flagged for the coordinator as a new,
  self-contained follow-up packet** (`semio-framework-hash`, `lib test` target, ~29
  errors, same bug class, same mechanical fix pattern as every prior packet) — not fixed
  here, since it is entirely outside this packet's stated ownership boundary
  (`semio-framework-machine-derive` only) and touching it was not authorized.

Within R11's own explicit scope — `semio-framework-machine` and
`semio-framework-machine-derive` — **both crates are now fully clean on every checked
target** (native lib, native lib test, `wasm32-unknown-unknown`, `wasm32-wasip2`, debug
and release test runs). The single in-scope failure named in the packet brief
(`semio-framework-machine`, 3 lib-test errors) is resolved to 0.
