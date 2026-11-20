# Layout dev verification, 20 November 2026

The working target is the Layout component in semio OS Dev on WGPU. Acceptance requires a browser opening the current build, an edit through the host, undo and redo, visible and exported output, and a saved document opened in a separate owner. The browser contract and launcher are retained in the 16 September Layout Plugin End to End ticket.

## TypeScript verification

The direct behavior suite passed 36 tests. The first normal `repo run @semio-tech/layout-js:test` attempt exited 1 before executing tests. Project graph planning retried three times and reported the same missing SQLite drawing member JSON. Its original caller had already been replaced by the active page-canvas test, which exercises two shared fixture cases and eight refusals against SQLite. The surviving test now also checks reference-only membership and that the parent's relational projection contains only Layout tables. Removed the superseded test and its duplicate SQLite oracle. Verification of the updated source and normal target is in progress.

The updated page Drawing test passed using the registered Bun runtime and the independent SQLite engine (1 selected test).

The complete 16-file behavior selection caught six stale media-export source assertions. The production exporter now forwards a construction account and pays for body creation through it. The contract now checks exact account forwarding, copy and capacity charges, and settlement before downcast. Three negative fixtures independently perturb the account, charge and settlement. All 22 export contract tests pass. The package’s strict TypeScript selection also passed.

The final direct suite passed all 80 tests across the 16 selected files (333 Bun expectations). The native verification routes previously selected the repository root Cargo.toml and named the umbrella Layout package even for tests mounted only in companion crates. The updated routes use the repository admission API with the Layout package directory and the owning editor-command or frame-mutation companions.

## Huge-sheet canvas

Interactive rendering now checks borrowed frame bounds, including parent-page overrides, before sorting or cloning frames. Only in-view, selected, or hovered frames enter the paint list. Selection membership uses one index per paint. Full-document export still resolves the entire page. A shared case exercises inherited override geometry and verifies the original frame indices in paint order. The independent Lodash fixture suite passed all 14 assertions. Native compilation is in progress.

## PDF dependency

The Layout WASI component imports the production PDF editor through the hub. Its PDF coordinate code was mounted inside a crate named stdio-pdf-tests-2, which reached 4,064 active source lines against its 4,000-line ceiling. Extracted the PDF page viewport, crop and rotation mapping, and affine geometry into a separately compiled, dependency-free PDF coordinate crate. The same editor entry points call those functions. The next WASI check passed source admission and entered rustc. Shared numerical cases cover positive and negative 90-degree rotation, non-default physical units, reversed media coordinates and a crop box.

The native coordinate crate passed its shared-case Rust test through repo Cargo. PDF.js independently parsed actual PDF documents from the same fixture and matched both transformed points and viewport bounds (4 assertions, 0 failures).

The registered PDF TypeScript suite now includes the coordinate oracle. Two additional cases exposed a disjoint or zero-area crop producing a blank page. The page viewport now falls back to the full media box when the intersection has zero area. PDF.js loaded all four fixture documents and matched the eight expected point and bounds values. All 62 PDF package tests across the 14 registered files passed, with 1,858 assertions. Rust also passed the expanded shared-case corpus through the admitted Cargo test.

## Managed Wasm tools

Setup found the pinned wasm-bindgen executable, but the command registry searched only PATH and the user Cargo bin. The managed Cargo partition supplies both wasm-bindgen and wasm-pack. Binaryen uses its version, host and full digest below the tools partition. The registry now resolves all three managed installation paths. The new fixture failed before the repair and the registry and resolution suites passed all 43 tests afterward, including nine path resolutions across Linux, Windows and macOS. The actual registered commands printed wasm-bindgen 0.2.129, wasm-pack 0.15.0 and wasm-opt 130.

## Drawing component compilation

The production WASI build compiled through the plugin reactor and failed with 68 diagnostics in Drawing model code. Registered Drawing source lookup, SQLite IO, gradient sampling, document disposal, and validation were among the affected owners. The Drawing validator now receives the caller’s actual allocation effect through the framework child-admission port and narrows it for ancestry allocation, release and source bindings. Cold document disposal releases nested group layers iteratively. The neutral Drawing admission oracle passes. Recompilation and the installed browser journey remain outstanding.

Drawing’s retained multi-target drag now forwards its work quota, allocation marker and typed replies while preparing sparse transforms and diagnostics. Cancellation returns or retires the original native field and row-vector owners through the selected close operation. The command keeps its original frame-visibility, lock and ancestry checks. The changed Rust sources parsed successfully. Compiler and native behavioral verification remain pending.

## Layout retained apply and Drawing work quotes

The central Layout preflight, commit, denial and close paths now accept the current effect and reply contracts. Page, story, and nested group owners forward the original close loan for displaced values, and the portable diff tests cover the same RFC 6902 witnesses as the native tests. The Layout TypeScript suite passed 36 tests. The independent final graph suite passed 20 cases and 220 cancellation cuts. The page-cache oracle passed seven cases with 82,038 denied-grant checks. Native Layout compilation remains pending.

