# Runtime Launch Instructions — 2026-10-28

Source-only preparation; no process was started, no port was probed, and no build was run. Run after the canonical gate settles successfully. Current main4072, backend16280 and frontend82275/daemon32120 remain independently owned.

## Supported Local Stack

From the repository root, retain one owned command session for:

~~~sh
repo run @teaching/architecture-course:dev --configuration=prototype
~~~

The command syntax is identical on native Windows, macOS, Linux and in the devcontainer; it does not need shell-specific environment assignment. Ensure its inherited SEMIO_BUILD_MODE is absent or prototype, because explicit configuration conflicts intentionally refuse. The default build backend is the repository policy backend; repo --tool=bazel run @teaching/architecture-course:dev --configuration=prototype selects the same owner through Bazel when wanted. No dashboard invocation is required.

README.md:100 documents this whole-stack dev target. The actual router 📦️packages/🟦️typescript/📜️script.ts:47–51 calls runDevStack, and its dev-site target at53–58 is only the frontend. Use dev, rather than independently launching dev-site and a raw proctor. The project manifest targets.dev declares continuous=true, cache=false and prerequisites assets, styling tokens, qualified proctor build and both assignment PDFs. The managed stack acquires the qualified proctor product itself; a native Cargo unit-cache build alone does not replace that receipt.

Default settings require no additional variables:

| Item | Default / Readiness |
|---|---|
| Frontend | http://127.0.0.1:6061/ ; GET / must answer, followed by real browser introduction check. |
| Backend | http://127.0.0.1:8791/instance ; successful response must contain teaching-proctor. /health and /ready are not the declared probe. |
| Site port override | TEACHING_ARCHITECTURE_COURSE_PORT |
| Backend port override | PROCTOR_PORT |
| Persistent development data | .🧬semio/🎓️teaching/proctor-dev/proctor.sqlite ; override directory with PROCTOR_DATA. |
| Course material | 🎓️teaching/🏛️architecture/📚️course/🔣️.json ; override with PROCTOR_COURSE only when deliberately selecting another course. |
| Source following | Enabled normally; TEACHING_ARCHITECTURE_COURSE_WATCH=off disables automatic following for a steady stack. |

Defaults come from course 🧱️stack/🟦️.ts:109–116,400–404, proctor 🏗️bootstrap/🟦️.ts:45–65,70 and422–437, and project targets.dev readiness metadata. Vite routes /instance, /commands, /queries, /actors and /scopes to the local proctor with websocket support (course 🏗️builder/🌐️vite/🟦️.ts:24–25,180–183). Dev needs no baked public proctor URL; ordinary browser traffic uses the frontend origin and proxy.

For an intentionally separate beside stack, set TEACHING_ARCHITECTURE_COURSE_PORT=6063, PROCTOR_PORT=8793 and PROCTOR_DATA to the absolute workspace path .🧬semio/🎓️teaching/proctor-beside before the same command; set the watch variable off if steady operation is desired. These are documented metadata-backed settings (README.md:122–131 and targets.dev parameters). Set environment using the host shell or the owning launch tool; do not assume POSIX assignment syntax on Windows. Distinct ports must also have distinct data directories.

## Readiness And Ownership

Retain the returned owned execution session instead of ending it when startup prints readiness. The stack announces an address only after its readiness probe answers (🧱️stack/🟦️.ts:102–106,323,433). Record both actual ready URLs and a browser introduction/material check in ticket evidence. Source review alone does not establish availability.

runDevStack reuses an already ready backend or site and explicitly says it will not stop it (420–435); a reused backend receives a contract comparison, and a mismatch is announced at422–424. If both already answer, the command returns rather than creating a new owner. Therefore record whether each service is owned by this session or retained from a named preexisting session; do not claim the new command owns reused processes.

Ctrl+C, original invocation cancellation, SIGTERM or terminal close signal the owned stack. Its finally block stops only its launched frontend and supervised proctor and releases watchers/artifact readers, using process-tree termination (🧱️stack/🟦️.ts:412–417,447–457; launchOwned:145–147; SupervisedProctor.stop:295–307). A site exit also ends the stack (442–444). Keep the command session alive for the promised availability; shut it down through that same session when explicitly retiring the local service. Do not stop peer gate/build sessions to obtain ports.

