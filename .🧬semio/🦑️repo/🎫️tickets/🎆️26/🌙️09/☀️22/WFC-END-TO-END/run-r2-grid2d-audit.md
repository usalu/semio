# Grid2d Revision Cache Audit

Read-only review2026-11-04. Only this report authored. Reviewed current fill, preview, root adapter and preview tests.

## Source Result

No remaining stale-arity caller was found in grid2d source for commit_effect, cached_commit, paint_commit, live_fill_payload, scene or editor preview render. Viewer has its own separate preview module and unchanged one-argument API; its callers are not stale editor callers.

Root editor preview adapter now passes doc.render_operation_ref().map(|operation|operation.canonical_base_revision), the correct render authority. CachedSolve.source_revision originates from self.identity.base_revision at fill finish, so delayed delivery cannot falsely stamp current document identity. Cached and live projections require an exact matching32byte current revision; absent,31byte,33byte source revisions are refused.

Shared fixture include_str path physically resolves to plugin 🧪️tests/🧫️preview-revisions/🔣️.json. Native preview tests cover six cache rows and three live rows (live identity itself requires32bytes). Existing cached render tests now wrap native inference commits in CachedSolve. No obvious nested derive mismatch found: CachedSolve and canonical Grid2dInferenceCommit both supply native ToValue/FromValue, camelCase wrapper fields and canonical tuple-array inference assignments.

New cache schema fill/🧬️cache/🔣️.json resolves its absolute inference reference to schema/💡️inferences/🔣️.json. It requires exactly32 byte-valued integers and a complete canonical commit, with unknown properties refused. This is source review plus reference validation, not native FromValue execution.

## Independent Validation Executed

repo tools exec --light bun -- -e used installed Ajv with canonical inference schema registered by its id, then compiled new cache schema. All six authored revision rows matched independent visibility expectations; four malformed nested commits were refused: short assignment tuple, numeric tile identifier, missing entropy, and unknown commit field. Exit0: Ajv canonical inference+cache:6 revision cases and4 malformed nested commits pass.

An earlier Python jsonschema probe failed because jsonschema is not installed. No dependency installed; existing Bun/Ajv was used successfully.

## Remaining Native Evidence

Native tests still need to execute actual CachedSolve FromValue/JSON round trips and mounted render authority. Pure helper tests cannot independently detect a future accidental command-operation accessor in the root adapter. Existing bitmap mounted render regression is the right pattern. Completion-to-preview window routing remains a separate browser observation; the revision change does not alter that addressed config ownership.

No source edit, build or native queue was launched by this audit.

## Mounted Adapter Regression And Shared Inputs Recheck

Reviewed root's mounted_preview_uses_render_authority_for_live_source_binding regression in grid2d editor/🧪️tests/🔬️unit/🦀️.rs. Ownership/signatures match current SDK: ArtifactView::with_render_context is async and takes render context by value; AppRenderOperationContext derives Copy (framework declaration/view/operation-contexts30-38), so reuse with struct update is valid. ArtifactView::with_tool_run consumes self (artifact-view114), and the test borrows doc for empty first then consumes it exactly once for fresh. Changed view is separately created. Public ToolRunView::new owns its payload; make_run borrows payload only to encode an independent owned Vec each invocation. No move-after-consume or shared payload lifetime mismatch found.

The test's fresh tree differs from empty and stale tree equals empty. Its explicit operation_optional().is_none() makes a mistaken command-authority accessor observable. This closes the earlier static-only root-adapter coverage concern; actual native execution remains pending.

All five Rust project manifests under artifact/📦️packages/🦀️rust/📋️project.json include {workspaceRoot}/✏️s/🔌️plugins/🀄️wfc/🧪️tests/**/* in namedInputs.nativeSources. Thus shared preview-revisions fixture is within the explicit shared native source set for bitmap,grid2d,wfc2d,grid3d,wfc3d. Grid2d nativeSources additionally includes its artifact JSON glob, covering the new cache schema. No fixture-input amendment identified.

## Restart Payload Invariant And Envelope Audit

Grid2d fill declares ToolRunRebasePolicy::Restart and ToolRunReconfigurePolicy::Restart. Exact framework path:
🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⏯️tool-run/🧵️instance/👀️generation-watch-and-turn-driver/🦀️.rs60-75 closes current job and calls discard_provisional BEFORE changing entry.identity.base_revision. The called implementation in ⏯️tool-run/📒️driver-and-ledger/🦀️.rs438-464 delegates release_provisional(false), requests native payload closure and clears entry.payload. Therefore the existing grid2d live fence against ToolRunView.identity.base_revision is valid for its declared Restart policy. No legal path preserving the prior payload under new identity was established. NonRestart refold/rebind retains payload, but grid2d does not select it. No additional payload source field is needed for this policy.

The command envelope is a separate concrete scale issue. GRID2D_RETAINED_RAW_BYTES is65536, factory uses it, while bounded proof also embeds literal65536; output is262144. A JSON sample of4096 tuple assignments for64x64 with tile ID tile yields60306byte cache but68524byte escaped solveJson argument envelope. Thus a reasonable control instance may solve successfully but lose completed-cache publication at command admission. Grid3d owner independently selected1MiB raw/output against existing262KiB tick ceiling; root was advised to align grid2d raw constant and proof literal with that bounded transport decision. Runtime receipt laws remain necessary; increasing bounds is not proof of physical incremental copying.

## Portable Command Budget Vector

Independent Bun/Ajv execution registered the canonical inference schema and validated the complete generated cache against the new cache schema. Exit0; exact UTF8 counts confirmed60306cache/68524argument bytes.

Proposed language-neutral fixture (compact generator specification; no copied4096row source):

```json
{
  "id": "grid2d-64x64-completion-envelope",
  "width": 64,
  "height": 64,
  "tileId": "tile",
  "order": "row-major",
  "sourceRevision": [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
  "commit": {"contradiction": false, "entropy": []},
  "cacheFieldOrder": ["sourceRevision","commit"],
  "commitFieldOrder": ["assignments","contradiction","entropy"],
  "argumentKey": "solveJson",
  "cells": 4096,
  "cacheUtf8Bytes": 60306,
  "argumentUtf8Bytes": 68524,
  "oldRawBytes": 65536,
  "rawBytes": 1048576,
  "oldRawAdmits": false,
  "rawAdmits": true
}
```

For index0..4095 generate tuple[index%width,floor(index/width),tileId]. Serialize compact JSON with the stated field order, then serialize {solveJson:cacheString}. This preserves portable exact-byte assertions across native and independent JSON libraries. The argument count includes JSON string escaping; counting only the inner cache misses the original refusal. These are public argument-envelope counts, not proof of complete native command-wire framing or memory receipts.
