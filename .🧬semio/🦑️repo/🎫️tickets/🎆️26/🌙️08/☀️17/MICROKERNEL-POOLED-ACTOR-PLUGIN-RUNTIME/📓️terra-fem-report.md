# 📓️ terra-fem report

Packet: `dyn-fem`. Deliverable: zero `dyn Element` in `✏️s/🔨️modules/🏗️fem/**`, via `dyn_enum_close!`
(mechanism built by the `dyn-enum-macro` packet, `semio-framework-dispatch-macros`, already a
registered workspace member — verified: `grep -n dispatch Cargo.toml` shows the member line already
present, so that packet's lease-request had already landed before I started).

## 0. The gap-check the brief asked for FIRST

Before touching `Element`, scanned all of `✏️s/🔨️modules` (not just `🏗️fem`) for first-party `dyn`
uses, comments excluded, two differently-implemented ways:

```
$ python3 <regex over ✏️s/🔨️modules, block+line comments stripped, dyn Future/Fn/FnMut/FnOnce/Any/Error excluded>
dyn Element: 20 occurrences
TOTAL FAMILIES: 1
TOTAL OCCURRENCES: 20
```
```
$ grep -rn --include='*.rs' -E '\bdyn\s+[A-Za-z_]' ✏️s/🔨️modules | grep -vE '\bdyn\s+(Future|Fn|FnMut|FnOnce|Any|Error)\b' | grep -oE 'dyn [A-Za-z_][A-Za-z0-9_]*' | sort -u
dyn Element
```
**Finding for the coordinator: `✏️s/🔨️modules` holds no other first-party `dyn` family besides
`Element`.** `🌐️spatial-kernel`, `💭️mindmap`, `📜️imperative` (and its `🧩️extension_sdk`) are all clean.
The fleet-inventory gap the brief described was real but narrow — exactly the one family named.

## 1. Starting / ending counts

| | count |
|---|---:|
| `dyn Element` uses, start | **20** (14 in type position `Box<dyn Element>`/`Vec<Box<dyn Element>>`/`&[Box<dyn Element>]`, 6 in fn signatures) |
| `dyn Element` uses, end | **0** — verified twice (regex-with-comments-stripped AND plain grep, both zero) |
| `Box::new(<ElementImpl> {..})` construction sites rewritten to `.into()` | **35** (not all contained the literal text `dyn Element` — the field/local type was inferred, so these were invisible to a `dyn Element` grep and had to be found via `Box::new(` itself) |
| first-party `dyn` families in `✏️s/🔨️modules` outside `Element` | **0** |

## 2. Mechanism — closed set, `dyn_enum_close!`

**Trait**: `Element` (`🏗️model/🦀️component.rs:85`), 8 methods, all `&self`, no associated
types/consts, no `self: Arc<Self>`/`&mut self` mix — none of the `dyn_enum` rejection shapes apply.

**Impl census** (`grep -rn 'impl Element for' ✏️s/🔨️modules/🏗️fem`): **13 impls, ALL inside this
crate's own modules** — `Bar2`/`BeamEb2`/`Tri3Cst`/`Tri6Lst`/`Quad4`/`Quad8`/`PlateDkt` in
`📏️elements2d`, `Bar3`/`Frame3`/`Tet4`/`Hex8`/`ShellFacet3` in `🧊️elements3d`, plus `AxialSpring`
(see §3) in `🏗️model` itself. Verified the set myself per R11's instruction ("verify the impl set
yourself; if any impl lives outside this module the set is open ⇒ generics instead") — every impl is
local, so this is a genuinely closed set ⇒ `dyn_enum_close!` per R11's decision procedure, not generics.

**Applied** (`🏗️model/🦀️component.rs`, same module as the trait — required for the macro's bare,
textually-scoped `__semio_dispatch_Element!` invocation per the dyn-enum-macro report's finding 1):

```rust
use semio_framework_dispatch_macros::{dyn_enum, dyn_enum_close};
...
#[dyn_enum]
pub trait Element { ... }               // unchanged body

dyn_enum_close! {
    pub enum Elements: Element {
        AxialSpring(AxialSpring), Bar2(Bar2), BeamEb2(BeamEb2), Tri3Cst(Tri3Cst), Tri6Lst(Tri6Lst),
        Quad4(Quad4), Quad8(Quad8), PlateDkt(PlateDkt), Bar3(Bar3), Frame3(Frame3), Tet4(Tet4),
        Hex8(Hex8), ShellFacet3(ShellFacet3),
    }
}
```

Every `Box<dyn Element>`/`Vec<Box<dyn Element>>`/`&[Box<dyn Element>]` became `Elements`/
`Vec<Elements>`/`&[Elements]`; every `Box::new(X{..})`[` as Box<dyn Element>`] became `X{..}.into()`
(the enum's generated `From<VariantTy>` impls).

## 3. `AxialSpring` — promoted from a test-only mock to a 13th production variant

`impl Element for AxialSpring` lived inside `🏗️model`'s `#[cfg(test)] mod tests` — a minimal hand-
calculable 2-node, `[Tx]`-only spring used ONLY to test `solve_linear_static` without a real
geometry+material fixture. Once `Model.elements` is a CLOSED enum, a test-only implementor can no
longer sit outside the crate's real type set (the enum has no cfg-gated-variant support, and even if
it did, `Model`'s field type can't vary by build config). Two options: (a) rewrite the test to use a
production element (`Bar2`) with parameters chosen to reproduce the same closed-form numbers — risky
without being able to run the test (see §5, crate is SDK-gated) since `Bar2` needs `[Tx,Ty]` DOFs and
geometry, not `AxialSpring`'s DOF-free formula, so swapping would also need new supports to avoid a
newly-unconstrained `Ty` DOF; or (b) promote `AxialSpring` unchanged (same fields, same stiffness/
recover formulas, zero numeric change) to a real production element in `🏗️model` proper, since it is
in fact a legitimate, reusable FEM primitive (a linear axial spring/connector), not merely test
scaffolding. Took (b) — zero risk, zero recomputation, and arguably improves the element library by
making a previously test-only primitive available to real models. `two_spring_model()`'s only call site
change: `Box::new(AxialSpring{..})` → `AxialSpring{..}.into()`.

## 4. Files touched (all inside `✏️s/🔨️modules/🏗️fem/**`, the only owned writable path)

```
$ git diff --stat -- ✏️s/🔨️modules/🏗️fem
 ⚙️engine/◻2d/🎵️modal-buckling/🦀️component.rs    |   8 +-
 ⚙️engine/◻2d/🕸️meshing/🦀️component.rs           |  14 +--
 ⚙️engine/🏗️model/🦀️component.rs                 | 105 +++++++++++++--------
 ⚙️engine/📏️elements2d/🦀️component.rs            |  33 +++----
 ⚙️engine/🧊️3d/🎵️modal-buckling/🦀️component.rs   |   4 +-
 ⚙️engine/🧊️3d/🕸️meshing/🦀️component.rs          |  12 +--
 ⚙️engine/🧊️elements3d/🦀️component.rs            |  20 ++--
 ⚙️engine/🧮️analyses/🦀️component.rs              |  32 +++----
 8 files changed, 128 insertions(+), 100 deletions(-)
```
Every hunk hand-reviewed against `git diff` before reporting (pasted in full during the session — all
mechanical: import additions, `Box<dyn Element>`→`Elements` in type position, `Box::new(X)`→`X.into()`
at construction sites, one doc-comment + one test-fn rename reflecting the new mechanism name).

## 5. Repair tool (R10 — diagnostic/syntax-driven, saved into the ticket folder)

`📓️…/terra-boxnew-to-into.py` (this ticket folder). **Not name-keyed** — R10 bans matching on an
identifier that could collide with an unrelated same-named std/first-party item (the `.await`-inserter
disaster). `Box::new` is not that: it is a single, non-overloadable std path, so a byte-span scan that
finds `Box::new(`, walks forward to the BALANCED matching `)` (honoring nested `()/{}/[]` and string/
char literals — not a naive regex over the inner expression), and rewrites to `EXPR.into()` (dropping a
trailing ` as Box<dyn Element>` cast if present) is unambiguous by construction. Dry-run verified
against copies of all 5 affected files in the scratchpad first (diffed by hand, all 35 rewrites
correct — including the one multi-line `Hex8 { .. }` case spanning 7 source lines) before running for
real. Final counts: `analyses` 14, `elements2d` 8, `elements3d` 7, `◻2d/meshing` 3, `🧊3d/meshing` 3 = 35
(`🏗️model`'s one `Box::new(AxialSpring{..})` site was edited by hand alongside the promotion in §3, not
by the script). Remaining `Vec<Box<dyn Element>>`/`&[Box<dyn Element>]` type-position occurrences (14
across 7 files) were a separate, even-simpler literal-string replacement (no balancing needed).

## 6. Acceptance — `cargo check`, run once, **blocked upstream of my crate**

```
$ CARGO_TARGET_DIR=<scratchpad>/target-fem cargo check -p semio-s-plugin-fem --lib
   ... (630 error lines) ...
error: could not compile `semio-framework-ui` (lib) due to 31 previous errors; 2 warnings emitted
$ echo $?
101
```
**All 630 error lines are inside `semio-framework-ui`** (`🧰️framework/🔨️modules/🖱️ui`), a crate
upstream of `semio-s-plugin-fem` in the dependency graph (reached via `semio-framework-plugin`/
`semio-framework-os-kernel`) — pre-existing missing-`.await` residue unrelated to this packet (e.g.
`Some(input.value.clone())` typed against a `Future`-returning field). Confirmed by grepping the full
error output for `🏗️fem`, `dyn Element`, and `semio-s-plugin-fem`: **zero matches** — the build never
reaches any file this packet touched. This matches the ticket status line ("Only TWO crates now gate
the guest SDK, and everything else in the repo waits behind it") and the sibling `dyn-enum-macro`
report's own experience hitting the same kind of upstream gate.

**Reporting per the acceptance criteria: UNRUN for `semio-s-plugin-fem` itself, blocker named** — the
gate is `semio-framework-ui`'s own pre-existing async-conversion defects, not anything in this packet's
scope or paths. The mechanical correctness of this packet's own edits was instead verified by: (a) two
independent zero-`dyn-Element` searches (§0/§1), (b) full `git diff` hand-review of all 8 changed files
(§4), (c) a dry-run-then-diff-then-apply discipline for the 35 scripted rewrites (§5).

## 7. `lease-request`s — both needed before this can actually compile, neither is mine to edit

`✏️s/🔌️plugins/🏗️fem/**` (the PLUGIN crate that mounts these modules via `#[path]` in `📦️glue.rs`) is
outside this packet's owned paths (`✏️s/🔨️modules/🏗️fem/**` only).

````
lease-request:
  file: ✏️s/🔌️plugins/🏗️fem/📦️packages/🦀️rust/Cargo.toml
  section: [dependencies]
  after_line: 46   # "semio-framework = { path = "../../../../../🧰️framework/📦️packages/🦀️rust", package = "semio-framework" }"
  insert:
    semio-framework-dispatch-macros = { path = "../../../../../🧰️framework/🔨️modules/🔀️dispatch/📦️packages/🦀️rust" }

lease-request:
  file: ✏️s/🔌️plugins/🏗️fem/📦️packages/🦀️rust/📦️glue.rs
  insert (near the top, crate-root inner attributes):
    #![allow(async_fn_in_trait)] // R3/R7: `Element` is object-safe via #[dyn_enum] + the `Elements`
    // enum; guest futures stay ?Send structurally (dyn_enum-macro report, R3/R7). Never resolved by
    // `-> impl Future + Send` or by making the trait sync.
````
Without both, `use semio_framework_dispatch_macros::{dyn_enum, dyn_enum_close};` doesn't resolve and
the crate emits the `async_fn_in_trait` warning under `-D warnings` gates. Neither edit touches
`Element`'s definition or any call site — additive only.

## 8. Found in passing, NOT fixed (out of this packet's scope, flagged per rule 8/W4-#8)

`✏️s/🔨️modules/🏗️fem/⚙️engine/🏗️model/🦀️component.rs` (and, from a quick same-pattern grep, `🧮️analyses`
too) has **pre-existing missing-`.await` residue independent of this packet**, e.g.:
- `Model`'s `fmt::Debug` impl: `self.elements.iter().map(|e| e.id())` — `id()` is `async fn`, called
  from a sync (E1, external-trait) `Debug::fmt`; this was ALREADY broken before my edit (I only changed
  the element type flowing through the same broken line, `Box<dyn Element>` → `Elements` — same defect,
  same file, same line, not introduced or worsened by this packet).
- `two_spring_model()` (an `async fn`) called unawaited at every one of its ~9 call sites
  (`let model = two_spring_model();` then used directly as `&Model`).
- `solve_linear_static(&model)` (also `async fn`) called unawaited throughout the same test module.
- Note: since `dyn Element` with `async fn` methods is `E0038`-illegal (object-unsafe), **this whole
  crate could never have compiled as `dyn` in the first place** — the residue above and this packet's
  `dyn_enum_close!` conversion are two independent defects layered on the same broken file; fixing dyn
  does not fix the missing awaits, and vice versa. This is squarely the shape R10's residue-tool
  (`insert-await.py`) is meant for, applied once the crate is reachable enough to emit rustc
  diagnostics (currently blocked, §6) — flagging so whichever packet/gate owns `.await` insertion for
  this crate knows the file needs a full pass, not just the one call site I touched.

## Files touched

- `✏️s/🔨️modules/🏗️fem/⚙️engine/🏗️model/🦀️component.rs`
- `✏️s/🔨️modules/🏗️fem/⚙️engine/🧮️analyses/🦀️component.rs`
- `✏️s/🔨️modules/🏗️fem/⚙️engine/📏️elements2d/🦀️component.rs`
- `✏️s/🔨️modules/🏗️fem/⚙️engine/🧊️elements3d/🦀️component.rs`
- `✏️s/🔨️modules/🏗️fem/⚙️engine/◻2d/🕸️meshing/🦀️component.rs`
- `✏️s/🔨️modules/🏗️fem/⚙️engine/◻2d/🎵️modal-buckling/🦀️component.rs`
- `✏️s/🔨️modules/🏗️fem/⚙️engine/🧊️3d/🕸️meshing/🦀️component.rs`
- `✏️s/🔨️modules/🏗️fem/⚙️engine/🧊️3d/🎵️modal-buckling/🦀️component.rs`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME/📓️terra-fem-report.md` (this file)
- scratch tool: `.../MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME/terra-boxnew-to-into.py` (copied in below)

No file outside `✏️s/🔨️modules/🏗️fem/**` and the ticket folder was edited. `✏️s/🔌️plugins/🏗️fem/**`
needs the two lease-requests in §7 before this compiles; nothing else in the repo depends on
`✏️s/🔨️modules/🏗️fem` besides that one plugin crate (`grep -rn '🔨️modules/🏗️fem' --include=Cargo.toml
--include='*.rs' .` outside the module tree itself turns up only `✏️s/🔌️plugins/🏗️fem/📦️glue.rs`'s
`#[path]` mounts).
