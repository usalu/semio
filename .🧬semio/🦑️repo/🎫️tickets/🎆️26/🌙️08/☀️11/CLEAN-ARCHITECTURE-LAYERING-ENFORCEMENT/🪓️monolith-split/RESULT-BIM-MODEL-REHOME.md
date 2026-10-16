# Result: bim model `🏢️model/🦀️.rs` re-homed (seam map section C)

Status: applied to the live tree. Cargo is gated, so the gate is module-tree equality plus the full-resolution differential on all five configurations. No compile or test was run.

Tool: `🪓️monolith-split/🧱️bim-rehome/📜️script.ts` (`extract`, `plan`, `apply`, `compare`, `oracle`, `oracle-diff`). Logs: `🗑️generated/bimre-oracle.log`, `🗑️generated/bimre-plan.log`.

## 1. Result in numbers

| | before | after |
|---|---|---|
| `🏢️model/🦀️.rs` | 7,817 lines | 127 lines (66 lines declaration code and unit-test mount, 61 lines mounts) |
| `#[path]` attributes (all files together) | 2,735 | 2,735 |
| `#[cfg(...)]` attributes (all files together) | 1,589 | 1,589 |
| modules in the resolved tree | 3,284 | 3,284 |
| files touched | | 234: 1 root rewritten, 12 existing files appended, 221 new `🦀️.rs` mount files |

## 2. Approach

The script parses the root with a real Rust lexer and item parser, not a line scan. It handles comments, strings and raw strings, and tracks `#[path]` and `#[cfg]` attributes. It then re-emits the inline tree by these rules:

1. **Wrapper with children.** Wrapper means the shape `#[path="."] pub mod W { #[path=F] mod component; pub use component::*; …children }`. The children move into `F` (the component of `W`), in a delimited `//#region 🧩️Modules` block at the end of the existing file. Every `#[path]` is rewritten relative to the directory of the declaring file. `W` stays as a wrapper without its children, so `crate::…::W::child` keeps resolving through the glob.
2. **Hop guard.** The hop adds one module level between a child and `W`. A child may hop only when both hold:
   - No file below it climbs more `super::` than its depth below `W` (climb ≤ depth). The check is token based, covers `pub(super)`, and is conservative.
   - Its name is not already bound in `F` (items and `use` leaves).

   A child that fails stays inline in the wrapper in the parent's file. Guard hits, all documented in `bimre-plan.log`:
   - `v1::subsets`, `any::schema` and `any::io` collide with `use …::{io, schema}` and `use …::subsets` in their components. The first attempt without the check produced E0255.
   - `element_solids::{walls, curtain_walls, fillers, wall_sweeps}` use `super::super::super::` (3 levels) in 4 files. The first attempt produced E0433 and E0432.
3. **Mutation wrappers (216).** The component is `<m>/🦠️mutation/🦀️.rs`, so no existing file in `<m>/` can own `diff`, `inverse` and the tests. Moving them into the payload file would break `super::super::delete_site` (2 levels). Leaving them inline would grow `🧬️mutations/🦀️.rs` to about 7,500 lines, and that file is `include_str!`-published as the mutations Rust leaf. So each mutation gets the canonical direct-leaf file `<m>/🦀️.rs`. The taxonomy already names it (`_mutationOwnershipComment`: "directly owns one 🦀️.rs"). The module `mutations::<m>` is now this file, and its children (`component`, `diff`, `inverse`, `tests_*`) keep their exact names, visibility and cfg. `🧬️mutations/🦀️.rs` gains 216 canonical mounts of the form `#[path = "<m>/🦀️.rs"] pub mod <m>;`, going from 501 to 950 lines.
4. **Collection namespaces without a component** (`commands`, `modes`, `panels`, `examples`) become file modules at their natural directory, with precedent in 27 `📚️examples`, 11 `🎮️commands`, 8 `📌️panels` and 2 `🎭️modes` mount files elsewhere in the repo. 5 new files:
   - `📚️examples/🦀️.rs`
   - `✏️editor/🎮️commands/🦀️.rs` (272 lines)
   - `✏️editor/📌️panels/🦀️.rs`
   - `✏️editor/🎭️modes/🦀️.rs`
   - `👁️viewer/🎭️modes/🦀️.rs`

   This keeps the bim editor component (`✏️editor/🦀️.rs`) at 1,264 lines instead of about 1,640.
5. Every other inline module (127) is re-emitted unchanged. New files start with a `//! <folder emoji>` docstring.

The 12 existing files that received a block: the bim editor and viewer components, the edit and view mode components, the house, office and demo example components, the `🧬️schema`, `💡️inferences`, `🧊️element-solids`, `🧬️mutations` and `🦉️wall-geometry` components. Each keeps its original content byte-identical as the prefix. The script asserts this, and every append is guarded by a SHA-256 of the file at plan time (a peer is editing `👁️viewer/🦀️.rs` right now; the guard would have refused the write if it had changed).

## 3. Module-tree equality (primary gate)

`extract` mirrors rustc's module-file resolution, including that `#[path]`-loaded files own their directory (verified empirically with a rustc probe). It lists every module as path, kind, resolved file, visibility and accumulated cfg. `compare` collapses the glob hop (`mod component; pub use component::*;`) and requires that the original and the result match module for module:

```
{"origModules":3284,"newModules":3284,"inlineToFileConversions":221,"problems":0}
```

