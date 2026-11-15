# Audit: reverted landings in the live tree (2026-11-15)

Read-only audit, no fix applied. Source: `/Users/Shared/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION/landings.md`. Scratch script: `/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/A4/audit_landings.py`.

Checked: files touched by landing sections dated 2026-11-15 (30 sections from landings.md line 1300), 216 distinct files. Each file's history includes earlier landings of any date (the matching set for REVERTED), so a revert to an older copy is found. Today's record lines that did not parse: 0.

Hash rule: a landing's after-hash is a 12-hex prefix of the sha256 of the file. CURRENT = equals the last landing's after; REVERTED = equals a before-hash or an earlier after-hash; DIVERGED = equals none; MISSING = absent.

## Counts

| status | files |
|---|---|
| REVERTED | 0 |
| MISSING | 0 |
| DIVERGED | 26 |
| CURRENT | 190 |

## Files by severity

| severity | status | file | current sha256 | last landing | detail | mtime |
|---|---|---|---|---|---|---|
| 2 | DIVERGED | `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-execution/🛰️sidecar/🧬️schema/🔣️.json` | 48f75d260a6d | R17 line 2031 after 637c82fb42b0 | hash equals no recorded hash of this file | 2026-11-15 06:44:41 |
| 2 | DIVERGED | `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏡️resident/⌨️command/🌐️host/🟨️.mjs` | 2af56d4d14f6 | R7 line 1634 after 03fcb667df20 | hash equals no recorded hash of this file | 2026-11-15 06:43:19 |
| 2 | DIVERGED | `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏡️resident/🏗️composition/🌐️host/👷️slice-pool/🧱️limit/🟨️.mjs` | 79722721c1b5 | R9 line 1701 after 8f523235800c | hash equals no recorded hash of this file | 2026-11-15 06:42:02 |
| 2 | DIVERGED | `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏡️resident/🏗️composition/🌐️host/🟨️.mjs` | 56e2f984594e | R7b line 2057 after 22f18d24c96f | hash equals no recorded hash of this file | 2026-11-15 06:47:56 |
| 2 | DIVERGED | `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏡️resident/🏛️service/🌐️host/🟨️.mjs` | 613b97a3174d | R9 line 1703 after 093a87762fe3 | hash equals no recorded hash of this file | 2026-11-15 06:42:45 |
| 2 | DIVERGED | `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏡️resident/🏛️service/🧯️failure/🟨️.mjs` | fe7e3be47e7d | R9 line 1704 after 51f31cc6ff50 | hash equals no recorded hash of this file | 2026-11-15 06:52:50 |
| 2 | DIVERGED | `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏡️resident/🟨️.mjs` | 01396a8c2dfd | R7b line 2063 after 44b06ede6784 | hash equals no recorded hash of this file | 2026-11-15 06:47:56 |
| 2 | DIVERGED | `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏡️resident/🧪️tests/🛡️plan-guard/🟦️.ts` | 488e02482822 | R9 line 1705 after e5686eb16f51 | hash equals no recorded hash of this file | 2026-11-15 06:42:02 |
| 2 | DIVERGED | `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏡️resident/🧬️schema/🔣️.json` | 4072de588d58 | R9 line 1707 after ecfa3e1b4de1 | hash equals no recorded hash of this file | 2026-11-15 06:43:19 |
| 2 | DIVERGED | `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/👁️observation/🟨️.d.mts` | 4310f21d3b4e | R11 line 1943 after e6f9be5745e2 | hash equals no recorded hash of this file | 2026-11-15 06:52:44 |
| 2 | DIVERGED | `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/👁️observation/🟨️.mjs` | dcfb15141380 | R11 line 1944 after 395b411563b4 | hash equals no recorded hash of this file | 2026-11-15 06:52:44 |
| 2 | DIVERGED | `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/👁️observation/🧪️tests/🧫️fixtures/🔣️.json` | f0e1bd78987c | R11 line 1945 after 4b0c5ece6057 | hash equals no recorded hash of this file | 2026-11-15 06:52:50 |
| 2 | DIVERGED | `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📖️texts/🔣️.json` | e57eeb35d160 | R7 line 1654 after 8129afc0686e | hash equals no recorded hash of this file | 2026-11-15 05:56:45 |
| 2 | DIVERGED | `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🌐️remote-exchange/🧬️schema/🔣️.json` | e71602ce9bf7 | R8 line 1782 after dc5eb6ade4b5 | hash equals no recorded hash of this file | 2026-11-15 06:13:52 |
| 2 | DIVERGED | `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️host/🟦️.ts` | 3fa7a0fed880 | R8 line 1777 after ce54abd07abb | hash equals no recorded hash of this file | 2026-11-15 06:03:55 |
| 2 | DIVERGED | `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🤝️prefetch/🟦️.ts` | 9ccc9c003713 | R8 line 1778 after 4bf52b8c27b9 | hash equals no recorded hash of this file | 2026-11-15 06:27:31 |
| 2 | DIVERGED | `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🧪️tests/🙋️fleet-read/🟦️.ts` | bd2829d88cd9 | R8 line 1791 after 9f1ee3829782 | hash equals no recorded hash of this file | 2026-11-15 06:27:40 |
| 2 | DIVERGED | `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧭️invocation/🛰️plan/🟦️.ts` | 4334c6b0da3c | R7b line 2066 after ae106dff4acb | hash equals no recorded hash of this file | 2026-11-15 06:43:36 |
| 2 | DIVERGED | `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🪪️graph-state/🙋️wanted/💰️cost/🟦️.ts` | 47572a1c88a3 | R14 line 1855 after 37eee27d895a | hash equals no recorded hash of this file | 2026-11-15 06:03:47 |
| 2 | DIVERGED | `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🪪️graph-state/🙋️wanted/💰️cost/🧪️tests/🟦️.ts` | 69e9c209a706 | R14 line 1860 after d3cc717823a5 | hash equals no recorded hash of this file | 2026-11-15 06:04:12 |
| 2 | DIVERGED | `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🪪️graph-state/🙋️wanted/💰️cost/🧬️schema/🔣️.json` | 8c4f501cbe65 | P14 line 1495 after 59b70969922a | hash equals no recorded hash of this file | 2026-11-15 06:03:47 |
| 2 | DIVERGED | `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🪪️graph-state/🙋️wanted/📖️read/🟦️.ts` | 9524d6234902 | R14 line 1853 after cd64703aff9c | hash equals no recorded hash of this file | 2026-11-15 06:06:05 |
| 2 | DIVERGED | `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🪪️graph-state/🙋️wanted/🧪️tests/🌍️two-platforms/🟦️.ts` | 7012e43f32c3 | P14 line 1515 after 8a7e15dafec8 | hash equals no recorded hash of this file | 2026-11-15 06:27:35 |
| 2 | DIVERGED | `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🪪️graph-state/🙋️wanted/🧪️tests/🍏️idle-grant/🟦️.ts` | 7b6b2adad4f1 | R4 line 1601 after d9fe82a9e755 | hash equals no recorded hash of this file | 2026-11-15 06:27:31 |
| 2 | DIVERGED | `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🪪️graph-state/🙋️wanted/🧪️tests/📖️read/🟦️.ts` | fbd5c0e2daa6 | R14 line 1857 after 074e0ef0b3fa | hash equals no recorded hash of this file | 2026-11-15 06:07:41 |
| 2 | DIVERGED | `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🪪️graph-state/🙋️wanted/🧪️tests/🛠️server/🟦️.ts` | 5158dd41bf28 | R14 line 1858 after 713c07ce356e | hash equals no recorded hash of this file | 2026-11-15 06:13:08 |
| 3 | CURRENT | `/Users/Shared/semio/🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/📏️decision/🟦️.ts` | ce45d091a99a | R13 line 1885 after ce45d091a99a | equals last landing | 2026-11-15 05:55:37 |
| 3 | CURRENT | `/Users/Shared/semio/🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/📐️estimates/🟦️.ts` | 53ed14d59042 | R13 line 1886 after 53ed14d59042 | equals last landing | 2026-11-15 05:55:37 |
| 3 | CURRENT | `/Users/Shared/semio/🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/📡️sampling/🟦️.ts` | 0ebcbeb40251 | R4 line 1592 after 0ebcbeb40251 | equals last landing | 2026-11-15 04:30:19 |
| 3 | CURRENT | `/Users/Shared/semio/🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🧪️tests/📡️sampling/🧫️fixtures/🔣️.json` | 072509526e4c | R4 line 1593 after 072509526e4c | equals last landing | 2026-11-15 04:30:19 |
| 3 | CURRENT | `/Users/Shared/semio/🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🧪️tests/🔋️available/🔣️.json` | fd610242f8b0 | R4 line 1594 after fd610242f8b0 | equals last landing | 2026-11-15 04:30:19 |
| 3 | CURRENT | `/Users/Shared/semio/🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🧪️tests/🔋️available/🟦️.ts` | eef419ad1c38 | R4 line 1595 after eef419ad1c38 | equals last landing | 2026-11-15 04:30:19 |
| 3 | CURRENT | `/Users/Shared/semio/🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🧪️tests/🟦️.ts` | 2a025b616517 | R13 line 1887 after 2a025b616517 | equals last landing | 2026-11-15 05:55:37 |
| 3 | CURRENT | `/Users/Shared/semio/🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🧬️schema/🔣️.json` | 524ee6819302 | R13 line 1884 after 524ee6819302 | equals last landing | 2026-11-15 05:55:37 |
| 3 | CURRENT | `/Users/Shared/semio/🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/📤️publication/📸️observation/🧪️tests/🟦️.ts` | 8b9ba86a8b72 | R6b line 1758 after 8b9ba86a8b72 | equals last landing | 2026-11-15 05:28:48 |
| 3 | CURRENT | `/Users/Shared/semio/🧰️framework/🔨️modules/📁️filesystem/🧾️observation/📁️descriptor/🟦️.ts` | 868575a718fc | P6 line 1301 after 868575a718fc | equals last landing | 2026-11-15 00:07:09 |
| 3 | CURRENT | `/Users/Shared/semio/🧰️framework/🔨️modules/📁️filesystem/🧾️observation/🧪️tests/🧭️denied-entry/🟦️.ts` | 4a860380a515 | P6 line 1302 after 4a860380a515 | equals last landing | 2026-11-15 00:07:09 |
| 3 | CURRENT | `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/📤️publication/⚙️setup/♻️retirement/🟦️.ts` | 158cd736c2f3 | R6 line 1619 after 158cd736c2f3 | equals last landing | 2026-11-15 05:01:54 |
| 3 | CURRENT | `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/📤️publication/⚙️setup/♻️retirement/🧪️tests/🔁️busy-driver/🟦️.ts` | afc7b44f0643 | R6b line 1757 after afc7b44f0643 | equals last landing | 2026-11-15 05:28:48 |
| 3 | CURRENT | `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/📤️publication/⚙️setup/♻️retirement/🧪️tests/🟦️.ts` | 23f8808e2af2 | R6 line 1623 after 23f8808e2af2 | equals last landing | 2026-11-15 05:01:54 |
| 3 | CURRENT | `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/📤️publication/⚙️setup/♻️retirement/🧬️schema/🔣️.json` | 00d55c66d3d1 | R6 line 1620 after 00d55c66d3d1 | equals last landing | 2026-11-15 05:01:54 |
| 3 | CURRENT | `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/📤️publication/⚙️setup/💰️funding/🟦️.ts` | d4331655f264 | R6b line 1752 after d4331655f264 | equals last landing | 2026-11-15 05:28:48 |
| 3 | CURRENT | `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/📤️publication/⚙️setup/🟦️.ts` | 2f92ddbdf3c9 | R6b line 1753 after 2f92ddbdf3c9 | equals last landing | 2026-11-15 05:28:48 |
| 3 | CURRENT | `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/📤️publication/⚙️setup/🧪️tests/⏳️busy-funding/🟦️.ts` | d4a0926aa5ef | R6b line 1756 after d4a0926aa5ef | equals last landing | 2026-11-15 05:28:48 |
| 3 | CURRENT | `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/📤️publication/⚙️setup/🧪️tests/🌊️moving-tree/🟦️.ts` | dbaa395a6b75 | R6b line 1759 after dbaa395a6b75 | equals last landing | 2026-11-15 05:28:48 |
| 3 | CURRENT | `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/📤️publication/⚙️setup/🧪️tests/🟦️.ts` | 2f2746cb0de1 | R1b line 1581 after 2f2746cb0de1 | equals last landing | 2026-11-15 04:07:45 |
| 3 | CURRENT | `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/📤️publication/⚙️setup/🧬️schema.json` | 860738d0d83f | R6b line 1754 after 860738d0d83f | equals last landing | 2026-11-15 05:28:48 |
| 3 | CURRENT | `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/📤️publication/🟦️.ts` | 13184062aea7 | R6b line 1751 after 13184062aea7 | equals last landing | 2026-11-15 05:28:48 |
| 3 | CURRENT | `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/📤️publication/🧪️tests/📋️roles/🔣️.json` | 977482d8c980 | R6c line 1928 after 977482d8c980 | equals last landing | 2026-11-15 06:05:40 |
| 3 | CURRENT | `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📖️texts/🔣️.json` | 0d42e537523b | R6b line 1755 after 0d42e537523b | equals last landing | 2026-11-15 05:28:48 |
| 3 | CURRENT | `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/🏗️native/🟦️.ts` | 812cd7619ffa | R11 line 1933 after 812cd7619ffa | equals last landing | 2026-11-15 06:13:10 |
| 3 | CURRENT | `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/💾️storage/🟦️.ts` | 5815fbbe5705 | R6b line 1750 after 5815fbbe5705 | equals last landing | 2026-11-15 05:28:48 |
| 3 | CURRENT | `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/💾️storage/🧬️schema/📥️install.json` | 09c8fcbb4f98 | R6c line 1927 after 09c8fcbb4f98 | equals last landing | 2026-11-15 06:05:40 |
| 3 | CURRENT | `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛰️fleet-fetch/🐍️uv/🟦️.ts` | dcbabc9a3a5e | R11 line 1934 after dcbabc9a3a5e | equals last landing | 2026-11-15 06:13:10 |
| 3 | CURRENT | `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛰️fleet-fetch/🐍️uv/🧪️tests/🟦️.ts` | 6635560de92e | R11 line 1935 after 6635560de92e | equals last landing | 2026-11-15 06:13:10 |
| 3 | CURRENT | `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛰️fleet-fetch/📦️install/🟦️.ts` | 69e653894934 | R11 line 1936 after 69e653894934 | equals last landing | 2026-11-15 06:13:10 |
| 3 | CURRENT | `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛰️fleet-fetch/🧬️schema/🔣️.json` | 861c2370373d | R11 line 1937 after 861c2370373d | equals last landing | 2026-11-15 06:13:10 |
| 3 | CURRENT | `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛰️fleet-fetch/🧰️setup/🟦️.ts` | 8709305adaed | R11 line 1938 after 8709305adaed | equals last landing | 2026-11-15 06:13:10 |
| 3 | CURRENT | `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛰️fleet-fetch/🧰️setup/🧪️tests/🐍️.py` | 637f38405584 | R11 line 1939 after 637f38405584 | equals last landing | 2026-11-15 06:13:10 |
| 3 | CURRENT | `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛰️fleet-fetch/🧰️setup/🧪️tests/🔣️.json` | 127c7d09f65d | R11 line 1940 after 127c7d09f65d | equals last landing | 2026-11-15 06:13:10 |
| 3 | CURRENT | `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛰️fleet-fetch/🧰️setup/🧪️tests/🟦️.ts` | 5802294fbc4b | R11 line 1941 after 5802294fbc4b | equals last landing | 2026-11-15 06:13:10 |
| 3 | CURRENT | `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛰️fleet-fetch/🧰️setup/🧬️schema/🔣️.json` | 89dafd94f6a3 | R11 line 1942 after 89dafd94f6a3 | equals last landing | 2026-11-15 06:13:10 |
| 3 | CURRENT | `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/💾️store/🟦️.ts` | 3b35806fcf6a | R16 line 1980 after 3b35806fcf6a | equals last landing | 2026-11-15 06:08:17 |
| 3 | CURRENT | `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/📜️script.ts` | ec3ca8ab442e | R16 line 1981 after ec3ca8ab442e | equals last landing | 2026-11-15 06:09:36 |
| 3 | CURRENT | `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/🔏️key/🟦️.ts` | ca56b4631af8 | R16 line 1974 after ca56b4631af8 | equals last landing | 2026-11-15 06:05:59 |
| 3 | CURRENT | `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/🔏️key/🧪️tests/🐍️oracle/🐍️.py` | 4f7f70608973 | R16 line 1975 after 4f7f70608973 | equals last landing | 2026-11-15 06:04:44 |
| 3 | CURRENT | `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/🔏️key/🧪️tests/🔣️.json` | 088431c3c9f7 | R16 line 1976 after 088431c3c9f7 | equals last landing | 2026-11-15 06:04:49 |
| 3 | CURRENT | `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/🔏️key/🧪️tests/🧬️components/🐍️.py` | f68b6d81503d | R16 line 1979 after f68b6d81503d | equals last landing | 2026-11-15 06:05:59 |
| 3 | CURRENT | `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/🔏️key/🧪️tests/🧬️components/🔣️.json` | e31d6a629650 | R16 line 1978 after e31d6a629650 | equals last landing | 2026-11-15 06:05:59 |
| 3 | CURRENT | `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/🔏️key/🧪️tests/🧬️components/🟦️.ts` | 0078d01919d2 | R16 line 1977 after 0078d01919d2 | equals last landing | 2026-11-15 06:06:10 |
| 3 | CURRENT | `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/🛰️fleet/📤️uploader/🟦️.ts` | 5c4ef1810246 | R10 line 1826 after 5c4ef1810246 | equals last landing | 2026-11-15 05:47:24 |
| 3 | CURRENT | `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/🛰️fleet/📤️uploader/🧪️tests/🟦️.ts` | b5fd9addd1d6 | R10 line 1827 after b5fd9addd1d6 | equals last landing | 2026-11-15 05:47:24 |
| 3 | CURRENT | `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/🛰️fleet/🟦️.ts` | a03a68ab552a | R10 line 1828 after a03a68ab552a | equals last landing | 2026-11-15 05:47:24 |
| 3 | CURRENT | `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/🛰️fleet/🧬️schema/🔣️.json` | f60b7bd6fa7b | R10 line 1829 after f60b7bd6fa7b | equals last landing | 2026-11-15 05:47:24 |
| 3 | CURRENT | `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/🛰️fleet/🪪️identity/🟦️.ts` | f8d8457e86f3 | R10 line 1830 after f8d8457e86f3 | equals last landing | 2026-11-15 05:47:24 |
| 3 | CURRENT | `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/🛰️fleet/🪪️identity/🧪️tests/🌍️machine-law/🟦️.ts` | 913ffc8ff458 | R10 line 1831 after 913ffc8ff458 | equals last landing | 2026-11-15 05:47:24 |
| 3 | CURRENT | `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/🧪️tests/🔁️churn/🟦️.ts` | bc83d7163fd4 | R16 line 1985 after bc83d7163fd4 | equals last landing | 2026-11-15 06:09:20 |
| 3 | CURRENT | `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/🧬️schema/🔣️.json` | 41d7234d4677 | R16 line 1973 after 41d7234d4677 | equals last landing | 2026-11-15 06:12:46 |
| 3 | CURRENT | `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/🧭️remap/🟦️.ts` | b0d1bb0da5e0 | R16 line 1982 after b0d1bb0da5e0 | equals last landing | 2026-11-15 06:05:04 |
| 3 | CURRENT | `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/🧭️remap/🧪️tests/🟦️.ts` | 83befdbda1f2 | R16 line 1983 after 83befdbda1f2 | equals last landing | 2026-11-15 06:05:04 |
| 3 | CURRENT | `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/🧹️gc/🟦️.ts` | eade8b383166 | R16 line 1984 after eade8b383166 | equals last landing | 2026-11-15 06:08:17 |
| 3 | CURRENT | `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/⌨️command/📥️entry/🟦️.ts` | 5110927fc673 | R13b line 1919 after 5110927fc673 | equals last landing | 2026-11-15 05:59:07 |
| 3 | CURRENT | `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-cache/💾️store/🟦️.ts` | 4eaec0aa7ce2 | R16 line 1986 after 4eaec0aa7ce2 | equals last landing | 2026-11-15 06:08:44 |
| 3 | CURRENT | `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-cache/🖥️server/🟦️.ts` | 1c38bcf049f2 | R16 line 1987 after 1c38bcf049f2 | equals last landing | 2026-11-15 06:08:44 |
| 3 | CURRENT | `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-cache/🧪️tests/🟦️.ts` | 04501f6c3347 | R16 line 1988 after 04501f6c3347 | equals last landing | 2026-11-15 06:09:04 |
| 3 | CURRENT | `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-execution/🌡️load/🔎️why/🟦️.ts` | 59632a75e7bc | R13b line 1911 after 59632a75e7bc | equals last landing | 2026-11-15 05:59:07 |
| 3 | CURRENT | `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-execution/🌡️load/🟦️.ts` | cdef458902db | R13 line 1889 after cdef458902db | equals last landing | 2026-11-15 05:55:37 |
| 3 | CURRENT | `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-execution/🚀️offload/🗺️paths/🟦️.ts` | 9908f95a70c7 | R10 line 1833 after 9908f95a70c7 | equals last landing | 2026-11-15 05:47:24 |
| 3 | CURRENT | `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-execution/🚀️offload/🧪️tests/🟦️.ts` | 5da463e7dea4 | R10 line 1834 after 5da463e7dea4 | equals last landing | 2026-11-15 05:47:24 |
| 3 | CURRENT | `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-execution/🛰️agent/🟦️.ts` | 6d36265b8420 | R17 line 2017 after 6d36265b8420 | equals last landing | 2026-11-15 06:27:33 |
| 3 | CURRENT | `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-execution/🛰️agent/🩺️readiness/🟦️.ts` | 905ef3b489a9 | R13b line 1908 after 905ef3b489a9 | equals last landing | 2026-11-15 05:59:07 |
| 3 | CURRENT | `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-execution/🛰️sidecar/👷️agent/🟦️.ts` | 4ca5990ae857 | R17 line 2018 after 4ca5990ae857 | equals last landing | 2026-11-15 06:27:33 |
| 3 | CURRENT | `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-execution/🛰️sidecar/🕸️graph/🟦️.ts` | 6d8341747bde | R17 line 2019 after 6d8341747bde | equals last landing | 2026-11-15 06:27:33 |
| 3 | CURRENT | `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-execution/🛰️sidecar/🟦️.ts` | fb2be36adbe5 | R17 line 2020 after fb2be36adbe5 | equals last landing | 2026-11-15 06:27:33 |
| 3 | CURRENT | `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-execution/🛰️sidecar/🧪️tests/🔁️restart/🟦️.ts` | 973d1fbbd168 | R17 line 2029 after 973d1fbbd168 | equals last landing | 2026-11-15 06:27:33 |
| 3 | CURRENT | `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-execution/🛰️sidecar/🧪️tests/🕸️graph/📡️wave-upload/🟦️.ts` | 72e956f82e15 | P13 line 1430 after 72e956f82e15 | equals last landing | 2026-11-15 02:36:12 |
| 3 | CURRENT | `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-execution/🛰️sidecar/🧪️tests/🚰️drain/🟦️.ts` | f919e1b65a15 | R17 line 2030 after f919e1b65a15 | equals last landing | 2026-11-15 06:27:33 |
| 3 | CURRENT | `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-execution/🛰️sidecar/🧪️tests/🟦️.ts` | 06af4815bd3f | R13b line 1912 after 06af4815bd3f | equals last landing | 2026-11-15 05:59:07 |
| 3 | CURRENT | `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-execution/🛰️sidecar/🧪️tests/🧮️closure/🟦️.ts` | a1f4f07ede63 | R17 line 2021 after a1f4f07ede63 | equals last landing | 2026-11-15 06:27:33 |
| 3 | CURRENT | `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-execution/🛰️sidecar/🧮️closure/🟦️.ts` | 571f2640e13f | R17 line 2022 after 571f2640e13f | equals last landing | 2026-11-15 06:27:33 |
| 3 | CURRENT | `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-execution/🛰️sidecar/🧰️shared/🟦️.ts` | 625f9d910f67 | R17 line 2023 after 625f9d910f67 | equals last landing | 2026-11-15 06:27:33 |
| 3 | CURRENT | `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-execution/🟦️.ts` | 5b25a99e8d1e | R17 line 2024 after 5b25a99e8d1e | equals last landing | 2026-11-15 06:27:33 |
| 3 | CURRENT | `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-execution/🧪️tests/🔎️why/🟦️.ts` | 7819acd68dbc | R13b line 1915 after 7819acd68dbc | equals last landing | 2026-11-15 05:59:07 |
| 3 | CURRENT | `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-execution/🧱️actions/🟦️.ts` | c2054f700e92 | R10 line 1835 after c2054f700e92 | equals last landing | 2026-11-15 05:47:24 |
| 3 | CURRENT | `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-execution/🧱️actions/🧪️tests/🟦️.ts` | e53a29815f3f | R10 line 1836 after e53a29815f3f | equals last landing | 2026-11-15 05:47:24 |
| 3 | CURRENT | `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏃️execution/🧬️schema/🌐️remote-cache.json` | 78a7d578d30d | R16 line 1989 after 78a7d578d30d | equals last landing | 2026-11-15 06:08:44 |
| 3 | CURRENT | `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏃️execution/🧬️schema/🌐️remote-execution.json` | 5ec255c0f8d1 | R17 line 2025 after 5ec255c0f8d1 | equals last landing | 2026-11-15 06:27:33 |
| 3 | CURRENT | `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏡️resident/🏗️composition/🌐️host/🍎️launchd/🟨️.mjs` | 3ae76f6c02f2 | P6 line 1303 after 3ae76f6c02f2 | equals last landing | 2026-11-15 00:07:09 |
| 3 | CURRENT | `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏡️resident/🏗️composition/🌐️host/👷️slice-pool/🌡️pressure/🟨️.d.mts` | dffd3c021411 | R9 line 1696 after dffd3c021411 | equals last landing | 2026-11-15 05:27:30 |
| 3 | CURRENT | `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏡️resident/🏗️composition/🌐️host/👷️slice-pool/🌡️pressure/🟨️.mjs` | 043ddb807e44 | R9 line 1697 after 043ddb807e44 | equals last landing | 2026-11-15 05:27:30 |
| 3 | CURRENT | `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏡️resident/🏗️composition/🌐️host/👷️slice-pool/👷️worker.ts` | fb69aa1eabff | R9 line 1698 after fb69aa1eabff | equals last landing | 2026-11-15 05:27:30 |
| 3 | CURRENT | `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏡️resident/🏗️composition/🌐️host/👷️slice-pool/🟨️.mjs` | 6d633c44f607 | R9 line 1699 after 6d633c44f607 | equals last landing | 2026-11-15 05:27:30 |
| 3 | CURRENT | `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏡️resident/🏗️composition/🌐️host/👷️slice-pool/🧱️limit/🟨️.d.mts` | 9b25f797cb54 | R9 line 1700 after 9b25f797cb54 | equals last landing | 2026-11-15 05:27:30 |
| 3 | CURRENT | `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏡️resident/🏗️composition/🌐️host/👷️slice-pool/🧵️process/🟨️.d.mts` | c8ffe4c79f4f | R7 line 1637 after c8ffe4c79f4f | equals last landing | 2026-11-15 05:09:05 |
| 3 | CURRENT | `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏡️resident/🏗️composition/🌐️host/👷️slice-pool/🧵️process/🟨️.mjs` | 452a6d7b0c2a | R9 line 1702 after 452a6d7b0c2a | equals last landing | 2026-11-15 05:27:30 |
| 3 | CURRENT | `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏡️resident/🏛️service/🧯️failure/🟨️.d.mts` | 939f58f35e58 | R7 line 1640 after 939f58f35e58 | equals last landing | 2026-11-15 05:09:05 |
| 3 | CURRENT | `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏡️resident/🔐️consent/🔎️probe/🟨️.mjs` | 284d35af899f | P6 line 1307 after 284d35af899f | equals last landing | 2026-11-15 00:07:09 |
| 3 | CURRENT | `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏡️resident/🔐️consent/🔎️sample/🟨️.mjs` | 6602c691dd0f | R7 line 1642 after 6602c691dd0f | equals last landing | 2026-11-15 05:09:05 |
| 3 | CURRENT | `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏡️resident/🔐️consent/🟨️.d.mts` | 83a9fac7ac73 | R7b line 2058 after 83a9fac7ac73 | equals last landing | 2026-11-15 06:42:43 |
| 3 | CURRENT | `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏡️resident/🔐️consent/🟨️.mjs` | 0e1892d620b4 | R7b line 2059 after 0e1892d620b4 | equals last landing | 2026-11-15 06:42:43 |
| 3 | CURRENT | `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏡️resident/🔐️consent/🧬️schema/🔣️.json` | a2858de7df04 | R7b line 2060 after a2858de7df04 | equals last landing | 2026-11-15 06:42:43 |
| 3 | CURRENT | `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏡️resident/🚪️client/🛰️plan/🟨️.d.mts` | 67595dc1117c | R7b line 2061 after 67595dc1117c | equals last landing | 2026-11-15 06:42:43 |
| 3 | CURRENT | `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏡️resident/🚪️client/🛰️plan/🟨️.mjs` | 57e5d3ddfc1f | R7b line 2062 after 57e5d3ddfc1f | equals last landing | 2026-11-15 06:42:43 |
| 3 | CURRENT | `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏡️resident/🧪️tests/🌐️host/🟦️.ts` | 753b00af2bb1 | R2b line 1575 after 753b00af2bb1 | equals last landing | 2026-11-15 04:05:05 |
| 3 | CURRENT | `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏡️resident/🧪️tests/🌡️pressure/🔣️.json` | d67066121225 | R4 line 1596 after d67066121225 | equals last landing | 2026-11-15 04:30:19 |
| 3 | CURRENT | `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏡️resident/🧪️tests/🌡️pressure/🟦️.ts` | d8a582df0293 | R4 line 1597 after d8a582df0293 | equals last landing | 2026-11-15 04:30:19 |
| 3 | CURRENT | `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏡️resident/🧪️tests/🏷️worker-error/🟦️.ts` | 0b3c8f4021b2 | R7 line 1649 after 0b3c8f4021b2 | equals last landing | 2026-11-15 05:09:05 |
| 3 | CURRENT | `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏡️resident/🧪️tests/🔐️consent/🟦️.ts` | 4a2cebfafbf6 | R7b line 2064 after 4a2cebfafbf6 | equals last landing | 2026-11-15 06:42:43 |
| 3 | CURRENT | `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏡️resident/🧪️tests/🛡️consent-service/🟦️.ts` | 5b20b9a0397b | R7b line 2065 after 5b20b9a0397b | equals last landing | 2026-11-15 06:42:43 |
| 3 | CURRENT | `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏡️resident/🧪️tests/🧯️failure/🟦️.ts` | 16e5f4dc15a7 | R7 line 1652 after 16e5f4dc15a7 | equals last landing | 2026-11-15 05:09:05 |
| 3 | CURRENT | `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏡️resident/🧪️tests/🧽️plan-answered/🟦️.ts` | 26362edf5094 | R2 line 1551 after 26362edf5094 | equals last landing | 2026-11-15 03:30:21 |
| 3 | CURRENT | `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏡️resident/🧪️tests/🪓️plan-limit/🟦️.ts` | a4f7be140836 | P12b line 1447 after a4f7be140836 | equals last landing | 2026-11-15 03:03:30 |
| 3 | CURRENT | `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏡️resident/🧪️tests/🪜️plan-refusal/🟦️.ts` | 4fe742172fbd | R9 line 1706 after 4fe742172fbd | equals last landing | 2026-11-15 05:27:30 |
| 3 | CURRENT | `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/👁️observation/🧪️tests/📥️reuse/🟦️.ts` | 35b5127b0c5c | R8 line 1793 after 35b5127b0c5c | equals last landing | 2026-11-15 05:30:23 |
| 3 | CURRENT | `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🧮️digest/🟨️.mjs` | e18792464f61 | R8 line 1779 after e18792464f61 | equals last landing | 2026-11-15 05:21:21 |
| 3 | CURRENT | `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔣️policy.json` | 288e855e9f72 | R13b line 1909 after 288e855e9f72 | equals last landing | 2026-11-15 05:59:07 |
| 3 | CURRENT | `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/🔗️closure/🟦️.ts` | a9dd51af8dcf | R8 line 1780 after a9dd51af8dcf | equals last landing | 2026-11-15 05:21:21 |
| 3 | CURRENT | `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/🧪️tests/📥️closure/🟦️.ts` | dea9304f8018 | R8 line 1792 after dea9304f8018 | equals last landing | 2026-10-25 09:16:07 |
| 3 | CURRENT | `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📁️workspace/🟦️.ts` | 97c6934b509e | R8 line 1774 after 97c6934b509e | equals last landing | 2026-11-15 05:14:08 |
| 3 | CURRENT | `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🧭️script-inputs/💾️entries/🟨️.d.mts` | 7aa50855cc6d | R11 line 1946 after 7aa50855cc6d | equals last landing | 2026-11-15 06:13:10 |
| 3 | CURRENT | `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🧭️script-inputs/💾️entries/🟨️.mjs` | 660d8e81898a | R11 line 1947 after 660d8e81898a | equals last landing | 2026-11-15 06:13:10 |
| 3 | CURRENT | `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🧭️script-inputs/🟨️.d.mts` | e68c5a67a5a7 | R11 line 1948 after e68c5a67a5a7 | equals last landing | 2026-11-15 06:13:10 |
| 3 | CURRENT | `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🧭️script-inputs/🟨️.mjs` | 23a4fcd535b6 | R11 line 1949 after 23a4fcd535b6 | equals last landing | 2026-11-15 06:13:10 |
| 3 | CURRENT | `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🧭️script-inputs/🧪️tests/🧽️walk-maps/🟦️.ts` | 9109f57011ac | R2 line 1552 after 9109f57011ac | equals last landing | 2026-11-15 03:30:21 |
| 3 | CURRENT | `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🧭️script-inputs/🧹️retention/🟨️.mjs` | f1e0bd328f28 | P12b line 1451 after f1e0bd328f28 | equals last landing | 2026-11-15 03:03:30 |
| 3 | CURRENT | `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🧭️script-inputs/🧹️retention/🧪️tests/🪶️bounded-read/🟦️.ts` | 98b369044f70 | P12b line 1461 after 98b369044f70 | equals last landing | 2026-11-15 03:03:30 |
| 3 | CURRENT | `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🌍️environment/🟨️.d.mts` | 57a69eb8f683 | P12b line 1452 after 57a69eb8f683 | equals last landing | 2026-11-15 03:03:30 |
| 3 | CURRENT | `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🌍️environment/🟨️.mjs` | abf517e538ee | P12b line 1453 after abf517e538ee | equals last landing | 2026-11-15 03:03:30 |
| 3 | CURRENT | `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🌍️environment/🧪️tests/🧭️operations/🟦️.ts` | 886f452181e2 | P12b line 1460 after 886f452181e2 | equals last landing | 2026-11-15 03:03:30 |
| 3 | CURRENT | `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🌐️remote-exchange/🟦️.ts` | 57da53c6c018 | R8 line 1775 after 57da53c6c018 | equals last landing | 2026-11-15 05:30:50 |
| 3 | CURRENT | `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🌐️remote-exchange/🧪️tests/🏁️head-rank/🟦️.ts` | 43fd073e097c | R8 line 1787 after 43fd073e097c | equals last landing | 2026-11-15 02:36:12 |
| 3 | CURRENT | `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🌐️remote-exchange/🧪️tests/📡️wave-delta/🟦️.ts` | 0d2760c37826 | R8 line 1786 after 0d2760c37826 | equals last landing | 2026-11-15 02:36:12 |
| 3 | CURRENT | `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🌐️remote-exchange/🧪️tests/🟦️.ts` | 972e3b7a8451 | R8 line 1784 after 972e3b7a8451 | equals last landing | 2026-11-15 05:18:54 |
| 3 | CURRENT | `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🌐️remote-exchange/🧪️tests/🧲️closure-leaves/🟦️.ts` | 47eaed8d8d7c | R8 line 1785 after 47eaed8d8d7c | equals last landing | 2026-11-15 05:31:03 |
| 3 | CURRENT | `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/📡️wave-upload/🟦️.ts` | e3a10405b979 | P13 line 1425 after e3a10405b979 | equals last landing | 2026-11-15 02:36:12 |
| 3 | CURRENT | `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/📡️wave-upload/🧪️tests/🟦️.ts` | 8e4f6d4ca13c | R8 line 1788 after 8e4f6d4ca13c | equals last landing | 2026-11-15 02:36:12 |
| 3 | CURRENT | `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/📡️wave-upload/🧬️schema/🔣️.json` | ece8d6d38f28 | P13 line 1426 after ece8d6d38f28 | equals last landing | 2026-11-15 02:36:12 |
| 3 | CURRENT | `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🔌️load/🟨️.d.mts` | a7b5eacb1c98 | P11 line 1348 after a7b5eacb1c98 | equals last landing | 2026-11-15 02:34:18 |
| 3 | CURRENT | `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🔌️load/🟨️.mjs` | 10f89f44005d | R8 line 1776 after 10f89f44005d | equals last landing | 2026-11-15 05:15:51 |
| 3 | CURRENT | `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🔌️load/🧪️tests/🔁️carried-release/🟦️.ts` | 77b8193f89d3 | P11 line 1361 after 77b8193f89d3 | equals last landing | 2026-11-15 02:34:18 |
| 3 | CURRENT | `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🔌️load/🧪️tests/🟦️.ts` | 05e36f26a00b | R8 line 1789 after 05e36f26a00b | equals last landing | 2026-11-15 05:18:13 |
| 3 | CURRENT | `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🟨️.d.mts` | cb3913696a20 | P11 line 1350 after cb3913696a20 | equals last landing | 2026-11-15 02:34:18 |
| 3 | CURRENT | `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🟨️.mjs` | 7df19582a1fd | P11 line 1351 after 7df19582a1fd | equals last landing | 2026-11-15 02:34:18 |
| 3 | CURRENT | `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🧪️tests/⏱️stages/🟦️.ts` | f8f19d30369c | P11 line 1360 after f8f19d30369c | equals last landing | 2026-11-15 02:34:18 |
| 3 | CURRENT | `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🧪️tests/📈️adaptive-waves/🟦️.ts` | 40a3a5f7b2df | P11 line 1352 after 40a3a5f7b2df | equals last landing | 2026-11-15 02:34:18 |
| 3 | CURRENT | `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🧬️schema/🔣️.json` | 122ae45da26f | P13 line 1423 after 122ae45da26f | equals last landing | 2026-11-15 02:36:12 |
| 3 | CURRENT | `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🧭️steering/🟨️.d.mts` | 341a1d6dd85f | P11 line 1353 after 341a1d6dd85f | equals last landing | 2026-11-15 02:34:18 |
| 3 | CURRENT | `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🧭️steering/🟨️.mjs` | 85d7e4a021dd | P11 line 1354 after 85d7e4a021dd | equals last landing | 2026-11-15 02:34:18 |
| 3 | CURRENT | `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🧭️steering/🧪️tests/📤️offered/🟦️.ts` | a1b792be9ff6 | P11 line 1358 after a1b792be9ff6 | equals last landing | 2026-11-15 02:34:18 |
| 3 | CURRENT | `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🧭️steering/🧬️schema/🔣️.json` | a1b665c6e869 | P11 line 1355 after a1b665c6e869 | equals last landing | 2026-11-15 02:34:18 |
| 3 | CURRENT | `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🧪️tests/📤️offered-once/🟦️.ts` | 06b2d7683ffd | P11 line 1359 after 06b2d7683ffd | equals last landing | 2026-11-15 02:34:18 |
| 3 | CURRENT | `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🧪️tests/🗑️reset-closure/🟦️.ts` | b4d68a1f696b | R8 line 1790 after b4d68a1f696b | equals last landing | 2026-11-14 05:48:51 |
| 3 | CURRENT | `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🧪️tests/🙋️fleet-read/🌍️small-booking/🟦️.ts` | dc75597fd8c6 | P14 line 1517 after dc75597fd8c6 | equals last landing | 2026-11-15 06:02:35 |
| 3 | CURRENT | `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🧪️tests/🙋️fleet-read/🍏️macos-reading/🟦️.ts` | 2cf95b5a4081 | R4 line 1602 after 2cf95b5a4081 | equals last landing | 2026-11-15 04:30:19 |
| 3 | CURRENT | `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🧪️tests/🪜️plan-stages/🟦️.ts` | 1df948a9f9e8 | P12b line 1454 after 1df948a9f9e8 | equals last landing | 2026-11-15 03:03:30 |
| 3 | CURRENT | `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🧭️discovery/🟦️.ts` | 9f62f96c3f5c | P12b line 1455 after 9f62f96c3f5c | equals last landing | 2026-11-15 03:03:30 |
| 3 | CURRENT | `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🩹️closure-repair/🟨️.d.mts` | 8e54b4517db7 | R8 line 1781 after 8e54b4517db7 | equals last landing | 2026-11-15 05:14:03 |
| 3 | CURRENT | `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🩹️closure-repair/🟨️.mjs` | d88a074db8e0 | R8 line 1773 after d88a074db8e0 | equals last landing | 2026-11-15 05:14:03 |
| 3 | CURRENT | `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🩹️closure-repair/🧪️tests/🟦️.ts` | 41a155c5a784 | R8 line 1783 after 41a155c5a784 | equals last landing | 2026-11-15 05:17:46 |
| 3 | CURRENT | `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧠️memory/🟨️.d.mts` | 14ca84038000 | R13b line 1917 after 14ca84038000 | equals last landing | 2026-11-15 05:59:07 |
| 3 | CURRENT | `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧠️memory/🟨️.mjs` | 680344abaa46 | R13b line 1916 after 680344abaa46 | equals last landing | 2026-11-15 05:59:07 |
| 3 | CURRENT | `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧠️memory/🧪️tests/🔣️.json` | 061d5282a76e | R13 line 1899 after 061d5282a76e | equals last landing | 2026-11-15 05:55:37 |
| 3 | CURRENT | `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧠️memory/🧪️tests/🟦️.ts` | f9a08dfe9216 | R13b line 1918 after f9a08dfe9216 | equals last landing | 2026-11-15 05:59:07 |
| 3 | CURRENT | `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧠️memory/🧪️tests/🧬️schema/🔣️.json` | 19fb48b913f9 | R13 line 1900 after 19fb48b913f9 | equals last landing | 2026-11-15 05:55:37 |
| 3 | CURRENT | `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧠️memory/🧬️schema/🔣️.json` | 8f14b30b9547 | R13 line 1881 after 8f14b30b9547 | equals last landing | 2026-11-15 05:55:37 |
| 3 | CURRENT | `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧠️memory/🧽️settle/🟨️.d.mts` | eadc26e9f068 | R2 line 1548 after eadc26e9f068 | equals last landing | 2026-11-15 03:30:21 |
| 3 | CURRENT | `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧠️memory/🧽️settle/🟨️.mjs` | b7d03126208c | R2 line 1547 after b7d03126208c | equals last landing | 2026-11-15 03:30:21 |
| 3 | CURRENT | `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧠️memory/🧽️settle/🧪️tests/🟦️.ts` | 0d6f4f02256e | R2 line 1550 after 0d6f4f02256e | equals last landing | 2026-11-15 03:30:21 |
| 3 | CURRENT | `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧠️memory/🧽️settle/🧬️schema/🔣️.json` | 51df94eee984 | R2 line 1549 after 51df94eee984 | equals last landing | 2026-11-15 03:30:21 |
| 3 | CURRENT | `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧠️memory/🪜️plan-stage/🟨️.d.mts` | 92478f392d66 | P12b line 1456 after 92478f392d66 | equals last landing | 2026-11-15 03:03:30 |
| 3 | CURRENT | `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧠️memory/🪜️plan-stage/🟨️.mjs` | c7c3f50c51f1 | P12b line 1457 after c7c3f50c51f1 | equals last landing | 2026-11-15 03:03:30 |
| 3 | CURRENT | `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧠️memory/🪜️plan-stage/🧪️tests/🟦️.ts` | fe4be7ada4bf | P12b line 1458 after fe4be7ada4bf | equals last landing | 2026-11-15 03:03:30 |
| 3 | CURRENT | `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧠️memory/🪜️plan-stage/🧬️schema/🔣️.json` | fb5fc80b3037 | P12b line 1459 after fb5fc80b3037 | equals last landing | 2026-11-15 03:03:30 |
| 3 | CURRENT | `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧭️invocation/🧪️tests/🛰️plan/🟦️.ts` | 4d2feda28453 | R7b line 2067 after 4d2feda28453 | equals last landing | 2026-11-15 06:42:43 |
| 3 | CURRENT | `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🪪️graph-state/🌍️platform-class/🟦️.ts` | 5c24055e8a5d | P14 line 1504 after 5c24055e8a5d | equals last landing | 2026-11-15 03:05:18 |
| 3 | CURRENT | `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🪪️graph-state/🌍️platform-class/🧪️tests/🐍️.py` | 0cc790967ea1 | P14 line 1508 after 0cc790967ea1 | equals last landing | 2026-11-15 03:05:18 |
| 3 | CURRENT | `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🪪️graph-state/🌍️platform-class/🧪️tests/🔣️.json` | 0d0a86cef12e | P14 line 1507 after 0d0a86cef12e | equals last landing | 2026-11-15 03:05:18 |
| 3 | CURRENT | `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🪪️graph-state/🌍️platform-class/🧪️tests/🟦️.ts` | eaf7c666823e | P14 line 1506 after eaf7c666823e | equals last landing | 2026-11-15 03:05:18 |
| 3 | CURRENT | `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🪪️graph-state/🌍️platform-class/🧬️schema/🔣️.json` | 939557c037bf | P14 line 1505 after 939557c037bf | equals last landing | 2026-11-15 03:05:18 |
| 3 | CURRENT | `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🪪️graph-state/📦️project-records/🟦️.ts` | f563d3213f30 | R11 line 1950 after f563d3213f30 | equals last landing | 2026-11-15 06:13:10 |
| 3 | CURRENT | `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🪪️graph-state/📦️project-records/🧪️tests/🌍️two-platforms/🟦️.ts` | 8ae47d71db43 | P14 line 1514 after 8ae47d71db43 | equals last landing | 2026-11-15 03:05:18 |
| 3 | CURRENT | `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🪪️graph-state/📦️project-records/🧪️tests/🚚️closures/🟦️.ts` | 038d77cd952e | R11 line 1951 after 038d77cd952e | equals last landing | 2026-11-15 06:13:10 |
| 3 | CURRENT | `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🪪️graph-state/📦️project-records/🧪️tests/🟦️.ts` | b6a3ba23c977 | R11 line 1952 after b6a3ba23c977 | equals last landing | 2026-11-15 06:13:10 |
| 3 | CURRENT | `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🪪️graph-state/📦️project-records/🧬️schema/🔣️.json` | 7ae09c6620f0 | R11 line 1953 after 7ae09c6620f0 | equals last landing | 2026-11-15 06:13:10 |
| 3 | CURRENT | `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🪪️graph-state/📦️project-records/🧳️portable/🟦️.ts` | 954f50c9da4a | P14 line 1509 after 954f50c9da4a | equals last landing | 2026-11-15 03:05:18 |
| 3 | CURRENT | `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🪪️graph-state/📦️project-records/🧳️portable/🧪️tests/🐍️.py` | fd17b832a3f8 | P14 line 1513 after fd17b832a3f8 | equals last landing | 2026-11-15 03:05:18 |
| 3 | CURRENT | `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🪪️graph-state/📦️project-records/🧳️portable/🧪️tests/🔣️.json` | 5c58a04b3be1 | P14 line 1512 after 5c58a04b3be1 | equals last landing | 2026-11-15 03:05:18 |
| 3 | CURRENT | `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🪪️graph-state/📦️project-records/🧳️portable/🧪️tests/🟦️.ts` | 6d9f9aeb966c | P14 line 1511 after 6d9f9aeb966c | equals last landing | 2026-11-15 03:05:18 |
| 3 | CURRENT | `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🪪️graph-state/📦️project-records/🧳️portable/🧬️schema/🔣️.json` | 9ad35098b9ae | P14 line 1510 after 9ad35098b9ae | equals last landing | 2026-11-15 03:05:18 |
| 3 | CURRENT | `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🪪️graph-state/🙋️wanted/💰️cost/🧪️tests/🌍️measured-only/🟦️.ts` | 3f0d66c47d67 | P14 line 1516 after 3f0d66c47d67 | equals last landing | 2026-11-15 03:05:18 |
| 3 | CURRENT | `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🪪️graph-state/🙋️wanted/🛠️server/🟦️.ts` | 903c1e29ef39 | R17 line 2026 after 903c1e29ef39 | equals last landing | 2026-11-15 06:27:33 |
| 3 | CURRENT | `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🪪️graph-state/🙋️wanted/🟦️.ts` | 01f40d8e6ce2 | R17 line 2027 after 01f40d8e6ce2 | equals last landing | 2026-11-15 06:27:33 |
| 3 | CURRENT | `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🪪️graph-state/🙋️wanted/🧪️tests/🔁️resume/🟦️.ts` | 3907ef9b178f | R17 line 2032 after 3907ef9b178f | equals last landing | 2026-11-15 06:27:33 |
| 3 | CURRENT | `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🪪️graph-state/🙋️wanted/🧪️tests/🛠️server/💰️priority/🟦️.ts` | c0d729dfa1c1 | R14 line 1859 after c0d729dfa1c1 | equals last landing | 2026-11-15 05:52:42 |
| 3 | CURRENT | `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🪪️graph-state/🙋️wanted/🧪️tests/🟦️.ts` | 2b88a9a1c028 | R14 line 1856 after 2b88a9a1c028 | equals last landing | 2026-11-15 05:52:42 |
| 3 | CURRENT | `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🪪️graph-state/🙋️wanted/🧬️schema/🔣️.json` | 3a65634544a4 | R17 line 2028 after 3a65634544a4 | equals last landing | 2026-11-15 06:27:33 |