## Public Host Is A Separate Deployment

Local managed dev availability does not publish the site or repair the public backend. The current authored deployment configuration 🚀️deploy/🔣️.json names https://architektur-und-technologie.de and https://semio.iek.uni-hannover.de ; the latter is the public /instance readiness origin. The ticket description still names quizze.architektur-und-technologie.de, so report the current config hostname explicitly rather than silently conflating them.

The production site is a staged static release artifact with its backend origin baked in, while the proctor host uses its staged Docker/Caddy stack, persistent volume, DNS, reachable80/443 and certificates (README.md:3–7,21,29 and deployment runbook). The E2E rehearsal and local dev command do not install or maintain that remote stack. Existing external-runtime/deployment-readiness reports remain authoritative for observed public reachability. No remote upload, DNS change, firewall change or service mutation is part of this launch preparation.

## Isolated Native Backend Diagnostic Preparation

Source-only instructions while backend17816 owns canonical native compilation; no launch/build/probe was performed by this audit. From cwd /home/deck/semio/🎓️teaching, the supported scoped invocation is:

~~~sh
repo cargo --store=shared --configuration=release --parallelism=1 -- run -p teaching-proctor --bin proctor -- serve
~~~

The repository Cargo launcher schema Configuration.const.subcommands.compile explicitly admits run, and the launcher retains its Original mode and Cargo-tail guards. The actual package/bin mapping is teaching-proctor/proctor. The second separator belongs to Cargo: it passes exactly serve to the program. Proctor CLI 🔨️modules/⌨️cli/🦀️.rs:55–58 admits exactly [serve]; ports/course/data are environment variables, not invented serve flags.

Set the owning launch environment deliberately: PROCTOR_DATA to an absolute isolated directory beneath the ticket generated folder; PROCTOR_COURSE to the absolute 🎓️teaching/🏛️architecture/📚️course/🔣️.json; PROCTOR_BIND=127.0.0.1; PROCTOR_PORT to a chosen nonconflicting local port; PROCTOR_MODE=development; PROCTOR_TRUSTED_FORWARDING=none. Do not share the persistent dev or peer data directory. Reset conflicting inherited PROCTOR_* explicitly, particularly bind/course/data/origins. Config 🔨️modules/🎚️config/🦀️.rs:5–11,212–224,349–359 requires course/data, defaults loopback8791, and infers development on loopback. At390–402 absent allowed-origins permits loopback browser origins; an explicit allowed frontend origin can narrow it. No public production origin is required for this isolated local diagnostic.

After the managed command reports listening, GET http://127.0.0.1:<chosen-port>/instance is the declared readiness endpoint; bootstrap/🟦️.ts:422–437 expects successful status and teaching-proctor in the response. Then POST /queries with the actual query envelope, matching connector queryEnvelope:332–342 and Rust end-to-end harness159–170:

~~~json
{
  "queryId": "00000000000000000000000000000001",
  "kind": "course.course",
  "version": 10,
  "scope": "architecture",
  "principal": {"kind":"anonymous"},
  "arguments": [123,34,116,121,112,101,34,58,34,99,111,117,114,115,101,34,125],
  "consistency": {"kind":"authority"},
  "cursor": null
}
~~~

arguments is UTF8 JSON bytes for {"type":"course"}. Successful query result is kind=snapshot whose value bytes decode to the catalog/course view. Queries module:76–88 enforces current wire version, exact architecture scope and matching query shape before returning the loaded course view; this first read requires no seeded learner. Actual server startup CLI:136–147 loads course, assembles, prepares/catches up and binds. No state-changing signup is needed merely to establish catalog readiness.

Keep the owning repo Cargo run session alive during the bounded diagnostic and stop through that same owner when finished. This directly runs an actual locally compiled native backend under repository admission. It does not issue or replace the qualified proctor product receipt needed by architecture-course:dev, acquireProctor, E2E or staged release packaging. Successful native readiness would be new limited evidence, not full course release/E2E success or public university-host availability.

