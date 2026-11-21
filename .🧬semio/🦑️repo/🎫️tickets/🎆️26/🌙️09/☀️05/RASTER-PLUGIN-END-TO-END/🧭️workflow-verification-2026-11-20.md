# Raster Workflow Verification on 2026-11-20

## Launch and registration

Raster is registered in the Hub composition catalog under the `raster` variant. The editor app ID is `s.raster.raster@1/*#editor`. The declared browser ports are 6060 for React and 6160 for wgpu. The current React entry point is `repo run @semio-tech/framework-os-dev:serve-raster-react-prototype --lang=en --output=stream`. The older `dev:raster` and `activate-raster-react-dev` routes have been superseded by renderer/profile targets.

The tracked descriptor is owned by the Hub component composition. The artifact crate alone does not build or publish the playable plugin. The first WASI check was admitted via `repo cargo` against the Hub workspace and is compiling on one core. A fresh preview and descriptor verification require staging the built component and an activation receipt. The current local browser product partition did not contain either before verification started.

## Test evidence

- The 19 existing tests suites from the Raster TypeScript test route ran directly through the repository tool interface while the graph runner was queued. They passed 243 tests and 885 assertions, including decoded pixel, selection geometry, layer transforms, editable archive fixtures, and PNG encoding oracles. This first run preceded the new example and source-image regressions.
- The Hub I/O matrix now has an explicit Raster PNG export row. It opens Raster from the catalog, opens a nonempty example and verifies the existing editable document export/import route and download validity. The new pin was read successfully by the owning harness.
- [Browser workflow](🔬️raster-browser-workflow.ts) drives the React canvas with real pointer events. It compares PNG pixels via Sharp before a stroke, after edit/undo/redo, and after the editable archive is imported in a fresh browser context. The initial attempt reached Chromium but `127.0.0.1:6060` refused the connection while the launch target was queued.
- Raster has a retained `image:in` media port. The global Open File action resolves PNG to the stdio PNG editor. It does not place a PNG on a layer in the currently focused Raster document.

## Current pressure

Local fleet reports show the launch target waiting for the shared graph and memory admission. The WASI check is making compiler progress. The host reports about 6 GiB of free disk versus its 15 GiB floor. The browser check remains pending until the preview can serve the current component.

## Component Command Repair

The repository advertises `repo ./📜️script.ts components iterate raster` as the standalone browser component loop. The direct command previously reached its handler without the native resources declared at invocation birth and failed with `Original native caller capabilities required`. Its closed root request list now includes the supported component operations. Unknown operations and help retain workspace-only access. An already received invocation retains its existing resources.

The new neutral cases reproduced the refusal and then passed against the existing SQLite selection oracle and Ajv checks. The actual Raster iteration now passes the missing-authority point and reaches the shared native storage lease. That confirms entry into the owned build path. It has not produced a staged browser component yet.

The rerun of the 19 Raster TypeScript suites passed 243 tests and 886 assertions after the source-image import fix. It includes the new 16-bit grayscale PNG and Unicode metadata oracle. The mounted React Vitest fallback requires its repository-issued test configuration and did not run when invoked directly.

The browser workflow now begins by importing a local PNG using the Raster picker, selects the imported layer before painting, and requires the import completion notice before checking a reopened editable archive. PNG bytes from each export are independently decoded with Sharp.

## Import and Renderer Checks

The picker now exercises a 262,913-byte PNG in the browser workflow. Sharp independently confirms its 256 × 256 source pixels. The picker sends 32 KiB chunks through the existing shell effect. The framework stages the ordered chunks before dispatching a typed Raster command. The app then gives the complete base64 source to the retained PNG producer used by its media port. The mounted app regression checks that two chunks create one layer and that undo removes it.

The canonical descriptor command is `SEMIO_BUILD_MODE=prototype SEMIO_LANGUAGE=en repo ./📜️script.ts components describe raster --profile prototype --facet browser`. It superseded the read-only iteration attempt once the new public action was stable. The WASI check was started before that action landed and is useful as a baseline only. The generated descriptor and running browser must come from the final source.

## November 21 Checkpoint

The editor's mounted PNG test now feeds two picker chunks through the framework staging path. Existing standalone command laws were mounted from their command crates so the mounted editor companion fits its Source limit. One mounted test source snapshot passed all 1,039 source-admission units before it was superseded by final source. The final mounted run exited at disk preflight before Cargo or tests, with approximately 5 GiB available after attempted reclamation. See [frontend verification](raster-editor-frontend-2026-11-21.md).

