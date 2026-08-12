# W0 Shim Inventory

Generated: 2026-08-12
Ticket: `26/08/12/SUBSET-CONFORMANCE-AND-INTEGRATED-ROUNDTRIPS`

## Summary

| Metric | Count |
|--------|------:|
| `📦️glue.rs` files scanned | 33 |
| Plugins with pre-migration shim blocks | 33 |
| Total shim blocks (marked comment) | 82 |

## Shim Pattern Taxonomy

### A. Primary pre-migration blocks

Marked with `// ---- Shims: keep pre-migration module paths resolving for external callers ----`.

### B. Trailing type re-exports

Some artifacts add `pub use ...::{Snapshot,Mutation,Diff}` immediately after shim blocks.

### C. Crate-root path shims (energy)

`🔋️energy` L31–~130: 50 flat `#[path]` declarations to engine subdirs without subset prefix.

### D. Internal duplicate module aliases (animate)

`🎞️animate` L58–83: `pub mod animate { ... }` legacy alias tree.

## Per-File Shim Blocks

### `✏️s/🔌️plugins/✒️writer/📦️packages/🦀️rust/📦️glue.rs`

**Blocks:** 1

#### Block 1 — artifact `any` (L282–L306)

**Contains:** engine, op/dsl/spr wire

```rust
        // ---- Shims: keep pre-migration module paths resolving for external callers ----
        pub mod schema {
            pub use super::standards::v1::subsets::any::schema::*;
        }
        pub mod engine {
            pub use super::standards::v1::engine::*;
        }
        pub mod io {
            pub use super::standards::v1::subsets::any::io::*;
        }
        pub mod op { pub use crate::artifacts::writer::standards::v1::subsets::any::schema::mutations::text::*; pub use crate::artifacts::writer::standards::v1::subsets::any::schema::mutations::WriterMutation; }
        pub mod dsl { pub use crate::artifacts::writer::standards::v1::subsets::any::schema::snapshot::text::*; }
        pub mod spr { pub use crate::artifacts::writer::standards::v1::subsets::any::schema::mutations::binary::*; }
        pub mod pack { pub use crate::artifacts::writer::standards::v1::subsets::any::schema::snapshot::binary::*; }
        pub mod diff { pub use crate::artifacts::writer::standards::v1::subsets::any::schema::diff::*; pub mod schema { pub use crate::artifacts::writer::standards::v1::subsets::any::schema::diff::*; } pub mod text { pub use crate::artifact // ...
        pub mod mutations { pub use crate::artifacts::writer::standards::v1::subsets::any::schema::mutations::*; }
        pub mod snapshot {
            pub mod schema { pub use crate::artifacts::writer::standards::v1::subsets::any::schema::snapshot::*; }
            pub mod pack { pub use crate::artifacts::writer::standards::v1::subsets::any::schema::snapshot::binary::*; }
        }
        pub use crate::artifacts::writer::standards::v1::subsets::any::schema::snapshot::WriterSnapshot;
        pub use crate::artifacts::writer::standards::v1::subsets::any::schema::mutations::WriterMutation;
        pub use crate::artifacts::writer::standards::v1::subsets::any::schema::diff::WriterDiff;
```

### `✏️s/🔌️plugins/➗️mathematical/📦️packages/🦀️rust/📦️glue.rs`

**Blocks:** 1

#### Block 1 — artifact `any` (L346–L370)

**Contains:** engine, op/dsl/spr wire

```rust
        // ---- Shims: keep pre-migration module paths resolving for external callers ----
        pub mod schema {
            pub use super::standards::v1::subsets::any::schema::*;
        }
        pub mod engine {
            pub use super::standards::v1::engine::*;
        }
        pub mod io {
            pub use super::standards::v1::subsets::any::io::*;
        }
        pub mod op { pub use crate::artifacts::mathematical::standards::v1::subsets::any::schema::mutations::text::*; pub use crate::artifacts::mathematical::standards::v1::subsets::any::schema::mutations::MathematicalMutation; }
        pub mod dsl { pub use crate::artifacts::mathematical::standards::v1::subsets::any::schema::snapshot::text::*; }
        pub mod spr { pub use crate::artifacts::mathematical::standards::v1::subsets::any::schema::mutations::binary::*; }
        pub mod pack { pub use crate::artifacts::mathematical::standards::v1::subsets::any::schema::snapshot::binary::*; }
        pub mod diff { pub use crate::artifacts::mathematical::standards::v1::subsets::any::schema::diff::*; pub mod schema { pub use crate::artifacts::mathematical::standards::v1::subsets::any::schema::diff::*; } pub mod text { pub use cra // ...
        pub mod mutations { pub use crate::artifacts::mathematical::standards::v1::subsets::any::schema::mutations::*; }
        pub mod snapshot {
            pub mod schema { pub use crate::artifacts::mathematical::standards::v1::subsets::any::schema::snapshot::*; }
            pub mod pack { pub use crate::artifacts::mathematical::standards::v1::subsets::any::schema::snapshot::binary::*; }
        }
        pub use crate::artifacts::mathematical::standards::v1::subsets::any::schema::snapshot::MathematicalSnapshot;
        pub use crate::artifacts::mathematical::standards::v1::subsets::any::schema::mutations::MathematicalMutation;
        pub use crate::artifacts::mathematical::standards::v1::subsets::any::schema::diff::MathematicalDiff;
```

### `✏️s/🔌️plugins/🌀️procedural/📦️packages/🦀️rust/📦️glue.rs`

**Blocks:** 2

#### Block 1 — artifact `any` (L365–L385)

**Contains:** engine, op/dsl/spr wire

```rust
        // ---- Shims: keep pre-migration module paths resolving for external callers ----
        pub mod schema {
            pub use super::standards::v1::subsets::any::schema::*;
        }
        pub mod engine {
            pub use super::standards::v1::engine::*;
        }
        pub mod io {
            pub use super::standards::v1::subsets::any::io::*;
        }
        pub mod op { pub use crate::artifacts::procedural2d::standards::v1::subsets::any::schema::mutations::text::*; }
        pub mod dsl { pub use crate::artifacts::procedural2d::standards::v1::subsets::any::schema::snapshot::text::*; }
        pub mod spr { pub use crate::artifacts::procedural2d::standards::v1::subsets::any::schema::mutations::binary::*; }
        pub mod diff { pub use crate::artifacts::procedural2d::standards::v1::subsets::any::schema::diff::*; pub mod schema { pub use crate::artifacts::procedural2d::standards::v1::subsets::any::schema::diff::*; } pub mod text { pub use cra // ...
        pub mod mutations { pub use crate::artifacts::procedural2d::standards::v1::subsets::any::schema::mutations::*; pub mod schema { pub use crate::artifacts::procedural2d::standards::v1::subsets::any::schema::mutations::*; } pub mod tex // ...
        pub mod snapshot { pub use crate::artifacts::procedural2d::standards::v1::subsets::any::schema::snapshot::*; pub mod schema { pub use crate::artifacts::procedural2d::standards::v1::subsets::any::schema::snapshot::*; } pub mod text { // ...
        pub use crate::artifacts::procedural2d::standards::v1::subsets::any::schema::snapshot::Procedural2dSnapshot;
        pub use crate::artifacts::procedural2d::standards::v1::subsets::any::schema::mutations::Procedural2dMutation;
        pub use crate::artifacts::procedural2d::standards::v1::subsets::any::schema::diff::Procedural2dDiff;
```

#### Block 2 — artifact `any` (L782–L802)

**Contains:** engine, op/dsl/spr wire

```rust
        // ---- Shims: keep pre-migration module paths resolving for external callers ----
        pub mod schema {
            pub use super::standards::v1::subsets::any::schema::*;
        }
        pub mod engine {
            pub use super::standards::v1::engine::*;
        }
        pub mod io {
            pub use super::standards::v1::subsets::any::io::*;
        }
        pub mod op { pub use crate::artifacts::procedural3d::standards::v1::subsets::any::schema::mutations::text::*; }
        pub mod dsl { pub use crate::artifacts::procedural3d::standards::v1::subsets::any::schema::snapshot::text::*; }
        pub mod spr { pub use crate::artifacts::procedural3d::standards::v1::subsets::any::schema::mutations::binary::*; }
        pub mod diff { pub use crate::artifacts::procedural3d::standards::v1::subsets::any::schema::diff::*; pub mod schema { pub use crate::artifacts::procedural3d::standards::v1::subsets::any::schema::diff::*; } pub mod text { pub use cra // ...
        pub mod mutations { pub use crate::artifacts::procedural3d::standards::v1::subsets::any::schema::mutations::*; pub mod schema { pub use crate::artifacts::procedural3d::standards::v1::subsets::any::schema::mutations::*; } pub mod tex // ...
        pub mod snapshot { pub use crate::artifacts::procedural3d::standards::v1::subsets::any::schema::snapshot::*; pub mod schema { pub use crate::artifacts::procedural3d::standards::v1::subsets::any::schema::snapshot::*; } pub mod text { // ...
        pub use crate::artifacts::procedural3d::standards::v1::subsets::any::schema::snapshot::Procedural3dSnapshot;
        pub use crate::artifacts::procedural3d::standards::v1::subsets::any::schema::mutations::Procedural3dMutation;
        pub use crate::artifacts::procedural3d::standards::v1::subsets::any::schema::diff::Procedural3dDiff;
```

### `✏️s/🔌️plugins/🌊️flow/📦️packages/🦀️rust/📦️glue.rs`

**Blocks:** 1

#### Block 1 — artifact `any` (L304–L328)

**Contains:** engine, op/dsl/spr wire

```rust
        // ---- Shims: keep pre-migration module paths resolving for external callers ----
        pub mod schema {
            pub use super::standards::v1::subsets::any::schema::*;
        }
        pub mod engine {
            pub use super::standards::v1::engine::*;
        }
        pub mod io {
            pub use super::standards::v1::subsets::any::io::*;
        }
        pub mod op { pub use crate::artifacts::flow::standards::v1::subsets::any::schema::mutations::text::*; pub use crate::artifacts::flow::standards::v1::subsets::any::schema::mutations::FlowMutation; }
        pub mod dsl { pub use crate::artifacts::flow::standards::v1::subsets::any::schema::snapshot::text::*; }
        pub mod spr { pub use crate::artifacts::flow::standards::v1::subsets::any::schema::mutations::binary::*; }
        pub mod pack { pub use crate::artifacts::flow::standards::v1::subsets::any::schema::snapshot::binary::*; }
        pub mod diff { pub use crate::artifacts::flow::standards::v1::subsets::any::schema::diff::*; pub mod schema { pub use crate::artifacts::flow::standards::v1::subsets::any::schema::diff::*; } pub mod text { pub use crate::artifacts::f // ...
        pub mod mutations { pub use crate::artifacts::flow::standards::v1::subsets::any::schema::mutations::*; }
        pub mod snapshot {
            pub mod schema { pub use crate::artifacts::flow::standards::v1::subsets::any::schema::snapshot::*; }
            pub mod pack { pub use crate::artifacts::flow::standards::v1::subsets::any::schema::snapshot::binary::*; }
        }
        pub use crate::artifacts::flow::standards::v1::subsets::any::schema::snapshot::FlowSnapshot;
        pub use crate::artifacts::flow::standards::v1::subsets::any::schema::mutations::FlowMutation;
        pub use crate::artifacts::flow::standards::v1::subsets::any::schema::diff::FlowDiff;
```

### `✏️s/🔌️plugins/🌍️gis/📦️packages/🦀️rust/📦️glue.rs`

**Blocks:** 2

#### Block 1 — artifact `any` (L367–L387)

**Contains:** engine, op/dsl/spr wire

```rust
        // ---- Shims: keep pre-migration module paths resolving for external callers ----
        pub mod schema {
            pub use super::standards::v1::subsets::any::schema::*;
        }
        pub mod engine {
            pub use super::standards::v1::engine::*;
        }
        pub mod io {
            pub use super::standards::v1::subsets::any::io::*;
        }
        pub mod op { pub use crate::artifacts::gisterrain::standards::v1::subsets::any::schema::mutations::text::*; }
        pub mod dsl { pub use crate::artifacts::gisterrain::standards::v1::subsets::any::schema::snapshot::text::*; }
        pub mod spr { pub use crate::artifacts::gisterrain::standards::v1::subsets::any::schema::mutations::binary::*; }
        pub mod diff { pub use crate::artifacts::gisterrain::standards::v1::subsets::any::schema::diff::*; pub mod schema { pub use crate::artifacts::gisterrain::standards::v1::subsets::any::schema::diff::*; } pub mod text { pub use crate:: // ...
        pub mod mutations { pub use crate::artifacts::gisterrain::standards::v1::subsets::any::schema::mutations::*; pub mod schema { pub use crate::artifacts::gisterrain::standards::v1::subsets::any::schema::mutations::*; } pub mod text {  // ...
        pub mod snapshot { pub use crate::artifacts::gisterrain::standards::v1::subsets::any::schema::snapshot::*; pub mod schema { pub use crate::artifacts::gisterrain::standards::v1::subsets::any::schema::snapshot::*; } pub mod text { pub // ...
        pub use crate::artifacts::gisterrain::standards::v1::subsets::any::schema::snapshot::GisTerrainSnapshot;
        pub use crate::artifacts::gisterrain::standards::v1::subsets::any::schema::mutations::GisTerrainMutation;
        pub use crate::artifacts::gisterrain::standards::v1::subsets::any::schema::diff::GisTerrainDiff;
```

#### Block 2 — artifact `any` (L772–L792)

**Contains:** engine, op/dsl/spr wire

```rust
        // ---- Shims: keep pre-migration module paths resolving for external callers ----
        pub mod schema {
            pub use super::standards::v1::subsets::any::schema::*;
        }
        pub mod engine {
            pub use super::standards::v1::engine::*;
        }
        pub mod io {
            pub use super::standards::v1::subsets::any::io::*;
        }
        pub mod op { pub use crate::artifacts::gismap::standards::v1::subsets::any::schema::mutations::text::*; }
        pub mod dsl { pub use crate::artifacts::gismap::standards::v1::subsets::any::schema::snapshot::text::*; }
        pub mod spr { pub use crate::artifacts::gismap::standards::v1::subsets::any::schema::mutations::binary::*; }
        pub mod diff { pub use crate::artifacts::gismap::standards::v1::subsets::any::schema::diff::*; pub mod schema { pub use crate::artifacts::gismap::standards::v1::subsets::any::schema::diff::*; } pub mod text { pub use crate::artifact // ...
        pub mod mutations { pub use crate::artifacts::gismap::standards::v1::subsets::any::schema::mutations::*; pub mod schema { pub use crate::artifacts::gismap::standards::v1::subsets::any::schema::mutations::*; } pub mod text { pub use  // ...
        pub mod snapshot { pub use crate::artifacts::gismap::standards::v1::subsets::any::schema::snapshot::*; pub mod schema { pub use crate::artifacts::gismap::standards::v1::subsets::any::schema::snapshot::*; } pub mod text { pub use cra // ...
        pub use crate::artifacts::gismap::standards::v1::subsets::any::schema::snapshot::GisMapSnapshot;
        pub use crate::artifacts::gismap::standards::v1::subsets::any::schema::mutations::GisMapMutation;
        pub use crate::artifacts::gismap::standards::v1::subsets::any::schema::diff::GisMapDiff;
```

### `✏️s/🔌️plugins/🌿️vcs/📦️packages/🦀️rust/📦️glue.rs`

**Blocks:** 1

#### Block 1 — artifact `any` (L299–L323)

**Contains:** engine, op/dsl/spr wire

```rust
        // ---- Shims: keep pre-migration module paths resolving for external callers ----
        pub mod schema {
            pub use super::standards::v1::subsets::any::schema::*;
        }
        pub mod engine {
            pub use super::standards::v1::engine::*;
        }
        pub mod io {
            pub use super::standards::v1::subsets::any::io::*;
        }
        pub mod op { pub use crate::artifacts::vcs::standards::v1::subsets::any::schema::mutations::text::*; pub use crate::artifacts::vcs::standards::v1::subsets::any::schema::mutations::VcsDemoMutation; }
        pub mod dsl { pub use crate::artifacts::vcs::standards::v1::subsets::any::schema::snapshot::text::*; }
        pub mod spr { pub use crate::artifacts::vcs::standards::v1::subsets::any::schema::mutations::binary::*; }
        pub mod pack { pub use crate::artifacts::vcs::standards::v1::subsets::any::schema::snapshot::binary::*; }
        pub mod diff { pub use crate::artifacts::vcs::standards::v1::subsets::any::schema::diff::*; pub mod schema { pub use crate::artifacts::vcs::standards::v1::subsets::any::schema::diff::*; } pub mod text { pub use crate::artifacts::vcs // ...
        pub mod mutations { pub use crate::artifacts::vcs::standards::v1::subsets::any::schema::mutations::*; }
        pub mod snapshot {
            pub mod schema { pub use crate::artifacts::vcs::standards::v1::subsets::any::schema::snapshot::*; }
            pub mod pack { pub use crate::artifacts::vcs::standards::v1::subsets::any::schema::snapshot::binary::*; }
        }
        pub use crate::artifacts::vcs::standards::v1::subsets::any::schema::snapshot::VcsSnapshot;
        pub use crate::artifacts::vcs::standards::v1::subsets::any::schema::mutations::VcsDemoMutation;
        pub use crate::artifacts::vcs::standards::v1::subsets::any::schema::diff::VcsDiff;
```

### `✏️s/🔌️plugins/🎞️animate/📦️packages/🦀️rust/📦️glue.rs`

**Blocks:** 1

#### Block 1 — artifact `any` (L429–L446)

**Contains:** engine, op/dsl/spr wire

```rust
        // ---- Shims: keep pre-migration module paths resolving for external callers ----
        pub mod schema {
            pub use super::standards::v1::subsets::any::schema::*;
        }
        pub mod engine {
            pub use super::standards::v1::engine::*;
        }
        pub mod io {
            pub use super::standards::v1::subsets::any::io::*;
        }
        pub mod op { pub use crate::artifacts::present::standards::v1::subsets::any::schema::mutations::text::*; }
        pub mod dsl { pub use crate::artifacts::present::standards::v1::subsets::any::schema::snapshot::text::*; }
        pub mod spr { pub use crate::artifacts::present::standards::v1::subsets::any::schema::mutations::binary::*; }
        pub mod diff { pub use crate::artifacts::present::standards::v1::subsets::any::schema::diff::*; pub mod schema { pub use crate::artifacts::present::standards::v1::subsets::any::schema::diff::*; } pub mod text { pub use crate::artifa // ...
        pub mod mutations { pub use crate::artifacts::present::standards::v1::subsets::any::schema::mutations::*; pub mod schema { pub use crate::artifacts::present::standards::v1::subsets::any::schema::mutations::*; } pub mod text { pub us // ...
        pub mod snapshot { pub use crate::artifacts::present::standards::v1::subsets::any::schema::snapshot::*; pub mod schema { pub use crate::artifacts::present::standards::v1::subsets::any::schema::snapshot::*; } pub mod text { pub use c // ...
```

### `✏️s/🔌️plugins/🎥️shooting/📦️packages/🦀️rust/📦️glue.rs`

**Blocks:** 1

#### Block 1 — artifact `any` (L646–L664)

**Contains:** engine, op/dsl/spr wire

