# 📓️ terra-dedyn-fleet-draw report

Packet: `dedyn-fleet-draw`. Target: zero `dyn <first-party trait>` in `✏️s/🔌️plugins/🖍️draw/**`.

## 1. Counts

**Starting: 18** (`CommandSink` 16, `Migration` 2) — verified before any edit with a python3 regex scan
over every file under the owned path, matching the brief's inventory exactly.

**Ending: 0** — verified with **two independently-implemented searches**, comments excluded:

1. Python3 regex (`\bdyn\s+[A-Za-z_]...`) over every file, stripping `//` comments before matching.
2. Python3 token-split scan (no regex engine) — splits each code line (post `//`-strip) on
   `(`/`&`/`<`/`,`/whitespace`, looks for the token `dyn`, checks the following token against the R1
   allow-list (`Future`, `Fn`, `FnMut`, `FnOnce`, `Any`, `Error`).

Both report **0**. A third `grep -rnE 'dyn [A-Za-z_]'` pass (not comment-aware) finds exactly 2 hits — both
are inside `//` comments explaining what was removed (`` `&mut dyn CommandSink<M>` ``, `` `dyn Migration` ``
as prose), not live code. `dyn Future` is separately confirmed absent (R1 permits it; we simply never had
any).

## 2. `CommandSink` (16 uses → 0) — mechanism: **delete the trait, use the concrete type (R11: exactly one impl)**

