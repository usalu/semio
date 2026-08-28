# Run Direct 31 Independent Review

## Scope And Evidence

This is a read-only review of the current canonical Run five-leaf tree, its mounted aggregate, and
the workflow/run application seam. No production source or Cargo command was changed or run.

The language-neutral contract is
`🧪️run-direct-31-independent-review/🛂️schema.json` with its five exact descriptor/codec vectors
in `🧪️run-direct-31-independent-review/🔣️vectors.json`. Its third-party Ajv 2020 validation was
executed with Bun:

```text
bun .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️run-direct-31-independent-review/📜️script.ts
[DEBUG] Run direct neutral fixture accepted descriptors=5 codecs=5
```

The prepared public client is `🧪️run-direct-31-independent-review/🦀️client.rs`. It is intentionally
not executed yet: it must be linked by the root lane to one fresh `semio-framework`,
`semio-framework-os-kernel`, `semio-framework-async`, and `serde_json` artifact set. It does not
build the framework itself. It compares all fourteen serialized descriptor fields to the neutral
fixture, checks the five semantic tuples, requires text and binary round trips for tags `0..4`,
checks both `RunTrigger` JSON variants, tests the currently declared replacement inverse, and tests
the real checked admission seam for seal and duplicate-start rejection.

`rustfmt --emit stdout --edition 2021` parsed the client successfully. That is syntax-only evidence;
it does not link dependency artifacts or establish runtime behavior.

## Current Canonical Facts

`semio-framework` mounts `workflow/🦀️component.rs` and reexports its public surface. The mounted
`RunMutation` aggregate has the five direct payload variants in this exact order: `StartRun`,
`StartRunNode`, `FinishRunNode`, `AppendRunLog`, and `SealRun`; it derives `Mutations` and
`DslOps`. The component supplies the aggregate `OpText` and `OpBinary` adapters through
`DslVariants`, so no private conversion enum is present.

Each leaf descriptor has all fourteen required fields, fixed payload-schema locator
`🧬️schema/🔣️.json`, text opcode, binary tag, apply-only participation, applied outcome,
atomic composition, and the four Rust/JSON-schema/text/binary surfaces. The five payload schemas
are strict at every object level. Their primitive constraints match the current Rust structs; no
schema currently asserts a domain constraint that the Rust model itself does not assert.

The runner uses wrapped canonical variants. `RunSink::record` is intended to be the live mutation
path; the binary creates `StartRun`, runner records node start/finish, and both success and failure
branches append/seal using the canonical leaves.

## Findings Requiring Owner Resolution

1. **Aggregate JSON Schema is absent.** The canonical tree has five leaf payload schemas but no
   aggregate strict `operation` envelope/one-of schema. The frozen Run plan explicitly requires
   that aggregate contract, so leaf-level Ajv acceptance does not prove complete wire acceptance.

2. **Automation JSON casing needs runtime proof and is likely wrong.** `RunTrigger` uses
   `#[serde(tag = "kind", rename_all = "camelCase")]`, while the Automation struct-variant fields
   are `automation_ref` and `event_fingerprint`; the payload schema requires `automationRef` and
   `eventFingerprint`. Serde's `rename_all_fields` is absent. The public client asserts the schema
   spelling for both Manual and Automation and will record the actual behavior after the fresh
   framework artifact is available. This is a static concern, not a claimed runtime result.

3. **The checked live seam does not enforce the documented duplicate-start rule.**
   `RunDiff::apply` rejects `Start` for a non-pending/already-started run, but
   `apply_run_operation_checked` currently checks only `sealed`. `RunSink::record` routes through
   the latter. The independent client therefore requires a second checked `StartRun` to fail; it
   is expected to expose the discrepancy until the owner changes the checked boundary.

4. **`FinishRunNode` is only a replacement inverse.** With a prior record for the same node, its
   inverse correctly returns that prior `FinishRunNode`; with no prior record it returns no inverse.
   Therefore a first insertion cannot be undone even though the descriptor declares
   `explicit-mutation`. This can be valid only if the descriptor contract explicitly means
   replacement-only rather than total inverse behavior.

5. **Current source is not framework-client compilable.**
   `apply_run_operation_checked` is `async`, but `RunSink::record` assigns its returned future to
   `RunArtifact` without awaiting it. This is a concurrent Workflow18/run integration blocker,
   not a change from this review. It must be resolved before the public-client runtime probe can
   produce authoritative result evidence.

## Reviewed Source Fingerprints

