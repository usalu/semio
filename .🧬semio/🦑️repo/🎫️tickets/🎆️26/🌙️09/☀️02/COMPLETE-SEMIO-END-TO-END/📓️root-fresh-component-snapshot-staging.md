# Fresh Component Snapshot Staging

## Current Boundary

The current Hub script already has a concrete fixed Stdio/GIS bootstrap and immutable generation staging. This corrects the older derived-catalog packet's no-producer assumption. Root read `📓️terra-fresh-component-owned-snapshot-current-audit.md` fully before implementing the first prerequisite; there is no second GIS producer.

The previous producer verified descriptor buffers, erased them, then copied the descriptor pathname without comparing that copy to the verified bytes. It also hashed a reopened staged component for BLAKE3. The private producer now captures the raw component once through a bounded descriptor before JCO, copies that snapshot to a private work input, captures JCO's core once before the descriptor emitter and gives the emitter a private exact core copy. Descriptor JSON/Pack are captured and strictly verified against those retained raw/core digests. `freshStage` writes the exact retained byte buffers with exclusive creation, bounded chunks and fsync; it has no source path to reopen. Both staged SHA256 values must equal their admitted snapshots; BLAKE3 is calculated on the retained raw bytes. Raw/core/descriptor owners are wiped in the producer's finalizer. The exported receipt remains metadata-only.

The previously qualified browser file capture primitive was moved, not duplicated, to repo library `readStableBuildFile`. It exports only standard `Uint8Array`, retains pre/post descriptor/path identity and shared byte admission, and is used by both browser codegen and fresh component staging. Browser90129 is GREEN20 (`browser-actor-factory-SA99vS`,115627bytes) after that refactor, with all Node/WebCrypto/compiler/host/WASI comparisons preserved.

## Test-First Receipts

-81418 stopped on the new fixture's initially incorrect relative path, before testing behavior. That path is corrected.
-48562 is expected missing-capture-helper RED.
-24582 passes the five new staging laws (`fresh-component-staging-AjI87t`), then finds the old bootstrap generation pin. The coordinator's existing report independently identified the same stale pin after Stdio codec data changed; root did not remove the generation assertion.
-22322 is registered GREEN for the new laws plus the existing bootstrap corpus after updating the four exact generation/rotation references from `7cf0515d…` to `b96fb865…`. Independent Node/WebCrypto framing agrees; package closure remains two packages/28 selected codecs/one target, with19 hostile cases and four descriptor-pair cases. The separately linked provider fleet has29 receipts, which is not the selected two-package closure count.
-30368 is GREEN after capturing the core before emitter invocation and checking verification against that retained core even after its original path is replaced (`fresh-component-staging-eQtlpv`). Five named laws cover verified descriptor retention, source replacement, staged-file independence of the retained snapshot/receipt, cancellation after a file was written but before successful handoff, and descriptor identity mismatch before staging. AJV, the first-party Pack verifier, WebCrypto and the fixed BLAKE3 known answer supply independent checks.

All new tests run inside the existing registered `os-hub:trusted-stdio-gis-bundle-check --source` workflow; no new command/launch entry or standalone script was introduced. Partial cancellation output is removed only at the exact exclusively created test destination; retained reports, fixtures and remaining evidence are preserved.

## Remaining Integration

This is not a complete fresh Cargo/JCO/emitter materialization receipt or Hub startup proof. The tests invoke the capture/staging boundary with a neutral descriptor, not a fabricated complete build. A callback-scoped opaque snapshot owner is still required so the Hub can derive a browser actor before producer retirement without exposing/reopening a component path. One-time strict codec JSON capture and final whole-generation file revalidation are also pending. A mutation of a staged file cannot alter the retained snapshot, but this slice does not claim it is detected before generation rename.

After those input and publication boundaries are coherent, add the mandatory tagged browser-actor bundle record, bind every actor/source/policy field into generation, then wire the immutable loader, public plan/lease, protected target body and contained Worker. None of those downstream steps is claimed by these staging receipts.