```rust
        // ---- Shims: keep pre-migration module paths resolving for external callers ----
        pub mod schema {
            pub use super::standards::v1::subsets::any::schema::*;
        }
        pub mod engine {
            pub use super::standards::v1::engine::*;
        }
        pub mod io {
            pub use super::standards::v1::subsets::any::io::*;
        }
        pub mod op { pub use crate::artifacts::shooting::standards::v1::subsets::any::schema::mutations::text::*; }
        pub mod dsl { pub use crate::artifacts::shooting::standards::v1::subsets::any::schema::snapshot::text::*; }
        pub mod spr { pub use crate::artifacts::shooting::standards::v1::subsets::any::schema::mutations::binary::*; }
        pub mod diff { pub use crate::artifacts::shooting::standards::v1::subsets::any::schema::diff::*; pub use crate::artifacts::shooting::standards::v1::subsets::any::schema::diff::text::*; pub mod schema { pub use crate::artifacts::shoo // ...
        pub mod pack { pub use crate::artifacts::shooting::standards::v1::subsets::any::schema::snapshot::binary::*; }
        pub mod mutations { pub use crate::artifacts::shooting::standards::v1::subsets::any::schema::mutations::*; }
        pub mod snapshot { pub mod schema { pub use crate::artifacts::shooting::standards::v1::subsets::any::schema::snapshot::*; } pub mod pack { pub use crate::artifacts::shooting::standards::v1::subsets::any::schema::snapshot::binary::*; // ...
```

### `✏️s/🔌️plugins/🎪️demonstrator/📦️packages/🦀️rust/📦️glue.rs`

**Blocks:** 1

#### Block 1 — artifact `any` (L252–L269)

**Contains:** engine, op/dsl/spr wire

```rust
        // ---- Shims: keep pre-migration module paths resolving for external callers ----
        pub mod schema {
            pub use super::standards::v1::subsets::any::schema::*;
        }
        pub mod engine {
            pub use super::standards::v1::engine::*;
        }
        pub mod io {
            pub use super::standards::v1::subsets::any::io::*;
        }
        pub mod op { pub use crate::artifacts::playground::standards::v1::subsets::any::schema::mutations::text::*; }
        pub mod dsl { pub use crate::artifacts::playground::standards::v1::subsets::any::schema::snapshot::text::*; }
        pub mod spr { pub use crate::artifacts::playground::standards::v1::subsets::any::schema::mutations::binary::*; }
        pub mod diff { pub use crate::artifacts::playground::standards::v1::subsets::any::schema::diff::*; pub mod schema { pub use crate::artifacts::playground::standards::v1::subsets::any::schema::diff::*; } pub mod text { pub use crate:: // ...
        pub mod mutations { pub use crate::artifacts::playground::standards::v1::subsets::any::schema::mutations::*; pub mod schema { pub use crate::artifacts::playground::standards::v1::subsets::any::schema::mutations::*; } pub mod text {  // ...
        pub mod snapshot { pub use crate::artifacts::playground::standards::v1::subsets::any::schema::snapshot::*; pub mod schema { pub use crate::artifacts::playground::standards::v1::subsets::any::schema::snapshot::*; } pub mod text { pub // ...
```

### `✏️s/🔌️plugins/🎬️sequence/📦️packages/🦀️rust/📦️glue.rs`

**Blocks:** 1

#### Block 1 — artifact `any` (L299–L316)

**Contains:** engine, op/dsl/spr wire

```rust
        // ---- Shims: keep pre-migration module paths resolving for external callers ----
        pub mod schema {
            pub use super::standards::v1::subsets::any::schema::*;
        }
        pub mod engine {
            pub use super::standards::v1::engine::*;
        }
        pub mod io {
            pub use super::standards::v1::subsets::any::io::*;
        }
        pub mod op { pub use crate::artifacts::sequence::standards::v1::subsets::any::schema::mutations::text::*; }
        pub mod dsl { pub use crate::artifacts::sequence::standards::v1::subsets::any::schema::snapshot::text::*; }
        pub mod spr { pub use crate::artifacts::sequence::standards::v1::subsets::any::schema::mutations::binary::*; }
        pub mod diff { pub use crate::artifacts::sequence::standards::v1::subsets::any::schema::diff::*; pub use crate::artifacts::sequence::standards::v1::subsets::any::schema::diff::text::*; pub mod schema { pub use crate::artifacts::sequ // ...
        pub mod mutations { pub use crate::artifacts::sequence::standards::v1::subsets::any::schema::mutations::*; }
        pub mod snapshot { pub mod schema { pub use crate::artifacts::sequence::standards::v1::subsets::any::schema::snapshot::*; } pub mod pack { pub use crate::artifacts::sequence::standards::v1::subsets::any::schema::snapshot::binary::*; // ...
```

### `✏️s/🔌️plugins/🏗️fem/📦️packages/🦀️rust/📦️glue.rs`

**Blocks:** 2

#### Block 1 — artifact `any` (L510–L530)

**Contains:** engine, op/dsl/spr wire

```rust
        // ---- Shims: keep pre-migration module paths resolving for external callers ----
        pub mod schema {
            pub use super::standards::v1::subsets::any::schema::*;
        }
        pub mod engine {
            pub use super::standards::v1::engine::*;
        }
        pub mod io {
            pub use super::standards::v1::subsets::any::io::*;
        }
        pub mod op { pub use crate::artifacts::fem2d::standards::v1::subsets::any::schema::mutations::text::*; }
        pub mod dsl { pub use crate::artifacts::fem2d::standards::v1::subsets::any::schema::snapshot::text::*; }
        pub mod spr { pub use crate::artifacts::fem2d::standards::v1::subsets::any::schema::mutations::binary::*; }
        pub mod diff { pub use crate::artifacts::fem2d::standards::v1::subsets::any::schema::diff::*; pub mod schema { pub use crate::artifacts::fem2d::standards::v1::subsets::any::schema::diff::*; } pub mod text { pub use crate::artifacts: // ...
        pub mod mutations { pub use crate::artifacts::fem2d::standards::v1::subsets::any::schema::mutations::*; pub mod schema { pub use crate::artifacts::fem2d::standards::v1::subsets::any::schema::mutations::*; } pub mod text { pub use cr // ...
        pub mod snapshot { pub use crate::artifacts::fem2d::standards::v1::subsets::any::schema::snapshot::*; pub mod schema { pub use crate::artifacts::fem2d::standards::v1::subsets::any::schema::snapshot::*; } pub mod text { pub use crate // ...
        pub use crate::artifacts::fem2d::standards::v1::subsets::any::schema::snapshot::Fem2dSnapshot;
        pub use crate::artifacts::fem2d::standards::v1::subsets::any::schema::mutations::Fem2dMutation;
        pub use crate::artifacts::fem2d::standards::v1::subsets::any::schema::diff::Fem2dDiff;
```

#### Block 2 — artifact `any` (L994–L1014)

**Contains:** engine, op/dsl/spr wire

```rust
        // ---- Shims: keep pre-migration module paths resolving for external callers ----
        pub mod schema {
            pub use super::standards::v1::subsets::any::schema::*;
        }
        pub mod engine {
            pub use super::standards::v1::engine::*;
        }
        pub mod io {
            pub use super::standards::v1::subsets::any::io::*;
        }
        pub mod op { pub use crate::artifacts::fem3d::standards::v1::subsets::any::schema::mutations::text::*; }
        pub mod dsl { pub use crate::artifacts::fem3d::standards::v1::subsets::any::schema::snapshot::text::*; }
        pub mod spr { pub use crate::artifacts::fem3d::standards::v1::subsets::any::schema::mutations::binary::*; }
        pub mod diff { pub use crate::artifacts::fem3d::standards::v1::subsets::any::schema::diff::*; pub mod schema { pub use crate::artifacts::fem3d::standards::v1::subsets::any::schema::diff::*; } pub mod text { pub use crate::artifacts: // ...
        pub mod mutations { pub use crate::artifacts::fem3d::standards::v1::subsets::any::schema::mutations::*; pub mod schema { pub use crate::artifacts::fem3d::standards::v1::subsets::any::schema::mutations::*; } pub mod text { pub use cr // ...
        pub mod snapshot { pub use crate::artifacts::fem3d::standards::v1::subsets::any::schema::snapshot::*; pub mod schema { pub use crate::artifacts::fem3d::standards::v1::subsets::any::schema::snapshot::*; } pub mod text { pub use crate // ...
        pub use crate::artifacts::fem3d::standards::v1::subsets::any::schema::snapshot::Fem3dSnapshot;
        pub use crate::artifacts::fem3d::standards::v1::subsets::any::schema::mutations::Fem3dMutation;
        pub use crate::artifacts::fem3d::standards::v1::subsets::any::schema::diff::Fem3dDiff;
```

### `✏️s/🔌️plugins/🏛️architect/📦️packages/🦀️rust/📦️glue.rs`

**Blocks:** 1

#### Block 1 — artifact `any` (L2667–L2686)

**Contains:** engine, op/dsl/spr wire

```rust
        // ---- Shims: keep pre-migration module paths resolving for external callers ----
        pub mod schema {
            pub use super::standards::v1::subsets::any::schema::*;
        }
        pub mod engine {
            pub use super::standards::v1::engine::*;
        }
        pub mod io {
            pub use super::standards::v1::subsets::any::io::*;
        }
        pub mod op { pub use crate::artifacts::program::standards::v1::subsets::any::schema::mutations::text::*; }
        pub mod dsl { pub use crate::artifacts::program::standards::v1::subsets::any::schema::snapshot::text::*; }
        pub mod spr { pub use crate::artifacts::program::standards::v1::subsets::any::schema::mutations::binary::*; }
        pub mod diff { pub use crate::artifacts::program::standards::v1::subsets::any::schema::diff::*; pub use crate::artifacts::program::standards::v1::subsets::any::schema::diff::text::*; pub mod schema { pub use crate::artifacts::progra // ...
        pub mod mutations { pub use crate::artifacts::program::standards::v1::subsets::any::schema::mutations::*; }
        pub mod snapshot { pub mod schema { pub use crate::artifacts::program::standards::v1::subsets::any::schema::snapshot::*; } pub mod pack { pub use crate::artifacts::program::standards::v1::subsets::any::schema::snapshot::binary::*; } // ...
        pub mod kernel { pub use crate::artifacts::program::standards::v1::subsets::any::schema::kernel::*; }
        pub mod registers { pub use crate::artifacts::program::standards::v1::subsets::any::schema::registers::*; }
```

### `✏️s/🔌️plugins/🏭️process/📦️packages/🦀️rust/📦️glue.rs`

**Blocks:** 1

#### Block 1 — artifact `any` (L500–L517)

**Contains:** engine, op/dsl/spr wire

```rust
        // ---- Shims: keep pre-migration module paths resolving for external callers ----
        pub mod schema {
            pub use super::standards::v1::subsets::any::schema::*;
        }
        pub mod engine {
            pub use super::standards::v1::engine::*;
        }
        pub mod io {
            pub use super::standards::v1::subsets::any::io::*;
        }
        pub mod op { pub use crate::artifacts::process3d::standards::v1::subsets::any::schema::mutations::text::*; }
        pub mod dsl { pub use crate::artifacts::process3d::standards::v1::subsets::any::schema::snapshot::text::*; }
        pub mod spr { pub use crate::artifacts::process3d::standards::v1::subsets::any::schema::mutations::binary::*; }
        pub mod diff { pub use crate::artifacts::process3d::standards::v1::subsets::any::schema::diff::*; pub mod schema { pub use crate::artifacts::process3d::standards::v1::subsets::any::schema::diff::*; } pub mod text { pub use crate::ar // ...
        pub mod mutations { pub use crate::artifacts::process3d::standards::v1::subsets::any::schema::mutations::*; pub mod schema { pub use crate::artifacts::process3d::standards::v1::subsets::any::schema::mutations::*; } pub mod text { pu // ...
        pub mod snapshot { pub use crate::artifacts::process3d::standards::v1::subsets::any::schema::snapshot::*; pub mod schema { pub use crate::artifacts::process3d::standards::v1::subsets::any::schema::snapshot::*; } pub mod text { pub u // ...
```

### `✏️s/🔌️plugins/💠️lowpoly/📦️packages/🦀️rust/📦️glue.rs`

**Blocks:** 1

#### Block 1 — artifact `any` (L493–L510)

**Contains:** engine, op/dsl/spr wire

```rust
        // ---- Shims: keep pre-migration module paths resolving for external callers ----
        pub mod schema {
            pub use super::standards::v1::subsets::any::schema::*;
        }
        pub mod engine {
            pub use super::standards::v1::engine::*;
        }
        pub mod io {
            pub use super::standards::v1::subsets::any::io::*;
        }
        pub mod op { pub use crate::artifacts::lowpoly::standards::v1::subsets::any::schema::mutations::text::*; }
        pub mod dsl { pub use crate::artifacts::lowpoly::standards::v1::subsets::any::schema::snapshot::text::*; }
        pub mod spr { pub use crate::artifacts::lowpoly::standards::v1::subsets::any::schema::mutations::binary::*; }
        pub mod diff { pub use crate::artifacts::lowpoly::standards::v1::subsets::any::schema::diff::*; pub use crate::artifacts::lowpoly::standards::v1::subsets::any::schema::diff::text::*; pub mod schema { pub use crate::artifacts::lowpol // ...
        pub mod mutations { pub use crate::artifacts::lowpoly::standards::v1::subsets::any::schema::mutations::*; }
        pub mod snapshot { pub mod schema { pub use crate::artifacts::lowpoly::standards::v1::subsets::any::schema::snapshot::*; } pub mod pack { pub use crate::artifacts::lowpoly::standards::v1::subsets::any::schema::snapshot::binary::*; } // ...
```

### `✏️s/🔌️plugins/💡️reasoning/📦️packages/🦀️rust/📦️glue.rs`

**Blocks:** 1

#### Block 1 — artifact `any` (L362–L379)

**Contains:** engine, op/dsl/spr wire

```rust
        // ---- Shims: keep pre-migration module paths resolving for external callers ----
        pub mod schema {
            pub use super::standards::v1::subsets::any::schema::*;
        }
        pub mod engine {
            pub use super::standards::v1::engine::*;
        }
        pub mod io {
            pub use super::standards::v1::subsets::any::io::*;
        }
        pub mod op { pub use crate::artifacts::wires::standards::v1::subsets::any::schema::mutations::text::*; }
        pub mod dsl { pub use crate::artifacts::wires::standards::v1::subsets::any::schema::snapshot::text::*; }
        pub mod spr { pub use crate::artifacts::wires::standards::v1::subsets::any::schema::mutations::binary::*; }
        pub mod diff { pub use crate::artifacts::wires::standards::v1::subsets::any::schema::diff::*; pub mod schema { pub use crate::artifacts::wires::standards::v1::subsets::any::schema::diff::*; } pub mod text { pub use crate::artifacts: // ...
        pub mod mutations { pub use crate::artifacts::wires::standards::v1::subsets::any::schema::mutations::*; pub mod schema { pub use crate::artifacts::wires::standards::v1::subsets::any::schema::mutations::*; } pub mod text { pub use cr // ...
        pub mod snapshot { pub use crate::artifacts::wires::standards::v1::subsets::any::schema::snapshot::*; pub mod schema { pub use crate::artifacts::wires::standards::v1::subsets::any::schema::snapshot::*; } pub mod text { pub use crate // ...
```

### `✏️s/🔌️plugins/📋️forms/📦️packages/🦀️rust/📦️glue.rs`

**Blocks:** 1

#### Block 1 — artifact `any` (L302–L319)

**Contains:** engine, op/dsl/spr wire

```rust
        // ---- Shims: keep pre-migration module paths resolving for external callers ----
        pub mod schema {
            pub use super::standards::v1::subsets::any::schema::*;
        }
        pub mod engine {
            pub use super::standards::v1::engine::*;
        }
        pub mod io {
            pub use super::standards::v1::subsets::any::io::*;
        }
        pub mod op { pub use crate::artifacts::forms::standards::v1::subsets::any::schema::mutations::text::*; pub use crate::artifacts::forms::standards::v1::subsets::any::schema::mutations::{apply_form_edit_mutation, inverse_form_mutation // ...
        pub mod dsl { pub use crate::artifacts::forms::standards::v1::subsets::any::schema::snapshot::text::*; }
        pub mod spr { pub use crate::artifacts::forms::standards::v1::subsets::any::schema::mutations::binary::*; }
        pub mod diff { pub use crate::artifacts::forms::standards::v1::subsets::any::schema::diff::*; pub mod schema { pub use crate::artifacts::forms::standards::v1::subsets::any::schema::diff::*; } pub mod text { pub use crate::artifacts: // ...
        pub mod mutations { pub use crate::artifacts::forms::standards::v1::subsets::any::schema::mutations::*; }
        pub mod snapshot { pub mod schema { pub use crate::artifacts::forms::standards::v1::subsets::any::schema::snapshot::*; } pub mod pack { pub use crate::artifacts::forms::standards::v1::subsets::any::schema::snapshot::binary::*; } }
```

### `✏️s/🔌️plugins/📏️layout/📦️packages/🦀️rust/📦️glue.rs`

**Blocks:** 1

#### Block 1 — artifact `any` (L500–L517)

**Contains:** engine, op/dsl/spr wire

```rust
        // ---- Shims: keep pre-migration module paths resolving for external callers ----
        pub mod schema {
            pub use super::standards::v1::subsets::any::schema::*;
        }
        pub mod engine {
            pub use super::standards::v1::engine::*;
        }
        pub mod io {
            pub use super::standards::v1::subsets::any::io::*;
        }
        pub mod op { pub use crate::artifacts::layout::standards::v1::subsets::any::schema::mutations::text::*; pub use crate::artifacts::layout::standards::v1::subsets::any::schema::mutations::LayoutMutation; }
        pub mod dsl { pub use crate::artifacts::layout::standards::v1::subsets::any::schema::snapshot::text::*; }
        pub mod spr { pub use crate::artifacts::layout::standards::v1::subsets::any::schema::mutations::binary::*; }
        pub mod diff { pub use crate::artifacts::layout::standards::v1::subsets::any::schema::diff::text::*; pub use crate::artifacts::layout::standards::v1::subsets::any::schema::diff::*; pub mod schema { pub use crate::artifacts::layout:: // ...
        pub mod mutations { pub use crate::artifacts::layout::standards::v1::subsets::any::schema::mutations::*; }
        pub mod snapshot { pub mod schema { pub use crate::artifacts::layout::standards::v1::subsets::any::schema::snapshot::*; } pub mod pack { pub use crate::artifacts::layout::standards::v1::subsets::any::schema::snapshot::binary::*; } }
```

### `✏️s/🔌️plugins/📐️cad/📦️packages/🦀️rust/📦️glue.rs`

**Blocks:** 1

#### Block 1 — artifact `any` (L559–L573)

**Contains:** engine

```rust
        // ---- Shims: keep pre-migration module paths resolving for external callers ----
        pub mod schema {
            pub use super::standards::v1::subsets::any::schema::*;
        }
        pub mod engine {
            pub use super::standards::v1::engine::*;
        }
        pub mod io {
            pub use super::standards::v1::subsets::any::io::*;
        }
        pub mod mutations { pub use crate::artifacts::cad::standards::v1::subsets::any::schema::mutations::*; }
        pub mod diff { pub use crate::artifacts::cad::standards::v1::subsets::any::schema::diff::*; pub mod schema { pub use crate::artifacts::cad::standards::v1::subsets::any::schema::diff::*; } pub mod text { pub use crate::artifacts::cad // ...
        pub mod snapshot { pub mod schema { pub use crate::artifacts::cad::standards::v1::subsets::any::schema::snapshot::*; } pub mod pack { pub use crate::artifacts::cad::standards::v1::subsets::any::schema::snapshot::binary::*; } }
```

