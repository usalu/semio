# Surface Direct Type Ownership Audit

Current physical full-source/provider frames and every Surface symbol occurrence are retained in `🗑️generated/surface-direct-type-independent/source-frames-1.json`. This is source identity review; no Rust compiler or runtime execution was performed.

`DomainHover`, `DomainSelection`, and `SelectionMethod` are defined in General Replication `📡️wire/🦀️.rs` (lines 2635, 2591, 2450). The package root mounts that exact file as wire::frames, reexports frames through wire and wire through the root. Cargo package `semio-framework-replication` explicitly names its library `protocol`; the correct direct Rust import is `use protocol::{DomainHover, DomainSelection, SelectionMethod};`. OS Kernel's os_spr reexports protocol::wire and wire::* and then os_spr::* at its root. These paths name the exact same nominal declarations; no wrapper or conversion is needed.

`Viewport2d` is defined in General UI Viewport `◻️2d/🧬️schema/🦀️.rs`, mounted/reexported by the viewport domain root. Its package `semio-framework-ui-viewport` uses that domain root as its [lib] path. OS Kernel explicitly reexports that same crate's Viewport2d. The correct direct import is `use semio_framework_ui_viewport::Viewport2d;`.

Surface Cargo needs normal direct dependencies `semio-framework-replication = { path = "../../../📡️replication/📦️packages/🦀️rust" }` and `semio-framework-ui-viewport = { path = "../../../🖱️ui/🪟️viewport/📦️packages/🦀️rust" }`, unless using already defined exact workspace dependencies. No feature is required for these types. In particular, the replication library name is protocol, not semio_framework_replication.

All executable Surface references to these four types are in NodeGraph and its original unit-law file. They cover public viewport fixture/session values, selection gather method/label mapping, sync_interaction inputs, bridge-created selection/hover records, and original tests/serde oracle. Paint and TiledMap references are documentation only. Replacing the sole NodeGraph import preserves nominal public argument/field/result identities and all existing tests. This does not establish removal of other OS Kernel or Infinite Canvas dependencies: Surface still uses its Store alias and product canvas APIs.

## Direct Import Stage Admission

Independent review admits surface-direct-types-source-1.json: both current predecessor bodies/hashes and all8 defining contexts exact. NodeGraph changes only the sole four-type import to exact defining providers. Independent TOML parse proves preexisting manifest data exact after removing two added normal dependencies; both relative paths resolve to exact named packages. Publication, metadata and owning whole runtime remain pending.
