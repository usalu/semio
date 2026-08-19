# 📓️ terra — dyn-os-backbone report

Packet: `dyn-os-backbone`. Owned path: `🧰️framework/🛍️products/💻️os/**`, families `OsBackbonePort`,
`SpaceBackbonePort`, `Emit` only.

## 0. Counts

Two differently-implemented searches (python3 `re` over absolute paths, and `grep`), comments excluded
by construction (both patterns match `dyn <Ident>` code tokens, not doc-comment prose separately checked
by eye).

| family | file | before | after |
|---|---|---:|---:|
| `dyn OsBackbonePort` | `🖥️host/🦀️component.rs` | 14 | **0** |
| `dyn SpaceBackbonePort` | `🔨️modules/🪐️space/🦀️component.rs` | 8 | **0** |
| `dyn Emit` | — | 21 (all in `🛢️db/**`) | **not touched — see §3** |

Repo-wide re-sweep of the whole `💻️os/**` tree for `dyn OsBackbonePort` / `dyn SpaceBackbonePort`
(python3 `re`, second pass after every edit): **zero hits anywhere**, code or comment.

## 1. `OsBackbonePort` — mechanism: hand-written enum dispatch (no macro)

`🖥️host/🦀️component.rs`. Two implementors found by reading, not assumed:
- blanket `impl<T: store::BackbonePort> OsBackbonePort for T` — closed via `store::BackbonePorts`
  (already built by `store-dedyn`, per `📓️terra-store-dedyn-report.md`).
- one direct impl, `impl OsBackbonePort for backbone::SpaceBackbonePort` (a **struct**, unrelated to the
  trait of the same name in the `space` module — confusing but confirmed by reading both).

Added, inside `mod host` right after the blanket impl:

```rust
pub enum OsBackbonePorts {
    Store(store::BackbonePorts),
    Space(crate::backbone::SpaceBackbonePort),
}

impl OsBackbonePort for OsBackbonePorts {
    fn read(&self, uri: &str) -> Result<Vec<u8>, VcsError> {
        match self {
            Self::Store(port) => OsBackbonePort::read(port, uri),
            Self::Space(port) => OsBackbonePort::read(port, uri),
        }
    }
    fn write(&self, uri: &str, payload: &[u8]) -> Result<(), VcsError> { /* same shape */ }
}
```

Hand-written rather than `dyn_enum_close!` (the macro from `📓️terra-dyn-enum-macro-report.md`): only 2
variants/2 methods, and hand-writing avoided adding a new `semio-framework-dispatch-macros` path
dependency to this crate's `Cargo.toml` for a two-arm match — lower risk, same shape store-dedyn itself
used for `Backbones`/`BackbonePorts`/`BackboneChannelPorts`.

Every `Arc<dyn OsBackbonePort>` (by value AND the 4 by-reference sites — `sync_backbone_document`,
`port_key`, `track_/untrack_os_space_backbone_uri`) → `Arc<OsBackbonePorts>`. Since I own every one of
these signatures myself (all declared in this same file), the by-reference vs by-value distinction that
blocked a sibling packet (see §2) never applied to me — I changed the type outright rather than relying
on unsizing coercion.

`open_folder_space_backbone`/`open_file_space_backbone` now return `Result<Arc<OsBackbonePorts>, VcsError>`
and construct `OsBackbonePorts::Space(SpaceBackbonePort::folder(..)?)` / `::file(..)?`. Added
`OsBackbonePorts` to the crate-root `pub use host::{...}` re-export list (it needs to be nameable from
`semio_framework_os::OsBackbonePorts` the same way `OsBackbonePort` already was).

One in-file test (`creates_and_lists_space_catalog_entries`) constructed a bare
`Arc::new(MemoryBackbonePort::new())` relying on coercion to `Arc<dyn OsBackbonePort>` — now
`Arc::new(OsBackbonePorts::Store(store::BackbonePorts::Memory(MemoryBackbonePort::new())))`.

### This closes an already-filed sibling lease-request

`✏️s/🔌️plugins/🪐️space/🦀️component.rs` (packet `dedyn-fleet-space`, already landed,
`📓️terra-dedyn-fleet-space-report.md`) left 4 residue `dyn OsBackbonePort` sites
(`shared_studio_ports`/`register_studio_port`/`register_studio_port_for_test`) specifically because
`open_folder_space_backbone`/`open_file_space_backbone` erased the type before that file ever saw it, and
filed a lease-request against exactly this file asking for a concrete/enum return type. That lease is now
satisfied structurally (the two functions return `Arc<OsBackbonePorts>`); closing the 4 residue sites on
the fleet side is that packet's own follow-up (`✏️s/🔌️plugins/**` is out of my owned path — not done by
me, per rule 3).