### `✏️s/🔌️plugins/📕️norm/📦️packages/🦀️rust/📦️glue.rs`

**Blocks:** 15

#### Block 1 — artifact `io` (L335–L355)

**Contains:** engine, op/dsl/spr wire

```rust
        // ---- Shims: keep pre-migration module paths resolving for external callers ----
        pub mod schema {
            pub use super::standards::v1::subsets::any::schema::*;
        }
        pub mod engine {
            pub use super::standards::v1::engine::*;
        }
        pub mod io {
            pub use super::standards::v1::subsets::any::io::*;
        }
        pub mod op { pub use crate::artifacts::iso16757::standards::v1::subsets::any::schema::mutations::text::*; }
        pub mod dsl { pub use crate::artifacts::iso16757::standards::v1::subsets::any::schema::snapshot::text::*; }
        pub mod spr { pub use crate::artifacts::iso16757::standards::v1::subsets::any::schema::mutations::binary::*; }
        pub mod diff { pub use crate::artifacts::iso16757::standards::v1::subsets::any::schema::diff::*; pub mod schema { pub use crate::artifacts::iso16757::standards::v1::subsets::any::schema::diff::*; } pub mod text { pub use crate::arti // ...
        pub mod mutations { pub use crate::artifacts::iso16757::standards::v1::subsets::any::schema::mutations::*; pub mod schema { pub use crate::artifacts::iso16757::standards::v1::subsets::any::schema::mutations::*; } pub mod text { pub  // ...
        pub mod snapshot { pub use crate::artifacts::iso16757::standards::v1::subsets::any::schema::snapshot::*; pub mod schema { pub use crate::artifacts::iso16757::standards::v1::subsets::any::schema::snapshot::*; } pub mod text { pub use // ...
        pub use crate::artifacts::iso16757::standards::v1::subsets::any::schema::snapshot::Iso16757Snapshot;
        pub use crate::artifacts::iso16757::standards::v1::subsets::any::schema::mutations::Iso16757Mutation;
        pub use crate::artifacts::iso16757::standards::v1::subsets::any::schema::diff::Iso16757Diff;
```

#### Block 2 — artifact `io` (L618–L638)

**Contains:** engine, op/dsl/spr wire

```rust
        // ---- Shims: keep pre-migration module paths resolving for external callers ----
        pub mod schema {
            pub use super::standards::v1::subsets::any::schema::*;
        }
        pub mod engine {
            pub use super::standards::v1::engine::*;
        }
        pub mod io {
            pub use super::standards::v1::subsets::any::io::*;
        }
        pub mod op { pub use crate::artifacts::vdi3805::standards::v1::subsets::any::schema::mutations::text::*; }
        pub mod dsl { pub use crate::artifacts::vdi3805::standards::v1::subsets::any::schema::snapshot::text::*; }
        pub mod spr { pub use crate::artifacts::vdi3805::standards::v1::subsets::any::schema::mutations::binary::*; }
        pub mod diff { pub use crate::artifacts::vdi3805::standards::v1::subsets::any::schema::diff::*; pub mod schema { pub use crate::artifacts::vdi3805::standards::v1::subsets::any::schema::diff::*; } pub mod text { pub use crate::artifa // ...
        pub mod mutations { pub use crate::artifacts::vdi3805::standards::v1::subsets::any::schema::mutations::*; pub mod schema { pub use crate::artifacts::vdi3805::standards::v1::subsets::any::schema::mutations::*; } pub mod text { pub us // ...
        pub mod snapshot { pub use crate::artifacts::vdi3805::standards::v1::subsets::any::schema::snapshot::*; pub mod schema { pub use crate::artifacts::vdi3805::standards::v1::subsets::any::schema::snapshot::*; } pub mod text { pub use c // ...
        pub use crate::artifacts::vdi3805::standards::v1::subsets::any::schema::snapshot::Vdi3805Snapshot;
        pub use crate::artifacts::vdi3805::standards::v1::subsets::any::schema::mutations::Vdi3805Mutation;
        pub use crate::artifacts::vdi3805::standards::v1::subsets::any::schema::diff::Vdi3805Diff;
```

#### Block 3 — artifact `io` (L928–L948)

**Contains:** engine, op/dsl/spr wire

```rust
        // ---- Shims: keep pre-migration module paths resolving for external callers ----
        pub mod schema {
            pub use super::standards::v1::subsets::any::schema::*;
        }
        pub mod engine {
            pub use super::standards::v1::engine::*;
        }
        pub mod io {
            pub use super::standards::v1::subsets::any::io::*;
        }
        pub mod op { pub use crate::artifacts::din4108::standards::v1::subsets::any::schema::mutations::text::*; }
        pub mod dsl { pub use crate::artifacts::din4108::standards::v1::subsets::any::schema::snapshot::text::*; }
        pub mod spr { pub use crate::artifacts::din4108::standards::v1::subsets::any::schema::mutations::binary::*; }
        pub mod diff { pub use crate::artifacts::din4108::standards::v1::subsets::any::schema::diff::*; pub mod schema { pub use crate::artifacts::din4108::standards::v1::subsets::any::schema::diff::*; } pub mod text { pub use crate::artifa // ...
        pub mod mutations { pub use crate::artifacts::din4108::standards::v1::subsets::any::schema::mutations::*; pub mod schema { pub use crate::artifacts::din4108::standards::v1::subsets::any::schema::mutations::*; } pub mod text { pub us // ...
        pub mod snapshot { pub use crate::artifacts::din4108::standards::v1::subsets::any::schema::snapshot::*; pub mod schema { pub use crate::artifacts::din4108::standards::v1::subsets::any::schema::snapshot::*; } pub mod text { pub use c // ...
        pub use crate::artifacts::din4108::standards::v1::subsets::any::schema::snapshot::Din4108Snapshot;
        pub use crate::artifacts::din4108::standards::v1::subsets::any::schema::mutations::Din4108Mutation;
        pub use crate::artifacts::din4108::standards::v1::subsets::any::schema::diff::Din4108Diff;
```

#### Block 4 — artifact `io` (L1597–L1617)

**Contains:** engine, op/dsl/spr wire

```rust
        // ---- Shims: keep pre-migration module paths resolving for external callers ----
        pub mod schema {
            pub use super::standards::v1::subsets::any::schema::*;
        }
        pub mod engine {
            pub use super::standards::v1::engine::*;
        }
        pub mod io {
            pub use super::standards::v1::subsets::any::io::*;
        }
        pub mod op { pub use crate::artifacts::din16798::standards::v1::subsets::any::schema::mutations::text::*; }
        pub mod dsl { pub use crate::artifacts::din16798::standards::v1::subsets::any::schema::snapshot::text::*; }
        pub mod spr { pub use crate::artifacts::din16798::standards::v1::subsets::any::schema::mutations::binary::*; }
        pub mod diff { pub use crate::artifacts::din16798::standards::v1::subsets::any::schema::diff::*; pub mod schema { pub use crate::artifacts::din16798::standards::v1::subsets::any::schema::diff::*; } pub mod text { pub use crate::arti // ...
        pub mod mutations { pub use crate::artifacts::din16798::standards::v1::subsets::any::schema::mutations::*; pub mod schema { pub use crate::artifacts::din16798::standards::v1::subsets::any::schema::mutations::*; } pub mod text { pub  // ...
        pub mod snapshot { pub use crate::artifacts::din16798::standards::v1::subsets::any::schema::snapshot::*; pub mod schema { pub use crate::artifacts::din16798::standards::v1::subsets::any::schema::snapshot::*; } pub mod text { pub use // ...
        pub use crate::artifacts::din16798::standards::v1::subsets::any::schema::snapshot::Din16798Snapshot;
        pub use crate::artifacts::din16798::standards::v1::subsets::any::schema::mutations::Din16798Mutation;
        pub use crate::artifacts::din16798::standards::v1::subsets::any::schema::diff::Din16798Diff;
```

#### Block 5 — artifact `io` (L1799–L1819)

**Contains:** engine, op/dsl/spr wire

```rust
        // ---- Shims: keep pre-migration module paths resolving for external callers ----
        pub mod schema {
            pub use super::standards::v1::subsets::any::schema::*;
        }
        pub mod engine {
            pub use super::standards::v1::engine::*;
        }
        pub mod io {
            pub use super::standards::v1::subsets::any::io::*;
        }
        pub mod op { pub use crate::artifacts::en1990::standards::v1::subsets::any::schema::mutations::text::*; }
        pub mod dsl { pub use crate::artifacts::en1990::standards::v1::subsets::any::schema::snapshot::text::*; }
        pub mod spr { pub use crate::artifacts::en1990::standards::v1::subsets::any::schema::mutations::binary::*; }
        pub mod diff { pub use crate::artifacts::en1990::standards::v1::subsets::any::schema::diff::*; pub mod schema { pub use crate::artifacts::en1990::standards::v1::subsets::any::schema::diff::*; } pub mod text { pub use crate::artifact // ...
        pub mod mutations { pub use crate::artifacts::en1990::standards::v1::subsets::any::schema::mutations::*; pub mod schema { pub use crate::artifacts::en1990::standards::v1::subsets::any::schema::mutations::*; } pub mod text { pub use  // ...
        pub mod snapshot { pub use crate::artifacts::en1990::standards::v1::subsets::any::schema::snapshot::*; pub mod schema { pub use crate::artifacts::en1990::standards::v1::subsets::any::schema::snapshot::*; } pub mod text { pub use cra // ...
        pub use crate::artifacts::en1990::standards::v1::subsets::any::schema::snapshot::En1990Snapshot;
        pub use crate::artifacts::en1990::standards::v1::subsets::any::schema::mutations::En1990Mutation;
        pub use crate::artifacts::en1990::standards::v1::subsets::any::schema::diff::En1990Diff;
```

#### Block 6 — artifact `io` (L2189–L2209)

**Contains:** engine, op/dsl/spr wire

```rust
        // ---- Shims: keep pre-migration module paths resolving for external callers ----
        pub mod schema {
            pub use super::standards::v1::subsets::any::schema::*;
        }
        pub mod engine {
            pub use super::standards::v1::engine::*;
        }
        pub mod io {
            pub use super::standards::v1::subsets::any::io::*;
        }
        pub mod op { pub use crate::artifacts::en1991::standards::v1::subsets::any::schema::mutations::text::*; }
        pub mod dsl { pub use crate::artifacts::en1991::standards::v1::subsets::any::schema::snapshot::text::*; }
        pub mod spr { pub use crate::artifacts::en1991::standards::v1::subsets::any::schema::mutations::binary::*; }
        pub mod diff { pub use crate::artifacts::en1991::standards::v1::subsets::any::schema::diff::*; pub mod schema { pub use crate::artifacts::en1991::standards::v1::subsets::any::schema::diff::*; } pub mod text { pub use crate::artifact // ...
        pub mod mutations { pub use crate::artifacts::en1991::standards::v1::subsets::any::schema::mutations::*; pub mod schema { pub use crate::artifacts::en1991::standards::v1::subsets::any::schema::mutations::*; } pub mod text { pub use  // ...
        pub mod snapshot { pub use crate::artifacts::en1991::standards::v1::subsets::any::schema::snapshot::*; pub mod schema { pub use crate::artifacts::en1991::standards::v1::subsets::any::schema::snapshot::*; } pub mod text { pub use cra // ...
        pub use crate::artifacts::en1991::standards::v1::subsets::any::schema::snapshot::En1991Snapshot;
        pub use crate::artifacts::en1991::standards::v1::subsets::any::schema::mutations::En1991Mutation;
        pub use crate::artifacts::en1991::standards::v1::subsets::any::schema::diff::En1991Diff;
```

#### Block 7 — artifact `io` (L2607–L2627)

**Contains:** engine, op/dsl/spr wire

```rust
        // ---- Shims: keep pre-migration module paths resolving for external callers ----
        pub mod schema {
            pub use super::standards::v1::subsets::any::schema::*;
        }
        pub mod engine {
            pub use super::standards::v1::engine::*;
        }
        pub mod io {
            pub use super::standards::v1::subsets::any::io::*;
        }
        pub mod op { pub use crate::artifacts::en1992::standards::v1::subsets::any::schema::mutations::text::*; }
        pub mod dsl { pub use crate::artifacts::en1992::standards::v1::subsets::any::schema::snapshot::text::*; }
        pub mod spr { pub use crate::artifacts::en1992::standards::v1::subsets::any::schema::mutations::binary::*; }
        pub mod diff { pub use crate::artifacts::en1992::standards::v1::subsets::any::schema::diff::*; pub mod schema { pub use crate::artifacts::en1992::standards::v1::subsets::any::schema::diff::*; } pub mod text { pub use crate::artifact // ...
        pub mod mutations { pub use crate::artifacts::en1992::standards::v1::subsets::any::schema::mutations::*; pub mod schema { pub use crate::artifacts::en1992::standards::v1::subsets::any::schema::mutations::*; } pub mod text { pub use  // ...
        pub mod snapshot { pub use crate::artifacts::en1992::standards::v1::subsets::any::schema::snapshot::*; pub mod schema { pub use crate::artifacts::en1992::standards::v1::subsets::any::schema::snapshot::*; } pub mod text { pub use cra // ...
        pub use crate::artifacts::en1992::standards::v1::subsets::any::schema::snapshot::En1992Snapshot;
        pub use crate::artifacts::en1992::standards::v1::subsets::any::schema::mutations::En1992Mutation;
        pub use crate::artifacts::en1992::standards::v1::subsets::any::schema::diff::En1992Diff;
```

#### Block 8 — artifact `io` (L2863–L2883)

**Contains:** engine, op/dsl/spr wire

```rust
        // ---- Shims: keep pre-migration module paths resolving for external callers ----
        pub mod schema {
            pub use super::standards::v1::subsets::any::schema::*;
        }
        pub mod engine {
            pub use super::standards::v1::engine::*;
        }
        pub mod io {
            pub use super::standards::v1::subsets::any::io::*;
        }
        pub mod op { pub use crate::artifacts::en1993::standards::v1::subsets::any::schema::mutations::text::*; }
        pub mod dsl { pub use crate::artifacts::en1993::standards::v1::subsets::any::schema::snapshot::text::*; }
        pub mod spr { pub use crate::artifacts::en1993::standards::v1::subsets::any::schema::mutations::binary::*; }
        pub mod diff { pub use crate::artifacts::en1993::standards::v1::subsets::any::schema::diff::*; pub mod schema { pub use crate::artifacts::en1993::standards::v1::subsets::any::schema::diff::*; } pub mod text { pub use crate::artifact // ...
        pub mod mutations { pub use crate::artifacts::en1993::standards::v1::subsets::any::schema::mutations::*; pub mod schema { pub use crate::artifacts::en1993::standards::v1::subsets::any::schema::mutations::*; } pub mod text { pub use  // ...
        pub mod snapshot { pub use crate::artifacts::en1993::standards::v1::subsets::any::schema::snapshot::*; pub mod schema { pub use crate::artifacts::en1993::standards::v1::subsets::any::schema::snapshot::*; } pub mod text { pub use cra // ...
        pub use crate::artifacts::en1993::standards::v1::subsets::any::schema::snapshot::En1993Snapshot;
        pub use crate::artifacts::en1993::standards::v1::subsets::any::schema::mutations::En1993Mutation;
        pub use crate::artifacts::en1993::standards::v1::subsets::any::schema::diff::En1993Diff;
```

#### Block 9 — artifact `io` (L3164–L3184)

**Contains:** engine, op/dsl/spr wire

```rust
        // ---- Shims: keep pre-migration module paths resolving for external callers ----
        pub mod schema {
            pub use super::standards::v1::subsets::any::schema::*;
        }
        pub mod engine {
            pub use super::standards::v1::engine::*;
        }
        pub mod io {
            pub use super::standards::v1::subsets::any::io::*;
        }
        pub mod op { pub use crate::artifacts::en1994::standards::v1::subsets::any::schema::mutations::text::*; }
        pub mod dsl { pub use crate::artifacts::en1994::standards::v1::subsets::any::schema::snapshot::text::*; }
        pub mod spr { pub use crate::artifacts::en1994::standards::v1::subsets::any::schema::mutations::binary::*; }
        pub mod diff { pub use crate::artifacts::en1994::standards::v1::subsets::any::schema::diff::*; pub mod schema { pub use crate::artifacts::en1994::standards::v1::subsets::any::schema::diff::*; } pub mod text { pub use crate::artifact // ...
        pub mod mutations { pub use crate::artifacts::en1994::standards::v1::subsets::any::schema::mutations::*; pub mod schema { pub use crate::artifacts::en1994::standards::v1::subsets::any::schema::mutations::*; } pub mod text { pub use  // ...
        pub mod snapshot { pub use crate::artifacts::en1994::standards::v1::subsets::any::schema::snapshot::*; pub mod schema { pub use crate::artifacts::en1994::standards::v1::subsets::any::schema::snapshot::*; } pub mod text { pub use cra // ...
        pub use crate::artifacts::en1994::standards::v1::subsets::any::schema::snapshot::En1994Snapshot;
        pub use crate::artifacts::en1994::standards::v1::subsets::any::schema::mutations::En1994Mutation;
        pub use crate::artifacts::en1994::standards::v1::subsets::any::schema::diff::En1994Diff;
```

#### Block 10 — artifact `io` (L3446–L3466)

**Contains:** engine, op/dsl/spr wire

```rust
        // ---- Shims: keep pre-migration module paths resolving for external callers ----
        pub mod schema {
            pub use super::standards::v1::subsets::any::schema::*;
        }
        pub mod engine {
            pub use super::standards::v1::engine::*;
        }
        pub mod io {
            pub use super::standards::v1::subsets::any::io::*;
        }
        pub mod op { pub use crate::artifacts::en1995::standards::v1::subsets::any::schema::mutations::text::*; }
        pub mod dsl { pub use crate::artifacts::en1995::standards::v1::subsets::any::schema::snapshot::text::*; }
        pub mod spr { pub use crate::artifacts::en1995::standards::v1::subsets::any::schema::mutations::binary::*; }
        pub mod diff { pub use crate::artifacts::en1995::standards::v1::subsets::any::schema::diff::*; pub mod schema { pub use crate::artifacts::en1995::standards::v1::subsets::any::schema::diff::*; } pub mod text { pub use crate::artifact // ...
        pub mod mutations { pub use crate::artifacts::en1995::standards::v1::subsets::any::schema::mutations::*; pub mod schema { pub use crate::artifacts::en1995::standards::v1::subsets::any::schema::mutations::*; } pub mod text { pub use  // ...
        pub mod snapshot { pub use crate::artifacts::en1995::standards::v1::subsets::any::schema::snapshot::*; pub mod schema { pub use crate::artifacts::en1995::standards::v1::subsets::any::schema::snapshot::*; } pub mod text { pub use cra // ...
        pub use crate::artifacts::en1995::standards::v1::subsets::any::schema::snapshot::En1995Snapshot;
        pub use crate::artifacts::en1995::standards::v1::subsets::any::schema::mutations::En1995Mutation;
        pub use crate::artifacts::en1995::standards::v1::subsets::any::schema::diff::En1995Diff;
```

