# Quiz Proof of Concept Deploy Bundle — summary

Goal: the first page and server of architektur-und-technologie as a proof of concept that the owner can deploy.

## What existed

The page (`🎓️teaching/🏛️architecture/❓️quiz`, the quizzes site) and the server (`🎓️teaching/🛂️proctor`) were built and gated, but
deploying them took four outward steps (DNS zone, GitHub Pages, GHCR, Docker host) because the site is a CDN artifact and the
proctor a second origin behind its own host name.

## What this ticket added

`poc` and `poc-check` verbs of `@teaching/architecture-quiz` (code in `🚀️deploy/🟦️.ts`, region `🛫️Poc`; stack files in
`🚀️deploy/🛫️poc/`):

- `bun nx run @teaching/architecture-quiz:poc -- --host <IPv4 address or host name>` builds the site for the one origin of
  that host (an IPv4 address becomes `<a-b-c-d>.sslip.io`, which resolves without any DNS record), verifies the artifact
  (sealed Content-Security-Policy, hashed assets, budget; no `CNAME`), and writes one directory: `site/`, `compose.yaml`,
  `Caddyfile`, `.env`, `README.txt` and `proctor-image.tar` (`--no-image` skips it).
- On the host: `docker load --input proctor-image.tar` then `docker compose up --detach --wait`. Caddy obtains the
  certificate, serves the page and forwards `/instance /commands /queries /actors* /scopes*` to the proctor on the same origin.
  The hardening, limits, data volume and backup/restore are those of the split deployment.
- `poc-check` stages the bundle for `localhost`, runs it as a host would, and probes the one origin.

Also: `Door.origin` in the verbs (checks speak as a chosen origin), `siteArtifactProblems(…, { cname })`, drift check now covers the
poc stack files, taxonomy kind `deploy-poc` (`🛫️poc` under `🚀️deploy`), launch rows (both `launch.json` and the seed),
root `package.json` scripts, README section "Proof of concept on one host".

## Evidence (run 2026-10-02)

- `bun ./📜️script.ts poc-check` (direct; nx timed out on this loaded machine): `proof of concept passed` — compose config, `caddy fmt`/`validate` without warning,
  `up --wait` healthy in 8.9 s, `GET /` 200 sealed document, immutable hashed asset 200 and a missing asset 404, deep link 200 with the document,
  `/instance` 200 with API headers, HTTP → 308 HTTPS, catalog queried and a learner registered and known, a foreign origin gets no grant,
  presence WebSocket (two learners, state, departure) and a foreign-origin socket refused.
- `poc` itself ran for `127.0.0.1` (→ `127-0-0-1.sslip.io`, 33 MiB image tar) and `localhost`; the localhost bundle was started with compose and driven in
  Chromium (Playwright, local certificate error ignored, `browse_poc.mjs`): title "How the quizzes work · Architecture and Technology Quizzes", header
  "Connection: All answers saved", the only gateway request `200 POST /queries`, no console error, page error or failed request.
- `bun ./📜️script.ts test deploy`: 11/11 tests pass, including the new one for the poc.
- `verify taxonomy report --scope 🎓️teaching/🏛️architecture/❓️quiz`: clean.

## Not proven / caveats

- A public certificate (Let's Encrypt via sslip.io or a real DNS name) and reaching a real host cannot be rehearsed here.
- The proctor image on this machine was built from a dirty tree at 09:23 local time; other sessions have changed the proctor since (period
  leaderboards). Rebuild with `docker-image-build` before the bundle is made.
- `@teaching/architecture-quiz:typecheck` currently fails in `🧰️framework/🛍️products/❓️quiz/🎯️targets/⚛️react/🔨️modules/🛂️proctor/🟦️.ts(409)`
  (a `leaderboard` query type), part of another session's in-progress work; none of the files of this ticket has a type error.
- Nx hung on plugin loading while many other sessions were running (`NX_ISOLATE_PLUGINS=false` did not help); the verbs were run through
  `bun ./📜️script.ts` in the package directory.
