# Demo Law Module Ownership

Pass546 moves the nine remaining demo law modules from parent plugins into their extracted artifact crates: Equation, Playbook, Program, Wires, Forms, Layout, DAG, Flow and Raster. Current source inspection confirmed that each law addresses its artifact through crate-root types or schema modules, and each artifact manifest compiles its own root. The test source, assets and assertions were not changed. This applies the same ownership correction prompted by native532's Lowpoly, Procedure, CAD and Process errors.

- `✏️s/🔌️plugins/➗️mathematical/📦️packages/🦀️rust/🦀️.rs`
- `✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/🦀️.rs`
- `✏️s/🔌️plugins/📖️playbook/📦️packages/🦀️rust/🦀️.rs`
- `✏️s/🔌️plugins/📖️playbook/🗿️artifacts/📖️playbook/🦀️.rs`
- `✏️s/🔌️plugins/🏛️architect/📦️packages/🦀️rust/🦀️.rs`
- `✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🦀️.rs`
- `✏️s/🔌️plugins/💡️reasoning/📦️packages/🦀️rust/🦀️.rs`
- `✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/🦀️.rs`
- `✏️s/🔌️plugins/📋️forms/📦️packages/🦀️rust/🦀️.rs`
- `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🦀️.rs`
- `✏️s/🔌️plugins/📏️layout/📦️packages/🦀️rust/🦀️.rs`
- `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🦀️.rs`
- `✏️s/🔌️plugins/🕸️dag/📦️packages/🦀️rust/🦀️.rs`
- `✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/🦀️.rs`
- `✏️s/🔌️plugins/🌊️flow/📦️packages/🦀️rust/🦀️.rs`
- `✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🦀️.rs`
- `✏️s/🔌️plugins/🖨️raster/📦️packages/🦀️rust/🦀️.rs`
- `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🦀️.rs`

Syntax, compiler and execution of these laws remain to be verified.

