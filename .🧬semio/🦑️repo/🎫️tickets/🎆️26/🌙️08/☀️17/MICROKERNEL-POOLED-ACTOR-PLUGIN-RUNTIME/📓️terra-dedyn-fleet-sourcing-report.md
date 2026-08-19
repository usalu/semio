# 📓️ terra-dedyn-fleet-sourcing report

Packet: `dedyn-fleet-sourcing`. Owned paths: `✏️s/🔌️plugins/🪵️sourcing/**` + ticket folder only.

## 1. Counts

- **Starting `dyn <first-party trait>` count: 4** (verified against the brief's inventory).
- **Ending count: 0** in code; the string `dyn SourcingModule` survives only inside one doc comment
  (`schema/🦀️component.rs:569`, explaining the removal), which does not count under R1 ("comments
  excluded").

All 4 uses were in **one file**:
`✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curate/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️component.rs`
— `pub async fn sourcing_modules() -> Vec<Box<dyn SourcingModule>>` (declaration + `vec![Box::new(..)..]`
literal + one `.map(|m| Box::new(..) as Box<dyn SourcingModule>)`) and
`pub async fn module_for(..) -> Option<Box<dyn SourcingModule>>`.

Correction to the brief: the brief described the 4 impls as living "across sourcing's 3 extension
crates". They do not — **all 4 `impl SourcingModule for ..` blocks live in this same schema file**
(`beams`/`windows`/`slabs` are inline submodules of the schema component, plus a private
`ContributedSourcingModule`). The 3 extension crates under `🧩️extensions/{🪵️beams,🧱️slabs,🪟️windows}`
only *import* `SourcingModule` to call methods on the concrete `BeamsModule`/`WindowsModule`/`SlabsModule`
types directly (never `dyn`) when building their `ExtensionBundle` topic contribution — nothing there
needed editing.

## 2. Mechanism chosen — R11 decision procedure

**Closed set, 4 known implementors in one file ⇒ `dyn_enum_close!`.** Textbook case per R11 (all impls
known in one crate/module) — no generics, no associated types, no hand-written enum needed.

```rust
#[dyn_enum]
pub trait SourcingModule {
    async fn module_id(&self) -> &'static str;
    async fn label(&self) -> &'static str;
    async fn typology(&self) -> TypologyNode;
    async fn demo_kinds(&self) -> Vec<ObjectKind>;
    async fn preview_mesh(&self, kind: &ObjectKind) -> MeshDataSpec { .. } // default body, unaffected
}

dyn_enum_close! {
    pub enum SourcingModules: SourcingModule {
        Beams(beams::BeamsModule),
        Windows(windows::WindowsModule),
        Slabs(slabs::SlabsModule),
        Contributed(ContributedSourcingModule),
    }
}
```

`Vec<Box<dyn SourcingModule>>` → `Vec<SourcingModules>`, `Option<Box<dyn SourcingModule>>` →
`Option<SourcingModules>`; `Box::new(X)` constructions → `X.into()` (the generated `From<Variant>` impls).

## 3. Macro friction

None beyond what `📓️terra-dyn-enum-macro-report.md` already documents. One thing I had to apply myself
that report didn't need to (its worked example used only public structs): `ContributedSourcingModule`
was a private (non-`pub`) struct being folded into a `pub enum SourcingModules`. Left private, this trips
the `private_interfaces` lint (a struct less visible than the public item exposing it) — zero-warning bar
would fail. Fix: made the struct `pub` (fields stay non-`pub`; construction is still internal to the
module, only the type name is now nameable from outside). Documented in a comment at the struct so the
next reader doesn't wonder why a "hot-installed, internal" type is `pub`.

Followed the macro's own constraints throughout: bare (unqualified) `dyn_enum_close!` invocation, same
module, trait declared first (finding 1); `dyn_enum_close`, not `dyn_enum` (finding 2, E0428 avoided);
no cross-module `use crate::__semio_dispatch_*` needed since the closing site is in the same module as
the `#[dyn_enum]` trait.

## 4. Edits made

1. `✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curate/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️component.rs`
   — `use semio_framework_dispatch_macros::{dyn_enum, dyn_enum_close};`; `#[dyn_enum]` on the trait;
   `dyn_enum_close! { .. }` block after the last impl; `ContributedSourcingModule` → `pub`;
   `sourcing_modules`/`module_for` return types + constructors swapped to the enum.
2. `✏️s/🔌️plugins/🪵️sourcing/📦️packages/🦀️rust/📦️glue.rs` (crate root) —
   `#![allow(async_fn_in_trait)]` added (R7), with a comment naming R3/R7 and the concrete reason
   (`SourcingModule` is the trait that needed it).
3. `✏️s/🔌️plugins/🪵️sourcing/📦️packages/🦀️rust/Cargo.toml` — added
   `semio-framework-dispatch-macros = { path = "../../../../../🧰️framework/🔨️modules/🔀️dispatch/📦️packages/🦀️rust", package = "semio-framework-dispatch-macros" }`
   to `[dependencies]`. **No root-`Cargo.toml` lease needed** — `semio-framework-dispatch-macros` is
   already a registered workspace member (root `Cargo.toml` line 103), so this was a same-crate,
   in-scope edit only.

No `+ Send` was added anywhere (R3). No boxed-future alternative was used or considered (O1).

## 5. R2 tagging — deliberately-sync fns in these paths

Checked the whole owned tree for untagged non-`async` fns outside E1–E5. Found none needing a new tag in
code I touched; `leak_str`, `default_contributions_json` etc. are already `async fn` (blindly asyncified
by the fleet codemod, not something I reverted — see §6, that's a *missing-await* defect, not a
sync-tagging one).

## 6. Verification (two differently-implemented searches, comments excluded)

**Search A — python3, `os.walk` + regex, over absolute paths:**
```
$ python3 -c "... regex r'dyn\s+\w' over every *.rs under ✏️s/🔌️plugins/🪵️sourcing, target dirs excluded ..."
✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/…/🧬️schema/🦀️component.rs:569:  /// … (O1 — no `Box<dyn SourcingModule>`).
```
Exit: ran clean, one hit, a doc comment.

**Search B — `find` + `xargs grep` (independent implementation):**
```
$ find "✏️s/🔌️plugins/🪵️sourcing" -name '*.rs' -not -path '*🎯️target*' -print0 | xargs -0 grep -n 'dyn '
✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/…/🧬️schema/🦀️component.rs:569:  /// … (O1 — no `Box<dyn SourcingModule>`).
```
Same single comment-only hit. **Zero code-level `dyn <first-party trait>` remains.**

## 7. Compile reality

`CARGO_TARGET_DIR=<scratchpad>/target-dedyn-sourcing cargo check -p semio-s-plugin-sourcing --lib`
(foreground, one run, near the end as instructed):

```
...
Checking semio-framework-os-kernel v0.1.0 (…)
error[E0599]: no method named `map_err` found for opaque type `impl Future<Output = Result<protocol::MutationEnvelope, protocol::ProtocolError>>` in the current scope
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️component.rs:3485:58
    |
3485 |         crate::os_spr::decode_envelope(&bytes, &mut pos).map_err(serde::de::Error::custom)
    |                                                          ^^^^^^^ method not found in `impl Future<..>`
    = help: consider `.await`ing on the Future first
error: could not compile `semio-framework-os-kernel` (lib) due to 1 previous error; 9 warnings emitted
```

**Acceptance UNRUN — blocked upstream, in `semio-framework-os-kernel`** (`🏪️store/🦀️component.rs:3485`,
a missing `.await` residue, several dependency layers below my crate and entirely outside my path scope
— not touched, per the binding rule against editing outside owned paths). This matches the ticket's
documented compile reality (SDK gate not yet cleared). Structural proof instead: §6's two searches, plus
`rustfmt --check --edition 2021` on the edited schema file, which produced only pre-existing formatting
diffs (this repo's own `rustfmt.toml` wasn't passed) and **zero parse errors** — confirming the edit is
syntactically valid Rust. `Cargo.toml`'s new dependency line and the path it points at were verified to
resolve (`os.path.exists` on the normalized 5-levels-up relative path, `True`).

## 8. Residue worth flagging (out of my packet's scope — dyn only)

The whole `schema/🦀️component.rs` file (~950 lines, dozens of `async fn`) has **zero `.await`
occurrences** (`grep -c '\.await'` → `0`), confirming the fleet-wide async-signature codemod ran here but
per-file await-insertion has not. This includes calls I did NOT touch: `available_modules()` calling
`sourcing_modules().into_iter()` and `.module_id()`/`.label()`/`.typology()`/`.demo_kinds()` without
`.await`; `sync_sourcing_module_contributions` calling the now-`async fn leak_str` without `.await`; and
similarly in the sibling extension crates (`🧩️extensions/🪵️beams`, `🧱️slabs`, `🪟️windows`), whose
`bundle()` fns call `module.module_id()`/`.label()`/`.typology()`/`.demo_kinds()` without `.await` inside
a `serde_json::json!({..})` literal. None of this is new — it predates my edit and is unrelated to `dyn`
(the two functions I *did* edit, `sourcing_modules`/`module_for`, carry the identical pre-existing
missing-`.await` shape, left as-is since fixing it piecemeal by hand across a ~950-line file with no
compiling target to diagnose against is exactly the guessing R10 warns against — this needs the
`insert-await.py` span-keyed pass run against real rustc diagnostics once the SDK gate clears, plus a
sibling doing the equivalent pass for the 3 extension crates). Flagging here per the "cross-packet
findings must be lifted the moment they are read" rule (W4 item 8) — whichever packet owns sourcing's
await-insertion residue should start from this file.

## 9. Lease requests

None. Everything needed (the dispatch-macros crate as a workspace member, the path dependency in my own
crate's `Cargo.toml`) was already available or inside my owned paths.

## Files touched

- `✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curate/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️component.rs`
- `✏️s/🔌️plugins/🪵️sourcing/📦️packages/🦀️rust/📦️glue.rs`
- `✏️s/🔌️plugins/🪵️sourcing/📦️packages/🦀️rust/Cargo.toml`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME/📓️terra-dedyn-fleet-sourcing-report.md` (this file)
