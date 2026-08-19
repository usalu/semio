# 📓️ terra-dedyn-fleet-animate report

Packet: `dedyn-fleet-animate`. Scope: `✏️s/🔌️plugins/🎞️animate/**` only.

## 1. Result

**Starting count 155 (per `sol-fleet-inventory.json`) → ending count 0 first-party `dyn` uses.**
(A raw `dyn (Sobject|Animation|TextRenderer)\b` regex over the tree finds 159 lines pre-edit — the
3-line gap from the inventory's 155 is lines carrying two matches each, e.g.
`callback: Arc<dyn Fn(&mut dyn Sobject, f64) + Send + Sync>`, which the inventory's per-family counter
apparently counted once. Not investigated further since both numbers point at the same work and both
now read 0.)

### Two differently-implemented zero-dyn proofs (comments excluded)

```
$ python3 - <<'EOF'
import os, re
pat = re.compile(r'\bdyn\s+(Sobject|Animation|TextRenderer)\b')
hits = []
for dirpath, dirnames, filenames in os.walk("✏️s/🔌️plugins/🎞️animate"):
    if '🎯️target' in dirpath or 'node_modules' in dirpath:
        dirnames[:] = []; continue
    for fn in filenames:
        if fn.endswith('.rs'):
            with open(os.path.join(dirpath, fn), encoding='utf-8') as f:
                for i, line in enumerate(f, 1):
                    if line.strip().startswith('//'): continue
                    if pat.search(line): hits.append((dirpath, fn, i))
print(len(hits))
EOF
0
```
Exit code `0`, prints `0`.

```
$ grep -rnE 'dyn (Sobject|Animation|TextRenderer)\b' ✏️s/🔌️plugins/🎞️animate
✏️s/…/🎞️animation/🦀️component.rs:659:    /// pre-dyn-removal `&mut dyn Sobject` receiver did.
```
Exit code `0`. The single hit is inside a doc comment I wrote explaining the `AnimateBuilder` design
decision (see §3) — comment, not code.