Command correction: the prepared commands use the common launch environment SEMIO_LANGUAGE=en. The unsupported --lang=en token was removed; use repo run/cargo with the language supplied by the owning launch environment. No command or service was launched for this correction.

## Runtime CLI Language Flag Audit

Read-only bounded current Source search found NO hardcoded --lang token in course/proctor runtime TS/MJS/JSON, including package routers, stack, E2E, bootstrap/product and project commands (excluding docs, generated/dependency trees and non-watcher tests). No concrete reachable nested invocation blocker was identified. Existing runtime commands use repo ./script.ts, repo run targets or issued Cargo/owned tool execution without injecting the rejected global repo --lang=en argument.

Watcher mapping verified: caching project target test-caller-environment invokes repo ./📜️script.ts test caller-environment; suite catalog maps caller-environment to 🧪️tests/🌍️caller-environment/🟦️.ts. That test and its watcher-startup helper subtree contain no --lang token. Its actual SDK child probes use captureOwnedCommandV1 node -e and scriptInvocationEnvironment, preserving the launch environment. The targeted process execution/playground helpers inspected likewise contain no --lang injection. This is bounded Source evidence, not a new managed watcher/runtime pass.

Unrelated caching ledger error texts/shared language-parser unit assertions do mention --lang, but they are not hardcoded global repo process invocations on these paths. A script-specific/help language flag must not be confused with prepending a flag before the installed repo command. Source help advertising a flag does not supersede the already observed native-bootstrap rejection.

Use command argv beginning repo run or repo cargo, and set SEMIO_LANGUAGE=en in the owning environment for all prepared runtime/gate launches. Earlier global-flag examples in this historical launch report are superseded by this correction; target/configuration/ownership requirements remain unchanged. No runtime Source edits, gates or processes were launched by this audit.

## Fresh Browser Introduction Against An Already Managed Pair

Existing Playwright config has NO webServer/global service lifecycle. Existing boot project only opens fresh browser contexts and checks rendered localized introduction/branding/text; the separate existing both-languages language-following test checks en-US/de-CH, unsupported fr-FR bilingual chooser, explicit choice and reload persistence. Neither selected test invokes the proctor stop/start control helper. Device fixture closes only its own browser contexts. Therefore a registered direct Playwright invocation can validate an already owned persistent pair WITHOUT starting/replacing/stopping those services.

After the managed owner has real readiness, use normal owning environment SEMIO_LANGUAGE=en,absolute SEMIO_TICKET_DIR,PLAYWRIGHT_BASE_URL=http://127.0.0.1:6061/,TEACHING_ARCHITECTURE_COURSE_E2E_TOPOLOGY=dev,TEACHING_ARCHITECTURE_COURSE_E2E_PROCTOR=http://127.0.0.1:8791. Leave E2E_CONTROL unset: these selected tests do not need a control server. Do not copy learner storage, inject preview/identity or mock backend. From repo root, exact existing registered command:

~~~sh
repo tools exec playwright -- test --config 🎓️teaching/🏛️architecture/📚️course/🎭️e2e/🎚️config/🟦️.ts --project boot --project desktop --grep "the site boots and shows|the language follows the browser" --workers 1 --output <absolute-ticket>/🗑️generated/persistent-introduction
~~~

Require actual2selected tests/nonzero pass and fresh contexts, not a no-tests result. This intentionally narrow PREVIEW check does not replace any full-gate membership. The boot spec warms once with tolerated cold-load failures, then separately creates fresh en/de contexts whose unannounced console/page/request errors fail; it verifies real introduction title/paragraphs, actual logo image dimensions and no unresolved labels. The language test exercises actual rendered chooser/selection/reload, not just HTTP metadata.