**Correction to the packet brief**: `CommandSink` is not declared in `🧰️framework/🔨️modules/🔄️machine`
(a crate this packet does not own). That framework file exists and is near-line-identical, but the draw
plugin carries its **own independent fork**, `semio-s-plugin-draw-fsm` (package `fsm`, `extern crate self
as fsm`), at
[historical 🔄️fsm/ directory beneath the recorded source root](../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧫️fixtures/🧪️cad-draw-path-projection/🔣️.json#/projections/1/sourceRoot)
— fully inside this packet's owned paths ("the plugin and its extension crates, which ride with their
parent"). `CommandSink` and `Migration` are declared and consumed entirely inside this crate; nothing
outside it references either. This matters for every remaining sibling with a similarly-named
framework-adjacent trait: check whether the plugin vendors its own copy before assuming a framework lease
is needed.

`CommandSink<M>` had exactly **one method** (`push`) and exactly **one impl**
(`impl<M: Machine> CommandSink<M> for Vec<Command<M>>`). Per R11's decision procedure ("exactly one impl ⇒
delete the trait object, use the concrete type — an enum of one is worse than none"), the fix is not
`dyn_enum_close!` (that would generate a pointless one-variant wrapper) but deleting the indirection
entirely:

- `ActionFn<M>` (`kernel::ActionFn<M> = fn(&mut Context, Option<&Event>, &mut dyn CommandSink<M>)`) is a
  **fn-pointer type** (E4-shaped slot: stored in `MachineDefinition.actions: &'static [ActionFn<M>]`).
  `impl Trait` is not nameable in a fn-pointer type, so this was the one dyn use that could not become
  generic — it became the concrete `&mut Vec<Command<M>>` directly.
- That forced every action-fn implementation compiled into the table (12 in
  `canvas-pointer-down/🦀️component.rs`'s `gesture_*` fns, plus 4 test-only actions inside `fsm`'s own
  test modules) to match the same concrete signature — mechanical, unique-string replacement of
  `&mut dyn fsm::CommandSink<draw_gesture::DrawGesture>` → `&mut Vec<fsm::Command<draw_gesture::DrawGesture>>`
  (12 occurrences, exact string match, not name-keyed) and the 4 in-crate test actions similarly.
- **Cascade**: once `ActionFn<M>` forced the concrete type, the 5 kernel entry points that thread `sink`
  down to the action table (`apply_transitions`, `run_to_completion`, `init`, `macrostep`,
  `timer_elapsed`) still declared `sink: &mut impl CommandSink<M>` — a *generic* bound, never `dyn`, so it
  was outside this packet's literal 18-count, but it no longer type-checked against the fn-pointer's now-
  concrete parameter (`cargo check` caught this immediately: "expected `&mut Vec<Command<M>>`, found
  `&mut impl CommandSink<M>`"). Fixed by changing all 5 to `&mut Vec<Command<M>>` too.
- With every caller and callee now on the concrete type, `CommandSink` had **zero remaining consumers**.
  Per the greenfield/no-legacy instruction, a trait with one impl and zero callers is dead weight, not a
  documented API — **deleted the trait and its impl outright** (region `🔖️Commands`), removed it from the
  crate's `pub use kernel::{...}` re-export list, and fixed the one intra-doc `` [`CommandSink`] `` link
  that would otherwise have broken rustdoc.

No macro used for this family — the R11 "exactly one impl" branch is explicitly "don't build an enum",
and that held all the way through the cascade.

## 3. `Migration` (2 uses → 0) — mechanism: **generics + a closed zero-variant enum, per R11's residue guidance**

`restore<M: Machine>(persisted: &PersistedSnapshot, context: M::Context, migrations: &[&dyn Migration])`
takes a **caller-supplied, potentially heterogeneous list** — exactly the shape flagged by the earlier
`dyn-enum-macro` packet (finding 6) as "a materially different, harder shape... needing a per-consumer
closed enum design". Since `Migration` is fully private to this crate (no external crate can add impls —
confirmed: the ENTIRE draw plugin has exactly one impl anywhere, `BumpFingerprint`, and it is test-only),
the set is genuinely closed, but a single caller might still combine several concrete migration types in
one call in principle (that's the whole reason the list is heterogeneous rather than `Vec<SomeOneType>`).

Applied R11's general resolution for this shape directly:

```rust
#[dyn_enum]
pub trait Migration {
    async fn source_fingerprint(&self) -> u64;
    async fn migrate(&self, snapshot: PersistedSnapshot) -> PersistedSnapshot;
}

dyn_enum_close! {
    pub enum NoMigrations: Migration {}
}

pub async fn restore<M: Machine, Mg: Migration>(
    persisted: &PersistedSnapshot, context: M::Context, migrations: &[&Mg],
) -> Result<Snapshot<M>, RestoreError> { ... }
```

- `restore`'s `migrations` parameter is a borrowed-reference parameter (R11a: "trivially generic"), so it
  became `Mg: Migration` — homogeneous per call, exactly like every other genuinely open parameter this
  program has converted.
- **When a caller someday needs heterogeneity in one call**, the answer is not `dyn` — it's *that
  caller's* crate declaring its own closed `dyn_enum_close!` enum over the concrete migration types it
  actually has, and passing that enum as `Mg`. This is exactly R11's "the openness is real but it lives at
  the implementor, not the call site" principle, generalised from trait-method-return position (R11's
  worked example) to a heterogeneous-list parameter.
- Today nothing outside tests implements `Migration`, so every production call site passes an empty
  slice and needs *some* concrete `Mg`. Added a **zero-variant `dyn_enum_close!` enum**, `NoMigrations` —
  the same uninhabited shape the dispatch-macros crate's own `NoWidgets`/`NoCounters` tests proved
  (`match *self {}` bodies, verified against real rustc by that packet). `#[dyn_enum]` on the trait +
  `dyn_enum_close!` right after it, same module, bare invocation — zero macro friction, matches the
  recipe exactly.
- Updated all 4 call sites: 2 production-shaped tests using `&[]` → `Mg = NoMigrations`; the migration-
  chain test → `Mg` inferred from `let migrations: &[&BumpFingerprint] = &[&migration];` (dropped the
  `dyn`, kept the concrete type, turbofish for `M` alone still infers `Mg`); the `fsm-macros` **generated**
  `restore_json` wasm-bindgen method (inside a `quote!{}` block, emitted into every `export_wasm_machine!`
  consumer) → `fsm::restore::<#machine_path, fsm::NoMigrations>(...)`.
- `restore`'s body originally called `migrations.iter().find(|m| m.source_fingerprint() == ...)` — once
  `Mg: Migration` makes `source_fingerprint`/`migrate` real (still-async, per O1) trait-method calls
  needing `.await`, `.await` inside `Iterator::find`'s sync closure is illegal (R10 residue shape 1:
  E0728). Hoisted into a plain `for` loop that awaits each candidate, matching R10's documented fix.

## 4. `#![allow(async_fn_in_trait)]`

