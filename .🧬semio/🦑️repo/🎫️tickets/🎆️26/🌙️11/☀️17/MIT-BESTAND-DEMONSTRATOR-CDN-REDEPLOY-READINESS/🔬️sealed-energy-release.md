# Energie release verification

The existing queue run `wt48zqN` completed all 1,029 compiler units and all eleven release stages, including final provenance recheck. The outer command subsequently failed while reading a concurrently edited manifest. The build had already published the optimized module. The ticket verifier checks the closed compiler record, its source and component closure, the original staged Wasm, the descriptor, the two stage-cache transitions, the entire deployed module directory and the English and German static boot evidence before recording the exact canonical receipt.

| Artifact | SHA-256 |
| --- | --- |
| Raw component, 382,370,730 bytes | `4ed604e3f2b913be3718f3eac7c2bc361bbf9db8182c790cfea11712c55452f3` |
| Transpiled guest | `e06a97d96fc5b8785ecc2ae07345459195b6d95afc038361fb2cd1d4f9a016e4` |
| Published and static optimized guest | `32db45ad1079e2205425f17238a3de2b53e74f23ddccf1647242cadc8d66680f` |
| Descriptor | `f42992705c59a41853f9f4cbbea455d88a7ec2f8791ca468e4508f7a115facb8` |
| Source tree | `837f167d2e46e2031bf315bbd24204d9ab8885563920415dcdce2f0ac1296566` |
| Compiler closure | `d85eae02c1ada18f3c259c1230e29535db54368350d125e9a7f3fca008f925f0` |

All 37 deployed files matched the published module. Independent shasum verification matched the raw component, descriptor and optimized core. Four in-memory fault injections separately changed a stage outcome, source digest, custody settlement and optimized cache digest. Each was refused. The recorded receipt equals the published JSON. The most recent full static run passed Energie in English and German.

The 17:01 static rerun used the revised Playwright acceptance suite against the deployed Energy files over a dedicated HTTP origin. English passed in 12.6 seconds and German in 13.7 seconds. The origin served 107 GETs with zero agent-bridge requests. The two test records report actual passes and no skipped or flaky outcomes.
