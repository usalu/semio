# Source Direction Integration Audit

Read-only implementation audit of the current TypeScript owner-removability and Rust compile-reference increment. Reviewed coordination plan, TypeScript enforcement report, shared scanner, source-edge matcher, execution inventory, portable tests, Nx/package/launch wiring. No implementation or Git modifications. Audit scratch files were created only under ticket generated output and removed.

## Findings Reported to Root

1. Rust compile path references lost inline module context. Resolving every path against source dirname misidentifies inline-module #[path] targets. Root is implementing a contextual fix.
2. The scanner accepted arbitrary nested attribute arguments named path, including #[serde(path = "../../✏️s/data.rs")], as compile dependencies. Bun execution confirmed a spurious kind:path reference. Root acknowledged and is fixing.
3. concat!(env!("CARGO_MANIFEST_DIR"), "/../../✏️s/data.json") inside include_str! silently produced no reference. Bun confirmed. Root is adding manifest-relative support. Other unsupported expression macros remain a scope limitation unless rejected explicitly.
4. Unused macro_rules templates produced compile-reference edges although rustc does not expand them. Bun confirmed scanner output; root is evaluating template handling.
5. Rust edge matcher ignores rule from.pathNot and to.pathNot. Currently selected semantic rules appear to have no exclusions, so this is future-policy drift risk rather than a demonstrated current live violation.
6. TypeScript rootSources uses Dirent.isFile filtering before inventory; a root-level .ts/.js symlink is silently absent, while directory inventory rejects source links. A linked root executable can therefore escape source completeness inventory.

## rustc Path Semantics Proof

Executed rustc --crate-type lib --emit=dep-info=dependencies.d source.rs against isolated ticket files. All successful cases exited zero with no stderr. These are direct physical target proofs, not inferred path normalization.

| Mount | Nested declaration | Dep-info physical target |
| --- | --- | --- |
| crate root source.rs | mod nested { #[path="leaf.rs"] mod item; } | nested/leaf.rs |
| source.rs: mod sibling; conventional sibling.rs | mod nested { #[path="leaf.rs"] mod item; } | sibling/nested/leaf.rs |
| source.rs: #[path="sibling.rs"] mod sibling; | same nested declaration in sibling.rs | nested/leaf.rs |
| crate source.rs | #[path="alternate"] mod nested { #[path="leaf.rs"] mod item; } | alternate/leaf.rs |
| conventional sibling.rs via mod sibling; | same overridden inline declaration | alternate/leaf.rs |

The last case initially supplied sibling/alternate/leaf.rs and rustc failed explicitly looking for alternate/leaf.rs; retry supplying alternate/leaf.rs succeeded and dep-info included source.rs sibling.rs alternate/leaf.rs. Shared inspectRustModuleGraph currently joins inline pathTarget to context.moduleBase, which retains sibling/ and disagrees with this proof. Do not reuse this override branch without correcting its semantics. My preliminary message incorrectly included the crate source basename in crate-root inline resolution; the above rustc results correct that assumption.

## Integration Assessment

Nx targets invoke canonical 📜️script.ts commands and disable cache for live source scans. Matching package scripts invoke Nx and launch entries exist for TS test/lint and Rust source test/lint. TypeScript verification filters present taxonomy areas, while configured workspace ownership skips absent directories and still validates damaged present manifests. This matches removability intent. I did not rerun the expensive real graph; the author report explicitly records its failed unresolved dependency and must remain the final limitation until rerun. Rust fixture rustc oracle exercises physical includes and root-level path mounts but lacked the inline/context cases above at audit time. No broad architecture completion claim is justified by those root-only vectors.
