# Actor Return Control Codec

## Current Boundary

The canonical fixed drive/control codec is implemented in actor/📤️return/🟦️component.ts and re-exported by ShardClient. The new file is included in the actor test target. There is no public dispatch, guest result, native owner or live renderer cutover in this checkpoint. The native/schema owner supplied the protocol and language-neutral fixtures; no parallel schema or compatibility branch was introduced.

The implementation validates exact positive u64 authority, safe transport request numbers, shortest ULEB128, canonical boolean/length constraints, truncation, overflow and trailing bytes. Decoding freezes only the small decoded records. This is format validation, not activation authority, immutable source provenance, bounded retirement or an 8 ms certificate.

The exact original transport request is distinct from Open/Close's semantic request. The updated canonical contract retains the last-admitted request frontier after tombstone eviction. Its eight replay transitions and fourteen ownership requirements remain acceptance requirements, not executed state-owner proof.

## Executed Evidence

- actor-return-drive-red-1: test syntax error before collection; not a behavioral RED.
- actor-return-drive-red-2: three failed / 98 skipped; two missing-codec failures and one strict Ajv schema error. The schema owner fixed the missing string type; strict mode was not relaxed.
- actor-return-drive-green-1: 101/101 in eight files, 2.44 s.
- actor-return-drive-green-2: 101/101 in eight files, 1.78 s, start 20:16:48.
- actor-return-drive-strict-1: the seven existing tutorial diagnostics plus one new invalid-boolean test cast. The test now declares its deliberately invalid inputs as unknown values.
- actor-return-drive-green-3: 101/101 in eight files, 3.12 s, start 20:20:12, exit 0.
- actor-return-drive-launch-1: the exact newly registered focused command passed three tests / 98 skipped in eight collected files, 1.46 s, start 20:25:18, exit 0.
- actor-return-drive-strict-2: exit 1; no return/Shard diagnostic. Seven tutorial diagnostics and the concurrent UI indexed-JSON constructor/method TDD diagnostics at UiDocumentStore 1187–1207 remain. This is not a full typecheck pass.

Logs are retained alongside this report as 🧪️actor-return-drive-{red,green,strict}-N.log. Three codec tests consume all six canonical drive vectors, eleven malformed vectors and maximum 43-byte framing. They use strict Ajv and an independent @webassemblyjs/leb128 implementation with Node Buffer input, plus offset views, every proper truncation and invalid object-value boundaries.

## Captured Hashes

| Source | SHA256 |
| --- | --- |
| Return TS | d5ca43731b5bdf781d7e802cb20c81ad8d193913add7848781962381630daf8b |
| ShardClient | b36b197b27a69fe9b644233a2473734d49588c85ba513c16bb1de0d207949b7d |
| Return schema | 0cf9197fa556d4c0b382465d825425f0787e6de4a718bc8f489efb7ee1db5bb1 |
| Return fixture | 7e75ffbce0eadc7ba189605f234b0ba5929ec7693ee1748b95faeb5714351ec3 |
| Return fixture schema | 7395952af17577d25e40d737b8d1a1d7ef50d2ae872717de06319a1f2a3bf45a |

The ffba2728→b36b197b Shard delta observed during the peer's full React R22 is solely the return codec/type re-export. It is not a scheduler, lifecycle or dispatch change. Peer-reported R22 628/628 does not establish a stable current Shard snapshot.

The runtime coordinator subsequently reported independent actor R15 101/101 in eight files, 1.43 s, start 20:21:39. That is delegated test evidence; its hash report was still pending at receipt.

## Coordination And Next Required Work

The canonical return report and all three updated schema/fixture files were read completely. Result vectors and the variable semantic-section grammar remain with the native/schema owner. Do not mount old/new result alternatives or a whole-copy interim bridge. The next integration must reserve original response ownership before dispatch, bind the frozen origin to the captured activation/worker, page typed native content before WIT lifting, and keep raw input ACK separate from semantic UI publication and instance-close ACK.

The canonical seed owner registered the exact focused ActorReturnDrive command as ⚖️gate🎭️actor📤️return-drive at 4_gate/400.4 in .vscode/🧩️launch.seed.jsonc and .vscode/launch.json, without executing it. Delegated pure-producer readback: output dc49a183e824547d8afe25b1bc08c2c186026c817dc699aee06dafba93e0432d; seed db9d99abd566d9006f1d5ee4345d6b2849eb9cae3e5799e8ca0608f690ed675d. No other row was changed by that registration.

The mutation owner released the preceding native slot after an OS-kernel no-run compile (50.50 s) and 94 selected tests: 89 passed, one failed, four not run. Its corrected Store text plus seven added SPR laws now have the next bounded jobs2/default-profile slot. This lane has started no Cargo build. All shared targets, logs and authored evidence remain preserved.

That next slot has also been explicitly released: delegated OS-kernel compile 32.858 s; 101 selected = 98 passed, one failed, two not run. AddN is now green; Store severity fixture rejects clean-n n=7. The mutation owner retains that vocabulary repair; it is not a lifecycle or Plugin failure. No Cargo build was started here.

The goal remains open: no fresh guest, browser window content, all-six-app interaction or complete close/reopen claim.
