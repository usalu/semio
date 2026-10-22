# Course Delivery Scope Handoff Review

Current curriculum still contains five Energy quizzes and eleven task definitions:physics3,ventilation3,heating2,cooling1,demand2. Canonical index lists exactly those five quiz paths; material module statically imports them in that order. Topic grid additionally names Parametrics,Grasshopper,Excel,BIM andOptimization, each with empty quiz list. README explicitly calls those five placeholders with no run action. They must not be described as completed subjects.

English and German are present in course/topic titles and the quiz locale objects inspected; no object carrying either en/de key lacked its counterpart in the five quiz documents. This is structural bilingual coverage, not a translation-quality or scientific-correctness certification. Task counts describe definitions, not a new count of generated runs/questions.

Truthful final wording: “Architecture course implementation with five bilingual Energy quizzes covering eleven task definitions; five further topic placeholders are planned. Build, browser and deployment readiness are reported only from actual completed checks.” Do not claim all six subjects, scientific validation or live deployment from authored assets or infrastructure fixes.

Remaining actual acceptance evidence: owned course test/typecheck/check; realbrowser dev and release-rehearsal flows for explicit language choice,quiz answering/submission/results,badges/progress reload and short proctor outage/recovery; release artifact and proctor image/stack readiness on the documented supported host; finalserved-site/proctor smoke and owner-controlled deployment steps. README itself states the chain is not yet verified. Infrastructure focused results or prefix/neutral proofs alone cannot establish those course acceptance results. This review runs none of them and adds no gate.

Current raw pins:

- 🎓️teaching/🏛️architecture/📚️course/README.md — 83278/d8f67c051405fad7024266a65c2c81d4c8fede9aa440327585daa73658b0bf9e
- 🎓️teaching/🏛️architecture/📚️course/🔣️.json — 12703/84dc1ca85e69176c1d33092bd134d246a31c019619145a693c40a4c0ca1c35d6
- 🎓️teaching/🏛️architecture/📚️course/📚️course/🟦️.ts — 1675/c71332d45a0f591c1a9e7075438d5d876b96c9de62e56ffd6f8106ca29d8a2d9
- 🎓️teaching/🏛️architecture/⚡️energy/🧲️physics/❓️quiz/🔣️.json — 145812/1795e5c9491d9f46f5bd591e4e10d45faea6e13188a61522e913dc811a5bae26; tasks 3
- 🎓️teaching/🏛️architecture/⚡️energy/🌬️ventilation/❓️quiz/🔣️.json — 113656/9f256c7b64ca73c124633d320d76632424ec562713546ee6fa9abf53fe46762d; tasks 3
- 🎓️teaching/🏛️architecture/⚡️energy/🔥️heating/❓️quiz/🔣️.json — 102061/db04b89a6218b9296f2f129290c0d675562cac470f7b46e8d30749e2ffc5a319; tasks 2
- 🎓️teaching/🏛️architecture/⚡️energy/❄️cooling/❓️quiz/🔣️.json — 43566/82320ffc1602ca2f2c29bdf768c59ab490acfd4c68a9469329e3c00350ba55c4; tasks 1
- 🎓️teaching/🏛️architecture/⚡️energy/📊️demand/❓️quiz/🔣️.json — 89545/73da95631f43d2034bc2911178e50575ad66c2cc1de47b3a4d1364d8c23b1f4c; tasks 2

No Source edits, Source evaluation, Native or CLI occurred.