| Source | SHA-256 |
| --- | --- |
| `workflow/🦀️component.rs` | `1a97e7e43ac8f117db4379b133488803d2f9e01aaf8fad260f359d7e095b8081` |
| `run/🧬️schema/🧬️mutations/🦀️.rs` | `77b59d87f1ebc293022002c07c7c1afe5cdd1d1ac4584cbc7cc13d79cb92667e` |
| `🚀️start-run/🦀️.rs` | `984cb2f6ffbea3347c2e183d2ffade48366911678e8d51cd59c7460f9b267d7b` |
| `▶️start-run-node/🦀️.rs` | `d888251682d6864fc0aeea37bf8c62d14769287b1dc9c0d02dc8e0d124d139c1` |
| `✅️finish-run-node/🦀️.rs` | `fec8f641ab79dc25650e092bf527a0bd484e5bc919b4ebe46b51563f0c1509e9` |
| `🪵️append-run-log/🦀️.rs` | `a502551838f6a93ada05c7ab6c8ac7fd8a694736a4b3b3024fe4ff37abcae3af` |
| `🔏️seal-run/🦀️.rs` | `60049ef2e739176d784f0159582f117b202190a00a443894ea182f8c3d283109` |

The review remains open pending owner repairs. The fresh independent public-client compile/run is
recorded below.

## Schema-Parity Packet

The packet adds `🧪️run-direct-31-independent-review/🛂️payload-vectors.schema.json` and
`🧪️run-direct-31-independent-review/🔣️payload-vectors.json`. It carries six valid leaf payloads
(both Manual and Automation `StartRun` forms) and nineteen invalid leaf payloads. The negatives
cover an unknown field at every distinct nested object shape: the Start root, parameter row, Manual
trigger, Automation trigger, StartNode root, Finish root, node record, input fingerprint, output
fingerprint, output row, Append root, and Seal root. They additionally cover required primitive
type and enum failures. Aggregate vectors cover all five operation tags plus unknown operation,
mismatched payload, and envelope unknown-field rejection.

The combined Bun/Ajv 2020 script was executed after the packet was added:

```text
[DEBUG] Run direct neutral fixture accepted descriptors=5 codecs=5
[DEBUG] Run payload parity accepted leafPositive=6 leafNegative=19
[DEBUG] Current flattened aggregate is missing and closed-leaf refs accepted=0/5
[DEBUG] Proposed shared-payload aggregate accepted=5 rejected=22
```

The fourth line is deliberate red/missing evidence expressed as a passing review assertion: no
current aggregate schema file exists, and an in-memory flattened `operation` envelope composed
with each current leaf's strict root `$ref` accepts none of the five valid operations. The leaf
root's `additionalProperties: false` rejects the required sibling `operation` field before a
branch-level `unevaluatedProperties: false` can close the composed object.

The minimal no-duplication long-term representation is to keep each leaf's complete payload body
once under `#/$defs/payload`, including its nested strictness. The leaf document wraps that
definition with `allOf: [{ "$ref": "#/$defs/payload" }]` and top-level
`unevaluatedProperties: false`. The aggregate then has one strict branch per serde tag:

```json
{
  "type": "object",
  "allOf": [
    { "$ref": "start-run/🧬️schema/🔣️.json#/$defs/payload" },
    { "type": "object", "required": ["operation"], "properties": { "operation": { "const": "startRun" } } }
  ],
  "unevaluatedProperties": false
}
```

The review script constructs this proposed structure from the current payload bodies without
copying their fields, confirms each proposed leaf still accepts all six/rejects all nineteen, and
then confirms the aggregate accepts five/rejects twenty-two envelope vectors. This is a schema
design proof only; production schemas remain unchanged.

## Independent Rust Client Observability

`🧪️run-direct-31-independent-review/🦀️client.rs` runs its metadata, codec, trigger-JSON,
current replacement-inverse, total-inverse contract, seal-admission, and duplicate-start-admission
checks independently under `catch_unwind`, emitting one `[DEBUG]` result per check before a final
failure count. The total-inverse contract deliberately fails for a first `FinishRunNode` insertion
until the descriptor/behavior contract is made coherent.

## Current Artifact Runtime Evidence

The current paired framework artifacts compiled the independent client successfully through the
workspace Bun/Nx launcher. Each dependency was supplied twice as separate argv pairs, first its
`.rlib` then its matching `.rmeta`; the public framework pair used the `0ff4ad5272f93f3d` stem,
kernel its unstemmed pair, async `75ee77491e40e8a5`, serde `73de109b1e55818a`, and serde_json
`0caf27179e7b9139`.

The compiled client then executed through the same launcher. It produced four passing independent
checks (`metadata`, `codecs`, `replacement-inverse`, and `seal-admission`) and three visible
semantic failures:

