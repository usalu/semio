# Remodel Browser Acceptance on 2026-11-21

The browser runner now requires scoped rendered content on both editors for the identity receipt. The example receipt compares visible stream and camera IDs and frame counts against all three committed examples. Media imports require the expected shared counts and a new stream while retaining the prior streams. The JPEG check additionally needs an added durable archive member. Finalization needs both matching displayed products and a changed snapshot. Mesh and QC downloads are checked against the GLB and report contracts, and the GLB counts must match the finalized mesh shown in the editor.

The browser port now asks Chromium to cancel an actual file-chooser request. It requires a file-input chooser event and restores interception after the interaction. The Remodel receipt also requires the original and peer archives to match before and after cancellation. The browser adapter test requires a trusted DOM cancel event, no change event, and an empty file selection. The sequencing test covers valid and rejected browser input events and asserts interception cleanup in both cases.

The registered test runner was invoked through `repo`. Vitest's esbuild process failed with `EPERM` and its fork pool timed out. The ticket-local `🧪️browser-fixture-runner.ts` preload reruns the authored fixture assertions through Bun’s test API. The Remodel fixture file passed 18 tests with 154 assertions. The shared browser-port fixture file passed 9 tests. Both retain the authored JSON fixture inputs and independent SQLite and AJV checks.

```bash
SEMIO_LANGUAGE=en repo tools exec --light bun -- test --preload './<ticket>/🧪️browser-fixture-runner.ts' './✏️s/🔌️plugins/📸️remodel/🧪️tests/🌐️browser/🧪️tests/🟦️.ts' './🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/👥️two-human/🧩️opened/🧪️tests/🟦️.ts'
SEMIO_LANGUAGE=en repo tools exec --light tsc -- --project '✏️s/🔌️plugins/📸️remodel/📦️packages/🟦️typescript/tsconfig.json' --pretty false
```

The live browser-port test stopped at the loopback HTTP listen with `EPERM`. A direct Playwright launch through the registered Bun tool also stopped before navigation when pinned Chromium's `sandbox_host_linux.cc` returned `shutdown: Operation not permitted`. The repo MCP calls for catalogue, Hub, React host and browser execution returned 300-second timeouts. The native repo acceptance entry hit a read-only host-capacity directory, then a read-only resident-service directory when retried with a writable cache. A direct script invocation correctly refused without a parent invocation. No browser behavior, two-human journey, viewer content, or persisted per-window configuration was verified on this host.

The declared full acceptance route is `repo run @semio-tech/remodel-js:browser-acceptance` with `--hub`, `--serve`, `--locale`, `--kinds 3d.remodeling`, and `--out` under this ticket's generated output directory. The development Hub target is `os-hub:local-hub-owner` on port 8787, and the Remodel React prototype host uses port 6063.

## Live Environment Recheck

- `SEMIO_LANGUAGE=en repo service host status` reached source admission and exited 1. Owned auxiliary output publication to descriptor 3 failed with `EPERM` on `write` (reference `07fd2723`). A resident graph socket pathname was visible on disk. The status call did not establish a connection to a running service.
- A loopback listen through the registered Bun tool still failed. Both `http://127.0.0.1:8787` and `http://127.0.0.1:6063` returned `FailedToOpenSocket`. The execution namespace showed zero TCP listeners in `/proc/net/tcp` and `/proc/net/tcp6`.
- The registered Playwright Chromium launches exited before navigation in both modes. Headless reported `sandbox_host_linux.cc:41 shutdown: Operation not permitted`. Headful reported `crashpad/util/linux/socket.cc:45 setsockopt: Operation not permitted`. Display variables and socket pathnames were visible. Tool discovery exposed no connected browser control or browser endpoint.

