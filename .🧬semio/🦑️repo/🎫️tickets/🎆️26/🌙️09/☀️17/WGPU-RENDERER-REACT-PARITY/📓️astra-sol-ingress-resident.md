# Contribution ingress and resident-budget receipt

## Exact AppChannel contribution ingress

The contribution preflight previously encoded the raw view and priced every outer `AppCommand::Command` as sequence `1`. The real `AppChannelClient.command` consumes the handle-owned `AppChannelRequestSequence`, normalizes the view through `viewContextWireValue`, Pack-encodes it, and then encodes the outer command. The mismatch was observable before the command crossed the shard boundary:

| next sequence | stale estimate | actual AppChannel bytes |
| ---: | ---: | ---: |
| 127 | 410 | 389 |
| 128 | 410 | 390 |
| 9,007,199,254,740,991 | 410 | 396 |

The full page-ceiling fixture also showed the sequence-width error independently: `190982` estimated bytes versus `190983` actual bytes at sequence `128`.

The repaired estimator now accepts the exact next channel sequence, applies `viewContextWireValue` before Pack encoding, and computes the outer byte length from the two encoded byte-vector lengths. It does not materialize number arrays or encode a duplicate whole command envelope. `pushScopedContributions` reads `channelRequests.checkpoint().sequence`, refuses exhaustion, and passes the next sequence into the estimator immediately before the synchronous `AppChannelClient.command` admission path. There is no yield between that checkpoint and sequence consumption.

The neutral ingress fixture records the `127`, `128`, and maximum-safe sequence-owner cases. The existing integer-carrier view fixture is validated against the view-context JSON schema with Ajv, then sent through a real `AppChannelClient`; its actual encoded `view_state` must equal the fixture's canonical Pack hex. This covers integer-tag normalization rather than comparing the estimator to a second copy of its own algorithm.

### Focused validation

Fail-first command:

```text
NX_DAEMON=false NX_FORCE_REUSE_CACHED_GRAPH=true bun nx exec --projects=workspace -- bun x vitest run 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔬️wgpu-extension-dispatch/🟦️.ts --config 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧪️tests/🎚️config/🟦️.ts
```

Before the repair: `10 passed, 8 failed`. The failures included all three real-client sequence cases, the low-level signature change, and the page-ceiling mismatch.

After the repair: `18 passed, 0 failed` in `3.33s`.

`git diff --check` passed for the production estimator, fixture, and focused test.

### Files

- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🐚️plugin-bridge/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔬️wgpu-extension-dispatch/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧫️fixtures/🔬️wgpu-extension-dispatch/🔣️.json`

## Resident budget receipt

The populated retained-document fixture uses the real resident record accounting and establishes the current budget boundary without changing the budget or capacities: 62 full 128-node documents are admitted and the 63rd is refused. Static backing is 566,352 bytes and each populated document is 835,986 bytes under the measured fixture. The earlier unsupported 64-document claim was removed from production documentation and assertion messages by the root integration pass.
