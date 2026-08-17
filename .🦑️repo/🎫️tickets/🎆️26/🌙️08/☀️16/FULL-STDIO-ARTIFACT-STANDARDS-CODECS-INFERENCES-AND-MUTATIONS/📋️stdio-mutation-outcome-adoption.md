# Stdio Mutation Outcome Adoption

## Frozen Contract

The authoritative builder trait is [`ArtifactBuilder`](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs) at `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs:797`. Its mutation boundary is:

```rust
fn mutate(self, mutation: Self::Mutation) -> (Self, protocol::MutationOutcome<Self::Diff>);
```

`MutationOutcome<D>` is defined in `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🦀️component.rs:177`. The adopted access pattern is `outcome.diff()` only at a real raw-diff boundary; public apply helpers return the whole outcome after applying that raw diff. `messages()` remains available to callers and tests.

## Implementations And Callers

The final non-GLTF stdio implementation inventory contains 81 `ArtifactBuilder::mutate` implementations with the frozen outcome signature. The complete reproducible location inventory is:

```sh
rg -n --glob '🦀️component.rs' \
  'fn mutate\\(mut self, mutation: Self::Mutation\\) -> \\(Self, protocol::MutationOutcome<Self::Diff>\\)' \
  '✏️s/🔌️plugins/🗄️stdio' -g '!**/🧊️gltf/**' -g '!**/gltf/**'
```

The trait implementation locations directly repaired or audited in the final test-consumer pass were:

- `🌐️html/.../🧬️schema/🧬️mutations/🦀️component.rs`
- `🎞️pptx/.../✳️any/🧬️schema/🦀️component.rs` and `🧬️mutations/🦀️component.rs`
- `🎞️pptx/.../✳️strict/🧬️schema/🦀️component.rs` and `✳️transitional/🧬️schema/🦀️component.rs` (already correct forwarding outcomes; audited)
- `📄txt/.../🧬️schema/🦀️component.rs` and `🧬️mutations/🦀️component.rs`
- `📝️md/.../🧬️schema/🦀️component.rs`
- `📰xml/.../🧬️schema/🦀️component.rs`
- `💾️binary/.../🧬️schema/🧬️mutations/🦀️component.rs`
- `📊️csv/.../🧬️schema/💡️inferences/🦀️component.rs`
- `📕️xlsx/.../🧬️schema/🧬️mutations/🦀️component.rs`
- `📜️docx/.../🧬️schema/🧬️mutations/🦀️component.rs`
- `🔣️json/.../🧬️schema/🧬️mutations/🦀️component.rs`
- `🧿️semio/.../✳️audio|document|presentation/🧬️schema/🦀️component.rs`
- `🧿️semio/.../✳️document|mesh|presentation|value|video/🧬️schema/🧬️mutations/🦀️component.rs`
- `🧿️semio/.../✳️flow/🧬️schema/🔺️diff/🦀️component.rs`

The parent-path ellipses above expand from `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts`; they are intentionally limited to the assigned non-GLTF stdio files. No framework, store, plugin, or GLTF implementation was modified in this lane.

## Source Adoption

- Forwarding leaf mutation functions and direct mutation implementations now return `MutationOutcome<Diff>`.
- Public `apply_*_mutation` helpers compute the outcome, apply `outcome.diff()`, and return the untouched outcome.
- Semio envelope delegation maps child outcomes through `MutationOutcome::map`, preserving child diagnostics. Its kind mismatch produces the `mutation.target-kind-mismatch` error outcome with an unapplied default diff.
- Test helpers that semantically return an outcome now have `MutationOutcome<Diff>` return types. Diff algebra tests project only the underlying diff for `apply`, `inverse`, `absorb`, `encode_diff`, and `is_empty`.
- The 19 malformed test files were repaired by explicit file-local patches. The Semio aggregate mismatch test asserts both its unchanged diff and its preserved mismatch diagnostic.

## Final Consumer Pass

The final no-run consumer remediation changed 21 assigned files and audited 23. It repaired 43 `assert_absorb_matches_sequential` calls by passing `d1.diff()` and `d2.diff()`, and corrected raw-diff use in protocol encoding, builder re-absorption, diff algebra, and no-op assertions. The two PPTX forwarding schemas required no edit because they already forward their full outcome.

## Verification

- `rustfmt --edition 2021 --check` passed for the 19 repaired test files, the two P1 implementation repairs, and all 23 final assigned consumer paths.
- `git diff --check` passed for each scoped batch.
- Non-GLTF stdio static scans are zero for literal `${1}`/`${name}`, broad `${...}`, accidental `++ ✏️s/` markers, bare diff helper returns, legacy `ArtifactBuilder` signatures, direct `mutation.diff(base).apply`, and accidental `.diff().diff().apply`.
- The serialized `cargo check -p semio-s-plugin-stdio --lib` passed after the two bounded P1 source fixes.
- No Cargo command was started by this lane. The subsequent test no-run result must be attributed only to the parent-controlled serialized gate.