- The 221 inline-to-file conversions are exactly the 221 created files; `compare` rejects any conversion whose file is not in the plan.
- `missing` (a `#[path]` that does not resolve) is false for all 3,284 modules, in the original and in the result.
- Negative control: three injected faults (dropped cfg, retargeted file, dropped row) were caught by 3 reported problems.
- The check also ran against the live tree after `apply`, with the same result.

## 4. Full-resolution differential (`📓️RESOLUTION-ORACLE.md`)

`rustc +nightly-2026-07-20 --edition 2021 --crate-type lib --crate-name semio_s_artifact_bim_model --emit=metadata --error-format=json` from the crate lib root, original sandbox against result. `-o /dev/null` fails under the sandbox, so the output goes to a scratch `.rmeta`; nothing reaches the gated store. Diagnostics are keyed by level, code, message, file relative to the crate root and primary label (line numbers dropped, sandbox root and `/./` normalised, `component::` hops stripped from messages).

| configuration | original | result | differing |
|---|---|---|---|
| native | 703 (E0432 465, E0433 233, E0463 5) | 703 | 0 |
| `--test` | 951 | 951 | 0 |
| `--target wasm32-wasip2` | 703 | 703 | 0 |
| `--test --target wasm32-wasip2` | 951 | 951 | 0 |
| `--target wasm32-unknown-unknown` | 703 | 703 | 0 |
| live tree: native, `--test`, wasip2 | | same counts | 0 |

All diagnostics are external-crate errors (the gate prevents building dependencies), so every `crate::`, `super::` and `self::` path resolves in both. The oracle is sensitive: the first plan iteration produced 14 differing diagnostics (E0255 name clashes, E0433 and E0432 from `super::super::super::`), which led to the guards in section 2. E0603, E0412, E0425 and the other listed codes: none new.

Limits: typeck is cut short by the missing dependencies, so field and method privacy (E0616, E0624) is not exercised. Argument for safety: every moved child stays a descendant of the same module chain it had, plus one extra ancestor that is a glob-visible superset of its former parent. `module_path!()` is used only below `io`, which is not moved.

## 5. Other checks

- `rustfmt +nightly --config skip_children=true --emit stdout` on all 234 touched files: exit 0, no output. Negative control: a broken file exits 1.
- Path length: the longest touched path is 221 bytes absolute (limit 240).
- Losslessness: `#[path]` 2,735 = 2,735 and `#[cfg(` 1,589 = 1,589 over root plus created files plus appended blocks. Existing content of the 12 appended files is a byte prefix.
- Sizes: largest touched file 1,264 lines (`✏️editor/🦀️.rs`), then 950 (`🧬️mutations/🦀️.rs`), all under 2,500.

## 6. Source-text readers and generators

- `include_str!` readers of the touched files: `🧬️schema/🦀️.rs` publishes `🦀️.rs`, `📸️snapshot/🦀️.rs`, `🔺️diff/🦀️.rs` and `🧬️mutations/🦀️.rs` as Rust schema leaves, and `💡️inferences/🦀️.rs` publishes itself. The appended mount blocks are therefore visible in the published Rust text of the schema artifact, mutations and inferences leaves. The mutations leaf already carried 10 `#[path]` mounts before. No test reads these texts beyond JSON, TypeScript and GraphQL leaf presence (checked `🧬️schema/🧪️tests`).
- The seam-map readers (`include_str!` assertions) concern the puzzle editors, not bim. The repo grep for the literal `🏢️model/🦀️.rs` outside `.🧬semio` found no reader (checked with `git grep -F` over all tracked files, which returned nothing, and with the recursive `/usr/bin/grep -rn --exclude-dir=node_modules --exclude-dir=.🧬semio -F` over the whole tree, which finished with 7 hits, all generated files under `.cache/` and none of them source).
- **Scaffolder / generator: no change needed.** No generator emits mounts into this manifest. The only code that writes Rust mounts is `mountProjectedRustLeaf` in `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🧬️surface-schema/🟦️.ts`. It locates the `#[path]` mount of a surface by indexing every `#[path]` of all plugin Rust files, so it follows the moved declarations. Checked with `findSurfaceMountAnchor` on the original and the result: both resolve the editor and viewer surfaces to the unchanged root wrappers (`bim`), whose canonical `mod component; pub use component::*;` shape is preserved. The `surface-scaffold` (`new surface`) and `authorArtifactScaffold` write no Rust mounts. The `normalization` rename tool rewrites `#[path]` by resolving them from any file, so it handles the new declaring files. A new mutation is now added by creating `<m>/🦀️.rs` and one `#[path = "<m>/🦀️.rs"] pub mod <m>;` in `🧬️mutations/🦀️.rs`, which is also the canonical form the mutation structural-reachability gate (`📐️structural-reachability`) expects for the aggregate mount.

## 7. Caveats

- The mutation structural gate still needs `pub use <child>::<Payload>` for the wrapped-payload proof; bim uses glob re-exports, as before. The aggregate mount condition of that gate is now met, which it was not before. Not run.
- Taxonomy validation of the 221 new `🦀️.rs` files was not run (gated). Direct-leaf `🦀️.rs` in a mutation directory and `🦀️.rs` in `🎮️commands`, `🎭️modes`, `📌️panels` and `📚️examples` all have precedent in the repo.
- Hop modules now live at `…::W::component::X` in the raw tree. Code that names modules by absolute path or `super::` is unaffected (proved by the oracle); anything that prints `module_path!()` for moved modules would see the extra segment (none exists).
- A peer has uncommitted edits to `Cargo.toml`, `📦️packages/🦀️rust/Cargo.toml` and `👁️viewer/🦀️.rs` (4 lines). They are untouched, apart from the appended viewer block.
