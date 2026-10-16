# Bazel Service Acknowledgement Source Membership

This is a bounded read-only Source/declaration audit against the latest retained exact three-entry capture, `🗑️generated/tooling-closures-watch-queue-trim-held.json`. It does not read the SDK filesystem, acquire Tools/graph leases, rescan closure imports, or attest whole-current SDK acceptance. No production Source changed.

The actual current Bazel owner is `build/🔧️tools/🏗️bazel/📜️script.ts`; no sibling `🟦️.ts` exists. Its server cancellation (`bazelCancellationControl`), action receiver cancellation (`withBazelCancellation`), birth handshake, text framing, and primary-preserving cleanup (`withBazelTaskCleanupV1`) are local functions in that owner. The owner, its policy, its birth schema, and every other path beneath this Bazel implementation directory have zero entries in each of the captured SDK roles.

High confirmed the proposed service-ACK correction is contained in this owner and its own closed protocol schema/test fixtures, with no planned PROCESS/routing/owned-execution production change. On that scope, the batch does not change any of the existing SDK Source entries or the producer-nine fingerprint. Any introduced imports must still be checked by the normal actual scanner before a new SDK qualification; this report does not assume future Source topology.

| Current Source path | Reader | Runtime | Compiler | Producer9 |
| --- | --- | --- | --- | --- |
| `🏗️build/🔧️tools/🏗️bazel/📜️script.ts` | no | no | no | no |
| `🏗️build/🔧️tools/🏗️bazel/🔣️policy.json` | no | no | no | no |
| `🏗️build/🔧️tools/🏗️bazel/🧬️schema/⚙️policy.json` | no | no | no | no |
| `🏗️build/🔧️tools/🏗️bazel/🧬️schema/📥️birth.json` | no | no | no | no |
| `🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts` | no | no | no | no |
| `🏃️process/🎛️owned-execution/🟦️.ts` | yes | yes | yes | no |
| `🏃️process/🎛️owned-execution/📥️capture/🟦️.ts` | yes | yes | yes | no |
| `🏃️process/🪓️termination/🟦️.ts` | yes | yes | yes | no |
| `🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts` | yes | yes | yes | no |
| `🏃️process/🧭️routing/📥️invocation/🏃️process/🟦️.ts` | yes | yes | yes | no |
| `🏃️process/🏗️build/🎟️capacity/🟦️.ts` | yes | yes | yes | no |

Shared implementation files behind the execution/routing facades are present in all three roles. Their individually reread current hashes match their captured entries at this audit. Therefore changing shared owned execution, capture, process-tree termination, Original routing/issuance, or compiler admission control would affect all three roles. The Bazel execution facade itself is not in these closure sets, although its underlying owned-execution implementation is. Source rows/counts are reader101, runtime174, compiler58; a contained owner/schema change leaves these declared captured sets unchanged.

The nine producer implementation inputs are explicitly the graph owner, graph admission owner, three selection/owner/admission schemas, neutral graph storage MJS, quota schema, ledger MJS, and ledger schema. The publication digest additionally hashes the publication schema. Neither Bazel owner nor shared process cancellation files is among those exact nine inputs. This does not remove normal graph-observation currentness checks: Source imports and task input facts can still invalidate an in-flight selected computation independently of the nine-module fingerprint. The current public workspace therefore retains all its ordinary final rechecks.

Capture digests:

- reader: 101 paths, `2ba730311b27f59d7e879f88f6b8ef2960a9939152694b575597081af4c19998`.
- runtime: 174 paths, `edd3eb46eade2d4aa730dc4fb448fe95a53d0ddc72340ea68a9bccc3896a21be`.
- compiler: 58 paths, `1dfc1718593747fd506f51c75347fae8fbdaf37fddd0f0de6c97036eadb0ca75`.

Exact path/hash membership and the nine input paths are retained in `🗑️generated/bazel-service-ack-source-membership.json`. This is preparation for deciding whether the actual held batch requires SDK refresh, not a runtime ACK acceptance or a new SDK success claim.