The embedded child-admission port quotes the Drawing cursor’s actual work, including a paged ancestry reservation. A caller with too little work cannot enter the child. The original vector-preparation path propagates that quote, records the cursor’s allocated bytes, and only claims the Drawing source after its binding is funded.

## Live canvas and story edits

The live Layout canvas now resolves visible Drawing members from the actual ArtifactView and passes the admitted typed source through the scoped composition effect. The export and media paths respect both phases of publishing the original parent read before drawing preparation starts. The original snapshot remains the source of member identity even when the canvas displays a transform preview.

The editor already emitted EditStory, while its retained owner declined to resolve that command through the numeric simple-mutation path. EditStory now selects the story planner, prepares Unicode scalar splices and UTF-8 style intervals, and records the old span and style runs for undo through the normal aggregate. The 39-case Layout behavior suite passed including three new RFC 6902 oracle fixtures for mixed-width replacement, formatting at EOF, and unstyled removal. The focused strict TypeScript check passed. The native aggregate test is registered and awaits a current Rust build.

The first Layout diff Cargo request reached repository source admission and refused 264,579 bytes against the 262,144-byte bound. Changing the module declaration alone still left 275,949 bytes assigned to the original package. Moved the native page cache and final-list planner files into the existing diff planning crate, preserving the same fixture inputs. That reduced the next reported boundary to 265,526 bytes before the scanner stopped. Moving generic sparse collection staging and borrowed keyed rows into the planning crate removed another 17,030 bytes from the concrete diff owner. The fresh focused WASI check under a 2,304 MiB inherited grant passed all 616 source admission units and reached Cargo. The focused Layout diff WASI check then completed successfully. The focused mutations check passed source admission and type-checked its dependencies, including the extracted typed field owners. It reached the outer mutation package and reported five uses of the same out-of-scope local demand variable. The installed host still needs full component and renderer checks.

The focused Drawing WASI retry verified the Drawing model crate and stopped in Drawing Geometry. The workspace’s typed Arc retirement no longer supports the former recursive type. Drawing’s Geometry owner now uses the same typed lease pattern as the raster payload. A fresh WASI check is queued, alongside the retained activation and renderer build. The installed host run remains open.

## Import and export publication

Reserved import, media output, and interactive export now retire their publication payloads and typed result owners using the live job effect. The old grant-only snapshot box entry points no longer exist in the framework exports. The refactor keeps a delivered publication in custody until its writer and payload are terminal and the next close phase can be funded. Edited Rust sources parse. The native export and archive round-trip still depend on the guest build.

## Build queue and host evidence

The supported 2,304 MiB inherited capacity command admitted the focused checks and reached repository source admission. The original full guest and renderer requests spent over an hour waiting for memory and never started the compiler on those handles. They were cancelled selectively so focused work could proceed. The canonical activation request remained live. The raw renderer has been requeued independently. Remote execution status reports no configured scheduler for this workspace.

The loopback URL `http://127.0.0.1:6179/?plugin=layout&example=demo` refused the connection during verification. The current checkout has no published activation receipt for this run. This ticket remains open through a real browser session, native artifact verification, and archive reopening.

## WGPU renderer

The renderer build passed the fixed wasm32 focus-routing overflow. The next real Rust compiler frontier exposed changed scene retirement, Flow custody, and DAG viewport APIs in the renderer registry. Packet close now polls the original scene effect. A denied turn leaves the packet in place. Frame queue backing birth, release, and unlink each have a funded step. The updated opaque-scene native law drives one real fixture caller from page preparation through packet and backing release. Renderer retry and native law execution remain pending.

The full Layout WASI check stopped upstream at a source-generation skew in VCS genesis. The current producer exposes the exact type and verified constructor called by the current store readers. The fresh component check is queued against these current sources.

The renderer's shared Canvas scene close now settles one actual Work unit before it removes a local command, transfers a child into a retirement owner, or releases backing. Delegated owners continue to settle their own Work. A native regression supplies full item and byte authority with zero Work and asserts the command stays attached, then checks that each admitted local step's Work matches the original receipt. The edited sources parse. This regression and the installed renderer still need compilation and execution.

The fresh Layout WASI guest passed its memory and source gates and is checking Rust. The new renderer build is also compiling Rust. Scoped generic registry generation reached an owner-only graph slice using the Semio build backend. It is waiting for the graph slice's 2,048 MiB host grant. The canonical build is still attached to the shared project graph, which has no published full generation.

The WGPU browser, embedded renderer and worker bundle producers now compile current source and pass the independent esbuild boot law. The exact source closure drives their cache inputs. See [WGPU browser build](wgpu-browser-build.md).

The fresh integrated guest checked the Store reader and Drawing model after the previous source skew and proceeded to its PDF and DWG dependencies. The renderer's previous link progressed to asset decoding. The PNG decoder now receives the original physical allocation and Work effect, keeps a denied transfer in place, restores invalid input to its caller, and closes typed custody before publishing the decoded image. Sharp independently matched the fixture's RGBA and invalid signature behavior. The updated raw renderer is queued.

