# External Runtime Audit — 2026-10-28

Read-only HTTPS audit from the development host, using managed Node through `SEMIO_LANGUAGE=en repo tools exec --light node -- -e`, native fetch and `AbortSignal.timeout(15000)`. Hosts come from `🎓️teaching/🏛️architecture/📚️course/🚀️deploy/🔣️.json`; expected readiness comes from the course and proctor READMEs. No commands, queries, registration, user runs, WebSockets, publication or remote mutation were performed.

## Observations

- `https://architektur-und-technologie.de/`: HTTP 200, `text/html; charset=utf-8`, no redirect. Title: Architecture and Technology Quizzes · Quizze zu Architektur und Technologie. HTML size 5,706 bytes.
- HTML references `/assets/🌐️-Cesb4stJ.js` and `/assets/🌐️-3G5lB9Lg.css`. Both return HTTP 200 with appropriate JavaScript/CSS content types. Decoded response sizes: 1,245,686 and 271,599 bytes respectively. These are uncompressed sizes and do not establish a gzip-budget failure.
- The HTML has no HTTP CSP header, but does contain a CSP meta element. It limits scripts and styles to self plus hashes, permits inline style attributes, and allows HTTPS/WSS connections to `semio.iek.uni-hannover.de`; object-src and base-uri are none. The JS includes that configured proctor host.
- `https://semio.iek.uni-hannover.de/instance`: two independent GET attempts failed to connect. The second reports `UND_ERR_CONNECT_TIMEOUT`, attempted host port 443, internal connect timeout 10,000 ms. The first also failed after approximately 10.5 seconds. No HTTP status or instance JSON was available.

## Interpretation and Limits

The public frontend is already deployed and serves its declared entry assets. Backend readiness is not demonstrated from this host: network routing, firewall or service availability may explain the timeout. This does not prove a universal outage. The README requires GET /instance to return 200 with instance id teaching-proctor; this requirement could not be verified.

The current source imports WIRE_VERSION from @semio-tech/course and explicitly prevents commands and queries when the proctor contract differs. Because no public instance document was obtained, public command/query kinds and wire compatibility cannot be assessed. A reachable frontend is insufficient evidence that the deployed source revision matches the local source or that current learner journeys, assignment PDFs, persistence, multi-user behavior, CORS and backend protocol work. Asset delivery and host wiring are verified only at the level stated above. Local integration and end-to-end verification remain necessary.

No source files were changed. No generated output files were retained.

## Bounded Ingress Follow-up

The follow-up inspected only the declared backend hostname and standard public HTTP/HTTPS ingress ports, with 15-second socket/request timeouts. DNS A resolves to `130.75.188.198`; DNS AAAA returns `ENODATA`. TCP connection attempts to ports 80 and 443 both time out after about 15,020 ms. A TLS handshake attempt on port 443 also times out after about 15,020 ms, before certificate or TLS protocol evidence is available. An explicit IPv4 HTTPS GET `/instance` times out after about 15,013 ms. An explicit IPv6 HTTPS GET `/instance` fails `ENOTFOUND` after 11 ms, consistent with the missing AAAA record.

The public frontend explicitly configures this backend hostname; no reachable backend path was demonstrated from this environment. These outcomes establish a DNS-resolving but currently unreachable IPv4 public ingress path from the development host. They do not identify the cause or establish that VPN access is required: service state, firewall policy, routing and network access requirements remain unknown. No alternative ports, private endpoints, broad scans, authentication or remote mutations were attempted. This evidence supplies no deployment authority or host access path.

## Public Refresh — 2026-10-28 18:42–18:43 UTC

Read-only Node HTTP/DNS/TCP probes through repo tools exec --light; independent requests ran concurrently, each transport attempt bounded at15 seconds. No browser/build job or external mutation.

At18:42:58.968Z, configured public frontend https://architektur-und-technologie.de/ returned200 in331ms. Its referenced first-party /assets/%F0%9F%8C%90%EF%B8%8F-Cesb4stJ.js returned200 in651ms; /assets/%F0%9F%8C%90%EF%B8%8F-3G5lB9Lg.css returned200 in442ms. These are transport/asset checks, not a browser application or backend integration pass.