## Reverted files: exact identities

None.

## Chain breaks: a landing's before-hash is not the previous landing's after-hash (unrecorded change in between)

- `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/📤️publication/⚙️setup/💰️funding/🟦️.ts`
  - previous: R1b (today, landings.md line 1564, section line 1563 "Package R1b landing (2026-11-15 04:05): R1b landin", edit, before 1c6fef07b43d, after ff6c8978d5af)
  - next: R1b (today, landings.md line 1582, section line 1580 "Package R1b landing (2026-11-15 04:07): R1b landin", edit, before 1c6fef07b43d, after ff6c8978d5af)
- `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/📤️publication/⚙️setup/🟦️.ts`
  - previous: R1b (today, landings.md line 1565, section line 1563 "Package R1b landing (2026-11-15 04:05): R1b landin", edit, before afd7543c167e, after f1d34a93fcc4)
  - next: R1b (today, landings.md line 1583, section line 1580 "Package R1b landing (2026-11-15 04:07): R1b landin", edit, before afd7543c167e, after f1d34a93fcc4)
- `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/📤️publication/⚙️setup/🧪️tests/⏳️busy-funding/🟦️.ts`
  - previous: R1b (today, landings.md line 1567, section line 1563 "Package R1b landing (2026-11-15 04:05): R1b landin", edit, before 920a3db36861, after 2fd069b2067e)
  - next: R1b (today, landings.md line 1585, section line 1580 "Package R1b landing (2026-11-15 04:07): R1b landin", edit, before 920a3db36861, after 2fd069b2067e)
