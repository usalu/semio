# Actor Return Result Codec

## Protocol Fault Refinement

The released schema now includes authority-free protocolFault results: exact bytes 050c/050d for malformedControl/mixedControl. Invalid zero or overflowing authority is never echoed through a fabricated valid identity. The fixed result is correlated only by the captured transport envelope. All three updated schema/fixture files were read before the test and implementation.

Actual protocol-fault RED: 2 failed, 3 passed, 101 skipped / 106 collected, 7.95 s, start 20:40:08. Actual full actor GREEN: 106/106, eight files, 7.86 s, start 20:41:09, exit 0. Logs are 🧪️actor-return-protocol-fault-{red,green}-1.log. The strict run reports eight diagnostics: known tutorial seven plus UI-owned UiDocumentStore:1226:137 TS2344 from ConstructorParameters on the private OwnedUiSceneJsonDocumentReader constructor. This was routed to its owner, not counted as a strict pass.

The three malformed-drive fixtures reject locally and the two exact protocol-fault vectors match independent LEB128/Buffer encoding. This is format validation, not execution of native preadmission, owner retention, semantic suppression or close laws.

Current hashes: return TS 836fe0351b67f1a86e953b5c41cb526fb67e1ef99090f377f43c714893751191; ShardClient b1a16ae654a7dcbdfa08f5b8807b0f0a31b80388c8c0fedd0fad6d833e540855; return schema a1039d317bb150607bad46902d7891e61a49ea8ae24abd7236396460783b2033; fixture ed2c9f97b5abdb39963969d13684e68701ea705d03adc1f7823ac0c25c3aa1e7; fixture schema 3328697d8ed6e7e8c3d939c5213ea276d7075aaa029b75e620258869ade72fff.

The following 105-test boundary remains historical evidence.

## Implemented Scope

The fixed ActorReturnResult codec now shares the canonical origin, identity, receipt and control primitives with ActorReturnDrive. ShardClient re-exports the result codec, types and maximum size. No live dispatch, WIT mapping, retained input owner or semantic content decoder was mounted.

The schema owner released eleven fixed-result vectors, two fixed-page vectors, seven contradictory result objects and explicit enum arrays. All three updated schema/fixture files and the independent oracle report were read before implementation.

Canonical poll behavior is now explicit: successful poll yields pending/page/retired, while a fixed control result echoing poll must be blocked/refused with a non-none fault. Accepted/duplicate poll controls reject. Refused results cannot carry fault none; accepted/duplicate non-poll controls require none.

Page encoding writes one receipt and exactly 4096 neutral bytes, using receipt.length as the only binary used-length field. The maximum envelope is 4138 bytes. Decoding checks exact fixed body length and zero padding before building the frozen neutral page. Control results allocate only a small fixed envelope, not a page. This bounds the format's allocations; it is not physical grant admission, an 8 ms timing certificate, producer provenance, page retirement or successful native close.

## Actual TDD

- 🧪️actor-return-result-red-1.log: four failed / 101 skipped, eight collected files, 1.40 s, start 20:31:10. Missing encoder, decoder and maximum constant were the expected failures.
- 🧪️actor-return-result-green-1.log: 105/105, eight files, 8.14 s, start 20:32:10, exit 0.
- 🧪️actor-return-result-strict-1.log: exit 1; exactly seven tutorial diagnostics. No return, actor or new UI JSON diagnostic in this captured run.
- 🧪️actor-return-result-green-2.log: 105/105 after the Shard public re-export, eight files, 7.49 s, start 20:34:12, exit 0.

The four new tests cover all released fixed results, both 4103/4138-byte page envelopes, all proper prefix truncations, offset views, trailing data, mismatched used lengths, nonzero tail, unknown tags/enums and source preservation on rejection. All 224 control/outcome/fault combinations agree with strict Ajv. Independent @webassemblyjs/leb128 plus Node Buffer reproduces the fixed fields and all 512 page words. A constructor probe confirms that a fixed refusal does not allocate a 4096-byte payload.

All temporary logs and evidence are retained. No schema weakening, generated publication, browser access, native compile, target cleanup or evidence deletion was performed.

## Captured Source Boundary

| Source | SHA256 |
| --- | --- |
| Return TS | 0fa849ae15ca2a37b1add66295b8a1127122004c574ed379b275809eb9216198 |
| ShardClient | b1a16ae654a7dcbdfa08f5b8807b0f0a31b80388c8c0fedd0fad6d833e540855 |
| Return schema | 16dd2ae43e260da23de7aca82895b10b9482bec966dc27f1e3b86ef9b4ebdb38 |
| Return fixture | 286cca66aaf417e3fb38fe25f76d09af74513cea7bba4fbe47d4105afaeaba75 |
| Return fixture schema | 0cd86b864db778320073464f2d911cfc5d52b0beb3d6e077ad3e0465e8669045 |

## Next Owned Integration

The coordinator accepted the outer content framing ownership split: this lane owns framing/section order and exact native page-fragment source; UI owns independently admitted semantic payload storage and typed UI decoding. Its complete native-operation-paging report was read. Proposed source types are OwnedKernelReturnInputField, OwnedKernelReturnInputFragment and OwnedKernelReturnInputRelease under Kernel/📤️return/📦️content/📥️input; these are proposed names, not implemented authority.

The actual source must originate in the captured activation/worker's reserved response, bind one exact destination builder, and retain original page/range identity through copied-input or cancelled-input proof and private release receipt. A copied fragment is not a decoded operation, UI publication or instance retirement. A cancelled fragment needs proof that no reader remains; it must not masquerade as a full copy. The parent source must retain all unoffered remainder independently.

The schema owner has released the pack-dialect clarification and Invocation/presence vectors. All four updated content files were read. Packed UI fields, opaque AppFrame bytes and nested scene bytes remain distinct. Full native presence binary parity is still pending; the declared UTF-8 fields alone are not parity. No whole field/result concatenation or old/new production result union is permitted.

Mutation released its bounded target after delegated checked-integer six and Store/SPR 101 passes. No Plugin/Flow/GIS build is included. After reading Flow VCS's actual Option-clearing mutation retirement, this lane handed the exact mutation-retirement struct/factory and mutation-owned collection/decode cleanup to Mutation; adjacent snapshot retirement and SharedRegistry/SetContributions remain protected. No compiler or publication permission follows from that ownership handoff. The demonstrator goal remains open: all six apps still need fresh consumed artifacts, visible content, meaningful interaction and exact close/reopen verification.
