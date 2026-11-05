# IDML Browser Contract Red

The remaining279pass1fail958assert failure is a genuine staged availability gap, not a stale unit expectation. The exact assertion at acceptance/contract line183 expects ownedApps.get(stdio-office:s.stdio.idml@8.0/*#editor) to equal editor and receives undefined. ownedApps is built from current family component descriptors.

Filesystem recheck: hub/compositions/stdio/extensions/office/component/JSON has18apps and zero IDML apps. Authoritative source office/🦀.rs declares typed IdmlEditor/IdmlViewer variants at32–33, hosts the IDML artifact at43, registers actual editor/viewer builders at62–63 and activation at67. The component Cargo playground metadata pins s.stdio.idml@8.0/*#editor at81. The acceptance fixture owner therefore agrees with authored source; changing it would weaken required38coverage.

Correct repair remains authoritative Office build/describe and publication/staging receipt validation, preserving exact editor and paired viewer IDs/schema. No fixture update, generated JSON handedit, or assertion exception is justified. Existing report missing-role-source-and-staging.md documents the same boundary. No product/test source changed and no test or native job was launched in this recheck.
