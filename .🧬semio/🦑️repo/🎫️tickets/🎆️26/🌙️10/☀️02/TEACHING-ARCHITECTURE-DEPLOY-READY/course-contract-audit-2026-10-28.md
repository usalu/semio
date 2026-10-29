# Architecture Course Application Contract Audit — 2026-10-28

Read-only bounded source audit; no clips, builds, browser execution, or runtime availability assertions. The existing ticket owns lifecycle and final verification. All reads and this report used SEMIO_LANGUAGE=en repo tools exec --light node --.

## Application And Material

The actual website entry is 🎓️teaching/🏛️architecture/📚️course/🟦️.ts:16–34. It imports mountCourse from @semio-tech/course-react/app, explicitly uses tenant architecture, and passes loadArchitectureCourseMaterial. VITE_PROCTOR_URL selects the baked release proctor; otherwise the origin is empty and the dev proxy owns the gateway routes. The app export exists in the course-react package manifest.

📚️course/📚️course/🟦️.ts lazily imports the course and ten quiz documents. A lightweight JSON/path inspection confirmed all eleven dynamic imports resolve and every quiz title has en and de. Course id is architecture. Course document paths and material order agree: physics (3 tasks), ventilation (3), heating (2), cooling (1), demand (2), parametric-design (5), grasshopper-data (4), excel-workbook (3), bim-concepts (3), optimization-decisions (2). This was structural validation, not full schema or scoring validation.

## Wire And Routes

Both 🧰️framework/🛍️products/📚️course/🧬️schema/🟦️.ts:32 and 🧬️schema/📚️quiz-documents/🦀️.rs:29 define WIRE_VERSION = 10.

The actual frontend connector is 🧰️framework/🛍️products/📚️course/🎯️targets/⚛️react/🔨️modules/🛂️proctor/🟦️.ts. Lines 273–295 require five command kinds and six query kinds at version 10 before sending data. It negotiates GET /instance through ServerClient (540–549). Commands use POST /commands through submitCommand (454–459); reads use POST /queries through query (464–468). There is no quiz-specific REST session endpoint in this connector: identity and runs are course command/query envelopes. Envelopes carry scope architecture, target tenant architecture, payload/arguments bytes, and version 10 (311–342); session and device fields are null. Handle identification targets course-handle; anonymous identification and run commands target course-learner (300–302).

Required commands: course.identify-learner, course.start-run, course.open-task, course.record-answer, course.submit-run. Required queries: course.course, course.learner, course.run, course.leaderboard, course.crowd, course.handle.

Backend registration is 🎓️teaching/🛂️proctor/🔨️modules/🧩️instance/🦀️.rs:202–218. It declares all five client commands plus internal enrollment, and all six queries, using the same WIRE_VERSION. Proctor::assemble at 480–514 mounts RouteGroups::CORE, publishes the instance definition, registers CourseQuery for every QueryKind::ALL with the loaded course id as tenant, assigns learner/presence policies, and applies admission, settling and edge handling. Queries reject wrong versions and wrong tenant scopes (❓️queries/🦀️.rs:76–85). The six query wire names are explicit at 42–52.

Readiness is GET /instance, not /health or /ready: 🎓️teaching/🛂️proctor/🏗️bootstrap/🟦️.ts:422–434; CLI health probes that same route (⌨️cli/🦀️.rs:162–175). The development course constant points to the actual architecture course JSON (bootstrap:51). Vite proxies /instance, /commands, /queries, /actors and /scopes with websocket support (🏗️builder/🌐️vite/🟦️.ts:25,182). Backend unknown paths use the JSON not-found fallback (instance:513).

One concrete descriptor discrepancy exists in the connector test/helper courseInstance:282: it marks open-task authorityRequired whereas the real backend manifest marks open-task optimistic (instance:209). Agreement currently compares kind/version only (connector:290–295), so this discrepancy does not demonstrate an application route or wire compatibility failure. The helper is not the real server registration.

## Existing Browser Coverage

🎭️e2e/🎚️config/🟦️.ts:44–57 registers boot, desktop, phone, layout, presence, shortage, away, faults and pets projects. The e2e owner documents dev and rehearsal topologies, the latter using a release site and production-mode cross-origin proctor (🎭️e2e/🟦️.ts:4–8). This is coverage evidence, not a passing execution claim.

- All ten quizzes: 🧪️tests/🎯️quiz-runs/🟦️.ts loops QUIZZES for perfect runs and all course badges, and loops each quiz through easy/medium/hard/expert balanced sheets. Partial-credit cases explicitly cover physics and demand.
- Multi-user: 🧪️tests/🏆️live-leaderboard/🟦️.ts:57 tests ordering and another watching device updating without reload. 👥️shared-presence/🟦️.ts tests cooperative reconnect, roster/answers, cursors/pages, run/results, mobile swipe and card scroll.
- Shortages: 🔌️connection-shortage/🟦️.ts:16,35 tests reload answer persistence and delivery after the proctor returns. 📴️proctor-away/🟦️.ts:29,78 covers first arrival without a proctor and reconciliation of an existing handle.
- Faults/contracts: 🛟️proctor-faults/🟦️.ts:57,88 covers repeated deaths during an expert run/submission and older/newer contract refusal followed by recovery.
- Languages: 🗣️both-languages/🟦️.ts runs full screen-language switching for en/de, includes task/results journeys for physics/heating/demand, and separately tests en-US/de-CH negotiation, unsupported fr-FR explicit selection and persisted choice. It does not independently play all ten quizzes in both languages.

## Bounded Finding

No missing required quiz route, mismatched current wire version, missing material import, or missing app export was found in the inspected application contract chain. Full frontend compilation, backend compilation, and both-topology browser acceptance remain matters for their canonical execution reports. This audit does not replace them.
