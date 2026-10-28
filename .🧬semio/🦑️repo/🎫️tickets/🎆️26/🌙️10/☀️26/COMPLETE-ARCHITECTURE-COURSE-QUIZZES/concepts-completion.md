# BIM and Parametric Quiz Completion

Created bilingual BIM and parametric quiz content after authoring independent language-neutral fixtures.

- BIM: 3 classification tasks, 55 cases; data versus information, class, instance, attribute, method, inheritance, composition, exchange interfaces, coordination and isolated work.
- Parametric design: 5 classification tasks, 65 cases; discrete/continuous domains, independent inputs and derived outputs, overdetermined controls, intrinsic/infrastructure complexity, four radar axes, manual/automated/semi-automated execution.
- Each semantic slot contains exactly five disjoint alternatives; draw equals slot count; every task has a 300-second budget. All visible text is English/German, all items have explanations and distinct emoji/motion identities. Short labels satisfy the 40-character noun requirement.
- Radar profiles explicitly contrast a four-input broad coherent family, including a complex spaghetti network, with many redundant controls over a narrow family. Script size and input count alone do not determine versatility. Ratings are stated illustrative teaching assumptions.
- Correct relative schema pointers resolve to the shared Quiz schema.

## Validation

`repo tools exec --light bun -- <ticket>/verify.ts bim-concepts parametric-design` passed: 40 challenge/rotation runs, owned domain validator and AJV agreement, five fresh attempts without repetition, pool/label/icon contracts, no answer leakage, perfect and empty scoring.

`repo tools exec --light python -- -X utf8 <ticket>/verify-concepts.py` passed: 14 independent classification expectations per quiz, expected task identities and parametric axis/profile contracts, valid schema file pointers.

The parent owns central integration, official target verification and ticket closure. No generated output files were retained. Authoring and verification scripts remain as ticket input assets.

## Files Created

- `🎓️teaching/🏛️architecture/🏗️bim/❓️quiz/🔣️.json`
- `🎓️teaching/🏛️architecture/📐️parametric/❓️quiz/🔣️.json`
- `🎓️teaching/🏛️architecture/📚️course/🧫️fixtures/🏗️bim/🔣️.json`
- `🎓️teaching/🏛️architecture/📚️course/🧫️fixtures/📐️parametric/🔣️.json`
- Ticket `author-bim.py`, `author-parametric.py`, `verify-concepts.py`, `concepts-research.md`, `concepts-completion.md`.
