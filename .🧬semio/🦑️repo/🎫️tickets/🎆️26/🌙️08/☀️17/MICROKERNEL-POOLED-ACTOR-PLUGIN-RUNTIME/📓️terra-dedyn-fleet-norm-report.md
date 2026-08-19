# 📓️ terra-dedyn-fleet-norm report

Packet: `dedyn-fleet-norm`. Owned paths: `✏️s/🔌️plugins/📕️norm/**` only.

## 1. Starting / ending `dyn` counts

- **Start**: 26 first-party `dyn` uses (`sol-fleet-inventory.json` entry for `📕️norm`, independently
  confirmed by my own scan before editing — see §3, method 1, run pre-edit).
- **End**: **0**. Verified after editing with two differently-implemented searches (§3) — both report 0
  `dyn <anything>` in code (comments excluded) anywhere under `✏️s/🔌️plugins/📕️norm/`.

Families, matching the brief exactly: `NationalAnnex` (25 uses, 9 methods, 2 impls — `NaDe`/`NaEn`) and
`ScriptRuntime` (1 use, 1 method, 1 impl — `DefaultScriptRuntime`).

## 2. Mechanism chosen per family, and why (R11's four cases)

### `NationalAnnex` — **mixed**: generics for parameters, `dyn_enum_close!` for the one runtime-chosen slot

All 25 uses are `&dyn NationalAnnex`. Reading every call site (not just the signature) split them into two
shapes that R11 already names explicitly:

- **22 of 25 — borrowed-reference function parameters, never boxed/stored/returned.** This is exactly
  R11(a), "trivially generic": `annex: &dyn NationalAnnex` → `<A: NationalAnnex>(annex: &A)`. Applied to
  19 functions in `🗿️artifacts/📘️en1990/…/🧬️schema/🦀️component.rs` (`psi_for_category`, `psi_for_imposed`,
  `gamma_for_situation`, `xi_for_situation`, the six `combination_*` fns, `check_combination`,
  `check_combination_set`, `check_uls_action`, `append_combination_set`, `check_design_basis`,
  `combination_6_12b`, `check_seismic_situation`) and 3 in `🗿️artifacts/📘️en1991/…/🧬️schema/🦀️component.rs`
  (`part_1_1::check_imposed`, `part_1_3::check_snow`, `part_1_4::check_wind`). Every one of these calls
  another generic fn in the same module with the same `annex` reference, so the type parameter propagates
  cleanly with zero call-site friction — verified by reading every call chain, not assumed. No macro needed
  for this half.

