# Direct Rustc State Independent Source Audit

2026-10-16. Read-only audit of frozen Source and the three requested ticket reports. No Native/image/cache/compiler/SDK/lease operation, physical test, Source replay, mock registrar, allocation or fresh capacity observation ran. Only this report was authored. All 29 frozen roster hashes still match.

## Findings

### P1 — Lower Volume Session Exports Unwrapped Live Native Authority

Location: `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📦️units/🏗️compiler/💾️state/💾️volume/🟦️.ts:36`, public receiver at line39.

The exported `withRustCompilerStateVolumeV1` passes a session to arbitrary callback code whose `image` is the actual `opened.image`; `reopen()` also resolves to the actual Native image. These are not private loan tokens or scoped method wrappers. The Native image contract exposes `port`, `close`, `recheck` and `withExclusiveMaintenance`. Only the session's `reopen` and `recheck` methods pass through its finite32 `accept()` enrollment; operations invoked on the exposed image bypass that session admission and pending set. A callback can close its owner's image early, invoke maintenance directly without the compiler state's descriptor/census transaction, or retain the live port/image while its surrounding Original remains live. Object.freeze on the outer session does not revoke these inner objects. This contradicts the reviewed lower-session requirement that callers receive scoped operations without live image/port authority. The current upper state owner is disciplined, but the exported receiver does not enforce that discipline.

Required correction: keep the image/port entirely private to the actual owner, expose only authenticated scoped operations, and make all effectful operations share the same admission/drain boundary. Do not replace it with structural public image registration.

### P1 — Concurrent Accepted Reopens Can Lose a Live Child Reader

Location: `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📦️units/🏗️compiler/💾️state/💾️volume/🟦️.ts:36`, `reopen` and `close` implementations.

`accept()` permits multiple pending `reopen()` operations and does not serialize them. Each executes `await current!.close()`, then awaits an independent `openBoundCompilerStateVolumeV1(...)`, then replaces the single `current` variable. Two calls admitted before either close finishes can both close the same old image and independently open the same persistent child. When their opens resolve, the latter assignment discards the first new image handle. Both promises leave `pending`, so `session.close()` subsequently closes only the final `current` plus the parent. Nothing retains/closes the other successfully opened child reader. Canonical operation drainage waits for the open promises; it does not close a discarded returned image. This can leak child SH custody beyond parent release and make later child EX maintenance refuse or wait. The existing upper caller awaits its single reopen, but the lower exported contract admits this interleaving.

Required correction: serialize image replacement, reject overlapping reopen, or retain every acquired reader until exact settlement. The admission cap alone is insufficient.

## Other Scoped Review Results

No additional concrete defect was established in the inspected charge/intent relation, privately copied policy, authenticated malformed-warm reset branch, maintenance EX/FD/fresh recensus ordering, positive environment receiver, metadata/content identity separation, or current Script's compile-through-publication callback. The prior owner-name adoption and public policy mutation findings appear corrected in this frozen Source: existing unbound candidates refuse, and primitive policy is captured independently.

Known frontiers remain unchanged: prototype physical state has no current qualification; release temporary-v1 keeps repository cwd; generic Cargo cfg is separate; Windows complete descriptor inventory refuses; global Rust writer and CPU1 remain; full census costs remain. Observed Rust free4,325,548,032B cannot fund the initial8,589,934,592B predicate. No runtime or fresh funding success is inferred.

Existing recorded evidence remains state BOTH10/123, context/profile/unit regressions BOTH15/561, expanded strict BOTH0, with prior child handles terminal. This audit did not rerun them.

## Exact Frozen Source Hash Verification

