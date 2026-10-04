# Puzzle5d Current JSON Caller Audit

Read-only source audit on 2026-10-02; no Cargo or runtime test executed. Physical paths were checked with `rg --files`. The prerequisite receipt describes 91 edited files, 613 authority replacements, and 396 policy additions; this audit independently scanned 381 physical Rust files in the owning Puzzle5d tree.

## Actionable Findings

- `🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:1090` contains invalid `.parse(, semio_framework_pack_json::JsonMemberPolicy::Reject)`. This is an ordinary numeric method call, accidentally captured by the imported JSON parse rewrite. Restore ordinary `.parse()`.
- Removed JSON macro imports leave bare `json!` unresolved in four production files: `✏️editor/🎭️modes/✏️edit/🦀️.rs:65–68`; its `☑️options/🖌️brush/🦀️.rs:34`; its `🪟️windows/🧊️3d/☑️options/🎯️select/🦀️.rs:43,51,59`; and its `🪟️windows/🧊️3d/🪛️utilities/🔄️transform/🦀️.rs:66,74`. These files have neither macro imports nor wildcard imports supplying the macro. Qualify the canonical macro or restore the appropriate explicit import.
- Initial readback found `🤖️generated/🖐️puzzle5d-default/🦀️.rs:158` still using `semio_framework_os_kernel::json::from_json_str(PUZZLE5DDEFAULT_MANIFEST_JSON)`. This is the removed kernel authority and a single-argument reader. It was reported immediately; the later scan no longer found that kernel authority, indicating concurrent parent correction. Regenerate through its owning generator and verify readback.

Two test files also have bare macro calls without local imports, but use `super::*`; their actual parent scope must supply the macro: `✏️editor/🧪️tests/🧪️transform-tool/🦀️.rs:38` and `✏️editor/📌️panels/🛍️catalogue/🧪️tests/🔬️unit/🦀️.rs:38,40`. These are candidates requiring scope verification, not independently established missing imports.

## Confirmed Source Evidence

The comment-stripped DSL/Store/Protocol facade scan found zero executable references. Remaining textual DSL JSON mentions are documentation. Qualified canonical reader calls use explicit policy. The only imported canonical reader name is `parse` in the editor owner and apply-board-events owner; their actual JSON calls use explicit Reject, with the numeric-method corruption above being the exception. No renamed canonical reader aliases, imported `from_json_str`, or imported `parse_bytes` were found.

Engagement-submit correctly has three explicitly qualified canonical `json!` calls for move, rotate, and scale. Its prerequisite prose saying two is a receipt typo. The direct `semio-framework-pack-json` dependency exists in the owning Rust Cargo manifest at line 23, and its relative path resolves to the actual framework Pack JSON package directory.

The reader scan was a lightweight balanced-delimiter source check, not Rust compilation. A Python TOML-library availability error interrupted the initial dependency check after its source findings were printed; the dependency path was then independently extracted and successfully checked. Compilation and native SQLite acceptance remain the parent's sole Cargo lane.
