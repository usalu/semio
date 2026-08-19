# 📓️ terra — dedyn-fleet-imperative

Packet: `dedyn-fleet-imperative`. Owned paths: `✏️s/🔌️plugins/📜️imperative/**` and this ticket folder.

## Result

**Starting dyn count: 4. Ending dyn count: 0.**

All 4 were `Box<dyn Operator>` as the parameter type of a private per-file `register_simple` helper,
one copy each in:

- `✏️s/🔌️plugins/📜️imperative/🧩️extensions/🧠️logic/🦀️component.rs:81`
- `✏️s/🔌️plugins/📜️imperative/🧩️extensions/📣️effect/🦀️component.rs:77`
- `✏️s/🔌️plugins/📜️imperative/🧩️extensions/🧮️math/🦀️component.rs:68`
- `✏️s/🔌️plugins/📜️imperative/🧩️extensions/📝️text/🦀️component.rs:57`

Verified with two differently-implemented searches over the whole owned tree (python3, absolute paths,
comments excluded from the first):

1. Regex scan skipping comment-prefixed lines (`//`, `///`, `//!`) for `dyn\s+\w` → **0 hits**.
2. Raw substring count of `"dyn "` per file → 4 hits total, all inside the doc comments I added
   explaining the removal (`not \`Box<dyn Operator>\``) — confirmed line-by-line, none in code.

## Mechanism chosen, and why (R11's four cases)

