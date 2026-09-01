# Scalar Source-Oracle Provenance and Three-Cohort Proposal

Status: ticket-only proposal; no canonical, runtime, API, price, caller, launch, or frozen R204 JSON edit.

## Exact Paired Pointer Delta

Relative to R208, change the proposed values at the four existing top-level status pointers to `runtime-unmounted-source-declaration` (declaration/data schema) and `runtime-unmounted-source-oracle` (fixture/data schema). These refer to runtime adoption, not physical absence of canonical files.

Add four pointer replacements: declaration `/registeredTestProposal/status` and paired schema `/properties/registeredTestProposal/const/status` become `source-oracle-contract`; declaration `/registeredTestProposal/launch` and paired schema `/properties/registeredTestProposal/const/launch` become the normative launch contract below. The other seven ID/required pointers are unchanged. There are 15 distinct pointers relative to frozen R204, not 19.

```json
[
  {
    "file": 0,
    "op": "replace",
    "path": "/$id",
    "value": "semio.ui.retained.resident.scalar.declaration.v1"
  },
  {
    "file": 0,
    "op": "replace",
    "path": "/status",
    "value": "runtime-unmounted-source-declaration"
  },
  {
    "file": 1,
    "op": "replace",
    "path": "/$id",
    "value": "semio.ui.retained.resident.scalar.declaration.schema.v1"
  },
  {
    "file": 1,
    "op": "replace",
    "path": "/properties/$id/const",
    "value": "semio.ui.retained.resident.scalar.declaration.v1"
  },
  {
    "file": 1,
    "op": "replace",
    "path": "/properties/status/const",
    "value": "runtime-unmounted-source-declaration"
  },
  {
    "file": 2,
    "op": "add",
    "path": "/$id",
    "value": "semio.ui.retained.resident.scalar.tests.v1"
  },
  {
    "file": 2,
    "op": "replace",
    "path": "/status",
    "value": "runtime-unmounted-source-oracle"
  },
  {
    "file": 3,
    "op": "replace",
    "path": "/$id",
    "value": "semio.ui.retained.resident.scalar.tests.schema.v1"
  },
  {
    "file": 3,
    "op": "add",
    "path": "/required/10",
    "value": "$id"
  },
  {
    "file": 3,
    "op": "add",
    "path": "/properties/$id",
    "value": {
      "const": "semio.ui.retained.resident.scalar.tests.v1"
    }
  },
  {
    "file": 3,
    "op": "replace",
    "path": "/properties/status/const",
    "value": "runtime-unmounted-source-oracle"
  },
  {
    "file": 0,
    "op": "replace",
    "path": "/registeredTestProposal/status",
    "value": "source-oracle-contract"
  },
  {
    "file": 1,
    "op": "replace",
    "path": "/properties/registeredTestProposal/const/status",
    "value": "source-oracle-contract"
  },
  {
    "file": 0,
    "op": "replace",
    "path": "/registeredTestProposal/launch",
    "value": "Canonical source-oracle release requires the taxonomy-owned seed and generated launch row for the existing Nx target with -t OwnedResidentScalar; no output-only launch row or separate task. Executed registration evidence belongs in the ticket, not this normative declaration."
  },
  {
    "file": 1,
    "op": "replace",
    "path": "/properties/registeredTestProposal/const/launch",
    "value": "Canonical source-oracle release requires the taxonomy-owned seed and generated launch row for the existing Nx target with -t OwnedResidentScalar; no output-only launch row or separate task. Executed registration evidence belongs in the ticket, not this normative declaration."
  }
]
```

The norm does not assert a launch row currently exists or is absent. Actual source registration, launch generation, test execution and runtime readiness remain independently evidenced in the ticket. Existing taxonomy instruction is already normative (“Before canonical leaf creation…”), not a present-state assertion; it is unchanged.

## Concrete Canonical Source Test Proposal

Only after taxonomy vocabulary/path release and root approval, use the four kind-only paths in R208. Add four static JSON imports at existing `UiDocumentStore/🟦️component.tsx` test-only `TypedWire` region; the existing Actor neutral u64 schema and reader/page fixtures remain the original canonical imports. Do not import the ticket or evaluate serialized controller source.

Handcraft these three actual `test` bodies under the existing long-tier renderer route:

1. `OwnedResidentScalarDeclaration`: `validateScalarDeclarationCold` receives the four imported declarations; strict Ajv validates both pairs and rejects cross-pairs, wrong IDs/statuses/unknown fields. Enforce closed profile set, unique semantic/vector/transition IDs, accepted↔expected consistency, exact field/closure/witness census and proposed charges. Run all 43 original scalar vectors with the existing independent Buffer/DataView/TextDecoder/BigInt oracles; reject contradictory semantic fixtures. This is scalar arithmetic/admission policy, not a live byte-reader claim.
2. `OwnedResidentScalarReceiptModel`: `runScalarReceiptModelCold` executes the frozen closed receipt/cursor/latch/serial/settle/fault/EOF-versus-backpressure traces with explicit immutable original identities and Immer parity. Preserve the 91 traces/2238 rows and 65B preflight/child read-or-parse versus separate 64B cursor/observation phases. It must not instantiate a fake positive production consumer or call a proposed runtime API.
3. `OwnedResidentScalarCloseModel`: `runScalarCloseModelCold` incorporates BOTH former arithmetic and exact-alias controllers, not just their total. Check every full-close and child-prefix sum, 14 construction prefixes/121 transitions, both original state aliases, genuine body/cell/record/witness ordering, refusal while either alias remains, short grants, before/after faults, replay and omitted-unlink negatives. Keep 40 child suffixes, 7 maintenance and 16 reader-order cases. These are desired source-oracle laws, not actual intrinsic refund tests.

Helpers remain local test-only functions in the same authored test region; use subregions for declaration/receipt/close. No production export, new script, dynamic library interface, external runtime dependency or runtime registration. Extract semantic logic from the four already independently replayed R204 controllers by hand; do not transplant ticket paths, embedded base64/eval, transient filesystem imports, or duplicated script dispatch.

Use the existing `@semio-tech/framework-renderer-react:test-long` router/project, existing budget and exact `-t OwnedResidentScalar` launch selector. Require actual enumeration of all THREE selected tests; Nx exit0/passWithNoTests alone is not success. The first mount gate must also compare the canonical copies against frozen originals through ONLY these 15 pointer differences, and verify that each inverse restores the complete frozen value. Root/taxonomy owns the seed/generated-launch join. No mount occurs in this packet.

## Actual In-Memory Validation

R209 orchestration error: used singular `--project` with `nx exec`, which repeated the read-only preview across projects. Stopped only its own exec session with Ctrl-C; exit130. Output was truncated, so no complete transcript/census or scoped success is claimed. No source/write action was in the preview. This is not a semantic RED.

R210 corrected `--projects=@semio-tech/framework-renderer-react`, using the existing environment flags. Actual start 2026-08-28T04:59:54.215Z; terminal exit 0. Printed: pairedSchemas4, pointerChanges15, crossPairRejections4, inverseMatches4, originalVectors43, four frozen hashes unchanged. No canonical written or runtime executed. This validates paired provenance only; it does not rerun all R204 semantic controllers.

Raw output: `🧪️renderer-scalar-source-oracle-provenance-r210-2026-08-28.txt`.
Executable preview/controller and exact command: `🧪️renderer-scalar-source-oracle-provenance-r210-2026-08-28.json`.