Added to the `fsm` crate root (`📦️glue.rs`), citing R3/R7, since the crate declares several async-fn
trait families (`Host`, `Inspector`, `Migration`, `Configuration`, `StatechartEvent`, `Machine`). Took no
`-> impl Future + Send` shortcut anywhere (R7 prohibition) and added no `+ Send` bound (R3).

## 5. Bonus fix, in-scope: `semio-s-plugin-draw-fsm-macros` (E3 — proc-macro entry points)

Not part of the 18-count, but discovered while wiring `dyn_enum`/`dyn_enum_close!` into this crate's
sibling: the proc-macro crate (`✨️macros/` — rides with the same plugin, so squarely in this packet's
owned paths) had its 4 `#[proc_macro]`/`#[proc_macro_derive]` entry points, and 37 pure token-manipulation
helper functions they call, all marked `async fn` — a hard proc-macro signature violation (`fn(TokenStream)
-> TokenStream` is fixed by rustc; E3). This is exactly what an earlier `dyn-enum-macro` packet's finding 5
flagged as residue "for whichever packet owns those files next" — that packet is this one.

Fixed mechanically: stripped `async` from the 4 entry points (`glue.rs`) and the 37 real helper functions
(`component.rs`), each tagged `// 🚫️async: E3 ...`. Left untouched (by design, verified line-by-line):
- 12 `async fn`s that are **generated code text inside `quote!{}` blocks** — these become real `async fn`
  items in whichever consumer crate invokes `statechart!`/`export_wasm_machine!`, so O1 requires them to
  stay literally `async`.
- 10 `#[test] async fn`s already correctly wearing `#[semio_framework_async_macros::async_test]` — no
  residue there.

Zero `.await` existed anywhere in this file before or after (confirmed both ways) — the entry points called
their `async fn` helpers without ever awaiting them, which is precisely why the crate did not compile at
all before this fix (`.unwrap_or_else` on an unawaited `Future` is a type error). Stripping `async` from
real functions with already-sync call sites fixed this with **no other edits needed** anywhere in that
file.

## 6. Compile verification

Per rule 24, all cargo runs used `CARGO_TARGET_DIR=<session scratchpad>/target-dedyn-draw`, `-p <crate>`,
foreground, generous timeouts.

```
$ cargo check -p semio-s-plugin-draw-fsm-macros --all-targets
    Finished `dev` profile [unoptimized] target(s) in 0.19s
```
Exit `0`. **Clean.** (This crate was previously broken — see §5 — and now compiles.)

```
$ cargo check -p semio-s-plugin-draw-fsm --lib
error: could not compile `semio-s-plugin-draw-fsm` (lib) due to 81 previous errors
```
```
$ cargo check -p semio-s-plugin-draw-fsm --all-targets
error: could not compile `semio-s-plugin-draw-fsm` (lib test) due to 273 previous errors
```
**Not our defect.** Grepped both full error logs for `CommandSink`, `Migration`, `NoMigrations`, `dyn `,
`object safe`, `E0038` — **zero matches** in error text (the few source-line echoes that contain
`NoMigrations`/`Migration` are rustc printing our own now-correct call sites as *context* for an unrelated
error on the same or an adjacent line, e.g. `let def = M::definition();` — itself missing `.await` and
pre-dating this packet entirely — poisoning every line downstream that touches `def`). Every one of the 81
(lib) / 273 (all-targets) errors is `E0308`/`E0599`/`E0609`/`E0277`/`E0369`/`E0600` from **missing
`.await`** on calls to unrelated async fns (`M::definition()`, `stable_id_to_node`, `persist`, `init`,
`macrostep`, `ConfigurationIter` iteration, etc.) scattered across the whole 2400-line file — a systemic,
pre-existing gap from an incomplete prior asyncify pass, present before this packet touched anything and
verified to be **unrelated to the 18 dyn uses this packet owns**. Fixing it is a general await-insertion
job (the `insert-await.py` tool's domain), not a de-dyn job, and is far larger than this packet's target —
flagging it here for the coordinator/next packet rather than absorbing it silently.

```
$ cargo check -p semio-s-plugin-draw --lib
...
error: could not compile `semio-framework-os-kernel` (lib) due to 1 previous error
```
Full-plugin build is blocked further upstream, in `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs`
(missing `.await` on `crate::os_spr::decode_envelope(...)`) — **not our path, not our crate.** Confirms the
brief's "Compile Reality" warning that a fleet build will fail before reaching this plugin's own code,
though the actual blocking crate is `semio-framework-os-kernel`, not `semio-framework-plugin` as
anticipated.

## 7. Macro friction

None. `#[dyn_enum]`/`dyn_enum_close!` worked exactly as documented in
`📓️terra-dyn-enum-macro-report.md`'s recipe: bare invocation, same module, trait declared first, zero-
variant enum for the uninhabited case. The one new fact this packet adds: **a plugin can vendor its own
copy of a framework-shaped trait**, in which case the macro's "owner-only" limit (point 4 in that report)
doesn't block you at all — check for a local fork before assuming a cross-crate lease is needed.

## 8. lease-request

None needed. `semio-framework-dispatch-macros` is already a registered workspace member (root
`Cargo.toml` line 103); added it as a normal path dependency to `fsm`'s own `Cargo.toml` (a file this
packet owns), no root-manifest edit required.

