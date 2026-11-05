# Runtime Handoff — 2026-11-04

Read-only authoritative snapshot at 2026-11-04T22:45:57.859Z. Native repo fleet/capacity commands both completed exit 0; no gates launched or jobs changed.

{"running":9,"blocked":0,"waiting":10,"alerts":0,"warnings":8}; memory pressure warn. This is liveness/admission evidence, not test or compiler success.

| Verified PID(s) | Owner | Resource session | Target / phase |
|---|---|---|---|
| 79078, 79084 | 01a1a72a-e611-7ee1-a514-96fdd28577ba | a3fba9d52db0.a1ndjxyvfy | @semio-tech/framework-renderer-wgpu:wasm:prototype; running; resource-directory-admission:v1 |
| 57214, 83550, 83648 | 01a1a729-b2e2-74b0-b267-69a4cb5617f1 | a3fba9d52db0.a11zl9jdzxw | @semio-tech/repo-lib:test-native-dependencies; running; facts 245760/408854 |
| 61078, 88335, 88460 | 01a1a72d-29f8-7c03-9b70-c3c39633bce2 | a3fba9d52db0.a1737ebikwt | @semio-tech/layout-layout-rs:test; running; dependencies:generation-verify 98304/? |
| 93587 | 01a1a72a-e611-7ee1-a514-96fdd28577ba | a3fba9d52db0.a1ndjxyvfy | child process; running; running |
| 93682 | 01a1a72a-e611-7ee1-a514-96fdd28577ba | a3fba9d52db0.a1ndjxyvfy | child process; running; cargo |

Bazel slot s1 live server PID 90095, holder PID 79084. No stale-owner inference from duration. Agent-scoped exec handles cannot be mapped independently from these resource sessions. Previously reported controller handles 96692 / 17125 / 74049 / 25997 are parent reports, not independently verified handles.

## Current Private OS Source

Issuer current SHA256 66ea3a8c730f381d251f0111bfa2e81a9b16c4a71eb50c7f56dd235c20baef21. Lines 44–48 bind actual invocation and caller control, derive nativeCallerPhaseV1 from that control, and use withRequestedIssuedResourceControlV1(phase.leaseControl, funding.maximumMilliseconds). Lines 56–57 check finite inherited remaining/deadline and project that issued control into the actual OS capture. This confirms the phase fix is authored; no new execution proof is inferred.

Private schema actual SHA256 2f0b075f981b1b1d1f54abe335f0f86e9b23f5ac3dfa4bc02001a40edf81ad78 matches schema-catalog.json:21158. Target registration is at packages/🟦️typescript/📋️project.json:4217; default dependency at :1517. Current target DATA:

{"executor":"repo:script","cache":false,"inputs":["default","^default","testEnv","reads"],"outputs":[],"dependsOn":[],"options":{"cwd":"{projectRoot}","command":"repo ./📜️script.ts test private-native-birth"},"metadata":{"semio":{"execution":{"version":1,"kind":"orchestration"},"sourceInputObservation":"freshness","runtimeSourceEntries":["{workspaceRoot}/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️testing/🌱️genuine-birth/🧫️private-workspace/🧪️tests/🚪️owner/🟦️.ts","{workspaceRoot}/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️testing/🌱️genuine-birth/🧫️private-workspace/🧪️tests/👷️child/📜️script.ts"],"cache":{"version":1,"class":"live","reason":"exercises-host-resources"}}}}

Direct package runtime-and-path ownership remains intentional; no caching codeSuites addition required.

## Retained Evidence

Authoritative snapshots: 🗑️generated/fleet-handoff-status-2026-11-04.json and fleet-handoff-capacity-2026-11-04.json. Relevant shared generated logs remain unmodified; directory contains fleet-wgpu-private-code-root-types-2026-11-04.log and producer-phase-{red,strict,strict-current}.log plus producer-phase-verification.json. Older host logs live under 🗑️generated/fleet-host-2026-11-04/ (layout-native-batches.log and activation-bazel-batches.log); their presence is not current completion proof.

Outstanding acceptance remains actual Native 20, genuine private OS witness, real WGPU product, prototype/release output laws, activation on 6179, retained browser history/export/archive/cleanup journey. Preserve earlier failed/cancelled preassertion results; do not equate controller exit, schema fixtures, strict typing, or Source audit with those gates. Root continues execution; this lane has no live test handles.
