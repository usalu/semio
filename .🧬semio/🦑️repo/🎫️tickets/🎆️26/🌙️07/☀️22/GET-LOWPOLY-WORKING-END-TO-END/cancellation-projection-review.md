# Cancellation Projection Review

Read-only independent review, 2026-11-05. No source changes, Native, server, or browser execution. No production correctness defect found in the narrow presentation projection. Two useful missing native regression cases were sent to Verify: wrong controller and registryless preservation.

## Shared projection and authority boundary

Owner: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📜️declaration/🌳️tree/🦀️.rs:140`, project_tool_cancellation. It constructs the same AppActionRegistry representation used by mount validation. Registryless returns before any mutation. Expected IDs are exactly generated OpBinary TOOL_JOB_IDS intersected with migrated registry keys, excluding typed-command, matching validate_tool_job_rows's expected-set policy.

Rows are checked against concrete ToolOwnerWitness::of::<A>(), definition ID controller, A::DOCUMENT_SCHEMA, nonempty owner_file, expected ID membership, and uniqueness. Missing rows refuse. Every check precedes either public action/command mutation loop, so returned errors are atomic. Matched app actions, every window action, public commands, and mode commands receive only the contract cancellation projection; unrelated authored flags remain unchanged. PerOperation is the sole current enum variant and is matched exhaustively; no fabricated Never case or blanket migrated flag appears.

The helper does NOT validate concrete factory TypeId/name, registered contract equality, schema-bound bus admission, or create qualified proofs. These remain authoritative in `👷️job/🏛️reserved/🦀️.rs:901` validate_tool_job_rows. A registered read-only git diff for this file was empty. That validator still joins owner/controller/tool/schema/factory identity+contract+admission at runtime. The new presentation helper neither replaces nor weakens it. Its generated/migrated and owner-file checks match existing catalog rules; it does not add a new unrelated stricter rule.

stamp_app_contracts first stamps existing fault notices, then invokes projection and panics on invalid assembly. Direct projector errors preserve the whole definition; wrapper failure does not publish a definition but should not be described as rollback of prior fault-notice stamping.

## Nine assembly sites

All nine former typed fault-stamp sites now invoke stamp_app_contracts before sharing definitions:

- builder/🦀️.rs:363 typed app, :436 viewer, :505 editor.
- builder/🌱️surfaces/🦀️.rs:13 editor, :22 viewer.
- instance/🛬️lifted/🌳️tree/🦀️.rs:45 editor surface, :66 viewer surface, :90 typed editor definition, :96 typed viewer definition.

No direct stamp_fault_notices caller remains in these builder/lifted owners. This covers independent builder and declared-tree entry points.

## Tests and limits

Native owner `🧪️tests/🔬️plugin-runtime-plugin-builder-contract-part-6/🏭️tool-factory-keys-and-shared-actions/🦀️.rs:242` reads cancellation.json (ten cases) and mutates real AppDefinition/proof values. Cases cover app/window action, public command/mode command, unmatched false/true preservation, wrong owner, wrong artifact schema, duplicate proof, missing proof. Every refused case asserts complete serde definition equality with its pre-call value. Command construction uses the actual bounded_catalog constructor; builder mutation appends app-level actions, so test's action lookup is consistent with current API.

The accompanying strict schema has additionalProperties:false at document/case levels and explicit enums. Its current proof enum/test match does not include wrong-controller or registryless cases. Source checks those branches, but they remain uncovered by this corpus. Suggested additions: wrong row.controller_id, mismatched definition.controller_id, and a registryless definition carrying authored metadata with provided rows ignored. These are coverage additions rather than observed implementation failures.

Composition owner `🌎️hub/🧩️compositions/💠️lowpoly/🧪️tests/🔬️surface/🦀️.rs:30` builds the real plugin and checks all matching app/window copies of importMeshFile/exportMesh/paintFill/fillBucket, plus committed descriptor copies, against neutral cancellation.json. It requires nonempty matches, preventing vacuous pass. This is four selected integration tools, not a full generated-catalog equality test or browser cancellation proof. Root separately reports 51 committed descriptor rows / 48 exact tools and owning codec/oracle green; no Rust execution is claimed here.


## Authored Coverage Follow-Up

Verify reports fourteen cases now include wrong proof controller, wrong definition controller and registryless authored false/true preservation with explicit registryless assertion. Strict Ajv passes; no Native execution. Owning unit package is semio-framework-plugin-tests-11-2 and currently missing from Lowpoly runtime selection; see runtime-target-coverage.md.
