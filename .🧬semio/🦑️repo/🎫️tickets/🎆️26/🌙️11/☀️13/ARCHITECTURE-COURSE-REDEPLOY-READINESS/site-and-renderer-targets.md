# Site and renderer targets

## Results
- `repo run @semio-tech/course-react:test --skip-remote-cache`: exit 0, 45 files, 1100/1100 tests.
- `repo run @teaching/architecture-course:test --skip-remote-cache`: exit 0, 19 files, 847/847 tests (7 tasks incl. dependencies). The earlier exit 130 was a run cancelled for memory, not a failing task.

## Root cause (course-react)
All 45 files failed at transform time, not by assertion: `shared-border.ts` in the WindowChrome element declared `isElement` twice (a hoisted `function` near the imports and the `const` helper later), so esbuild raised "The symbol isElement has already been declared". No generator dependency was missing.

## Fix
- Removed the duplicate `function isElement` from `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🗂️WindowChrome/shared-border.ts`; the `const isElement` remains.

## Earlier fixes verified present
- `const isElement` in shared-border.ts: present (now the only declaration).
- Proctor rust project.json: `🧵️job` (24 hits) and `⏱️failure-owner` (2 hits): present.
- `languageUnavailable` and `updated` under the `app` group in both English and German renderer bundles: present.
- `📚️app/🛟️load-fallback/🟦️.tsx`: present and imported by the app.
- `$schema` in the excel and grasshopper quizzes: present.