The framework's boxed command work adapter had been falling back to default methods for four ownership hooks. The adapter now forwards wire demand, wire receipt and retention, and original output-chunk receipt to the underlying work. An independent oracle reproduced the missing forwarding before the change and passed afterward. The native regression passed source admission and entered compilation.

The direct component descriptor build is held on the shared native dependency-image lease. The React launch also still awaits a repository graph and its memory grant. Neither process has published a fresh runtime or opened port 6060.

The installed headless Chromium returned a real WebGPU adapter on a secure loopback page with the workflow probe launch flags. This checks browser prerequisites while the Raster service is unavailable.

## Native ownership and Store publication

Raster's paged model map now moves each entry into a retained slot under a copy quote. It allocates the entry's retirement cursor only after payment, keeps the slot on refusal, and releases the empty map page with its exact layout receipt. The editor Store preparation and history replay now pass the original effect into the host candidate, snapshot retirement, publication assembly, and clone close. The Store factory checks and reports the full header copy it quotes.

The shared snapshot-read helper had quoted only the retirement frame allocation and depth. It now derives the payload transfer and frame allocation from the same typed constructor used by admission. The neutral regression reproduced the missing quote, then passed with SQLite's independent exact-versus-short decision (4 assertions). An additional Rust law checks the actual allocation marker, source retention with a one-byte short grant, and complete retirement after a funded move. The Rust law has not run yet.

The browser probe now selects the newly opened dock tab by its actual identity and performs its second stroke in that tab's accessible panel. The latest probe passed Bun syntax parsing. No Rust typecheck or browser workflow was established by a syntax check.

Current local sessions still include the WASI hub check, the React launch, and canonical descriptor generation. The first WASI attempt reached Raster's model and failed. The retry is compiling upstream shared/native dependencies and has not reached the final Raster sources yet. Disk recovered to approximately 9.4 GiB, below the 15 GiB floor. The descriptor still waits for the exclusive dependencies image holder.

## Latest build and test evidence

The second WASI check reached the Raster model. It reported seven errors, down from the first check's 45. The selection receiver now forwards its original source-turn witness, separates mutable work from reply settlement, and follows the packed builder's state without using `?` outside its result scope. Cold selection copies preserve the identity of the same packed page root through a funded immutable alias. The image payload uses the defining retained allocation work without importing an unavailable helper.

Raster’s owning test route now includes the selection alias and binary replay oracles. All 248 tests passed across 21 files with 909 assertions. The focused native shared image custody suite passed both transfer and cancellation laws. Cargo completed those tests before the repository wrapper was interrupted during post-test collection. The selection alias oracle also passed three cases and 15 assertions. Its native companion is pending. See [packed selection verification](🎯️packed-selection-alias.md). The native PNG picker test was also submitted through the repository's shared Cargo ingress. It is waiting for memory admission. The new WASI check started source admission using one compiler job. See [image custody and composite verification](🔗️raster-image-custody.md) and [host and export verification](raster-host-effects-2026-11-21.md).

The direct descriptor route and React lazy activation both acquire the same dependencies-image lease before compilation or staging. The setup process still holds it during dependency postorder selection. No staged Raster browser component was present in the product cache examined. The editable browser workflow remains unverified.

The three focused component birth, retirement transfer, and selection alias fixtures passed together with 231 assertions. Scoped whitespace checks and the latest browser probe’s syntax check also passed. The mounted browser pipeline still requires a current component.

The third full component check stopped upstream in `semio-os-store-one-item-publication` with E0599 for its binary receiving cursor’s `close_work_demand` method. The shared worktree already contained the missing trait method and its tagged implementation when we inspected the failure. A narrower WASI check of the Raster editor’s owning manifest entered compiler work on one job. The full component and browser verdicts are still pending.

## Mutation replay

Raster’s Store registers the original JSON operation reader for live publication. The editable SPR archive writes typed binary history through each mutation’s `encode_op` and restores the resulting forward and inverse payloads through `OpBinaryReceiving`. The Raster mutation reader now consumes that tagged record format for archive hydration. It retains image frames and metadata, packed selections, pixel samples, and adjustment parameters across bounded conversion turns. The native regression checks byte-identical encoding after receiving an operation from a larger source range. The independent Sharp and SQLite checks passed eight assertions. See [binary mutation receiving](📥️raster-mutation-receiving.md).

