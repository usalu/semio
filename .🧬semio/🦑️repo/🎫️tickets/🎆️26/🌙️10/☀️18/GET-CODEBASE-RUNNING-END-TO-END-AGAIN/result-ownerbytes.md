FOR coordinator: batch ready `C/ownerbytes-tools/staged/ownerbytes` (10 rows, all `base` on live at 12:30, no pending overlap).
- rebuild right before landing (live files are hot): `python3 -I C/ownerbytes-tools/build.py` (exact-string edits, stops loudly if an anchor moved; regenerates `staged/ownerbytes`)
- check / apply / revert: `python3 -I C/ownerbytes-tools/staged/ownerbytes/stage.py check|apply|revert` (root = live checkout by default, keeps file modes; law `python3 -I C/xplat5-tools/tests/test_stage_mode.py <stage.py>` OK)
- verify (live or mirror root, no mirror name needed): `sh C/ownerbytes-tools/staged/ownerbytes/verify.sh <root>` -> `VERIFY OK ownerbytes`; negative controls (mirror only): `... verify.sh <mirror-root> before`
- smoke after landing (read-only, no cache effect): `bun C/ownerbytes-tools/routes.ts <root>` -> 8 rows (`kind`/`target`), exit 0. Landing edits the revision reader schema: running processes report "Stale loaded Source revision reader; restart required" once, like every reader edit.

# Result ownerbytes: recovery owner byte ceiling vs. Source revision text ceiling

## Fault (confirmed)
`readNativeRepoRecoveryPlanV1` reads the recovery owner's `📋️project.json` through `captureBoundedSourceRevisionV1(path, protocol.maximumOwnerBytes, true)`. The reader refuses `retainText && maximumBytes > maximumTextBytes (65536)`; a file bigger than `maximumBytes` is refused too ("Bounded physical Source revision required"). Two drifts hit the same entry:
- 10-27 11:06: bootstrap `maximumOwnerBytes` 65536 -> 262144 > revision `maximumTextBytes` 65536 -> every route died with "Reducing finite Source revision request required".
- 10-28 12:12 (gate fix): ceiling back at 65536 -> frontend works, but `⚡️caching/📋️project.json` is 105,717 B -> the 6 maintenance/reset routes die with "Bounded physical Source revision required".
Nothing caught it: the bootstrap tests swap the reader for a fake in the oracle path, and the tests that call the real reader (arguments, maintenance, reset, capacity) were red and nobody saw it (7 of 9 bootstrap-cli tests and 1 of 2 arguments tests red on the mirror before the batch).

## Decision: raise the revision text ceiling to 256 KiB + a headroom law (not shrink, not a dedicated read)
Evidence on the owner (`⚡️caching/📋️project.json`):
- It is the manifest of the one Nx project `repo`: 208 targets; the 4 recovery targets are 3,561 B (3 %) of 103,603 compact bytes. Nx cannot split one project over two manifests; moving the recovery targets to a new project would rename the user-facing commands `repo:cache-reset-*`/`repo:cache-retire-inactive*` across docs/tests/hints.
- Duplication is real but authored: 59,754 B (58 %) are exact repeats of >= 60-char values (the `metadata.semio.cache` block 144x, `cwd` strings, input globs). Folding them into `targetDefaults` changes the inference input of all 208 targets -> a full graph re-inference (60-120 min per brief 1026g) for a stopgap that regrows. Rejected.
- It grows fast: git snapshots 16.8 KB -> 35 KB -> 65 KB -> 74 KB, and 105.7 KB in the working tree (+31 KB since the last commit). Any fixed ceiling is a tripwire problem, so the law, not the number, is the fix.
- Dedicated bounded owner read: the reader is the single custody for source text (descriptor identity, churn, ancestry, settle cache); a second reader would duplicate that and still need its own ceiling. A real structural alternative (recovery reads a small authored recovery manifest, the Nx project is checked against it by a law) changes the runtime "changed owner refuses before effects" contract and its 25+49 fault corpus; not needed while a tripwire exists. Follow-up candidate if the file keeps doubling.
- Memory: retained text exists only for explicit `retainText` requests (a dozen schemas + the owner, 105 KB, once per CLI process, 1 ms parse). Theoretical worst case of the settled cache is `maximumSettledRevisions (4096) x maximumTextBytes` = 1 GiB (was 256 MiB); in practice < 2 MB (few retained files). Flagged, not changed.