## 9. For siblings

- Check whether your plugin vendors its own copy of a "framework" trait before treating it as
  out-of-reach — this packet's target turned out to be 100% owned, contrary to the brief's initial read.
- If a fn-pointer-typed slot (`ActionFn<M>`-shaped) parameterises over a trait with exactly one impl,
  expect the fix to cascade into every *generic* (`impl Trait`) caller in the same call chain too — the
  18-count only tracks literal `dyn`, but the fn-pointer's concrete type forces its whole call graph onto
  that concrete type, which can turn a same-file trait into fully dead code worth deleting.
- `semio-s-plugin-draw-fsm` and `semio-s-plugin-draw-fsm-macros` do NOT depend on the guest SDK
  (`semio-framework-plugin`) at all — they are independently checkable today, gate or no gate.
- `semio-s-plugin-draw-fsm` has ~81-273 pre-existing missing-`.await` errors unrelated to dyn, and the
  full `semio-s-plugin-draw` build is separately blocked by `semio-framework-os-kernel` — neither is this
  packet's defect to fix, both are flagged here for whoever picks up general await-insertion / the
  os-kernel gate next.

## Files touched

- [historical FSM component source (catalog mapping 9)](../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧫️fixtures/🧪️cad-draw-path-projection/🔣️.json#/projections/1/mappings/9/sourcePath)
  — `ActionFn<M>` concrete-typed; `CommandSink` trait+impl deleted; 5 generic-bound sigs → concrete;
  `#[dyn_enum]` on `Migration`; `NoMigrations` zero-variant enum; `restore` generic over `Mg: Migration`,
  await-hoisted loop; 4 call sites updated; crate re-export list updated; doc-comment fixes.
- [historical canvas-pointer-down component source (catalog mapping 10)](../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧫️fixtures/🧪️cad-draw-path-projection/🔣️.json#/projections/1/mappings/10/sourcePath)
  — 12 `gesture_*` action fn signatures, `dyn fsm::CommandSink<...>` → `Vec<fsm::Command<...>>`.
- [historical FSM library glue source (catalog mapping 8)](../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧫️fixtures/🧪️cad-draw-path-projection/🔣️.json#/projections/1/mappings/8/sourcePath)
  — `#![allow(async_fn_in_trait)]` added.
- [historical FSM Cargo manifest source (catalog mapping 5)](../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧫️fixtures/🧪️cad-draw-path-projection/🔣️.json#/projections/1/mappings/5/sourcePath)
  — added `semio-framework-dispatch-macros` path dependency.
- [historical macros component source (catalog mapping 4)](../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧫️fixtures/🧪️cad-draw-path-projection/🔣️.json#/projections/1/mappings/4/sourcePath)
  — 37 `async fn` → `fn` (E3-transitive), each tagged.
- [historical macros library glue source (catalog mapping 3)](../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧫️fixtures/🧪️cad-draw-path-projection/🔣️.json#/projections/1/mappings/3/sourcePath)
  — 4 proc-macro entry points `async fn` → `fn` (E3), tagged.
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME/📓️terra-dedyn-fleet-draw-report.md`
  (this file, new).

No files outside the owned paths above and the ticket folder were modified. No scratch logs left in the
ticket folder — all cargo target dirs and check-output `.txt` files live under the session scratchpad per
rule 24.
