# Verification

## Passing Checks

- The complete TypeScript course core suite passed: **563 tests in 17 files**. This includes the shared date-boundary vectors and independent Ajv checks of schedule decisions and the normative schema. The final focused schedule run passed **11 tests**, including malformed timestamps and overlong release keys.
- The focused React and authored-course regression run passed **339 tests in 9 files**: scheduled access, home grids, quiz cards, navigation, learner pages, challenge views, hints, clocks and course documents. The final scheduled-access UI run passed **9 tests**, including pointer/keyboard blocking, ordinary deep links, the hidden argument, exact release instants, sleeping tabs, localization, saved runs and history destinations.
- The course React TypeScript check passed with exact aliases read from the current authored package exports. The ticket's `aliases.ts`, `check.ts` and `vitest.ts` retain the reproducible resolution procedure without altering unrelated package manifests.
- Actual browser journeys passed at **1440 × 900** and **375 × 812** against the real course entry, course projection, React components and session. Energy opened; every other topic stayed locked; ordinary hashes could not open locked topics; `?preview=1#parametric` opened author preview. No browser page errors occurred.
- An authored-date browser scenario passed through the real material loader, deputy, course view, session and run player. A scheduled question stayed rendered and inert under its tint, its navigation control was disabled, and both its question and topic unlocked when the tab rechecked after the release. The proctor was unavailable, so this also exercised the existing local-first deputy.
- Desktop and phone screenshots were visually inspected. The source previews remained mounted behind the tint and locked status. Temporary screenshots and other generated output are removed on completion.

## Repository Gate Limits

`repo run @semio-tech/course:test`, `repo run @semio-tech/course-react:typecheck`, `repo cargo --store=shared --configuration=prototype -- test -p semio-framework-course --lib`, and the scoped dependency synchronization attempt stopped in repository source admission before reaching their requested operation. The common failure was `ENOENT` for `C:/git/semio/.cache/b/l/fixed.dir`. `repo doctor --only=storage` reported no repairable finding. The Rust implementation and schema wiring are present, but the Rust suite and a proctor-backed full-stack gate are **not verified**.

The normal React Vitest configuration also stopped on `Bun published workspace patterns drift: 🎓️teaching/package.json`; the normal Vite configuration could not resolve `@semio-tech/framework-transient`. The ticket's diagnostic configurations use the existing registered tools and current authored package exports to test the actual application source. They do not replace the repository's standard gates.

## Concurrent Checkout Changes

Another session expanded the course material and tests while this work was being verified. Its changes were preserved. A fresh browser restart briefly failed because the newly referenced Parametrics, Grasshopper, Excel, BIM and Optimization quiz documents were not yet on disk. These were independent content edits; the release map continued to lock all five topics and their descendants. Final browser rerun evidence is recorded below.

The missing content subsequently appeared. A new server process then tested the expanded **10-quiz** course from current source, and **all browser scenarios passed again**: desktop, phone, ordinary locked deep links, explicit author preview and independently scheduled topic/question releases. In this final run, Playwright installed its clock before application startup and advanced past the release; both the topic and question unlocked through their actual release timers **without a reload or focus event**. There were no browser page errors. The earlier missing-file boot failure is therefore resolved in the final browser evidence; the repository source-admission and standard gate limits above remain.

The final TypeScript check also passed against the expanded current source. Whitespace checking passed for this ticket's source files with Windows CRLF handling enabled. Every ticket path was checked against the 256-character limit; the longest generated or authored ticket path was 167 characters. All diagnostic servers were stopped before cleanup.

## Reproduction

Run commands from the repository root, through `repo`:

```text
repo tools exec vitest -- run --config 🧰️framework/🛍️products/📚️course/🧪️tests/🎚️config/🟦️.ts
repo tools exec vitest -- run --config .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️26/COURSE-SCHEDULED-QUIZ-ACCESS/vitest.ts
repo tools exec --light bun -- .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️26/COURSE-SCHEDULED-QUIZ-ACCESS/check.ts
repo tools exec tsc -- --noEmit -p .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️26/COURSE-SCHEDULED-QUIZ-ACCESS/🗑️generated/tsconfig.json
repo tools exec vite -- --config .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️26/COURSE-SCHEDULED-QUIZ-ACCESS/vite.ts
repo tools exec bun -- .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️26/COURSE-SCHEDULED-QUIZ-ACCESS/browser-check.mjs
```

The browser diagnostic uses installed Microsoft Edge through Playwright. Browser checks exercise actual client-side behavior and the deputy; they do not prove proctor synchronization. Source-edit scripts in this ticket are historical inputs and must not be replayed over concurrent edits.