- `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📖️texts/🔣️.json`
  - previous: R1b (today, landings.md line 1566, section line 1563 "Package R1b landing (2026-11-15 04:05): R1b landin", edit, before b4021e84bae0, after e091c5bff889)
  - next: R1b (today, landings.md line 1584, section line 1580 "Package R1b landing (2026-11-15 04:07): R1b landin", edit, before b4021e84bae0, after e091c5bff889)
- `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-execution/🛰️agent/🟦️.ts`
  - previous: R13b (today, landings.md line 1914, section line 1907 "Package R13b landing (2026-11-15 05:59): lender re", edit, before 0e404f51b876, after 880ef597fdde)
  - next: R17 (today, landings.md line 2017, section line 2016 "Package R17 landing (2026-11-15 06:27): closure-ba", edit, before 9065969bf362, after 6d36265b8420)
- `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-execution/🛰️sidecar/🕸️graph/🟦️.ts`
  - previous: P13 (today, landings.md line 1419, section line 1418 "Package P13 landing (2026-11-15 02:36): the servin", copy, before 28ff84b044bb, after 3802e6e74219)
  - next: R17 (today, landings.md line 2019, section line 2016 "Package R17 landing (2026-11-15 06:27): closure-ba", edit, before 7de7f8c5272b, after 6d8341747bde)
