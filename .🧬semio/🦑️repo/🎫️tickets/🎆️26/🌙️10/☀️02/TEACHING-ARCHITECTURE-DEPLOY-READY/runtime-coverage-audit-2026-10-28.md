# Runtime Coverage Audit — 2026-10-28

Read-only bounded audit of current acceptance, ticket description and real browser harness. No builds, browser probes or process changes. Existing main4072, backend16280 and frontend82275/daemon32120 remain independently owned. All source references below are relative to 🎓️teaching/🏛️architecture/📚️course unless stated otherwise.

## Coverage Against Current Acceptance

| Requirement | Actual Source Evidence | Boundary |
|---|---|---|
| All ten quizzes end to end | 🧪️tests/🎯️quiz-runs/🟦️.ts:10 loops QUIZZES at15, answers/submits/checks feedback and badges;75 starts one per-quiz pool test across easy/medium/hard/expert with persisted results at110–116. | All-quiz journeys use English and author-preview URLs (11–13,77–79). This proves content playability, not that every quiz is normally available today. |
| Both languages and first visit | 🎭️e2e/🎚️config/🟦️.ts:46 includes first-visit and both-languages. 🧪️tests/🚀️site-boot/🟦️.ts:13 runs both languages. 🗣️both-languages/🟦️.ts:80 loops locales,111 checks topic/quiz labels,138 drives task/results screens for physics/heating/demand,160 covers negotiation and persistence. Software mockup journeys cover Excel/Grasshopper in both languages (🎯️quiz-runs:124). | No complete ten-quizzes × two-languages gameplay matrix. The explicit acceptance asks all quizzes and both languages separately, so this is a narrower evidence boundary rather than a demonstrated missing requirement. |
| Multi-user | 👥️shared-presence/🟦️.ts:81,169,270,332,385,431 cover reconnect/follow, roster and answers, cursors/pages, runs/results, mobile swipe and scroll. 🏆️live-leaderboard/🟦️.ts:57 checks ordering and watching-device updates. | Real multi-context browser journeys are present. Passing source review is not passing runtime. |
| Outage/reconnect | 🔌️connection-shortage/🟦️.ts:16 tests mid-run reload;35 stops actual proctor, accepts answers while absent, verifies failed requests, restarts, and checks answers on fresh device (49–70). 📴️proctor-away:29,78 cover offline first arrival and known-handle reconciliation. 🛟️proctor-faults:57,88 cover repeated deaths/submission and contract refusal/recovery. | Explicit requirements are represented by executable specs. |
| Dev proxy and release origins | 🎭️e2e/🟦️.ts:62 defaults both topologies;268 selects development or production proctor with exact allowed site origin;294–300 launches dev-site;302–314 builds release site with baked proctor, validates artifact, serves static site on separate origin. 🎭️e2e/🎚️config/🟦️.ts:45–53 selects browser projects, and sessions/🔣️.json assigns all projects to both topologies. | Correct topology construction exists. It exercises allowed-origin functional traffic; this review does not claim a hostile-origin refusal test or public deployment success. |

## Explicit Work Outside The Suite

Current ticket acceptance-2026-10-28.md:14 requires a site that remains available to use. The harness deliberately stops every stack and releases artifacts in 🎭️e2e/🟦️.ts:316–317,428–433; even --keep retains diagnostics rather than running services. Therefore a successful full gate cannot satisfy persistent availability. After the gate, the owner must start the authorized managed dev stack and record actual readiness URLs. This is the concrete remaining runtime acceptance gap outside the suite.

The ticket description (🎫️ticket.json:5) also requires capacity/deploy-readiness gates and both staged deliverables on one checkout. These are separate gates/artifacts, not behavior missing from the browser specs. Its summary at6 records historical checkpoint667 success; acceptance-2026-10-28.md:17 explicitly says current browser results remain pending. Do not reuse that historical summary as current execution evidence.

## Actionable Evidence Qualification

All-quiz content tests intentionally call authorPreview, whose implementation only adds preview=1 to the URL (🎭️e2e/🚶️learner/🟦️.ts:405–410). Preserve the distinction when reporting all quizzes: content can be playable through preview while normal release scheduling controls availability. No requirement found in the current acceptance says all authored quizzes must bypass their release schedule.

No explicit required multi-user, outage/reconnect, or two-topology journey was found absent from the configured suite. Full current browser execution, backend/frontend compilation and persistent availability remain unproven by this source-only audit.