The managed repo MCP execution host can launch pinned Chromium. A `repo/capacity_run` probe with 512 MB used `repo tools exec --light bun` to launch and close Chromium successfully. Both declared origins returned `ConnectionRefused` there. A listener and process census found no running Remodel React server or `os-hub` process. MCP `repo/service_host` returned `No repo graph service is running on this host` in 1.54 seconds. Current-graph `repo/show_project` queries for `@semio-tech/remodel-js`, `@semio-tech/framework-os-dev`, and `os-hub` each timed out at 300 seconds.

The shared browser fixture could open its loopback HTTP socket on the managed host but required an original parent Script invocation when run directly. The ticket's `🧪️browser-playwright-probe.ts` instead drove the existing adapter against a real Chromium page with a detached file input, matching the shell picker shape. A 768 MB `repo/capacity_run` using `repo tools exec --light bun -- ./<ticket>/🧪️browser-playwright-probe.ts` exited 0. The page observed an actual trusted `cancel` event with zero files, and a `change` event would have failed the assertion.

The managed execution host could read the ticket's 230259496-byte component. Filesystem probes derived from the repo owners found no canonical Remodel browser module, activation directory, or published Hub executable under either prototype or release. The React serve operation requires an activation receipt or runs activation. The component browser publisher consumes the original compiler capture and emits the module and receipt. The ticket component by itself does not satisfy that ownership path. The local Hub owner selects `stdio,gis,note,writer,draw,puzzle`, which excludes Remodel. Read-only trusted catalog preflight for `stdio,gis,remodel` exited 1 because those rows were absent from the generated registry. That path subsequently requires a shipped release component and matching receipt.

The recheck did not start a build. The trusted picker cancellation passed in isolation. Live acceptance still needs the published browser runtime and a running Hub with a Remodel catalog.

## Independent WebAssembly Materialization

The latest stripped component in the ticket scratch area was 230,726,631 bytes with SHA-256 `5dd1d04cce07fab7e40177efa1fec2c694538a79e8d3c31fc3c5433799edfe75`. The staged native transpiler was absent. Pinned Jco 1.39.0 ran through `repo/capacity_run` with a 4096 MB grant and used the browser materializer's two framework import mappings. It emitted one 230,643,531-byte core module (SHA-256 `4f4e851e07c042a872c5b80ab0d29e7dc61655ee237e54c3e99d70c105b38f7e`) and its JavaScript and TypeScript bindings in 12.75 seconds.

The retained `🧪️browser-wasm-core-probe.ts` input served that core over loopback as `application/wasm`. Pinned Chromium 156.0.8078.4 ran `WebAssembly.compileStreaming` successfully in 623 ms, with 73 imports and 104 exports. The registered Bun runner ran under a 3072 MB managed grant and wrote an observation only after the compile completed.

The retained `🧪️browser-wasm-descriptor-probe.ts` input staged the actual repository host shim, browser Preview2 shim and generated-module rewrites under sibling `module` and `vendor` directories. The second Jco emission's core hash equaled the first. Registered Node 24.15.0 ran the unchanged repository descriptor probe with `--experimental-wasm-jspi` under a 3072 MB managed grant. The guest's actual `describe()` returned a 139,988-byte pack. The repository canonical verifier accepted it. It was byte-for-byte equal to the checked-in Pack (SHA-256 `1cde010821201041444c1591476b55d6a680ed4d864dc81eb9c7f7684a28198e`), and canonical JSON values matched the checked-in JSON.

Pinned Chromium then imported those same staged browser modules from a temporary loopback server. Browser orchestration adapted the authored probe's Node URL and stdout transport while keeping its original WIT export checks and guest grant. The 3072 MB managed capacity run exited 0 in 2.97 seconds. A separate verifier read Chromium's 139,988-byte pack and confirmed the same SHA-256, exact checked-in Pack bytes, and equal checked-in JSON values.

The managed runner used `repo capacity run --memory=4096 -- repo tools exec jco -- transpile <component> -o <staging>/module --name remodel --map semio:framework/pure=./🟨️.js semio:framework/host-async=./🟨️.js` for the descriptor stage. The retained descriptor probe takes `stage`, `chromium` or `verify [chromium]` under `repo tools exec --light bun`. For Node, the stage creates `descriptor-probe.mjs`, executed via `repo tools exec --light node -- --v8-pool-size=2 --experimental-wasm-jspi <staging>/descriptor-probe.mjs <staging>/module/remodel.js`.