#### Block 11 — artifact `io` (L3746–L3766)

**Contains:** engine, op/dsl/spr wire

```rust
        // ---- Shims: keep pre-migration module paths resolving for external callers ----
        pub mod schema {
            pub use super::standards::v1::subsets::any::schema::*;
        }
        pub mod engine {
            pub use super::standards::v1::engine::*;
        }
        pub mod io {
            pub use super::standards::v1::subsets::any::io::*;
        }
        pub mod op { pub use crate::artifacts::en1996::standards::v1::subsets::any::schema::mutations::text::*; }
        pub mod dsl { pub use crate::artifacts::en1996::standards::v1::subsets::any::schema::snapshot::text::*; }
        pub mod spr { pub use crate::artifacts::en1996::standards::v1::subsets::any::schema::mutations::binary::*; }
        pub mod diff { pub use crate::artifacts::en1996::standards::v1::subsets::any::schema::diff::*; pub mod schema { pub use crate::artifacts::en1996::standards::v1::subsets::any::schema::diff::*; } pub mod text { pub use crate::artifact // ...
        pub mod mutations { pub use crate::artifacts::en1996::standards::v1::subsets::any::schema::mutations::*; pub mod schema { pub use crate::artifacts::en1996::standards::v1::subsets::any::schema::mutations::*; } pub mod text { pub use  // ...
        pub mod snapshot { pub use crate::artifacts::en1996::standards::v1::subsets::any::schema::snapshot::*; pub mod schema { pub use crate::artifacts::en1996::standards::v1::subsets::any::schema::snapshot::*; } pub mod text { pub use cra // ...
        pub use crate::artifacts::en1996::standards::v1::subsets::any::schema::snapshot::En1996Snapshot;
        pub use crate::artifacts::en1996::standards::v1::subsets::any::schema::mutations::En1996Mutation;
        pub use crate::artifacts::en1996::standards::v1::subsets::any::schema::diff::En1996Diff;
```

#### Block 12 — artifact `io` (L4046–L4066)

**Contains:** engine, op/dsl/spr wire

```rust
        // ---- Shims: keep pre-migration module paths resolving for external callers ----
        pub mod schema {
            pub use super::standards::v1::subsets::any::schema::*;
        }
        pub mod engine {
            pub use super::standards::v1::engine::*;
        }
        pub mod io {
            pub use super::standards::v1::subsets::any::io::*;
        }
        pub mod op { pub use crate::artifacts::en1997::standards::v1::subsets::any::schema::mutations::text::*; }
        pub mod dsl { pub use crate::artifacts::en1997::standards::v1::subsets::any::schema::snapshot::text::*; }
        pub mod spr { pub use crate::artifacts::en1997::standards::v1::subsets::any::schema::mutations::binary::*; }
        pub mod diff { pub use crate::artifacts::en1997::standards::v1::subsets::any::schema::diff::*; pub mod schema { pub use crate::artifacts::en1997::standards::v1::subsets::any::schema::diff::*; } pub mod text { pub use crate::artifact // ...
        pub mod mutations { pub use crate::artifacts::en1997::standards::v1::subsets::any::schema::mutations::*; pub mod schema { pub use crate::artifacts::en1997::standards::v1::subsets::any::schema::mutations::*; } pub mod text { pub use  // ...
        pub mod snapshot { pub use crate::artifacts::en1997::standards::v1::subsets::any::schema::snapshot::*; pub mod schema { pub use crate::artifacts::en1997::standards::v1::subsets::any::schema::snapshot::*; } pub mod text { pub use cra // ...
        pub use crate::artifacts::en1997::standards::v1::subsets::any::schema::snapshot::En1997Snapshot;
        pub use crate::artifacts::en1997::standards::v1::subsets::any::schema::mutations::En1997Mutation;
        pub use crate::artifacts::en1997::standards::v1::subsets::any::schema::diff::En1997Diff;
```

#### Block 13 — artifact `io` (L4598–L4618)

**Contains:** engine, op/dsl/spr wire

```rust
        // ---- Shims: keep pre-migration module paths resolving for external callers ----
        pub mod schema {
            pub use super::standards::v1::subsets::any::schema::*;
        }
        pub mod engine {
            pub use super::standards::v1::engine::*;
        }
        pub mod io {
            pub use super::standards::v1::subsets::any::io::*;
        }
        pub mod op { pub use crate::artifacts::en1998::standards::v1::subsets::any::schema::mutations::text::*; }
        pub mod dsl { pub use crate::artifacts::en1998::standards::v1::subsets::any::schema::snapshot::text::*; }
        pub mod spr { pub use crate::artifacts::en1998::standards::v1::subsets::any::schema::mutations::binary::*; }
        pub mod diff { pub use crate::artifacts::en1998::standards::v1::subsets::any::schema::diff::*; pub mod schema { pub use crate::artifacts::en1998::standards::v1::subsets::any::schema::diff::*; } pub mod text { pub use crate::artifact // ...
        pub mod mutations { pub use crate::artifacts::en1998::standards::v1::subsets::any::schema::mutations::*; pub mod schema { pub use crate::artifacts::en1998::standards::v1::subsets::any::schema::mutations::*; } pub mod text { pub use  // ...
        pub mod snapshot { pub use crate::artifacts::en1998::standards::v1::subsets::any::schema::snapshot::*; pub mod schema { pub use crate::artifacts::en1998::standards::v1::subsets::any::schema::snapshot::*; } pub mod text { pub use cra // ...
        pub use crate::artifacts::en1998::standards::v1::subsets::any::schema::snapshot::En1998Snapshot;
        pub use crate::artifacts::en1998::standards::v1::subsets::any::schema::mutations::En1998Mutation;
        pub use crate::artifacts::en1998::standards::v1::subsets::any::schema::diff::En1998Diff;
```

#### Block 14 — artifact `io` (L4934–L4954)

**Contains:** engine, op/dsl/spr wire

```rust
        // ---- Shims: keep pre-migration module paths resolving for external callers ----
        pub mod schema {
            pub use super::standards::v1::subsets::any::schema::*;
        }
        pub mod engine {
            pub use super::standards::v1::engine::*;
        }
        pub mod io {
            pub use super::standards::v1::subsets::any::io::*;
        }
        pub mod op { pub use crate::artifacts::en1999::standards::v1::subsets::any::schema::mutations::text::*; }
        pub mod dsl { pub use crate::artifacts::en1999::standards::v1::subsets::any::schema::snapshot::text::*; }
        pub mod spr { pub use crate::artifacts::en1999::standards::v1::subsets::any::schema::mutations::binary::*; }
        pub mod diff { pub use crate::artifacts::en1999::standards::v1::subsets::any::schema::diff::*; pub mod schema { pub use crate::artifacts::en1999::standards::v1::subsets::any::schema::diff::*; } pub mod text { pub use crate::artifact // ...
        pub mod mutations { pub use crate::artifacts::en1999::standards::v1::subsets::any::schema::mutations::*; pub mod schema { pub use crate::artifacts::en1999::standards::v1::subsets::any::schema::mutations::*; } pub mod text { pub use  // ...
        pub mod snapshot { pub use crate::artifacts::en1999::standards::v1::subsets::any::schema::snapshot::*; pub mod schema { pub use crate::artifacts::en1999::standards::v1::subsets::any::schema::snapshot::*; } pub mod text { pub use cra // ...
        pub use crate::artifacts::en1999::standards::v1::subsets::any::schema::snapshot::En1999Snapshot;
        pub use crate::artifacts::en1999::standards::v1::subsets::any::schema::mutations::En1999Mutation;
        pub use crate::artifacts::en1999::standards::v1::subsets::any::schema::diff::En1999Diff;
```

#### Block 15 — artifact `io` (L5153–L5173)

**Contains:** engine, op/dsl/spr wire

```rust
        // ---- Shims: keep pre-migration module paths resolving for external callers ----
        pub mod schema {
            pub use super::standards::v1::subsets::any::schema::*;
        }
        pub mod engine {
            pub use super::standards::v1::engine::*;
        }
        pub mod io {
            pub use super::standards::v1::subsets::any::io::*;
        }
        pub mod op { pub use crate::artifacts::din18599::standards::v1::subsets::any::schema::mutations::text::*; }
        pub mod dsl { pub use crate::artifacts::din18599::standards::v1::subsets::any::schema::snapshot::text::*; }
        pub mod spr { pub use crate::artifacts::din18599::standards::v1::subsets::any::schema::mutations::binary::*; }
        pub mod diff { pub use crate::artifacts::din18599::standards::v1::subsets::any::schema::diff::*; pub mod schema { pub use crate::artifacts::din18599::standards::v1::subsets::any::schema::diff::*; } pub mod text { pub use crate::arti // ...
        pub mod mutations { pub use crate::artifacts::din18599::standards::v1::subsets::any::schema::mutations::*; pub mod schema { pub use crate::artifacts::din18599::standards::v1::subsets::any::schema::mutations::*; } pub mod text { pub  // ...
        pub mod snapshot { pub use crate::artifacts::din18599::standards::v1::subsets::any::schema::snapshot::*; pub mod schema { pub use crate::artifacts::din18599::standards::v1::subsets::any::schema::snapshot::*; } pub mod text { pub use // ...
        pub use crate::artifacts::din18599::standards::v1::subsets::any::schema::snapshot::Din18599Snapshot;
        pub use crate::artifacts::din18599::standards::v1::subsets::any::schema::mutations::Din18599Mutation;
        pub use crate::artifacts::din18599::standards::v1::subsets::any::schema::diff::Din18599Diff;
```

### `✏️s/🔌️plugins/📖️playbook/📦️packages/🦀️rust/📦️glue.rs`

**Blocks:** 1

#### Block 1 — artifact `any` (L337–L354)

**Contains:** engine, op/dsl/spr wire

```rust
        // ---- Shims: keep pre-migration module paths resolving for external callers ----
        pub mod schema {
            pub use super::standards::v1::subsets::any::schema::*;
        }
        pub mod engine {
            pub use super::standards::v1::engine::*;
        }
        pub mod io {
            pub use super::standards::v1::subsets::any::io::*;
        }
        pub mod op { pub use crate::artifacts::playbook::standards::v1::subsets::any::schema::mutations::text::*; pub use crate::artifacts::playbook::standards::v1::subsets::any::schema::mutations::{apply_playbook_mutation, PlaybookMutation // ...
        pub mod dsl { pub use crate::artifacts::playbook::standards::v1::subsets::any::schema::snapshot::text::*; }
        pub mod spr { pub use crate::artifacts::playbook::standards::v1::subsets::any::schema::mutations::binary::*; }
        pub mod diff { pub use crate::artifacts::playbook::standards::v1::subsets::any::schema::diff::text::*; pub use crate::artifacts::playbook::standards::v1::subsets::any::schema::diff::*; pub mod schema { pub use crate::artifacts::play // ...
        pub mod mutations { pub use crate::artifacts::playbook::standards::v1::subsets::any::schema::mutations::*; }
        pub mod snapshot { pub mod schema { pub use crate::artifacts::playbook::standards::v1::subsets::any::schema::snapshot::*; } pub mod pack { pub use crate::artifacts::playbook::standards::v1::subsets::any::schema::snapshot::binary::*; // ...
```

### `✏️s/🔌️plugins/📜️imperative/📦️packages/🦀️rust/📦️glue.rs`

**Blocks:** 1

#### Block 1 — artifact `any` (L260–L277)

**Contains:** engine, op/dsl/spr wire

```rust
        // ---- Shims: keep pre-migration module paths resolving for external callers ----
        pub mod schema {
            pub use super::standards::v1::subsets::any::schema::*;
        }
        pub mod engine {
            pub use super::standards::v1::engine::*;
        }
        pub mod io {
            pub use super::standards::v1::subsets::any::io::*;
        }
        pub mod op { pub use crate::artifacts::imperative::standards::v1::subsets::any::schema::mutations::text::*; }
        pub mod dsl { pub use crate::artifacts::imperative::standards::v1::subsets::any::schema::snapshot::text::*; }
        pub mod spr { pub use crate::artifacts::imperative::standards::v1::subsets::any::schema::mutations::binary::*; }
        pub mod diff { pub use crate::artifacts::imperative::standards::v1::subsets::any::schema::diff::*; pub use crate::artifacts::imperative::standards::v1::subsets::any::schema::diff::text::*; pub mod schema { pub use crate::artifacts:: // ...
        pub mod mutations { pub use crate::artifacts::imperative::standards::v1::subsets::any::schema::mutations::*; }
        pub mod snapshot { pub mod schema { pub use crate::artifacts::imperative::standards::v1::subsets::any::schema::snapshot::*; } pub mod pack { pub use crate::artifacts::imperative::standards::v1::subsets::any::schema::snapshot::binary // ...
```

### `✏️s/🔌️plugins/📸️remodel/📦️packages/🦀️rust/📦️glue.rs`

**Blocks:** 1

#### Block 1 — artifact `any` (L675–L692)

**Contains:** engine, op/dsl/spr wire

```rust
        // ---- Shims: keep pre-migration module paths resolving for external callers ----
        pub mod schema {
            pub use super::standards::v1::subsets::any::schema::*;
        }
        pub mod engine {
            pub use super::standards::v1::engine::*;
        }
        pub mod io {
            pub use super::standards::v1::subsets::any::io::*;
        }
        pub mod op { pub use crate::artifacts::remodel::standards::v1::subsets::any::schema::mutations::text::*; }
        pub mod dsl { pub use crate::artifacts::remodel::standards::v1::subsets::any::schema::snapshot::text::*; }
        pub mod spr { pub use crate::artifacts::remodel::standards::v1::subsets::any::schema::mutations::binary::*; }
        pub mod diff { pub use crate::artifacts::remodel::standards::v1::subsets::any::schema::diff::*; pub use crate::artifacts::remodel::standards::v1::subsets::any::schema::diff::text::*; pub mod schema { pub use crate::artifacts::remode // ...
        pub mod mutations { pub use crate::artifacts::remodel::standards::v1::subsets::any::schema::mutations::*; }
        pub mod snapshot { pub mod schema { pub use crate::artifacts::remodel::standards::v1::subsets::any::schema::snapshot::*; } pub mod pack { pub use crate::artifacts::remodel::standards::v1::subsets::any::schema::snapshot::binary::*; } // ...
```

### `✏️s/🔌️plugins/🔋️energy/📦️packages/🦀️rust/📦️glue.rs`

**Blocks:** 1

#### Block 1 — artifact `any` (L417–L437)

**Contains:** engine, op/dsl/spr wire

```rust
        // ---- Shims: keep pre-migration module paths resolving for external callers ----
        pub mod schema {
            pub use super::standards::v1::subsets::any::schema::*;
        }
        pub mod engine {
            pub use super::standards::v1::engine::*;
        }
        pub mod io {
            pub use super::standards::v1::subsets::any::io::*;
        }
        pub mod op { pub use crate::artifacts::model::standards::v1::subsets::any::schema::mutations::text::*; }
        pub mod dsl { pub use crate::artifacts::model::standards::v1::subsets::any::schema::snapshot::text::*; }
        pub mod spr { pub use crate::artifacts::model::standards::v1::subsets::any::schema::mutations::binary::*; }
        pub mod diff { pub use crate::artifacts::model::standards::v1::subsets::any::schema::diff::*; pub mod schema { pub use crate::artifacts::model::standards::v1::subsets::any::schema::diff::*; } pub mod text { pub use crate::artifacts: // ...
        pub mod mutations { pub use crate::artifacts::model::standards::v1::subsets::any::schema::mutations::*; pub mod schema { pub use crate::artifacts::model::standards::v1::subsets::any::schema::mutations::*; } pub mod text { pub use cr // ...
        pub mod snapshot { pub use crate::artifacts::model::standards::v1::subsets::any::schema::snapshot::*; pub mod schema { pub use crate::artifacts::model::standards::v1::subsets::any::schema::snapshot::*; } pub mod text { pub use crate // ...
        pub use crate::artifacts::model::standards::v1::subsets::any::schema::snapshot::EnergyModelSnapshot;
        pub use crate::artifacts::model::standards::v1::subsets::any::schema::mutations::EnergyModelMutation;
        pub use crate::artifacts::model::standards::v1::subsets::any::schema::diff::EnergyModelDiff;
```

### `✏️s/🔌️plugins/🔱️trinity/📦️packages/🦀️rust/📦️glue.rs`

**Blocks:** 2

#### Block 1 — artifact `any` (L319–L339)

**Contains:** engine, op/dsl/spr wire

```rust
        // ---- Shims: keep pre-migration module paths resolving for external callers ----
        pub mod schema {
            pub use super::standards::v1::subsets::any::schema::*;
        }
        pub mod engine {
            pub use super::standards::v1::engine::*;
        }
        pub mod io {
            pub use super::standards::v1::subsets::any::io::*;
        }
        pub mod op { pub use crate::artifacts::rewrite::standards::v1::subsets::any::schema::mutations::text::*; }
        pub mod dsl { pub use crate::artifacts::rewrite::standards::v1::subsets::any::schema::snapshot::text::*; }
        pub mod spr { pub use crate::artifacts::rewrite::standards::v1::subsets::any::schema::mutations::binary::*; }
        pub mod diff { pub use crate::artifacts::rewrite::standards::v1::subsets::any::schema::diff::*; pub mod schema { pub use crate::artifacts::rewrite::standards::v1::subsets::any::schema::diff::*; } pub mod text { pub use crate::artifa // ...
        pub mod mutations { pub use crate::artifacts::rewrite::standards::v1::subsets::any::schema::mutations::*; pub mod schema { pub use crate::artifacts::rewrite::standards::v1::subsets::any::schema::mutations::*; } pub mod text { pub us // ...
        pub mod snapshot { pub use crate::artifacts::rewrite::standards::v1::subsets::any::schema::snapshot::*; pub mod schema { pub use crate::artifacts::rewrite::standards::v1::subsets::any::schema::snapshot::*; } pub mod text { pub use c // ...
        pub use crate::artifacts::rewrite::standards::v1::subsets::any::schema::snapshot::RewriteSnapshot;
        pub use crate::artifacts::rewrite::standards::v1::subsets::any::schema::mutations::RewriteRuleMutation;
        pub use crate::artifacts::rewrite::standards::v1::subsets::any::schema::diff::RewriteDiff;
```

#### Block 2 — artifact `any` (L664–L684)

**Contains:** engine, op/dsl/spr wire

