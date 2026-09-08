# Rust Import Scope Review

Native512 reported ten unused self-crate aliases in extracted artifacts. Source scans found no uses in the CAD artifact. The nine other artifacts referenced their package names only from standalone fixture adapters, which import the artifact as an external dependency of the generated test host. These adapters do not require a self-crate alias inside the artifact library. Removed the ten redundant aliases. The fresh all-target native check still needs to verify the changed source.

Files:
- `✏️s/🔌️plugins/🌿️vcs/🗿️artifacts/🌿️vcs/🦀️.rs`
- `✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/🦀️.rs`
- `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🦀️.rs`
- `✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🦀️.rs`
- `✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/🦀️.rs`
- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🦀️.rs`
- `✏️s/🔌️plugins/📜️imperative/🗿️artifacts/📜️procedure/🦀️.rs`
- `✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🦀️.rs`
- `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🦀️.rs`
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🦀️.rs`

The older renderer dead-code inventory was reviewed without changing its result-retention or acknowledgement protocols. Several methods reported by native491 already have production callers in the current source, so that inventory cannot justify deleting them. The render-snapshot publication revision remains a separate unresolved diagnostic pending the fresh renderer check.

Stdio Semio's older unused demo helpers are test-only and consumed by conversion-gated conformance tests. They require validation in the current feature matrix before changing their visibility or conditions. No demo fixtures or conformance laws were removed in this review.


Removed the seven result_large_err allowances and explanatory blocks that belonged to the deleted self-crate aliases. Those attributes had no effect on command handlers; retaining them would attach them to unrelated imports. Equation's separate attribute on its CAS module is outside this change. Native522 was already running and may have read the earlier snapshot; subsequent Clippy verification remains required.


Pass528 relocated only machine-applicable unnecessary-qualification suggestions from native512 whose entire diagnostic source block still appears exactly once. It recomputed UTF-8 positions and verified the original highlighted bytes at the relocated location, then checked each compiler suggestion in the same source block. It prepared 91 candidate edits across 9 files and skipped 111 unavailable or ambiguous diagnostics. No source was changed by this audit; saved plans retain full file hashes for a later guarded application.


Pass529 applied 91 relocated compiler suggestions across 9 artifact roots after checking whole-file hashes and exact original bytes. Skipped files changed since the audit: 0. These remove redundant crate-qualified paths; no imports, declarations or runtime behavior were replaced. Native522 has not yet completed, and clean native/WASI/browser links remain outstanding.
