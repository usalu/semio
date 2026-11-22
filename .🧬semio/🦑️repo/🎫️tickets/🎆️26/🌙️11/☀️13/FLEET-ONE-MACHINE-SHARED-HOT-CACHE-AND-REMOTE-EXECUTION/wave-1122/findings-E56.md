# Findings E56 (15:42 CET)

## 1. m03 lost its dependency tree while its `tools` stage was running (suspect: E58 round 2 collector)
- 15:28 g1-m03 was `running tools` (attempt 10, adopted, 4.3 h old, process started before E58's landing). 15:40 it is `backoff dependencies attempt 11, tooling-compiler`: "No TypeScript compiler is installed where the tooling looks for it: .../🚀️bootstrap/📦️parsers/node_modules/typescript does not exist (wanted 5.9.3)". The stage's hours of work are gone and the dependency stage has to run again.
- Timing matches E58 (coordination 15:24): the new dependency collection member "live retired 12 generations", "holds per generation+pid", installer holds added in round 3 (15:27). A stage process that started on older code (every adopted stage, i.e. most of the fleet's long-running ones) takes no per-generation hold, so the collector can retire the generation it uses.
- Not proven (no access to m03's store log; the S mirror only shows the compiler message). To check: `repo cache gc` / dependency collector log on m03 around 15:24-15:30, and whether `j/<gen>` of the previous receipt is gone. At risk: every worker whose adopted stage started before 15:12 (m02 tools attempt 20, m07 tools, workspace workers that read `node_modules`). Suggested guard: the collector must not retire a generation whose binding is the target of `node_modules` of a live checkout, regardless of holds (or skip while `ready` has a running stage: `.cache/f/ready/running.json` alive).

## 2. `tools` stage native input read is slow (m03 about 30 steps/min, m04 0.5 per s)
`verifyProtectedArtifactInputFilesV1` (`process/📦️artifacts/🏗️native-build/💾️storage/📤️publication/🌐️platform/🟦️.ts` line ~140) publishes a lease progress row and yields the continuation once per hashed chunk; `native-artifact-input-read:N` was 13413 at 14:55 and 14136 at 15:18 on m03 (16 GB, memory pressure). Area process-native-build; not touched by E56. Idea: batch the progress row (one per second or per N chunks) and hash larger chunks when memory allows.

## 3. g1-m05: no zero-touch way out (details in result-E56.md)
The apply queue has been dead since 05:06 (its replica `attest.json`), so no new code can reach it; the manual replica restart of findings-E6.md stays necessary. Please schedule it with the person at that machine.

## 4. Inode and memory thresholds elsewhere
`diskRequirements` of the ready schema stays declared and absolute for MB (6144 for dependencies) beyond the 25 % volume ceiling; only the inode need got a share rule. m04 has 475 GB with 5.7 GB free: the volume ceiling does not help a full disk, only collection does.