The shared decoder now propagates the Work quote of its selected retirement child. An independent oracle matched the Rust close selector across 144 states, including the two-unit action after value handoff. The repository Cargo law for that selection is still pending.

## Native browser launch

The setup process released the dependency image. The direct descriptor command then failed because its component driver supplied a profile-only environment to a compiler store that requires the live invocation. The driver now supplies the envelope from its typed parent. All seven component-stage tests passed. The actual descriptor command entered the compiler store and ran its scheduler.

The direct React command is `SEMIO_BUILD_MODE=prototype SEMIO_LANGUAGE=en repo ./📜️script.ts dev playground raster react prototype`. It grants the exact requested playground route and delegates serving to the existing OS dev owner. Its 38 focused assertions cover the closed request, live authority, and rejection of variant substitution. The same route publishes the missing catalog and Raster session through their existing owners. The Raster session marker is now present. Activation has reached the browser support prerequisite. Preview2 shim publication is still pending.

The narrow WASI check type-checked Raster’s model. Its diff crate then required an owned colorspace enum receiver for the newly connected mutation reader. The added receiver uses the existing ordinals and labels. A later retry and the first admitted full descriptor build encountered the concurrent ImageProps field update. The shared field catalog now includes the exact optional source byte count of the image projection. Both Image wire fixtures passed, and the descriptor retry reached native compilation with 441 reused units.

The graph-free owner now provides the missing Preview 2 shim, shard worker, and font support through their established publishers. The native artifact planner projects the same typed invocation into its compiler-store request. The live route reached the Raster component availability check after that correction. It then entered source admission for the required Surface renderer engine through the existing WasmScript. The renderer delivery and route laws passed 61 assertions. I cancelled this task’s redundant graph-based launch after the graph-free path entered actual native engine work.

The direct launch produced Surface and Editor WebAssembly. NodeGraph first stopped on a concurrent source edit. The launcher now retries source drift through the existing owner and uses published boot artifacts when available. I stopped the direct launcher when its boot preparation started a duplicate Raster component build, leaving the older descriptor build and its compiled dependencies intact.

The canonical build reached the Raster binary mutation crate. Its conversion state is exclusive and contains send-only retirement owners. Both state derives now select the framework’s send-only retirement path. The next invocation stopped in Cargo’s unit graph because the Hub manifests and locked dependency state had diverged. The scoped repository Cargo update refreshed the Hub lock for the current local workspace edges. Cargo reported zero external package updates. The canonical component retry then admitted all 1,011 source units.

After the lock repair, one descriptor attempt saw the Pack schema change during compilation. The shared source already contained the required type annotation. The next attempt detected a replacement Cargo graph that exceeded its original plan and refused to publish from it. The root descriptor command now uses the repository’s build-level concurrent-edit retry. It releases the rejected plan, opens a new one with the same original invocation, and preserves cancellation. All 25 churn tests and seven component-stage tests passed. The canonical Raster build was restarted against the current workspace.

The next full run compiled the Raster binary receiver and reached the host and editor session. Flood fill now holds its source through the native image lease and accounts for the backing admission. Replay keeps the allocator’s typed identity. The editor configuration validator reports Work at the framework’s full width. The canonical WASI component build is running with those corrections. The Raster semantic suite passed 248 tests with 909 assertions, including the independent Sharp and SQLite checks. Browser acceptance remains pending.

## Editable archive bounds

The history decoder used a 1 MiB frame and operation ceiling inside the existing 4 MiB archive transport. A 512 × 512 Raster asset exceeds the old operation limit before record overhead. The history decoder now uses the archive’s existing byte ceiling for its file, frame and binary operation. Cumulative native history allocation remains 8 MiB. The archive encoder checks the same 4 MiB raw extent before publishing a download. The shell’s data URL stays within its 8 MiB host answer limit even at that boundary. The focused Sharp, SQLite and TypeScript archive tests passed 21 assertions.

The mounted picker companion cleared 1,040 source-admission units but was still compiling shared SPR dependencies when the browser scheduler froze under memory pressure. I stopped this ticket’s redundant host build after it acknowledged cancellation and lease release. Its compiled units remain reusable. The browser scheduler resumed. The mounted picker and browser workflow remain unverified.

