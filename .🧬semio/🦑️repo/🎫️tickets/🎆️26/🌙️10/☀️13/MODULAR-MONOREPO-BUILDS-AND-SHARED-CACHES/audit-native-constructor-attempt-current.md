# Independent Native Constructor Attempt Source Audit

Read-only Source review, 2026-10-16. No implementation edits, Native/provider execution, compiler, test, service, mount or other physical effect was performed. Review targets the current observed files; the checkpoint bytes are not certified because 15 of its 51 unique observed paths differ. Public descriptor import drift was traced: `🧰️framework/📦️packages/🟦️typescript/package.json:214` selects its descriptor facade, whose line 2 explicitly reexports `requireArtifactDescriptorClosureV1` from the original module. That facade adds no issuer or private owner.

## Actionable Findings

1. **Preserve the original Fixed preparation failure before observing backing.** `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🧱️fixed/🟦️.ts:167`: `platform.prepare` catches its original Error or undefined into `preparationError`, but the following `existsSync`/`lstatSync` and backing validation can throw before `if(preparationFailed)throw preparationError`. The enclosing catch consequently stores the later inspection error as `primary`. A failed prepare that leaves a changed, oversized or inaccessible backing demonstrates the Source path. Retain the first prepare failure immediately, perform subsequent evidence/settlement work with separately retained secondary failures, and add a case where preparation throws undefined followed by backing validation failure. This is a Source finding; it was not physically reproduced.

## Qualified Ordering Observation

**Requested genuine-control-before-DATA order is reversed, without an identified authority bypass.** `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🧹️cleanup/🎛️command/🟦️.ts:149` and `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🧱️fixed/🟦️.ts:231` invoke `nativeImagePreparationDecisionV1` before capturing and requiring the authentic live writer proof under `attempt.body`. The incoming attempts are private plain objects created by the authenticated constructor callbacks at command:120 and fixed:167; no caller-supplied getter or public attempt registrar reaches these handoff functions. Actual handoff remains after proof validation, so this is not an actionable authority bypass. It does fail the explicitly requested control-before-DATA ordering. Check private attempt receiver/guard identity and genuine body writer/control before constructing/reducing preparation DATA; retain the separate actual handoff rechecks.

## Supported Source Observations

Both constructor paths create private receiver-bound attempts inside existing local accepted cleanup enrollment, capture the original creator EX cleanup loan before creator effects, store and await ordinary preparation, and attempt success handoff before explicit terminal transition. macOS closes and joins actual body operations and commands (`command:54,78,120`); Fixed joins pending commands and retains recorded command failures (`fixed:167`). Both actual receiver completions release EX, acquire SH, accept the real borrower and recheck physical facts (`command:118`, `fixed:229`). Schema preparation DATA does not itself issue physical custody.

The explicit failed flag preserves thrown undefined in the outer attempt retain functions. Transition failure and later settlement are separate try blocks, so a transition failure does not skip the subsequent cleanup attempt. Failed-reader helpers retain/report secondary errors rather than replacing the primary (`command:122`, `fixed:105`); their descriptor-closure refusal remains conservative. EX-to-SH gap failure remains unresolved: the attempt loan is retained, but this slice supplies no safe recovery issuer. Parent-token/preappend association, actual preclaim consumption, Volume integration and parent retirement completion remain unimplemented as the checkpoint acknowledges.

## Evidence Limits

The checkpoint records Source test handles 9377/72183 as exit 0 with 37 tests/892 assertions, and strict handles 57277/77811 as exit 0. The named generated logs are absent from the ticket at review time. Their result/status assertions are therefore checkpoint evidence only, not independently reexecuted or terminal-status verified by this audit; an empty strict log would not prove success. No Native, compiler, runtime or cross-platform proof follows from this Source audit. Concurrent facade normalization qualifies the byte drift; it does not invalidate the concrete control flow findings above.

## Final Hash Observation

Initial check and final check both found 15 differing paths against the checkpoint. The two constructor hashes at initial and final read match each other: command `2c8395ede9535a5fc71d64809ab26489127ae620c13fa620d6b7667b60146cc3`; Fixed `16b8340fa9d4758648e9de75ed87621117d57e3ae9b962db9db3f967660e1f8a`. This does not certify all intermediate reads or concurrent files as frozen.