This independent staging produced no managed component receipt, Hub executable, or shared artifact journey result.

### Standalone Reactor Smoke Review

The browser component bridge's `createActorApi` requires the original operation admission, clock, and return carrier. Its poll path first admits the owned actor context and input. The shard worker documents that the kernel supplies the actual `instance-open` event, including application identity, configuration and quotas. The browser staging fixture checks lifecycle and UI patch routing through a generated bridge with fixture guest responses. The standalone Jco actor-import test runs a separate, purpose-built guest and has no input route for the Remodel component. These existing harnesses did not provide a supported standalone Remodel lifecycle open producing a real UI patch. The independent probe stopped at genuine guest `describe()` in Chromium and Node.

### Rebuilt Component and Bundled Examples

After the linker finished, the staging copy was 230,716,297 bytes and matched the published SHA-256 `a0d10976a1ca24d6b6ce2d9fbd8486731a44fd104e398554e12d0d1b8831d692` before and after the probe. The existing Jco route emitted a 230,633,197-byte core with SHA-256 `ee07ecf4017e15faf15df586f442179f6bccb7f21e23e4154e366ca08a62ce50` in 10.84 seconds.

The existing Chromium descriptor probe, with the real repository shim and browser Preview2 modules, completed in 2.817 seconds under the 3072 MB managed grant. The emitted Pack was canonical and measured 139,958 bytes with SHA-256 `7c7dcdf70145fe16dd94335ee95a566c5ab622c81e99ab978b1a9c9f343a331c`. The verifier compared exact example text via the authored picker fixture's file paths.

| Example | Source bytes | SHA-256 |
| --- | ---: | --- |
| `demo` | 1,910 | `ca6f10283286e3fdc3ee953ac04f14b0a24cc26e61cce2fe822efe8066855133` |
| `demo-session` | 2,159 | `817f7f5af8d697aa5a9108ccb3a381c9b9e4024c91b0fcdc51e5198bbf974021` |
| `synthetic-orbit` | 4,372 | `0126dcecf8eba6c4eb35be754796571cd711c3809cdb9e1c9e53a1041ac547af` |

All three matched byte-for-byte. A recursive comparison against the previously checked-in JSON found changes only in the three `manifest.examples[*].artifactJson` strings. The retained probe's `verify-examples chromium` phase emitted the canonical exact JSON and preserved the browser-emitted Pack under `🗑️generated/browser-materialization` for the ticket owner. This evidence is scoped to the component hash above and its browser `describe()` output.

### Final Close-Lifecycle Relink

The `wasm-close-lifecycle.sha256` linker marker identified a new 230,717,924-byte component. The staged copy's SHA-256 matched `367f00c20914a73606dd5e01f0b6cfc3baa896090bb874f21d500795ec4d4f10` before and after the probe. The unchanged Jco route produced a 230,634,824-byte core with SHA-256 `cfbf4997a3157564f40f97d1254e0374cbc25d1c40a0dd9153e3e56571d162cb` in 14.051 seconds.

The managed Chromium `describe()` run completed in 8.794 seconds on that relink. Both `verify chromium` and `verify-examples chromium` passed. The fresh canonical 139,958-byte Pack had SHA-256 `7c7dcdf70145fe16dd94335ee95a566c5ab622c81e99ab978b1a9c9f343a331c` and was byte-identical to the updated checked-in Pack. Its canonical JSON values matched the checked-in JSON, and all three example texts matched their current source bytes and hashes listed above. The final Pack and JSON were retained as proof outputs, and the independent staging inputs and intermediates were removed. The descriptor probe did not exercise the reactor close lifecycle or a Hub-backed browser journey.

## Isolated viewer reactor

