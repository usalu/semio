# Guest-Issued Patch Authority

## Current Checkpoint

The new Actor patch receipt codec and outer field are source-mounted after an actual compile-RED with 15 missing-API errors. The compiler owner then ran the new three tests: **3 passed, 0 failed, 102 skipped, .043s**, and existing lifecycle/outer regression: **5 passed, 0 failed, 100 skipped, .211s**. Logs are `🧪️member-actor-patch-receipt-green-r2-native-2026-08-27.txt` and `🧪️member-actor-patch-lifecycle-regression-r3-native-2026-08-27.txt`. The common Kernel patch-owner repair has an actual 12-test native pass; none of these is a guest lifecycle aggregate pass. Kernel/reactor lifecycle mounting remains incomplete and has not passed a fresh integration or Wasm gate.

## Canonical Contract

`ActorUiPatchReceipt` contains the full `ActorInstanceLifetime` and a positive runtime-issued `patchSequence` in the complete u64 domain. JSON uses canonical decimal strings; TypeScript uses bigint. Four canonical unsigned LEB128 fields encode activation generation, u32 instance, guest lifetime, and patch sequence in at most 35 bytes.

`TurnResult.uiPatchReceipt` follows `lifecycleReceipt` in the existing outer packet as another bounded optional-length field. No semantic `UiPatch` field changes are needed. The typed boundary requires exactly zero patches with no receipt or one patch with one receipt. Empty patch payload is represented by zero bytes; opaque actor payload validation cannot substitute for the typed consumer's decoded-count check.

Both `PatchAck` and `PatchRejected` carry the exact receipt. Surface and revision remain payload validation fields, not owner identity. Runtime-issued sequence, full guest lifetime, surface, and revision must match the retained original publication. Stale feedback after same-activation numeric-ID and revision reuse must not affect the replacement. Clock or transport failure retains the exact receipt and structural source owner.

## Ownership Findings

Current `UiTurnPatches::IntoIterator` and `UiTurnPatchTransportLease::take_owner` release source slots before all raw patch descendants have finished in their downstream consumer. Slot absence is therefore not descendant retirement. The native close join must retain the original structural publication/handback owner until its exact receipt is consumed, separately from the host's retained UI aggregate.

The Kernel and WIT `PatchAck` and `PatchRejected` records now carry the receipt. Native conversion helpers map the exact fields. The reactor's semantic rejection branch still resets by surface alone and awaits the issued-owner join; the contract change by itself does not repair that branch.

The pending reactor now stages whole patches in the native-tested `UiPendingPatch` owner. Its immutable cached instance scope survives retirement of surface text; a full handback slot rejects a replacement without overwriting either root. Exact grant closure for runtime Ready/Published owners is being implemented by the retained lane; no zero-progress accounting or boundedness credit is inferred from the old boolean methods.

## Evidence And Remaining Gates

Permanent actor schemas and fixtures live under `framework/actor/🚪️lifetime/🩹️patch` in the repository taxonomy. They include four wire vectors, eight malformed vectors, five pairing cases, and feedback reuse/duplicate/clock expectations. Three passing native tests cover codec, rejected authority, and the outer field. Root independently validated schema and the four vectors with webassemblyjs LEB128, including the 35-byte maximum. Feedback reuse/duplicate/clock semantics remain integration requirements, not credit from codec tests.

Kernel held-lock Drop and normal-close laws each produced a genuine behavioral RED: waited true versus false, with the exact owner recovered before assertion. See `📓️kernel-turn-patch-red-r2-native-2026-08-27.md` and `📓️kernel-turn-patch-red-r3-native-2026-08-27.md`. The first wrong OS-Kernel target was unrelated test-metadata debt, not evidence for these laws.

The repair uses one inline `UiPendingPatch`, a pre-reserved key, a static fixed handback array, and atomic publication without a mutex or allocation in Drop. Normal arena access uses try-lock. Partial typed cursor state is transferred intact; the fixed key-plus-owner tuple is statically bounded to 4096 bytes. Exact multibyte descendant retirement is tested under 1/64/4096-byte grants, with zero-grant nonmutation. The compiler owner ran the proper common-framework `ui_turn_patch_` selector: **12 passed, 0 failed, 235 skipped, .253s**, including three owner laws and nine regressions. See `📓️kernel-turn-patch-green-r4-native-2026-08-27.md`. Raw transport and generic transfer/iterator boundaries still require their own retained descendant authority.

Required remaining work includes codec/outer/WIT joins, runtime issuance, exact native structural handback, ACK/rejection owner binding, all descendant preadmission, final receipt delivery/retry, and fresh native/Wasm integration. The seven tutorial joins and sixteen ordinary paged command ingress routes remain queued, not complete.

## General Actor Return Paging Inspection

Native `plugin/🖥️host/⏳️runtime.rs::convert_poll_success` and `plugin/🖥️host/🦀️component.rs::execute_turn` currently discard WIT patches and return an empty Kernel patch owner. The latter also drains emitted patches without a consumer. These are preexisting runtime handoff gaps. Merely forwarding the new nonempty receipt would create an invalid pairing; these conversions require retained, caller-granted typed phases, with the original WIT result retained before any fallible conversion. No boundedness credit follows from synchronous whole-array lifting.

There is no existing general actor return-paging API identified in the current actor Rust contract, common Kernel contract, or reactor WIT. Actor `TurnResult` still contains opaque whole `Vec<u8>` fields for patches, effects, and command-ingress. WIT returns typed whole patch/effect collections; its `command-ingress-page` is input authority, not a general output witness. Neither patch receipts nor AppCommand input witnesses authorize disposal of arbitrary return wrappers.

The peer-owned `actor/🪪️activation/🚪️instance/📥️output/🟦️component.ts` provides `OwnedActorTurnOutputs.reserve()` and an exact strong `OwnedActorTurnOutput` response/outcome slot. Its current `captureResponse()` preserves the original response before normalization, but no returned-root retirement or release method is implemented. This is an admission/retention seam, not bounded eventual output retirement. Unknown wrappers must remain retained. A future schema-owned output cursor must enumerate the canonical return fields and hand off their exact descendants before terminal release; arbitrary object traversal is not a witness.