- **3 of 25 — a local variable that picks between `&NaDe`/`&NaEn` at runtime based on a `bool`/enum
  comparison** (`en1990/…/💡️inferences/🦀️component.rs:108`, `en1991/…/💡️inferences/🦀️component.rs:99,112`).
  This is NOT the generic-parameter shape — the concrete type is chosen by a runtime condition, which
  generics cannot express. This is the closed-set case R11 names for `dyn_enum_close!`. Applied it at the
  trait's home:
  - `📄️artifact/🦀️component.rs`: added `#[dyn_enum]` above `pub trait NationalAnnex { .. }` (9 methods, all
    `async fn`, some with default bodies).
  - `🗿️artifacts/📘️en1990/…/🧬️schema/🦀️component.rs` (where `NaDe`/`NaEn` — the only two impls — already
    live): closed the set right after the `NaEn` impl with
    `dyn_enum_close! { pub enum NationalAnnexes: NationalAnnex { De(NaDe), En(NaEn) } }`, invoked **bare**
    (never via `semio_framework_dispatch_macros::dyn_enum_close!`, per the shared macro report's finding 1
    / rustc#52234) with an explicit `use crate::__semio_dispatch_NationalAnnex;` immediately above it,
    since the closing site is a different module from the trait's declaration (`crate::document`) —
    exactly the cross-module case the report's recipe step 5 describes.
  - The three call sites became `let annex: NationalAnnexes = if cond { NaDe.into() } else { NaEn.into() };`
    and every downstream call that used to pass the bare `&dyn` reference now passes `&annex` (the enum is
    an owned value, not already a reference) — I traced each one by hand rather than doing a blind
    find/replace, since some call sites (`annex.choice()`) call a method on the value directly and needed
    no change.

  Why not use the enum for ALL 25 uses instead of splitting? Because R11(a) is explicit that
  parameter/reference positions are "trivially generic" and that is strictly simpler — no macro
  plumbing, no `Cargo.toml` dependency needed at 22 of the 25 sites, and it keeps the function signatures
  generic over any future third `NationalAnnex` impl without touching them again. The enum earns its keep
  only at the 3 sites where a concrete type is genuinely chosen at runtime.

### `ScriptRuntime` — **R11's "exactly one impl" case: delete the trait object**

`ScriptRuntime` has one method (`execute`) and exactly one impl, `DefaultScriptRuntime`
(`🗿️artifacts/📓️iso16757/…/🧬️schema/🦀️component.rs:543,551`), used at exactly one call site
(`calculate_part_number`, line 631). Per R11 ("an enum of one is worse than none"), the trait object was
deleted outright: `runtime: &dyn ScriptRuntime` → `runtime: &DefaultScriptRuntime`. No macro touched. The
trait itself (`pub trait ScriptRuntime { async fn execute(..); }`) stays as the documented contract; only
the erased pointer at the one call site is gone.

## 3. Verification — two differently-implemented searches, comments excluded

**Method 1 — Python, line-by-line, regex `\bdyn\s+\w+`, skips `//`-prefixed lines:**
```
$ python3 - <<'EOF'
import os, re
root = ".../✏️s/🔌️plugins/📕️norm"
pattern = re.compile(r'\bdyn\s+([A-Za-z_][A-Za-z0-9_:<>, \']*)')
hits = []
for dirpath, dirnames, filenames in os.walk(root):
    if '🎯️target' in dirpath: continue
    for fn in filenames:
        if fn.endswith('.rs'):
            for i, line in enumerate(open(os.path.join(dirpath, fn), encoding='utf-8'), 1):
                if line.strip().startswith('//'): continue
                if pattern.search(line): hits.append((...))
print(len(hits))
EOF
0
```
Exit observed: prints `total raw dyn-word hits (pre filter, code lines only): 0`.

**Method 2 — `grep -n 'dyn '` per file via `subprocess`, filtering `//`/`///` lines in Python (not the
regex engine):**
```
TOTAL non-comment 'dyn ' grep hits: 0
```

Both over the same 1,680-file tree, both python3-over-absolute-paths (per the standing rule that shell
globbing/grep over these emoji paths has previously under-reported), independently implemented (manual
regex scan vs. shelling out to `grep` and filtering separately), same result: **zero**.

Pre-edit, the identical method-1 script (run before any edit landed) printed exactly 26 hits, matching
`sol-fleet-inventory.json`'s `"dyn": 26` and its `"top": {"NationalAnnex": 25, "ScriptRuntime": 1}` —
recorded here as the starting-count proof.

## 4. `#![allow(async_fn_in_trait)]`

Added once, at the crate root (`📦️packages/🦀️rust/📦️glue.rs`, the file `[lib] path` points at), with a
comment citing R3/R7. This is the plugin's only crate (`semio-s-plugin-norm`), confirmed by
`find … -iname Cargo.toml` returning exactly one file — so one crate-root attribute covers the whole
plugin, including `NationalAnnex`/`ScriptRuntime` and every other async trait already in the plugin.

## 5. `Cargo.toml` dependency — no lease needed

`✏️s/🔌️plugins/📕️norm/📦️packages/🦀️rust/Cargo.toml` is inside my owned path (it is the plugin's own
manifest, not the registrar-only root), so I added
`semio-framework-dispatch-macros = { path = "../../../../../🧰️framework/🔨️modules/🔀️dispatch/📦️packages/🦀️rust", package = "semio-framework-dispatch-macros" }`
directly — no `lease-request` required. The root `Cargo.toml` already lists
`🧰️framework/🔨️modules/🔀️dispatch/📦️packages/🦀️rust` as a workspace member (confirmed by
`grep -n "🔀️dispatch" Cargo.toml` → line 103), so the workspace-membership half of the
`dyn-enum-macro` packet's lease-request was already granted by the time I got here; I did not need to ask
for it again.

## 6. Macro friction encountered

None beyond what the shared report already documents and that I followed exactly:
- Invoked `dyn_enum_close!` **bare** (via `use semio_framework_dispatch_macros::dyn_enum_close;` then a
  bare call), never via the qualified path, per the report's point 1.
- Used `dyn_enum_close!`, not `dyn_enum!` (point 2) — never wrote the wrong name to begin with.
- Added the explicit `use crate::__semio_dispatch_NationalAnnex;` at the closing site because it is
  cross-module from the trait's declaration (`crate::document` vs. the `en1990` schema module) — point 3.
- Never needed point 4 (annotating a trait I don't own) — `NationalAnnex` is declared inside my own owned
  path (`📄️artifact/🦀️component.rs`), so `#[dyn_enum]` applies directly.

No `compile_error!` from `dyn_enum_close!` was hit — `NationalAnnex` has no associated types/consts, no
`self`-less methods, no destructuring params, and does not mix a `self: Arc<Self>` method with `&mut self`
(all 9 methods take plain `&self`), so none of the macro's rejection cases applied.

## 7. Compile reality (per the ticket's standing rule)

Tried the real build once, foreground, own scratch target dir:
```
$ CARGO_TARGET_DIR=<scratchpad>/target-dedyn-norm cargo check -p semio-s-plugin-norm --lib
...
error[E0599]: no method named `map_err` found for opaque type `impl Future<Output = Result<protocol::MutationEnvelope, protocol::ProtocolError>>` in the current scope
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️component.rs:3485:58
    |
3485 |         crate::os_spr::decode_envelope(&bytes, &mut pos).map_err(serde::de::Error::custom)
    |                                                          ^^^^^^^ method not found in `impl Future<...>`
    = help: consider `await`ing on the `Future` ...
error: could not compile `semio-framework-os-kernel` (lib) due to 1 previous error; 9 warnings emitted
```
**Acceptance: UNRUN, blocked by `semio-framework-os-kernel`** — a missing `.await` at
`🏪️store/🦀️component.rs:3485`, entirely outside my owned path (`🧰️framework/**`, explicitly not mine).
The norm plugin's own source was never reached. Per the ticket's compile-reality rule I did not touch that
file — flagging it here as a cross-packet finding since it blocks anyone building through
`semio-framework-os-kernel` right now, not specific to this packet.

Syntax-checked every file I edited with `rustfmt --check --edition 2021` — all seven parsed cleanly (the
diffs rustfmt reported are pre-existing formatting deltas in code blocks I did not touch, not parse
errors — a parse failure would print `error:`, not a diff). The `Cargo.toml` edit was verified by locating
the added dependency's `path` on disk (`os.path.isdir`/`isfile` both `True`).

## 8. Files touched

- `✏️s/🔌️plugins/📕️norm/📦️packages/🦀️rust/Cargo.toml` — added `semio-framework-dispatch-macros` dependency
- `✏️s/🔌️plugins/📕️norm/📦️packages/🦀️rust/📦️glue.rs` — crate-root `#![allow(async_fn_in_trait)]`
- `✏️s/🔌️plugins/📕️norm/📄️artifact/🦀️component.rs` — `#[dyn_enum]` on `NationalAnnex`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️component.rs` —
  19 fns to generics, `NationalAnnexes` closing site
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️component.rs`
  — runtime-chosen site converted to `NationalAnnexes`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️component.rs` — 3
  fns to generics
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️component.rs`
  — 2 runtime-chosen sites converted to `NationalAnnexes`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📓️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️component.rs` —
  `ScriptRuntime` trait object deleted, concrete `DefaultScriptRuntime` used
- This file (new)

## 9. Nothing needed from a sibling / no `lease-request`

Both leases the `dyn_enum-macro` report anticipated needing were either already granted (workspace
membership) or fell inside my own owned paths (the plugin's `Cargo.toml`), so no `lease-request` was
required this packet.

## 10. What a sibling should know

- The `semio-framework-os-kernel` missing-`.await` at `🏪️store/🦀️component.rs:3485` blocks every
  downstream crate's `cargo check` right now (confirmed live, not from a stale report) — anyone whose
  acceptance run reaches that far will hit the identical error until it's fixed. Not mine to fix
  (outside `🧰️framework/**`), flagging per the standing "cross-packet findings must be lifted" rule.
- The `NationalAnnex` split (generics for parameters, enum only for the runtime-chosen slot) is a
  reusable pattern: any family that is *mostly* borrowed-reference parameters with a small number of
  "pick the concrete type based on a runtime condition" sites should get the same treatment rather than
  either extreme (all-generic, which cannot express the runtime choice; or all-enum, which adds macro
  plumbing to call sites that never needed it).