The ticket probe called viewer-open on the actual guest through the repository's generated component bridge, actual host shim and production renderer work stock. It sent the production viewer ID `s.remodel.remodeling@1/*#viewer` and the renderer's empty initial configuration, capabilities, assets and quotas. Its later stages require a captured instance receipt, the Model surface's guest-issued patch, and the exact patch acknowledgement. The probe creates no guest receipts or host answers.

Pinned Jco took 19.599 seconds to restage the 230,717,924-byte component. The component and its 230,634,824-byte core matched the previously verified SHA-256 values `367f00c20914a73606dd5e01f0b6cfc3baa896090bb874f21d500795ec4d4f10` and `cfbf4997a3157564f40f97d1254e0374cbc25d1c40a0dd9153e3e56571d162cb`. The managed runner used a 3072 MB capacity reservation and `repo tools exec --light bun -- ./<ticket>/🧪️browser-remodel-guest-probe.ts`. The first attempt reached the browser's 50-second deadline. The stage-logging retry reported `evaluate: Target crashed` in 46.172 seconds. The diagnostic invocation with `--diagnose` added only stage markers to the generated bridge and selected Liftoff and lazy WebAssembly compilation. It reported the same crash in 35.999 seconds.

Chromium completed the actual guest channel-version call and nine context-admission calls. It returned from input admission and crashed inside the first guest `reactor.poll(slice)`. The kernel recorded renderer SIGSEGVs at 03:31:06, 03:32:47 and 03:34:02 local time, all at the same executable-relative instruction. The fault address was 0x8fc0 above the stack pointer in each case. The matching kernel journal window contained no OOM event. The systemd journal confirmed the low-tier attempt generated a core dump.

The same exact staged guest and diagnostic bridge in pinned Node v24.15.0 returned `RuntimeError: wasm trap: out of bounds memory access` from wasm-function[34319] at core offset `0xbfdc63`, called by wasm-function[34317] at `0xbfd890`. The 3.623-second run validated ten original receipts through the production owner, nine context receipts and one input receipt. No completed poll or actual lifecycle receipt, scene patch or application effect was returned. The cross-check command was `repo tools exec --light node -- --v8-pool-size=2 --experimental-wasm-jspi --liftoff-only --wasm-lazy-compilation ./<ticket>/🧪️node-remodel-guest-probe.mjs`, through the same managed capacity owner.

The ticket's `🧪️wasm-shadow-stack-probe.py` read the exact stripped core's instruction bytes. Mutable global 0 starts at 8,388,608. The trapping function starts at `0xbfdc23` and subtracts 11,030,240 from that global, then writes back the stack pointer. The first trap address is in a byte store addressed from that frame. Six identifiable active callers reserve a further 4,510,368 bytes. The trapping frame alone exceeds the configured shadow stack. The only custom section in the emitted core was `producers`, so this binary does not provide function names.

The earlier unstripped guest retains the three opening function names. The exact 40-byte instruction prefixes and relative function positions identify the trapping function as `plugin_advance_actor_opening::<RemodelApps>` and its immediate callers as `plugin_resume_actor_opening::<RemodelApps>` and `plugin_open_actor_instance::<RemodelApps>`. The current indices are 34319, 34317 and 34315. The earlier unstripped indices are 85, 83 and 81.

## Viewer with the 32 MiB relink

The isolated relink's exact 230,693,559-byte component had SHA-256 `80fce366a40446965a9321b5e6819b4b08766e41285c7dd055b82065e2ad4801`. Managed Jco restaged it in 16.977 seconds. Its 230,610,459-byte core had SHA-256 `89104be24e0feab5228dd8c5fc7da6316b3f05a77be53116686a86d351cd2906`. Its initial shadow-stack pointer was 33,554,432 bytes. This ticket-local relink was verified separately from the published component.

