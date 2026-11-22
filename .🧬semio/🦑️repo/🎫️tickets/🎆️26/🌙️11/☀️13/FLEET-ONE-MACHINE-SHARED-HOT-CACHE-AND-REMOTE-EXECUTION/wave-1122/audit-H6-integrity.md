# Audit H6 (integrity) of wave 1122

Read-only. No source file changed. Scope: the 81 `## Wave 1122 ...` and `## Wave 1122 coordinator ...` sections of `T/landings.md` (lines 5982 to end, 2026-11-21 23:56Z to 03:3x CET 2026-11-22).

## Method

1. Extracted every file reference with a hex sha256 prefix (before and after forms, `X -> Y`, bare hash) from the wave sections. 228 hashed references. Paths were abbreviated in the landings; each reference was matched to live files by basename plus the longest matching directory suffix. 28 references (short basenames such as `🟦️.ts` with no distinguishing directory) stayed ambiguous and are not in the table.
2. Live sha256 of each matched file compared with the prefix recorded in each landing section (`shasum`-equivalent, sha256 of the file bytes).
3. Where a live file differed from its latest landed `after`, the file was checked for: (a) a later section listing a hash that matches the live file (the parser misses some prose-only records), (b) the live sha matching an earlier landed `after` (a whole-file revert), (c) identifiers named in the landing text for that file, grepped in the live file.
4. Identifiers were checked per landing segment and per file. Identifiers that the landing text places in a different file (for example `loadSlice` in the planner port, not in the host) were not counted as missing.

## Table (files with a sha256 in the wave sections)

Verdicts: OK = live equals the latest landed sha; changed-by-later = live equals the latest landed sha but an earlier landing of the same file differs (expected); OK (recorded in a later section) = parser missed a later hashed record that matches live; DRIFT = live differs from the latest landed sha and no section lists the live sha, so an unrecorded edit happened after the last landing (mtime given). No file reverted to an earlier landed sha (checked for all 132 rows).