## 2. `SpaceBackbonePort` — mechanism: delete the trait object, use the concrete type (R11, single impl)

`🔨️modules/🪐️space/🦀️component.rs`. Read every `impl SpaceBackbonePort for` site: **exactly one**,
the blanket `impl<T: store::BackbonePort> SpaceBackbonePort for T`. Per R11 ("exactly one impl ⇒ delete
the trait object, use the concrete type"), no enum needed at all — `store::BackbonePorts` already
satisfies the blanket bound, so every `&Arc<dyn SpaceBackbonePort>` / `Arc<dyn SpaceBackbonePort>` became
`&Arc<store::BackbonePorts>` / `Arc<store::BackbonePorts>` directly: `expire_drafts`,
`list_drafts_sweeping_expired`, `discard_draft`, `promote_draft`, `demote_asset`,
`draft_catalog_port_key`, `draft_catalog_for`, plus the test helper `memory_draft_port` (now constructs
`store::BackbonePorts::Memory(store::MemoryBackbonePort::new())`). Two doc-comment mentions of the old
type were updated to match. The trait `SpaceBackbonePort` and its blanket impl are untouched — they still
provide `.read()/.write()` via static dispatch on the concrete enum, just never `dyn`.

### This closes the other half of the same sibling lease-request

`draft_backbone_port` in `✏️s/🔌️plugins/🪐️space/🦀️component.rs` was left returning
`Arc<dyn SpaceBackbonePort>` deliberately, because its consumers (`draft_catalog_for` &co., all in this
file) took the port **by reference**, and `&Arc<Concrete>` does not coerce to `&Arc<dyn Trait>` (verified
by that packet against real rustc, `terra-dedyn-space-coerce-probe.rs.txt`). Now that these signatures
take `&Arc<store::BackbonePorts>` (a concrete type, not `dyn`), that coercion problem is moot — the
fleet's `draft_backbone_port` can return the concrete `Arc<store::BackbonePorts>` it already constructs
internally and drop its own residue `dyn`. Again, landing that edit is out of my owned path
(`✏️s/🔌️plugins/**`) — not done by me.

## 3. `Emit` — NOT touched, scope conflict with an explicit hard restriction

Read first, per the brief. `trait Emit` and all 5 `impl Emit for` sites, and **all 21** `dyn Emit` code
occurrences (`🛢️db/⚙️engine` 4, `🛢️db/🎭️actor` 4, `🛢️db/👁️observe` 3, `🛢️db/📄️artifact` 1,
`🛢️db/🔒️security` 6, `🛢️db/🕸️version-graph` 3 — the trait itself is declared at
`🛢️db/🕸️version-graph/🦀️component.rs:121`) are **entirely inside `🛢️db/**`**. Nothing outside `🛢️db/**`
in the whole `💻️os/**` tree references `Emit` as this trait — every other `Emit` hit in the tree
(`plugin`, `spr`, `mcp`) is an unrelated concrete struct (`pub struct Emit<Mutation, ConfigMutation,
DraftMutation>`, command-dispatch result type, never `dyn`).

My own brief states, verbatim: **"⚠️ 🏪️store/**, 🚪️io/**, 🛢️db/**, 🔌️plugin/** are completed packets —
read them for shapes, do not re-edit."** — `🛢️db/**` explicitly named. Since the entire `Emit` family
lives inside that excluded path with zero reachable surface anywhere else, I could not convert it without
violating that restriction (and rule 3, "never edit outside your packet's path_scope"). I did not touch
`🛢️db/**`.

**This is a genuine scoping conflict, not an oversight** — flagging for the coordinator rather than
guessing: either `🛢️db/**`'s "completed" status doesn't actually cover `Emit` (in which case whoever owns
`db-dedyn`/`db-trait-flip` should pick it up — it's a closed 5-impl set, straightforward
`dyn_enum_close!` or hand-written match-delegation, same shape as `OsBackbonePorts` above), or this
packet's family list should not have named `Emit` at all. I did not guess; I did not edit.

## 4. Acceptance

**`cargo check -p semio-framework-os-kernel --lib`**, `CARGO_TARGET_DIR=<scratchpad>/target-dyn-backbone`,
foreground, one turn:

```
$ CARGO_TARGET_DIR=.../target-dyn-backbone cargo check -p semio-framework-os-kernel --lib
warning: `semio-framework-os-kernel` (lib) generated 417 warnings (run `cargo fix --lib -p semio-framework-os-kernel` to apply 9 suggestions)
    Finished `dev` profile [unoptimized] target(s) in 0.48s
EXIT:0
```
Full output saved: `terra-os-backbone-kernel-check.txt` (417 pre-existing missing-`.await` warnings, none
new, none fatal — same baseline as before any of my edits, re-verified by running this command both
BEFORE touching any file and again AFTER). **Stayed EXIT 0, unaffected either way** — neither
`🖥️host/🦀️component.rs` nor `🪐️space/🦀️component.rs` is mounted into this crate at all (`os-kernel`'s
own `📦️glue.rs` only mounts `dsl`/`pack`/`spr`/`vcs`/`directory`/`io`/`store`/`engine`/`inference`/
`semio`/`extension` — verified by reading it, not assumed).

**`cargo check -p semio-framework-os --lib`** (the crate my two edited files actually belong to,
`CARGO_TARGET_DIR` same scratchpad dir): run both before and after my edits.

```
EXIT:101 (both times) — error: could not compile `semio-framework-ui` (lib) due to N previous errors
```
Full output saved: `terra-os-backbone-host-check.txt` (4541 lines). **Every single `error[...]` in the
log is inside `semio-framework-ui`** (`🧰️framework/🔨️modules/🖱️ui/**`, a dependency of this crate via
the `ui_wgpu` package) — `grep` for `🖥️host/🦀️component.rs`/`🪐️space/🦀️component.rs` in the log returns
**zero matches**. This crate was already unbuildable before I touched anything (verified by running the
check first, unedited, and getting the identical failure signature — `semio-framework-ui`'s own
async-conversion fallout: `impl Future<Output = T>` returned where `T` is expected, `E0308`/`E0277`/
`E0369`/`E0382`, none of it mine). I cannot get a green (or even same-crate) compile signal for my own
family's edits from this command — same situation `store-dedyn` and `dedyn-fleet-space` both hit and
reported rather than chased. My own family's correctness rests on the static dyn-census above plus
careful reading (every replaced type checked against its real definition and every constructor site
found and fixed), not a compiler run.

## 5. A pre-existing bug I found but did NOT fix (out of scope, not introduced by me)

`OsBackbonePort`'s blanket impl (`impl<T: store::BackbonePort> OsBackbonePort for T`, unchanged by me)
and `backbone::SpaceBackbonePort`'s direct impl both call into now-`async fn` methods
(`store::BackbonePort::read/write`, `store::sync::FolderTextStorage`/`FolderSqliteStorage`'s `read`/
`write`/`read_pack`/`write_pack`) **without `.await`** — a `?`-on-`Future` type error, present on disk
before I touched the file (confirmed: `store::BackbonePort::read`/`write` are already `async fn` in the
live tree; the call sites calling them are not). This is `🏪️store/**`/`🏪️store/🔄️sync/**` territory
(explicitly "completed, do not re-edit" per my brief) reaching into my family's trait bodies — I left it
exactly as I found it rather than converting `OsBackbonePort`/`SpaceBackbonePort`'s own methods to
`async fn`, because doing so would cascade `.await` requirements through ~10 public functions in this
file AND every downstream caller in `✏️s/🔌️plugins/🪐️space/**` and `✏️s/🔌️plugins/🪐️space/🗿️artifacts/
🏠️home/**` (confirmed by grepping call sites — `create_os_space`/`delete_os_space`/
`list_os_space_catalog_entries`/`draft_catalog_for`/`promote_draft`/etc. are all called from ≥1 fleet
file each), which is out of my owned path. Not a new defect; not chased, per the same "elsewhere fallout,
report don't chase" precedent `store-dedyn` set. `semio-framework-os --lib` cannot currently prove or
disprove this either way (blocked earlier in the graph by `semio-framework-ui`, §4).

## 6. Lease-requests

None from me — the two lease-requests this packet's boundary touches were filed by `dedyn-fleet-space`
against my file and are now satisfied on my side (§1, §2); closing the fleet-side residue is that
packet's own follow-up, not mine to do (out of path).

## 7. Files touched

- `🧰️framework/🛍️products/💻️os/🖥️host/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🪐️space/🦀️component.rs`
- Ticket-folder logs: `terra-os-backbone-kernel-check.txt`, `terra-os-backbone-host-check.txt`, this report.

`🛢️db/**` (the `Emit` family's actual location) was read but **not edited** — see §3.