- `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏡️resident/🏗️composition/🌐️host/🟨️.mjs`
  - previous: R7 (today, landings.md line 1639, section line 1633 "Package R7 landing (2026-11-15 05:07): a graph ser", copy, before cd46b7c26290, after dd8d1b04cdf8)
  - next: R7b (today, landings.md line 2057, section line 2056 "Package R7b landing (2026-11-15 06:41): the mount ", copy, before 8d46a0c5842d, after 22f18d24c96f)
- `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏡️resident/🏛️service/🧯️failure/🟨️.mjs`
  - previous: P6 (earlier, landings.md line 695, section line 685 "Package P6 landing (2026-11-14 20:34): macOS file ", copy, before 4f9b75d00132, after 0450f5b7a787)
  - next: P6 (today, landings.md line 1306, section line 1300 "Package P6 landing (2026-11-15 00:04): macOS file ", copy, before ca750e30de57, after 8a1cd68de138)
- `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏡️resident/🚪️client/🛰️plan/🟨️.d.mts`
  - previous: R7 (today, landings.md line 1646, section line 1633 "Package R7 landing (2026-11-15 05:07): a graph ser", copy, before 6798ecb0bf13, after d556ebf1cb62)
  - next: R7b (today, landings.md line 2061, section line 2056 "Package R7b landing (2026-11-15 06:41): the mount ", copy, before e33ba0f86c43, after 67595dc1117c)