For direct backend readiness/catalog/CORS independently of frontend fallback, existing retained proctor-native-smoke.mjs can first run through registered Node against http://127.0.0.1:8791 and actual course JSON (same exact invocation structure recorded above). It asserts exact teaching-proctor identity and query/CORS; the browser introduction alone cannot prove all backend-dependent gameplay or rule out local fallback. Neither command owns the existing services. Keep persistent managed handle live throughout and record actual URLs/owner separately.

Browser prerequisite follows existing course-browser-preparation audit: direct Playwright CLI does NOT call E2E ensureBrowser, so if actual installed package Chromium/headless-shell revision is absent, coordinate the one existing registered repo tools exec playwright -- install chromium first, with same repository browser cache. Never substitute system Chromium. Do not duplicate install while another normal gate owns preparation. This report performs no browser preparation/launch or HTTP check. Actual capacity admission still applies to the registered browser process; limit1worker narrows concurrency, not required full acceptance scope.

## Additional diagnostic frontend ingress review

No existing supported route was found that avoids the blocked managed Native helper while retaining all current course Vite contracts. The supported standalone frontend owner is `repo run @teaching/architecture-course:dev-site --configuration=prototype --parallel=1` with real backend at PROCTOR_PORT. It declares assets:build, styling-tokens:generate and both assignment PDF producer dependencies; unlike dev it does not own proctor startup. This is a valid separate service owner, but it still uses normal repo/Nx acquisition and is not an established bypass around the current helper failure.

Direct registered Vite is insufficient: the actual config calls viteCacheDirectory, which refuses without the original bounded SEMIO_VITE_CACHE_DIRECTORY inside the canonical products partition. It also installs courseAssignmentPdfsVitePlugin, whose configureServer requires an authenticated course-assignment-pdfs runtime channel; missing transport throws. DevSiteScript obtains actual published PDFs under withPrintExecution/withPublishedPrintDocument custody, retains/rechecks them for the entire server lifetime, and calls runViteBunxDev with actual compiler admission control and runtimeChannels. That function checks Original/control lineage and delegates to serveViteCommandV1 with owned config/mode/runtime. CoreRuntime selection does not supply these missing artifact/channel/cache authorities.

The real dev config proxies instance/commands/queries/actors/scopes to127.0.0.1:PROCTOR_PORT, default8791; PROCTOR_URL is a build-time release origin, not a dev proxy override. Latest actual availability00:31UTC found local6061/8791 refused, so no configured live backend pair is ready to demonstrate this diagnostic. Existing qualified release artifact evidence alone is not a live service. This review did not replay PDF receipt validation or assert prototype product readiness.

Thus no additional launch is recommended before fixing the canonical helper ingress. Do not synthesize cache/channel environment, copy assets, omit PDF plugin or launch a raw unsupported Vite process. No process, build, browser or gate was launched; full normal dev and both-topology E2E remain required.

## Revalidated minimal persistent-pair browser invocation

Current existing selected tests remain exactly the site boot introduction law and the language-selection law. Run from repository root only once the actual persistent pair is ready:

`SEMIO_LANGUAGE=en PLAYWRIGHT_BASE_URL=http://127.0.0.1:6061/ TEACHING_ARCHITECTURE_COURSE_E2E_TOPOLOGY=dev TEACHING_ARCHITECTURE_COURSE_E2E_PROCTOR=http://127.0.0.1:8791 TEACHING_ARCHITECTURE_COURSE_E2E_CONTROL= repo tools exec playwright -- test --config 🎓️teaching/🏛️architecture/📚️course/🎭️e2e/🎚️config/🟦️.ts --project boot --project desktop --grep "the site boots and shows|the language follows the browser" --workers 1 --output .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️02/TEACHING-ARCHITECTURE-DEPLOY-READY/🗑️generated/persistent-introduction`

Config has no webServer or service startup/teardown. Both selected projects have no dependencies on the exclusive presence/shortage/away/fault projects. Learner fixture only creates and closes fresh browser contexts and observes console/page/request failures. Neither selected test calls the proctor stop/start function. Empty CONTROL prevents accidentally providing a service-control endpoint. This invocation therefore does not own or stop persistent services.

