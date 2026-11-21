# Native publication audit

Read-only inspection on 20 November around 23:38 CET. The canonical browser module root is `.cache/b/p/m/browser/prototype/modules`. Each of the five Norm `.semio-artifact.json` inventories has every named file, and each JCO component JavaScript references its own present core. The core is a transpiled artifact: the receipt's `wasmSha256` describes the original component, so its digest must not be compared to the core digest.

| Plugin | Canonical receipt component SHA-256 | Matching linked run | Core SHA-256 prefix | Publication time CET |
|---|---|---|---|---|
| norm | `2ce4f85c3f769ca5f260eab4244ff6daa2d43673498a12e59b85feeb8008ef5c` | `wKEwym4` | `f6af234d5956fac5` | 22:36 |
| norm-eurocode | `cd30e0caa34c514e4c5f2d6f5c5ed4942a22959b41121a593bed1c370c6ffbc2` | `wa041jP` | `d78a58639c5a6e8d` | 22:41 |
| norm-eurocode-structural | `612b6afeb983341a19bd031b7a9c831c7421825c47e86ce8d5d6059c97c0ed6b` | `wa041jP` | `40961c43db2616fd` | 22:46 |
| norm-eurocode-materials | `ba785ac9542ae3e6e310ddf71a010d89fa13e2fe6461bce466eae6b88e77157e` | `wa041jP` | `22a4c454081fd85a` | 22:30 |
| norm-energy | `2067efa92cc266711ca3aef9a5355207e091f49b22bb0ed1fec854b289be7f98` | older canonical receipt | `0c1f9b50cccd9c08` | 20:04 |

Direct SHA-256 of the linked run components establishes the four matches. `wKEwym4` and `wa041jP` reports record successful compiler passes and passed final provenance rechecks. Their wrappers exited with later retryable source-admission problems, so wrapper exit alone is not a publication verdict. The canonical receipt, run component digest, inventory and provenance report together establish these published component identities, not behavioral correctness or current source equivalence.

Energy `wdxiPax` linked component SHA-256 is `706607c073f102dadcd32705fde2524ddd619fe71ba2d91a5d291b3f8d484917`, which differs from the canonical Energy receipt. Its compiler pass succeeded but the report says final provenance recheck was not reached; the build log reports a source read extent mismatch and one failure. Its output is therefore not an authoritative successful publication. Do not substitute its linked or transpiled output into the canonical bundle; use the normal publication process to obtain a fresh validated Energy component before testing that fix in the canonical lane.

The coordinator's `canonical-resumed-lanes.log` records successful session, prepare and activation for DIN 4108, EN 1990, EN 1992 and EN 1995, each exit zero by 23:38:10. Activation confirms the lane setup only; browser interaction and successful archive transfer require separate evidence. No server, build, browser or activation was started for this audit.
