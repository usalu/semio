# 📓️ terra-dedyn-fleet-procedural report

Packet: `dedyn-fleet-procedural`. Owned paths: `✏️s/🔌️plugins/🌀️procedural/**` + this ticket folder.

## 1. Starting / ending `dyn` count

**Start: 12** first-party `dyn` uses (verified against the brief's inventory before editing):
`Constraint` — 10 uses across `⛓️constraint/component.rs:130`,
`🔳️solver-grid-2d/component.rs:24,53,85`, `🕸️solver-graph/component.rs:26,55,80`,
`🧱️solver-grid-3d/component.rs:23,52,84`. `SoftConstraint` — 2 uses,
`🪶️soft/component.rs:52` and `🧬️evolve/component.rs:57`.

**End: 0.** Verified with two differently-implemented searches over absolute python3 paths (shell
globbing on this repo's emoji paths is known-unreliable per the ticket's rule 21):

1. Python regex `\bdyn\s+\w`, comment lines excluded → **0 hits**.
2. `grep -n "dyn "` (all lines, comments included) minus `dyn Future/Fn/Any/Error` → **7 hits, all
   inside `//`/`///` doc comments** I wrote explaining the two conversions (`` `Box<dyn Constraint>` ``
   in prose, `` `&dyn SoftConstraint` `` in prose, and the macro names `dyn_enum`/`dyn_enum_close!`
   themselves). Zero live-code occurrences.

A third check (`grep -c "Box<dyn\|&dyn "` per file) turned up the same 3 comment-only files, nothing
else. `dyn <first-party trait>` is zero across all 342 `.rs` files in the owned path.

## 2. Mechanism per family, and why (R11's four-way decision procedure)

### `Constraint` — **closed set → `dyn_enum_close!`**

Read the declaration site first per the brief's warning (`⛓️constraint/component.rs`) — confirmed
this is a locally-declared trait unrelated to `🧮️math`'s same-named `Constraint`. Then enumerated
every `impl Constraint for _` in the owned path with a python AST-free regex scan: exactly **4**,
all in this crate — `FlowConstraint` (`🌊️flow`), `ConnectivityConstraint` + `ReachabilityConstraint`
(`🔗️constraints-conn`), `CardinalityConstraint` (`🔢️constraints-card`). Closed set, all in-crate ⇒
`dyn_enum_close!` per R11.

- `#[dyn_enum]` added directly on `pub trait Constraint` in `⛓️constraint/component.rs` (trait body
  unchanged — 4 methods, all `&self`, no destructuring params, no associated types/consts, so none
  of the macro's rejection cases apply).
- `dyn_enum_close! { pub enum Constraints: Constraint { Flow(FlowConstraint),
  Connectivity(ConnectivityConstraint), Reachability(ReachabilityConstraint),
  Cardinality(CardinalityConstraint) } }` placed in the **same module** as the trait (right after
  it, replacing the old `Box<dyn Constraint>` field type immediately below) — this is what lets the
  macro emit a **bare** `dyn_enum_close!`-internal invocation of the captured dispatch macro with
  zero warnings (rustc#52234, per `📓️terra-dyn-enum-macro-report.md` finding 1). The four concrete
  types live in sibling modules (`flow`, `constraints_conn`, `constraints_card`), so I imported them
  into `constraint.rs` — this makes `constraint.rs` import from modules that already import `Constraint`
  back from `constraint.rs`, a module-level `use` cycle. Verified this is legal Rust (no orphan/const-eval
  cycle, just type-name resolution) against the existing pattern already used throughout this same
  crate (e.g. `flow.rs` already imports from `constraint.rs` and vice versa was always implicit through
  the trait bound).
- Both impl blocks (`impl Constraint for FlowConstraint` etc., in their own files) needed **zero
  edits** — exactly the point of the mechanism.
- Every `Box<dyn Constraint>` field/parameter (`ConstraintSet::constraints`,
  `{Grid2d,Grid3d,Graph}Solver{,Builder}::constraints`, and all three `.constraint(c: Box<dyn
  Constraint>)` builder methods) became `Constraints`/`Vec<Constraints>` — dropping the `Box` (the
  enum is already a concrete, non-fat-pointer type; nothing here needed heap allocation for any other
  reason).
- 3 test call sites in `🕸️solver-graph/component.rs` (`GraphSolverBuilder::new(..).constraint(Box::new(constraint))`)
  became `.constraint(constraint.into())`, using the generated `From<VariantTy> for Constraints` impls
  (E1 — sync, per the macro's own design).

### `SoftConstraint` — **open set → generics** (R11 case (a), the trivially-generic parameter shape)

`SoftConstraint`'s two `dyn` sites (`best_of_n`, `evolve`) are exactly the "parameters and borrowed
references" shape R11 calls out as no-design-question generic conversions, not the "method returns an
implementation" shape that needs associated types. The doc comments describe the scorer as
**caller-supplied** — any future `impl SoftConstraint` a caller writes, not a fixed set this crate
enumerates (the crate ships exactly one concrete impl, `ScoreFn<F>`, itself generic over an arbitrary
closure `F` — i.e. the design already treats this as an open extension point, reinforcing that an enum
would be the wrong shape here even though only one concrete type exists today).

- `pub async fn best_of_n(.., scorer: &dyn SoftConstraint, ..)` → `pub async fn
  best_of_n<S: SoftConstraint>(.., scorer: &S, ..)` in `🪶️soft/component.rs`.
- `pub async fn evolve(.., scorer: &dyn SoftConstraint, ..)` → `pub async fn evolve<S:
  SoftConstraint>(.., scorer: &S, ..)` in `🧬️evolve/component.rs`.
- No call-site changes needed anywhere (checked: `best_of_n(`/`evolve(` only appear in these two
  files' own tests, all passing a concrete `&ScoreFn { .. }` already — generic inference picks it up
  unchanged).

## 3. Macro friction

None beyond what `📓️terra-dyn-enum-macro-report.md` already documented and I followed exactly:
same-module placement for the bare invocation, `dyn_enum_close!` (not `dyn_enum!`) at the closing
site, `.into()` at construction sites instead of hand-writing `Constraints::Flow(..)`. No associated
types, no `self: Arc<Self>`/`&mut self` mixing, no destructuring params — `Constraint`'s shape hit
none of the macro's four documented rejection cases, so this was a clean mechanical application.

## 4. Crate-level plumbing

- Added `semio-framework-dispatch-macros` as a dependency of `semio-s-plugin-procedural`
  (`📦️packages/🦀️rust/Cargo.toml`) — this file is inside my owned path, so no lease was needed. The
  crate itself is already a registered workspace member (verified: `grep -n "🔀️dispatch" Cargo.toml`
  at repo root hit line 103) — no root-`Cargo.toml` lease needed either, unlike the dispatch-macro
  packet's own worked example which hit an *unregistered* target.
- Added `#![allow(async_fn_in_trait)]` at the crate root (`📦️glue.rs`), one-line comment pointing at
  R3/R7, per the macro report's recipe item 6 (a `#[dyn_enum]` trait with `async fn` methods needs
  this in the *consuming* crate; the macro cannot inject it across the boundary). Verified nothing else
  in this crate already declared it (`grep -n "allow(async_fn_in_trait)"` — no hits before my edit).

## 5. Acceptance — command, output, exit code

SDK gate status checked first, as instructed: **still blocked**, one previous-session E0599 in
`semio-framework-os-kernel` (`🏪️store/component.rs:3485`, `decode_envelope(..).map_err(..)` on an
un-awaited future — unrelated to `dyn`, unrelated to my packet, not in any owned path).

```
$ CARGO_TARGET_DIR=<scratchpad>/target-procedural cargo check -p semio-s-plugin-procedural --lib
   ...
error[E0599]: no method named `map_err` found for opaque type `impl Future<Output = Result<protocol::MutationEnvelope, protocol::ProtocolError>>` in the current scope
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️component.rs:3485:58
error: could not compile `semio-framework-os-kernel` (lib) due to 1 previous error; 9 warnings emitted
```
Exit code: `101` (captured directly from `cargo`, no pipe — rule 10). `grep -n "procedural"` over the
full build log: **zero matches** — rustc never reached this crate's own source; the failure is
entirely upstream in the SDK dependency chain. Per the ticket's compile-reality section, this is
reported **acceptance UNRUN**, structural verification (§1) standing in its place.

## 6. Fleet-wide condition found (out of scope, flagged for whoever runs await-insertion on this plugin)

Confirmed via python3 (`.await` substring count) across **every** `.rs` file in the owned path: **0
occurrences, 0 of 342 files**. `asyncify-fleet.py`/`asyncify-universal.py` has clearly already run
here (every fn in the crate is already `async fn`), but `insert-await.py` has not — this is a
plugin-wide condition, not something my 12-`dyn`-site packet introduced or is scoped to fix (that would
mean hand-fixing hundreds of call sites fleet-wide, explicitly a separate tracked effort with its own
shared tool). One specific residue worth flagging concretely for whoever does that pass on this plugin:
`🔍️search/component.rs:224`, `constraints_accept`'s
`cs.constraints.iter().all(|c| c.validate_complete(&assignment, cs.adjacency).is_ok())` is exactly
R10's residue shape #1 (`.await` inside a sync `Iterator::all` closure) — it needs hoisting into a
plain loop (`for c in cs.constraints { if c.validate_complete(..).await.is_err() { return false } }`
or equivalent), not a mechanical `.await` insertion. This line does not compile today regardless of my
`dyn`→enum change (calling `.is_ok()` on an un-awaited `Future` was never valid) — my conversion is
inert with respect to this pre-existing defect, neither fixing nor worsening it.

## 7. `pending_effects` (brief's aside)

The brief noted this plugin "historically held 2 of the 3 `pending_effects` sites." Checked: these are
`async fn pending_effects(doc: &ArtifactView<..>, ..) -> Vec<Effect>` — ordinary trait-method
implementations of a framework trait, not a `dyn`-dispatched seam (no `Box<dyn ..>`/`&dyn ..` anywhere
near them). Unrelated to this packet's target; no action taken or needed.

## 8. `lease-request`

None. Every file touched is inside `✏️s/🔌️plugins/🌀️procedural/**` or the ticket folder.

## Files touched

- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧩️wfc-engine/⛓️constraint/🦀️component.rs`
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧩️wfc-engine/🔳️solver-grid-2d/🦀️component.rs`
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧩️wfc-engine/🧱️solver-grid-3d/🦀️component.rs`
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧩️wfc-engine/🕸️solver-graph/🦀️component.rs`
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧩️wfc-engine/🪶️soft/🦀️component.rs`
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧩️wfc-engine/🧬️evolve/🦀️component.rs`
- `✏️s/🔌️plugins/🌀️procedural/📦️packages/🦀️rust/Cargo.toml`
- `✏️s/🔌️plugins/🌀️procedural/📦️packages/🦀️rust/📦️glue.rs`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME/📓️terra-dedyn-fleet-procedural-report.md` (this file)
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME/terra-dedyn-procedural-build.txt` (build log)