Boot warms an English browser then checks fresh English/German visitors against every authored introduction paragraph, actual visible brand image/load dimensions and unresolved labels. Language law checks de-CH/en-US matching; unsupported fr-FR renders bilingual language choice with no default app/html language, then actual German selection/reload and header switch/reload. It visits the actual root route, not authorPreview. These2cases establish rendered intro/language behavior, not all10quizzes, full material interactions, answer persistence or both-topology completion.

Registered Playwright uses its owned package/browser cache; no executable override appears in config. Current root Node resolution of playwright-core/package.json failed MODULE_NOT_FOUND, so this audit does not assert which managed browser revision is installed. The prior version-derived preparation report remains applicable: the normal full runner ensureBrowser reads its actual installed browsers.json and checks Chromium/headless INSTALLATION_COMPLETE; direct CLI does not invoke that helper. If that actual revision is missing, canonical preparation is `repo tools exec playwright -- install chromium`, coordinated once, not system Chromium substitution. No browser, installer, build or service was launched here.

## Separate registered browser provisioning route

A supported existing route DOES exist, without starting/stopping course services or selecting E2E tests:

`SEMIO_LANGUAGE=en repo tools exec playwright -- install chromium`

The actual process/tools registry declares playwright on Darwin/Linux/Windows, resolves its package cli.js via Bun, constrains version from root devDependencies/playwright, permits all callers and projects PLAYWRIGHT_BROWSERS_PATH to the owned tools/ms-playwright cache. Tool arguments are forwarded; no registry subcommand restriction excludes install. Therefore this runs the same declared Playwright package installer rather than ambient npx or system Chromium. Preserve the same explicit PLAYWRIGHT_BROWSERS_PATH, if one was already selected for later testing; otherwise use the registry default consistently.

The full course runner private ensureBrowser is not separately exported or exposed as a router verb. It resolves playwright-core from the real installed Playwright CLI (correctly avoiding the absent root playwright-core link), reads that package browsers.json and checks both Chromium/headless installation markers before issuing install chromium. The registered tool command requests the same version-derived browser installation but does not invoke the private skip-if-present helper. This distinction does not require a new interface for provisioning.

Registry package provisioning is separately declared as repo tools setup all when the Playwright package itself is unavailable; that broad setup is not a browser-only preparation and must not be confused with installing the browser revisions. Course ci-browser-prepare installs Debian system dependencies and is also a different operation. No installer, version probe, browser or service was launched; actual package availability and successful revision preparation still require the normal owned command result.

## Actual registered Playwright resolution01:15:24UTC

Read-only registered resolver/environment functions executed through repo tools exec --light bun, without spawning Playwright. Current resolver selects package-script /home/deck/semio/node_modules/playwright/cli.js via the managed Bun runtime; real CLI is /home/deck/semio/.cache/b/d/m/j/864244544477f3c50412/node_modules/.bun/playwright@1.64.1/node_modules/playwright/cli.js. Installed package version is1.64.1; registry constraint is root ^1.57.0.

Resolved playwright-core through that REAL CLI using createRequire, as the full runner does. Core is same managed generation at .bun/playwright-core@1.64.1/node_modules/playwright-core/package.json. Its actual browsers.json requires Chromium and chromium-headless-shell revision1248, browser156.0.8078.4.

Actual registered cache is /home/deck/semio/.🧬semio/🦑️repo/⚡️cache/tools/ms-playwright. Both chromium-1248/INSTALLATION_COMPLETE and chromium_headless_shell-1248/INSTALLATION_COMPLETE are absent. Expected Linux binaries chromium-1248/chrome-linux64/chrome and chromium_headless_shell-1248/chrome-headless-shell-linux64/chrome-headless-shell are also absent. Thus browser preparation is concretely missing even though the registered Playwright package now resolves. Earlier root require failure was merely the isolated package-link topology, not proof Playwright itself was unavailable.

Supported next provisioning remains SEMIO_LANGUAGE=en repo tools exec playwright -- install chromium, retaining this same cache. No installer, version subprocess, browser launch or service mutation occurred in this audit.

