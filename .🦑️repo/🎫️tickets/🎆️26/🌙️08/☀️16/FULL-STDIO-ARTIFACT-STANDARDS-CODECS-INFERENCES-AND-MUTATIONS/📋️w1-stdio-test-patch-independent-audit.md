# Independent Stdio Mutation Test-Patch Audit

Audit time: 2026-08-16 15:27:22 +0200

Scope: final frozen non-glTF stdio mutation/test patch. This was read-only; no source, fixture, or configuration file was edited, and no Cargo, rustc, or nextest command was run.

## Verdict

PASS for the requested template, marker, path-replacement, rustfmt-readiness, and MutationOutcome test-adoption audit. The final frozen tree has no unresolved Rust template artifacts or path markers in the audited tree.

An earlier probe during the writer's active edit observed 29 transient `${1}` tokens in the Semio `any` test consumer. The final frozen rescan below is clean; those tokens are not present in the current tree.

## Evidence

- Exact `${...}` scan over non-glTF Rust: 0 matches, including `${1}` and `${name}`.
- Literal `++ <path>`: 0 matches.
- Rust lines beginning with `++`: 0 matches.
- Suspicious path-only replacement lines: 0 matches.
- Literal `<path>`: 1 match, a legitimate SVG exporter documentation comment (`drawing/.../🎨️svg/.../component.rs:2`), not a replacement marker.
- Small-file path audit: 24 Rust files at or below 200 bytes inspected; 0 path-only files.
- `git diff --check` over the stdio artifact tree: no output/errors.
- `rustfmt --edition 2021 --check` over all 19 affected files: 19/19 exit success, 0 failures. This is parser/format readiness only, not a compile or test result.
- Legacy bare `fn diff` return signatures in the affected files: 0. The 17 trait mutation leaves return `protocol::MutationOutcome`; Markdown and XML mutation methods return `MutationOutcome` in their `(snapshot, outcome)` result.
- Double-outcome `.diff().diff()` chains: 0.
- Test declarations: 178 at `HEAD`, 178 current across the exact 19 files.
- Rust declarations (`struct`, `enum`, `trait`, `type`, `fn`): 665 at `HEAD`, 665 current.
- Function-name loss: 0 production/test functions lost. One test was intentionally renamed from `kind_mismatch_wrapped_mutation_is_a_safe_no_op` to `kind_mismatch_wrapped_mutation_records_an_error_outcome`; its test count remains unchanged.

## Exact audited files

```text
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🌐️html/🏅️standards/🔖️5/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎥️mp4/🏅️standards/🔖️isobmff/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📝️md/🏅️standards/🔖️commonmark/🪆️subsets/✳️any/🧬️schema/🦀️component.rs
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📰️xml/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🧬️schema/🦀️component.rs
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📼️avi/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️animation/🧬️schema/🧬️mutations/🦀️component.rs
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️audio/🧬️schema/🧬️mutations/🦀️component.rs
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️document/🧬️schema/🧬️mutations/🦀️component.rs
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️flow/🧬️schema/🧬️mutations/🦀️component.rs
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️image/🧬️schema/🧬️mutations/🦀️component.rs
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️model/🧬️schema/🧬️mutations/🦀️component.rs
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️presentation/🧬️schema/🧬️mutations/🦀️component.rs
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️video/🧬️schema/🧬️mutations/🦀️component.rs
```

The audit intentionally does not claim compilation or runtime test success because Cargo execution was out of scope for this independent read-only pass.
