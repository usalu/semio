# Task Topic Marker Ownership Repair

The complete normal React owner 73624 produced the actual baseline: 1,073 passed and two failed across 43 files, with 1,075 total tests. The pet law failed at hand-topics line 81 because the marked task nodes were DIV rather than its stale LI expectation. The full run settled with exit 1 before this edit.

Current QuizPage lines 223–230 render native OL role=list, then an LI per task, then AccessGate containing the marked DIV. This retains ordered-list accessibility while keeping a locked marker inside its inert preview. Moving the marker to the outer LI would change that boundary and was rejected.

The correction changes tests and a closed neutral corpus/schema only. It retains overview control ownership and all task IDs, order, card ancestry, labels and absence of task topics. It now requires exact DIV hosts and independently requires each marker to belong to the direct native LI child of the ordered list. It also verifies no outer LI is marked. Two corpus cases exercise open tasks and the first task locked: the locked marker must remain inside inert/aria-hidden preview while the other remains outside it.

The actual component test parses the rendered page into an independent JSDOM instance and compares marker identity plus LI/OL ancestry with the same neutral output. Ajv validates the closed corpus. The focused schema/TSX parsing command exited 0 (generated/pet-task-topics-schema.log), with two cases and two rows; this is not a component GREEN. Root owns the one full unfiltered ordinary React rerun after the separate mockup fixture repair, so no duplicate focused execution was started.

No production UI/access, schedule, pet behavior or public-lock expectation changed. The Source checkpoint is generated/pet-task-topics-checkpoint.json.

- 🧰️framework/🛍️products/📚️course/🧪️tests/🎛️pet-allowances/🔸️hand-topics/🟦️.tsx — 234432d2e1fa79acf99903df13dd2c9b27595b7d18d03dfd1c2da364a6659b9b
- 🧰️framework/🛍️products/📚️course/🧫️fixtures/🐾️pet-contexts/📋️task-topics/🔣️.json — f6d67995767154d92c87ed0ce56a4f21a1c02a1783732ed8d6e5d89aeccb056a
- 🧰️framework/🛍️products/📚️course/🧫️fixtures/🐾️pet-contexts/📋️task-topics/🧬️schema/🔣️.json — c288f1d6dd94ea4f2e8795e6405c51d4e38c1409a132a8bb130638adc0ebb0f2

## Full Ordinary Owner Acceptance

Parent authoritatively recovered full React owner 89772 exit 0: all 1,075 tests across 43 files passed, including all 25 pet-allowance tests. This validates the actual task-marker/ordered-list/inert-preview law in its complete ordinary owning suite, along with the separate mockup fixture repair. No repeat is needed. The earlier 73624 full RED remains preserved.