The restored Layout archive registration shares the actual Drawing member catalogue across editor and viewer. The framework maintenance ladder leaves the exact active archive replacement in charge of its member roster. Native proof covers root, linked media, Drawing, a fresh JSON owner, and cancellation at the member handoff. See [Native engine verification](engine.md). The native cases await the build.

Retained CreatePage now has its atomic successor and exact inverse registered, with six neutral cases passing. See [Retained Page Creation](create-page.md).

The integrated WASI check reached the Layout mutation crate and reported one dangling documentation comment after test relocation. The stale line is removed. The fresh full component check is queued, with the renderer compiling under its own admitted grant. The pinned Rust parser also checked the new page and renderer sources without writing them.

The renderer boot tasks now declare their actual generated UI-axes producer. The redundant generic registry request was cancelled after its inventory pass so the scoped boot publication could enter the same checkout's graph queue. The Specific catalogue rendered 74 on-disk plugin entries including Layout.

CreatePage, DeletePage and SetParentPageContent are integrated into original retained mutation preparation. The current editor factory selects that issuer. The source registration law covered all 58 current command kinds. The most recent source seal refused 270,079 bytes for the concrete mutations crate against its existing 262,144-byte limit. The agent is moving an existing typed owner into the lower fields crate to preserve the boundary before retry.

Story mutation output and inverse now live in the existing typed field crate. The measured Layout WASI source closures are 258,274 bytes across 23 files for concrete mutations and 175,990 bytes across 15 files for Fields. The declared 262,144-byte source limit remains in place. The actual component retry is queued with the current retained editor factory and archive source.

The WASI guest passed the current source seal and entered its Rust build. The refreshed renderer progressed through the OS store and UI closure. Five original WGPU kernel laws passed with 201 assertions. The native SurfaceResizeJob units now exercise the current scalar turn implementation and the mounted-lane law continues through the full public callback. Its neutral SQLite geometry and receipt law passed with 19 assertions. The dynamic WGPU serve target uses its browser owner and the Specific composition path.

The original shared activation graph runner received SIGINT during selected source input acquisition. The scoped WGPU boot graph subsequently passed inventory and began admitting its 776 selected inputs. A separate scoped activation plan and browser support task are waiting for that checkout fact-store worker. The WASI and raw renderer retries stopped at the same original receiving API seam. The checked-out producer had already gained the exact borrowed refusal accessor after those source snapshots. Fresh authoritative Cargo checks are queued against that producer.

A separate current graph refused the BREP axial face preview crate because its native owner row was absent. The repository's scoped reconciler added its one authored owner row in `📋️build-owners.json`, without any removed rows. A second scoped read had zero findings and required no change. The fresh actual guest has entered Rust compilation.

The fresh WASI build checked the one-item publication crate through the previous borrowed-refusal failure and continued through Store initialization, grouping and engine dependencies. The renderer is compiling its fresh source snapshot. The browser boot graph automatically refreshed after a shared project-node source changed and made progress on its new scoped pass.

## Current WGPU Launch Checkpoint

All three WGPU JavaScript boot targets completed production publication. The prototype browser support receipt lists the shard worker and Preview2 vendor tree, and both are present on disk. A later duplicate support attempt returned exit 1. The owner-only Layout activation dry plan resolved the exact dynamic WGPU target. Fresh authoritative WASI and renderer builds are running against the current checkout after both previous attempts reported API mismatches in concurrently updated source. The Layout session catalogue and font tasks are running. Port 6179 refused the live probe before publication.

The prototype support receipt was rechecked with the repository's digest verifier. It returned valid with zero findings. The canonical browser boot task executed all three owned targets successfully. The current guest uses the repository's real Cargo ingress and the existing current renderer target is queued behind the current checkout fact-store build. The published descriptor emitter is present in the browser tool store.

The font owner published its prototype pack through the normal compiler and publication scripts under the repository's workspace Script bootstrap. The product manifest is owned by `infinite:fonts` and the pack is 8,757,072 bytes. The redundant queued graph client was cancelled after publication. The Specific dev command also published its 12,660-byte standard format claims. The actual descriptor channel check reported 75 committed descriptors at pin 25 and zero stale. The checked-in Layout JSON and Pack still need semantic agreement from the freshly linked guest.

The owned font bytes have SHA-256 `05c4bbb7d07a3ee0c77274d546a3a4c5942366ccbc82eedad18b4885aa21fc5a`. The direct renderer owner verified the pinned bindgen and Binaryen tools and is admitting its declared Rust sources under the standard root Script bootstrap. The standard component describe owner passed all 946 current sources and is compiling the linked browser guest. The browser journey accepts the complete dock window identifier after the stack key. Static selectors were compared to the current registrations and Node syntax validation passed.

The first fully admitted prototype Layout link compiled 1,020 units and exposed the first downstream production errors. The Layout engine's Drawing composition, paragraph input and numeric scene cleanup now follow their current original effects. The Layout host diagnostics reduce to the missing typed binary operation receivers for Layout and Drawing mutations. The live PDF declarations, dependency and new module matched their callers after the previous compiler snapshot was taken. The renderer entered actual Cargo after a bounded repository source-change retry.