- `trigger-json`: actual Automation JSON is `automation_ref`/`event_fingerprint`, not the schema's
  required `automationRef`/`eventFingerprint`.
- `total-inverse-contract`: a first `FinishRunNode` insertion has no inverse.
- `duplicate-start-admission`: a second checked `StartRun` is accepted.

The run exits 101 by design after reporting all seven checks; the three failures are retained in
`🧪️run-direct-31-independent-review/🧫️run-u9hOml/🧪️runtime-paired-argv.log`, not hidden as a
successful test. Compilation is retained in `🧪️compile-paired-argv.log`. SHA-256 fingerprints of
the ten exact `.rlib`/`.rmeta` inputs before and after the compile/run are identical in
`🔣️artifact-pre.sha256` and `🔣️artifact-post.sha256`; no framework artifact was changed.

The root-owned replay with the known-good controller independently produced the same four passes
and three failures, with compile exit 0 and runtime exit 101 in
`🧪️workflow-actual-source-34/🧫️run-fr2fEk`. It confirms these are semantic findings rather than
an artifact-pairing or client-link failure.

## Permanent Regression and Fix Design

No production edit is made by this review. When its owner releases the Run source region, the
minimal permanent packet is:

1. Add `rename_all_fields = "camelCase"` to the existing internally tagged `RunTrigger` serde
   declaration. Add a Rust JSON round-trip assertion for both variants that requires exactly
   `{ "kind": "manual", "actor": ... }` and
   `{ "kind": "automation", "automationRef": ..., "eventFingerprint": ... }`, while decoding
   snake-case Automation input as an error under the current strict payload contract.
2. Change `apply_run_operation_checked` into the one admission authority for `StartRun`: before
   application, reject it when `status != Pending` or `started_at` is nonempty. This is the exact
   predicate already implemented by `RunDiff::apply`, so the permanent async regression must show
   first start succeeds, second start returns `Err`, and a rejected operation leaves the artifact
   unchanged. Keep the existing post-seal exhaustive rejection test.
3. Decide the `FinishRunNode` inverse contract before implementation. The fixed five-operation
   roster has no deletion operation, so a first insertion cannot have a truthful total inverse;
   manufacturing a nonempty `FinishRunNode` would not restore the prior artifact. The coherent
   fixed-roster choice is to mark and test this leaf as replacement-only: replacement restores the
   prior record and first insertion has an empty inverse. If the metadata policy instead requires a
   total inverse, it must intentionally add a sixth removal mutation plus descriptor, codec,
   aggregate-schema branch, and round-trip vectors; that is a separate canonical-roster change,
   not a local inverse implementation.
4. Add the approved aggregate JSON document using shared `#/$defs/payload` bodies: each leaf
   remains strict, and each aggregate `operation` branch references its payload plus the literal
   tag under `unevaluatedProperties: false`. Reuse this review's six valid, nineteen invalid leaf,
   and five valid/twenty-two invalid aggregate vectors as the neutral Ajv regression.

The first two are direct correctness fixes. Item 3 deliberately records the unresolved semantic
policy boundary rather than disguising an impossible total inverse as a passing test.

## Strictness and Admission Repair

The released Run source region now applies the first two repair items without changing the
five-operation roster or `FinishRunNode` inverse. `RunTrigger` has exact camel-case variant fields
and rejects unknown/snake-case JSON. The shared nested Run payload records also reject unknown
fields. The five payload schemas now expose one open outer `#/$defs/payload` each; strictness is
enforced by the leaf wrapper and by every aggregate branch's `unevaluatedProperties: false`, while
every nested object remains closed. The new aggregate schema at
`run/🧬️schema/🧬️mutations/🔣️.json` references those definitions rather than repeating payload
bodies.

`apply_run_operation_checked` now returns `protocol::MutationApplyResult<RunArtifact>`. It first
converts any default-merge-policy-rejected outcome message into the exact typed
`MutationApplyError`, retaining its code, message, and target, then delegates application to the
existing `RunDiff::apply`. Thus the checked seam and ordinary diff application share the same
sealed and duplicate-Start rejection. `RunSink` maps that owned error to
`RunError::MutationApply` without flattening it to a string and does not append a rejected
operation.

The ticket fixture's updated Ajv run passed with the strict aggregate: five aggregate positives
and twenty-two aggregate/nested negatives were rejected. The Rust tests were added but not run in
this lane because the root serializes the fresh framework build. The root controller's expected
workflow test count changes from 47 to 49; the separate RunSink test is outside that controller.

The remaining non-implemented algebra observation is that `RunDiff::absorb` retains only the last
non-empty diff, rather than composing sequential diffs. It remains outside this strictness and
admission packet, alongside the held immutable-history inverse decision.