Backend semio.iek.uni-hannover.de resolved through the system all-address lookup to130.75.188.198, family4; no IPv6 address was returned by that lookup. Both explicitly IPv4 http://semio.iek.uni-hannover.de/instance and https://semio.iek.uni-hannover.de/instance reached the absolute15-second bound (15005ms each) without an HTTP response status.

A separate direct IPv4 TCP check completed at18:43:30.190Z:130.75.188.198:80 timed out connecting after15004ms and :443 after15003ms. Thus this machine could not establish TCP to either public backend port in these attempts. This is a measured connection failure, not evidence identifying firewall, server process, routing, university policy or certificate as its cause; TLS and application behavior were not reached. Public backend availability remains unproven.

## Public Artifact Contract Check — 2026-10-28 18:44 UTC

One bounded root plus its single referenced entry JS was read in memory, without crawling lazy chunks or loading a browser. At18:44:32.972Z both returned200. The entry was /assets/%F0%9F%8C%90%EF%B8%8F-Cesb4stJ.js (1,235,825 JavaScript string characters).

The root meta Content-Security-Policy explicitly contains connect-src https://semio.iek.uni-hannover.de wss://semio.iek.uni-hannover.de. The same HTTPS origin is directly present in the served entry JavaScript. Thus the public artifact demonstrably points to the configured university backend, whose connection failure was measured above. No HTTP CSP header was returned by this probe; the observed policy is the HTML meta policy.

Local current TS schema 🧰️framework/🛍️products/📚️course/🧬️schema/🟦️.ts:32 and Rust schema 🧬️schema/📚️quiz-documents/🦀️.rs:29 both declare WIRE_VERSION=10. The bounded entry scan did not directly identify WIRE_VERSION, wireVersion, wire_version or course.identify-learner. Minification/lazy splitting can remove these strings, so absence is not proof of another protocol. The served entry hash,200 status and matching backend origin do not establish current-source identity or wire10 compatibility. No public /instance contract could be compared because the preceding backend transport checks could not establish TCP. Public deployment version remains unidentified.

## Current Bounded Availability Recheck — 2026-10-28T21:43:31.845Z

Executed retained availability-probe-2026-10-28.mjs through registered repo Node with SEMIO_LANGUAGE=en. Four GET requests ran concurrently, each with a10000ms abort signal. No login, keys, remote mutation or process changes. Raw bounded output: 🗑️generated/availability-probe-current.json.

| Endpoint | Actual Result | Elapsed |
| --- | --- | --- |
| https://architektur-und-technologie.de/ | HTTP200, text/html; charset=utf-8 |315ms |
| https://semio.iek.uni-hannover.de/instance | TimeoutError; no HTTP status/body obtained |10000ms |
| http://127.0.0.1:6061/ | ECONNREFUSED |5ms |
| http://127.0.0.1:8791/instance | ECONNREFUSED |5ms |

Requests began at21:43:21.797–21:43:21.847Z. Public frontend retained prefix contains the bilingual Architecture and Technology Quizzes title and CSP connect-src naming the public proctor HTTPS/WSS origin. Only the first response body chunk, at most4096bytes, was retained before cancelling the body. This proves current frontend root HTTP availability and recognizable metadata; it does not verify assets, JS execution, gameplay, current Source deployment identity or a functioning frontend/backend session. No parsed proctor instance identity was obtained anywhere.

The public backend timeout agrees with prior bounded observations but does not diagnose DNS/TLS/firewall/server cause or prove universal unavailability. Local refusal establishes these exact loopback ports did not accept the requests at this instant; it does not inventory other ports or override queued qualified-owner status. Persistent default frontend/proctor availability remains unproven.

## Availability refresh2026-10-29T00:31UTC

Reused retained availability-probe-2026-10-28.mjs through registered repo Node. Four parallel GET requests began00:31:38.189–.241UTC and finished00:31:48.240UTC, ten-second per-request abort, no mutation. Distinct raw output: 🗑️generated/availability-probe-2026-10-29T00-31-48-240Z.json.

- Public frontend: HTTP200 in448ms, HTML charsetUTF8, same final URL. Retained prefix identifies bilingual Architecture and Technology Quizzes title and CSP pointing to public proctor.
- Public proctor /instance: TimeoutError at10,000ms; no HTTP status or instance identity obtained.
- Local127.0.0.1:6061: ECONNREFUSED in4ms.
- Local127.0.0.1:8791/instance: ECONNREFUSED in4ms.