```rust
        // ---- Shims: keep pre-migration module paths resolving for external callers ----
        pub mod schema {
            pub use super::standards::v1::subsets::any::schema::*;
        }
        pub mod engine {
            pub use super::standards::v1::engine::*;
        }
        pub mod io {
            pub use super::standards::v1::subsets::any::io::*;
        }
        pub mod op { pub use crate::artifacts::jack::standards::v1::subsets::any::schema::mutations::text::*; }
        pub mod dsl { pub use crate::artifacts::jack::standards::v1::subsets::any::schema::snapshot::text::*; }
        pub mod spr { pub use crate::artifacts::jack::standards::v1::subsets::any::schema::mutations::binary::*; }
        pub mod diff { pub use crate::artifacts::jack::standards::v1::subsets::any::schema::diff::*; pub mod schema { pub use crate::artifacts::jack::standards::v1::subsets::any::schema::diff::*; } pub mod text { pub use crate::artifacts::j // ...
        pub mod mutations { pub use crate::artifacts::jack::standards::v1::subsets::any::schema::mutations::*; pub mod schema { pub use crate::artifacts::jack::standards::v1::subsets::any::schema::mutations::*; } pub mod text { pub use crat // ...
        pub mod snapshot { pub use crate::artifacts::jack::standards::v1::subsets::any::schema::snapshot::*; pub mod schema { pub use crate::artifacts::jack::standards::v1::subsets::any::schema::snapshot::*; } pub mod text { pub use crate:: // ...
        pub use crate::artifacts::jack::standards::v1::subsets::any::schema::snapshot::JackSnapshot;
        pub use crate::artifacts::jack::standards::v1::subsets::any::schema::mutations::TrinityGraphMutation;
        pub use crate::artifacts::jack::standards::v1::subsets::any::schema::diff::JackDiff;
```

### `✏️s/🔌️plugins/🕸️dag/📦️packages/🦀️rust/📦️glue.rs`

**Blocks:** 1

#### Block 1 — artifact `any` (L383–L407)

**Contains:** engine, op/dsl/spr wire

```rust
        // ---- Shims: keep pre-migration module paths resolving for external callers ----
        pub mod schema {
            pub use super::standards::v1::subsets::any::schema::*;
        }
        pub mod engine {
            pub use super::standards::v1::engine::*;
        }
        pub mod io {
            pub use super::standards::v1::subsets::any::io::*;
        }
        pub mod op { pub use crate::artifacts::dag::standards::v1::subsets::any::schema::mutations::text::*; pub use crate::artifacts::dag::standards::v1::subsets::any::schema::mutations::DagMutation; }
        pub mod dsl { pub use crate::artifacts::dag::standards::v1::subsets::any::schema::snapshot::text::*; }
        pub mod spr { pub use crate::artifacts::dag::standards::v1::subsets::any::schema::mutations::binary::*; }
        pub mod pack { pub use crate::artifacts::dag::standards::v1::subsets::any::schema::snapshot::binary::*; }
        pub mod diff { pub use crate::artifacts::dag::standards::v1::subsets::any::schema::diff::*; pub mod schema { pub use crate::artifacts::dag::standards::v1::subsets::any::schema::diff::*; } pub mod text { pub use crate::artifacts::dag // ...
        pub mod mutations { pub use crate::artifacts::dag::standards::v1::subsets::any::schema::mutations::*; }
        pub mod snapshot {
            pub mod schema { pub use crate::artifacts::dag::standards::v1::subsets::any::schema::snapshot::*; }
            pub mod pack { pub use crate::artifacts::dag::standards::v1::subsets::any::schema::snapshot::binary::*; }
        }
        pub use crate::artifacts::dag::standards::v1::subsets::any::schema::snapshot::DagSnapshot;
        pub use crate::artifacts::dag::standards::v1::subsets::any::schema::mutations::DagMutation;
        pub use crate::artifacts::dag::standards::v1::subsets::any::schema::diff::DagDiff;
```

### `✏️s/🔌️plugins/🖍️draw/📦️packages/🦀️rust/📦️glue.rs`

**Blocks:** 1

#### Block 1 — artifact `any` (L407–L424)

**Contains:** engine, op/dsl/spr wire

```rust
        // ---- Shims: keep pre-migration module paths resolving for external callers ----
        pub mod schema {
            pub use super::standards::v1::subsets::any::schema::*;
        }
        pub mod engine {
            pub use super::standards::v1::engine::*;
        }
        pub mod io {
            pub use super::standards::v1::subsets::any::io::*;
        }
        pub mod op { pub use crate::artifacts::draw::standards::v1::subsets::any::schema::mutations::text::*; pub use crate::artifacts::draw::standards::v1::subsets::any::schema::mutations::{draw_op_for_layer_field, patch_layer_field, DrawM // ...
        pub mod dsl { pub use crate::artifacts::draw::standards::v1::subsets::any::schema::snapshot::text::*; }
        pub mod spr { pub use crate::artifacts::draw::standards::v1::subsets::any::schema::mutations::binary::*; }
        pub mod diff { pub use crate::artifacts::draw::standards::v1::subsets::any::schema::diff::text::*; pub use crate::artifacts::draw::standards::v1::subsets::any::schema::diff::*; pub mod schema { pub use crate::artifacts::draw::standa // ...
        pub mod mutations { pub use crate::artifacts::draw::standards::v1::subsets::any::schema::mutations::*; }
        pub mod snapshot { pub mod schema { pub use crate::artifacts::draw::standards::v1::subsets::any::schema::snapshot::*; } pub mod pack { pub use crate::artifacts::draw::standards::v1::subsets::any::schema::snapshot::binary::*; } }
```

### `✏️s/🔌️plugins/🖨️raster/📦️packages/🦀️rust/📦️glue.rs`

**Blocks:** 1

#### Block 1 — artifact `any` (L450–L467)

**Contains:** engine, op/dsl/spr wire

```rust
        // ---- Shims: keep pre-migration module paths resolving for external callers ----
        pub mod schema {
            pub use super::standards::v1::subsets::any::schema::*;
        }
        pub mod engine {
            pub use super::standards::v1::engine::*;
        }
        pub mod io {
            pub use super::standards::v1::subsets::any::io::*;
        }
        pub mod op { pub use crate::artifacts::raster::standards::v1::subsets::any::schema::mutations::text::*; pub use crate::artifacts::raster::standards::v1::subsets::any::schema::mutations::{apply_raster_mutation, RasterMutation}; }
        pub mod dsl { pub use crate::artifacts::raster::standards::v1::subsets::any::schema::snapshot::text::*; }
        pub mod spr { pub use crate::artifacts::raster::standards::v1::subsets::any::schema::mutations::binary::*; }
        pub mod diff { pub use crate::artifacts::raster::standards::v1::subsets::any::schema::diff::text::*; pub use crate::artifacts::raster::standards::v1::subsets::any::schema::diff::*; pub mod schema { pub use crate::artifacts::raster:: // ...
        pub mod mutations { pub use crate::artifacts::raster::standards::v1::subsets::any::schema::mutations::*; }
        pub mod snapshot { pub mod schema { pub use crate::artifacts::raster::standards::v1::subsets::any::schema::snapshot::*; } pub mod pack { pub use crate::artifacts::raster::standards::v1::subsets::any::schema::snapshot::binary::*; } }
```

### `✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/📦️glue.rs`

**Blocks:** 28

#### Block 1 — artifact `any` (L155–L166)

**Contains:** engine

```rust
        // ---- Shims: keep pre-migration module paths resolving for external callers ----
        pub mod schema {
            pub use super::standards::v_raw::subsets::any::schema::*;
        }
        pub mod engine {
            pub use super::standards::v_raw::engine::*;
        }
        pub mod io {
            pub use super::standards::v_raw::subsets::any::io::*;
        }
```

#### Block 2 — artifact `any` (L308–L319)

**Contains:** engine

```rust
        // ---- Shims: keep pre-migration module paths resolving for external callers ----
        pub mod schema {
            pub use super::standards::v_utf_8::subsets::any::schema::*;
        }
        pub mod engine {
            pub use super::standards::v_utf_8::engine::*;
        }
        pub mod io {
            pub use super::standards::v_utf_8::subsets::any::io::*;
        }
```

#### Block 3 — artifact `io` (L476–L487)

**Contains:** engine

```rust
        // ---- Shims: keep pre-migration module paths resolving for external callers ----
        pub mod schema {
            pub use super::standards::v_rfc8259::subsets::any::schema::*;
        }
        pub mod engine {
            pub use super::standards::v_rfc8259::engine::*;
        }
        pub mod io {
            pub use super::standards::v_rfc8259::subsets::any::io::*;
        }
```

#### Block 4 — artifact `io` (L644–L655)

**Contains:** engine

```rust
        // ---- Shims: keep pre-migration module paths resolving for external callers ----
        pub mod schema {
            pub use super::standards::v1_0::subsets::any::schema::*;
        }
        pub mod engine {
            pub use super::standards::v1_0::engine::*;
        }
        pub mod io {
            pub use super::standards::v1_0::subsets::any::io::*;
        }
```

#### Block 5 — artifact `any` (L797–L808)

**Contains:** engine

```rust
        // ---- Shims: keep pre-migration module paths resolving for external callers ----
        pub mod schema {
            pub use super::standards::v_rfc4180::subsets::any::schema::*;
        }
        pub mod engine {
            pub use super::standards::v_rfc4180::engine::*;
        }
        pub mod io {
            pub use super::standards::v_rfc4180::subsets::any::io::*;
        }
```

#### Block 6 — artifact `any` (L950–L961)

**Contains:** engine

```rust
        // ---- Shims: keep pre-migration module paths resolving for external callers ----
        pub mod schema {
            pub use super::standards::v_commonmark::subsets::any::schema::*;
        }
        pub mod engine {
            pub use super::standards::v_commonmark::engine::*;
        }
        pub mod io {
            pub use super::standards::v_commonmark::subsets::any::io::*;
        }
```

#### Block 7 — artifact `any` (L1087–L1098)

**Contains:** engine

```rust
        // ---- Shims: keep pre-migration module paths resolving for external callers ----
        pub mod schema {
            pub use super::standards::v_rfc1950::subsets::any::schema::*;
        }
        pub mod engine {
            pub use super::standards::v_rfc1950::engine::*;
        }
        pub mod io {
            pub use super::standards::v_rfc1950::subsets::any::io::*;
        }
```

#### Block 8 — artifact `io` (L1263–L1274)

**Contains:** engine

```rust
        // ---- Shims: keep pre-migration module paths resolving for external callers ----
        pub mod schema {
            pub use super::standards::v2_0::subsets::any::schema::*;
        }
        pub mod engine {
            pub use super::standards::v2_0::engine::*;
        }
        pub mod io {
            pub use super::standards::v2_0::subsets::any::io::*;
        }
```

#### Block 9 — artifact `cc6` (L1481–L1492)

**Contains:** engine

```rust
        // ---- Shims: keep pre-migration module paths resolving for external callers ----
        pub mod schema {
            pub use super::standards::v_ap214::subsets::any::schema::*;
        }
        pub mod engine {
            pub use super::standards::v_ap214::engine::*;
        }
        pub mod io {
            pub use super::standards::v_ap214::subsets::any::io::*;
        }
```

#### Block 10 — artifact `cobie` (L1790–L1809)

**Contains:** engine

```rust
        // ---- Shims: keep pre-migration module paths resolving for external callers ----
        pub mod schema {
            pub use super::standards::v4::subsets::any::schema::*;
        }
        pub mod engine {
            pub use super::standards::v4::engine::*;
            /// 📎 Registers BOTH standards' engines (v4 canonical + v2x3 new-this-ticket) -- a
            /// flat glob re-export can't do this (two `register` fns of the same name would
            /// collide), so this local definition shadows the glob-imported v4 one and calls both
            /// explicitly. Same shape as pdf's own shim fix for 1.4/1.7.
            pub fn register() {
                super::standards::v4::engine::register();
                super::standards::v2x3::engine::register();
            }
        }
        pub mod io {
            pub use super::standards::v4::subsets::any::io::*;
        }
```

#### Block 11 — artifact `any` (L1934–L1945)

**Contains:** engine

```rust
        // ---- Shims: keep pre-migration module paths resolving for external callers ----
        pub mod schema {
            pub use super::standards::v1_0::subsets::any::schema::*;
        }
        pub mod engine {
            pub use super::standards::v1_0::engine::*;
        }
        pub mod io {
            pub use super::standards::v1_0::subsets::any::io::*;
        }
```

#### Block 12 — artifact `any` (L2070–L2081)

**Contains:** engine

```rust
        // ---- Shims: keep pre-migration module paths resolving for external callers ----
        pub mod schema {
            pub use super::standards::v2_0::subsets::any::schema::*;
        }
        pub mod engine {
            pub use super::standards::v2_0::engine::*;
        }
        pub mod io {
            pub use super::standards::v2_0::subsets::any::io::*;
        }
```

#### Block 13 — artifact `any` (L2215–L2226)

**Contains:** engine

```rust
        // ---- Shims: keep pre-migration module paths resolving for external callers ----
        pub mod schema {
            pub use super::standards::v3_0::subsets::any::schema::*;
        }
        pub mod engine {
            pub use super::standards::v3_0::engine::*;
        }
        pub mod io {
            pub use super::standards::v3_0::subsets::any::io::*;
        }
```

#### Block 14 — artifact `any` (L2351–L2362)

**Contains:** engine

```rust
        // ---- Shims: keep pre-migration module paths resolving for external callers ----
        pub mod schema {
            pub use super::standards::v1_0::subsets::any::schema::*;
        }
        pub mod engine {
            pub use super::standards::v1_0::engine::*;
        }
        pub mod io {
            pub use super::standards::v1_0::subsets::any::io::*;
        }
```

#### Block 15 — artifact `any` (L2487–L2498)

**Contains:** engine

```rust
        // ---- Shims: keep pre-migration module paths resolving for external callers ----
        pub mod schema {
            pub use super::standards::v_r12::subsets::any::schema::*;
        }
        pub mod engine {
            pub use super::standards::v_r12::engine::*;
        }
        pub mod io {
            pub use super::standards::v_r12::subsets::any::io::*;
        }
```

#### Block 16 — artifact `any` (L2647–L2658)

**Contains:** engine

```rust
        // ---- Shims: keep pre-migration module paths resolving for external callers ----
        pub mod schema {
            pub use super::standards::v_ascii::subsets::any::schema::*;
        }
        pub mod engine {
            pub use super::standards::v_ascii::engine::*;
        }
        pub mod io {
            pub use super::standards::v_ascii::subsets::any::io::*;
        }
```

#### Block 17 — artifact `basic` (L2820–L2831)

**Contains:** engine

```rust
        // ---- Shims: keep pre-migration module paths resolving for external callers ----
        pub mod schema {
            pub use super::standards::v1_1::subsets::any::schema::*;
        }
        pub mod engine {
            pub use super::standards::v1_1::engine::*;
        }
        pub mod io {
            pub use super::standards::v1_1::subsets::any::io::*;
        }
```

#### Block 18 — artifact `any` (L2972–L2983)

**Contains:** engine

```rust
        // ---- Shims: keep pre-migration module paths resolving for external callers ----
        pub mod schema {
            pub use super::standards::v_v3::subsets::any::schema::*;
        }
        pub mod engine {
            pub use super::standards::v_v3::engine::*;
        }
        pub mod io {
            pub use super::standards::v_v3::subsets::any::io::*;
        }
```

#### Block 19 — artifact `any` (L3212–L3213)

```rust
        // ---- Shims: keep pre-migration module paths resolving for external callers ----
        // 🎫️26/08/10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION: default
```

#### Block 20 — artifact `any` (L3402–L3413)

**Contains:** engine

```rust
        // ---- Shims: keep pre-migration module paths resolving for external callers ----
        pub mod schema {
            pub use super::standards::v1_2::subsets::any::schema::*;
        }
        pub mod engine {
            pub use super::standards::v1_2::engine::*;
        }
        pub mod io {
            pub use super::standards::v1_2::subsets::any::io::*;
        }
```

#### Block 21 — artifact `h` (L3816–L3817)

```rust
        // ---- Shims: keep pre-migration module paths resolving for external callers ----
        // 🔀️ S-6 twin (`.claude/plans/the-current-schemas-are-scalable-journal.md`; W0 recon's
```

#### Block 22 — artifact `baseline` (L4011–L4022)

**Contains:** engine

```rust
        // ---- Shims: keep pre-migration module paths resolving for external callers ----
        pub mod schema {
            pub use super::standards::v_jfif_1_01::subsets::any::schema::*;
        }
        pub mod engine {
            pub use super::standards::v_jfif_1_01::engine::*;
        }
        pub mod io {
            pub use super::standards::v_jfif_1_01::subsets::any::io::*;
        }
```

#### Block 23 — artifact `any` (L4288–L4289)

```rust
        // ---- Shims: keep pre-migration module paths resolving for external callers ----
        // 🔀️ S-6 (`.claude/plans/the-current-schemas-are-scalable-journal.md`): 89a is the richer
```

#### Block 24 — artifact `baseline` (L4472–L4483)

**Contains:** engine

```rust
        // ---- Shims: keep pre-migration module paths resolving for external callers ----
        pub mod schema {
            pub use super::standards::v6_0::subsets::any::schema::*;
        }
        pub mod engine {
            pub use super::standards::v6_0::engine::*;
        }
        pub mod io {
            pub use super::standards::v6_0::subsets::any::io::*;
        }
```

#### Block 25 — artifact `transitional` (L4664–L4675)

**Contains:** engine

```rust
        // ---- Shims: keep pre-migration module paths resolving for external callers ----
        pub mod schema {
            pub use super::standards::v_ecma_376::subsets::any::schema::*;
        }
        pub mod engine {
            pub use super::standards::v_ecma_376::engine::*;
        }
        pub mod io {
            pub use super::standards::v_ecma_376::subsets::any::io::*;
        }
```

#### Block 26 — artifact `transitional` (L4872–L4883)

**Contains:** engine

```rust
        // ---- Shims: keep pre-migration module paths resolving for external callers ----
        pub mod schema {
            pub use super::standards::v_ecma_376::subsets::any::schema::*;
        }
        pub mod engine {
            pub use super::standards::v_ecma_376::engine::*;
        }
        pub mod io {
            pub use super::standards::v_ecma_376::subsets::any::io::*;
        }
```

#### Block 27 — artifact `transitional` (L5078–L5089)

**Contains:** engine

```rust
        // ---- Shims: keep pre-migration module paths resolving for external callers ----
        pub mod schema {
            pub use super::standards::v_ecma_376::subsets::any::schema::*;
        }
        pub mod engine {
            pub use super::standards::v_ecma_376::engine::*;
        }
        pub mod io {
            pub use super::standards::v_ecma_376::subsets::any::io::*;
        }
```

#### Block 28 — artifact `any` (L5246–L5257)

**Contains:** engine

```rust
        // ---- Shims: keep pre-migration module paths resolving for external callers ----
        pub mod schema {
            pub use super::standards::v2_1::subsets::any::schema::*;
        }
        pub mod engine {
            pub use super::standards::v2_1::engine::*;
        }
        pub mod io {
            pub use super::standards::v2_1::subsets::any::io::*;
        }
```

### `✏️s/🔌️plugins/🗒️note/📦️packages/🦀️rust/📦️glue.rs`

**Blocks:** 1

#### Block 1 — artifact `any` (L565–L582)

**Contains:** engine, op/dsl/spr wire

