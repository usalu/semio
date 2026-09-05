# Actor Hand-Reviewed Repair

Status: all 20 reviewed moves applied. A physical traversal found 146 entries, 140 governed, and zero emoji findings. The native Nx quick route passed all 121 tests with zero skips using a ticket-local Cargo target and nextest artifacts.

The shard client sends actor traffic through a bounded worker pool, so `📮️shard-client` distinguishes its transport/client role from the `🧵️shard-runtime` worker runtime. Existing shard semantic IDs and exported APIs remain unchanged.

Each of the following fixture-schema documents was read and confirmed to describe neutral test laws or vectors rather than runtime payload data. Its containing `🧪️schema` folder will use `📐️schema`, distinct from the fixture and executable-test siblings:

| Owner | Confirmed fixture contract |
| --- | --- |
| `📃️page` | Byte-page vectors, lengths, padding, and ownership |
| `📤️return` | Retained-return wire vectors, limits, and laws |
| `📤️return/📨️response` | Response tags, vectors, invalid bytes, and authority |
| `📤️return/📨️response/🎟️credit` | Receiver/worker credit and reservation laws |
| `🚪️lifetime` | Lifecycle, receipt, completion, and failure vectors |
| `🚪️lifetime/🩹️patch` | Patch receipt pairing and wire-feedback vectors |
| `🪪️activation` | Activation lease revocation/disposal cases |
| `🪪️activation/📨️inbound` | Actor inbound activation cases |
| `🪪️activation/🚪️instance` | Instance phases, refusals, and cancellation |
| `🪪️activation/🚪️instance/📥️output/🏘️admission` | Output admission, dispatch, and cancellation laws |
| `🪪️activation/📤️return` | Captured return authority and boundaries |
| `🪪️activation/📤️return/🏘️admission` | Captured-return admission and retained-fault laws |

Only three fixture folders also collide with executable-test siblings: page, retained return, and lifetime patch. These will use `🧫️fixture`; other already distinct fixture names are retained.

The two admission contract JSON documents will use `🤝️contract.json`, distinguishing an agreement from `🧬️schema.json`. Lifetime's fault specimen will use `🚨️fault.fixture.json`; its fault-validation schema will use `🧯️fault.schema.json`.

No automatic emoji picker, bulk replacement, normalization write, or Git mutation is used.

The first post-move TypeScript run caught three missed response-to-return fixture imports. Those exact imports were repaired and the quick route was rerun: 198 passes, one failure, matching the pre-move baseline. The remaining `ActorWorkerInboxInventory` failure expects `Error: post-after-observation` but receives `{}`; no assertion was weakened. The response framing fixture imports and strict TypeScript diagnostics now pass. Both `🌿️framing` directories still exist and their relative references were confirmed; they were not renamed merely to satisfy a global lookup.
