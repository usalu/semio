# Snapshot Equation Delta Orphans (11/24)

Tree: `/Users/ueli/.sinc/play`. Nothing was compiled. No cargo or rustc was run. Every finding comes from reading source plus a crate-aware `#[path]`/`include!` resolver run over 3161 plugin `Cargo.toml` roots.

## Root cause

`list_delta!` and `plain_list_delta!` (`🧰️framework/🛍️products/💻️os/🔨️modules/🪡️list-delta/🦀️.rs`) emit `impl $crate::Keyed for $row` only for these key spellings:
- `key: <field>`, which expands to `|row| row.<field>.clone()`
- `key: T = |row| expr`

Other spellings emit no foreign-trait impl:
- `= keyed` and `= by Marker` emit none.
- `row: String` emits none.
- `row_patch!` emits only `impl RowPatch<$row> for $patch` on a patch struct it defines itself, so that impl is always local.

The equation schema diff (`✳️any/🧬️schema/🔺️diff/🦀️.rs`) is compiled only by `semio-s-artifact-mathematical-equation-inferences`, through its `🪞️mirror`. Its row types `EquationNode` and `EquationEdge` live in `💡️inferences/🌿️cas-internals/🧮️expr/🦀️.rs` `artifact_items_2`, which only `semio-s-artifact-mathematical-equation-snapshot` compiles. So `key: id` produced an orphan `Keyed` impl (E0117).

## Fix (wfc / layout / fem pattern)

Each owning crate implements `Keyed`, and the diff uses the `key: String = keyed` spelling.
- `…/➗️equation/…/✳️any/🧬️schema/💡️inferences/🌿️cas-internals/🧮️expr/🦀️.rs`: added `impl semio_os_kernel::list_delta::Keyed for EquationNode` and the same for `EquationEdge`, both keyed by `id`. The snapshot crate already depends on `semio-os-kernel`.
- `…/➗️equation/…/✳️any/🧬️schema/🔺️diff/🦀️.rs`: both invocations now use `list: Vec<…>, key: String = keyed, values_only`.

Type paths do not change. Every type stays where it was, so the editor, the hub and the inferences re-exports are unaffected. No dependency or `Cargo.lock` change was needed.

## Sweep (`✏️s/🔌️plugins`, 589 invocations)

| Form | Count | Status |
|---|---|---|
| `by Marker` | 264 | Clean by construction (the macro emits no impl) |
| `row_patch!` | 205 | Clean by construction (the patch type is local) |
| `keyed` (before the fixes) | 31 | Clean by construction |
| `row: String` | 2 | Clean by construction |
| `key: <field>` / closure | 87 | Checked crate by crate, see below |

Of the 87 field/closure invocations, 2 were the equation ones fixed above. That leaves 85:
- **19 local, clean:** fem 2d diff ×5 and loads ×1, block document-fields ×4, norm design-case rows ×3, animate tile-grid, space editor config, gis gismap, sourcing curation, gltf collection-algebra ×2.
- **66 provable orphans:** all in `🏛️architect/…/🏛️program/…/🧬️schema/🔺️diff/🦀️.rs`, which is compiled by `semio-s-artifact-architect-program-inferences`. These rows are defined in other crates:
  - 22 in `…-program-structure`
  - 30 in `…-context-requirements`
  - 13 in `…-registers`
  - `TraceLink` in `…-tests` (the crate holding `🧱️kernel`)

  This run did not report them because `semio-s-artifact-architect-program-snapshot` failed earlier (E0433), so the inferences crate was never compiled.

### Architect fix

- `…/🏛️program/…/✳️any/🧬️schema/🧱️kernel/🦀️.rs`:
  - The `impl_identified_header!` macro now also emits `impl protocol::list_delta::Keyed for $ty`, keyed by `header.id.0`. Its 65 call sites sit in the 11 register files that own the rows, inside the 3 owning crates. Each of those crates aliases `semio_os_kernel as protocol` and depends on it.
  - Added a hand-written `Keyed` impl for `TraceLink`, keyed by `id.0`.
- The 66 invocations in `…/🏛️program/…/🔺️diff/🦀️.rs` now use `key: String = keyed`.
- No dependency or lockfile change was needed.

## Unresolved / not checked

- Hand-written `KeyOf` impls for `by Marker` markers were not part of this sweep.
- Not checked: the architect snapshot E0433, and the other crate errors in this run log (forms diff, architect RetireOwned, …).