```rust
        // ---- Shims: keep pre-migration module paths resolving for external callers ----
        pub mod schema {
            pub use super::standards::v1::subsets::any::schema::*;
        }
        pub mod engine {
            pub use super::standards::v1::engine::*;
        }
        pub mod io {
            pub use super::standards::v1::subsets::any::io::*;
        }
        pub mod op { pub use crate::artifacts::note::standards::v1::subsets::any::schema::mutations::text::*; pub use crate::artifacts::note::standards::v1::subsets::any::schema::mutations::NoteMutation; }
        pub mod dsl { pub use crate::artifacts::note::standards::v1::subsets::any::schema::snapshot::text::*; }
        pub mod spr { pub use crate::artifacts::note::standards::v1::subsets::any::schema::mutations::binary::*; }
        pub mod diff { pub use crate::artifacts::note::standards::v1::subsets::any::schema::diff::*; pub mod schema { pub use crate::artifacts::note::standards::v1::subsets::any::schema::diff::*; } pub mod text { pub use crate::artifacts::n // ...
        pub mod mutations { pub use crate::artifacts::note::standards::v1::subsets::any::schema::mutations::*; }
        pub mod snapshot { pub mod schema { pub use crate::artifacts::note::standards::v1::subsets::any::schema::snapshot::*; } pub mod pack { pub use crate::artifacts::note::standards::v1::subsets::any::schema::snapshot::binary::*; } }
```

### `✏️s/🔌️plugins/🧩️puzzle/📦️packages/🦀️rust/📦️glue.rs`

**Blocks:** 3

#### Block 1 — artifact `any` (L530–L550)

**Contains:** engine, op/dsl/spr wire

```rust
        // ---- Shims: keep pre-migration module paths resolving for external callers ----
        pub mod schema {
            pub use super::standards::v1::subsets::any::schema::*;
        }
        pub mod engine {
            pub use super::standards::v1::engine::*;
        }
        pub mod io {
            pub use super::standards::v1::subsets::any::io::*;
        }
        pub mod op { pub use crate::artifacts::puzzle2d::standards::v1::subsets::any::schema::mutations::text::*; }
        pub mod dsl { pub use crate::artifacts::puzzle2d::standards::v1::subsets::any::schema::snapshot::text::*; }
        pub mod spr { pub use crate::artifacts::puzzle2d::standards::v1::subsets::any::schema::mutations::binary::*; }
        pub mod diff { pub use crate::artifacts::puzzle2d::standards::v1::subsets::any::schema::diff::*; pub mod schema { pub use crate::artifacts::puzzle2d::standards::v1::subsets::any::schema::diff::*; } pub mod text { pub use crate::arti // ...
        pub mod mutations { pub use crate::artifacts::puzzle2d::standards::v1::subsets::any::schema::mutations::*; pub mod schema { pub use crate::artifacts::puzzle2d::standards::v1::subsets::any::schema::mutations::*; } pub mod text { pub  // ...
        pub mod snapshot { pub use crate::artifacts::puzzle2d::standards::v1::subsets::any::schema::snapshot::*; pub mod schema { pub use crate::artifacts::puzzle2d::standards::v1::subsets::any::schema::snapshot::*; } pub mod text { pub use // ...
        pub use crate::artifacts::puzzle2d::standards::v1::subsets::any::schema::snapshot::Puzzle2dSnapshot;
        pub use crate::artifacts::puzzle2d::standards::v1::subsets::any::schema::mutations::Puzzle2dMutation;
        pub use crate::artifacts::puzzle2d::standards::v1::subsets::any::schema::diff::Puzzle2dDiff;
```

#### Block 2 — artifact `any` (L1054–L1074)

**Contains:** engine, op/dsl/spr wire

```rust
        // ---- Shims: keep pre-migration module paths resolving for external callers ----
        pub mod schema {
            pub use super::standards::v1::subsets::any::schema::*;
        }
        pub mod engine {
            pub use super::standards::v1::engine::*;
        }
        pub mod io {
            pub use super::standards::v1::subsets::any::io::*;
        }
        pub mod op { pub use crate::artifacts::puzzle5d::standards::v1::subsets::any::schema::mutations::text::*; }
        pub mod dsl { pub use crate::artifacts::puzzle5d::standards::v1::subsets::any::schema::snapshot::text::*; }
        pub mod spr { pub use crate::artifacts::puzzle5d::standards::v1::subsets::any::schema::mutations::binary::*; }
        pub mod diff { pub use crate::artifacts::puzzle5d::standards::v1::subsets::any::schema::diff::*; pub mod schema { pub use crate::artifacts::puzzle5d::standards::v1::subsets::any::schema::diff::*; } pub mod text { pub use crate::arti // ...
        pub mod mutations { pub use crate::artifacts::puzzle5d::standards::v1::subsets::any::schema::mutations::*; pub mod schema { pub use crate::artifacts::puzzle5d::standards::v1::subsets::any::schema::mutations::*; } pub mod text { pub  // ...
        pub mod snapshot { pub use crate::artifacts::puzzle5d::standards::v1::subsets::any::schema::snapshot::*; pub mod schema { pub use crate::artifacts::puzzle5d::standards::v1::subsets::any::schema::snapshot::*; } pub mod text { pub use // ...
        pub use crate::artifacts::puzzle5d::standards::v1::subsets::any::schema::snapshot::Puzzle5dSnapshot;
        pub use crate::artifacts::puzzle5d::standards::v1::subsets::any::schema::mutations::Puzzle5dMutation;
        pub use crate::artifacts::puzzle5d::standards::v1::subsets::any::schema::diff::Puzzle5dDiff;
```

#### Block 3 — artifact `any` (L1723–L1743)

**Contains:** engine, op/dsl/spr wire

```rust
        // ---- Shims: keep pre-migration module paths resolving for external callers ----
        pub mod schema {
            pub use super::standards::v1::subsets::any::schema::*;
        }
        pub mod engine {
            pub use super::standards::v1::engine::*;
        }
        pub mod io {
            pub use super::standards::v1::subsets::any::io::*;
        }
        pub mod op { pub use crate::artifacts::puzzle3d::standards::v1::subsets::any::schema::mutations::text::*; }
        pub mod dsl { pub use crate::artifacts::puzzle3d::standards::v1::subsets::any::schema::snapshot::text::*; }
        pub mod spr { pub use crate::artifacts::puzzle3d::standards::v1::subsets::any::schema::mutations::binary::*; }
        pub mod diff { pub use crate::artifacts::puzzle3d::standards::v1::subsets::any::schema::diff::*; pub mod schema { pub use crate::artifacts::puzzle3d::standards::v1::subsets::any::schema::diff::*; } pub mod text { pub use crate::arti // ...
        pub mod mutations { pub use crate::artifacts::puzzle3d::standards::v1::subsets::any::schema::mutations::*; pub mod schema { pub use crate::artifacts::puzzle3d::standards::v1::subsets::any::schema::mutations::*; } pub mod text { pub  // ...
        pub mod snapshot { pub use crate::artifacts::puzzle3d::standards::v1::subsets::any::schema::snapshot::*; pub mod schema { pub use crate::artifacts::puzzle3d::standards::v1::subsets::any::schema::snapshot::*; } pub mod text { pub use // ...
        pub use crate::artifacts::puzzle3d::standards::v1::subsets::any::schema::snapshot::Puzzle3dSnapshot;
        pub use crate::artifacts::puzzle3d::standards::v1::subsets::any::schema::mutations::Puzzle3dMutation;
        pub use crate::artifacts::puzzle3d::standards::v1::subsets::any::schema::diff::Puzzle3dDiff;
```

### `✏️s/🔌️plugins/🧱️block/📦️packages/🦀️rust/📦️glue.rs`

**Blocks:** 3

#### Block 1 — artifact `any` (L508–L528)

**Contains:** engine, op/dsl/spr wire

```rust
        // ---- Shims: keep pre-migration module paths resolving for external callers ----
        pub mod schema {
            pub use super::standards::v1::subsets::any::schema::*;
        }
        pub mod engine {
            pub use super::standards::v1::engine::*;
        }
        pub mod io {
            pub use super::standards::v1::subsets::any::io::*;
        }
        pub mod op { pub use crate::artifacts::block2d::standards::v1::subsets::any::schema::mutations::text::*; }
        pub mod dsl { pub use crate::artifacts::block2d::standards::v1::subsets::any::schema::snapshot::text::*; }
        pub mod spr { pub use crate::artifacts::block2d::standards::v1::subsets::any::schema::mutations::binary::*; }
        pub mod diff { pub use crate::artifacts::block2d::standards::v1::subsets::any::schema::diff::*; pub mod schema { pub use crate::artifacts::block2d::standards::v1::subsets::any::schema::diff::*; } pub mod text { pub use crate::artifa // ...
        pub mod mutations { pub use crate::artifacts::block2d::standards::v1::subsets::any::schema::mutations::*; pub mod schema { pub use crate::artifacts::block2d::standards::v1::subsets::any::schema::mutations::*; } pub mod text { pub us // ...
        pub mod snapshot { pub use crate::artifacts::block2d::standards::v1::subsets::any::schema::snapshot::*; pub mod schema { pub use crate::artifacts::block2d::standards::v1::subsets::any::schema::snapshot::*; } pub mod text { pub use c // ...
        pub use crate::artifacts::block2d::standards::v1::subsets::any::schema::snapshot::Block2dSnapshot;
        pub use crate::artifacts::block2d::standards::v1::subsets::any::schema::mutations::Block2dMutation;
        pub use crate::artifacts::block2d::standards::v1::subsets::any::schema::diff::Block2dDiff;
```

#### Block 2 — artifact `any` (L1141–L1161)

**Contains:** engine, op/dsl/spr wire

```rust
        // ---- Shims: keep pre-migration module paths resolving for external callers ----
        pub mod schema {
            pub use super::standards::v1::subsets::any::schema::*;
        }
        pub mod engine {
            pub use super::standards::v1::engine::*;
        }
        pub mod io {
            pub use super::standards::v1::subsets::any::io::*;
        }
        pub mod op { pub use crate::artifacts::block5d::standards::v1::subsets::any::schema::mutations::text::*; }
        pub mod dsl { pub use crate::artifacts::block5d::standards::v1::subsets::any::schema::snapshot::text::*; }
        pub mod spr { pub use crate::artifacts::block5d::standards::v1::subsets::any::schema::mutations::binary::*; }
        pub mod diff { pub use crate::artifacts::block5d::standards::v1::subsets::any::schema::diff::*; pub mod schema { pub use crate::artifacts::block5d::standards::v1::subsets::any::schema::diff::*; } pub mod text { pub use crate::artifa // ...
        pub mod mutations { pub use crate::artifacts::block5d::standards::v1::subsets::any::schema::mutations::*; pub mod schema { pub use crate::artifacts::block5d::standards::v1::subsets::any::schema::mutations::*; } pub mod text { pub us // ...
        pub mod snapshot { pub use crate::artifacts::block5d::standards::v1::subsets::any::schema::snapshot::*; pub mod schema { pub use crate::artifacts::block5d::standards::v1::subsets::any::schema::snapshot::*; } pub mod text { pub use c // ...
        pub use crate::artifacts::block5d::standards::v1::subsets::any::schema::snapshot::Block5dSnapshot;
        pub use crate::artifacts::block5d::standards::v1::subsets::any::schema::mutations::Block5dMutation;
        pub use crate::artifacts::block5d::standards::v1::subsets::any::schema::diff::Block5dDiff;
```

#### Block 3 — artifact `any` (L1738–L1758)

**Contains:** engine, op/dsl/spr wire

```rust
        // ---- Shims: keep pre-migration module paths resolving for external callers ----
        pub mod schema {
            pub use super::standards::v1::subsets::any::schema::*;
        }
        pub mod engine {
            pub use super::standards::v1::engine::*;
        }
        pub mod io {
            pub use super::standards::v1::subsets::any::io::*;
        }
        pub mod op { pub use crate::artifacts::block3d::standards::v1::subsets::any::schema::mutations::text::*; }
        pub mod dsl { pub use crate::artifacts::block3d::standards::v1::subsets::any::schema::snapshot::text::*; }
        pub mod spr { pub use crate::artifacts::block3d::standards::v1::subsets::any::schema::mutations::binary::*; }
        pub mod diff { pub use crate::artifacts::block3d::standards::v1::subsets::any::schema::diff::*; pub mod schema { pub use crate::artifacts::block3d::standards::v1::subsets::any::schema::diff::*; } pub mod text { pub use crate::artifa // ...
        pub mod mutations { pub use crate::artifacts::block3d::standards::v1::subsets::any::schema::mutations::*; pub mod schema { pub use crate::artifacts::block3d::standards::v1::subsets::any::schema::mutations::*; } pub mod text { pub us // ...
        pub mod snapshot { pub use crate::artifacts::block3d::standards::v1::subsets::any::schema::snapshot::*; pub mod schema { pub use crate::artifacts::block3d::standards::v1::subsets::any::schema::snapshot::*; } pub mod text { pub use c // ...
        pub use crate::artifacts::block3d::standards::v1::subsets::any::schema::snapshot::Block3dSnapshot;
        pub use crate::artifacts::block3d::standards::v1::subsets::any::schema::mutations::Block3dMutation;
        pub use crate::artifacts::block3d::standards::v1::subsets::any::schema::diff::Block3dDiff;
```

### `✏️s/🔌️plugins/🪐️space/📦️packages/🦀️rust/📦️glue.rs`

**Blocks:** 1

#### Block 1 — artifact `any` (L266–L283)

**Contains:** engine, op/dsl/spr wire

```rust
        // ---- Shims: keep pre-migration module paths resolving for external callers ----
        pub mod schema {
            pub use super::standards::v1::subsets::any::schema::*;
        }
        pub mod engine {
            pub use super::standards::v1::engine::*;
        }
        pub mod io {
            pub use super::standards::v1::subsets::any::io::*;
        }
        pub mod op { pub use crate::artifacts::home::standards::v1::subsets::any::schema::mutations::text::*; }
        pub mod dsl { pub use crate::artifacts::home::standards::v1::subsets::any::schema::snapshot::text::*; }
        pub mod spr { pub use crate::artifacts::home::standards::v1::subsets::any::schema::mutations::binary::*; }
        pub mod diff { pub use crate::artifacts::home::standards::v1::subsets::any::schema::diff::*; pub mod schema { pub use crate::artifacts::home::standards::v1::subsets::any::schema::diff::*; } pub mod text { pub use crate::artifacts::h // ...
        pub mod mutations { pub use crate::artifacts::home::standards::v1::subsets::any::schema::mutations::*; pub mod schema { pub use crate::artifacts::home::standards::v1::subsets::any::schema::mutations::*; } pub mod text { pub use crat // ...
        pub mod snapshot { pub use crate::artifacts::home::standards::v1::subsets::any::schema::snapshot::*; pub mod schema { pub use crate::artifacts::home::standards::v1::subsets::any::schema::snapshot::*; } pub mod text { pub use crate:: // ...
```

### `✏️s/🔌️plugins/🪵️sourcing/📦️packages/🦀️rust/📦️glue.rs`

**Blocks:** 1

#### Block 1 — artifact `any` (L302–L319)

**Contains:** engine, op/dsl/spr wire

```rust
        // ---- Shims: keep pre-migration module paths resolving for external callers ----
        pub mod schema {
            pub use super::standards::v1::subsets::any::schema::*;
        }
        pub mod engine {
            pub use super::standards::v1::engine::*;
        }
        pub mod io {
            pub use super::standards::v1::subsets::any::io::*;
        }
        pub mod op { pub use crate::artifacts::curate::standards::v1::subsets::any::schema::mutations::text::*; }
        pub mod dsl { pub use crate::artifacts::curate::standards::v1::subsets::any::schema::snapshot::text::*; }
        pub mod spr { pub use crate::artifacts::curate::standards::v1::subsets::any::schema::mutations::binary::*; }
        pub mod diff { pub use crate::artifacts::curate::standards::v1::subsets::any::schema::diff::*; pub mod schema { pub use crate::artifacts::curate::standards::v1::subsets::any::schema::diff::*; } pub mod text { pub use crate::artifact // ...
        pub mod mutations { pub use crate::artifacts::curate::standards::v1::subsets::any::schema::mutations::*; pub mod schema { pub use crate::artifacts::curate::standards::v1::subsets::any::schema::mutations::*; } pub mod text { pub use  // ...
        pub mod snapshot { pub use crate::artifacts::curate::standards::v1::subsets::any::schema::snapshot::*; pub mod schema { pub use crate::artifacts::curate::standards::v1::subsets::any::schema::snapshot::*; } pub mod text { pub use cra // ...
```

## Additional Shim-Like Blocks (No Comment Marker)

| File | Lines | Description |
|------|-------|-------------|
| `✏️s/🔌️plugins/🔋️energy/📦️packages/🦀️rust/📦️glue.rs` | L31–~130 | Crate-root flat `pub mod <domain>` with `#[path]` to engine subdirs (50 modules) |
| `✏️s/🔌️plugins/🔋️energy/📦️packages/🦀️rust/📦️glue.rs` | L14–19 | Doc comment preserving pre-migration flat re-export surface |
| `✏️s/🔌️plugins/🎞️animate/📦️packages/🦀️rust/📦️glue.rs` | L42–57 | Direct `#[path]` engine subdir declarations |
| `✏️s/🔌️plugins/🎞️animate/📦️packages/🦀️rust/📦️glue.rs` | L58–83 | `pub mod animate { ... }` duplicate alias tree |
| `✏️s/🔌️plugins/🪐️space/📦️packages/🦀️rust/📦️glue.rs` | L13 | Doc comment referencing pre-migration bundle crate |

## Trailing Type Re-exports After Shim Blocks