## What the batch changes (10 rows)
- `🏗️build/📥️inputs/🔏️revision/🧬️schema/🔣️.json`: `Policy.maximumTextBytes` 65536 -> 262144, root `text.maxLength` 262144, EN + DE `PolicyDescriptions.maximumTextBytes` say 256 KiB and now name project manifests ("…schemas, policy files and project manifests are read under this ceiling." / "…Schemas, Richtliniendateien und Projektmanifeste werden unter dieser Obergrenze gelesen.").
- revision tests: corpus `text-maximum` 262144, `over-text-request` 262145; the SQLite admission oracle takes the ceiling as a bound parameter from the schema instead of the literal 65536; the child-output cap of the windows law scales with the ceiling (`ceil(max/3)*4 + 4096`; the fixed 98304 refused the 350 KB base64 line of the new maximum row).
- `🚀️bootstrap/⌨️cli/🧬️schema/🔣️.json`: `Protocol.maximumOwnerBytes` 65536 -> 262144 (equal to the revision ceiling) and new `ownerHeadroomPercent: 25`.
- NEW law `🚀️bootstrap/⌨️cli/🧪️tests/📏️owner-bytes/` (`🟦️.ts`, corpus `🔣️.json` + `🧬️schema/🔣️.json`, `🐍️oracle/🐍️.py`) + a Scenario in `🥒️.feature`. Rule: `maximumOwnerBytes <= maximumTextBytes` AND for each distinct owner (frontend, maintenance, reset): `ownerBytes * 100 <= maximumOwnerBytes * (100 - ownerHeadroomPercent)`, i.e. the law goes red at 196,608 B while commands still work up to 262,144 B (today 105,717 B = 40 %: ~1.9x room). Three tests: corpus closed by schema; 9 language-agnostic cases (today's owner, exact/one-over headroom, empty, equal ceilings, the 10-27 state, the 10-28 state, zero/full headroom) decided identically by corpus, an independent Python reading and SQLite; the checkout law (Python reads both schemas and stats the manifests independently and must equal the module readings, every owner accepted, and the REAL reader returns a plan for all 8 routes).
- bootstrap-cli test (stale, not mine, masked by this fault): `runOwnedBuildInvocationV1` now ends in `return import("./🏛️sealed/📥️execution/🟦️.ts").then(...executeImplementationV1...)` (the sealed-execution rewiring); the test's source transform only rewrote `await import`/`=> import` and its `load` shim did not know the sealed module, so "capacity-admitted recovery …" failed with `Cannot find module './🏛️sealed/…'` as soon as the owner read stopped failing first. Fix: regex also rewrites `return import(`, and the shim maps the sealed module to the existing `executeCapturedBuildSourceV1` stub (2 edits, no assertion weakened).

## Proof (mirror `ob`, closure + `✏️s/🧑‍💻dev`; process-envelope runner; failure sets, not counts)
| suite | before | after |
|---|---|---|
| bootstrap ⌨️cli tests | 3 pass / 7 fail | 9 pass / 0 fail |
| bootstrap 🧮️arguments | 1 / 1 fail | 2 / 0 |
| owner-bytes law (new) | RED at both drift states | 3 / 0 |
| revision tests | 8 / 0 | 8 / 0 (own-stamp 2/0, stale-reader 8/0, 🔏️closure 1/0 unchanged) |
| 🧬️policy-literals law | - | 63 / 0 |
| `law_germanqa.py` | - | 0 findings |
| `inputs/📋️data` tests | 3 / 8 fail | 3 / 8 fail, same 8 names (pre-existing, not touched) |
Negative controls (`verify.sh <mirror> before`): bootstrap ceiling 65536 (gate state 10-28) -> law RED and only the 2 frontend routes enter; revision text ceiling 65536 under bootstrap 262144 (state 10-27) -> law RED. Each restored.
Real plan read of every route through the real reader (`bun C/ownerbytes-tools/routes.ts <root>`, exit 0):
`repo-mcp:build/install` (3,342 B), `repo:cache-retire-inactive-preview`, `repo:cache-retire-inactive`, `repo:cache-reset-graph|nx|rust|products` (each 105,717 B of 262,144) -> kinds frontend/maintenance/cache-reset, targets as authored.
`bun C/gate-g-tools/scan.ts` on the 3 changed/new TS: scanned 3, bad 0; 5 JSON parse, oracle py_compile ok. Live tree untouched (only `C/ownerbytes-tools`). Mirror dropped, `🗑️generated/ownerbytes` deleted.

## Overlap with other staged batches
Scanned every `manifest.json` under C (depth <= 6) for the 10 paths: `germanqa-tools/staged/libbuild` (revision schema) is LANDED (live == its `new`, my base is that file); `schemafirst-tools/staged/revision` (same file) and `backlog-tools/merge/backup/route2@live` (cli schema) are historical (live matches neither base nor new, drift-guarded). No pending batch touches these files. Re-run germanqa `rebuild_all` after landing is safe (it rebuilds from live; German law 0 findings with the new text).

## Observations for others
- The reset owner schema (`ResetOwner`) only requires `cache-reset-graph` and `cache-reset-nx`; the protocol also routes `cache-reset-rust` and `cache-reset-products`, which have NO authored target in `⚡️caching/📋️project.json` (the early native route is their only path, so the owner check validates nothing for them). Not changed here.
- If the manifest keeps growing, the law will go red at 196,608 B: then either raise BOTH ceilings together (the law forces that) or take the recovery declaration out of the whole-project manifest (follow-up above).