Raster’s asset mutation now stores RGBA and ICC octets in the existing packed bytes representation. Its owned receiver moves the paged bytes into a bounded conversion. This avoids allocating one generic value per image byte when reopening recorded edits. The shared image snapshot and diff shapes keep their existing layouts. The latest owning Raster suite passed 249 tests with 914 assertions across 21 files. The native compact-wire law and the Chromium workflow remain pending.

The full native build advanced past the earlier Raster host and session errors. It stopped in editor modes because the paint preview stored a bare `Arc<RasterSnapshot>` inside controlled retirement. The preview and its producer now retain the same backing as an `ArcLease`, preserving revision filtering and renderer address identity. The new native law exercises alias and final-owner retirement against the neutral fixture. The canonical component build has been restarted. The standalone NodeGraph task still entered the project inventory despite dependency inference being disabled. I cancelled that redundant request. The graph-free playground route already issues the exact invocation for NodeGraph after checking the component publication.

A focused downstream audit found the same stale image ownership in the wand selection producer. Its shared allocation now uses the native quote and exact Work, preserving the image for funded cancellation on denial. The independent Sharp fixture matched the expected flood coverage. The new native refusal and cleanup law is pending.

The browser probe watches the renderer’s imported-layer asset identity before taking PNG evidence after undo or redo. This synchronizes the pixel comparisons with a published edit instead of assuming that clicking the action finished it. Bun parsed the updated probe.

The next WASI run compiled Raster’s model, compact mutation codec, host, I/O, session, and modes. The editor commands then failed because the brush and wand imported the shared Work quote from the custody submodule, and the brush statechart macro generated references to the `machine` namespace. The quote now comes from the allocation owner and the brush supplies the macro’s expected name. The canonical process refreshed its local Cargo lock and entered another admitted plan after these fixes. No component has published yet.

One retry refused the shared DSL record model during source admission at 265,731 bytes against the 262,144-byte crate limit. A concurrent DSL update brought the same closure to 261,849 bytes. The repository’s actual reader confirmed 26 source files and 2,332 active lines for both WASI and macOS. No sources or policy were relaxed for this recovery. A fresh canonical build then cleared all 1,012 source-admission units and entered Cargo again.

The next attempt cleared the statechart and Work imports, then reported seven concrete editor-command diagnostics. Merge and flatten passed the allocation marker to a comparison that only reports copy and Work. Those calls now use the retained compare contract. The remaining brush diagnostics concern full-width Work receipts and the effect-backed authoring identity clone. No browser runtime has published yet.

Brush sample, comparison, and append receipts now use the framework’s full Work width. Continued strokes clone the same authoring seed under an original allocation effect and debit its measured Work once. The latest native browser component attempt uses those fixes.

After the brush changes, the component run retried on concurrent framework edits. Its retry had a stale dependency graph for the recently split media declaration crate. It refused three imports in reactor lifecycle and instance render. The live manifests and lockfile now contain both direct dependencies through an independent shared-workspace repair. The existing component process is reissuing Cargo planning against the fresh graph.

The replanned WASI build passed the shared media split and compiled Raster’s command crate. It reached the main editor and reported seven errors from older retirement, SDK export and job-factory interfaces. PNG codec errors now retain their original scalar/static details through a typed retirement cursor. Reserved import uses the SDK’s public app re-export. PNG export admits its existing job through the current construction witness and returns the original construction status. The next canonical attempt is running.

The subsequent native attempt caught an edited reactor input while verifying compiler evidence and retried. Cargo then refused a stale Hub lock during planning. The standalone offline update requested an exclusive registry lease held by unrelated active compiles, so I cancelled only that update. The canonical route has a per-workspace lock repair and is running again against the current sources.

The raw component unit-graph observer now classifies only Cargo’s stale-lock diagnostic as a build-level concurrent edit and retains the original cause. The enclosing retry can release the previous plan and use the existing per-workspace lock repair. The focused churn suite passed 38 tests with 130 assertions. The latest owning Raster semantic suite passed 249 tests and 914 assertions again.

The next standalone descriptor run found that repository reclamation had evicted the descriptor emitter product. The graph-free playground command already carries a new prerequisite repair through the actual descriptor emitter owner. I launched that route to restore the emitter, build the Raster component, and activate the browser. There is still no published Raster guest or loopback server.

Standalone descriptor generation now also prepares the descriptor emitter on a cold cache. Its focused six tests passed 89 assertions. The direct launch reached the real host emitter build and compiled 221 units. A shared replication source changed during compilation. The launch’s attempted retries then reused a closed parent invocation and exited. A new native launch has a fresh original invocation. The direct probe remains pending.

