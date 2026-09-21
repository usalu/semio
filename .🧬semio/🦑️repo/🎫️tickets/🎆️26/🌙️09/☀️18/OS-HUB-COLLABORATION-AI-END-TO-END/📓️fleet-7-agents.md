# Fleet 7 — agent handles (coordinator session 7 [5db45a], launched 2026-09-21 ~03:40)

Resume a cut worker with `SendMessage` to its id from THIS session (ids do not resolve elsewhere). Port plan: C5 hub **7621** (`jc1-boot`, 1.34 catalog) + serves 6190 (`s`) / 6191 (`gis2d`); PR1 hub **7611** (`gm1-boot`, HS1 binary) + serves 6194/6195; M8 hub **7631** (`hs1-boot`) ; S10 hub **7641** (`s10-boot`, fresh) + serve 6071; coordinator hub suite in `target-coordinator-hub`.

| slice | model | agent id | report |
|---|---|---|---|
| C5 | opus | a2edc67ac69581f8d | 📓️c5-live-collaboration-over-hub.md |
| PR1 | opus | a8637763dfb890879 | 📓️pr1-presence-roster-symmetry.md |
| M8 | opus | a7ed9e544ce195d1c | 📓️m8-mcp-agent-third-participant.md |
| S10 | opus | acd36fe8751d70fac | 📓️s10-s-host-studios-and-sweep.md |
| PZ1 | opus | a379219b86f6c9a8f | 📓️pz1-catalog-zero-diagnostics.md |
| U3 | opus | a9467e46b7819a318 | 📓️u3-mutation-label-localisation.md |
| HT13 | opus | ad0866e5850f2e826 | 📓️ht13-hub-suite-last-red.md |
| TC3 | opus | a02eb741a255f2ae6 | 📓️tc3-catalog-carried-genesis.md |
| RB1 | opus | a6b7ae4160bc5edbf | 📓️rb1-release-builds-and-production-posture.md |
| DB2 | opus | a5319e9c5a1f68ac1 | 📓️db2-postgres-neo4j-live-lanes.md |
| TC3b | opus | ac37ac1bf7d15ebfa | 📓️tc3b-catalog-genesis-landed.md |
| M9 | opus | afe50544c1bd003b7 | 📓️m9-agent-edits-hub-document.md |
| TC3c | opus | a964e4276094b6121 | 📓️tc3c-n-plugin-bootstrap-and-note-creation.md |
| U3b | opus | a50224610e207da9e | 📓️u3b-localized-label-tree-green.md |
| DB3 | opus | a8ee14bf5b48ece59 | 📓️db3-full-hub-on-postgres.md |
| HT14 | opus | a2cc6073bafaaf1ba | 📓️ht14-hub-reds-after-genesis-landing.md |
| FP4 | opus | a1ff1bdd23e28e7c2 | 📓️fp4-plugin-lib-gate.md |
| C6 | opus | ab2d4ea27cbe784fb | 📓️c6-browser-actor-load-and-scenario.md |
| FP5 | opus | ac00297852f7948fe | 📓️fp5-plugin-lib-zero-red.md |
| C7 | opus | a41716b4d2f6370df | 📓️c7-hub-document-mount-and-scenario.md |
| FP6 | opus | a6750f626500942f7 | 📓️fp6-plugin-lib-zero-red.md |
| JB1 | opus | a13b72604a1197a2d | 📓️jb1-builtin-jobs-in-production.md |
| FP7 | opus | a2deeefc34e45e908 | 📓️fp7-plugin-lib-zero-red.md |
| TC3d | opus | a770b58260db77123 | 📓️tc3d-guest-genesis-and-note-creation.md |
| FL2 | opus | abcf668a81cec2e91 | 📓️fl2-child-lane-replication-and-window-config.md |
| FP8 | opus | ab41e6c4672c1833c | 📓️fp8-plugin-lib-zero-red.md |
| KN1 | opus | a3b961a69c6588545 | 📓️kn1-kernel-lib-gate.md |
| FP9 | opus | ae9b7df07959ba13c | 📓️fp9-plugin-lib-zero-red.md |
| KN2 | opus | a3c7eb02ceb56d115 | 📓️kn2-kernel-lib-zero-red.md |
| G14 | sonnet | aecbba2d7d4a548d8 | 📋️g14-acceptance-ledger.md |
| G15 | sonnet | a449aa9e65495824d | 📓️g15-production-readiness-reaudit.md |
| G16 | sonnet | aed4d623e8b6b9f7f | 📓️g16-hub-backend-reaudit.md |

Coordinator hub run: `📜️coordinator-hub-run.sh s7a` (pid 83054, 03:42) → `🗑️generated/coordinator-hub-nextest-latest-s7a.txt`; HT13 launches on its Summary.
