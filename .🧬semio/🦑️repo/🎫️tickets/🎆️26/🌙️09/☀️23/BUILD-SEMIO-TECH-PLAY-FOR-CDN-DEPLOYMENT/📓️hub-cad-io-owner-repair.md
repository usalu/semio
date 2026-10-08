# Hub Cad IO Owner Repair

## Confirmed Component Failure

The actual cold-generation Cad component-dev retry failed after 24m17s in `🗑️generated/cad-repair-component-dev.log`. The earlier Cad artifact compiler roots were resolved; the remaining reported E0433 is Hub Cad's mesh handler registration at line 31, using the removed root `crate::artifacts::cad::io` path. Rustc identifies the existing public canonical owner as `crate::artifacts::cad::standards::v1::subsets::any::io`.

## Canonical Consumer Repair

Only that function-pointer path was changed, retaining the current canonical `cad_document_from_mesh` implementation and existing handler identity, artifact kind and schema. No alias, root facade, compatibility reexport or new dependency was created. The canonical function already exists and returns the framework-owned first-party JSON boundary type; its implementation and payload behavior are unchanged.

The applicable root AGENTS instructions were already read; a recursive `rg --files 🌎️hub -g AGENTS.md` inventory found no Hub-specific descendant instructions. A recursive scan of selected Hub Cad Rust sources for obsolete Cad root `io`, `native`, `schema`, `dialect`, `snapshot` and `dsl` consumer paths found only this confirmed live IO call. Existing Hub assembly tests call `plugin()` and assert the assembled editor/viewer fleet. This is a compile-only consumer path correction, with no new payload schema or semantic implementation.

## Verification Boundary

The real E0433 compile above is the red baseline. The build agent's same private `release-6Z7Lxu` shipping component retry finished actual exit 0 after 22m46s, with caches bypassed and completed prerequisites excluded. `🗑️generated/cad-hub-io-component-dev.log` retains that component success, verifying the current consumer path. This is provisional component proof: the editor's temporary diagnostic instrumentation must be removed and the final full graph must rebuild its resulting clean sources before publication readiness is claimed. No partial final publication tree was audited or deployed.