Chromium 156.0.8078.4 passed the same real viewer probe with default browser flags and the unmodified generated bridge in 3.251 seconds under managed capacity. The low-tier diagnostic invocation also passed in 4.853 seconds. The guest returned an exact viewer capture receipt for activation 1, instance 1, guest lifetime 1 and request sequence 1. Across 41 real polls the production root owner validated 91 returned original receipts. The guest published Model surface revision 1 over base revision 0, addressed to the same lifetime, with patch sequence 1, nine upserts and a set-root operation. The World3d mesh lane and instance lane carry `remodeling-result` with the 36-index startup cube. The guest consumed the exact patch acknowledgement. No application effect descriptors were emitted on this read-only startup projection.

This verifies actual isolated viewer startup and default model projection. Reconstruction, persisted artifact consumption and full Hub-backed browser acceptance remain separate checks.

## Editor command ingress with the 32 MiB relink

Two independent default-Chromium runs opened and acknowledged the actual editor lifetime in 39 turns. The first encoded the published `importFramePayload` action using the checked-in 75-byte `📷️two-by-two.png` as a data URL. The second encoded the simpler `importFrames` action whose application handler requests a file picker. Both used the renderer's AppChannel command codec and paged ingress producer. The ticket runner invokes these scenarios with `--editor` and `--editor-picker`.

Each command was admitted and then stayed `command-pending` for all 1,024 continuations allowed by the normal renderer. The import attempt took 8.332 seconds under managed capacity and consumed 28,053 guest work units after editor startup. The picker attempt took 8.178 seconds and consumed 27,295. The real root owner validated 2,137 original receipts in each run. Neither completed command ingress or returned an application frame, application effect descriptor or UI patch. The import was not confirmed, and no browser picker was opened by the guest in these runs. The common blocked lane precedes guest publication independently of PNG decoding.

An editor `ReadDocumentIdentity` control on the same bytes, invoked with `--editor-read`, passed in 2.888 seconds under managed capacity. The same AppChannel ingress completed in 37 polls with 1,070 work units and two genuine `send-message` effects. The decoded `DocumentIdentity` frame matched request sequence 1 and instance 1 and carried a guest-created document identifier and base revision. The root owner validated 161 receipts across startup and the control. The failures above are specific to action invocation rather than all AppChannel commands or editor initialization.

## Exact Cargo component with the 32 MiB stack and terminal custody

The standard Cargo build produced a 230,721,883-byte component with SHA-256 `d18a4b245215b15d89b3e3b3c2e8455babafcf3b92582a7abe5c54c223d85593`. Pinned Jco materialized it in 12.286 seconds. Its 230,638,783-byte core had SHA-256 `e8034c14700e3fbc31daaecf4688fbdde8441bab593fb8bf50f727f4b8992276` and a 33,554,432-byte initial stack pointer.

The actual Chromium descriptor call completed in 2.104 seconds. It returned a 139,958-byte Pack with SHA-256 `7c7dcdf70145fe16dd94335ee95a566c5ab622c81e99ab978b1a9c9f343a331c`. Both the Pack bytes and JSON values match the checked-in pair. The embedded demo, demo-session and synthetic-orbit examples still match their respective source files byte for byte.

Default Chromium passed the actual viewer lifecycle, default model projection and exact patch acknowledgement in 2.749 seconds. The 41 polls and 91 original receipts matched the relink observation. Chromium also passed the real editor `ReadDocumentIdentity` command in 3.133 seconds. That run completed 76 polls, validated 161 original receipts and decoded two guest `send-message` effects, including the requested document identity.

The exact component published the editor Frames patch with the real window key and the renderer's two-pane roster. The probe acknowledged that patch before testing either editor command. The frame and picker commands each made more than 330 genuine command-pending polls and then returned a semantic refusal before any application effect or response frame. The existing compiled guest diagnostic identified its primary cause as `Window owner must declare its original disposer birth`. The paired work receipt spent 9,848 work units, copied 9,775 items and 2,552,329 bytes, and released 572,316 bytes. Remodel's Frames, Model and Report config owner implementations provide bounded disposer factories but omitted the explicit birth declaration required by the framework. The probe made no production edits or fabricated progress receipts.
