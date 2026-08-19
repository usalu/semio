# terra — dedyn-fleet-cad

## Scope
Owned paths only: `✏️s/🔌️plugins/📐️cad/**`. No other plugin, no `🧰️framework/**`, no root manifests touched.

## Starting / ending counts
- Start: **24** first-party `dyn` uses, all `dyn BrepKernel` (confirmed against `sol-fleet-inventory.json`'s `📐️cad` entry: `"dyn": 24, "top": {"BrepKernel": 24}`).
- End: **0**. Verified with two differently-implemented fresh queries after all edits:
  1. `python3` `os.walk` over `✏️s/🔌️plugins/📐️cad`, code-only (comments stripped at `//`), regex-free substring scan for `dyn ` — **0 hits**.
  2. `grep -rn --include="*.rs" -E '\bdyn\b'` over the same tree — **exit 1 (no matches)**.

## Mechanism chosen: R11 case 3 — exactly one impl → delete the trait object, use the concrete type
Verified the single-impl claim myself before committing:
```
grep for `trait BrepKernel` / `impl BrepKernel for` across ✏️s/** and 🧰️framework/**
→ ✏️s/🔌️plugins/🗄️stdio/…/✳️brep/🧬️schema/⚙️engine/🦀️component.rs:140  pub trait BrepKernel
→ ✏️s/🔌️plugins/🗄️stdio/…/✳️brep/🧬️schema/⚙️engine/🦀️component.rs:1191 impl BrepKernel for Brep
```
Exactly one impl, `Brep` (`pub struct Brep`, same file, line 294), owned by `🗄️stdio` (not mine — I did not touch it). Per R11's decision procedure ("exactly one impl ⇒ delete the trait object, use the concrete type — an enum of one is worse than none"), a 92-arm `dyn_enum_close!` was never appropriate here; there is nothing to enumerate.

Every one of the 24 sites was a plain parameter-position `&dyn BrepKernel` / `&mut dyn BrepKernel` (never a trait-method return, never boxed) — R1's ban on `dyn Future` in return position doesn't even come up for this family. I traced the whole call chain: `cad_brep_kernel()` (in `🧬️schema/💡️inferences/🦀️component.rs`) already returns concrete `Brep` — every one of these 24 functions was already being fed a concrete `Brep` unsized-coerced up to `&dyn BrepKernel` at the call site. Removing the trait-object type removes a coercion that was never load-bearing; it does not change what value flows through any call site.

## Edits (5 files, 24 sites)
| file | sites | change |
|---|---:|---|
| `🚪️io/🦀️component.rs` | 2 | `&mut dyn BrepKernel` → `&mut Brep`; added `Brep` to the existing `use …engine::{block_on, BrepKernel, GeometryHandle}` |
| `🚪️io/🗺️geometry-import/🦀️component.rs` | 7 | same, `&mut dyn BrepKernel` → `&mut Brep` (all 7 were `&mut`) |
| `🧬️schema/💡️inferences/🦀️component.rs` | 8 (3× `&dyn`, 5× `&mut dyn`, across two production submodules `derive_transformation` and `scene_compute` — neither `#[cfg(test)]`) | `&dyn BrepKernel` → `&Brep`, `&mut dyn BrepKernel` → `&mut Brep`; `Brep` added to both local `use` imports (one per submodule) |
| `✏️editor/🦀️component.rs` | 2 | `&mut dyn BrepKernel` → `&mut Brep`; `Brep` added to import |
| `✏️editor/⚙️engine/🕹️interaction/🦀️component.rs` | 5 | `&mut dyn BrepKernel` → `&mut Brep`; import changed from single-item `use …engine::BrepKernel;` to `use …engine::{Brep, BrepKernel};` |

`BrepKernel` (the trait) stays imported everywhere — it's still needed in scope for method-call resolution (`kernel.box_prim(...)` etc. are trait methods, `Brep` has no inherent methods of the same names outside its own `_sync` variants). Only the *type position* changed, from the trait object to the concrete struct.

All replacements used `Edit` with the literal, disambiguated token `dyn BrepKernel` (never a bare identifier/name-keyed pattern — R10 doesn't apply here, this is a type-token substitution, not a `.await` insertion, and `dyn BrepKernel` has zero collision risk with any std construct). `replace_all` was used only after first counting occurrences fresh from disk and confirming every instance in that file matched the same literal pattern (`&mut dyn BrepKernel`, or split `&dyn`/`&mut dyn` in the inferences file, done as two separate targeted `replace_all` passes).

## `#![allow(async_fn_in_trait)]`
Not needed: `✏️s/🔌️plugins/📐️cad/**` declares **zero** first-party traits of its own (confirmed by search — no `trait ` declarations anywhere under the plugin root). `BrepKernel` is declared in `🗄️stdio`, out of my ownership; its `#[async_trait(?Send)]` attribute (R8 — async_trait must go) is still present there. That's a finding for whoever owns `🗄️stdio`/`asyncfleet-stdio`, not something in R8's already-measured 12-site table — flagging here since R8's table doesn't mention `BrepKernel`, and it should probably be added to that census.

## Acceptance
Tried the real build once, as instructed:
```
CARGO_TARGET_DIR=<scratchpad>/target-dedyn-fleet-cad cargo check -p semio-s-plugin-cad --all-targets
exit 101
```
Full output saved: `terra-dedyn-fleet-cad-cargo-check.txt` (this folder). The failure is **entirely inside `semio-framework-os-kernel`** (10 errors — `AsyncFnMut`/`AsyncFnOnce` "not general enough" HRTB issues and unresolved `.await` on `impl Future` in unrelated store/schema code), a dependency crate several layers upstream of `semio-s-plugin-cad`. `grep -n "Compiling semio-s-plugin-cad\|semio-s-plugin-cad"` and `grep BrepKernel` against the log both return nothing — the build never reaches my crate. This matches the documented "SDK not yet green" blocker; **acceptance is UNRUN for `semio-s-plugin-cad` itself**, verified structurally instead (see counts above). Not my crate, not touched, not worked around.

## Macro friction
None — `dyn_enum_close!` was never invoked. R11's third case (exactly one impl) applies cleanly and needs no macro; a 92-method enum-of-one would have been strictly worse per the ruling.

## Lease requests
None. No file outside `✏️s/🔌️plugins/📐️cad/**` was touched.

## For siblings
- `BrepKernel` in `🗄️stdio` still carries `#[async_trait(?Send)]` on both the trait declaration and its sole `impl … for Brep` (lines 139–140 and 1190–1191 of `✏️s/🔌️plugins/🗄️stdio/…/✳️brep/🧬️schema/⚙️engine/🦀️component.rs`). Not in R8's measured 12-site/6-file table — worth adding to that census for whoever owns stdio's async_trait removal.
- Any other plugin with a `dyn BrepKernel` seam (none found in cad, but if a sibling has one) can use the identical fix: `Brep` is `pub`, reachable via `semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::brep::schema::engine::Brep`, and there is still exactly one impl.
