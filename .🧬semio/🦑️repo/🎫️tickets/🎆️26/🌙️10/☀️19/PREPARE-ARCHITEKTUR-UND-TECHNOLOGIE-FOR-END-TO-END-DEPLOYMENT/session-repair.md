# Course Session and Journey Fixture Repair

The current course product starts at the course overview and offers explicit sign-in. Quizzes sit within the Quizzes overview card. The current wire contract uses `course.*` commands and actor kinds, and the course document schema is `semio.course/v1`. The shared journey double and its assertions still used the previous wire names and initial navigation. This made genuine command retry and contract checks fail before they could test the intended behavior.

The shared fixture now speaks the current protocol, including outage interception and submission accounting. Session cancellation assertions check three unique answer command IDs and the actual accepted answers, allowing the legitimate idempotent transport retry. App fixtures follow the current walkthrough, sign-in, nested overview and language labels. Each test resets its URL so the previous test's hash cannot change its starting screen. Presence checks follow the current footer roster, course roster surface, preferences cursor room and home watch scopes. Peer frames now consistently declare the peer's actual place in both roster and cursor state; current view filtering intentionally excludes cursor frames for a different place. The submission fixture now checks the current accessible confirmation for incomplete tasks instead of expecting submission to be disabled.

Files owned by this repair, under `🧰️framework/🛍️products/📚️course/🧪️tests/`:

- `🧰️learner-journey-kit/🟦️.tsx`
- `🏔️challenge-session/🟦️.tsx`
- `🧷️run-truth/🟦️.tsx`
- `🎬️session-playthrough/🟦️.tsx`
- `🧩️deputy-and-contract/🟦️.tsx`
- `🚶️journey-and-tabs/🟦️.tsx`

No product behavior was changed for these fixture repairs. Existing tests exercise the actual React app and CourseSession with the framework's protocol encoder/decoder and core deciders, using Testing Library as the independent DOM interaction/assertion implementation.

## Observed Validation

The first selected diagnostic run completed with 45 passing and 5 failing tests across five suites. Challenge session (10), run truth (8), and session playthrough (3) were green. Remaining failures were in the deputy and journey fixtures: obsolete German Explore text, a lazy walkthrough title wait, obsolete overview navigation text, and hash leakage between tests.

After correcting those fixtures, the next five-suite run completed with 48 passing and 2 failing tests (`🗑️generated/session-current.log`). Deputy and contract (15) is now green alongside the other three files. Journey is 12 of 14; actual remaining failures are a nested Settings region assertion and an expected home cursor payload that omitted the current `home` and `place` fields. Both assertions are corrected and a journey-only rerun is in progress (`🗑️generated/journey-current.log`).

The following journey-only runs exposed later stale assertions rather than product failures. The first reached the actual source policy permitting incomplete Medium submission and peer filtering by matching roster/cursor place. After correcting those fixtures, run 64721 completed 13 of 14 tests: the complete presence journey is green; the learner journey passed answer entry, outage, retries, submission and results and reached leaderboard filters. Its sole remaining failure was SVG formatting whitespace in filter-button `textContent`. The fixture now trims the label before comparing the unchanged expected filter names and selection states.

Run 72617 passed the leaderboard filters and reached reload, again 13 of 14 tests. The preserved `#quizzes` URL correctly reopens Quizzes, so the stale root learner-region assertion could not find the inaccessible background card. The test now explicitly asserts the restored Quizzes heading, navigates to Course overview, then inspects the retained learner and badges. Run 33876 validates this final fixture correction. No product behavior issue has been identified in these iterations.

## Final Focused Result and Full Gate Handoff

Run 33876 completed successfully: journey and tabs **14 of 14 passed**, process exit 0, 91.90 seconds. This verifies the complete learner journey through submitted results, leaderboard filters and reload, alongside the complete presence journey. Combined with the unchanged four green suites from the 48-of-50 run, all five owned suites have actual green results: challenge session 10, run truth 8, session playthrough 3, deputy and contract 15, journey and tabs 14 (50 unique tests).

The full configured 38-file React gate has been started with the same current native config loader and four workers. Its output is `🗑️generated/ui-full-react-final.log`. Root owns polling, interpreting the full result and the final readiness report; the full gate is pending at this handoff.

Command from `C:/git/semio`, with `SEMIO_TICKET_DIR` set to the current ticket:

```powershell
node --experimental-transform-types ./node_modules/vitest/vitest.mjs run --configLoader native --config '🧰️framework/🛍️products/📚️course/🎯️targets/⚛️react/🧪️tests/🎚️config/🟦️.ts' --maxWorkers=4
```

The diagnostic uses installed Node/Vitest, `--experimental-transform-types`, `--configLoader native`, the actual current React config and the genuine teaching workspace ownership resolver. It does not allocate a compiler store or establish canonical native release readiness.
