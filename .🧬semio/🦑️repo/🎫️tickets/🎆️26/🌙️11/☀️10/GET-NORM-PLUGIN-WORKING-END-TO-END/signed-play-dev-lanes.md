# Signed Play development lanes

The previous Play 6034 verification recorded successful session, prepare and activate steps for DIN4108, EN1990, EN1992 and EN1995. DIN18599 ran after the Energy successor published. The previous results are in `🗑️generated/canonical-resumed-lanes.log`.

`verify-signed-play-lanes.py` reused the existing component verifier and native development lane. The passive check confirmed byte-for-byte agreement between served and installed receipts for the five representatives before waiting for a new Energy wrapper beyond `2067efa92cc266711ca3aef9a5355207e091f49b22bb0ed1fec854b289be7f98`. The waiter captures the three earlier Eurocode group receipts since that grouped publisher is active. If the new group is available when Energy settles, the workflow verifies all five representatives together. Otherwise, it verifies DIN18599 first and verifies EN1990, EN1992 and EN1995 after the grouped receipts advance.

Every replay verifies the wrapper and served browser bytes before and after session, prepare and activate. Signed receipt bytes must match across each phase. Per-route phase logs and exact receipt hashes are stored in `🗑️generated/signed-dev-lanes-*`. The waiter and replay use no browser and run only delivery and lane verification.

The workflow’s routing check passed with the grouped publication still pending, already published, and absent. A live preflight also matched the five installed receipt bodies with the five served receipt bodies at 6034.

## DIN18599

DIN18599 passed session at 03:12:13, prepare at 03:12:16 and activate at 03:12:19 local time on 2026-11-21. The published wrapper was `713c1b5fae81fdcd2e75e9b40101594515fddcb59b41997c6c1106c1a80c6dc2`. The served receipt, bridge, glue and core hashes agreed before and after lane activation. The receipt SHA-256 was `d8d1d564c3127aebac936d30008f8ca8041c873797e1ae03c79d3d54ad22a0ec`, and the core SHA-256 was `4d9f7de6b1d43034de8d2fd49f358a9cc37f2b571583641fddd2a933e35ac07a`.

See `🗑️generated/signed-dev-lanes-din18599-{steps,before,after}.log` and `🗑️generated/signed-dev-lanes-din18599-receipts.json`. The grouped successor did not advance the served receipts before its verification wait ended.

## Final representative verification

All five session, prepare and activate chains passed. The first four completed on 2026-11-20. Their component wrapper and source tree hashes match the contemporaneous component census `🗑️generated/verified-e55ba4b3-components.json` and the current live 6034 receipts. DIN18599 completed on 2026-11-21 with the replacement Energy wrapper and an exact receipt check before and after. An independent final delivery census matched the wrapper and all served bytes for each of the five currently installed components.

| Lane | Session | Prepare | Activate | Wrapper SHA-256 |
| --- | --- | --- | --- | --- |
| DIN4108 | 23:37:41 | 23:37:44 | 23:37:47 | `2ce4f85c3f769ca5f260eab4244ff6daa2d43673498a12e59b85feeb8008ef5c` |
| DIN18599 | 03:12:13 | 03:12:16 | 03:12:19 | `713c1b5fae81fdcd2e75e9b40101594515fddcb59b41997c6c1106c1a80c6dc2` |
| EN1990 | 23:37:49 | 23:37:53 | 23:37:55 | `cd30e0caa34c514e4c5f2d6f5c5ed4942a22959b41121a593bed1c370c6ffbc2` |
| EN1992 | 23:37:57 | 23:37:59 | 23:38:02 | `612b6afeb983341a19bd031b7a9c831c7421825c47e86ce8d5d6059c97c0ed6b` |
| EN1995 | 23:38:04 | 23:38:06 | 23:38:10 | `ba785ac9542ae3e6e310ddf71a010d89fa13e2fe6461bce466eae6b88e77157e` |

Times are local and mark exit zero. The final census is `🗑️generated/all-five-signed-delivery-census.log`. The exact receipts, asset digests, and step results are captured together in `🗑️generated/signed-dev-lanes-final-five.json`. Eurocode remains on the verified `wa041jP` releases. The optional `wB34Xuf` wait was stopped after confirming that the three installed receipts still match the verified release.
