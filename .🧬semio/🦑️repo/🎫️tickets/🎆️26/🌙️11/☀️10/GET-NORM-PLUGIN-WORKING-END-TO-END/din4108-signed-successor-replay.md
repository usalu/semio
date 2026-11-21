# DIN 4108 Signed Successor Replay

The browser replay waits for native Play 6034 to serve a different Base wrapper from signed `wKEwym4` digest `2ce4f85c3f769ca5f260eab4244ff6daa2d43673498a12e59b85feeb8008ef5c`. The receipt watcher checks two consecutive observations, hashes the canonical wrapper against its receipt, and compares the served receipt, bridge, JavaScript bindings and Wasm core with the installed files.

The existing DIN 4108 browser probe now has an opt-in lifecycle witness. Its observation runs at the end of the actual renderer close loop, with the instance ID and complete lease state. The probe records the three example acknowledgements, both role archive acknowledgements, subsequent paint, and each predecessor’s terminal disposition. The same browser runs the entire sequence with the authored actor and root stock.

## First build candidate

Run `wqw4JWI` linked and completed its descriptor stages. Provenance then rejected the compiler’s consumed bytes for `store-replacement-initializer-and-retirement/🦀️.rs` because they had no matching retained source witness. The component command returned 75 and Play 6034 continued to serve the prior signed Base wrapper. The passive watcher remains active for a coherent successor.

## Unsigned `wqw4JWI` comparison

The private 6034 Chromium page used the exact cached pair for linked wrapper SHA-256 `24dfc6bfd219be66ef7a3ac36a656ad51248c30828125e287e564567982a41ca`. The Wasm core was `e2cf091d7dd6a6472c56b52091e23ae559d1893f181ff475379ca08e8c9885c2` and the raw JCO bindings were `f42b3f1f953b7aad28e2cd858effd7c34f4b619234e9477bd009aeb66f53a60c`. The ordinary browser materializer’s Preview2 import rewrite served the bindings. A loopback stream carried the unaltered core for this single browser context. Canonical delivery still served its signed baseline. The page used authored stock, including 8,388,608 operation items and 2 GiB operation copy.

The first three archives acknowledged at 23.3, 48.5 and 67.8 seconds. The restored example, picker and Complies verdict agreed at 70.4 seconds. Viewer instance 2 acknowledged its transfer at 86.8 seconds and settled at 95.3 seconds with four canvases and 1,620 new draws. Editor instance 3 acknowledged at 124.9 seconds and settled at 136.5 seconds with four canvases and 6,060 new draws. The probe recorded the terminal disposition of both predecessor close attempts. Editor instance 1 exhausted the renderer’s close budget at 120.2 seconds. Viewer instance 2 returned native `original lifecycle receipt differs from its caller authority` at epoch 50038, 132.5 seconds. The strict retirement assertion failed.

The unique refusal originates in `MountedOwnerTurnV1::advance_lifecycle`. It checks the full reported receipt against the original caller’s posted receipt, the fresh debit against both the call grant and cumulative phase grant, and the presence of fresh work debit. The WIT message provides the shared text. It omits the failing comparison, the receipt amounts and which close owner advanced. The previous `original job close receipt exceeds its grant` and `non-empty terminal witness` refusals were absent in this run.

One candidate is the accounting boundary between `original_lifecycle_step` and the mounted validator. The inner helper computes unposted spend from a ledger with redeemed reserved returns removed. It then forwards the unchanged step. A reserved redemption in that callback would make the caller’s gross posted receipt equal the reported receipt plus the redeemed delta. The outer validator requires the full posted receipt. Its existing reserved-return test drives the validator directly and establishes the intended fresh-versus-gross distinction. The browser capture leaves reserved redemption and the close rung unresolved.

The evidence is in `🗑️generated/din4108-wqw-unsigned-pair.json` and `🗑️generated/din4108-wqw-unsigned-replay.log`. The signed successor watcher continues independently.

## Reserved-return control

The ticket-local Rust probe linked the actual nested lifecycle helper and mounted validator from the live `w0fqQVa` WASI dependency cohort. It replayed both rows of the existing mounted-owner reserved-return fixture. The outer-only control accepted a gross receipt on both rows. The nested helper control accepted the fresh receipt when the reserved return was absent.

With redemption present, the nested helper returned the same fresh receipt and the caller posted the gross amount. In the first row, the caller posted 3 items, 8,748 copy bytes and 6,144 capacity bytes. The forwarded receipt had 2 items, 8,000 copy bytes and the same capacity. The next row posted 4 items and 173 copy bytes while the forwarded receipt had 3 items and 55 copy bytes. The outer validator raised the exact mounted refusal on both rows. SQLite independently reproduced both differences.

This control proves the receipt boundary disagrees when a callback redeems reserved credit. The browser trace still needs the redeemed delta and close rung to establish whether the observed Viewer refusal used this boundary. See `🗑️generated/din4108-mounted-reserved-final.log` and `🗑️generated/din4108-mounted-reserved-sqlite.log`.

## Mounted receipt settlement

The mounted validator now checks the producer’s reported receipt against either the complete posted ledger or its exact fresh delta. It returns the complete posted ledger when a producer reported the fresh delta. The per-call, cumulative phase, and fresh work checks still use their original currency accounting. An already-normalized composed step keeps its full receipt, and the inner helper continues to debit only fresh unposted currency.

The repository fixture uses both reserved-return cases to assert the exact gross receipt, fresh remaining grant, and single reserved debit. It also exercises an incorrect producer receipt and an overrun of the phase copy grant. The WASI red run reproduced the old refusal. The green probe linked the real caller APIs and nested helper from the `w0fqQVa` dependency cohort and compiled the changed validator and receipt projection from current source in a ticket-local module. Eight checks passed across the two fixture rows, including the direct gross, no-redemption, composed gross, and fresh nested paths.