| File | Line | Snippet |
|------|-----:|---------|
| `✏️s/🔌️plugins/✒️writer/📦️packages/🦀️rust/📦️glue.rs` | 292 | `pub mod op { pub use crate::artifacts::writer::standards::v1::subsets::any::schema::mutations::text::*; pub use crate::artifacts::writer::standards::v1::subsets::any::schema::mutations::WriterMutation; }` |
| `✏️s/🔌️plugins/✒️writer/📦️packages/🦀️rust/📦️glue.rs` | 302 | `pub use crate::artifacts::writer::standards::v1::subsets::any::schema::snapshot::WriterSnapshot;` |
| `✏️s/🔌️plugins/✒️writer/📦️packages/🦀️rust/📦️glue.rs` | 303 | `pub use crate::artifacts::writer::standards::v1::subsets::any::schema::mutations::WriterMutation;` |
| `✏️s/🔌️plugins/✒️writer/📦️packages/🦀️rust/📦️glue.rs` | 304 | `pub use crate::artifacts::writer::standards::v1::subsets::any::schema::diff::WriterDiff;` |
| `✏️s/🔌️plugins/➗️mathematical/📦️packages/🦀️rust/📦️glue.rs` | 356 | `pub mod op { pub use crate::artifacts::mathematical::standards::v1::subsets::any::schema::mutations::text::*; pub use crate::artifacts::mathematical::standards::v1::subsets::any::schema::mutations::MathematicalMutation; }` |
| `✏️s/🔌️plugins/➗️mathematical/📦️packages/🦀️rust/📦️glue.rs` | 366 | `pub use crate::artifacts::mathematical::standards::v1::subsets::any::schema::snapshot::MathematicalSnapshot;` |
| `✏️s/🔌️plugins/➗️mathematical/📦️packages/🦀️rust/📦️glue.rs` | 367 | `pub use crate::artifacts::mathematical::standards::v1::subsets::any::schema::mutations::MathematicalMutation;` |
| `✏️s/🔌️plugins/➗️mathematical/📦️packages/🦀️rust/📦️glue.rs` | 368 | `pub use crate::artifacts::mathematical::standards::v1::subsets::any::schema::diff::MathematicalDiff;` |
| `✏️s/🔌️plugins/🌀️procedural/📦️packages/🦀️rust/📦️glue.rs` | 381 | `pub use crate::artifacts::procedural2d::standards::v1::subsets::any::schema::snapshot::Procedural2dSnapshot;` |
| `✏️s/🔌️plugins/🌀️procedural/📦️packages/🦀️rust/📦️glue.rs` | 382 | `pub use crate::artifacts::procedural2d::standards::v1::subsets::any::schema::mutations::Procedural2dMutation;` |
| `✏️s/🔌️plugins/🌀️procedural/📦️packages/🦀️rust/📦️glue.rs` | 383 | `pub use crate::artifacts::procedural2d::standards::v1::subsets::any::schema::diff::Procedural2dDiff;` |
| `✏️s/🔌️plugins/🌀️procedural/📦️packages/🦀️rust/📦️glue.rs` | 798 | `pub use crate::artifacts::procedural3d::standards::v1::subsets::any::schema::snapshot::Procedural3dSnapshot;` |
| `✏️s/🔌️plugins/🌀️procedural/📦️packages/🦀️rust/📦️glue.rs` | 799 | `pub use crate::artifacts::procedural3d::standards::v1::subsets::any::schema::mutations::Procedural3dMutation;` |
| `✏️s/🔌️plugins/🌀️procedural/📦️packages/🦀️rust/📦️glue.rs` | 800 | `pub use crate::artifacts::procedural3d::standards::v1::subsets::any::schema::diff::Procedural3dDiff;` |
| `✏️s/🔌️plugins/🌊️flow/📦️packages/🦀️rust/📦️glue.rs` | 314 | `pub mod op { pub use crate::artifacts::flow::standards::v1::subsets::any::schema::mutations::text::*; pub use crate::artifacts::flow::standards::v1::subsets::any::schema::mutations::FlowMutation; }` |
| `✏️s/🔌️plugins/🌊️flow/📦️packages/🦀️rust/📦️glue.rs` | 324 | `pub use crate::artifacts::flow::standards::v1::subsets::any::schema::snapshot::FlowSnapshot;` |
| `✏️s/🔌️plugins/🌊️flow/📦️packages/🦀️rust/📦️glue.rs` | 325 | `pub use crate::artifacts::flow::standards::v1::subsets::any::schema::mutations::FlowMutation;` |
| `✏️s/🔌️plugins/🌊️flow/📦️packages/🦀️rust/📦️glue.rs` | 326 | `pub use crate::artifacts::flow::standards::v1::subsets::any::schema::diff::FlowDiff;` |
| `✏️s/🔌️plugins/🌍️gis/📦️packages/🦀️rust/📦️glue.rs` | 383 | `pub use crate::artifacts::gisterrain::standards::v1::subsets::any::schema::snapshot::GisTerrainSnapshot;` |
| `✏️s/🔌️plugins/🌍️gis/📦️packages/🦀️rust/📦️glue.rs` | 384 | `pub use crate::artifacts::gisterrain::standards::v1::subsets::any::schema::mutations::GisTerrainMutation;` |
| `✏️s/🔌️plugins/🌍️gis/📦️packages/🦀️rust/📦️glue.rs` | 385 | `pub use crate::artifacts::gisterrain::standards::v1::subsets::any::schema::diff::GisTerrainDiff;` |
| `✏️s/🔌️plugins/🌍️gis/📦️packages/🦀️rust/📦️glue.rs` | 788 | `pub use crate::artifacts::gismap::standards::v1::subsets::any::schema::snapshot::GisMapSnapshot;` |
| `✏️s/🔌️plugins/🌍️gis/📦️packages/🦀️rust/📦️glue.rs` | 789 | `pub use crate::artifacts::gismap::standards::v1::subsets::any::schema::mutations::GisMapMutation;` |
| `✏️s/🔌️plugins/🌍️gis/📦️packages/🦀️rust/📦️glue.rs` | 790 | `pub use crate::artifacts::gismap::standards::v1::subsets::any::schema::diff::GisMapDiff;` |
| `✏️s/🔌️plugins/🌿️vcs/📦️packages/🦀️rust/📦️glue.rs` | 309 | `pub mod op { pub use crate::artifacts::vcs::standards::v1::subsets::any::schema::mutations::text::*; pub use crate::artifacts::vcs::standards::v1::subsets::any::schema::mutations::VcsDemoMutation; }` |
| `✏️s/🔌️plugins/🌿️vcs/📦️packages/🦀️rust/📦️glue.rs` | 319 | `pub use crate::artifacts::vcs::standards::v1::subsets::any::schema::snapshot::VcsSnapshot;` |
| `✏️s/🔌️plugins/🌿️vcs/📦️packages/🦀️rust/📦️glue.rs` | 320 | `pub use crate::artifacts::vcs::standards::v1::subsets::any::schema::mutations::VcsDemoMutation;` |
| `✏️s/🔌️plugins/🌿️vcs/📦️packages/🦀️rust/📦️glue.rs` | 321 | `pub use crate::artifacts::vcs::standards::v1::subsets::any::schema::diff::VcsDiff;` |
| `✏️s/🔌️plugins/🏗️fem/📦️packages/🦀️rust/📦️glue.rs` | 526 | `pub use crate::artifacts::fem2d::standards::v1::subsets::any::schema::snapshot::Fem2dSnapshot;` |
| `✏️s/🔌️plugins/🏗️fem/📦️packages/🦀️rust/📦️glue.rs` | 527 | `pub use crate::artifacts::fem2d::standards::v1::subsets::any::schema::mutations::Fem2dMutation;` |
| `✏️s/🔌️plugins/🏗️fem/📦️packages/🦀️rust/📦️glue.rs` | 528 | `pub use crate::artifacts::fem2d::standards::v1::subsets::any::schema::diff::Fem2dDiff;` |
| `✏️s/🔌️plugins/🏗️fem/📦️packages/🦀️rust/📦️glue.rs` | 1010 | `pub use crate::artifacts::fem3d::standards::v1::subsets::any::schema::snapshot::Fem3dSnapshot;` |
| `✏️s/🔌️plugins/🏗️fem/📦️packages/🦀️rust/📦️glue.rs` | 1011 | `pub use crate::artifacts::fem3d::standards::v1::subsets::any::schema::mutations::Fem3dMutation;` |
| `✏️s/🔌️plugins/🏗️fem/📦️packages/🦀️rust/📦️glue.rs` | 1012 | `pub use crate::artifacts::fem3d::standards::v1::subsets::any::schema::diff::Fem3dDiff;` |
| `✏️s/🔌️plugins/📋️forms/📦️packages/🦀️rust/📦️glue.rs` | 312 | `pub mod op { pub use crate::artifacts::forms::standards::v1::subsets::any::schema::mutations::text::*; pub use crate::artifacts::forms::standards::v1::subsets::any::schema::mutations::{apply_form_edit_mutation, inverse_form_mutation, FormMutation}; }` |
| `✏️s/🔌️plugins/📏️layout/📦️packages/🦀️rust/📦️glue.rs` | 510 | `pub mod op { pub use crate::artifacts::layout::standards::v1::subsets::any::schema::mutations::text::*; pub use crate::artifacts::layout::standards::v1::subsets::any::schema::mutations::LayoutMutation; }` |
| `✏️s/🔌️plugins/📕️norm/📦️packages/🦀️rust/📦️glue.rs` | 351 | `pub use crate::artifacts::iso16757::standards::v1::subsets::any::schema::snapshot::Iso16757Snapshot;` |
| `✏️s/🔌️plugins/📕️norm/📦️packages/🦀️rust/📦️glue.rs` | 352 | `pub use crate::artifacts::iso16757::standards::v1::subsets::any::schema::mutations::Iso16757Mutation;` |
| `✏️s/🔌️plugins/📕️norm/📦️packages/🦀️rust/📦️glue.rs` | 353 | `pub use crate::artifacts::iso16757::standards::v1::subsets::any::schema::diff::Iso16757Diff;` |
| `✏️s/🔌️plugins/📕️norm/📦️packages/🦀️rust/📦️glue.rs` | 634 | `pub use crate::artifacts::vdi3805::standards::v1::subsets::any::schema::snapshot::Vdi3805Snapshot;` |
| `✏️s/🔌️plugins/📕️norm/📦️packages/🦀️rust/📦️glue.rs` | 635 | `pub use crate::artifacts::vdi3805::standards::v1::subsets::any::schema::mutations::Vdi3805Mutation;` |
| `✏️s/🔌️plugins/📕️norm/📦️packages/🦀️rust/📦️glue.rs` | 636 | `pub use crate::artifacts::vdi3805::standards::v1::subsets::any::schema::diff::Vdi3805Diff;` |
| `✏️s/🔌️plugins/📕️norm/📦️packages/🦀️rust/📦️glue.rs` | 944 | `pub use crate::artifacts::din4108::standards::v1::subsets::any::schema::snapshot::Din4108Snapshot;` |
| `✏️s/🔌️plugins/📕️norm/📦️packages/🦀️rust/📦️glue.rs` | 945 | `pub use crate::artifacts::din4108::standards::v1::subsets::any::schema::mutations::Din4108Mutation;` |
| `✏️s/🔌️plugins/📕️norm/📦️packages/🦀️rust/📦️glue.rs` | 946 | `pub use crate::artifacts::din4108::standards::v1::subsets::any::schema::diff::Din4108Diff;` |
| `✏️s/🔌️plugins/📕️norm/📦️packages/🦀️rust/📦️glue.rs` | 1613 | `pub use crate::artifacts::din16798::standards::v1::subsets::any::schema::snapshot::Din16798Snapshot;` |
| `✏️s/🔌️plugins/📕️norm/📦️packages/🦀️rust/📦️glue.rs` | 1614 | `pub use crate::artifacts::din16798::standards::v1::subsets::any::schema::mutations::Din16798Mutation;` |
| `✏️s/🔌️plugins/📕️norm/📦️packages/🦀️rust/📦️glue.rs` | 1615 | `pub use crate::artifacts::din16798::standards::v1::subsets::any::schema::diff::Din16798Diff;` |
| `✏️s/🔌️plugins/📕️norm/📦️packages/🦀️rust/📦️glue.rs` | 1815 | `pub use crate::artifacts::en1990::standards::v1::subsets::any::schema::snapshot::En1990Snapshot;` |
| `✏️s/🔌️plugins/📕️norm/📦️packages/🦀️rust/📦️glue.rs` | 1816 | `pub use crate::artifacts::en1990::standards::v1::subsets::any::schema::mutations::En1990Mutation;` |
| `✏️s/🔌️plugins/📕️norm/📦️packages/🦀️rust/📦️glue.rs` | 1817 | `pub use crate::artifacts::en1990::standards::v1::subsets::any::schema::diff::En1990Diff;` |
| `✏️s/🔌️plugins/📕️norm/📦️packages/🦀️rust/📦️glue.rs` | 2205 | `pub use crate::artifacts::en1991::standards::v1::subsets::any::schema::snapshot::En1991Snapshot;` |
| `✏️s/🔌️plugins/📕️norm/📦️packages/🦀️rust/📦️glue.rs` | 2206 | `pub use crate::artifacts::en1991::standards::v1::subsets::any::schema::mutations::En1991Mutation;` |
| `✏️s/🔌️plugins/📕️norm/📦️packages/🦀️rust/📦️glue.rs` | 2207 | `pub use crate::artifacts::en1991::standards::v1::subsets::any::schema::diff::En1991Diff;` |
| `✏️s/🔌️plugins/📕️norm/📦️packages/🦀️rust/📦️glue.rs` | 2623 | `pub use crate::artifacts::en1992::standards::v1::subsets::any::schema::snapshot::En1992Snapshot;` |
| `✏️s/🔌️plugins/📕️norm/📦️packages/🦀️rust/📦️glue.rs` | 2624 | `pub use crate::artifacts::en1992::standards::v1::subsets::any::schema::mutations::En1992Mutation;` |
| `✏️s/🔌️plugins/📕️norm/📦️packages/🦀️rust/📦️glue.rs` | 2625 | `pub use crate::artifacts::en1992::standards::v1::subsets::any::schema::diff::En1992Diff;` |
| `✏️s/🔌️plugins/📕️norm/📦️packages/🦀️rust/📦️glue.rs` | 2879 | `pub use crate::artifacts::en1993::standards::v1::subsets::any::schema::snapshot::En1993Snapshot;` |
| `✏️s/🔌️plugins/📕️norm/📦️packages/🦀️rust/📦️glue.rs` | 2880 | `pub use crate::artifacts::en1993::standards::v1::subsets::any::schema::mutations::En1993Mutation;` |
| `✏️s/🔌️plugins/📕️norm/📦️packages/🦀️rust/📦️glue.rs` | 2881 | `pub use crate::artifacts::en1993::standards::v1::subsets::any::schema::diff::En1993Diff;` |
| `✏️s/🔌️plugins/📕️norm/📦️packages/🦀️rust/📦️glue.rs` | 3180 | `pub use crate::artifacts::en1994::standards::v1::subsets::any::schema::snapshot::En1994Snapshot;` |
| `✏️s/🔌️plugins/📕️norm/📦️packages/🦀️rust/📦️glue.rs` | 3181 | `pub use crate::artifacts::en1994::standards::v1::subsets::any::schema::mutations::En1994Mutation;` |
| `✏️s/🔌️plugins/📕️norm/📦️packages/🦀️rust/📦️glue.rs` | 3182 | `pub use crate::artifacts::en1994::standards::v1::subsets::any::schema::diff::En1994Diff;` |
| `✏️s/🔌️plugins/📕️norm/📦️packages/🦀️rust/📦️glue.rs` | 3462 | `pub use crate::artifacts::en1995::standards::v1::subsets::any::schema::snapshot::En1995Snapshot;` |
| `✏️s/🔌️plugins/📕️norm/📦️packages/🦀️rust/📦️glue.rs` | 3463 | `pub use crate::artifacts::en1995::standards::v1::subsets::any::schema::mutations::En1995Mutation;` |
| `✏️s/🔌️plugins/📕️norm/📦️packages/🦀️rust/📦️glue.rs` | 3464 | `pub use crate::artifacts::en1995::standards::v1::subsets::any::schema::diff::En1995Diff;` |
| `✏️s/🔌️plugins/📕️norm/📦️packages/🦀️rust/📦️glue.rs` | 3762 | `pub use crate::artifacts::en1996::standards::v1::subsets::any::schema::snapshot::En1996Snapshot;` |
| `✏️s/🔌️plugins/📕️norm/📦️packages/🦀️rust/📦️glue.rs` | 3763 | `pub use crate::artifacts::en1996::standards::v1::subsets::any::schema::mutations::En1996Mutation;` |
| `✏️s/🔌️plugins/📕️norm/📦️packages/🦀️rust/📦️glue.rs` | 3764 | `pub use crate::artifacts::en1996::standards::v1::subsets::any::schema::diff::En1996Diff;` |
| `✏️s/🔌️plugins/📕️norm/📦️packages/🦀️rust/📦️glue.rs` | 4062 | `pub use crate::artifacts::en1997::standards::v1::subsets::any::schema::snapshot::En1997Snapshot;` |
| `✏️s/🔌️plugins/📕️norm/📦️packages/🦀️rust/📦️glue.rs` | 4063 | `pub use crate::artifacts::en1997::standards::v1::subsets::any::schema::mutations::En1997Mutation;` |
| `✏️s/🔌️plugins/📕️norm/📦️packages/🦀️rust/📦️glue.rs` | 4064 | `pub use crate::artifacts::en1997::standards::v1::subsets::any::schema::diff::En1997Diff;` |
| `✏️s/🔌️plugins/📕️norm/📦️packages/🦀️rust/📦️glue.rs` | 4614 | `pub use crate::artifacts::en1998::standards::v1::subsets::any::schema::snapshot::En1998Snapshot;` |
| `✏️s/🔌️plugins/📕️norm/📦️packages/🦀️rust/📦️glue.rs` | 4615 | `pub use crate::artifacts::en1998::standards::v1::subsets::any::schema::mutations::En1998Mutation;` |
| `✏️s/🔌️plugins/📕️norm/📦️packages/🦀️rust/📦️glue.rs` | 4616 | `pub use crate::artifacts::en1998::standards::v1::subsets::any::schema::diff::En1998Diff;` |
| `✏️s/🔌️plugins/📕️norm/📦️packages/🦀️rust/📦️glue.rs` | 4950 | `pub use crate::artifacts::en1999::standards::v1::subsets::any::schema::snapshot::En1999Snapshot;` |
| `✏️s/🔌️plugins/📕️norm/📦️packages/🦀️rust/📦️glue.rs` | 4951 | `pub use crate::artifacts::en1999::standards::v1::subsets::any::schema::mutations::En1999Mutation;` |
| `✏️s/🔌️plugins/📕️norm/📦️packages/🦀️rust/📦️glue.rs` | 4952 | `pub use crate::artifacts::en1999::standards::v1::subsets::any::schema::diff::En1999Diff;` |
| `✏️s/🔌️plugins/📕️norm/📦️packages/🦀️rust/📦️glue.rs` | 5169 | `pub use crate::artifacts::din18599::standards::v1::subsets::any::schema::snapshot::Din18599Snapshot;` |
| `✏️s/🔌️plugins/📕️norm/📦️packages/🦀️rust/📦️glue.rs` | 5170 | `pub use crate::artifacts::din18599::standards::v1::subsets::any::schema::mutations::Din18599Mutation;` |
| `✏️s/🔌️plugins/📕️norm/📦️packages/🦀️rust/📦️glue.rs` | 5171 | `pub use crate::artifacts::din18599::standards::v1::subsets::any::schema::diff::Din18599Diff;` |
| `✏️s/🔌️plugins/📖️playbook/📦️packages/🦀️rust/📦️glue.rs` | 347 | `pub mod op { pub use crate::artifacts::playbook::standards::v1::subsets::any::schema::mutations::text::*; pub use crate::artifacts::playbook::standards::v1::subsets::any::schema::mutations::{apply_playbook_mutation, PlaybookMutation}; }` |
| `✏️s/🔌️plugins/🔋️energy/📦️packages/🦀️rust/📦️glue.rs` | 433 | `pub use crate::artifacts::model::standards::v1::subsets::any::schema::snapshot::EnergyModelSnapshot;` |
| `✏️s/🔌️plugins/🔋️energy/📦️packages/🦀️rust/📦️glue.rs` | 434 | `pub use crate::artifacts::model::standards::v1::subsets::any::schema::mutations::EnergyModelMutation;` |
| `✏️s/🔌️plugins/🔋️energy/📦️packages/🦀️rust/📦️glue.rs` | 435 | `pub use crate::artifacts::model::standards::v1::subsets::any::schema::diff::EnergyModelDiff;` |
| `✏️s/🔌️plugins/🔱️trinity/📦️packages/🦀️rust/📦️glue.rs` | 335 | `pub use crate::artifacts::rewrite::standards::v1::subsets::any::schema::snapshot::RewriteSnapshot;` |
| `✏️s/🔌️plugins/🔱️trinity/📦️packages/🦀️rust/📦️glue.rs` | 336 | `pub use crate::artifacts::rewrite::standards::v1::subsets::any::schema::mutations::RewriteRuleMutation;` |
| `✏️s/🔌️plugins/🔱️trinity/📦️packages/🦀️rust/📦️glue.rs` | 337 | `pub use crate::artifacts::rewrite::standards::v1::subsets::any::schema::diff::RewriteDiff;` |
| `✏️s/🔌️plugins/🔱️trinity/📦️packages/🦀️rust/📦️glue.rs` | 680 | `pub use crate::artifacts::jack::standards::v1::subsets::any::schema::snapshot::JackSnapshot;` |
| `✏️s/🔌️plugins/🔱️trinity/📦️packages/🦀️rust/📦️glue.rs` | 681 | `pub use crate::artifacts::jack::standards::v1::subsets::any::schema::mutations::TrinityGraphMutation;` |
| `✏️s/🔌️plugins/🔱️trinity/📦️packages/🦀️rust/📦️glue.rs` | 682 | `pub use crate::artifacts::jack::standards::v1::subsets::any::schema::diff::JackDiff;` |
| `✏️s/🔌️plugins/🕸️dag/📦️packages/🦀️rust/📦️glue.rs` | 393 | `pub mod op { pub use crate::artifacts::dag::standards::v1::subsets::any::schema::mutations::text::*; pub use crate::artifacts::dag::standards::v1::subsets::any::schema::mutations::DagMutation; }` |
| `✏️s/🔌️plugins/🕸️dag/📦️packages/🦀️rust/📦️glue.rs` | 403 | `pub use crate::artifacts::dag::standards::v1::subsets::any::schema::snapshot::DagSnapshot;` |
| `✏️s/🔌️plugins/🕸️dag/📦️packages/🦀️rust/📦️glue.rs` | 404 | `pub use crate::artifacts::dag::standards::v1::subsets::any::schema::mutations::DagMutation;` |
| `✏️s/🔌️plugins/🕸️dag/📦️packages/🦀️rust/📦️glue.rs` | 405 | `pub use crate::artifacts::dag::standards::v1::subsets::any::schema::diff::DagDiff;` |
| `✏️s/🔌️plugins/🖍️draw/📦️packages/🦀️rust/📦️glue.rs` | 417 | `pub mod op { pub use crate::artifacts::draw::standards::v1::subsets::any::schema::mutations::text::*; pub use crate::artifacts::draw::standards::v1::subsets::any::schema::mutations::{draw_op_for_layer_field, patch_layer_field, DrawMutation}; }` |
| `✏️s/🔌️plugins/🖨️raster/📦️packages/🦀️rust/📦️glue.rs` | 460 | `pub mod op { pub use crate::artifacts::raster::standards::v1::subsets::any::schema::mutations::text::*; pub use crate::artifacts::raster::standards::v1::subsets::any::schema::mutations::{apply_raster_mutation, RasterMutation}; }` |
| `✏️s/🔌️plugins/🗒️note/📦️packages/🦀️rust/📦️glue.rs` | 575 | `pub mod op { pub use crate::artifacts::note::standards::v1::subsets::any::schema::mutations::text::*; pub use crate::artifacts::note::standards::v1::subsets::any::schema::mutations::NoteMutation; }` |
| `✏️s/🔌️plugins/🧩️puzzle/📦️packages/🦀️rust/📦️glue.rs` | 546 | `pub use crate::artifacts::puzzle2d::standards::v1::subsets::any::schema::snapshot::Puzzle2dSnapshot;` |
| `✏️s/🔌️plugins/🧩️puzzle/📦️packages/🦀️rust/📦️glue.rs` | 547 | `pub use crate::artifacts::puzzle2d::standards::v1::subsets::any::schema::mutations::Puzzle2dMutation;` |
| `✏️s/🔌️plugins/🧩️puzzle/📦️packages/🦀️rust/📦️glue.rs` | 548 | `pub use crate::artifacts::puzzle2d::standards::v1::subsets::any::schema::diff::Puzzle2dDiff;` |
| `✏️s/🔌️plugins/🧩️puzzle/📦️packages/🦀️rust/📦️glue.rs` | 1070 | `pub use crate::artifacts::puzzle5d::standards::v1::subsets::any::schema::snapshot::Puzzle5dSnapshot;` |
| `✏️s/🔌️plugins/🧩️puzzle/📦️packages/🦀️rust/📦️glue.rs` | 1071 | `pub use crate::artifacts::puzzle5d::standards::v1::subsets::any::schema::mutations::Puzzle5dMutation;` |
| `✏️s/🔌️plugins/🧩️puzzle/📦️packages/🦀️rust/📦️glue.rs` | 1072 | `pub use crate::artifacts::puzzle5d::standards::v1::subsets::any::schema::diff::Puzzle5dDiff;` |
| `✏️s/🔌️plugins/🧩️puzzle/📦️packages/🦀️rust/📦️glue.rs` | 1739 | `pub use crate::artifacts::puzzle3d::standards::v1::subsets::any::schema::snapshot::Puzzle3dSnapshot;` |
| `✏️s/🔌️plugins/🧩️puzzle/📦️packages/🦀️rust/📦️glue.rs` | 1740 | `pub use crate::artifacts::puzzle3d::standards::v1::subsets::any::schema::mutations::Puzzle3dMutation;` |
| `✏️s/🔌️plugins/🧩️puzzle/📦️packages/🦀️rust/📦️glue.rs` | 1741 | `pub use crate::artifacts::puzzle3d::standards::v1::subsets::any::schema::diff::Puzzle3dDiff;` |
| `✏️s/🔌️plugins/🧱️block/📦️packages/🦀️rust/📦️glue.rs` | 524 | `pub use crate::artifacts::block2d::standards::v1::subsets::any::schema::snapshot::Block2dSnapshot;` |
| `✏️s/🔌️plugins/🧱️block/📦️packages/🦀️rust/📦️glue.rs` | 525 | `pub use crate::artifacts::block2d::standards::v1::subsets::any::schema::mutations::Block2dMutation;` |
| `✏️s/🔌️plugins/🧱️block/📦️packages/🦀️rust/📦️glue.rs` | 526 | `pub use crate::artifacts::block2d::standards::v1::subsets::any::schema::diff::Block2dDiff;` |
| `✏️s/🔌️plugins/🧱️block/📦️packages/🦀️rust/📦️glue.rs` | 1157 | `pub use crate::artifacts::block5d::standards::v1::subsets::any::schema::snapshot::Block5dSnapshot;` |
| `✏️s/🔌️plugins/🧱️block/📦️packages/🦀️rust/📦️glue.rs` | 1158 | `pub use crate::artifacts::block5d::standards::v1::subsets::any::schema::mutations::Block5dMutation;` |
| `✏️s/🔌️plugins/🧱️block/📦️packages/🦀️rust/📦️glue.rs` | 1159 | `pub use crate::artifacts::block5d::standards::v1::subsets::any::schema::diff::Block5dDiff;` |
| `✏️s/🔌️plugins/🧱️block/📦️packages/🦀️rust/📦️glue.rs` | 1754 | `pub use crate::artifacts::block3d::standards::v1::subsets::any::schema::snapshot::Block3dSnapshot;` |
| `✏️s/🔌️plugins/🧱️block/📦️packages/🦀️rust/📦️glue.rs` | 1755 | `pub use crate::artifacts::block3d::standards::v1::subsets::any::schema::mutations::Block3dMutation;` |
| `✏️s/🔌️plugins/🧱️block/📦️packages/🦀️rust/📦️glue.rs` | 1756 | `pub use crate::artifacts::block3d::standards::v1::subsets::any::schema::diff::Block3dDiff;` |