| File (abbrev.) | Latest landed after (sha256 prefix) | Live sha256 prefix | Live mtime | Landings listing the file | Verdict |
|---|---|---|---|---|---|
| `B-mod:🏗️build/⌨️command/🌐️host/🟦️.ts` | 0f08147bd164 | 618bf2b7022c | 2026-11-22 02:54 | E14 round 5: slice pool size, E17: interactive priority th | DRIFT: live differs, no later section lists it (mtime shows a later edit) |
| `B-mod:🏗️build/🌐️remote-cache/💾️store/🟦️.ts` | 81766b95 | e4cca3a2a6b1 | 2026-11-22 03:28 | E5 round 2: fast, graceful c, E5 round 4: pressure leads t, E5 round 6: file descriptor  | DRIFT: live differs, no later section lists it (mtime shows a later edit) |
| `B-mod:🏗️build/🌐️remote-cache/🖥️server/🟦️.ts` | 642d4171 | 86a90b53829b | 2026-11-22 03:28 | E3 round 2: refused-result e, E3 round 4: newest record wi, E5 round 2: fast, graceful c, E5 round 3: zero-downtime ca, E5 round 6: file descriptor  | DRIFT: live differs, no later section lists it (mtime shows a later edit) |
| `B-mod:🏗️build/🌐️remote-cache/🖥️server/🧪️tests/🟦️.ts` | d0a13b5e | ee5acaef366d | 2026-11-22 02:28 | E5 round 2: fast, graceful c | DRIFT: live differs, no later section lists it (mtime shows a later edit) |
| `B-mod:🏗️build/🌐️remote-cache/🧪️tests/🟦️.ts` | 3f1dbe95 | 55f71351e571 | 2026-11-22 03:00 | E10 round 4: zero-touch Baze, E10: real-Nx test builds a p, E3 round 3: `repo remote-cac | DRIFT: live differs, no later section lists it (mtime shows a later edit) |
| `B-mod:🏗️build/🌐️remote-execution/🗓️scheduler/🧪️tests/🟦️.ts` | bd19e819 | b10193fa0c71 | 2026-11-22 03:05 | E5 round 5: one idle truth f | DRIFT: live differs, no later section lists it (mtime shows a later edit) |
| `B-mod:🏗️build/🌐️remote-execution/🛰️sidecar/🚦️ready/🟦️.ts` | 6686f8665b20 | 460b80ca7cc6 | 2026-11-22 01:25 | E13 round 2: volume facts in, E13: bounded same-checkout s | DRIFT: live differs, no later section lists it (mtime shows a later edit) |
| `B-mod:🏗️build/🌐️remote-execution/🛰️sidecar/🟦️.ts` | 6cb19f9e | 27f97fbf6905 | 2026-11-22 03:21 | E5 round 2: fast, graceful c, E5 round 3: zero-downtime ca, E5 round 5: handover retries, E5 round 6d: the restart of  | DRIFT: live differs, no later section lists it (mtime shows a later edit) |
| `B-mod:🏗️build/🌐️remote-execution/🛰️sidecar/🧪️tests/🔁️restart/🟦️.ts` | 7c1a6787 | 5d190518bb15 | 2026-11-22 03:01 | E5 round 2: fast, graceful c | DRIFT: live differs, no later section lists it (mtime shows a later edit) |
| `B-mod:🏗️build/🌡️host-capacity/🧪️tests/🔓️sealed-read/🟦️.ts` | 3e421237 | db99bfab3f16 | 2026-11-22 03:13 | E18: learned estimates from  | DRIFT: live differs, no later section lists it (mtime shows a later edit) |
| `B-mod:🏗️build/🏡️resident/🏗️composition/🌐️host/👷️slice-pool/🟨️.mjs` | 0de339905dba | 2b4ecc1aefac | 2026-11-22 03:18 | E13: bounded same-checkout s, E17: interactive priority th | DRIFT: live differs, no later section lists it (mtime shows a later edit) |
| `B-mod:🏗️build/🏡️resident/🏛️service/🌐️host/🟨️.mjs` | 47c6ed72e918 | 4ede3394f97b | 2026-11-22 02:54 | E17: interactive priority th | DRIFT: live differs, no later section lists it (mtime shows a later edit) |
| `B-mod:🏗️build/🏡️resident/🔌️ports/🧭️plan/🟨️.mjs` | 7f5d90e66c27 | 5ffc352cdf33 | 2026-11-22 02:54 | E17: interactive priority th | DRIFT: live differs, no later section lists it (mtime shows a later edit) |
| `B-mod:🏗️build/🏡️resident/🧪️tests/🏛️service/🟦️.ts` | ab8ae369b96e | 290d8db11389 | 2026-11-22 01:38 | E13 round 3: service watcher | DRIFT: live differs, no later section lists it |
| `B-mod:🏗️build/👁️observation/📥️reuse/🟨️.mjs` | 854371d8 | b265acc2cd85 | 2026-11-22 03:22 | E1 round 2: presence session | DRIFT: live differs, no later section lists it (mtime shows a later edit) |
| `B-mod:🏗️build/👁️observation/🟨️.d.mts` | 4ff04acf | c0a94a22ecb5 | 2026-11-22 02:28 | E1 round 3: owner record key | DRIFT: live differs, no later section lists it (mtime shows a later edit) |
| `B-mod:🏗️build/👁️observation/🟨️.mjs` | eb746f4cc3e2 | b77fc24ea984 | 2026-11-22 03:28 | E1 round 3: owner record key, E3 round 3: `repo remote-cac | DRIFT: live differs, no later section lists it (mtime shows a later edit) |
| `B-mod:🏗️build/📏️incremental/🏎️bench/🎯️slo/🔣️.json` | 606357f2 | 3ed9492fb771 | 2026-11-22 02:14 | E10: real-Nx test builds a p, E12 round 2: MCP value-set t, E12: fleet SLO probes and de | DRIFT: live differs, no later section lists it (mtime shows a later edit) |
| `B-mod:🏗️build/📏️incremental/🏎️bench/🟦️.ts` | 8eb8c3821131 | dbd7c3576833 | 2026-11-22 02:13 | E12: fleet SLO probes and de | DRIFT: live differs, no later section lists it (mtime shows a later edit) |
| `B-mod:🏗️build/🕸️graph/📥️inference/📦️project-nodes/🌐️owner-pool/🔬️inference/🟨️.mjs` | 40b61562 | 4498dc4cc799 | 2026-11-22 02:52 | E1 round 4: canonical row or | DRIFT: live differs, no later section lists it (mtime shows a later edit) |
| `B-mod:🏗️build/🧷️format-stamps/🧬️schema/🔣️.json` | e3cb1bcc3c5a | e35156b49e4d | 2026-11-22 02:31 | E14: doctor image repair and | DRIFT: live differs, no later section lists it (mtime shows a later edit) |
| `B-mod:🏗️build/🩺️doctor/🔎️probes/💾️storage/🟦️.ts` | ae05c9cb5bbe | b1d9bfa1fdb5 | 2026-11-22 01:18 | E14: doctor image repair and | DRIFT: live differs, no later section lists it (mtime shows a later edit) |
| `B-mod:🏗️build/🩺️doctor/🔣️policy.json` | a1721f2283d1 | 6464bc120e90 | 2026-11-22 01:11 | E11: tools provisioning, sch | DRIFT: live differs, no later section lists it (mtime shows a later edit) |
| `B-mod:🏗️build/🩺️doctor/🧮️analysis/🟦️.ts` | 1c5f10e92100 | db73b3249ef2 | 2026-11-22 01:06 | E11: tools provisioning, sch | DRIFT: live differs, no later section lists it (mtime shows a later edit) |
| `HC-mod:📦️artifacts/🏗️native-build/💾️storage/🍎️image/♻️reclaim/🟦️.ts` | b2924b8d1e17 | 9bb8739c4b6e | 2026-11-22 02:28 | E14: doctor image repair and | DRIFT: live differs, no later section lists it (mtime shows a later edit) |
| `HC-mod:🧰️tools/🔣️.json` | 59638b8b8f18 | b2562036912e | 2026-11-22 02:55 | E11: tools provisioning, sch | DRIFT: live differs, no later section lists it (mtime shows a later edit) |
| `B-mod:⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/♻️reset/🟦️.ts` | bddaf7194ffb | bddaf7194ffb | 2026-11-22 01:15 | E14: doctor image repair and | OK |
| `B-mod:⚡️caching/🦀️cargo/🗃️unit-cache/📜️script.ts` | d00edc72 | 35a474ec497c | 2026-11-22 03:06 | E5 round 2: fast, graceful c, E5 round 4: pressure leads t | OK (recorded in a later section: E3 round 5 (35a474ec497c)) |
| `B-mod:⚡️caching/🦀️cargo/🗃️unit-cache/🔏️key/🔗️sources/🟦️.ts` | 411d1240d6fa | 411d1240d6fa | 2026-11-22 02:53 | E3 round 5: the unit key cov | OK |
| `B-mod:⚡️caching/🦀️cargo/🗃️unit-cache/🔏️key/🔗️sources/🧪️tests/🟦️.ts` | 85e32d1044f8 | 85e32d1044f8 | 2026-11-22 03:05 | E3 round 5: the unit key cov | OK |
| `B-mod:⚡️caching/🦀️cargo/🗃️unit-cache/🚪️launch/🟦️.ts` | 1b73008e | 1b73008e26f6 | 2026-11-22 01:26 | E5 round 4: pressure leads t | OK |
| `B-mod:⚡️caching/🦀️cargo/🗃️unit-cache/🧭️remap/🟦️.ts` | b179ca2542ac | b179ca2542ac | 2026-11-22 02:18 | E3 round 4c: the standard li | OK |
| `B-mod:⚡️caching/🦀️cargo/🗃️unit-cache/🧭️remap/🧪️tests/🟦️.ts` | bda0c002f3a4 | bda0c002f3a4 | 2026-11-22 02:18 | E3 round 4c: the standard li | OK |
| `B-mod:🏗️build/⚙️operations/🔣️catalog.json` | a348fe3dfd69 | a348fe3dfd69 | 2026-11-22 01:53 | E12: fleet SLO probes and de, E3 round 3: `repo remote-cac | changed-by-later (live = latest landing) |
| `B-mod:🏗️build/⚙️operations/🧪️tests/🧫️fixtures/📖️help/🔣️.json` | 0f6324c2208f | 0f6324c2208f | 2026-11-22 01:53 | E12: fleet SLO probes and de, E3 round 3: `repo remote-cac | changed-by-later (live = latest landing) |
| `B-mod:🏗️build/🌐️remote-cache/⌨️command/🟦️.ts` | 5d45a9c4ca44 | 5d45a9c4ca44 | 2026-11-22 01:53 | E3 round 3: `repo remote-cac | OK |
| `B-mod:🏗️build/🌐️remote-cache/🏷️retention/🟦️.ts` | 4aeeb83f | 4aeeb83fc142 | 2026-11-22 01:34 | E5 round 2: fast, graceful c, E5 round 4: pressure leads t | changed-by-later (live = latest landing) |
| `B-mod:🏗️build/🌐️remote-cache/📏️fd-budget/🟦️.ts` | 6a1ba6b0 | 6a1ba6b0c516 | 2026-11-22 02:28 | E5 round 6: file descriptor  | OK |
| `B-mod:🏗️build/🌐️remote-cache/📖️texts/🔣️.json` | 8f843f4c94eb | 8f843f4c94eb | 2026-11-22 01:53 | E3 round 3: `repo remote-cac | OK |
| `B-mod:🏗️build/🌐️remote-cache/🔬️action-result/🟦️.ts` | 60b13d5bc78d | 60b13d5bc78d | 2026-11-22 01:23 | E3: AC divergences classifie | OK |
| `B-mod:🏗️build/🌐️remote-cache/🛰️fleet/🧪️tests/🟦️.ts` | edccae12 | edccae12703c | 2026-11-22 02:14 | E10 round 4: zero-touch Baze | OK |
| `B-mod:🏗️build/🌐️remote-cache/🛰️fleet/🪪️identity/🧬️schema/🔣️.json` | 502735be81a1 | 502735be81a1 | 2026-11-22 01:23 | E3: AC divergences classifie | OK |
| `B-mod:🏗️build/🌐️remote-cache/🧶️nx/🚦️single-flight/🟦️.ts` | dac64efc | dac64efcbad5 | 2026-11-22 02:19 | E10 round 5: Nx front down-w | OK |
| `B-mod:🏗️build/🌐️remote-cache/🧶️nx/🟦️.ts` | 9a49baec | 9a49baece9a9 | 2026-11-22 02:14 | E10 round 4: zero-touch Baze | OK |
| `B-mod:🏗️build/🌐️remote-cache/🧶️nx/🧰️scratch/🟦️.ts` | b9a84473 | b9a844731b36 | 2026-11-22 02:01 | E10: real-Nx test builds a p | OK |
| `B-mod:🏗️build/🌐️remote-cache/🧹️collector/🟦️.ts` | 64bc98ef | 64bc98efed78 | 2026-11-22 01:34 | E5 round 2: fast, graceful c, E5 round 4: pressure leads t | changed-by-later (live = latest landing) |
| `B-mod:🏗️build/🌐️remote-cache/🧾️diff/🟦️.ts` | 7d47b7944ccf | 7d47b7944ccf | 2026-11-22 01:53 | E3 round 3: `repo remote-cac | OK |
| `B-mod:🏗️build/🌐️remote-cache/🧾️provenance/🗂️evidence/🟦️.ts` | 2a1ce219d628 | 2a1ce219d628 | 2026-11-22 02:09 | E3 round 2: refused-result e, E3 round 4: newest record wi | changed-by-later (live = latest landing) |
| `B-mod:🏗️build/🌐️remote-cache/🧾️provenance/🟦️.ts` | faefc39ef8e0 | faefc39ef8e0 | 2026-11-22 02:09 | E3 round 2: refused-result e, E3 round 4: newest record wi | changed-by-later (live = latest landing) |
| `B-mod:🏗️build/🌐️remote-cache/🧾️provenance/🧪️tests/🐍️.py` | cf92784ae23a | cf92784ae23a | 2026-11-22 01:24 | E3: AC divergences classifie | OK |
| `B-mod:🏗️build/🌐️remote-cache/🧾️provenance/🧪️tests/🟦️.ts` | 6e28b606ff72 | 6e28b606ff72 | 2026-11-22 02:09 | E3 round 2: refused-result e, E3 round 4: newest record wi, E3: AC divergences classifie | changed-by-later (live = latest landing) |
| `B-mod:🏗️build/🌐️remote-cache/🧾️provenance/🧪️tests/🧮️by-class/🟦️.ts` | a30b6460c085 | a30b6460c085 | 2026-11-22 02:09 | E3 round 2: refused-result e, E3 round 4: newest record wi, E3: AC divergences classifie | changed-by-later (live = latest landing) |
| `B-mod:🏗️build/🌐️remote-execution/🖥️server/🟦️.ts` | b9b32574 | b9b32574ce88 | 2026-11-22 02:13 | E5 round 5: one idle truth f | OK |
| `B-mod:🏗️build/🌐️remote-execution/🗓️scheduler/⚖️placement/🟦️.ts` | e03250b9 | e03250b98729 | 2026-11-22 02:13 | E5 round 5: one idle truth f | OK |
| `B-mod:🏗️build/🌐️remote-execution/🗓️scheduler/🟦️.ts` | b32aece9 | b32aece991e4 | 2026-11-22 03:05 | E5 round 5: one idle truth f, E5 round 6b: the idle answer, E5 round 7: a refused offloa | changed-by-later (live = latest landing) |
| `B-mod:🏗️build/🌐️remote-execution/🚀️offload/📜️action/🟦️.ts` | 26f98d80 | 26f98d801064 | 2026-11-22 01:07 | E5 round 2: fast, graceful c | OK |
| `B-mod:🏗️build/🌐️remote-execution/🚀️offload/🗺️paths/🟦️.ts` | 3ad3a297 | 3ad3a297feb3 | 2026-11-22 01:00 | E5 round 2: fast, graceful c, E5: offload planner carries  | changed-by-later (live = latest landing) |
| `B-mod:🏗️build/🌐️remote-execution/🚀️offload/🚦️admission/🟦️.ts` | b53fc1e9 | b53fc1e9d274 | 2026-11-22 02:13 | E5 round 4: pressure leads t, E5 round 5: one idle truth f | changed-by-later (live = latest landing) |
| `B-mod:🏗️build/🌐️remote-execution/🚀️offload/🧪️tests/🌐️cluster/🟦️.ts` | 5332dc3d | 5332dc3dda82 | 2026-11-22 01:31 | E5 round 4: pressure leads t | OK |
| `B-mod:🏗️build/🌐️remote-execution/🚀️offload/🧪️tests/🟦️.ts` | 7d4de824 | 7d4de824d4d2 | 2026-11-22 02:14 | E5 round 4: pressure leads t, E5 round 5: one idle truth f | changed-by-later (live = latest landing) |
| `B-mod:🏗️build/🌐️remote-execution/🚀️offload/🧬️schema/🔣️.json` | bdc67c2d | bdc67c2ddad4 | 2026-11-22 01:26 | E5 round 2: fast, graceful c, E5 round 4: pressure leads t | changed-by-later (live = latest landing) |
| `B-mod:🏗️build/🌐️remote-execution/🚀️offload/🧰️toolchain/🟦️.ts` | 6f3b5466 | 6f3b54665ccd | 2026-11-22 01:07 | E5 round 2: fast, graceful c | OK |
| `B-mod:🏗️build/🌐️remote-execution/🛰️sidecar/🗄️cache/🟦️.ts` | 5d204e25 | 5d204e253543 | 2026-11-22 02:44 | E5 round 2: fast, graceful c, E5 round 3: zero-downtime ca, E5 round 6c: the cache says  | changed-by-later (live = latest landing) |
| `B-mod:🏗️build/🌐️remote-execution/🛰️sidecar/🧰️shared/🟦️.ts` | 03fc4e53 | 03fc4e536818 | 2026-11-22 02:44 | E5 round 6c: the cache says  | OK |
| `B-mod:🏗️build/🌡️host-capacity/🟦️.ts` | 70c9a65a | 38badfc9b176 | 2026-11-22 03:23 | E18: learned estimates from  | OK (recorded in a later section: E18 round 2 (38badfc9)) |
| `B-mod:🏗️build/🌡️host-capacity/🧪️tests/📉️shrink/🟦️.ts` | 46961faa | 46961faa4a9b | 2026-11-22 03:09 | E18: learned estimates from  | OK |
| `B-mod:🏗️build/🌡️host-capacity/🧪️tests/🟦️.ts` | 21a019b3 | 21a019b38471 | 2026-11-22 03:05 | E18: learned estimates from  | OK |
| `B-mod:🏗️build/🌡️host-capacity/🧪️tests/🦀️cargo-scaling/🟦️.ts` | 5b33408e | 5b33408ea817 | 2026-11-22 02:14 | E5 round 4: pressure leads t, E5 round 5: one idle truth f | changed-by-later (live = latest landing) |
| `B-mod:🏗️build/🌡️host-capacity/🧪️tests/🦀️cargo-scaling/🧫️fixtures/🔣️.json` | 2669f842 | 2669f8420dd4 | 2026-11-22 01:27 | E5 round 4: pressure leads t | OK |
| `B-mod:🏗️build/🌡️host-capacity/🧹️disk-recovery/🔌️real/🟦️.ts` | 99c0de434570 | 99c0de434570 | 2026-11-22 01:11 | E13: bounded same-checkout s | OK |
| `B-mod:🏗️build/🏃️execution/🧬️schema/🌐️remote-cache.json` | 116f6c270b2d | 116f6c270b2d | 2026-11-22 02:09 | E3 round 2: refused-result e, E3 round 3: `repo remote-cac, E3 round 4: newest record wi | changed-by-later (live = latest landing) |
| `B-mod:🏗️build/🏡️resident/🏛️service/🟨️.mjs` | 168b4b0d7d29 | 168b4b0d7d29 | 2026-11-22 01:32 | E13 round 3: service watcher | OK |
| `B-mod:🏗️build/🏡️resident/🏛️service/🧯️failure/🟨️.mjs` | 3555072aec72 | 3555072aec72 | 2026-11-22 01:35 | E13 round 2: volume facts in, E13 round 3: service watcher | changed-by-later (live = latest landing) |
| `B-mod:🏗️build/👁️observation/📂️source-names/🟨️.mjs` | 0c5be5f8 | 0c5be5f8f66e | 2026-11-22 00:51 | E1: slice looks 12x cheaper  | OK |
| `B-mod:🏗️build/👁️observation/📥️reuse/🧬️schema/🔣️.json` | aa91280b | aa91280b67d7 | 2026-11-22 00:57 | E1 round 2: presence session | OK |
| `B-mod:🏗️build/👁️observation/📥️reuse/🧺️coalesce/🟨️.mjs` | 5b1249b3 | 5b1249b3f43d | 2026-11-22 00:57 | E1 round 2: presence session | OK |
| `B-mod:🏗️build/👁️observation/🗄️analysis-store/🟨️.mjs` | 77892de8 | 77892de81bac | 2026-11-22 02:37 | E1 round 7: persisted source | OK |
| `B-mod:🏗️build/🔣️policy.json` | 54e5ae60 | 54e5ae60e138 | 2026-11-22 03:04 | E18: learned estimates from  | OK |
| `B-mod:🏗️build/🔧️tools/🏗️bazel/🌐️remote-cache/🧭️binary/🟦️.ts` | aecab2e5 | aecab2e510e9 | 2026-11-22 02:15 | E10 round 4: zero-touch Baze | OK |
| `B-mod:🏗️build/🕸️graph/📥️inference/📜️contract-reads/🟨️.mjs` | 36a95e55 | 36a95e556bae | 2026-11-22 01:35 | E1 round 3: owner record key | OK |
| `B-mod:🏗️build/🕸️graph/📥️inference/📦️project-nodes/🌐️owner-pool/🧒️child/🟨️.mjs` | a1ae6c5e | a1ae6c5ee072 | 2026-11-22 01:10 | E1 round 2: presence session | OK |
| `B-mod:🏗️build/🕸️graph/📥️inference/📦️project-nodes/🌐️owner-pool/🧬️schema/🔣️.json` | 571d593b | a597d75dc0ae | 2026-11-22 01:58 | E1 round 3: owner record key | OK (recorded in a later section: E3 round 3b (a597d75dc0ae)) |
| `B-mod:🏗️build/🕸️graph/📥️inference/📦️project-nodes/🏊️project-farm/🏊️pool/🧬️schema/🔣️.json` | d47212d3 | d47212d3465c | 2026-11-22 02:14 | E1 round 6: single-owner har | OK |
| `B-mod:🏗️build/🕸️graph/📥️inference/📦️project-nodes/📥️take/🟨️.mjs` | 952c5e7b | 952c5e7b602f | 2026-11-22 01:22 | E1 round 2: presence session, E1: slice looks 12x cheaper  | changed-by-later (live = latest landing) |
| `B-mod:🏗️build/🕸️graph/📥️inference/📦️project-nodes/🟨️.mjs` | 8eb3fca5 | 8eb3fca545c6 | 2026-11-22 02:51 | E1 round 2: presence session, E1 round 3: owner record key, E1 round 7: persisted source, E1: slice looks 12x cheaper  | changed-by-later (live = latest landing) |
| `B-mod:🏗️build/🗓️scheduler/⌨️command/🟦️.ts` | 931c464e8b8b | 931c464e8b8b | 2026-11-22 00:54 | E11: tools provisioning, sch | OK |
| `B-mod:🏗️build/🗓️scheduler/🎟️ticket/🟦️.ts` | 412b5a432a09 | 412b5a432a09 | 2026-11-22 00:54 | E11: tools provisioning, sch | OK |
| `B-mod:🏗️build/🗓️scheduler/🟦️.ts` | 5cbb9a387a95 | 5cbb9a387a95 | 2026-11-22 00:54 | E11: tools provisioning, sch | OK |
| `B-mod:🏗️build/🗓️scheduler/🧬️schema/🔣️.json` | 5672e3a2df70 | 5672e3a2df70 | 2026-11-22 00:54 | E11: tools provisioning, sch | OK |
| `B-mod:🏗️build/🧊️revision/🗂️fs/🟦️.ts` | 47dc7697b0b1 | 47dc7697b0b1 | 2026-11-22 00:45 | E9: build system test repair | OK |
| `B-mod:🏗️build/🧰️tool-setup/🔣️.json` | e208ac805c5a | e208ac805c5a | 2026-11-22 00:49 | E11: tools provisioning, sch | OK |
| `B-mod:🏗️build/🧰️tool-setup/🟦️.ts` | 5e75ba0eff9d | 5e75ba0eff9d | 2026-11-22 00:49 | E11: tools provisioning, sch | OK |
| `B-mod:🏗️build/🧰️tool-setup/🧬️schema/🔣️.json` | 998e3be410c0 | 998e3be410c0 | 2026-11-22 00:49 | E11: tools provisioning, sch | OK |
| `B-mod:🏗️build/🧱️job-limits/🎫️admission/🟦️.ts` | 0d061c45 | 0d061c4560fa | 2026-11-22 03:03 | E18: learned estimates from  | OK |
| `B-mod:🏗️build/🧱️job-limits/🧪️tests/🟦️.ts` | 0160708d | 0160708d991c | 2026-11-22 03:09 | E18: learned estimates from  | OK |
| `B-mod:🏗️build/🧹️cache-gc/🚀️launch/🟦️.ts` | 620137756b0d | 620137756b0d | 2026-11-22 01:46 | E14 round 3: collector launc | OK |
| `B-mod:🏗️build/🩺️doctor/📖️rules/🔣️.json` | 3c4a95a80412 | 3c4a95a80412 | 2026-11-22 01:11 | E14: doctor image repair and, coordinator: doctor catalogu | changed-by-later (live = latest landing) |
| `B-mod:🏗️build/🩺️doctor/🔎️probes/💾️storage/🔒️image-lease/🟦️.ts` | 3c5fdbb58302 | 3c5fdbb58302 | 2026-11-22 01:14 | E14: doctor image repair and | OK |
| `B-mod:🏗️build/🩺️doctor/🧬️schema/🔣️.json` | 4b81259effb3 | 4b81259effb3 | 2026-11-22 01:00 | E11: tools provisioning, sch | OK |
| `B-mod:🏗️build/🪪️graph-state/📦️project-records/🟦️.ts` | a323fa07 | a323fa07e873 | 2026-11-22 01:19 | E1 round 2: presence session | OK |
| `B-mod:🏗️build/🪪️graph-state/📦️project-records/🧳️portable/🟦️.ts` | 48202c8da8f7 | 48202c8da8f7 | 2026-11-22 02:11 | E1 round 4: canonical row or, E3 round 4: newest record wi | changed-by-later (live = latest landing) |
| `B-mod:🏗️build/🪪️graph-state/📦️project-records/🧳️portable/🧪️tests/🐍️.py` | af0712072aae | af0712072aae | 2026-11-22 02:09 | E3 round 4: newest record wi | OK |
| `B-mod:🏗️build/🪪️graph-state/📦️project-records/🧳️portable/🧪️tests/🔣️.json` | 0e532aca988e | 0e532aca988e | 2026-11-22 02:11 | E3 round 4: newest record wi | OK |
| `B-mod:🏗️build/🪪️graph-state/📦️project-records/🧳️portable/🧪️tests/🟦️.ts` | 13ba719a91f7 | 13ba719a91f7 | 2026-11-22 02:11 | E3 round 4: newest record wi | OK |
| `B-mod:🏗️build/🪪️graph-state/📦️project-records/🧳️portable/🧬️schema/🔣️.json` | cb6ef61a8e0d | cb6ef61a8e0d | 2026-11-22 02:09 | E3 round 4: newest record wi | OK |
| `HC-mod:🏗️build/🌡️host-capacity/🎫️gate/📋️contract/🟦️.ts` | acdd66eb | acdd66eb98eb | 2026-11-22 03:03 | E18: learned estimates from  | OK |
| `HC-mod:🏗️build/🌡️host-capacity/🎫️gate/🟦️.ts` | 92d7f437 | 92d7f4376c14 | 2026-11-22 03:04 | E18: learned estimates from  | OK |
| `HC-mod:🏗️build/🌡️host-capacity/📏️decision/🟦️.ts` | c0de0d4a9ade | dac19ceeb0e3 | 2026-11-22 03:21 | E18: learned estimates from , coordinator: restore hostUnd | OK (recorded in a later section: E18 round 2 (dac19cee)) |
| `HC-mod:🏗️build/🌡️host-capacity/📐️estimates/🟦️.ts` | 47b65f81 | 47b65f8177ef | 2026-11-22 03:03 | E18: learned estimates from  | OK |
| `HC-mod:🏗️build/🌡️host-capacity/🧪️tests/📐️estimates/🐍️oracle/🐍️.py` | 8ee02475 | 8ee02475918a | 2026-11-22 03:04 | E18: learned estimates from  | OK |
| `HC-mod:🏗️build/🌡️host-capacity/🧪️tests/📐️estimates/🟦️.ts` | d6d499cc | d6d499ccae7b | 2026-11-22 03:04 | E18: learned estimates from  | OK |
| `HC-mod:🏗️build/🌡️host-capacity/🧪️tests/📐️estimates/🧫️fixtures/🔣️.json` | fb84e084 | fb84e084c30a | 2026-11-22 03:04 | E18: learned estimates from  | OK |
| `HC-mod:🏗️build/🌡️host-capacity/🧪️tests/🟦️.ts` | 6c090b62 | 6c090b622312 | 2026-11-22 03:07 | E18: learned estimates from  | OK |
| `HC-mod:🏗️build/🌡️host-capacity/🧬️schema/🔣️.json` | ca8a29c9 | 2039987c1717 | 2026-11-22 03:21 | E18: learned estimates from  | OK (recorded in a later section: E18 round 2 (2039987c)) |
| `HC-mod:📦️packages/🟦️typescript/package.json` | 9ca3737ce013 | 9ca3737ce013 | 2026-11-22 01:10 | E14: doctor image repair and | OK |
| `HC-mod:🧪️testing/👷️workers/📐️learning/🟦️.ts` | 9b92f4c2 | 9b92f4c2d269 | 2026-11-22 03:03 | E18: learned estimates from  | OK |
| `HC-mod:🧪️testing/👷️workers/🧪️tests/📐️learning/🟦️.ts` | 038cf018 | 038cf0180d9f | 2026-11-22 03:05 | E18: learned estimates from  | OK |
| `HC-mod:🧰️tools/🔎️resolve/🟦️.ts` | e56f516aa8bb | e56f516aa8bb | 2026-11-22 00:48 | E11: tools provisioning, sch | OK |
| `HC-mod:🪪️agent-sessions/🗄️store/🟦️.ts` | 161c46557cba | 161c46557cba | 2026-11-22 00:57 | E11: tools provisioning, sch | OK |
| `HC-mod:🪪️agent-sessions/🧬️schema/🔣️.json` | 001fa5170bd6 | 001fa5170bd6 | 2026-11-22 00:57 | E11: tools provisioning, sch | OK |
| `HC-mod:🪪️agent-sessions/🪪️identity/🟦️.ts` | f5af3e925c77 | f5af3e925c77 | 2026-11-22 00:57 | E11: tools provisioning, sch | OK |
| `🔨️modules/⚙️runtime/🍞️bun/🧩️tooling/🟦️.ts` | 3452150a | 3452150ade15 | 2026-11-22 01:14 | E5 round 3: zero-downtime ca | OK |
| `🔨️modules/⚙️runtime/📋️contract/🟦️.ts` | 15896f58 | 15896f58ec03 | 2026-11-22 01:14 | E5 round 3: zero-downtime ca | OK |
| `🔨️modules/⚙️runtime/🪶️portable/🌍️serve/🟦️.ts` | f26757c6 | f26757c64e4c | 2026-11-22 01:14 | E5 round 3: zero-downtime ca | OK |
| `🛍️products/🦑️repo/📓️build-guide.md` | 6450597d | 6450597db9a0 | 2026-11-22 03:10 | E18: learned estimates from  | OK |
| `🛍️products/🦑️repo/🔨️modules/💻️client/🔌️mcp/🗂️catalog/🐹️.go` | e8ff5717eb03 | e8ff5717eb03 | 2026-11-22 01:09 | E12 round 2: MCP value-set t | OK |
| `🛍️products/🦑️repo/🔨️modules/💻️client/🔌️mcp/🧪️tests/🏗️build-surface/🐹️.go` | ef98d3e100cc | ef98d3e100cc | 2026-11-22 01:09 | E12 round 2: MCP value-set t | OK |
| `🛍️products/🦑️repo/🔨️modules/🔌️mcp/🖼️assets/🏗️build-surface.json` | 41f4c2988e11 | 41f4c2988e11 | 2026-11-22 01:53 | E12 round 2: MCP value-set t, E3 round 3: `repo remote-cac | changed-by-later (live = latest landing) |
| `🛍️products/🦑️repo/🔨️modules/🔌️mcp/🧪️tests/🧫️fixtures/🏗️build-surface.json` | 2c44e7f19e1d | 2c44e7f19e1d | 2026-11-22 01:53 | E3 round 3: `repo remote-cac | OK |
| `🛍️products/🦑️repo/🔨️modules/🤝️shared-worktree/🔁️replica/🛟️supervisor/🟦️.ts` | b132e5b9dbcd | b132e5b9dbcd | 2026-11-22 01:57 | E6 round 4: stuck store rest | OK |
| `🛍️products/🦑️repo/🔨️modules/🤝️shared-worktree/🔁️replica/🟦️.ts` | 8435215018aa | 8435215018aa | 2026-11-22 01:59 | E6 round 3: bounded rollout , E6 round 4: stuck store rest | changed-by-later (live = latest landing) |
| `🛍️products/🦑️repo/🔨️modules/🤝️shared-worktree/🚦️canary/🟦️.ts` | f6bd6bd33ffc | f6bd6bd33ffc | 2026-11-22 01:35 | E6 round 3: bounded rollout  | OK |


