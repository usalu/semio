# Deployment Readiness — 2026-10-28

## Verified Current State

The former standalone release build handle 35547 is no longer available: its owning write_stdin tool returns Unknown process id 35547. No replacement standalone build was started. The coordinator's full release end-to-end invocation owns the proctor build and remains the authoritative local verification path.

The separate external runtime audit supplied by the coordinator reports the public frontend and its assets answering HTTPS 200, while two bounded connections to the proctor's public HTTPS /instance endpoint timed out. A connection timeout establishes external unavailability from this environment; it does not identify whether the cause is ingress filtering, an unavailable host, or a listener that is not running.

## Canonical Deployment Contract

The authored source is [deployment configuration](/home/deck/semio/🎓️teaching/🏛️architecture/📚️course/🚀️deploy/🔣️.json), supported by [the runbook](/home/deck/semio/🎓️teaching/🏛️architecture/📚️course/README.md), [Compose](/home/deck/semio/🎓️teaching/🏛️architecture/📚️course/🚀️deploy/compose.yaml), [Caddy](/home/deck/semio/🎓️teaching/🏛️architecture/📚️course/🚀️deploy/Caddyfile), and [the manual workflow](/home/deck/semio/.github/workflows/architecture-course.yml).

| Owner | Authored value |
|---|---|
| Site origin | https://architektur-und-technologie.de |
| Proctor origin | https://semio.iek.uni-hannover.de |
| Image repository | ghcr.io/usalu/architecture-course-proctor |
| Internal API listener | 8791, exposed only to Caddy on the Compose network |
| Host footprint | ~/architecture-course, containing compose.yaml, Caddyfile and optional retained overrides/certificates |
| Durable learner state | architecture-course_proctor-data volume, proctor.sqlite and its WAL/SHM siblings |
| Public ingress | 80/tcp, 443/tcp and 443/udp |
| Runtime | Linux amd64, Docker Engine 25+, Compose 2.24+ |

The image supplies production mode, trusted proxy forwarding and its course. Compose grants the site origin, runs the proctor as an unprivileged user with a read-only root and no capabilities, and starts Caddy after the proctor is healthy. Caddy terminates TLS and proxies to proctor:8791. It obtains certificates automatically when reachable; an operator-supplied PEM is also supported. No certificate or key content was inspected.

The README records historical external ingress failure on 2026-10-02. That historical statement is not adopted as the current cause.

## Supported Access And Infrastructure

The repository's registered tool listing currently reports Docker and gh as missing. It contains no SSH tool registration. The deployment implementation invokes Docker through the repository tool interface; it implements image production/check/publication and stack rehearsal/bundling, not authenticated remote host installation. The runbook's only SSH example is a backup with the unresolved placeholder <user>. No university login identity or authenticated host connection is declared in the inspected deployment files. The current account has no ~/.ssh/config. No private environment values, credentials or key files were read or printed.

The manual GitHub workflow gates releases on its shared check job and only releases from the actual configured default branch. It requires an ARCHITECTURE_COURSE_RUNNER Actions variable naming a capable dedicated Debian 12 x64 runner. Its proctor job checks an image and stack and publishes the image; it does not start or update the university host. The source declares use of GITHUB_TOKEN for registry publication and OIDC for Pages. The current live runner variable, package visibility and remote host state were not authenticated or verified.

The generic repo remote-execution and remote-cache commands are build infrastructure. Their presence does not provide a declared university deployment login.

## Exact Next Availability Action

1. Complete the already-running local release learner verification against the current proctor. Do not publish an image or mutate the remote host before this succeeds.
2. Obtain a declared, repository-supported authenticated host access path, or have the university's existing operator perform the documented read-only host inspection. The first host checks in ~/architecture-course are docker compose ps and docker compose logs --since 1h proctor caddy. These were not executed remotely in this audit.
3. Compare that host result with external ingress. If both services are healthy and the API answers inside the stack while external HTTPS still times out, the next owner is the university network/operator for public ingress and routing. If a service is absent or unhealthy, its concrete host logs and selected image identify the next deployment or runtime repair. A certificate replacement alone cannot repair a public TCP connection timeout.
4. After local release proof, use the repository readiness/image gates below on a capable Docker host. For an existing deployment, take the documented consistent backup before selecting and applying a verified release image. Keep its existing data volume, certificates and other overrides. For first installation, use the verified image's own stack files or the registry-free bundle.
5. Verify public HTTPS GET /instance returns 200 with id teaching-proctor; HTTP redirects to HTTPS; OPTIONS /queries from the authored site origin returns 204 with the corresponding allow-origin; then rehearse sign-in, a scored run and leaderboard from the public site.

The missing authenticated host path is an access fact, not a request for remote mutation. No guessed SSH login, remote command, image push or deployment was attempted.

## Repository Commands For The Prepared Release

All commands are declared course targets; SEMIO_LANGUAGE=en selects the command language. These are the next operator commands after the current local release proof, not commands executed by this audit:

```text
SEMIO_LANGUAGE=en repo run @teaching/architecture-course:deploy-check --configuration=release
SEMIO_LANGUAGE=en repo run @teaching/architecture-course:docker-image-build --configuration=release
SEMIO_LANGUAGE=en repo run @teaching/architecture-course:docker-image-check --configuration=release
SEMIO_LANGUAGE=en repo run @teaching/architecture-course:docker-stack-check --configuration=release
SEMIO_LANGUAGE=en repo run @teaching/architecture-course:docker-stack-bundle --configuration=release
```

deploy-check already coordinates the course validation, image build, image check, stack check and browser gate, with typecheck and site publish as graph prerequisites. It pushes/uploads nothing. The individual commands above support focused repair and packaging rather than requiring a second complete gate run after an unchanged successful gate.

docker-image-publish is the explicit remote registry mutation. It checks the image and stack before pushing a content-addressed rollback tag, workspace version and latest. It is not a host deployment. The registry-free docker-stack-bundle packages the exact image, stack files, pinned .env and host instructions; its default output is the course TypeScript package's dist/proctor.

Local Docker availability remains unproven because the registered tool is missing; no daemon or image gate was run. The empty site.legal object is an authored readiness warning for missing imprint/privacy URLs, separate from the backend connection failure.

## Scope

Only this report was created for the readiness investigation. The older backend report contains the manifest/FEM repair ledger and source-analysis evidence. The main release invocation continues independently. No deployment configuration, remote system or git state was mutated.

## Remote Access Prerequisites Refresh — 19:52 UTC

One read-only IPv4 TCP attempt to130.75.188.198:22 through repo Node completed2026-10-28T19:52:13.704Z with connect timeout after10020ms (configured10second bound). No SSH protocol exchange/authentication, username guess, private-key read or server mutation occurred. This machine could not establish SSH transport in that attempt; it does not identify the network/server cause or prove access from another network.

Inspected only authored deploy config and course README. Config names the public site/proctor hosts, image and port but no host-login identity. README331 and612 use the literal placeholder <user>@semio.iek.uni-hannover.de for copy/SSH examples. No concrete authorized login was declared in these sources. Docker registry public-image instructions at463 concern image pull credentials, not SSH host access.

The runbook331–343 requires an authorized operator on the Linux host to load the staged image and start its compose stack, with /instance as readiness. Its historical firewall explanation at345–347/441–445 is authored operator guidance, not a cause established by current transport probes. Prior80/443timeouts and this22timeout establish only observed reachability failures from this environment. Credentials/account identity and authority to administer the public host remain separate prerequisites; a reachable port would not supply either. No request to alter DNS/firewall/certificates or send messages was made.