The publisher continues through the normal signed build path. See `🗑️generated/mounted-receipt-native-red.log` and `🗑️generated/mounted-receipt-native-green.log`.

The final mounted receipt edits are in these source paths.

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/🔌️plugin-app-impl/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/🧱️construction/📦️original/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏛️ownership/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏇️mounted-owner/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏇️mounted-owner/🧪️tests/🦀️.rs`

Ticket inputs are `check-mounted-reserved-return.rs`, `check-mounted-reserved-return.py`, and `check-mounted-receipt-metadata.py`. The passive delivery checker is `wait-din4108-signed-base.py` and the browser probe is `probe-din4108-role.mjs`.

The standalone whole-crate metadata attempt compiled the current retirement crate, then reached a mixed-cohort identity error in the value facade. The facade’s cached dependencies still use the previous retirement crate identity for `RetireOwned`. The green result therefore covers the isolated current source methods with actual caller libraries. Full integration requires a coherent dependency rebuild. See `🗑️generated/mounted-receipt-full-metadata.log`.

The final design leaves the foundational retirement contract byte-identical to its repository revision. The three mounted call sites pass the lifecycle step’s receipt projection directly. Pinned Rust compiled the complete current `semio_plugin_mounted_owner` and `semio_plugin_ownership` crates as WASI metadata and libraries against the exact `w0fqQVa` dependency cohort. Linking those unmodified current crate outputs into the probe passed all eight cases again. See `🗑️generated/mounted-receipt-current-libraries.log` and `🗑️generated/mounted-receipt-current-green.log`. The browser evidence still comes from the earlier unsigned wrapper.


The pinned WASI probe accepted all eight cases in the two neutral fixture rows. In the first row, the reserved receipt contributes one item and 748 bytes, yielding three items and 8,748 bytes gross. Its no-reservation control returns two items and 8,000 bytes. In the second row, the reserved receipt contributes one item and 118 bytes, yielding four items and 173 bytes gross. Its no-reservation control returns three items and 55 bytes. Each row passed the direct-gross, composed-gross, nested control and nested reserved-return cases. The native regression fixture also refuses corrupt reported progress and genuine phase overrun.

## Warm Base provenance refusal

The native publisher refused `w0fqQVa` with `Compiler consumed inconsistent source versions` for `🧰️framework/🔨️modules/🌱️value/♻️retirement/🧬️contract/🔗️arc-custody/🦀️.rs`. The first Cargo invocation reported `fresh:false` for both the host `16011ef3cbd78858` and WASI `956f530d4cb3fb81` units of `semio-framework-value-retirement-contract` in Base lane `p5d96da`. Both units wrote their dep-info around 00:51:33 local time on 2026-11-21. Both checksums record the source as 25,273 bytes with BLAKE3 `35ca65a2e282f561681c5b6bfe5ef4d66fc13da1b7c971f74c8a6915b2090c3d`. The retry reported `fresh:true` for the same two units and left their dep-info intact.

The source recorded a later mtime at 01:10:58.218073. At inspection it was 25,243 bytes with SHA-256 `ab4cd2dbacc386728e9830a5852cbef6d9b74e7e50e6676783d4a5f06f2e18cd`, byte-identical to the repository revision. The lengths prove that the same-lane retry reused outputs from before a live byte change during the original build. The provenance validator compares both recorded compiler digests and current digests when a source is encountered without a dep-info checksum, so the namesake refusal does not require that two different cached units consumed different bytes. The capture establishes the changed input and reuse sequence, without identifying the author of that change or which second provenance edge first selected the new digest. Canonical signing retained its existing publication and the passive delivery watcher continued without opening a browser. At 01:40 local time the outer command had logged that it would take a new snapshot, but the native Base driver and its Cargo process had exited. The Energy and grouped Eurocode drivers were still active.

## Energy provenance precheck

The active Energy `wdwI4gt` generation compiles in lane `pd19132`. Its host and WASI retirement-contract dep-info both record the current arc-custody source length and BLAKE3, 25,243 bytes and `85c8873a23c93817e7216502b288592a97f46769d66090961da920545a1e284e`. The handoff to the retry reused these units, whose original dep-info writes at 01:11:50 and 01:15:24 followed the source mtime at 01:10:58. The unit graph identifies the second input as belonging to the WASI-only `semio_plugin_document_media`. Before it emitted a dependency artifact, the live store replacement initializer measured 31,128 bytes, BLAKE3 `1913889bab605b56eeadee495df96b8ee2881f8f750d31e74481cf94c8f6825d`, with a source mtime of 00:20:17.

The WASI unit `5ea3b6c6f8c2a447` emitted document dep-info at 02:24:38.902192. The record exactly matches the precheck length and BLAKE3. Rehashing both live sources after the artifact appeared confirmed that the document record and both arc-custody records match their current source bytes. This resolves the two specific early-warning checks while the full signed publication gate remains pending.

## Signed Base verification status

The final 6034 delivery census and a subsequent exact receipt comparison both retain the signed Base wrapper `2ce4f85c3f769ca5f260eab4244ff6daa2d43673498a12e59b85feeb8008ef5c`. No newer Base publication is running. The signed-successor watcher was stopped with exit 130 after the last exact comparison, so the strict signed successor browser replay remains unrun. The archived signed baseline and exact unsigned replay capture the browser behavior described above, and the current mounted close projection has the eight-case pinned WASI evidence.