| Drifted Path | Checkpoint SHA-256 | Final Observed SHA-256 |
| --- | --- | --- |
| `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🧹️cleanup/🎛️command/🟦️.ts` | `4a7dabd7d1abb89738c4cb3cfb488fd0c87ed64ac1aefae87bc3f2ad57ad0983` | `2c8395ede9535a5fc71d64809ab26489127ae620c13fa620d6b7667b60146cc3` |
| `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🧱️fixed/🟦️.ts` | `5df6933cc41c7c932a805b9cb7f1ddd60662fdf41e5965f6f89e0e69690690f2` | `16b8340fa9d4758648e9de75ed87621117d57e3ae9b962db9db3f967660e1f8a` |
| `🧰️framework/🔨️modules/🏃️process/📦️artifacts/💾️cache/🧹️retirement/🧪️tests/🟦️.ts` | `3dff217c1a28ce534860075e3f7df6fe56abd36b7564776458cb40109e1ead27` | `9790e2cf1ac0346cad758f70affcb2cc8cf98e676472a3513596c84c3881c4f4` |
| `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🌱️birth/🧾️claim/🟦️.ts` | `33dec722f2597b1bdd18645bd2a4ea4c7555c473d3611d67dbdcdb2b91f369f0` | `512702f704204f6a7e5f3810fddee6a2de6cc53eed685e11c3b5faddd6386a0f` |
| `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🟦️.ts` | `0fc30fde879557b8b42efa7157791c65c8ce3c3cab083b37328e965c33ce0346` | `7ccfbf26a7743b3bbb992efc04ee35f81807b9a5e1f4c4884dcc005a6b996422` |
| `🧰️framework/🔨️modules/🏃️process/📦️artifacts/💾️cache/🧹️retirement/🧾️journal/🟦️.ts` | `5746097831ab26e0d61ad479cb70c8cb9f1094ff384693174e1e8ebbf86d1689` | `02b520029a3bd2a8d6b73731d5c7d863a56a5c67a5de8829fbab4d221f130ebd` |
| `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/📥️borrower/🟦️.ts` | `8842b932f693871f1e9545e274c7eaf68211e339c73a8b50e156617661e403fd` | `a7c27a9a0bd35105f2baeaf1c824fb98e56ca5dac849270be63f3d7da422b74d` |
| `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🧹️cleanup/♻️retirement/💾️durability/🟦️.ts` | `11dd71b8c2fe63760cf9339eda95f0e12ce094a924bf543bf552dadd19796ca7` | `20b3022c8370ce7da536a2a5e3727ea9800918a96f96a459afaa8be5744d87f9` |
| `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🍎️image/🟦️.ts` | `493356524ddbc13ef24a1e4d4a7fe4a17542b420e3e76bf13d03998575c565b2` | `195acc2593da6c8b751737a1c804264b47d538f3cfdc18e448cce370f0237d3d` |
| `🧰️framework/🔨️modules/🏃️process/💾️state/🧾️records/🟦️.ts` | `cd66522e5157848a49c748e3214401ce29fa319639d9e55e20706de92b681851` | `f1c56854721bb5c1be2f4a2df27534f64446a64390703d2f859cc49d2489bfe0` |
| `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🧾️metadata/🟦️.ts` | `77cc1d992a271ce7cdb17d72a3428f32c0255ada7e6a9d5214824fd718d43e12` | `3486b70ef912306efc2f1eb7cdb34d7a9149ccb55f20dc9fa8121596a99828e7` |
| `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🧾️receipt/📦️retention/📁️copy/🟦️.ts` | `95e3a99263460d41d6ec92f3849e62b7f0096f98fb0f02f414c590c1fc471aa8` | `4c4d1ec3eb1fcf585f7b499cd54bf8b3998090df2807c29c8298a4e24a1d5c17` |
| `🧰️framework/🛍️products/📓️print/📦️artifacts/📥️material/📤️producer/🟦️.ts` | `f1df14f945d211da7d5209f57c64db7f1d8d872218a6e537485d7568cfe43006` | `d0aa3d296bdd3062546d90933862e1615318fbc235a2d065fcf749f294c21fef` |
| `🌎️hub/🏗️bootstrap/🧾️provenance/📦️cargo/🧫️fixture/📥️reading/🟦️.ts` | `fc64eaaa14e87528bc8af03d3e185b38fe50d8e61acb92134cecbc979ca4623f` | `dc422360798dfbf302d24f61aa8643dac8d16c047a1882095632d1acb89b0781` |
| `🧰️framework/🔨️modules/📁️filesystem/🧾️observation/🧪️tests/🎛️control/🟦️.ts` | `2288bcb947a2dc3d0c377f5213a0b2b7e3656380b2813a2c27a393016d574bef` | `70b96db1c24d8e65ecf5de91f58c4d067425172676018c0e53427f64bb3f39e1` |