Ambiguous references (28, not tabled): short basenames such as `🟦️.ts`, `🔣️.json` or `🟨️.mjs` under a section context that the abbreviations do not pin down (for example `🟦️.ts` 8121258df8c8 in E18 and E18 r2). These cannot be attributed to one live file by path, so no verdict is given.

## Probable lost hunks

None confirmed. Candidates checked and cleared:

- E17 `slice pool` / planner / command host: identifiers `loadSlice`, `planPriorityV1`, `slicePriorityV1` are placed in other files by the landing text. The priority chain is present: `⌨️command/🌐️host/🟦️.ts` (`priority`), `🏡️resident/🏛️service/🌐️host/🟨️.mjs` (`frame.priority` into `planner.plan`), `🔌️ports/🧭️plan/🟨️.mjs` (`priority` into `loadSlice`), `🧭️invocation/🛰️plan/🟦️.ts` (`planPriorityV1`), `🏗️composition/🌐️host/👷️slice-pool/🟨️.mjs` (`slicePriorityV1`).
- E18 / coordinator restore: `host-capacity` decision, schema and product file are at the E18 round 2 restore hashes (dac19cee, 2039987c, 38badfc9). `hostUnderPressureV1` and `hostStarvedMillisecondsV1` are present in the decision module; `memoryWaitedOut` and `starvedMilliseconds` are present in the schema (the error id is not in the decision module, which is expected if the gate raises it).
- `B/🔣️policy.json` (54e5ae60, E18 landing) is live; `hostCapacity.swap` uses the share keys (`activeBasisPointsPerSecond`, `outBasisPointsPerSecond`, `shortBasisPoints`), no `activeMegabytesPerSecond` remains.
- E1 round 2/3/4 owner-claim and key identifiers (`createOwnerClaimsV1`, `takeClaimedV1`, `packRecordV1`, `projectAuthority`) are placed by the landing in `project-nodes/take`, `contract-reads`, `portable` and removed on purpose. Not in the files they were misattributed to.
- E5 round 4 `writeBudgetMilliseconds`, E5 round 5 `placementIdleCountV1` / `fleetHeadroomV1`, E14 `readNativeImageAncestryV1`: the names live in the store, placement and doctor files the landing names, not in the test or reclaim files they were compared with.
- `sources-v2` (E3 round 5) and `rustSourceRemapArgumentsV1` (E3 round 4c) are present in `unit-cache` `🔏️key/🔗️sources` and `🧭️remap`.
- Reverts: no live file equals an earlier landed sha.