## Engine Shim Lines (`pub use super::standards::*::engine::*`)

| File | Line | Snippet |
|------|-----:|---------|
| `✏️s/🔌️plugins/✒️writer/📦️packages/🦀️rust/📦️glue.rs` | 287 | `pub use super::standards::v1::engine::*;` |
| `✏️s/🔌️plugins/➗️mathematical/📦️packages/🦀️rust/📦️glue.rs` | 351 | `pub use super::standards::v1::engine::*;` |
| `✏️s/🔌️plugins/🌀️procedural/📦️packages/🦀️rust/📦️glue.rs` | 370 | `pub use super::standards::v1::engine::*;` |
| `✏️s/🔌️plugins/🌀️procedural/📦️packages/🦀️rust/📦️glue.rs` | 787 | `pub use super::standards::v1::engine::*;` |
| `✏️s/🔌️plugins/🌊️flow/📦️packages/🦀️rust/📦️glue.rs` | 309 | `pub use super::standards::v1::engine::*;` |
| `✏️s/🔌️plugins/🌍️gis/📦️packages/🦀️rust/📦️glue.rs` | 372 | `pub use super::standards::v1::engine::*;` |
| `✏️s/🔌️plugins/🌍️gis/📦️packages/🦀️rust/📦️glue.rs` | 777 | `pub use super::standards::v1::engine::*;` |
| `✏️s/🔌️plugins/🌿️vcs/📦️packages/🦀️rust/📦️glue.rs` | 304 | `pub use super::standards::v1::engine::*;` |
| `✏️s/🔌️plugins/🎞️animate/📦️packages/🦀️rust/📦️glue.rs` | 434 | `pub use super::standards::v1::engine::*;` |
| `✏️s/🔌️plugins/🎥️shooting/📦️packages/🦀️rust/📦️glue.rs` | 651 | `pub use super::standards::v1::engine::*;` |
| `✏️s/🔌️plugins/🎪️demonstrator/📦️packages/🦀️rust/📦️glue.rs` | 257 | `pub use super::standards::v1::engine::*;` |
| `✏️s/🔌️plugins/🎬️sequence/📦️packages/🦀️rust/📦️glue.rs` | 304 | `pub use super::standards::v1::engine::*;` |
| `✏️s/🔌️plugins/🏗️fem/📦️packages/🦀️rust/📦️glue.rs` | 515 | `pub use super::standards::v1::engine::*;` |
| `✏️s/🔌️plugins/🏗️fem/📦️packages/🦀️rust/📦️glue.rs` | 999 | `pub use super::standards::v1::engine::*;` |
| `✏️s/🔌️plugins/🏛️architect/📦️packages/🦀️rust/📦️glue.rs` | 2672 | `pub use super::standards::v1::engine::*;` |
| `✏️s/🔌️plugins/🏭️process/📦️packages/🦀️rust/📦️glue.rs` | 505 | `pub use super::standards::v1::engine::*;` |
| `✏️s/🔌️plugins/💠️lowpoly/📦️packages/🦀️rust/📦️glue.rs` | 498 | `pub use super::standards::v1::engine::*;` |
| `✏️s/🔌️plugins/💡️reasoning/📦️packages/🦀️rust/📦️glue.rs` | 367 | `pub use super::standards::v1::engine::*;` |
| `✏️s/🔌️plugins/📋️forms/📦️packages/🦀️rust/📦️glue.rs` | 307 | `pub use super::standards::v1::engine::*;` |
| `✏️s/🔌️plugins/📏️layout/📦️packages/🦀️rust/📦️glue.rs` | 505 | `pub use super::standards::v1::engine::*;` |
| `✏️s/🔌️plugins/📐️cad/📦️packages/🦀️rust/📦️glue.rs` | 564 | `pub use super::standards::v1::engine::*;` |
| `✏️s/🔌️plugins/📕️norm/📦️packages/🦀️rust/📦️glue.rs` | 340 | `pub use super::standards::v1::engine::*;` |
| `✏️s/🔌️plugins/📕️norm/📦️packages/🦀️rust/📦️glue.rs` | 623 | `pub use super::standards::v1::engine::*;` |
| `✏️s/🔌️plugins/📕️norm/📦️packages/🦀️rust/📦️glue.rs` | 933 | `pub use super::standards::v1::engine::*;` |
| `✏️s/🔌️plugins/📕️norm/📦️packages/🦀️rust/📦️glue.rs` | 1602 | `pub use super::standards::v1::engine::*;` |
| `✏️s/🔌️plugins/📕️norm/📦️packages/🦀️rust/📦️glue.rs` | 1804 | `pub use super::standards::v1::engine::*;` |
| `✏️s/🔌️plugins/📕️norm/📦️packages/🦀️rust/📦️glue.rs` | 2194 | `pub use super::standards::v1::engine::*;` |
| `✏️s/🔌️plugins/📕️norm/📦️packages/🦀️rust/📦️glue.rs` | 2612 | `pub use super::standards::v1::engine::*;` |
| `✏️s/🔌️plugins/📕️norm/📦️packages/🦀️rust/📦️glue.rs` | 2868 | `pub use super::standards::v1::engine::*;` |
| `✏️s/🔌️plugins/📕️norm/📦️packages/🦀️rust/📦️glue.rs` | 3169 | `pub use super::standards::v1::engine::*;` |
| `✏️s/🔌️plugins/📕️norm/📦️packages/🦀️rust/📦️glue.rs` | 3451 | `pub use super::standards::v1::engine::*;` |
| `✏️s/🔌️plugins/📕️norm/📦️packages/🦀️rust/📦️glue.rs` | 3751 | `pub use super::standards::v1::engine::*;` |
| `✏️s/🔌️plugins/📕️norm/📦️packages/🦀️rust/📦️glue.rs` | 4051 | `pub use super::standards::v1::engine::*;` |
| `✏️s/🔌️plugins/📕️norm/📦️packages/🦀️rust/📦️glue.rs` | 4603 | `pub use super::standards::v1::engine::*;` |
| `✏️s/🔌️plugins/📕️norm/📦️packages/🦀️rust/📦️glue.rs` | 4939 | `pub use super::standards::v1::engine::*;` |
| `✏️s/🔌️plugins/📕️norm/📦️packages/🦀️rust/📦️glue.rs` | 5158 | `pub use super::standards::v1::engine::*;` |
| `✏️s/🔌️plugins/📖️playbook/📦️packages/🦀️rust/📦️glue.rs` | 342 | `pub use super::standards::v1::engine::*;` |
| `✏️s/🔌️plugins/📜️imperative/📦️packages/🦀️rust/📦️glue.rs` | 265 | `pub use super::standards::v1::engine::*;` |
| `✏️s/🔌️plugins/📸️remodel/📦️packages/🦀️rust/📦️glue.rs` | 680 | `pub use super::standards::v1::engine::*;` |
| `✏️s/🔌️plugins/🔋️energy/📦️packages/🦀️rust/📦️glue.rs` | 422 | `pub use super::standards::v1::engine::*;` |
| `✏️s/🔌️plugins/🔱️trinity/📦️packages/🦀️rust/📦️glue.rs` | 324 | `pub use super::standards::v1::engine::*;` |
| `✏️s/🔌️plugins/🔱️trinity/📦️packages/🦀️rust/📦️glue.rs` | 669 | `pub use super::standards::v1::engine::*;` |
| `✏️s/🔌️plugins/🕸️dag/📦️packages/🦀️rust/📦️glue.rs` | 388 | `pub use super::standards::v1::engine::*;` |
| `✏️s/🔌️plugins/🖍️draw/📦️packages/🦀️rust/📦️glue.rs` | 412 | `pub use super::standards::v1::engine::*;` |
| `✏️s/🔌️plugins/🖨️raster/📦️packages/🦀️rust/📦️glue.rs` | 455 | `pub use super::standards::v1::engine::*;` |
| `✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/📦️glue.rs` | 160 | `pub use super::standards::v_raw::engine::*;` |
| `✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/📦️glue.rs` | 313 | `pub use super::standards::v_utf_8::engine::*;` |
| `✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/📦️glue.rs` | 481 | `pub use super::standards::v_rfc8259::engine::*;` |
| `✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/📦️glue.rs` | 649 | `pub use super::standards::v1_0::engine::*;` |
| `✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/📦️glue.rs` | 802 | `pub use super::standards::v_rfc4180::engine::*;` |
| `✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/📦️glue.rs` | 955 | `pub use super::standards::v_commonmark::engine::*;` |
| `✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/📦️glue.rs` | 1092 | `pub use super::standards::v_rfc1950::engine::*;` |
| `✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/📦️glue.rs` | 1268 | `pub use super::standards::v2_0::engine::*;` |
| `✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/📦️glue.rs` | 1486 | `pub use super::standards::v_ap214::engine::*;` |
| `✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/📦️glue.rs` | 1795 | `pub use super::standards::v4::engine::*;` |
| `✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/📦️glue.rs` | 1939 | `pub use super::standards::v1_0::engine::*;` |
| `✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/📦️glue.rs` | 2075 | `pub use super::standards::v2_0::engine::*;` |
| `✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/📦️glue.rs` | 2220 | `pub use super::standards::v3_0::engine::*;` |
| `✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/📦️glue.rs` | 2356 | `pub use super::standards::v1_0::engine::*;` |
| `✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/📦️glue.rs` | 2492 | `pub use super::standards::v_r12::engine::*;` |
| `✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/📦️glue.rs` | 2652 | `pub use super::standards::v_ascii::engine::*;` |
| `✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/📦️glue.rs` | 2825 | `pub use super::standards::v1_1::engine::*;` |
| `✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/📦️glue.rs` | 2977 | `pub use super::standards::v_v3::engine::*;` |
| `✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/📦️glue.rs` | 3222 | `pub use super::standards::v_ac1024::engine::*;` |
| `✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/📦️glue.rs` | 3407 | `pub use super::standards::v1_2::engine::*;` |
| `✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/📦️glue.rs` | 3825 | `pub use super::standards::v1_7::engine::*;` |
| `✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/📦️glue.rs` | 4016 | `pub use super::standards::v_jfif_1_01::engine::*;` |
| `✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/📦️glue.rs` | 4296 | `pub use super::standards::v89a::engine::*;` |
| `✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/📦️glue.rs` | 4477 | `pub use super::standards::v6_0::engine::*;` |
| `✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/📦️glue.rs` | 4669 | `pub use super::standards::v_ecma_376::engine::*;` |
| `✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/📦️glue.rs` | 4877 | `pub use super::standards::v_ecma_376::engine::*;` |
| `✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/📦️glue.rs` | 5083 | `pub use super::standards::v_ecma_376::engine::*;` |
| `✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/📦️glue.rs` | 5251 | `pub use super::standards::v2_1::engine::*;` |
| `✏️s/🔌️plugins/🗒️note/📦️packages/🦀️rust/📦️glue.rs` | 570 | `pub use super::standards::v1::engine::*;` |
| `✏️s/🔌️plugins/🧩️puzzle/📦️packages/🦀️rust/📦️glue.rs` | 535 | `pub use super::standards::v1::engine::*;` |
| `✏️s/🔌️plugins/🧩️puzzle/📦️packages/🦀️rust/📦️glue.rs` | 1059 | `pub use super::standards::v1::engine::*;` |
| `✏️s/🔌️plugins/🧩️puzzle/📦️packages/🦀️rust/📦️glue.rs` | 1728 | `pub use super::standards::v1::engine::*;` |
| `✏️s/🔌️plugins/🧱️block/📦️packages/🦀️rust/📦️glue.rs` | 513 | `pub use super::standards::v1::engine::*;` |
| `✏️s/🔌️plugins/🧱️block/📦️packages/🦀️rust/📦️glue.rs` | 1146 | `pub use super::standards::v1::engine::*;` |
| `✏️s/🔌️plugins/🧱️block/📦️packages/🦀️rust/📦️glue.rs` | 1743 | `pub use super::standards::v1::engine::*;` |
| `✏️s/🔌️plugins/🪐️space/📦️packages/🦀️rust/📦️glue.rs` | 271 | `pub use super::standards::v1::engine::*;` |
| `✏️s/🔌️plugins/🪵️sourcing/📦️packages/🦀️rust/📦️glue.rs` | 307 | `pub use super::standards::v1::engine::*;` |
