# Deploy Retarget — report

Scope: sections 1 and 3 of `📓️split-deploy-design.md` — the new hosts, the operator's own certificate, the bundle verb
for the proctor stack, the removal of the one-origin proof of concept. Run on 2026-10-02 against Docker Desktop 29.8.1
(Linux engine, amd64, compose v5.5.1), bun 1.4.2.

## What changed

| File | Change |
|---|---|
| `🎓️teaching/🏛️architecture/❓️quiz/🚀️deploy/🔣️.json` | `site.host = quizze.architektur-und-technologie.de`, `proctor.host = semio.iek.uni-hannover.de` |
| `…/🚀️deploy/Dockerfile` | header comment and `PROCTOR_ALLOWED_ORIGINS` name the new hosts |
| `…/🚀️deploy/compose.yaml` | new hosts; caddy mounts `./certificates:/certificates:ro`; header documents the supplied certificate, the loaded image and the recreate command |
| `…/🚀️deploy/Caddyfile` | new host; `tls { load /certificates }`; header documents both certificate sources; **an `@oversized` matcher refuses a declared body over the limit before anything is forwarded** (see "Found on the way") |
| `…/🚀️deploy/🟦️.ts` | `hostZone`, drift over the zones of both hosts, the certificates mount/`load` and the one body-limit default; `QUIZ_STACK_CERTIFICATES`, `QUIZ_BUNDLE_DIRECTORY`, `QUIZ_BUNDLE_IMAGE`; `STAGED_DIRECTORIES` = `pages`, `proctor`; `expectCheckoutStack` (shared by the image check and the bundle); `servedCertificate`, `mintLocalCertificate` and the supplied-certificate proof at the end of `checkQuizStack`; region `🎁️Bundle` (`stackBundleEnvironment`, `stackBundleReadme`, `stageQuizStackBundle`) replaces region `🛫️Poc`; removed `pocHost`, `pocOrigin`, `pocEnvironment`, `pocReadme`, `stagePocBundle`, `checkPocBundle`, `QUIZ_POC_*`, the `cname` option of `siteArtifactProblems`, `Door.origin` and `doorOrigin` |
| `…/🚀️deploy/🛫️poc/` (`compose.yaml`, `Caddyfile`) | removed |
| `…/📦️packages/🟦️typescript/📜️script.ts` | verbs `poc`, `poc-check` removed; `docker-stack-bundle [--tag <tag>] [--out <directory>]` registered |
| `…/📦️packages/🟦️typescript/📋️project.json` | targets `poc`, `poc-check` removed; target `docker-stack-bundle` (output `dist/proctor`) |
| `…/📦️packages/🟦️typescript/package.json` | description names the new site host |
| `…/🧪️tests/🧪️deploy/🟦️.ts` | hosts no longer assumed to be nested; seeded drifts for both zones (the old site name, a sub-host of the site, another host of the proctor zone, a sub-host of the proctor host, the GitHub challenge exception), for the certificates mount, the `load` path and the body-limit default; compose and Caddyfile expectations for the certificates; the poc test replaced by the bundle test (`.env` lists exactly the defaults `compose.yaml` substitutes, the README names both hosts once each from the constants and every step in order, lines ≤ 120, staged directories are not part of the site artifact) |
| `🎓️teaching/🏛️architecture/❓️quiz/README.md` | intro and table rows; "Proof of concept on one host" → "Proof of concept by hand"; Deploy: artifact table, DNS and firewall, the Docker host, "A certificate of your own", verify table (+ certificate row), update without a registry, backup, uptime, "When the certificate cannot be issued", Never |
| `🎓️teaching/README.md`, `🎓️teaching/🏛️architecture/README.md`, `🎓️teaching/🛂️proctor/README.md` | host names; "automatic TLS" → obtained or supplied |
| `.github/workflows/architecture-quiz.yml` | header comment names the new site host |
| `.vscode/launch.json`, `.vscode/🧩️launch.seed.jsonc` | rows `🛫️poc🎓️teaching🏛️architecture❓️quiz`, `⚖️gate🎓️teaching🏛️architecture❓️quiz🛫️poc` and input `quizPocHost` removed; row `📦️bundle🎓️teaching🏛️architecture❓️quiz🐳️docker-stack` (group `4_build`, order 11.4) added |
| `package.json` (root) | scripts `poc:teaching:architecture-quiz`, `check:teaching:architecture-quiz:poc` removed; `bundle:teaching:architecture-quiz:docker-stack` added |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json` | kind `deploy-poc` removed (it was the whole registration of `🛫️poc`) |

`.claude/launch.json` had no poc or host row; nothing changed there.

## Caddy and a supplied certificate — what the pinned image really does

Pinned image: `caddy:2.11.4@sha256:0c994536…9b52`. The syntax of the task works as given:

```
{$PROCTOR_HOST:semio.iek.uni-hannover.de} {
	tls {
		load /certificates
	}
```

with `- ./certificates:/certificates:ro` on the caddy service. Findings (`caddy_certificate_probe.sh`, run against the
final `Caddyfile`; fingerprints by `certificate_fingerprint.ts`):

| Case | Result |
|---|---|
| `caddy validate` without the directory | fails: `loading certificates: unable to open root directory /certificates` — so `checkQuizStack` runs `mkdir /certificates` inside its validate command; with the directory: `Valid configuration`, no warning |
| `localhost`, empty directory | Caddy obtains its own (`certificate obtained successfully … issuer local`) |
| `localhost`, a matching bundle (root, `0600`), container restarted | the served fingerprint equals the supplied one; log: `skipping automatic certificate management because one or more matching certificates are already loaded` |
| the same file owned by `1000:1000` with `0600` | Caddy exits 1: `walking certificates directory /certificates: openat own.pem: permission denied` (it runs as root with `cap_drop: ALL`, so it reads only as owner or as everyone) |
| the same file owned by `1000:1000` with `0644` | read and served |
| `semio.iek.uni-hannover.de`, no bundle for it (network without egress) | `obtaining certificate` and an ACME account request to `acme-v02.api.letsencrypt.org` |
| `semio.iek.uni-hannover.de`, a bundle for that name | no log line about ACME, the `skipping automatic certificate management` line for that domain, and the served fingerprint (SNI = that name) equals the supplied one |

Only `*.pem` files are loaded, at start; `docker compose up --detach --force-recreate caddy` recreates caddy alone (the
proctor kept its container: `Up 57 seconds` after it, compose v5.5.1), so the command of the task is the documented one.
Docker creates `./certificates` when it is missing, so a stack copied out of the image needs no `mkdir`.

**How the proof in `docker-stack-check` tells the two certificates apart.** After every other step it reads the SHA-256
fingerprint the listener presents (`node:tls` `connect` with `servername: "localhost"`, `getPeerCertificate().fingerprint256`)
— that is the certificate of the stack's own internal CA. It then mints a `localhost` certificate in a throw-away
container of the same pinned image (own `/data`, so another root and intermediate; waits for `localhost.crt`, `.key` and
`.json`, prints chain and key), takes `new X509Certificate(bundle).fingerprint256` of it, fails if both are equal, writes
the bundle to `certificates/localhost.pem`, recreates the caddy service and reads the listener again: it must present
exactly the supplied fingerprint, Caddy's log must carry the `skipping automatic certificate management` line for
`localhost`, and `GET /instance` must still answer 200.

## Commands and outcomes

All verbs were run as `bun ./📜️script.ts <verb>` in `🎓️teaching/🏛️architecture/❓️quiz/📦️packages/🟦️typescript`.

| Command | Outcome |
|---|---|
| `test deploy` | 11/11 pass (final state; includes `deploymentDrift(repoRoot)` = no drift) |
| `docker-image-build` (twice: before and after the `@oversized` matcher) | exit 0; final image `sha256:6001f14ac7ae…744e7e`, linux/amd64, 31 730 258 bytes, built from `48d881aa…-dirty` |
| `docker-image-check` | passed: labels, no shell, `/srv/quiz/deploy carries compose.yaml and Caddyfile of this checkout`, preflights granted to `https://quizze.architektur-und-technologie.de`, none for a foreign origin, body limit 16385 → 413, presence, drain, WAL folded |
| `docker-stack-check` | first image: run 1 **failed** at the body limit (`a body of 32769 bytes answered 502`), run 2 passed. Final image: 3 of 3 passed, each ending with `supplied certificate: Caddy served its own D2:FD:AB:…:10:31; with certificates/localhost.pem of another CA and the caddy service recreated it serves CA:EE:79:…:9E:2E, the supplied one, manages none for localhost, and GET /instance answers 200` / `stack passed` |
| `docker-stack-bundle` | `staged compose.yaml, Caddyfile, certificates/, .env, README.txt and proctor-image.tar (32 MiB) in …\dist\proctor`; both stack files byte-identical to the checkout |
| bundle proof in `🗑️generated/bundle-proof` (a copy of `dist/proctor`) | `docker image rm` of the tag, then `docker load --input proctor-image.tar` → `Loaded image: ghcr.io/usalu/architecture-quiz-proctor:latest`, same id `sha256:6001f14a…`; `PROCTOR_HOST=localhost QUIZ_HTTP_PORT=18082 QUIZ_HTTPS_PORT=18445 docker compose --project-name architecture-quiz-bundle-proof up --detach --wait` → both healthy (nothing pulled: `.env` pins `PROCTOR_TAG=latest`); `curl --insecure https://localhost:18445/instance` → 200 `{"id":"teaching-proctor",…`; preflight as `https://quizze.architektur-und-technologie.de` → 204 with `Access-Control-Allow-Origin: https://quizze.architektur-und-technologie.de`, max-age 7200; preflight as `https://foreign.example` → 204 without the grant; HTTP → `308 https://localhost/instance`; 300 bodies one byte over the limit → 300 × 413; a minted bundle in `certificates/own.pem` + `up --detach --force-recreate caddy` → served fingerprint `1E:BE:5D:…:29:C8` = supplied (before `AC:E4:FC:…:85:BD`), proctor untouched; `down --volumes` → no container or volume of the project left |
| README verify rows with a Linux curl (OpenSSL, the pinned `rust` builder image, in the caddy container's network) | `/instance` with `--fail` → JSON; redirect `308 https://localhost/instance`; preflight `204` + grant; `grep -E 'issuer|expire'` prints `expire date:` and `issuer:` |
| `tsc --noEmit -p tsconfig.deploy-retarget.json` (ticket folder: the package's config narrowed to `📜️script.ts`, `🚀️deploy/**`, `🧪️tests/🧪️deploy/**` and what they import) | 0 errors |
| `bun ./📜️script.ts verify taxonomy report --scope "🎓️teaching/🏛️architecture/❓️quiz"` (repo root) | first run: 1 error, `reference-preimage-unreadable …/🧪️tests/📴️proctor-away/🟦️.ts: Reference preimage changed since inventory` (that file was being edited by the other session during the run); two later runs: `clean=true errors=0 warnings=0` |

Not run, as instructed: `publish`, `build`, `typecheck` (the verb), `test-e2e`, `deploy-check`.

## Found on the way

1. **A 502 instead of a 413 for an over-limit body (fixed in the `Caddyfile`).** Caddy and the proctor hold one limit.
   The proctor refuses a request whose declared length is over it and closes; Caddy was still forwarding the body, and
   its write failed (`write: broken pipe`) before it read the refusal. Measured through Caddy with the first image:
   200 bodies of limit + 1 → 190 × 413, 10 × 502 (`body_limit_race.ts`). That made `docker-stack-check` (and so
   `deploy-check`) fail about once in twenty runs, and a learner would have seen it as a server fault. Fix: Caddy
   compares the declared `Content-Length` with the same placeholder and answers 413 before it forwards anything
   (`@oversized` + `respond @oversized 413`); `request_body max_size` stays for bodies that declare no length. After:
   300 × 413, limit exactly → read (400 from the proctor), empty and small bodies unchanged, a streamed over-limit body
   413, no error line in Caddy's log, `caddy fmt`/`validate` clean (`body_limit_edges.ts`). The drift check holds both
   placeholders to one default.
2. **A site build empties `dist`**, the staged bundle included (Vite's `outDir` lies inside its root). So
   `dist/proctor` staged now is gone after the next `publish`/`build`/`deploy-check`: run `docker-stack-bundle` again
   after them (seconds; the image stays). The README states the order (publish, image, bundle).
3. `docker-stack-bundle` writes into `--out` without deleting it: what an operator already put into `certificates/`
   stays.

## For the owner of the other files

- Old host names outside this scope: `🎓️teaching/🛂️proctor/🔨️modules/🎚️config/🦀️.rs:49` (doc comment,
  `https://quizzes.architektur-und-technologie.de`), `🎓️teaching/🏛️architecture/🐾️pets/README.md:3`
  (`quizzes.architektur-und-technologie.de`), and
  `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧑️contributors/🧫️fixtures/🧑️‍💻️contributor-documents.json:7` (a long
  line that mentions the domain; not inspected further). None of them is read by the drift check.
- `🧪️tests/🥞️layered-home/🟦️.ts` had syntax errors at 14:10 (lines 264 and 295: `TS1002 Unterminated string literal`,
  `TS1128`), which hides every semantic error of the whole package from `tsc`; mid-edit by the other session.
- `dist/proctor` holds the image built from the tree as it was at 14:19–14:24 (`48d881aa…-dirty`, with the final stack
  files). Any later change of the Rust proctor, of `compose.yaml` or of the `Caddyfile` needs `docker-image-build` and
  `docker-stack-bundle` again; the bundle verb refuses an image whose stack files are not the checkout's.
- Another session staged files in the git index while this work ran (`git diff` showed only the last edits); no git
  command that modifies anything was used here.
- `🗑️generated/` is shared with the other session (`e2e-*.log`, `site-build`): only this work's files were deleted from
  it, the folder itself was left.

## Not proven

- Anything outward: the upload of the site to its CDN, DNS for `quizze.architektur-und-technologie.de`, reaching
  `semio.iek.uni-hannover.de` on 80/443, a Let's Encrypt certificate, a certificate of a real authority (the proofs use
  certificates of Caddy's internal CA, for `localhost` and for the proctor host's name).
- The supplied certificate on a Linux host's bind mount: ownership and mode were proven in a Docker volume; Docker
  Desktop on Windows does not carry a host file's owner into the container, so `sudo chown root:root … && chmod 600`
  of the README is not rehearsed as written.
- The verbs through Nx: `bun nx show project @teaching/architecture-quiz` died with `Plugin Worker for @nx/js is exiting
  as it did not receive a load message within 10 seconds` on this loaded machine; `📋️project.json` is valid JSON and the
  new target mirrors its neighbours, the drift check accepts every `@teaching/architecture-quiz:<target>` named in the
  docs, the workflow and the root scripts.
- The launch rows were not started from the IDE.
- `publish`, `build`, `typecheck` (whole package), `test-e2e` and `deploy-check` with these changes.
- Compose 2.24 on the host: `--force-recreate caddy` leaving the proctor alone and a moved tag recreating the proctor
  were observed with compose v5.5.1 only.

## Scripts kept in the ticket folder

`caddy_certificate_probe.sh`, `certificate_fingerprint.ts`, `body_limit_race.ts`, `body_limit_edges.ts`,
`tsconfig.deploy-retarget.json`, `cut_region.py`, `cut_lines.py` (the last one could not write a file another process
held open; the edit was made by hand instead).