A second fresh launch refused a different concurrently edited reactor input. An executable fixture with the real Script invocation reproduced the accidental reuse of a closed owner. The root now places the finite build-churn retry outside settled fresh invocations for the exact React playground and component describe requests. Forwarded invocations keep their original authority. The existing native launch is still compiling and will be allowed to finish.

The next emitter build linked the actual native descriptor executable. It could not publish the artifact because its Pack JSON decoder source changed during provenance retention. The fresh-root retry now has executable coverage through original ownership, same request and new birth, forwarded custody, permanent failure, and interruption. Those tests and five focused launcher checks passed together with 333 assertions. I restarted the direct launch with that implementation.

The latest direct launch published the native descriptor emitter with its executable, artifact manifest and retained compilation evidence. It has advanced into the Raster browser component build in the same invocation. The guest and loopback browser endpoint are still pending.

The component build reached the main Raster editor again and cleared the previous retirement, reserved SDK and download-factory errors. Six further errors were fixed by matching the original command through its borrowed wrapper, importing the actual snapshot and preparation traits, and settling PNG construction before advancing its decoder. There was no browser component publication from that attempt. The graph-free launch is running with the corrected editor.

The next WASI pass compiled the complete Raster editor. It stopped inside the Hub Raster composition because the dispatch macro references the canonical `semio_os_kernel` crate name. The composition now names the already declared workspace dependency the same way as the other plugin roots. It is the same package and adds no dependency.

The subsequent compiler pass cleared the kernel dependency and found that the same dispatch expansion also uses framework Value in normal builds. Raster previously declared that package only for tests. The composition now links the existing workspace package in normal builds, which keeps its existing lockfile package set.

The refreshed direct build reached counted compiler capacity before opening Cargo. The shared capacity gate was held, while the run had no CPU reservation or child compiler. Two other live builds held nine of ten physical CPU slots. The remaining slot was unused because this driver asked for more than one to start. Browser verification was still pending at this point.

Granular component compilation now requests the preferred count from the same measured host plan and can start with one available CPU lease. Grant validation uses that same range and retains the original memory budget. The focused test passed ten assertions, including a separate range oracle. I cancelled only the queued Raster invocation, confirmed it released its leases, and opened a fresh direct launch.

The fresh launch acquired the previously unused CPU slot and started Cargo on the real browser transpiler. The two other builds kept their own CPU slots. The browser module will use the current guest-described manifest. The tracked Raster descriptor pair is updated by the rooted components describe command, separately from the prototype cache.

Mounted PNG import and export now refuse fixed allocations above the stable 16 MiB turn ceiling. A smaller current grant still yields when the complete operation can fit in a later turn. Python zlib and CRC verified a valid 2,049-square PNG small enough for the shell picker whose exact RGBA allocation cannot fit, and a separate valid small PNG case. The new native mounted refusal law is awaiting execution. The editable archive has its own 4 MiB limit on the full snapshot and recorded edits, so the total history also determines whether an archive fits.

The browser transpiler published successfully in the fresh run. The rooted playground has entered the real component closure again. The Chromium probe now checks that the imported color reaches the downloaded composite as well as checking paint, Undo, Redo and archive reopen.

The 21 exact files from the Raster TypeScript owner passed again after the mounted capacity changes. Bun reported 249 tests, 914 assertions and zero failures. The current WASI build uses the real semio-hub-raster-component target.

The current WASI pass reached the shared plugin instance commands. Cargo reported two uses of the canonical action-kind type without an import in staged action dispatch. The guest has not published from this pass.

The staged action dispatch now imports the same semio_framework::ActionKind type it already used by its full name. The failing Cargo pass continued compiling independent prerequisites for reuse on the next pass.

The rooted component driver automatically started a Cargo retry after the action-kind correction. The new pass had reused 622 compiled units at its first reported milestone.

The Cargo retry cleared the shared dispatch error, compiled the Raster host and editor, and reached compilation of semio-hub-raster with the corrected dependency declarations. Guest publication and Chromium execution remained pending at this milestone.

The Hub Raster library built and Cargo moved to semio-hub-raster-component. During the extended final compilation, a process sample showed a worker actively running LLVM ThinLTO optimization. No guest publication or listening preview existed at the time of the sample. The temporary sample file was removed.