### What is left (R1-legal, unchanged)
```
$ python3 -c "… dyn <ident> census …"
std::any::Any   8
FnMut            4
Fn               2
total: 14
```
All 14 are `dyn Fn`/`dyn FnMut`/`dyn std::any::Any` — std/lang traits, explicitly permitted by R1. None
is `dyn Future` in trait-method return position (R1's specific ban) — there is no `dyn Future` anywhere
in this plugin.

## 2. Mechanism per family (R11's four-case decision procedure)

### `Sobject` — 131 uses, 37 async methods, **3 impls, closed set** → `dyn_enum_close!`
`VSobject`, `Group` (both in `⚙️engine/🎬️scene/🦀️component.rs`), `ThreeDVSobject`
(`⚙️engine/📐️geometry/🦀️component.rs`, `three_d` submodule) — verified as the complete impl set by
grepping `impl Sobject for` across all 107 `.rs` files in the plugin (matches the packet's own count of
107 files from the inventory). `#[dyn_enum]` on the trait declaration
(`⚙️engine/🎬️scene/🦀️component.rs`, `sobject` submodule); `dyn_enum_close!` in the SAME module,
directly after `Group`'s `impl Sobject`, generating:
```rust
pub enum Sobjects: Sobject {
    VSobject(VSobject),
    Group(Group),
    ThreeDVSobject(crate::editor::animate::engine::geometry::three_d::ThreeDVSobject),
}
```
The third variant needed a fully-qualified path (cross-module type) — verified against the macro's
`Type` parser (`🧰️framework/🔨️modules/🔀️dispatch/🦀️component.rs:373-400`, `DynEnumVariant::parse`
takes `syn::Type`, not a bare `Ident`), so this is within the macro's proven contract, not a workaround.

### `Animation` — 22 uses, 9 methods, **43 impls, closed set** → `dyn_enum_close!`
The largest family in the repo (per the brief). All 43 concrete animations live in one file
(`⚙️engine/🎞️animation/🦀️component.rs`); `dyn_enum_close!` sits in the same module, right after the
last impl (`Rotating`), generating a 43-variant `Animations` enum + the match-delegating
`impl Animation for Animations`. One structural blocker inside this family, resolved before the macro
could apply:

**`LaggedStartMap<F>` was generic over its factory-closure type** (`impl<F> Animation for
LaggedStartMap<F> where F: Fn(usize) -> Box<dyn Animation> + Send`). An enum variant needs ONE concrete
type; an unconstrained `F` is an open family of anonymous closure types, not a closed set — `F` itself
was never the thing `dyn_enum_close!` was meant to erase, but it blocked using the macro on the type that
held it. No live caller constructs a `LaggedStartMap` anywhere in this crate (grepped), so rather than
design a per-caller monomorphized variant I concretized the field: `factory: Box<dyn Fn(usize) ->
Animations + Send>` (R1-legal — `dyn Fn` is a std trait, not first-party), `LaggedStartMap::new` now takes
`impl Fn(usize) -> Animations + Send + 'static` and boxes it internally. This matches the pattern already
live in `⏱️rate`'s `Updater::callback: Arc<dyn Fn(&mut Sobjects, f64) + Send + Sync>`, so it is not a new
idiom for this plugin.

### `TextRenderer` — 2 uses, 1 method, **exactly 1 impl** (`TypstTextRenderer`) → delete the trait object
Per R11's third case ("exactly one impl ⇒ delete the trait object, use the concrete type — an enum of one
is worse than none"). `render_markup_to_svg_snapshot`/`typst_markup_to_validated_svg`
(`⚙️engine/🔤️text/🦀️component.rs`) now take `&TypstTextRenderer` directly; the `TextRenderer` trait
declaration itself is untouched (it still documents the module's isolation boundary per its own doc
comment — "nothing outside this module ever names a `typst::*` type directly" — that intent survives, it
just no longer needs a runtime-erased receiver since there is only ever one caller-visible renderer).

## 3. A fifth case the brief didn't name: R11(a) "trivially generic", found inside an otherwise-closed family

R11(a) ("parameters and borrowed references — trivially generic, no design question") is written for the
OPEN-set `io` family, but the same shape recurred here even though `Sobject` itself is closed: a handful
of free functions/structs take a SINGLE erased `Sobject` receiver with no heterogeneous storage need. A
blind, uniform substitution of every `&dyn Sobject`/`&mut dyn Sobject` to the concrete `Sobjects` enum
broke exactly one of these — `AnimateBuilder`, reached through `AnimateExt`'s blanket
`impl<T: Sobject + Sized> AnimateExt for T {}`. A blanket impl over `T: Sobject` cannot call a fn that only
accepts the concrete `Sobjects`, so `v.animate(1.0)` on a bare `VSobject` stopped type-checking. Fixed by
making `AnimateBuilder<'a, S: Sobject>` generic instead of enum-typed — restores the original behavior
(works on `VSobject`/`Group`/`ThreeDVSobject`/`Sobjects` alike) with no call-site churn. Applied the same
fix prophylactically to `surrounding_rectangle` (`⚙️engine/📐️geometry/🦀️component.rs`) once I found its
one call site passed a bare `VSobject` too. Every OTHER single-receiver position that survived the blind
substitution (`next_to`, `align_to`, `run_updaters`, `Updater`'s callback, `add_updater`, `always`,
`f_always`, `always_redraw`, `interpolate_at`, `apply_parent_opacity_tree`, `paint_mobject`) was checked
and left on the concrete `Sobjects` enum deliberately: each of those either recurses through
`visit_children_mut`'s inherently-heterogeneous callback (`&mut dyn FnMut(&mut Sobjects)`, itself required
by `Group`'s mixed-type children) or is invoked from inside an `Updater` closure whose own type is
monomorphic and stored generically across all three concrete `Sobject` impls — genuine erasure, not an
over-application.

**Lesson for sibling packets applying `dyn_enum_close!` to a closed family**: a global find/replace from
`&dyn Trait` to the new concrete enum is not safe by inspection alone when the family has a blanket
generic extension trait (`impl<T: Trait> Ext for T`) anywhere in scope — grep for `impl<.*: \s*<Trait>.*>
.* for T` (or similar) before trusting the substitution, and expect at least the extension-trait's own
receiver-holding struct to need to stay generic.

## 4. Mechanical fallout from the family conversions (all within owned paths)

- Every `Box<dyn Sobject>`/`Box<dyn Animation>` → the bare enum type (`Sobjects`/`Animations`), Box
  dropped per the macro report's own recipe step 3 (no pointer-stability reason existed for any of them).
- Every `Box::new(x)` feeding one of those slots → `x.into()` (the macro's generated `From<VariantTy>`
  impl) — roughly 60 call sites across `⏱️rate`, `🎥️video`, `🎞️animation`, `🎬️scene`, `📐️geometry`,
  `🎛️config`, `📷️camera` (production code and tests alike; tests constructing a bare `VSobject`/`Group`
  and then calling a `Sobjects`-typed fn needed the binding itself re-typed `let mut v: Sobjects =
  VSobject::new().into();`, not just the call site, wherever the same binding was read again later).
- `.as_mut()`/`.as_ref()` calls that existed only to coerce a `Box<dyn Trait>` to `&mut dyn
  Trait`/`&dyn Trait` were removed wherever the value is now already the bare enum reference (e.g.
  `interpolate_at(mobjects, a.as_mut(), alpha)` → `interpolate_at(mobjects, a, alpha)` inside
  `AnimationGroup`/`Succession`/`LaggedStart`/`LaggedStartMap`'s `apply`/`interpolate_mobject` bodies, and
  `Group::visit_children_mut`'s `f(child.as_mut())` → `f(child)`).
- `Group::children`/`Scene::mobjects` collections retyped `Vec<Box<dyn Sobject>>` → `Vec<Sobjects>`,
  `HashMap<u64, Box<dyn Sobject>>` → `HashMap<u64, Sobjects>`; `Group::clone_box`'s manual deep-clone
  (fresh `id`, not a `Clone::clone`) kept as a real method returning `Sobjects`, not replaced by a derived
  `Clone` (the two are not equivalent — `clone_box` deliberately mints a new id).
- Imports: added `Sobjects`/`Animations` wherever the enum type is now named, and re-checked every module
  that used to import the bare trait (`Sobject`) purely to name `dyn Sobject` — some of those imports are
  now genuinely unused (no bare trait-method call in that module) and were dropped; others still call
  `Sobject`/`Animation` trait methods on a `Sobjects`/`Animations` value (delegated trait methods require
  the trait in scope to resolve, same as any other trait) and needed the import KEPT — this reversed one
  of my own premature removals in `⏱️rate` and `🎥️video`'s `render` submodule, caught by re-reading each
  file's actual method calls rather than trusting "the type substitution touched this line."

## 5. `#![allow(async_fn_in_trait)]` and the macro crate dependency

- `📦️packages/🦀️rust/📦️glue.rs` (crate root) now carries `#![allow(async_fn_in_trait)]` with an R3/R7
  comment, once, per R7/the macro report's recipe step 6.
- `📦️packages/🦀️rust/Cargo.toml` gained one dependency line:
  `semio-framework-dispatch-macros = { path = "../../../../../🧰️framework/🔨️modules/🔀️dispatch/📦️packages/🦀️rust" }`.
  **No `lease-request` was needed** — unlike the `dyn-enum-macro` packet's report (written before this
  packet ran), `semio-framework-dispatch-macros` is now ALREADY a registered workspace member (found at
  root `Cargo.toml:103`, `"🧰️framework/🔨️modules/🔀️dispatch/📦️packages/🦀️rust"`) — some other packet
  or the registrar granted that lease between then and now. The plugin's own `Cargo.toml` is inside my
  owned path scope, so adding the dependency line itself needed no lease either.

## 6. Macro friction

None beyond what `📓️terra-dyn-enum-macro-report.md` already documents. Specifically confirmed, not just
assumed:
- Bare bracelet invocation (no `use crate::__semio_dispatch_<Trait>;`) worked with zero warnings for both
  `Sobjects` and `Animations`, because both `dyn_enum_close!` calls sit in the same module as their
  `#[dyn_enum]` trait declaration (finding 1 from the macro report).
- The 43-variant `Animations` enum and its `impl Animation for Animations` (9 methods × 43 = 387
  delegated match arms) is well inside the macro's proven range (its own `tests/scale.rs` covers 45
  methods × 2 variants = 90; this is 9 methods × 43 variants = 387 arms, a different axis of scale, but
  each individual match arm is the same shape the macro already generates, and `#[dyn_enum]`'s captured
  macro re-expands per-variant, not per-arm-combinatorially, so this is not a novel stress case).