The frontend evidence is only the first body chunk capped at4096bytes, not asset execution, rendered intro, gameplay, deployment revision or complete application health. No public backend availability or current local pair is established. This HTTP refresh does not substitute for genuine browser or both-topology E2E.

## Endpoint Recheck 2026-10-29T01:12:33.002Z

- {"url":"http://127.0.0.1:6061/","elapsedMs":43,"error":"fetch failed","cause":"ECONNREFUSED"}
- {"url":"http://127.0.0.1:8791/instance","elapsedMs":16,"error":"fetch failed","cause":"ECONNREFUSED"}
- {"url":"https://architektur-und-technologie.de/","status":200,"elapsedMs":338,"proctorIdentity":false,"contentType":"text/html; charset=utf-8"}
- {"url":"https://semio.iek.uni-hannover.de/instance","elapsedMs":10001,"error":"The operation was aborted due to timeout","cause":null}

Bounded endpoint observations only; frontend HTTP success alone does not establish current Source identity, gameplay, browser integration or deployment completion. No remote mutation performed.

## Public refresh and declared host route02:01UTC

Actual parallel GETs began2026-10-29T02:01:48.729/.772UTC; final02:01:58.773UTC. Public frontend returned200/HTML in367ms, first chunk capped4096bytes identifying bilingual course title and CSP public backend origin. Public proctor /instance timed out at10,001ms; no API status/instance identity obtained. Raw output: 🗑️generated/public-availability-2026-10-29T02-01-58-773Z.json. No browser/deployment revision/gameplay proof follows from this prefix.

Deployment config names site architektur-und-technologie.de, proctor semio.iek.uni-hannover.de, image ghcr.io/usalu/architecture-course-proctor and internal8791. Runbook declares GitHub Pages and university linux/amd64 Docker Engine25+ host with Caddy. Operator directory ~/architecture-course, services proctor/caddy, external80/443, proxy proctor:8791. SSH is documented only as <user>@host, not an actual account/credential. Authenticated host inspection uses Docker compose health/logs; no SSH/login attempted. Timeout cannot distinguish firewall/host/TLS/service state.

Existing capability/config names only: Actions variable ARCHITECTURE_COURSE_RUNNER; workflow contents:read, pages:write/id-token:write and packages:write via GITHUB_TOKEN; optional registry read:packages for private-image pulls. No secret values or credential files read, no current access inferred.

Available tool metadata was searched for host/SSH/service/deployment access. No callable connector for this university host, SSH session or Docker service was found. Sites tools concern their own hosted service and confer no university deployment access. No plugin installation, login, deploy, DNS/service mutation or local browser/build occurred.

## Fresh Public Observation

{"url":"https://architektur-und-technologie.de/","at":"2026-10-29T13:09:22.551Z","status":200,"bytes":5706}

{"url":"https://semio.iek.uni-hannover.de/instance","at":"2026-10-29T13:09:22.593Z","error":"The operation was aborted due to timeout"}

Public reachability is separate from the locally live proctor8791. No remote mutation was performed.

## Current Local And Public Reachability

{"url":"http://127.0.0.1:6061/","status":200,"at":"2026-10-29T16:04:50.925Z"}

{"url":"http://127.0.0.1:8791/instance","status":200,"at":"2026-10-29T16:04:50.913Z"}

{"url":"https://architektur-und-technologie.de/","status":200,"at":"2026-10-29T16:04:51.629Z"}

{"url":"https://semio.iek.uni-hannover.de/instance","error":"The operation was aborted due to timeout","at":"2026-10-29T16:05:00.900Z"}

## Public Endpoint Recheck — 2026-10-29 17:27 UTC

A bounded Node HTTP probe through the repo tool interface completed with outer status0. The public frontend https://architektur-und-technologie.de/ returned200 in292ms with HTML. The public proctor https://semio.iek.uni-hannover.de/instance timed out after10001ms. Each request had its own10000ms abort deadline. No remote mutation or deployment was attempted.

The independent web tool could not access either URL, so it provides no additional availability conclusion. The direct network observation remains specific to this machine and time; a public backend response is still unverified. Local service evidence does not resolve this deployment gap.
