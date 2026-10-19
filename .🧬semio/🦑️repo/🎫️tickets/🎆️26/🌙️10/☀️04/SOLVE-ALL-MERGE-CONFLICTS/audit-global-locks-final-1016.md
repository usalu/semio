# Independent Final Global Lock Audit — 2026-10-16

Read-only audit of the Final Source Manifest Refresh section and `🗑️generated/root-final-lock-reconciliation.json`, followed by independent TOML/tree-record inspection. No Cargo resolution, compiler, build, tests, runtime or source/Git mutations. Only this report was written; no generated output retained.

## Lock Record Preservation

All three current shared locks parse. Compared every immutable HEAD package record using `(name, version, source)` identity, with explicit `semio-framework-quiz` → `semio-framework-course` identity mapping. No HEAD record is missing, and no non-name/non-dependency record field changed (including checksum/source/version).

| Lock | HEAD records | Current records | Missing HEAD identities | Unexpected record-field changes |
| --- | ---: | ---: | ---: | ---: |
| root | 1426 | 1426 | 0 | 0 |
| framework | 922 | 924 | 0 | 0 |
| hub | 2558 | 2564 | 0 | 0 |

This establishes preservation of resolved records, not registry resolution of newly authored source.

## Late Authored Edges

Independently checked dependency arrays wherever the parent package record exists:

| Parent → dependency | Root lock | Framework lock | Hub lock |
| --- | --- | --- | --- |
| store-replay → store-snapshot-clone | present | no parent record | present |
| os-inference → value-snapshot-read | present | no parent record | present |
| plugin-decl-app → actor | present | no parent record | present |
| renderer-wgpu-host-plumbing → os-services | present | no parent record | no parent record |
| renderer-wgpu-kernel-protocol → host-plumbing and job-worker | both present | no parent record | no parent record |
| renderer-wgpu-asset-decode → host-plumbing and plugin-ownership | both present | no parent record | no parent record |
| os-infinite-board-normal-fill → action-bus | present | no parent record | present |

Names in this table use the common `semio-framework-` prefix. Absence of a parent record is outside that shared lock's existing package inventory, not a missing edge failure.

## Qualified Authored Gaps

The reconciliation JSON records 82 source-changed package intents and no unresolved edge in root/framework locks; hub records six authored dependencies involving three names without resolved records. Independent inspection confirms all three are absent from all three current shared locks:

- `semio-s-artifact-fem-2d-results-state`
- `semio-s-artifact-raster-raster-editor`
- `semio-s-artifact-raster-raster-viewer`

These are preserved authored source names without existing resolved records, not evidence that HEAD records were removed. Resolving or repairing them is outside merge-preservation scope. The parent report's broader existing-lock census is not independently repeated here.

Tracked changed manifest TOML parsing independently covered 80 package manifests. Additional untracked manifests and renamed course owners mean that this tracked-file count is not the same selection as the parent's 82 source package intents. The stated 176 add-edge intents were not independently recounted; focused late-edge closure and all-HEAD-record preservation above were independently verified from actual lock bytes.

## Result

No actionable missing late edge was found where an existing parent record is present; all HEAD resolved records remain under the explicit Course identity rename. No compile/runtime success is claimed, and the three authored no-record names remain explicitly qualified.