- `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏡️resident/🚪️client/🛰️plan/🟨️.mjs`
  - previous: R7 (today, landings.md line 1647, section line 1633 "Package R7 landing (2026-11-15 05:07): a graph ser", copy, before 05759400ce81, after 3ec902b6cb0b)
  - next: R7b (today, landings.md line 2062, section line 2056 "Package R7b landing (2026-11-15 06:41): the mount ", copy, before ed29c72e6e79, after 57e5d3ddfc1f)
- `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏡️resident/🟨️.mjs`
  - previous: R7 (today, landings.md line 1648, section line 1633 "Package R7 landing (2026-11-15 05:07): a graph ser", copy, before 5ce69b085498, after f3b8c525783d)
  - next: R7b (today, landings.md line 2063, section line 2056 "Package R7b landing (2026-11-15 06:41): the mount ", copy, before 034090b16c0a, after 44b06ede6784)
- `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏡️resident/🧪️tests/🛡️consent-service/🟦️.ts`
  - previous: R7 (today, landings.md line 1651, section line 1633 "Package R7 landing (2026-11-15 05:07): a graph ser", copy, before fb875981233f, after 4f2bf2a0e69b)
  - next: R7b (today, landings.md line 2065, section line 2056 "Package R7b landing (2026-11-15 06:41): the mount ", copy, before f3aacf465e0c, after 5b20b9a0397b)
