# R116: census 06:00 (run 06:40:53 UTC): is the fleet converging?

Script: `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION/🧰️scripts/R116/census.ts` (read-only; imports the build library read-only; writes no file, only stdout). `L/` in the output below is `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/`; lines over 330 characters are cut.

Rerun (the run below used a 200 s budget and took 125 s on its own clock, 129 s wall):
```
cd /Users/ueli/Documents/semio && R116_BUDGET=200 nice -n 10 .cache/b/t/m/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION/🧰️scripts/R116/census.ts"
```
Without `R116_BUDGET` it stops judging at 118 s (a run at 06:32 judged 240 owners in 115 s at load 25). Knobs: `R116_N` (judged owners, default 300), `R116_KEYED` (keyed owners, default 310), `R116_WAVE` (wave directory name).

## Where the records are (correction to the brief)
`/Users/Shared/semio-store/v1` is the replica store of the shared worktree (file segments, `origins/*/pool.json`, `ready.json`), not the store of owner records. Owner records are action-cache entries of the cache server (`.cache/f/cache-server/v1/main/ac/<2 hex>/<entry key>`, one file per `graphEntryKeyV1(projectRecordStateV1(...))`, with `record.br` and one `closure-<recipeKey>.br` blob per shared closure entry). The script reads them through the fleet client (`fetchGraphEntryV1`) exactly as pool and reader do, and reads `origins/*/pool.json` from the replica store. Two consequences found while building it:
1. The file time of an action-cache file is the time of the last read or write, not the creation (median 181 min after the record's own `createdMs`, 298 of 300). All rates here therefore use the `createdMs` of the entry note, not file times.
2. The newest wave directory is often a partial wave (a 46-owner "late" wave appeared at 06:32). The script takes the newest wave whose `units.json` has at least 80 percent of the largest one: 4619 owners now (4774 is the project count of a read; the wave holds fewer owners).

## How the verdict is made
For each sampled owner: owner job, record key and state digest of the CURRENT code (record revision `["semio.graph-record/2",null,null]`, wave globals), `fetchGraphEntryV1`, `unpackRecordV1` + `localRecordV1` (the taker's localisation), every non-closure row of the record probed with `observationRowFreshV1` (what the reader's `projectRowsHoldV1` ends in), and every closure entry the record carries unpacked, localised with `localBundleV1` and probed row by row (rows and layers, one probe per distinct row), which is the verdict `closuresHoldOf` and a taker give. Not run: the reader's own `settleClosureRowsV1` pass (see "Not verified"). Sample: the first 310 owners of a seeded shuffle, interleaved 3:1:1 over rust/cargo, typescript, other; all 310 have a store file; 300 judged; totals are stratified by kind (rust/cargo 4390, typescript 62, other 167).

## Output (run 2026-11-17 06:40:53 UTC)
```
R116 census 2026-11-17T06:40:53.870Z UTC; host ueli-mac; elapsed budget 200 s
wave 1421c609bc4ad940 created 11-17 06:36:46 UTC, 4619 owners (rust/cargo 4390, typescript 62, other 167); record revision ["semio.graph-record/2",null,null]; globals {"projectAuthority":"506650ee","registryDigest":"c432e2b3","commandRevision":"a4c135c3","late":true}
phase A (key, state digest, store file time) covered 310 of 4619 owners in 49.1 s (0 key errors); phase B judged 300 owners in 76.1 s; 0 stored owners left unjudged by the time budget
waves on disk 8; 1 newer or older partial waves (under 80% of the largest units.json) skipped; the chosen wave is the newest full one
landings used: R113 05:33:32-05:41:00 UTC; R114 06:00:00-06:01:23 UTC; R115 06:01:10-06:03:00 UTC

== 1. Owner records (verdict = rows of the record probed against the live tree + every closure entry it carries probed row by row, the verdict a taker gives) ==
kind        owners  keyed  stored(file)  none(file)  judged  accepted  rejected  unreadable
rust/cargo    4390    186          186           0     180       149        31           0
typescript      62     62           62           0      60        23        37           0
other          167     62           62           0      60        39        21           0
estimate for all 4619 owners (stratified by kind from the sample): accepted 3766 (81.5%), rejected 853 (18.5%), stored but unreadable 0, no store file 0 (0.0%)
estimated rejected owners by reason class (stratified): closure-entry only 536 (11.6%); record-row only 233 (5.1%); both 84 (1.8%)  [record-row = the record's own rows, mostly readdirNames of a project directory that lists machine-local build output; closure-entry = a shared closure entry the record carries walked a file that ch ...
top rejection reasons (judged owners):
    46  record row: readdirNames
    17  closure entry stale (analysis)
     9  closure entry stale (analysis) | closure entry stale (analysis+file) | closure entry stale (file)
     3  closure entry stale (analysis) | closure entry stale (analysis+file) | closure entry stale (file) | record row: readdirNames
     2  record row: exists | record row: readdirNames
     2  closure entry stale (analysis) | closure entry stale (analysis+directoryExists) | closure entry stale (analysis+directoryExists+file) | closure entry stale (analysis+file) | closure entry stale (file) | record row: readdirNames
     1  closure entry stale (analysis+directoryExists) | closure entry stale (analysis+directoryExists+file) | closure entry stale (analysis+directoryExists+file+typescriptRealpath) | closure entry stale (analysis+directoryExists+typescriptRealpath) | closure entry stale (analysis+file) | closure entry stale (file)
     1  closure entry stale (analysis) | closure entry stale (analysis+file) | closure entry stale (analysis+file+typescriptRealpath) | closure entry stale (analysis+typescriptRealpath) | closure entry stale (file) | record row: readdirNames
  rust/cargo: 21x closure entry stale (analysis); 10x record row: readdirNames; 6x closure entry stale (analysis+file); 6x closure entry stale (file)
  typescript: 33x record row: readdirNames; 6x closure entry stale (analysis); 6x closure entry stale (analysis+file); 6x closure entry stale (file)
  other: 13x record row: readdirNames; 7x closure entry stale (analysis); 6x closure entry stale (analysis+file); 6x closure entry stale (file)
  readdirNames diagnosis: @semio-tech/playbook-playbook ️playbook/🗿️artifacts/📖️playbook/📦️packages/🟦️typescript recorded="46ea285948a3bf550813eb0238ddc012dadcdd53f723b35230161be6eb2890db" now=["dist","node_modules","package.json","📋️project.json","📜️script.ts","📤️dist"]
  readdirNames diagnosis: @semio-tech/ui-styling-tokens 🧰️framework/🔨️modules/🖱️ui/🎨️styling/📦️packages/🦀️rust recorded="5aff136cdea39897aba857d5e39c4c7b139bf65c25dec44c9a05c68b000c7925" now=["Cargo.toml","dist","📋️project.json","📜️script.ts"]
  readdirNames diagnosis: @semio-tech/stdio-txt ugins/🗄️stdio/🗿️artifacts/🔤️txt/📦️packages/🟦️typescript recorded="834a31c0dd829d4f9587a21f132f813561b52a3f141391e41faa2afbf02bac51" now=["dist","node_modules","package.json","🏅️standards","📋️project.json","📜️script.ts","📤️dist"]
  readdirNames diagnosis: @semio-tech/mathematical-equation thematical/🗿️artifacts/➗️equation/📦️packages/🟦️typescript recorded="46ea285948a3bf550813eb0238ddc012dadcdd53f723b35230161be6eb2890db" now=["dist","node_modules","package.json","📋️project.json","📜️script.ts","📤️dist"]
  readdirNames diagnosis: @semio-tech/hub-compositions 🌎️hub/🧩️compositions recorded="b055d32c07e300599a77b1cfa408caa2ebf938d3bbde7f249cc77d76b220bde1" now=[".DS_Store","✒️writer","➗️mathematical","🀄️wfc","🌀️procedural","🌊️flow","🌍️gis","🌿️vcs","🎞️animate","🎥️shooting","🎪️demonstrator","🎬️sequence","🏗️fem","🏙️bim","🏛️architect", ...
partial verdicts (a closure entry over 7 s or the budget ended): 0

== 2. Closure entries referenced by the judged owners ==
distinct shared closure entries referenced (by blob digest): 484; distinct recipes 287; fresh now 163; stale 321; not probed 0; entry probe time 56 s, rows probed 7849491
top 15 files that staled closure entries (entries = distinct stale shared entries containing a failing row of the file; owners = judged records carrying them):
entries owners born-stale  mtime(UTC)         age-min  kinds                   class                              file   [born-stale = judged records carrying the failing file that were created AFTER the file changed, i.e. the record shipped an entry that was already stale when it was written]
    262    333       2/39  11-16 20:28:07       613  analysis                framework other                    🧰️framework/🔨️modules/🏃️process/🪓️termination/🟦️.ts
    199    335       0/20  11-16 17:33:55       787  file                    framework other                    L/⚡️caching/🩹️patches/nx@23.2.0.patch
    188    214       0/37  11-17 06:01:48        39  analysis                build library (outside tool entries) L/🏗️build/🕸️graph/🧮️derived-nodes/🧩️closure-kinds/🏭️walk-farm/👷️child/🟨️.mjs
    188    214       0/37  11-17 06:34:31         6  analysis                build library (outside tool entries) L/🏗️build/🕸️graph/📥️inference/📦️project-nodes/🟨️.mjs
    188    214       0/37  11-17 01:00:00       341  analysis                build library (outside tool entries) L/🏗️build/🪪️graph-state/📦️project-records/🧳️portable/🟦️.ts
    188    214       0/37  11-17 01:53:16       288  analysis                build library (outside tool entries) L/🏗️build/🕸️graph/📥️inference/📦️project-nodes/🏊️project-farm/👷️worker/🟨️.mjs
    182    208       0/36  11-17 06:00:01        41  analysis                build library (outside tool entries) L/🏗️build/👁️observation/🟨️.mjs
    182    208       0/36  11-16 19:00:16       701  analysis                build library (outside tool entries) L/🏗️build/🕸️graph/🧮️derived-nodes/🧩️closure-kinds/🟨️.mjs
    182    208       0/36  11-17 04:00:00       161  analysis                build library (outside tool entries) L/🏗️build/🕸️graph/📥️inference/🧭️script-inputs/🟨️.mjs
    182    208       0/36  11-16 18:00:12       761  analysis                build library (outside tool entries) L/🏗️build/👁️observation/📥️reuse/🟨️.mjs
    182    208       0/36  11-16 22:34:40       486  analysis                build library (outside tool entries) L/🏗️build/🕸️graph/📥️inference/📦️project-nodes/🏊️project-farm/🏊️pool/🟨️.mjs
    182    208       0/36  11-17 06:34:31         6  analysis                build library (outside tool entries) L/🏗️build/🕸️graph/📥️inference/📦️project-nodes/🏊️project-farm/🟨️.mjs
    182    208       0/36  11-16 21:42:00       539  analysis                build library (outside tool entries) L/🏗️build/🧠️memory/🪜️plan-stage/🟨️.mjs
    182    208       0/36  11-17 04:00:00       161  analysis                build library (outside tool entries) L/🏗️build/🕸️graph/📥️inference/🧭️script-inputs/💾️entries/🟨️.mjs
    182    208       0/36  11-16 19:40:14       661  analysis                build library (outside tool entries) L/🏗️build/🏡️resident/🏛️service/🧯️failure/🟨️.mjs
failing files of closure entries: 729; in the build library 102; the rest 627; stale entries with at least one build-library failing file 257 of 321
mtime of failing closure-entry files, 30-minute buckets back from now (files): 0-30min:3  30-60min:2  150-180min:2  210-240min:1  270-300min:3  330-360min:1  480-510min:1  510-540min:1  570-600min:1  600-630min:1  630-660min:1  660-690min:1
failing closure-entry files by landing window (start-1 min to end+5 min): R113:0  R114:2  R115:1  none of the three:727
failing library files with mtime inside a landing window: 👁️observation/🟨️.mjs 06:00:01; 🕸️graph/🧮️derived-nodes/🧩️closure-kinds/🏭️walk-farm/👷️child/🟨️.mjs 06:01:48
top files/directories failing rows of the RECORD itself (not of a closure entry): owners  mtime  kinds  file
     1  11-16 17:57:53 readdirNames   ✏️s/🔌️plugins/📖️playbook/🗿️artifacts/📖️playbook/📦️packages/🟦️typescript
     1  11-15 06:09:29 readdirNames   🧰️framework/🔨️modules/🖱️ui/🎨️styling/📦️packages/🦀️rust
     1  10-15 17:03:33 exists         🧰️framework/🔨️modules/🖱️ui/🎨️styling/🔤️tokens/🦀️.rs
     1  11-16 17:57:57 readdirNames   ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/📦️packages/🟦️typescript
     1  11-16 17:57:44 readdirNames   ✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/📦️packages/🟦️typescript
     1  11-11 03:30:45 readdirNames   🌎️hub/🧩️compositions
     1  11-16 17:57:57 readdirNames   ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔺️stl/📦️packages/🟦️typescript
     1  11-11 03:26:29 readdirNames   🧰️framework/🔨️modules/🧪️test
row memo: 7972831 hits, 61195 probes

== 3. Rate ==
sample: 310 keyed owners (scaled x14.90 to 4619), 300 judged (accepted rates scaled x15.40). Store file time vs the note's createdMs for 300 judged owners: median lag 180.9 min, 298 of 300 files touched more than 2 min after the record was created (a store file time is NOT a creation time for those)
last 10 min: owners with a store file touched 4559; records created (note createdMs)   62 (6.2/min); of those accepted now   31 (3.1/min)
last 30 min: owners with a store file touched 4589; records created (note createdMs)  277 (9.2/min); of those accepted now  169 (5.6/min)
last 60 min: owners with a store file touched 4604; records created (note createdMs)  708 (11.8/min); of those accepted now  523 (8.7/min)
per origin: records created in the judged sample by note producer (scaled to all owners) w10/w30/w60 | accepted now w60 | claims granted in the claim log w10/w30/w60 | pool.json (ownersPerMinute, claimed, published, stored, children, wave)
  g1-m01        15   46  123 |  108 |   97  179  331 | 2.8/min claimed 20 published 13 stored 389 children 6 reason nothing-fits wave de0985b7 updated 06:42:56
  g1-m02        31   46  139 |   92 |   47  100  228 | 1.6/min claimed 11 published 6 stored 377 children 5 reason yielding wave de0985b7 updated 06:42:56
  g1-m03        15   62  123 |   77 |   41  236  308 | 3.2/min claimed 17 published 16 stored 421 children 5 reason yielding wave de0985b7 updated 06:42:56
  g1-m04         0    0    0 |    0 |    0    0    0 | 0/min claimed 3 published 0 stored 265 children 9 reason nothing-fits wave 40bfa3e7 updated 22:07:02
  g1-m05         0    0    0 |    0 |    0    2   45 | 0.2/min claimed 3 published 2 stored 43 children 1 reason yielding wave 6501c811 updated 15:42:06
  g1-m06         0    0    0 |    0 |    0    0    0 | 0.2/min claimed 2 published 2 stored 1210 children 1 reason nothing-fits wave 1dd2d90c updated 09:41:46
  g1-m07         0   62  139 |   77 |   97  190  476 | 4.4/min claimed 26 published 21 stored 463 children 4 reason nothing-fits wave de0985b7 updated 06:42:54
  g1-m08         0   31  108 |  108 |   79  143  290 | 2.6/min claimed 20 published 14 stored 344 children 4 reason nothing-fits wave 1421c609 updated 06:42:53
  g1-m10         0   31   77 |   62 |   26   59  159 | 2.2/min claimed 19 published 11 stored 367 children 4 reason nothing-fits wave 1421c609 updated 06:42:55
  g1-server      0    0    0 |    0 |   11   56   81 | 0/min claimed 1 published 0 stored 16 children 2 reason nothing-fits wave de0985b7 updated 06:42:58
size of the owners the pools must refresh (wave units): accepted: 211 owners, mean 432 MB expected memory, mean 67 s expected derivation | rejected, closure-entry class: 40 owners, mean 187 MB expected memory, mean 65 s expected derivation | rejected, record-row only: 49 owners, mean 2202 MB expected memory, mean 108 s expected  ...
waves the pools follow right now: 1421c609, 40bfa3e7, de0985b7, 6501c811, 1dd2d90c; pool reasons: m10=nothing-fits m04=nothing-fits m03=yielding m02=yielding m05=yielding server=nothing-fits m07=nothing-fits m08=nothing-fits m06=nothing-fits m01=nothing-fits; waves on disk created in the last 30 min: 8
accepted now (estimate) 3766 of 4619 = 81.5%; remaining 853
projection under a landing freeze: closure-entry class 619 owners / 8.7-5.6 per min = 1.2 to 1.8 h; record-row-only class 233 owners does not converge by refreshing (a refresh on a machine without the build output is rejected on a machine with it)
projected time to all acceptable, nothing re-staled: at the 30-min rate (5.65/min) 2.5 h; at the 60-min rate (8.72/min) 1.6 h

== 4. Re-staling by landings ==
per landing: judged records written before the landing start; of those rejected now; rejected now with a failing file (record row or closure entry) whose mtime is within [start-1 min, end+5 min] (= staled by that landing, they were acceptable before it if their rows held then); scaled to all owners
  R113 05:33:32-05:41:00 UTC: written before 246; rejected now 74; staled by files changed in this window 0 (0.0% of those before; ~0 of all owners); staled by this window alone 0
  R114 06:00:00-06:01:23 UTC: written before 270; rejected now 81; staled by files changed in this window 37 (13.7% of those before; ~570 of all owners); staled by this window alone 0
  R115 06:01:10-06:03:00 UTC: written before 270; rejected now 81; staled by files changed in this window 37 (13.7% of those before; ~570 of all owners); staled by this window alone 0
records written since R113 start: 54 judged, 39 still accepted, 15 already rejected
of 89 rejected judged records: 37 fail on a file changed in one of the three landing windows, 52 only on files changed outside them (peer edits), 0 without a located file
rejected judged records by when their failing files changed relative to the record's createdMs: born stale (every failing file changed BEFORE the record was written) 49; went stale (every failing file changed AFTER) 33; mixed 7
records created in the last 60 min: 46 judged, 34 accepted, 12 rejected
age of the judged records (note createdMs): <1 h: 34 accepted / 12 rejected; 1-3 h: 81 accepted / 22 rejected; 3-6 h: 95 accepted / 14 rejected; 6-12 h: 1 accepted / 3 rejected; >12 h: 0 accepted / 38 rejected
judged 300: accepted 211 = 70.3% +/- 5.2 points (95%, unstratified)
records created in the last 90 min and rejected now (name, kind, created UTC, origin, up to 3 failing files with mtime UTC):
  @semio-tech/ui-styling-tokens                rust/cargo 06:17:49 g1-m10   🧰️framework/🔨️modules/🖱️ui/🎨️styling/📦️packages/🦀️rust 06:09:29 | 🧰️framework/🔨️modules/🖱️ui/🎨️styling/🔤️tokens/🦀️.rs 17:03:33
  @semio-tech/stdio-txt                        typescript 05:50:45 g1-m02   ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/📦️packages/🟦️typescript 17:57:57
  @semio-tech/mathematical-equation            typescript 05:37:24 g1-m05   ugins/➗️mathematical/🗿️artifacts/➗️equation/📦️packages/🟦️typescript 17:57:44
  @semio-tech/stdio-stl                        typescript 05:50:08 g1-m07   ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔺️stl/📦️packages/🟦️typescript 17:57:57
  @semio-tech/framework-test                   other      05:33:51 g1-m01   🧰️framework/🔨️modules/🧪️test 03:26:29
  @semio-tech/trinity-jack-rs                  rust/cargo 06:38:14 g1-m02   ✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/📦️packages/🦀️rust 17:57:53
  @semio-tech/stdio-jpg                        typescript 05:35:54 g1-m01   ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📸️jpg/📦️packages/🟦️typescript 17:57:57
  @semio-tech/stdio-obj                        typescript 05:14:39 g1-m01   ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗽️obj/📦️packages/🟦️typescript 17:57:57
  @semio-tech/stdio-csv                        typescript 05:32:49 g1-m01   ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📊️csv/📦️packages/🟦️typescript 17:57:56
  @semio-tech/stdio-zip                        typescript 05:30:34 g1-m02   ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/📦️packages/🟦️typescript 17:57:54
  @semio-tech/hub-stdio-test-oracle            other      06:39:39 g1-m02   🌎️hub/🧩️compositions/🗄️stdio/🔮️oracles 17:57:59
  @semio-tech/block-5d                         typescript 06:18:17 g1-m07   ✏️s/🔌️plugins/🧱️block/🗿️artifacts/🖐️5d/📦️packages/🟦️typescript 17:57:58
  @semio-tech/framework-os-host-rs             rust/cargo 05:15:30 g1-m07   🧰️framework/🛍️products/💻️os/🖥️host/📦️packages/🦀️rust 03:27:02
  @semio-tech/course-react                     other      05:17:14 g1-m01   ork/🛍️products/📚️course/🎯️targets/⚛️react/📦️packages/🟦️typescript 17:57:41
rejected judged records by age and reason class (record-row only / closure-entry only / both): <1 h: 12/0/0; 1-3 h: 22/0/0; 3-6 h: 14/0/0; 6-12 h: 1/1/1; >12 h: 0/31/7
rejected records created in the last 6 h with a stale closure entry: 0; failing build-library files among them: 0
all failing build-library files of closure entries (entries owners mtime): /Users/ueli/Documents/semio/L/⚡️caching/🩹️patches/nx@23.2.0.patch 199 335 17:33:55 ;; 🕸️graph/🧮️derived-nodes/🧩️closure-kinds/🏭️walk-farm/👷️child/🟨️.mjs 188 214 06:01:48 ;; 🕸️graph/📥️inference/📦️project-nodes/🟨️.mjs 188 214 06:34:31 ;; 🪪️graph-state/📦️pro ...
recipes of the 8 stale shared entries carried by most judged records (entry name, owners, recipe start):
  closure-0e69de114025 13 {"fn":"tool-closure","name":"runtime-identity","view":"scheduling"}
  closure-4e568b6a6e75 13 {"fn":"tool-closure","name":"package-router","view":"scheduling"}
  closure-998b5d03430b 13 {"fn":"tool-closure","name":"orphans-parent-script","view":"scheduling"}
  closure-4ec28ef75054 9 {"fn":"tool-closure","name":"testing-budget","view":"scheduling"}
  closure-7d8e0c1b3826 9 {"fn":"tool-closure","name":"runtime-sqlite","view":"scheduling"}
  closure-9e9f6afebdaa 9 {"fn":"tool-closure","name":"free-space","view":"scheduling"}
  closure-e330beb15180 8 {"fn":"tool-closure","name":"package-budget","view":"scheduling"}
  closure-8c03f15b76e2 7 {"fn":"tool-closure","name":"runtime-parse","view":"scheduling"}
total seconds 125.3
```

## Reading
**Fraction done.** 81.5 percent of the 4619 owners of the current wave (3766; 95 percent interval about +/- 6 points, driven by rust/cargo: 149 of 180) have a record that passes the row and closure-entry probes on this machine right now; typescript 23 of 60, other 39 of 60. Every one of the 310 keyed owners has a store file under the current key (none missing): the store does not lack records, it holds records that a reader rejects. Rejected: 853 owners (18.5 percent) in three classes: closure-entry only 536 (11.6 percent), record-row only 233 (5.1 percent), both 84 (1.8 percent).

**Not visibly converging in the last half hour.** Same method, successive runs: 06:11 78.9 percent (42 judged), 06:17 77.2 (140), 06:20 81.4 (330), 06:25 81.2 (300), 06:28 81.7 (300), 06:35 76.6 (300, after the 06:30 library edits), 06:41 81.5 (300): flat within the sampling error, no upward trend. The records older than 12 hours, which are almost all rejected (the legacy class), were 14.4 percent of the sample at 06:20 (36 of 250), 13.7 at 06:25, 13.0 at 06:28, 14.7 at 06:35, 12.7 at 06:41 (38 of 300): no shrinkage beyond noise. New records are good: of the roughly 270 judged records created in the last 6 hours none has a stale closure entry (06:41 run); at the 06:35 run 3 had one (a `trinity-jack-rs` record created 06:20 on g1-m08 already failed on `walk-farm/👷️child/🟨️.mjs`, changed 06:01:48: born stale).

**Two causes, neither is a landing.**
1. Legacy closure entries (about 14 percent of owners, records 12 or more hours old). 321 of the 484 distinct shared closure entries the sample references are stale (66 percent); the eight carried by most records are tool-closure entries of declared tools (`runtime-identity`, `package-router`, `orphans-parent-script`, `testing-budget`, `runtime-sqlite`, `free-space`, `package-budget`, `runtime-parse`; `cargo-command` is next). Together they walk 102 build-library files as `analysis` rows. They were evidently derived before the R100/R105 landings: records younger than 6 hours (created after about 00:40 UTC) show no closure-entry rejection at all (age table). They go away only when a pool refreshes the owner; creation of accepted records runs at 5.6 to 8.7 per minute fleet-wide (from `createdMs`), not enough to see them shrink within half an hour.
2. Machine-local build output inside a directory listing (record-row only, 5.1 percent, plus part of "both"). 46 of 300 judged owners fail only on a `readdirNames` row of their own package directory (`.../📦️packages/🟦️typescript`, `.../🦀️rust`, `🧰️framework/🔨️modules/🧪️test`, `🌎️hub/🧩️compositions`, and more). Diagnosed: for `@semio-tech/stdio-txt`, `-stl` and `-jpg` the recorded digest equals exactly the digest of the current listing minus `dist` and `📤️dist`; `node_modules`, `.DS_Store` and `.cache` appear in the same listings. `directoryFactV1` (`👁️observation/🟨️.mjs` line 436) digests every entry name, build output and Finder files included, so a record made on a machine without `dist` is rejected on a machine that built the package and the other way round. All 48 rejections among records created in the last 6 hours (12 + 22 + 14 by age bucket) are this class; it does not converge by refreshing.

**Landings (item 4).** R113 (05:33:32 to 05:41:00): 0 of 246 judged earlier records fail on a file it changed. R114 (06:00:00 to 06:01:23) and R115 (06:01:10 to 06:03:00) overlap and are one set: 37 of 270 judged earlier records (13.7 percent, about 570 owners) fail on a file changed in that window (`👁️observation/🟨️.mjs` 06:00:01, `walk-farm/👷️child/🟨️.mjs` 06:01:48), but every one of them also fails on older edits (files changed 17:33 to 22:34 yesterday and 01:00 and 04:00 today). "Staled by this window alone" is 0 of 270 (rule-of-three upper bound 1.1 percent): in this sample a landing did not turn an acceptable record into a rejected one; it added a failing file to records that were already rejected. By failing-file timing, of 89 rejected judged records 49 are born stale (every failing file changed before the record was written, mostly the `dist` listing), 33 went stale (every failing file changed after), 7 are mixed. More library files changed at 06:34:31 (`project-nodes/🟨️.mjs`, `project-farm/🟨️.mjs`) and 06:30:04 (`host-capacity/📏️decision/🟦️.ts`): a landing not yet in landings.md; the 06:35 run, taken just after it, shows the one dip (76.6 percent, within noise).

**Projection under a landing freeze.** Closure class 620 owners (536 + 84) at 5.6 to 8.7 accepted records per minute: 1.2 to 1.8 hours from now (about 08:00 to 08:30 UTC). The 233 record-row-only owners do not reach it by refreshing while any machine has `dist` or Finder writes `.DS_Store`: without the `readdirNames` fix the plateau is about 95 percent (4386 of 4619) and the last 5 percent never closes. The script's own line (all 853 at the same rate, 1.6 to 2.5 hours) counts those 233 as if refreshing fixed them.

**Per origin (item 3).** Records created in the last 60 minutes, scaled from the sample (each origin carries an error of several owners): g1-m01 123, g1-m02 139, g1-m03 123, g1-m07 139, g1-m08 108, g1-m10 77, none from g1-m04, m05, m06, server. `pool.json` shows 0 to 4.4 owners per minute per machine, nine of ten pools in `nothing-fits` or `yielding`, g1-m04, m05 and m06 stuck on old waves (22:07, 15:42, 09:41). Eight waves were published in the last 30 minutes, so the `stored/claimed/published` counters of `pool.json` restart with every wave.

## The files that must stop being inputs (so a landing costs nothing)
1. Shared closure entries: the build-library files inside the tool-closure entries (`analysis` rows). 102 library files fail in the sample; the most referenced (stale entries / owners in the sample, mtime UTC): `walk-farm/👷️child/🟨️.mjs` 188/214 (06:01:48), `project-nodes/🟨️.mjs` 188/214 (06:34:31), `🪪️graph-state/📦️project-records/🧳️portable/🟦️.ts` 188/214 (01:00), `project-farm/👷️worker/🟨️.mjs` 188/214 (01:53), `project-farm/🏊️pool/🟨️.mjs` 182/208 (22:34), `project-farm/🟨️.mjs` 182/208 (06:34:31), `👁️observation/🟨️.mjs` 182/208 (06:00:01), `👁️observation/📥️reuse/🟨️.mjs` 182/208 (18:00), `closure-kinds/🟨️.mjs` 182/208 (19:00), `🧭️script-inputs/🟨️.mjs` and `🧭️script-inputs/💾️entries/🟨️.mjs` 182/208 (04:00), `🧠️memory/🪜️plan-stage/🟨️.mjs` (21:42), `🏡️resident/🏛️service/🧯️failure/🟨️.mjs` (19:40), `🏡️resident/🚪️client/🛰️plan/🟨️.mjs` (18:17), `🕸️graph/🧭️discovery/🟦️.ts`, `🕸️graph/🕰️host/🟦️.ts`, `🧠️memory/📋️slice-report/🟨️.mjs` (all under `L/🏗️build/`; the full list with counts is the "all failing build-library files" line of the output). The entries are the closures OF declared tools, so the R100 stop at a tool entry does not cut a tool's own group: the library tree has to be the stop (one declared tree tool whose group is a glob named input, the "next step" R100 named), so that no tool-closure entry lists a `🏗️build` module as an analysis row. Fresh entries already comply in the sample (0 of about 270 young records), so this keeps it that way when the tool groups are re-derived and retires the legacy entries.
2. Not library, the two largest single causes in the sample: `🧰️framework/🔨️modules/🏃️process/🪓️termination/🟦️.ts` (262 stale entries, 333 owners, edited yesterday 20:28) and `L/⚡️caching/🩹️patches/nx@23.2.0.patch` (199 entries, 335 owners, `file` row, 17:33): legitimate content edits inside declared tool groups, gone once the owners are refreshed.
3. Records: the `readdirNames` rows of project directories (`directoryFactV1`, `👁️observation/🟨️.mjs:436`; probes at lines 629 and 756) must not digest `dist`, `📤️dist`, `node_modules`, `.DS_Store`, `.cache` (declared outputs and ignored names), or the directory must not be observed as a listing.

## Not verified
- Reader side: one owner (`@semio-tech/trinity-jack-rs`) was pushed through the real take path (`runClosureTakeV1`, `settleClosureRowsV1`, `projectRowsHoldV1`, the census7 recipe of R105) at load 40: the gate answered in 50 ms, but settle and verify had not finished after 2 min 1 s CPU (2 min 53 s wall) and I stopped that process (my own pid). This census judged 180 rust/cargo owners in 76 s, so the readers' "taken 100 to 170 of 4774" is NOT explained by missing or stale records alone (81.5 percent pass the probes here); the per-owner settle and verify cost on the reader is the other bound. I did not profile it.
- Whether pools refresh the legacy class: pools claim stale records only; the sample shows no shrinkage in 20 minutes, but the sampled owner set differs a little per wave.
- `dist` is present on THIS machine; I did not read another machine's directories. The exact digest match shows only that the producing machine lacked `dist` and `📤️dist`.
- Origin attribution uses the note's producer host mapped through `pool.json host`; the claim-log columns (distinct granted claims per origin) are a cross-check and include refresh claims.
- No Windows run.