| Repository Path | Bytes | SHA256 | Frozen Comparison |
| --- | ---: | --- | --- |
| `.vscode/launch.json` | 8164600 | `0297c871e97e2639e0d28ff26578754fb6db37f9e2688c978fc7079f1dd0f39f` | matches |
| `.vscode/🧩️launch.seed.jsonc` | 6407571 | `a638a6abc3899c0e79cec3b2f927e1c3a4e63865eaeb2493a75c37a5031d55a0` | matches |
| `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️13/MODULAR-MONOREPO-BUILDS-AND-SHARED-CACHES/build-mode/📜️script.ts` | 7212 | `c351ec2b0cb59f53d77b510761b39e65eb39833ef27e9dd534c32917ccd86402` | matches |
| `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🟦️.ts` | 17027 | `e2fd43dc1db29d0888230f17bfc214263d5456d64880c9adaea6751a9089410f` | matches |
| `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🧬️schema/🔣️.json` | 8820 | `3d8e8d2d24faf9512ec7fd0fa753c69db5efc1f664a9bbad39f6520ae54614f3` | matches |
| `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🕸️units/📐️profile/🧪️tests/🟦️.ts` | 13256 | `306331bd579a5d490ac8cfa9f0e19308cd8da9d5b7c0a961a37bcd5812f92591` | matches |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/🏗️compiler/📏️capacity/🟦️.ts` | 8519 | `7741359351364a92a1437eff25b85dadcf1b5222fe639784dc0318ef0b491ac0` | matches |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📦️units/🏗️compiler/💾️state/⚙️policy/🟦️.ts` | 1319 | `2da5419f6f275647e6c198ece2c71ead057a14f2a25877f0c250a08d67fdda1b` | matches |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📦️units/🏗️compiler/💾️state/🌱️environment/🟦️.ts` | 1405 | `5760bed9d41a1f79071f800cf67782d42566894d2d5988eb2990ffb873652436` | matches |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📦️units/🏗️compiler/💾️state/💾️volume/🟦️.ts` | 12860 | `b32e6f17c379b2f49cebc6e5876f42fea60878dfae65a37490d635c2b9fd4994` | matches |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📦️units/🏗️compiler/💾️state/📋️model/🟦️.ts` | 4469 | `9d730037dbca249fff77f22047b7458ae6015fae47e95ada190b6e1be0e412cf` | matches |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📦️units/🏗️compiler/💾️state/📍️layout/🟦️.ts` | 1791 | `88b2601395edd49be833088977ab3240658699bcb7dedac975b90367fdac9e14` | matches |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📦️units/🏗️compiler/💾️state/📏️inventory/🟦️.ts` | 4472 | `ed3528862f14677fd27dc01ca3a23eed152e1e2892ad3d93f4c6ef590516e9f5` | matches |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📦️units/🏗️compiler/💾️state/🔣️policy.json` | 626 | `fc181c69584948a8517375a05ed5a70a7b65136a68f25b01505e4d1f37468efc` | matches |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📦️units/🏗️compiler/💾️state/🟦️.ts` | 13170 | `fbd7c55d55017956098b60d19cda42032d0814fd6a228e41bdf6dfa722861823` | matches |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📦️units/🏗️compiler/💾️state/🧪️tests/🔣️.json` | 18754 | `7125758b183f21d58d771e16f2fa6cabc3aef0616a43f2e582f13b6f408e084c` | matches |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📦️units/🏗️compiler/💾️state/🧪️tests/🟦️.ts` | 12218 | `17cca8a4043bd526bb70d02855911798f2a44768c6820899c22c9ca1fbbe2e12` | matches |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📦️units/🏗️compiler/💾️state/🧬️schema/🔣️.json` | 10395 | `15a21622827244fe163cb5d7c2e6093b5d7046074b0149adc3990cc1492698e5` | matches |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📦️units/🏗️compiler/🦀️rustc/🟦️.ts` | 9735 | `ae0bae5b3cd9bf74564432963aee2767eb9075b8bcbec6e15aa29f176236c7ea` | matches |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📦️units/📜️script.ts` | 12429 | `8bb2a8c73d96f52bc23b053e7ee9cfbcc299b405391879149acc52dcf6a7643d` | matches |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📦️units/📥️descriptor/🕸️graph/🟦️.ts` | 9669 | `009c0de109f01ae49666dc24dfae4608860d5f9fa2bf2e14ffad79436c578beb` | matches |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📦️units/📥️descriptor/🟦️.ts` | 7145 | `988f788dd3fb6b76a4d0d3b52cdbae1a57cb6b00cdaf0ae35ef65ec85053b15d` | matches |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📦️units/🟦️.ts` | 11561 | `5ee8c6295e080ba9eb679c815c8583e3058699ed06bae6be16d2065806b95a3a` | matches |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📦️units/🧪️tests/🏗️compiler/🟦️.ts` | 2337 | `2afeb9704bd9cc450e26010b97765180caa0cbc40a2233d8cc5043d9751345c5` | matches |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📦️units/🧪️tests/🟦️.ts` | 81495 | `03e7d3fe6ed78a5306e4557eeb951e21d9e3108d675c93adf94496e754db6fa8` | matches |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📦️units/🧪️tests/🧫️fixtures/🔣️.json` | 15529 | `285b8f16b64220b5e7f708969b91a5d46e891e0ea94c5d2031c3434dee0db93d` | matches |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📦️units/🧬️schema/🔣️.json` | 11774 | `f436cd95a3d9306bf96dccd12a68c8b1a3c12792fb5096cf603067688f2ccddf` | matches |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📦️units/🧭️context/🟦️.ts` | 2578 | `d4209f145b7ecb8200499faa1752ced1a2487e85d9541c9c6f8276dc67d7b455` | matches |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📦️units/🧭️context/🧬️schema/🔣️.json` | 17011 | `7ceedac70b52fb2b95af791268dd15e5a7a0a0ab31506384bda51f81814e5e04` | matches |