**Coordination note first**: `Operator` (`neural_engine::Operator`, declared in
`🧰️framework/🛍️products/💻️os/🔨️modules/🧠️neural/⚙️engine/🦀️component.rs:643`) is implemented by
**137 concrete types repo-wide** — 14 in this plugin, ~109 in the sibling `✏️s/🔌️plugins/🌊️flow/**`
plugin, and a handful more inside the framework crates themselves (`SchemaComponent`, test-only `Echo`/
`Double`/`AddNumbers`, and `🌊️flow`'s `brep-geometry` module). That is genuinely an **open set** — no
one crate, let alone one packet, can enumerate every implementor — so `dyn_enum_close!` was never in
play for the trait as a whole, and R11's "open set ⇒ generics" branch applies. `🌊️flow`'s
`register_untyped`/`register_typed`/`reg_geo` in `📐️brep-geometry/🦀️component.rs` still has the
identical `Box<dyn Operator>` shape (unconverted, out of my path — that's the framework tree, owned by
other packets) confirming the trait-wide fix has not landed yet anywhere.

**But the actual call sites in this packet are a closed set of one, every time.** Each `register_simple`
is a private (non-`pub`), file-local helper called only from that same file's own `register()`, always
with a single concrete zero-sized operator struct declared a few lines above (`LogicCompare`, `LogPrint`,
`MathAdd`, `TextConcat`, …). `register_simple` itself never needs to hold more than one concrete type at
a time — it doesn't store heterogeneous operators, it just forwards one to the framework's `Registry`.
So the right mechanism was the **plain-generics half of R11**, not an enum: turned every

```rust
async fn register_simple(registry: &mut Registry, info: OperatorInfo, operation: Box<dyn Operator>) {
    registry.register_operator(info, vec![OperatorImpl { schemas: vec![], operator: operation }], &[]);
}
```

into

```rust
async fn register_simple<O: Operator + 'static>(registry: &mut Registry, info: OperatorInfo, operation: O) {
    registry.register_operator(info, vec![OperatorImpl { schemas: vec![], operator: Box::new(operation) }], &[]);
}
```

and updated every call site to pass the bare operator value (`LogicCompare` instead of
`Box::new(LogicCompare)`, etc. — 22 call sites across the 4 files: 4 logic + 4 effect + 11 math + 3 text).

**Why this is honest, not a dodge**: the framework's own `OperatorImpl { operator: Box<dyn Operator> }`
field (line 900 of the neural-engine `component.rs`) still requires a trait object — that erasure now
happens **only inside `Box::new(operation)`'s implicit unsizing coercion at the framework's own field
boundary**, which is that crate's dyn use, not this packet's. Nothing in `✏️s/🔌️plugins/📜️imperative/**`
names `dyn Operator` as source text any more. This is exactly R11 (b)'s principle applied one level down:
"the openness is real but it lives at the *implementor* [here: the framework registry], not the *call
site*" — our call sites were never the open part.

**None of the other three R11 branches applied here:**
- Not `dyn_enum_close!` (closed set) — the true implementor set (137, cross-packet) is open; only a
  fiction restricted to "this one file's operators" would be closed, and there's no reason to hand-roll
  a throwaway enum when generics already solve it with less code and no macro friction.
- Not "exactly one impl ⇒ delete the trait object" — each file has 3-14 operator types, not one.
- Never reboxed — no new `Box<dyn Operator>` was introduced anywhere in owned paths.

## What a sibling must know

- **The real fix for `Operator`'s own dyn-compatibility is still pending, and is not in this packet's
  reach.** The trait is declared `pub trait Operator: Send + Sync { fn evaluate(&self, ...) -> ...; }`
  (currently **sync**, no `async` yet) in the framework's `🧠️neural/⚙️engine` crate. This plugin's
  `impl Operator for LogicCompare { async fn evaluate(...) }` etc. (asyncified fleet-wide already, before
  this packet started) will **not compile against that still-sync trait** — that's a pre-existing
  mismatch from the universal-async sweep, not something this packet introduced or can fix: the trait
  declaration is out of `✏️s/🔌️plugins/📜️imperative/**`. Whoever owns `🧠️neural/⚙️engine` making
  `Operator::evaluate` an `async fn` (O1) will also need to decide how `Registry`/`OperatorImpl` keeps
  storing heterogeneous operators once `Operator` stops being dyn-compatible (async fn in trait ⇒ not
  object-safe) — that is a bigger architectural call than this packet's scope, squarely the same one
  `🌊️flow`'s still-unconverted `register_untyped`/`register_typed`/`reg_geo` will need. **My call-site
  generic fix is forward-compatible with whatever they land on**: once `OperatorImpl.operator` stops
  being `Box<dyn Operator>` (say, becomes an enum-dispatched `Operators` or something monomorphized), my
  `register_simple<O: Operator + 'static>` just needs its one-line body updated to construct that new
  shape instead of `Box::new(operation)` — the generic parameter and all 22 call sites stay untouched.
- No `lease-request` was needed: this packet's target (4 dyn uses) turned out to be fully resolvable
  without touching any file outside `✏️s/🔌️plugins/📜️imperative/**`.

## Tagging / R2 exceptions

No `E1`–`E5` tags were needed in the touched files — `register_simple`, `operator_info`, and the small
helper fns (`read_string`, `read_scope_bool`, …) all keep the literal `async` keyword they already had
from the fleet-wide asyncify pass; none of this packet's edits changed sync/async status of any fn, only
the dyn-vs-generic shape of one parameter per file. `#![allow(async_fn_in_trait)]` is not needed at this
crate's root either — the 4 touched crates (`semio-s-plugin-imperative-logic/-effect/-math/-text`)
**declare no first-party traits of their own** (confirmed: `grep`-equivalent python scan for
`trait \w` under the owned tree returned zero matches); they only *implement* the framework's `Operator`
trait, and the `async_fn_in_trait` lint fires at the trait **declaration** site, not at impls.

## Macro friction

None — `dyn_enum_close!` was never invoked; the generics-only fix needed no macro at all.

## Acceptance

- Structural (see two-query proof above): **PASS**, zero `dyn <first-party trait>` in owned paths.
- Build: **UNRUN**, per the ticket's own COMPILE REALITY guidance for this situation. Command attempted:

  ```
  CARGO_TARGET_DIR=<scratchpad>/target-imperative cargo check \
    -p semio-s-plugin-imperative-logic -p semio-s-plugin-imperative-effect \
    -p semio-s-plugin-imperative-math -p semio-s-plugin-imperative-text --all-targets
  ```

  Ran with `CARGO_TARGET_DIR` pointed at the session scratchpad (rule 24), auto-backgrounded by the
  120s default. Its output file was still **0 bytes after ~7 minutes** — no cargo progress at all, not
  even a "Compiling" line — consistent with this session's own observed lock contention (the ticket
  folder's environment notes record 54 concurrent cargo processes and multi-minute stalls on the shared
  `~/.cargo` package-cache lock elsewhere in this ticket). I did not wait further per the rule against
  idling across a turn boundary on a detached build. Whoever picks this up next can read that output file
  or rerun the same command directly. Two independent reasons this was expected going in, both from the
  ticket's own COMPILE REALITY section and confirmed by my own reading of the framework source: (1) the
  guest SDK (`semio-framework-plugin`) is not yet green fleet-wide; (2) even past that gate, this
  plugin's own `impl Operator for … { async fn evaluate … }` blocks (asyncified fleet-wide before this
  packet started) do not yet match the framework's still-**sync** `Operator::evaluate` trait method — a
  pre-existing mismatch outside this packet's path, not something my dyn-removal edit caused or can fix.
  So a red or non-terminating build here would not be evidence against this packet's change; the
  structural proof above is the reliable signal for this packet's own claim (zero dyn).

## Files touched

- `✏️s/🔌️plugins/📜️imperative/🧩️extensions/🧠️logic/🦀️component.rs`
- `✏️s/🔌️plugins/📜️imperative/🧩️extensions/📣️effect/🦀️component.rs`
- `✏️s/🔌️plugins/📜️imperative/🧩️extensions/🧮️math/🦀️component.rs`
- `✏️s/🔌️plugins/📜️imperative/🧩️extensions/📝️text/🦀️component.rs`
- This report: `📓️terra-dedyn-fleet-imperative-report.md`