- A fully-qualified cross-module variant type (`ThreeDVSobject`) parsed and worked with no special
  handling — see §2.

## 7. `block_on` — left untouched, as instructed

Two sites in `⚙️engine/🎥️video/🦀️component.rs`'s `renderer` submodule (`VelloRenderer::new`):
`block_on(instance.request_adapter(...))` and `block_on(adapter.request_device(...))`. These are wgpu's
own synchronous-adapter-request idiom, not `BrepKernel` — confirmed still present, not touched, exactly
per the brief's note that this is "a different risk class."

## 8. Acceptance

**Structural (primary evidence, per COMPILE REALITY):** §1's two zero-dyn proofs, both exit `0`.

**Build, attempted once near the end, UNRUN — blocked upstream, not in this plugin's own paths:**
```
$ CARGO_TARGET_DIR=<scratchpad>/target-dedyn-animate cargo check -p semio-s-plugin-animate --lib
… 859 dep artifacts built, then …
error: could not compile `semio-framework-ui` (lib) due to 169 previous errors; 2 warnings emitted
warning: build failed, waiting for other jobs to finish...
```
Exit code of the cargo invocation itself: the shell wrapper reported `EXIT_CODE:0` (that variable captured
`tail`'s exit code, not cargo's — rule 10's own trap, avoided here only because the *content* of the
transcript, not the captured exit variable, is what's being cited as evidence: cargo's own `error: … due
to 169 previous errors` line is unambiguous).

