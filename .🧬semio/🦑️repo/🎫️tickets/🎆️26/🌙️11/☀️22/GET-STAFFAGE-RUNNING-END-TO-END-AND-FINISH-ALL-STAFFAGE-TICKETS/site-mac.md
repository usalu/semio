IN WORK

# 🍎️ Mac Lane site-mac: Takeover of s6 (suites) and s7 (overview) from g1-m04

State written 2026-11-25 20:50 (first full pass done, final pass pending). `R` = `/Users/ueli/Documents/semio`, `S` = `R/🎓️teaching/🏛️architecture/🧍️staffage`, `U` = this ticket. Scripts: `U/🧰️mac/`. Logs, screenshots and JSON: `U/🗑️generated/mac/`. Browser: the installed Google Chrome through Playwright's `chrome` channel (this Mac has no Playwright Chromium, none was downloaded). Only runs of this session are counted.

## Where it stands (so far)

- React typecheck: 0 errors. Site typecheck: 0 errors in staffage, 4 errors in `R/🧰️framework/🔨️modules/🎠️kernel/🛂️descriptor-admission/🟦️.ts` (another lane's uncommitted edit, pulled in through the site project). The two engine errors of line 254 are gone (the engine owner fixed them).
- React suite (all case files but `catalog-loading`): 364 passed, 4 failed in the first run; the four are explained and three repaired (section 2).
- Site suite: 17 passed, 1 timed out (`model-split` builds the whole site twice over; its own limit is 900 s, a release build takes 24 minutes on this loaded Mac).
- Release build: exit 0, 24 min 24 s, 375 asset modules. Smoke on it, four contexts: 62 of 62 checks, no console, page or network issue.
- s7 measured in Chrome on that build: idle English desktop overview, renderer 987 MB median, 1275 MB maximum from 60 s on (Linux before the lane: 2 to 3 GB), live previews bounded at 14.
- Tag titles: schema, 379 documents, React target, three tests (section 5).