## Not verified / limits

- DRIFT rows (26): the live file was edited after its last hashed record and no landing section gives the new hash. Several have a matching prose note without a hash (E4 r4 `tools/🔣️.json`, E8 r2 `ready/🟦️.ts`, E14 r2 doctor probe and reclaim, E12 r4 bench and SLO, E16 `sidecar/🟦️.ts` and test files, E18 r2 `sealed-read`). Others (server, store, observation files modified 02:28 to 03:28) have no prose note that names them. Their content was not verified against any spec, so they are reported as unexplained drift, not as lost hunks.
- Two landings name hashes without saying whether the hash is before or after (E17 `🏡️resident/🏛️service/🌐️host/🟨️.mjs` 47c6ed72e918). The table uses the latest hash as "after"; this can produce a false DRIFT.
- Mtimes are the live file mtimes at audit time (about 03:3x CET); later edits by others may already have changed the table.
- Identifiers were not checked for the 28 ambiguous references.

## Recommended follow-up (for the coordinator)

1. Ask the lanes that own the DRIFT files (server, store, observation `🟨️.mjs`/`🟨️.d.mts`, `sealed-read` test, owner-pool inference, slice pool) to record their current sha256 in `T/landings.md`.
2. Re-run this audit after the next stale-write incident; the same script can be reused against the new sections.
