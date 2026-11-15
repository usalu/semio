# Redo of the fixes reverted to HEAD at 07:58:50

## Re-applied (same edits as the original session, replayed from the transcripts with assert-guarded scripts)
B (print test re-export edge): `print/📦️packages/🟦️typescript/package.json` (7 `./testing/*` exports removed), `print/📦️packages/🟦️typescript/🧪️testing/` (7 files removed), `print/🧬️schema/💡️inferences/🔬️probes/🟦️.ts` (direct imports of `🧪️tests`).

C (repo build library, under `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/`): `⚡️caching/🚀️bootstrap/📦️dependencies/💾️storage/🧹️retirement/🟦️.ts` (`observed.target`), `🏗️build/🌡️host-capacity/📖️texts/🔣️.json` (`graph-plan-wait-cancelled` en+de), `🏗️build/🏡️resident/🕰️graph-hooks/📖️direct/🟦️.ts` (`CursorRowV1`), `…/🕰️graph-hooks/🔌️host/🟨️.d.mts` (`doubtful`, `JournalNoticeV1` as type), `…/🕰️graph-hooks/🟨️.d.mts` (policy fields, notice type), `🏗️build/🏡️resident/🧪️tests/🛑️cancel/🟦️.ts` (default free measure test), `🏗️build/🕸️graph/📥️load/🟦️.ts` (cast removed), `…/🕰️generations/🌐️remote-exchange/🟦️.ts` (typed `exchange`), `…/🕰️generations/🟨️.d.mts` (`directoryStampsFile`), `🏗️build/🧱️job-limits/🟦️.ts` (`defineCatalogV1`, typed limiter/handle).
The new `.d.mts` for `🗂️directory-stamps` and `🔎️doubt` and the pressure watch (`availableMemoryBytesV1`, 2 hits) and `🌡️pressure` test dir were untouched by the revert.

## Other reverted files
None found. Every other file edited in the session (teaching READMEs/quizzes, course presence/quantity/bounded-load/app/load-failures/outbox tests, WindowChrome `shared-border.test.ts`, course README, repo `💾️storage/🟦️.ts`) is identical to HEAD only because HEAD commits (e.g. 19ee8012ac8, 914d23addd2) contain them; their mtimes predate today. Files still differing from HEAD as expected: course README and project.json, proctor bootstrap + inputs, proctor rust project.json, course proctor-product test, WindowChrome `shared-border.ts`, kernel events test (other agent).

## Verification (cwd /Users/Shared/semio, SEMIO_LANGUAGE=en, --skip-remote-cache)
1. `repo run @teaching/architecture-course:typecheck` exit 0 (framework-rs:generate passed; no Rust failure).
2. `repo run @teaching/architecture-course:check` exit 0.
3. `repo run @teaching/architecture-course:dev` planned (waited about 10 minutes behind another agent's `@semio-tech/framework-rs:test`, which held 7 GB, no 405-owner slice), site up on 127.0.0.1:6061 (HTTP 200), proctor listening on 8791. Stopped with SIGINT; nothing listens on 6061/8791 afterwards.

## Unverified
Whole-repo pins and the `print` probe tests were not re-run after the replay.
