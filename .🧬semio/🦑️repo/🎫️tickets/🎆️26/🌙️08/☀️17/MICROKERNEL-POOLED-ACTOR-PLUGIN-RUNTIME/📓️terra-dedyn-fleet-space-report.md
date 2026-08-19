# 📓️ terra — dedyn-fleet-space report

Packet: `dedyn-fleet-space`. Owned paths: `✏️s/🔌️plugins/🪐️space/**` (the plugin and its extension
crates) + this ticket folder.

## 0. Counts

Verified twice, two differently-implemented tools (python3 regex over absolute paths, and `grep -rnE`
over the same paths — both from `/Users/ueli/Documents/semio`, per rule 21's "negative result" warning),
comments excluded by inspecting each hit's line prefix.

| | before | after |
|---|---:|---:|
| `dyn OsBackbonePort` (code) | 10 | 4 |
| `dyn SpaceBackbonePort` (code) | 1 | 1 |
| **total code `dyn <first-party trait>`** | **11** | **5** |

All 11 starting occurrences matched the brief's own count exactly. The 5 residue occurrences are all in
`✏️s/🔌️plugins/🪐️space/🦀️component.rs` (lines 138, 139, 165, 183, 368) — `shared_studio_ports`,
`register_studio_port`, `register_studio_port_for_test`, `draft_backbone_port`. Every one is
individually documented in-file with a doc comment explaining exactly why it could not close, and every
one is a genuine cross-crate architectural blocker, not an oversight — see §2.

Fully eliminated everywhere else: `catalog_port_concrete`/`temp_catalog_port_concrete`/`catalog_port`/
`temp_catalog_port`/`sync_os_space_document_helper` in the plugin root, plus one test site each in
`🏠️home/…/✏️editor/🦀️component.rs` and `⚙️engine/🪐️space/🎮️commands/💾️set-active-example/🦀️component.rs`.

## 1. Mechanism chosen per family (R11's four cases)

**Read first, per the brief: `📓️terra-store-dedyn-report.md`.** `store-dedyn` already closed the
underlying `store::BackbonePort` family into `pub enum BackbonePorts { Memory(MemoryBackbonePort),
LocalStorage(LocalStorageBackbonePort) }` + match-delegating `impl BackbonePort for BackbonePorts`, in
`🧰️framework/🔨️modules/🏪️store/🦀️component.rs`. Neither `OsBackbonePort`
(`🧰️framework/🛍️products/💻️os/🖥️host/🦀️component.rs`) nor `SpaceBackbonePort`
(`🧰️framework/🔨️modules/🪐️space/🦀️component.rs`) is a trait THIS packet owns or declares — both are
declared in framework, each with a blanket `impl<T: store::BackbonePort> {Os,Space}BackbonePort for T`.
Since `BackbonePorts: store::BackbonePort` (store-dedyn's own work), `BackbonePorts` gets BOTH
`OsBackbonePort` and `SpaceBackbonePort` **for free**, with zero framework edits needed — exactly the
"blanket impl … must keep working" the brief called out. This is **not** the `dyn_enum_close!` macro
case (I don't own either trait's declaration, and the macro can't annotate a trait it doesn't own —
`📓️terra-dyn-enum-macro-report.md` finding 4/§5.4) — it's "closed set, enum already built by a sibling
packet, reuse it directly," the shape R11's decision procedure calls out as the top case whenever it
applies. No macro friction encountered because no macro was invoked here.

**Fixable family (6 of 10 OsBackbonePort sites + the 1 SpaceBackbonePort site is NOT in this group —
see §2):** every site backed *exclusively* by `MemoryBackbonePort`/`LocalStorageBackbonePort`, feeding
framework consumer functions (`list_os_space_catalog_entries`, `seed_os_space_catalog_if_empty`,
`load_os_space_document`, `create_os_space`) that all take `Arc<dyn OsBackbonePort>` **by value**.
Converted `catalog_port_concrete()`/`temp_catalog_port_concrete()`/`catalog_port()`/
`temp_catalog_port()`/`sync_os_space_document_helper()` (plugin root) and one construction site each in
`editor/🦀️component.rs`'s test module and `set-active-example/🦀️component.rs`'s test module to return/
hold `Arc<store::BackbonePorts>` directly. Unsizing coercion (`Arc<Concrete> -> Arc<dyn Trait>`) fires
automatically at each by-value call site — verified this is real, not assumed, by finding every call
site's actual signature in framework source before relying on it.

Two disambiguation edits were needed (not dyn-removal per se, but required by removing the dyn): once
`port` is the concrete `BackbonePorts` enum, `.read(uri)`/`.write(...)` via plain method syntax become
**ambiguous** — `OsBackbonePort` and `SpaceBackbonePort` are both `use`d in `🦀️component.rs` and both
apply to `BackbonePorts` via their respective blanket impls (E0034, multiple applicable items). Fixed
with explicit UFCS (`OsBackbonePort::read(&port, uri)`, `OsBackbonePort::write(port.as_ref(), ..)`),
matching the ORIGINAL trait each call site meant before the type was a bare object.

Also fixed two **pre-existing missing-`.await` bugs** on the exact lines this work touched (not part of
the 11-dyn target, but directly in the way): `Arc::new(LocalStorageBackbonePort::new())` and
`Arc::new(MemoryBackbonePort::new())` called `async fn ::new()` without `.await` in 3 places (plugin
root + both test sites). Rather than inserting `.await` (illegal in one of the three call sites anyway —
`temp_catalog_port_concrete`'s `OnceLock::get_or_init` closure is plain `FnOnce`, not async, so `.await`
there is E0728), switched all three to the equivalent **sync** `::default()` constructor — verified
behaviorally identical by reading both types' `Default` impls in `🏪️store/🦀️component.rs`: `store::
MemoryBackbonePort::new()` is literally `async fn new() -> Self { Self::default() }`, and
`LocalStorageBackbonePort`'s hand-written `Default` impl produces the exact same value its `new()` does.
No R9 tag needed — this isn't declaring anything sync that the ticket asyncified; it's choosing an
already-existing, already-sync, already-equivalent constructor over an async one that added nothing.

## 2. The 5 residue sites — genuine architectural blockers, not oversights, with evidence

Both remaining sub-families were tempting to "just convert" the same way as §1's fixable family, and
both turned out to be blocked by the SAME class of problem for two DIFFERENT underlying reasons. Neither
is fixable from `✏️s/🔌️plugins/🪐️space/**` alone.

**(a) `shared_studio_ports`/`register_studio_port`/`register_studio_port_for_test` (4 sites) — the
producer already erased the type before this file ever sees it.** These hold ports opened by
`semio_framework_os::open_folder_space_backbone`/`open_file_space_backbone`
(`🧰️framework/🛍️products/💻️os/🖥️host/🦀️component.rs:1980,1985`, out of this packet's owned path), both
declared as returning `Arc<dyn OsBackbonePort>` directly. I read their bodies: both wrap a **single**
concrete struct (also confusingly named `SpaceBackbonePort`, a struct in `host`, unrelated to the trait
of the same name in a different module) via `SpaceBackbonePort::folder(..)`/`::file(..)` — so the true
set is NOT open in the abstract, but it IS opaque from here: the function that erases it is out of my
path, `OsBackbonePort` carries no `Any` bound to downcast through, and the concrete type is never handed
to this file. There is no legal move available from inside `✏️s/🔌️plugins/🪐️space/**` — recovering the
concrete type, adding a downcast bound, or widening `store::BackbonePorts` all require editing files
outside this packet's path. Confirmed the registry never receives anything from `catalog_port()`/
`temp_catalog_port()` (which ARE closed and WERE converted) — only from these two open-producing
functions plus, in tests, `MemoryBackbonePort` directly (already itself covered by `BackbonePorts`, but
the registry's storage type is fixed by the OTHER two callers regardless).

**(b) `draft_backbone_port` (1 site) — the CONSUMERS take it BY REFERENCE, and unsizing coercion does
not fire through a reference.** Every real consumer — `draft_catalog_for`, `DraftCatalog::
list_drafts_sweeping_expired`, `DraftCatalog::discard_draft`
(`🧰️framework/🔨️modules/🪐️space/🦀️component.rs:1525,1430,1437`, out of path) — takes
`&Arc<dyn SpaceBackbonePort>`, not `Arc<dyn SpaceBackbonePort>`. This is a DIFFERENT root cause than (a):
here I own the type of the concrete value fully (`BackbonePorts::Memory`, closed, no openness at all) —
the blocker is purely a Rust coercion-semantics fact I did not want to assume, so I verified it against
real rustc in a throwaway probe (`coerce-probe/`, copied to
`terra-dedyn-space-coerce-probe.rs.txt` in this folder): `Arc<Concrete> -> Arc<dyn Trait>` coerces fine
BY VALUE (return position, `let` with explicit type, by-value fn argument) but **`&Arc<Concrete>` does
NOT coerce to `&Arc<dyn Trait>`** — real rustc E0308, "expected `&Arc<dyn Port>`, found `&Arc<Concrete>`".
Since `draft_catalog_for` &co. are declared with the reference in their OWN signature (framework, out of
path), `draft_backbone_port()`'s return type is pinned to `Arc<dyn SpaceBackbonePort>` regardless of what
it constructs internally.

Both residues are consistent with R11's own precedent (`kernel-ripple`'s associated-types finding): the
correct fix lives at the PRODUCER's boundary, not the consumer's. I did not reintroduce a boxed trait
object anywhere — these were never removed in the first place; they're genuinely un-reachable from this
packet's path. Filed as lease-requests below rather than worked around.

## 3. Macro friction

None — `dyn_enum_close!`/`#[dyn_enum]` were never invoked. Both `OsBackbonePort` and `SpaceBackbonePort`
are declared outside this packet's owned path, and per `📓️terra-dyn-enum-macro-report.md` finding 4 the
macro cannot annotate a trait it doesn't own. The applicable resolution was simpler than the macro
anyway: `store::BackbonePorts` already exists, already implements the right trait via a blanket impl, so
this packet just had to reference it — R11's "closed set" case, satisfied by reuse rather than
generation.

## 4. `#![allow(async_fn_in_trait)]`

This plugin crate (`semio-s-plugin-space`, root `📦️glue.rs`) DOES declare one first-party async trait —
`OsParameterId` (`⚙️engine/🪐️space/⚙️engine/🦀️component.rs:30`, `async fn id(&self) -> &str`), unrelated
to the 11-dyn target (never used as `dyn`) but still triggering the R7 lint. Added
`#![allow(async_fn_in_trait)]` at `📦️glue.rs` (the crate root — confirmed via its `Cargo.toml`
`[lib] path = "📦️glue.rs"`), with the R3/R7 rationale comment, immediately after the module doc comments
and before the `extern crate` lines.

## 5. Lease-requests

```lease-request
Owner: whichever packet owns 🧰️framework/🛍️products/💻️os/🖥️host/🦀️component.rs
Needed change: open_folder_space_backbone/open_file_space_backbone (:1980, :1985) currently return
  Result<Arc<dyn OsBackbonePort>, VcsError>, erasing their own internal `host::SpaceBackbonePort` struct
  (a SINGLE concrete type, not actually open) before any caller ever sees it. Changing either to return
  a concrete/enum type instead (e.g. Result<Arc<host::SpaceBackbonePort>, VcsError> directly, or folding
  a File/Folder variant into store::BackbonePorts if `host::SpaceBackbonePort` is made to implement
  store::BackbonePort) would let ✏️s/🔌️plugins/🪐️space's shared_studio_ports/register_studio_port/
  register_studio_port_for_test (currently Arc<dyn OsBackbonePort>, 4 residue sites) close fully.
Why I can't do this myself: 🖥️host/🦀️component.rs is out of this packet's owned path
  (✏️s/🔌️plugins/🪐️space/** only).
```

```lease-request
Owner: whichever packet owns 🧰️framework/🔨️modules/🪐️space/🦀️component.rs
Needed change: draft_catalog_for(:1525), DraftCatalog::list_drafts_sweeping_expired(:1430),
  DraftCatalog::discard_draft(:1437) all take port: &Arc<dyn SpaceBackbonePort> BY REFERENCE. Changing
  any one of: (a) taking Arc<dyn SpaceBackbonePort> by value instead, or (b) genericizing to
  <P: SpaceBackbonePort>(port: &Arc<P>) — would let ✏️s/🔌️plugins/🪐️space's draft_backbone_port
  (currently Arc<dyn SpaceBackbonePort>, its one remaining residue site) return the concrete
  store::BackbonePorts it already constructs internally, since &Arc<Concrete> does not coerce to
  &Arc<dyn Trait> (verified against real rustc, see §2(b) and terra-dedyn-space-coerce-probe.rs.txt).
Why I can't do this myself: 🧰️framework/🔨️modules/🪐️space/🦀️component.rs is out of this packet's owned
  path.
```

## 6. Something a sibling must know

Anyone converting a family whose value is stored **behind a reference parameter** in a consumer they
don't own should check the consumer's exact signature for `&Arc<dyn T>` vs `Arc<dyn T>` BEFORE assuming
the enum-swap trick works — the by-value case (the common one, e.g. every `store-dedyn` example) coerces
for free; the by-reference case does not, silently, until you actually try to compile it. I verified this
with a two-line rustc probe rather than assuming from memory (`terra-dedyn-space-coerce-probe.rs.txt`),
per the ticket's "validate your assumptions" rule — worth reusing that probe file directly rather than
re-deriving it.

## 7. Acceptance

**Structural (trustworthy):** two differently-implemented searches (python3 regex, `grep -rnE`), both
over absolute/relative paths from repo root, comments excluded — **5 code-level `dyn <first-party
trait>` remain**, down from 11, all individually documented and all traced to an out-of-path blocker with
evidence (§2). Zero `dyn` from any OTHER first-party trait anywhere in `✏️s/🔌️plugins/🪐️space/**`.

**`cargo check -p semio-s-plugin-space --lib`** (`CARGO_TARGET_DIR` in the session scratchpad per rule
24, foreground within one turn, ~220s):

```
$ CARGO_TARGET_DIR=<scratchpad>/target-dedyn-space cargo check -p semio-s-plugin-space --lib
[...]
error: could not compile `semio-framework-os-kernel` (lib) due to 129 previous errors; 9 warnings emitted
```
Full output saved as `terra-dedyn-space-check1.txt` in this folder (3209 lines). **Never reaches
`semio-s-plugin-space`** — every one of the 128 `error[...]` lines is inside `semio-framework-os-kernel`
(the `store` dependency), specifically `🏪️store/🔄️sync/🦀️component.rs` (missing-`.await`/async-fallout,
same signature as the COMPILE REALITY warning describes, and matching what `store-dedyn`'s own report
already hit from the same upstream crate). `grep -n "✏️s/🔌️plugins/🪐️space" terra-dedyn-space-check1.txt`
returns **zero matches** — confirmed the failure is entirely pre-existing upstream damage, not anything
introduced by this packet's edits. Per the ticket's instruction, reporting acceptance **UNRUN** with the
blocking crate named (`semio-framework-os-kernel`) rather than claiming a green check that didn't happen.

## 8. Files touched

- `✏️s/🔌️plugins/🪐️space/🦀️component.rs` — the 8 fixed sites + 4 documented residue sites (of the 4
  OsBackbonePort residues) + the two disambiguation UFCS edits.
- `✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs` —
  one test-site conversion + dropped now-unused `OsBackbonePort` import.
- `✏️s/🔌️plugins/🪐️space/⚙️engine/🪐️space/🎮️commands/💾️set-active-example/🦀️component.rs` — one
  test-site conversion.
- `✏️s/🔌️plugins/🪐️space/📦️packages/🦀️rust/📦️glue.rs` — added `#![allow(async_fn_in_trait)]` (§4).
- Ticket-folder scratch: `terra-dedyn-space-check1.txt` (cargo check log, blocked upstream),
  `terra-dedyn-space-coerce-probe.rs.txt` (rustc coercion probe, real output pasted inline).

No file outside `✏️s/🔌️plugins/🪐️space/**` and this ticket folder was edited. No `[DEBUG]` logs left
behind (none were added — no runtime debugging was needed for this packet).
