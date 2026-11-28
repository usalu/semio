# Result H92: `repo test --affected`, ticket hygiene by tooling

Status: STAGED, not landed (16:50 build-system freeze). Stage: `/private/tmp/claude-501/-Users-ueli--claude-spares-spare-fuDDx3/098f233d-9be4-4a1a-b4a8-48924f807f61/scratchpad/H92/stage` (`repo land check` it, land in the first slot after the seal: `repo land --wave "Wave 1127" --lane H92 --title "Affected tests, ticket hygiene" <stage>`).

## 1. `repo test-file --affected <changed files>` (= `repo test --affected`, MCP `test_file` param `affected`)
- New module `B/🔬️test-file/🎯️affected` (policy section `affected` in `🔣️.json`, schema def `Affected`, texts en/de, tests). Reverse import graph of the scan roots (index cached per checkout in the machine cache, only changed files re-read), bounded depth 4.
- Law: a test file selects itself; a leaf module selects tests importing it plus the declared `🧪️tests/🟦️.ts` beside it; a module with > 40 importers in reach selects the declared suite(s) owning it (deepest tree in the suite table); no test in reach -> suite fallback; no table covers it -> nothing selected (said in both languages); > 64 tests -> refused (`affected.too-wide`), never the whole workspace.
- Front door: policy `aliases` (`test --affected` -> `test-file`), catalogue flag `--affected`, help snapshot regenerated, MCP golden vector `test-file-affected`.
- `repo land`: gate "tests" adds the affected tests of all changed paths to the declared ones (max raised 12 -> 24, truncation announced; a refused selection falls back to declared tests with a notice).
- Tests: Ajv + in-repo schema validation with mutations, leaf/wide/test/unmapped/fallback/too-wide, import graph equals TypeScript `preProcessFile` reverse closure (third-party oracle), index follows edits, real-checkout dry run bilingual, door alias.

## 2. Ticket hygiene
- `🧹️ticket-generated`: H28's sweep now removes only CLOSED tickets (manifest `🎫️ticket.json` status closed), idle 24 h (2 h under disk pressure); open or manifest-less tickets are reported `open`, never touched (this reverses H28's "open ticket is not spared"). Deletion capability (`issueTicketGeneratedDeletionV1`, issuers `ticket-close` and `closed-ticket-sweep`, unforgeable via WeakSet, per ticket); `requireTicketGeneratedDeletionV1` refuses (en/de) any other deletion inside a `🗑️generated` folder. `repo clean` (`cleanRemovePath`) now refuses ticket generated output (corpus updated).
- Per-ticket build lease `B/🔒️leases/🎫️ticket-build`: ticket from `SEMIO_TICKET` or cwd; cargo / run-many / affected take one lease per ticket, FIFO queue, bilingual queue/acquired/takeover/timeout/cancel messages, nested builds pass via inherited token, dead or >12 h holders taken over, light flags and builds outside a ticket unlimited. Hooked in `⌨️command/⚙️runtime` around the owned build invocation.

## Unverified / limits
- Rust/Go ticket-close purge unchanged (already closed-only); a TS caller issuing `ticket-close` does not exist yet, the Rust/Go deleters hold no TS capability.
- Lease not exercised across real separate processes (tests simulate with separate environments); lease wait uses a 1 h cap.
- MCP tool description text for `files` not extended with the changed-file meaning.
- `repo land check` (3rd run, 18:50): debug-lines, dangling-imports, typecheck PASSED; tests: all new/changed files green (affected 10, ticket-build 9, ticket-generated, removal 14) except `🔬️test-file/🧪️tests/🟦️.ts`, which fails in the overlay with `A directory of the native tools is not a plain directory: <overlay>/.cache/b/t/m` (overlay storage, not run on the live tree to compare; the command-line suite red is pre-existing). Re-run after the seal before landing. Test-sandbox images from my manual overlay runs were detached with hdiutil.
