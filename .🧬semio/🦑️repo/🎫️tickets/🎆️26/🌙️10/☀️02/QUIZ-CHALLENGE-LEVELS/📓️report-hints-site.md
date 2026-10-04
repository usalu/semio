# Report — site spec, driver and README for the specific hints (design §8, robust to §8.4a)

Agent: site hints. `S` = `🎓️teaching/🏛️architecture/❓️quiz`, `R` = `🧰️framework/🛍️products/❓️quiz/🎯️targets/⚛️react`,
`T` = this ticket folder. The vectors report (`📓️report-hints-vectors.md`) existed (01:51) before any e2e run of mine.
Mid-task the coordinator announced the second revision (§8.4a: verdict `reversed`, "really/tatsächlich", number words,
short labels, familiar references, profile hints relative to an item, quantity named on two dimensions, ≤ 3 hints,
wire 4) and asked for a spec that holds for both texts, one run, and no screenshot refresh after round 2.

## Changed files

Updated
- `S/🎭️e2e/🚶️learner/🟦️.ts` — `shownHints(device)` now returns `Record<item, ShownHint>` with
  `ShownHint = { kind, question }` (`kind` from `data-hint`, `question` = `.quiz-hint-question` innerText, keyed by the
  item each hint stands beside, `""` for none); new `HINT_OPENING` (`"Are you sure "` / `"Bist du sicher, dass "`),
  `GENERIC_HINT` (`/far too|wrong category|viel zu|Kategorie: \d/iu`), `quoted(label, locale)` (“…” / „…“);
  `SourceQuantity.additive`; `short?: Text` on items, categories, axes; `profile?` on categories; `axes?` on tasks.
- `S/🧪️tests/⛰️challenge-levels/🟦️.ts` — docstring; helpers `logReach`, `names`, `namesAnother`, `together`, `all`,
  `expectQuestions`, `expectPowersQuestions`, `expectUValueQuestions`, `farthestProfile`, `expectProfileQuestion`;
  the easy sorting/matching/classification tests and the German test rewritten; **new test** "in German a matching and a
  classification with spider profiles ask their easy hints in German" (13 tests now).
- `S/README.md` — easy row of the challenge table (§8 + §8.4a wording, ≤ 3 hints, short labels) and the
  `⛰️challenge-levels` row.
- `S/🔣️.json` — the catalog introduction (EN + DE) said easy "tells you how many items are in the wrong place"; now it
  says easy questions the relation the answer claims between two items (tea lights / kettle example) and asks whether a
  misplaced item fits. Only copy of that sentence in the repo.
- `R/🎨️.css` (challenge block) — `.quiz-hint-question { min-inline-size: 0; overflow-wrap: anywhere; }` so a long
  question or factor wraps instead of widening its row; block comment updated.
- `T/challenge_shots.spec.ts` — easy hint shot uses the extremes exchange, new German test (`*-2-easy-hint-de`), and a
  `-tail` shot of the smallest item (generated only).
- `T/📸️shots/` — replaced `desktop-2-easy-hint.png`, `phone-2-easy-hint.png`; added `desktop-2-easy-hint-de.png`,
  `phone-2-easy-hint-de.png` (taken against round-1 texts, before §8.4a; a later agent re-shoots after round 2).

Created: `T/site_hint_probe.ts` (which references/verdicts each spec arrangement produces over 600 sheets),
`T/site_question_probe.ts` (round-1 rendering vs the then-strict spec expectations, 300 seeds × 2 languages: 0
failures; stale for §8.4a texts), this report. Removed: my e2e run dirs and `T/🗑️generated/{hints-site,integration}`.

## What the spec asserts now (holds for §8 and §8.4a)

Per hinted item: hint kind (`compare` / `profile`), exactly the expected items carry hints, ≤ 3 per task, question
opens with `HINT_OPENING`, names its own item by short label or label (quoted), names another shown item (compare),
nothing in the task matches `GENERIC_HINT`. Arrangements:
- physics `powers`: (a) extremes exchanged → both extremes questioned (verdict reversed under §8.4a, so no wording);
  (b) the two largest exchanged → when `v_max / v_{n−2}` exceeds the reach (always with the Sun drawn; ~93 % of sheets)
  both are questioned and the largest's question contains "together" / "zusammen" (all anchors lie below, so the claim
  points the true way and understates: verdict `under` in both revisions, whatever reference familiarity picks);
  otherwise no hint at all.
