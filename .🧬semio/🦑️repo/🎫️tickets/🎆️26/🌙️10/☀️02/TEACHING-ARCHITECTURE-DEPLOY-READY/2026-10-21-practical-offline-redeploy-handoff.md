# Practical Offline Redeployment Handoff

Prepared instructions for the eventual verified final artifact. No artifact existence, export success or deployment success is asserted. Substitute only the actual accepted artifact root and tag/revision from final receipts.

Expected artifact layout: `<artifact-root>/site/` is the built course site and `<artifact-root>/proctor/` is the self-contained backend stack. Proctor contains `README.txt`, `compose.yaml`, `Caddyfile`, `.env`, `certificates/` and `proctor-image.tar`. Transfer the entire directory, including hidden `.env`; no repository checkout or registry download is needed on the destination. Use the generated bundle compose, whose pull_policy is never for both images, rather than the authored development compose with missing policy.

Host prerequisites from actual bundle runbook: Linux amd64, Docker Engine25+ and Compose plugin2.24+. Keep the same Compose project identity/directory for updates so the existing named volumes remain attached. DNS for the configured proctor host must reach this host; open80/tcp and443/tcp,443/udp as required. Learner browser must reach trusted HTTPS.

From the transferred `<artifact-root>/proctor` directory, these are intended operator commands only (not executed by this audit):

```sh
docker load --input proctor-image.tar
docker compose up --detach --wait
curl --fail https://<actual-proctor-host>/instance
docker compose logs --since 1h proctor caddy
```

Before starting, inspect `.env`: PROCTOR_TAG must equal the actual accepted release tag; retain other operator overrides. Default backend host is semio.iek.uni-hannover.de and allowed site origin is https://architektur-und-technologie.de. PROCTOR_ALLOWED_ORIGINS must match the actual HTTPS site origin; backend DNS/TLS host must match site/backend configuration. Serve the contents of site/ from its configured HTTPS origin using the destination hosting system. Do not invent alternate origins while preserving an artifact built for different endpoints. Optional COURSE_HTTP_PORT/COURSE_HTTPS_PORT alter host binding only.

TLS options: allow Caddy to obtain/renew its certificate with reachable DNS/ports, or place one `<name>.pem` in certificates/ containing full certificate chain then private key, readable to container Caddy (generated runbook specifies root-owned0600). A supplied certificate must cover actual PROCTOR_HOST. After replacing it:

```sh
docker compose up --detach --force-recreate caddy
```

Persist `proctor-data` (SQLite/WAL/SHM), `caddy-data` (ACME/account/certificates) and `caddy-config`. Back up before updates and retain copies off-host:

```sh
docker compose exec -T proctor proctor backup - > proctor.sqlite
```

For an update, preserve current `.env`, certificates/ and volumes; copy new verified image archive/compose/Caddyfile, set PROCTOR_TAG to new actual accepted tag while keeping other overrides, load the archive and run up --detach --wait again. Loading an archive alone does not change the tag in an existing `.env`. `docker compose down` keeps data; do not use down --volumes on the persistent deployment.

Before handing these directories to an operator, actual final evidence must join the same cohort/tag/revision throughout controller terminal, image labels/config IDs, proctor product receipt, export receipt and final verifier. Authenticate the artifact inventory including proctor/proctor-image.tar bytes/hash; final offline-archive check must bind both Linux amd64 configs, layers/tags and executable provenance to actual exported archive. Final operator-contract check requires exact generated offline compose, `.env` tag, Caddyfile and README. Both images must be present locally after load, using the archive’s immutable backend config and local Caddy digest tag.

Actual managed controller proof must include browser preparation, course suites/typechecks, material five-fingerprint receipt, deployment checks and bundle creation. Actual final verification/export acceptance must precede claiming delivery; after destination startup, confirm /instance returns teaching-proctor JSON and perform the intended course/proctor browser smoke against the same HTTPS site/backend origins. Container health alone is not the material/browser/product proof.

Source-backed from current stack-bundle producer (generated README/environment/offline compose), authored compose/Caddyfile, and final verifier operator-contract/offline-archive checks. No CLI, production helper or Source mutation performed.

## Active CB37 Input Projection — 2026-10-21T16:08:37.169Z

Current actual Native prefix session97506, cohort `cb37fb2500e87070c8b17ab93a5e15d6`, candidate deployment tag `architecture-cb37fb2500e8`. Expected eventual output remains `🎓️teaching/🏛️architecture/📚️course/📦️packages/🟦️typescript/dist/redeploy-cb37fb2500e8/{site,proctor}`; it does not yet exist as verified delivery evidence. Prefixunits/capture6, capturedneutral37, controller21, exactrealfacts/export/final verification all remain required. Historical images, cachedreceipts, preparedmetadata and launch alone cannot satisfy them.

For the first downstream transition, use only current `2026-10-21-cb37-captured-neutral-field-map.md` pins and actual authenticated five raw receipts after prefixaccepted+cleanup+outer0. No suffix plan was authored in advance. Public service publication was not requested; this workflow produces the reusable offline package for redeployment and a validated same-tag local rehearsal.
