# Independent OS Plugin Lower Preservation Audit — 2026-10-16

Finite read-only audit of `preserve-os-plugin-lower-stash-1016.md`. Immutable base `f856dff081373e131ebf72618120cb73e303a912`, HEAD `1bae6115bc56bf5e1b01212774dd76877a5f64fe`, latest stash `ec7b3361858df86599da79e5b6dde70ccbbc3625`. No source/Git/index/stash mutation, build, compiler, tests or runtime execution. Only this report was written; no generated output retained.

## Independent Evidence

Read all 60 explicit preservation table rows. Byte comparison confirms all 49 labeled exact-stash files match immutable stash bytes, including fixture newline bytes. The 10 integrated owners and one mapped context-fields source were separately inspected through base-to-stash additions, HEAD-to-working diffs and current canonical source links. No unexplained stash content loss found in this finite comparison.

Confirmed canonical app lifted context contains both Cursor demand aggregation and RetirementCursor `next_demand`; the old context source stays a reexport. Canonical lifted fields retain stash fields, retirement derives and source-graph custody. Differences from stash are HEAD lift documentation, crate import routes and public visibility needed across split crates; no field/body deletion was found.

Confirmed segmented media owner now calls authored `original_birth_demands`/`construct_original` and mounts `🏭️construction/🦀️.rs`; its implementation/tests/fixtures are stash-exact. Confirmed full-operation owner includes `🏭️producer/🦀️.rs`, which is stash-exact. Stash full-operation factory methods reside at HEAD `👷️job/🛬️lifted/🔄️full-operation/🦀️.rs`, including `admission_demands` and `admit_job`, rather than duplicating the factory inside old retained-command ownership.

Confirmed views expose ArtifactPendingView construction, ToolRunPollView and borrowed presence/interaction constructors; `with_render_context` borrows the parent document string. App and surface import the views-owned pending/preparation types, and surface forwarding retains new producer/preparation/pending signatures. Changed signatures correspond to authored stash changes, with HEAD crate boundaries retained.

Confirmed five-field original retained-grant minimum intersection exists at lower `🏛️ownership/🦀️.rs`, and view preparation calls `semio_framework_plugin_ownership::bounded_ownership::original_plugin_turn_grant`; no upward reactor dependency was added by that route. App Cargo manifest includes workspace actor dependency for typed diagnostic publication. These are source-link checks, not compiler validation.

Independent bounded census matches the report: 161 files in declaration/retained-command owners, 57 JSON and six TOML files parsed, zero strict conflict markers. Scoped `git diff --check HEAD -- <two owners>` returned zero (only ordinary manifest CRLF normalization warning).

## Result and Boundary

No actionable source-preservation loss or merge-introduced structural defect found in the completed lower-owner checkpoint. This does not establish compile/runtime correctness or whole-OS preservation. Store/reactor/instance remain active and were not judged final. No completed store checkpoint report was present among current OS stash reports, so store closure was deferred to its executor checkpoint rather than auditing unsettled bytes as final.