- heating `u-values`: extremes' cards exchanged → both questioned against another item; true cards back → none.
- demand `standard-profiles`: first item moved to the profile with the largest gap/reach (precondition > 1 asserted) →
  one profile question naming the axis (short or label, case-insensitive) and either that profile or another standard.
- Same three checks in German (physics in the existing German test, heating + demand in the new one).

## Gates (exact commands, in `S/📦️packages/🟦️typescript`, `SEMIO_TEST_BUDGET_MS=900000 NX_PLUGIN_NO_TIMEOUTS=true`)

| Gate | Result |
|---|---|
| `bun nx run @teaching/architecture-quiz:typecheck --skip-nx-cache` (strict spec, round 1) | success |
| `bun nx run @teaching/architecture-quiz:test --skip-nx-cache` (after the catalog text) | 5 files, 235/235 |
| `bun ./📜️script.ts test-e2e dev --project=desktop --no-deps challenge-levels` (strict round-1 spec: exact wording, reference by tie rule, factor bounds) | 12 passed, 1 failed (`hard hides the keys`: home-card click timeout, not hints); rerun `-g "hard hides the keys"` 1/1 |
| `bun ./📜️script.ts test-e2e dev` (whole dev gate, strict spec) | 34 passed, 2 failed, 22 did not run (18.3 min). `⛰️challenge-levels` **13/13**, layered-home 5/5, first-visit 4/4, both-languages 3/3, leaderboard 1/1, boot 1/1, layout 4/4, phone 2/2; red: `🎯️quiz-runs` perfect run (demand card click timeout) and demand drag (bin not lit) — the load failures `📓️report-site-final.md` lists; presence/shortage/away/pets did not run because desktop failed |
| first run of the version-neutral spec | proctor did not compile (round 2 in flight: `CompareHint` has no `under`, `short` missing) |
| `bun ./📜️script.ts test-e2e dev --project=desktop --no-deps challenge-levels` (version-neutral spec, after round-2 core/i18n had landed: `verdictOf`, "really", "higher" present) | **13/13** (5.0 min) |
| site `tsc` after the revision | my files clean; errors only in round-2 in-flight `R/🔨️modules/🧩️task/🟦️.tsx` and core `⛰️challenge/🟦️.ts` at that moment |

Not run: rehearsal topology; the version-neutral spec against round-1 texts (it only weakens the strict spec that
passed there); React suite (one CSS rule added).

## Screenshots (round 1, looked at all four)

Desktop: the question fits on one line under the label, left of the key column, no overlap with keys or move buttons.
Phone: wraps to 2–4 lines inside the row above the key and buttons; the long factor ("2.200.000.000") stays inside;
page width = window in every shot. The browser hyphenates inside quoted labels ("Ge-samte"), as the item labels do —
left as is.

## Deviations and decisions

1. Brief asked for "together only add up to" on the Sun case: with the Sun on the smallest key (the old arrangement)
   the claim points the wrong way, so §8 says `sumOver` ("it takes … to add up to") and §8.4a says `reversed`. The
   understated additive wording needs a claim the true way round: exchanging the two largest gives it.
2. After §8.4a the spec no longer pins numbers, verdict wording or the reference item (familiarity changes it); the core
   vectors and React tests pin those. The strict round-1 version passed 13/13 in the full dev gate before.
3. The catalog introduction was wrong under §8; I fixed it in both languages rather than leave learner text stale.
4. The `reach` used to decide whether the two largest are questioned is computed in the spec (`sqrt(spread)`, cap 1000,
   slack 1e-9) from the catalog values — the driver never imports the framework.

## Notes for the next agent

- `shownHints(device)` → `Readonly<Record<string, { kind: string; question: string }>>`; `HINT_OPENING`, `GENERIC_HINT`,
  `quoted(label, locale)` exported from the driver.
- Screenshots after round 2: `PLAYWRIGHT_BASE_URL=http://127.0.0.1:6063 bunx playwright test --config T/challenge_shots.config.ts -g "easy hint"`
  from `S/📦️packages/🟦️typescript` with the `architektur-und-technologie-quizze-beside` launch row running; output under
  `T/🗑️generated/integration/shots/`, copy `*-2-easy-hint*.png` (not `-tail`) into `T/📸️shots/`.
- When short labels land, the catalog content test requires `short` for labels over 40 characters; the spec already
  accepts short or label everywhere.