- `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏡️resident/🧪️tests/🧯️failure/🟦️.ts`
  - previous: P6 (earlier, landings.md line 705, section line 685 "Package P6 landing (2026-11-14 20:34): macOS file ", copy, before 0c1c8b780d6f, after fb3142d5705b)
  - next: P6 (today, landings.md line 1313, section line 1300 "Package P6 landing (2026-11-15 00:04): macOS file ", copy, before a3ef92ec0cb3, after 9e622c568cd0)
- `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏡️resident/🧬️schema/🔣️.json`
  - previous: P6 (earlier, landings.md line 971, section line 960 "Package P6 landing (2026-11-14 21:48): macOS file ", copy, before 1e95a0c97292, after b96bce82c510)
  - next: P12 (today, landings.md line 1387, section line 1382 "Package P12 landing (2026-11-15 02:35): the plan o", copy, before cea15465f529, after ecc5ebf57f73)
- `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📖️texts/🔣️.json`
  - previous: P6 (earlier, landings.md line 707, section line 685 "Package P6 landing (2026-11-14 20:34): macOS file ", copy, before 5def35dc04c7, after 697cf391e2fb)
  - next: P12 (today, landings.md line 1388, section line 1382 "Package P12 landing (2026-11-15 02:35): the plan o", copy, before 0e5dcc1b6cb8, after 49a1ee8313fb)