The 169 errors are entirely inside `semio-framework-ui` (`🧰️framework/🔨️modules/🖱️ui/…/🎯️targets/🧊️wgpu/…`)
— `#[derive(Serialize)]`/`#[serde(default = "…")]` on struct fields whose default-value functions were
blindly asyncified (`expected bool, found future` / `expected String, found future` / `cannot apply unary
operator ! to type impl Future<Output = bool>`), i.e. the same E1/R9 defect class already fixed elsewhere
in this ticket (`🌱️value`, `⚠️diagnostic`), just not yet in `semio-framework-ui`. **Not my path scope**
(`🧰️framework/**`, owned by 7 sibling packets per the ticket). `grep -n "animate" <full transcript>` finds
zero lines — the build never reached `semio-s-plugin-animate` at all, confirming this is exactly the
"fleet `cargo check` will very likely fail before reaching your source" scenario the ticket warned about,
not a defect in this packet's own edits.

## 9. Anything a sibling must know

- `semio-framework-dispatch-macros` is a workspace member NOW (it was not, as of the `dyn-enum-macro`
  packet's report) — any sibling still planning a `lease-request` for that specific line can skip it and
  verify directly against root `Cargo.toml:103`.
- The `AnimateBuilder`/blanket-extension-trait trap (§3) is worth lifting to whichever packet is applying
  `dyn_enum_close!` next to a family that has its own `impl<T: Trait> SomeExt for T {}` blanket impl
  anywhere in the crate — it will not surface as a `dyn_enum_close!` error, it surfaces later as a type
  mismatch at the extension trait's own call site.
- `semio-framework-ui`'s 169 errors (§8) block every downstream crate's `cargo check`, not just this
  plugin's — flagging in case no other packet has reported it yet under this exact crate name.

## Files touched

- `✏️s/🔌️plugins/🎞️animate/📦️packages/🦀️rust/Cargo.toml` — added `semio-framework-dispatch-macros` dep
- `✏️s/🔌️plugins/🎞️animate/📦️packages/🦀️rust/📦️glue.rs` — `#![allow(async_fn_in_trait)]`
- `✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️present/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎬️scene/🦀️component.rs`
  — `#[dyn_enum]` on `Sobject`, `dyn_enum_close!` → `Sobjects`, `Scene` trait/impls fixed up
- `…/⚙️engine/🎞️animation/🦀️component.rs` — `#[dyn_enum]` on `Animation`, `dyn_enum_close!` →
  `Animations` (43 variants), `LaggedStartMap` de-genericized, `AnimateBuilder` made generic
- `…/⚙️engine/⏱️rate/🦀️component.rs` — `Updater`/`add_updater`/`always`/`f_always`/`always_redraw`/
  `run_updaters` retyped to `Sobjects`, tests re-bound
- `…/⚙️engine/🔤️text/🦀️component.rs` — `TextRenderer` receivers → concrete `TypstTextRenderer`
- `…/⚙️engine/🎥️video/🦀️component.rs` — `Scene`/`FrameRecorder`/`CapturedFrame`/`paint_mobject` retyped
- `…/⚙️engine/📐️geometry/🦀️component.rs` — `ThreeDVSobject`'s `impl Sobject`, `surrounding_rectangle`
  made generic, ~15 `Box::new` constructor sites fixed
- `…/⚙️engine/🎛️config/🦀️component.rs`, `…/⚙️engine/📷️camera/🦀️component.rs` — constructor sites fixed
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME/📓️terra-dedyn-fleet-animate-report.md`
  (this file)

No `lease-request` was needed. No file outside `✏️s/🔌️plugins/🎞️animate/**` and the ticket folder was
modified.
