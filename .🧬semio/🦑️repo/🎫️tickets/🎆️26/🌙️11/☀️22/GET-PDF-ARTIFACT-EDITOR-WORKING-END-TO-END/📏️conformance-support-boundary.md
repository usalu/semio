# Conformance-support source boundary

## What the boundary counts
`readRustUnitSourceClosureV1` (repo `📚️library/⚡️caching/🦀️cargo/📦️units/📥️inputs/🟦️.ts`) walks the crate entry through every `mod`/`#[path]` mount and `fixture_text!`/include input, `cfg(test)` mounts and fixtures included. The limit is `min(boundary.maxSourceFiles = 32, unit.limits.maximumSourceFiles)` files and 262144 bytes. Exceeding it throws `Rust crate Source boundary exceeded before compiler`, shown as "33 of 32 files".

## Why 33
`semio-s-artifact-stdio-pdf-conformance-support` mirrors the 1.7 writer/colour modules, the 1.4 a/x io mounts, the base io tests, the five 1.7 mutation leaves and, in its cfg(test) nest, four 1.4 mutation round-trip tests with their four fixtures (8 files). Adding `writer/🌊️dct-object.rs` made 33. The stopgap of inlining it into `writer/🦀️.rs` broke the one-file-per-component rule.

## Decision
1. Restored `📝️writer/🌊️dct-object.rs` as its own module (`#[path = "🌊️dct-object.rs"] pub mod dct_object;`); content is byte-identical to the committed file.
2. The four round-trip tests (set-page-text, clear-page-text, set-page-size, collapse-page-size) and their fixtures left the closure. They cannot sit in the mutation crates: they need `base::io::mutation_bridge` from graph-projection, which depends on schema-2, which depends on the mutation crates (a cycle). They cannot sit in graph-projection or schema-2 either, because they need the mutation aggregates. The only acyclic home is a crate downstream of conformance-support, so a new carve crate owns them: `semio-s-artifact-stdio-pdf-mutation-round-trip-tests` at `🏅️standards/4️⃣1.4/🪆️subsets/🧱️base/🧪️tests/🔄️mutation-round-trip/📦️packages/🦀️rust`. Test and fixture files stay where they are in their domain folders; only the mounts moved, and the module paths (`standards::v1_4::subsets::{a,x}::schema::mutations::<leaf>::tests`) are unchanged.
3. conformance-support now re-exports the a/x `mutations` modules directly; the unused `semio-s-artifact-stdio-pdf-tests` dependency, dev-dependency and testkit entry were dropped and the four `::tests` entries left its carve `owned` list.

Closure of conformance-support: 33 (with dct) minus 8 = 25 of 32.

## Note
The crates are produced by the crate-tree carve planner (`🏗️authoring/📦️crate-tree`); the new crate is hand-written in exactly the shape the planner emits (`🦀️.rs` wiring, `🪞️mirror/🦀️.rs`, `owned` list). A future re-carve will regroup it.

## Verification
- Native `cargo check -p semio-s-artifact-stdio-pdf-mutation-round-trip-tests --tests` passes (4 unused-import warnings in the moved tests, as before).
- `repo cargo -- check -p semio-s-artifact-stdio-pdf-conformance-support --lib` passed source admission (no boundary refusal) and reached the unit-graph stage; the compile itself was still queued behind shared capacity when this was written.
- Native `cargo test -p semio-s-artifact-stdio-pdf-mutation-round-trip-tests --lib`: 11 passed, 1 failed. The failure is `x::schema::mutations::collapse_page_size::tests::inverse_diffs_sum_to_the_negative_diff` (the summed inverse diff carries width and height, the negative of the forward diff only width). The test file and mutation code are unchanged by this move, so this looks like a pre-existing `CollapsePageSize` inverse-law defect now visible; it needs its own fix (not done here).

## CollapsePageSize inverse-law fix
- Symptom: `collapse_page_size::tests::inverse_diffs_sum_to_the_negative_diff` summed an inverse diff with width and height while the negative of the forward diff carried only width.
- Root cause: `CollapsePageSize` forward diff is correctly sparse (width only, by its semantics: "collapse page width to zero"). Its inverse is `SetPageSize{width,height}`, whose `diff` always emitted both fields, even when height was unchanged. The over-reporting side was `SetPageSize::diff`, violating the "sparse diff" contract.
- Fix: `SetPageSize::diff` (1.4 x `📐️set-page-size/🦀️.rs`) now emits `width`/`height` only when they differ from the base page.
- Siblings: `ResizePage::diff` (1.4 base `📐️resize-page/🦀️.rs`) had the identical always-both pattern and got the same sparse fix. 1.4 a mutations (text only) and 1.7 mutations (box/info/object leaves, no width/height pair) have no such diff; the 1.7 editor files only construct these mutations.
- Verification: see the final status below.