- `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🔌️load/🟨️.mjs`
  - previous: P9 (earlier, landings.md line 1269, section line 1262 "Package P9 landing (2026-11-14 23:38): adaptive ow", copy, before b372994cba87, after b1dd6baa46ad)
  - next: P11 (today, landings.md line 1349, section line 1347 "Package P11 landing (2026-11-15 02:34): the offer ", merge, before df20a6ab73e9, after b7ab232c4a20)
- `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧠️memory/🟨️.d.mts`
  - previous: R13 (today, landings.md line 1883, section line 1880 "Package R13 landing (2026-11-15 05:55): lent class", edit, before dc2693681482, after 46b1b6f65f6a)
  - next: R13b (today, landings.md line 1917, section line 1907 "Package R13b landing (2026-11-15 05:59): lender re", edit, before 2dad0709d996, after 14ca84038000)
- `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧠️memory/🟨️.mjs`
  - previous: R13 (today, landings.md line 1882, section line 1880 "Package R13 landing (2026-11-15 05:55): lent class", edit, before 5f0e1443442d, after 19d5cc1fd187)
  - next: R13b (today, landings.md line 1916, section line 1907 "Package R13b landing (2026-11-15 05:59): lender re", edit, before a2f39f322816, after 680344abaa46)
- `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧭️invocation/🛰️plan/🟦️.ts`
  - previous: P6 (earlier, landings.md line 709, section line 685 "Package P6 landing (2026-11-14 20:34): macOS file ", copy, before 053247e7d13a, after 978273bc5cb6)
  - next: P6 (today, landings.md line 1314, section line 1300 "Package P6 landing (2026-11-15 00:04): macOS file ", copy, before a8ed0e817ccd, after d23eb57654b2)