## Installed revision and host prerequisites after14552

Root reports registered install14552 authoritative0. Independent filesystem check at2026-10-29T01:16:55.991Z found all three installation markers as nonsymlink regular zero-byte files in the same registered cache: chromium-1248, chromium_headless_shell-1248 and ffmpeg-1013. Actual browser executables are nonsymlink regular mode755 files: chrome301,186,376bytes; headless207,474,000bytes; ffmpeg5,270,472bytes. Marker mtimes are01:15:38.656,01:15:54.201 and01:15:39.695UTC respectively. This supersedes the earlier missing-revision result.

Host metadata identifies SteamOS3.9.5, IDsteamos/Arch-like. Actual pinned1.64.1 coreBundle hostPlatform logic falls back to ubuntu24.04-x64 with officially-supported=false for this distro. At01:18:21.192UTC bounded selective reads parsed actual browser ELF64 program/dynamic/string tables without executing ldd or any binary. All32Chromium and28headless direct DT_NEEDED names have existing files in standard library locations; FFmpeg has no dynamic table. No concrete missing direct library was found.

These checks do not verify recursive dependency resolution, symbol versions/ABI, fonts, sandbox/display configuration or browser launch. No installer, browser or service was executed by the audit. Evidence was communicated during the root Source/report quiet window and appended only after release at01:21UTC. The real two-case browser invocation still requires the actual frontend/backend pair.

## Actual Registered Browser Host Launch

Root23397 ran SEMIO_LANGUAGE=en repo tools exec playwright -- screenshot --browser chromium about:blank <ticket>/generated/browser-host-smoke.png and authoritatively exited0. Normal capacity admission reserved2048MB. Registered Playwright navigated to about:blank and emitted a4254-byte validPNG1280x720, verified from its signature/IHDR. Retained generated/browser-host-smoke.log and generated/browser-host-smoke.png are tool output. This proves an actual bundled browser launch/render/close on this host; it proves no course route, backend integration, quiz or fullE2E assertion.

## Complete E2E coexistence with persistent6061/8791

Current full runner automatically isolates both topology stacks. endToEndPorts uses two simultaneously bound OS-selected loopback port0 listeners, closes/joins the probe and requires distinct valid ports; withStack rechecks availability before launch. It does not select6061/8791 or reuse their listeners. Each stack gets a fresh timestamp/PID run directory and per-label proctor-data. proctorEnvironment strips EVERY inherited PROCTOR_* value and supplies the actual test port/data/course/posture; dev-site receives the selected site port and test proctor port with watching off. No special test-port/data override is needed or declared by CLI.

Exact complete route remains `SEMIO_LANGUAGE=en repo run @teaching/architecture-course:test-e2e --configuration=release --parallel=1 -- --topology both --serial`. Do not add project/name filters. Both is already the default; serial sequences topologies through the existing worker scheduler, not a test omission. Existing full runner prepares/reuses the actual registered revision1248 browser cache.

Each local control endpoint captures only its own running proctor object and is allocated on another free loopback port. stop/start closes/restarts that owned proctor with the same test data. Cleanup reverse-joins only objects this stack launched: proctor, control listener, dev server or static listener and retained release artifact reader. Failure, interruption and success all enter that finally path; keep preserves output directory, not live services. No code here finds/kills persistent6061/8791 owners. OS port selection/recheck has a normal bind race, but no observed collision is established; a busy selected port refuses rather than authorizing peer termination.

Firstvisit/both-languages fixtures create fresh browser contexts and actual root-route intro/identity interactions; test proctor origin/control are supplied per stack. The runner-selected exclusive shortage/fault suites intentionally stop ONLY their test proctor via its own endpoint. Rehearsal builds a release site for its isolated cross-origin production proctor, holds the actual artifact reader through tests and validates its site artifact. Persistent service routes do not substitute for either topology.

This is current Source ownership/coexistence evidence, not an executed full suite or browser pass. No browser, port probe, service or build was launched by this audit. Resource scheduling and retained artifact readers remain real prerequisites; do not duplicate an active full runner.
