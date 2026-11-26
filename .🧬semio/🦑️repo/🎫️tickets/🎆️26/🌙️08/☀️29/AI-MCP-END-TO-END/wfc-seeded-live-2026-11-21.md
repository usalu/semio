# Persisted WFC Inference and Cancellation

The SDK connected to the real native MCP executable and discovered the pinned WFC bitmap solve service. The service published a schema for the guest result and bound the input through the `document` field. The native folder seeder recorded the tracked WFC owner genesis pair via `FolderEventLogStorage::write_if_current` and verified it through a separate `read_snapshot` on every attempt, including after each failure. Each gateway used a disposable folder without other workspace documents at launch.

| Native gateway or fixture | Identity |
| --- | --- |
| Gateway SHA-256 | `a1766c3f0e87cfee365c24c8461d1ad8aeec4983bb2a217806e2b22e576e004b` |
| WFC prepared model semantic SHA-256 | `94cde94f083e500f76c6a29d46f8aa6a62eee2109b8190ae1c692e8888edae9e` |
| Exact owner Pack, 405 bytes | `5e379b8f0b5f290b1dcac9a8fe23c20d2c826fbc94205b35d3d0dda776f66891` |
| Exact owner SPR, 211 bytes | `d2892d4d45a8b27ebbd89559cfb97a25624a45b3754ff2b48e5fdb3ee6be8771` |

## Observed on the Original Native Gateway

- The unbound `inference_run` emitted SDK progress at 0.05, 0.15, 0.2375, 0.25, and 0.35. The final three messages were `binding: reading-document`, `` routing `s.wfc.bitmap/s.wfc.bitmap.solve` to `wfc` ``, and `running in the plugin's guest`. The client then received MCP error −32000, `Connection closed`. Native stderr recorded `thread 'main' (29375642) has overflowed its stack` followed by `fatal runtime error: stack overflow, aborting`. No terminal result arrived, so the published result schema and solution invariants could not be applied.
- The unbound `inference_submit` returned a retryable `PLUGIN_UNAVAILABLE`: the seeded ID “has no schema this workspace can answer for”. The submit command calls `resolve_document_inference_service` before allocating the job and resolves the artifact kind through the schema resource. The executable's folder schema resource required an open probe or a plugin binding. The synchronous run accepts a caller-provided artifact kind and reaches the solver without this resource.
- In a fresh cancellation session, `artifact_open` successfully returned kind `s.wfc.bitmap` and folder event log custody, reporting Pack 405 and SPR 211 bytes. A subsequent `inference_submit` reached `semio-inference-job_736c7891c0912dbdb207c0c0e2`, whose native thread overflowed and aborted the process before the SDK received a handle. The SDK observed no active job or terminal cancellation.
- The synchronous guest command is driven with `exchange_blocking` on the caller thread after the activation pool handles document binding. Submitted jobs use an independently spawned native thread. The two process failures identify both affected entry threads. A diagnostic launcher was refused by the host when it tried to raise the inherited main stack limit, so it did not test a larger stack.

The independent fixture uses 16 × 16 input, three palette entries and 24 × 24 periodic output. If a succeeding gateway emits a result, the harness checks the published schema via Semio and Ajv, the absence of contradiction, and the canonical base64 representation of 576 indices in the input palette range. The pinned 1 × 1 solve and typed artifact creation still require their own end-to-end proof.

## Native Gateway With Sized Workers and Cold Schema Lookup

The rebuilt gateway was `b55d45cc16540e3d49c9c0228fc2c201e62489eab5c6dbc80c541b93e2f9a8d3`. The native stdio suite passed all 63 tests. Two independent pinned Bun processes loaded the official MCP SDK, started this exact gateway, and resolved `s.wfc.bitmap` from a cold folder resource before any artifact open.

- The solve reported actual document binding and guest dispatch, passed the previous stack overflow, and failed while dropping its routed `GuestRelayOperation` at `📦️operation.rs:77`. The first panic was `operation owner requires complete Source retirement`. Dropping the remaining original actor context raised a second panic, which aborted the connection before an inference result.
- Fresh submit succeeded without pre-opening the document. The SDK observed a running job at 0.35 and submitted cancellation for its returned job handle. The gateway recorded a `cancel-requested` event 167 ms after acceptance. At the end of the polling window the job was still running with cancellation requested. The gateway answered a subsequent ping.
- An independent native storage read verified the original 405-byte Pack and 211-byte SPR after each run. Neither run establishes successful typed creation, a successful inference result, or terminal cancellation.

The retained SDK script can reproduce each lane with `solve --schema-probe --gateway=<rebuilt gateway>` or `cancel --schema-probe --gateway=<rebuilt gateway>`. For the two runs above, the direct pinned Bun launch completed the SDK scripts after the repo tool wrapper returned before either script produced its SDK evidence.

## Authenticated Receipt Frontier

The diagnostic gateway `9603c9ffcfe0a44222d1be0fc799fae33cf4fa7ccf745e26bcef8918f2707e4b` again reached guest opening after resolving the cold schema and reading the same persisted document. The first routed operation refusal was `resident header awaits its authenticated escrow capture`. Each of the two close attempts then refused with `resident header retains its uncaptured escrow`. The ownership assertion aborted the connection after those recorded failures. The independent folder reader confirmed the exact Pack and SPR after the abort.

The cold route now uses the common frozen receipt validation, conservation and settlement stages with its own original History lease. The next native build exercises its typed Actor and Job continuation and retirement through the same WFC SDK call.

## Reproduction

```sh
repo tools exec --light bun -- "$ticket/wfc-seeded-link.ts"
repo tools exec --light bun -- "$ticket/wfc-seeded-sdk.ts" solve --label=trace
repo tools exec --light bun -- "$ticket/wfc-seeded-sdk.ts" cancel
repo tools exec --light bun -- "$ticket/wfc-seeded-sdk.ts" cancel --open-for-submit
```

The linker uses the cached full native folder owner variant `9d0b44bf77df9a85` and exact Cargo fingerprint dependencies. The SDK uses the `m-canon` WFC model, explicit policies, and a generated workspace. It can target a rebuilt gateway through `--gateway` and verify its cold schema resource through `--schema-probe`. The cancellation check requires SDK-observed active guest dispatch followed by terminal cancellation, no commit, a responsive server and byte-identical persisted document representations.

The native storage owner seeded a WFC bitmap and reread its exact 405-byte Pack and 211-byte journal. With that pre-existing document, the installed SDK submitted a WFC inference, observed the real job running while the gateway hashed the component, cancelled it with the returned handle, and received a terminal cancellation with no proposal. The gateway answered ping, and native readback confirmed identical document bytes. A separate solve reached the plugin’s cold-job route but the relay panicked while retiring its Source. The native gateway returned an internal tool error. Using a seeded document isolates inference from typed creation and does not qualify the combined journey.
