# Current Browser Acceptance Matrix

Read-only current manifests, scripts and configs; no Node, browser, test, compiler or Native execution. Commands below are exact repo commands; set the listed explicit environment in the invoking platform shell. Keep SEMIO_LANGUAGE=en and the existing ticket environment. Do not inject GPU software overrides or filter projects/specs.

| Full Gate | Required Build Mode | Owned Prerequisite/Server Chain | Coverage |
|---|---|---|---|
| repo run @semio-tech/mit-bestand-demonstrator:test-e2e | prototype | test→continuous serve-e2e→prepare-e2e→activate-prototype + workspace:deps-browsers | Chromium full acceptance file, all eight panes including real adapter map behavior |
| repo run @teaching/architecture-course:test-e2e | release | assets build + styling tokens generation + proctor build; script owns dev and rehearsal stacks | Default both topologies, boot, desktop, phone, layout, pets, presence→shortage→away→faults |
| repo run @semio-tech/semio-tech-play:test-e2e | prototype | test→continuous serve-e2e→prepare-e2e→activate-prototype + workspace:deps-browsers | Chromium whole configured suite, activated full plugin roster and real map/wgpu adapters |

## Demonstrator And Play

Current roots are ♻️mit-bestand/🧺️demonstrator and 🏢️semio-tech/🎡️play. Each manifest dispatches test through 🔨️modules/🧪️e2e/📜️script.ts test, preparation through runtime/📜️script.ts prepare-test, server through runtime/📜️script.ts serve-test. Servers are uncached continuous dependencies with continuousSharing=false; calling test leaf directly omits activation/browser/server ownership and is not equivalent.

Prepare opens an invocation-bound generation session. Server reuses that exact session, binds127.0.0.1 port0, and publishes generation-ready HTTP identity; no fixed6029/6033 assumption applies to E2E (those are ordinary dev defaults). Test reads the same session and waits with600000ms ready ceiling and15000ms progress. PLAYWRIGHT_BASE_URL is supplied from actual readiness. Config refuses a missing URL and starts no independent Playwright webServer. Serve finally closes the service session; Vite receives the original invocation signal. Play additionally freezes file watching for its E2E server generation.

Both test bodies start registered playwright with the actual config, one Chromium worker, and repository tools/ms-playwright cache. Both configs select Metal on macOS and GL otherwise with real GPU/WebGPU flags. DEMONSTRATOR_E2E_GPU and PLAY_E2E_GPU must remain unset: their swiftshader branch is explicitly reduced and cannot establish full map/tile behavior. Headless browser defaults do not themselves replace the real adapter requirement. Driver/container inventory is prerequisite evidence only, not browser acceptance.

Concrete cancellation limitation: browser test bodies create local AbortController and wire SIGINT/SIGTERM, then cancel the Playwright child on that local signal. They do not explicitly link invocation.control.signal into readiness/child cancellation. The registered tool layer may enforce ambient lifecycle, but complete Original deadline/control propagation cannot be claimed from these bodies. This precise finding was sent to Root; no fix or narrowed test run was made.

## Course

Current owner is 🎓️teaching/🏛️architecture/📚️course/📦️packages/🟦️typescript. Canonical test-e2e dispatches its package script and runCourseEndToEnd in course/🎭️e2e/🟦️.ts. Default topology is both; no need to pass a narrowing selector. Optional --serial changes scheduling without dropping either topology, but --project, --grep or --topology dev/rehearsal narrow acceptance and should not be used for this complete gate.

Each stack probes two distinct OS-selected loopback ports, verifies freedom, and owns isolated proctor data/logs. Dev launches real dev site with proxy to development proctor. Rehearsal builds release Vite under Original, retains its immutable artifact reader through the journey, serves static output, and runs production proctor on a separate permitted origin. A third loopback control endpoint permits authored proctor-away/restart scenarios. Shared and exclusive shard scheduling preserves warmup boot and dependency order; worker count follows admitted memory/cores.

Course browser preparation checks the exact Playwright installation revision and installs Chromium if missing under the existing invocation. Specs use registered node with the actual pinned Playwright CLI; they are not arbitrary global browser executables. Original cancellation is explicitly combined with local interruption; abort terminates the owned child tree. Stack finally closes every owned component and retained artifact reader; failed runs retain ticket diagnostics. No direct Docker dependency exists in these three test-e2e declarations; container acceptance is separate.

## Execution Order And Evidence

Run the three canonical targets individually after the pending infrastructure/setup/build gates settle, preserving their own server lifetime and resources. Record real terminal status, full configured project/spec coverage, host GPU adapter and any explicit platform skips. None of these gates has been launched by this audit. A package/browser installation, Vulkan enumeration, focused schema law or successful daemon does not substitute for these results.

## Cancellation Boundary Confirmed

The registered startToolV1 implementation forwards options.signal only when explicitly supplied; both Demonstrator and Play test calls omit it. Thus tool registration does not itself supply the missing Original abort linkage. Exact owners are Demonstrator e2e/TS runTestScript11–31 and Play e2e/TS TestScript.run20–40. Play same-file release test at60 already uses withScriptInvocationSignalV1 and owned capture with the existing pages control; Course uses an explicit combined Original/interruption signal. These are existing lifetime patterns, not authority to issue a fresh controller Original.

No dedicated browser-body cancellation fixture was found. The nearest existing durable owner is Demonstrator tests/demonstratorcompileclosure/e2e TS with adjacent cases JSON and schema; it proves dynamic browser Source registration but not cancellation. Existing energy/generation cancel browser cases test domain UI cancellation and do not cover this launcher boundary. Root owns the concrete linkage repair; no Source edits here.

## Cancellation Repair Review

Current Demonstrator and Play browser bodies now combine the actual issued invocation signal with the existing local interruption signal. The combined signal feeds both readiness and registered Playwright spawn. Manual duplicate kill listeners are removed; the existing tool spawn owns abort termination. SIGINT/SIGTERM exit behavior, normal failed-browser errors and finally listener removal remain. No new Original or wider capabilities are introduced. No concrete blocker found in this delta.

The six neutral cases execute both actual AST-extracted bodies through explicit process/tool ports. Envelopes and signal refinement are genuinely issued with empty capabilities; tests check exact caller reason, child kill count, local exit codes, completed/failed outcomes and removed signal listeners through Ajv plus the owned schema validator. The compileclosure root statically imports/registers these laws, and both Source bodies are declared as DATA resources and inputs. New test/fixture/schema absolute path lengths are115/117/127UTF16 units, within256. Owner reports causal2pass10fail then12/12green; this reviewer did not rerun tests or launch Node/Native/browser work. Full browser acceptance still remains required.
