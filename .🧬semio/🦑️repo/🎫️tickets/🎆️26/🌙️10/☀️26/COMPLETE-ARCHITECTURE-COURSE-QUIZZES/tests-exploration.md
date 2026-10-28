# Architecture Course Test Exploration

Existing domain tests: 🎓️teaching/🏛️architecture/📚️course/🧪️tests/🧪️course/🟦️.ts validate every quiz with owned quizIssues and independent AJV draft-07 validation against 🧰️framework/🛍️products/📚️course/🧬️schema/🔣️.json, Quiz definition. AJV is already a dev dependency. Checks cover bilingual labels, icons, short forms, familiar examples, balanced pool rotation, challenge difficulty, and material loader equality.

Language-neutral fixture precedent: 🧫️fixtures/🌬️ventilation/🔣️.json. New fixture vectors should contain expected task ids/kinds, classifications, quantities, axis profiles, bilingual content requirements, and source relationships. Write contract assertions first. For mockup schemas, include a valid example plus malformed missing-language, missing-axis, invalid-reference, and invalid-kind cases; compare acceptance from owned validation and AJV. Keep third-party validators in tests only.

Browser journeys: 🧪️tests/🎯️quiz-runs/🟦️.ts already plays all quizzes perfectly, checks badges, partial credit physics/demand, balanced pools and persisted individual answers. New quizzes are automatically loaded from course material. Add targeted checks for new controls, bilingual feedback, and no solution leakage. Harness: 🎭️e2e/🚶️learner/🟦️.ts.

Targets: repo run @teaching/architecture-course:test; repo run @teaching/architecture-course:typecheck; repo run @teaching/architecture-course:test-e2e. E2E boots stacks itself; use that gate instead of direct Playwright. Narrow topology/project/grep via supported forwarded parameters. Full gate defaults to dev and rehearsal. Vitest config explicitly includes named entry files: add assertions to existing course entry or register new entry.

No nested AGENTS.md was found within teaching. No tests/builds were run. repo tools exec rg fails because rg is unregistered; use repo tools exec --light python -- -X utf8 for file reads. Prune dependency/build folders before recursive discovery.
