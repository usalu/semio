# Course Non-Browser Suites Verification

2026-10-30. `repo://goals` was read. The open goals that this ticket already names are `🎯runningframework` (Running Framework) and `🎯runningproducts` (Running Products). Ticket `26/10/02/TEACHING-ARCHITECTURE-DEPLOY-READY` was already open; `ticket_reopen` answered `ticket is already open`, so the ticket stayed open.

## Command

`repo tools exec vitest -- run --config 🎓️teaching/🏛️architecture/📚️course/🧪️tests/🎚️config/🟦️.ts --configLoader native --reporter=verbose` on the six files below. Vitest 4.1.12. Node environment from that config. Playwright suites were not part of this run.

Exit code 0. 6 files, 125 tests, passed. Vitest duration 16.60s (start 21:23:42 local). The `repo` capacity gate held the process for about two and a half minutes before Vitest started.

## Suites

| Suite | File | Tests | Result |
| --- | --- | --- | --- |
| Assignment PDFs | `🧪️tests/📖️assignment-pdfs/🟦️.ts` | 14 | passed |
| Local stack | `🧪️tests/🧱️local-stack/🟦️.ts` | 43 | passed |
| Deploy | `🧪️tests/🧪️deploy/🟦️.ts` | 41 | passed |
| Dashboard | `🧪️tests/🎛️dashboard/🟦️.ts` | 4 | passed |
| Sessions | `🧪️tests/🎭️sessions/🟦️.ts` | 12 | passed |
| Continuous | `🧪️tests/🟢️continuous/🟦️.ts` | 11 | passed |

### Assignment PDFs

Passed coverage includes the closed print-quality plan (minimum 600 DPI in `📐️parametric/📝️assignment/icons.json`), Sharp pixel agreement of every authored SVG with the physical print density, authenticated EN and DE PDF transport, publication provenance, and the schema, D3, and arithmetic oracles.

### Local stack

Passed coverage includes server readiness and cancellation, proctor supervision and build, owned commands, storage-format handling, socket proxy when the proctor goes away, and the static origin of the release rehearsal (multi-page preview, directory confinement, module and style types).

### Deploy

Passed coverage includes Dockerfile image contract, `compose.yaml` runtime (unexposed proctor behind pinned Caddy), Caddyfile proxy and header hardening, the release workflow from the default branch, sealed publication for the baked proctor origin, the CDN artifact, and the offline proctor bundle.

### Dashboard, sessions, continuous

Dashboard contracts match Ajv and the project manifest. Session lifecycle and shard layout match the language-neutral cases. Continuous readiness resolves the declared port, refuses an invalid port, and prints the ready line only after the address answers.

## Live assignment PDFs

`repo tools exec --light node` against `http://127.0.0.1:6061`. Both responses were HTTP 200, `application/pdf`, and began with `%PDF-1.5`.

| URL | Status | Content-Type | Content-Length |
| --- | --- | --- | --- |
| `/assignments/architecture-parametric-parametrization-en.pdf` | 200 | `application/pdf` | 169673 |
| `/assignments/architecture-parametric-parametrization-de.pdf` | 200 | `application/pdf` | 169861 |

## Outcome

Every requested non-Playwright suite passed, and the live dev stack serves both language PDFs.
