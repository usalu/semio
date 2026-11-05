# Lowpoly Requirement-to-Evidence Audit

Read-only audit on 2026-11-04. Only this report was created. No build, browser, infrastructure change or source modification was performed. Reviewed status.md, active-runs.md, runtime.md, mounted.md, verification.md, audit.md, boot-audit.md, ticket metadata, lowpoly-browser.mjs and the permanent composition Rust script.

## Acceptance Scope

The reopened ticket requests Lowpoly working end to end and complete workflows. The current manifest inventory establishes the concrete workflow surface below. It does not require speculative architecture changes.

| Requirement | Current evidence | Remaining acceptance |
|---|---|---|
| Boot, editor and read-only viewer | Activation identity correction; descriptor codec roundtrip; authored composition surface regression | Actual native composition test; generated catalog; current editor and viewer boot with rendered geometry and no runtime errors |
| Selection and modeling | Managed duplicate and mounted edit contracts; engine reconstruction assertions | Native suites; browser create, select, transform, duplicate, edit and delete with observable geometry and isolation of unselected objects |
| Paint and UV | Existing paint undo/redo assertions reviewed; metadata exposes Paint and UV layout | Native paint tests; actual brush/fill, layer and UV interaction with visible effects |
| Undo and redo | Neutral duplicate/edit contracts and exact managed snapshot assertions authored | Native execution and visible browser reversal/reapplication of geometry and paint |
| Persistence | Pack/JSON reload contracts with fresh scratch and independent serde_json rewrite | Native execution; browser edit then reload using retained profile and verify geometry/paint survived |
| Import and export | Managed geometry exporters, scene hierarchy/reflection handling; independent Three.js expectations; OBJ/PLY/STL roundtrip test updates | Execute Rust IO/media tests; browser import/export through host picker/download surface and verify resulting geometry |
| English and German | Localized metadata exists; verification.md identifies remaining English literal titles/status/errors | Run both locales and assess the identified user-visible literals; existing label presence is insufficient |
| Cross-platform and tooling | Repo-owned targets and contained probe paths exist; static path budget and syntax checks pass | Successful current host execution; Windows/Linux/macOS portability remains unproved. audit.md flags Metal-specific matrix flags for inspection before reusing that harness |

No current native compiler/test body or live browser journey has run. Independent oracle and descriptor checks validate expectations/encoding only. They establish no Rust execution, UI rendering or persistence result. Historical ticket summary and root-level boot artifacts describe an earlier repository state; current status explicitly supersedes their success claim.

## Concrete Validation Gaps

1. The retained lowpoly-browser.mjs is a boot/DOM observation probe only. It contains no create, pick, edit, paint, UV, undo, redo, import/export or reload actions. Even a successful run cannot close complete-workflow acceptance. After the first live DOM observation, extend it with supported accessible interactions and explicit before/after expectations, then execute those journeys.
2. Its success expression checks canvas existence and absence of console/page errors, not rendered model content. It also records failedRequests and failedResponses without making them fail success. A blank canvas or failed plugin/resource request can therefore coexist with success. Inspect the screenshot and semantic model state, and classify recorded resource failures before treating boot as accepted.
3. The status resume list focuses on editor creation/edit/paint/reload. Preserve viewer, UV and import/export acceptance explicitly; none is presently proven by that list or probe. Existing media native roundtrips do not prove host file picker/download wiring.
4. test-runtime now includes mut-objects in addition to composition, editor, engine and IO. That fixes the known ownership regression scope omission. This source audit found no further concrete package omission requiring a production change.

## Next Meaningful Action

Wait for the existing shared dependency owner to repair and publish a ready generation. Then run the canonical integrated native target and catalog prerequisite chain. Launch the declared Lowpoly React prototype only after catalog generation. Run the retained probe to discover actual DOM, inspect rendered geometry, then implement and execute the explicit workflow assertions above. Do not start duplicate preparation or bypass the Source fence. Keep the ticket open until runtime evidence exists.