- `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧭️invocation/🛰️plan/🟦️.ts`
  - previous: R7 (today, landings.md line 1655, section line 1633 "Package R7 landing (2026-11-15 05:07): a graph ser", copy, before d23eb57654b2, after 7daa900c13a6)
  - next: R7b (today, landings.md line 2066, section line 2056 "Package R7b landing (2026-11-15 06:41): the mount ", copy, before e19e22c47fd3, after ae106dff4acb)
- `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧭️invocation/🧪️tests/🛰️plan/🟦️.ts`
  - previous: R7 (today, landings.md line 1656, section line 1633 "Package R7 landing (2026-11-15 05:07): a graph ser", copy, before 369900d002bb, after 7abcaf6caa30)
  - next: R7b (today, landings.md line 2067, section line 2056 "Package R7b landing (2026-11-15 06:41): the mount ", copy, before 6216af754373, after 4d2feda28453)
- `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🪪️graph-state/🙋️wanted/🛠️server/🟦️.ts`
  - previous: R14 (today, landings.md line 1854, section line 1850 "Package R14 landing (2026-11-15 05:52): the fleet ", file, before 0bf7f4144108, after 2064820f6dcc)
  - next: R17 (today, landings.md line 2026, section line 2016 "Package R17 landing (2026-11-15 06:27): closure-ba", edit, before 286b9baaa73b, after 903c1e29ef39)
- `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🪪️graph-state/🙋️wanted/🟦️.ts`
  - previous: R14 (today, landings.md line 1852, section line 1850 "Package R14 landing (2026-11-15 05:52): the fleet ", file, before 213223901867, after 71857f395a46)
  - next: R17 (today, landings.md line 2027, section line 2016 "Package R17 landing (2026-11-15 06:27): closure-ba", edit, before 28be7786a193, after 01f40d8e6ce2)
- `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🪪️graph-state/🙋️wanted/🧬️schema/🔣️.json`
  - previous: R14 (today, landings.md line 1851, section line 1850 "Package R14 landing (2026-11-15 05:52): the fleet ", file, before 6290f77eb9f0, after 1092ecac1e64)
  - next: R17 (today, landings.md line 2028, section line 2016 "Package R17 landing (2026-11-15 06:27): closure-ba", edit, before c57d5ecb0a60, after 3a65634544a4)

## Distinctive-string checks (result files)

| package | file | string | result file | status | file mtime |
|---|---|---|---|---|---|
| R7 | `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏡️resident/🏗️composition/🌐️host/🟨️.mjs` | `launchd route was refused` | result-R7.md | PASS | 2026-11-15 06:47:56 |
| R8 | `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🩹️closure-repair/🟨️.mjs` | `obtainClosureLeafV1` | result-R8.md | PASS | 2026-11-15 05:14:03 |
| R8 | `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🌐️remote-exchange/🟦️.ts` | `restoreClosuresV1` | result-R8.md | PASS | 2026-11-15 05:30:50 |
| R16 | `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/🔏️key/🟦️.ts` | `normalizeRustcArgumentsV1` | result-R16.md | PASS | 2026-11-15 06:05:59 |
| R16 | `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/🧭️remap/🟦️.ts` | `cargo-build-dir` | result-R16.md | PASS | 2026-11-15 06:05:04 |

## Sections included

- line 1300: Package P6 landing (2026-11-15 00:04): macOS file access consent, one identity for every l (15 records)
- line 1347: Package P11 landing (2026-11-15 02:34): the offer pass bounded and persisted, setup carrie (14 records)
- line 1382: Package P12 landing (2026-11-15 02:35): the plan of a wave runs in named stages under the  (14 records)
- line 1418: Package P13 landing (2026-11-15 02:36): the serving machine uploads its generation as a de (12 records)
- line 1443: Package P12b landing (2026-11-15 03:02): the closure-store sweep holds no entry rows (it t (18 records)
- line 1488: Package P14 landing (2026-11-15 03:05): the graph exchange serves every platform class, pr (29 records)
- line 1533: Package R1 landing (2026-11-15 03:20): the dependency store wait names its holder (pid, co (4 records)
- line 1542: Package R2 landing (2026-11-15 03:30): the service settles memory after an answered plan,  (10 records)
- line 1563: Package R1b landing (2026-11-15 04:05): R1b landing (a command that runs beneath an ancest (4 records)
- line 1573: Package R2b landing (2026-11-15 04:05): the idle rule of the host service takes the in-fli (2 records)
- line 1580: Package R1b landing (2026-11-15 04:07): R1b landing (a command that runs beneath an ancest (5 records)
- line 1591: Package R4 landing (2026-11-15 04:30): the macOS available memory is the kernel's (free pl (11 records)
- line 1618: Package R6 landing (2026-11-15 05:01): R6 landing (the obsolete dependency retirement wait (7 records)
- line 1633: Package R7 landing (2026-11-15 05:07): a graph service without access to the worktree neve (23 records)
- line 1695: Package R9 landing (2026-11-15 05:26): the plan-stage guard of the slice pool settles the  (12 records)
- line 1749: Package R6b landing (2026-11-15 05:28): R6b landing (the dependency funding is bound only  (10 records)
- line 1771: Package R8 landing (2026-11-15 05:31): a missing build input closure leaf is a cache miss  (21 records)
- line 1824: Package R10 landing (2026-11-15 05:47): nothing is compiled twice (worker policy for CARGO (12 records)
- line 1850: Package R14 landing (2026-11-15 05:52): the fleet read waits while the serving machine's p (10 records)
- line 1880: Package R13 landing (2026-11-15 05:55): lent class in host capacity, readiness marker of l (20 records)
- line 1907: Package R13b landing (2026-11-15 05:59): lender readiness from the live toolchain instead  (12 records)
- line 1926: Package R6c landing (2026-11-15 06:05): R6c landing (the dependency image declares the baz (2 records)
- line 1932: Package R11 landing (2026-11-15 06:10): setup outputs are restored from the fleet by class (21 records)
- line 1972: Package R16 landing (2026-11-15 06:20): attribution of unit lookups (named key components, (17 records)
- line 2016: Package R17 landing (2026-11-15 06:27): closure-based debounced restart with drain for the (16 records)
- line 2056: Package R7b landing (2026-11-15 06:41): the mount boundary: reset-marker lookups in the ac (11 records)

