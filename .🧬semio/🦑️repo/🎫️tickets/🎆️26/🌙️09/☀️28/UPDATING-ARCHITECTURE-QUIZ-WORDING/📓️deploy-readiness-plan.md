# 🧭 Deploy Readiness — Coordination Plan (2026-10-01/02)

Owner request: "Make sure that everything works end to end. Add a dev architekturundtechnologie quizze which starts
both backend and frontend. Have everything ready to be deployed."

## Audits (read-only, done)

| Report | Result |
|---|---|
| `📓️audit-final-requirements.md` | R1, R2, R4–R11, R14, R15 met; R3 (no `clip` leaf yet), R12 (nothing published — owner step), R13 (no presence before identity) partly; 5 defects |
| `📓️audit-deploy-security.md` | 3 blockers (learner ids readable through idempotency receipts; unvalidated client ids; no flood protection), 13 should, 12 notes |
| `📓️audit-final-i18n-a11y.md` | 7 defects, 25 improvements, 14 notes; key sets of both languages identical (224) |

## Work packages (running in parallel)

| Agent | Owns | Report |
|---|---|---|
| dev-e2e | one dev command for proctor + site (`🛠️dev🏛️architektur-und-technologie❓️quizze`), Playwright gate `test-e2e` in dev and release-rehearsal topology | `📓️dev-e2e-report.md` |
| deploy | Docker image/compose/Caddy/workflow hardening, readiness gate, runbook, ops findings S3–S9, S12, S13, N4–N11 | `📓️deploy-readiness-report.md` |
| edge | framework server + proctor instance/config: B1, B2 (hook, passivation, body cap), B3 (limits sized for a class of 300 behind one address), S8 routes, S11, N1, snapshots, capacity proof | `📓️edge-hardening-report.md` |
| domain | quiz core (both twins) + proctor actors/projections/queries/cli: id and handle rules, bounded growth, leaderboard `{ rows ≤ 100, learners, own? }`, `proctor health/backup/erase`, real-catalog badge test | `📓️domain-hardening-report.md` |
| ui | React client + site shell: every defect and improvement of the two UI audits, no default language, launch wording, legal links, new leaderboard contract, 429 backoff | `📓️ui-hardening-report.md` |

Note files between agents: `🗒️edge-needs-from-domain.md`, `🗒️domain-needs-from-edge.md`, `🗒️domain-notes-for-ui.md`,
`🗒️ui-notes-for-e2e.md`, `🗒️ui-notes-for-deploy.md`.

## Coordinator decisions

- `@teaching/architecture-quiz:dev` is the whole stack; the site alone is `dev site`; the two-terminal compound row goes.
- Handle recall without a password stays (owner's design); the UI says so plainly, the server hardens the handle rules.
- Rate limits must never throttle a lecture hall behind one NAT address; they exist to stop a single script.
- Leaderboard contract: top rows (≤ 100) + total count + the caller's own row.
- Nothing is published outward (no GHCR push, no workflow run, no DNS) without the owner.

## After the agents report

1. Integration: all nx tests, parity, taxonomy, e2e gate (both topologies, 3× green), readiness gate on the final tree.
2. Coordinator walk in the browser through the new dev row.
3. Closing summary section, ticket close.
