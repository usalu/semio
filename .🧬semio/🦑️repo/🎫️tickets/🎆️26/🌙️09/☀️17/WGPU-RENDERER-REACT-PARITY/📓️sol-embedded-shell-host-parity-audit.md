# Embedded Shell Host Parity Audit

The unrestricted renderer parity request includes exposed embed/mount behavior. The ordinary single-app browser journey does not establish equivalence for multiple shells or iframe introduction policy.

React ShellHost exposes suppressAutoIntroduction. Its effect stops an existing automatic introduction when that flag becomes true; otherwise it omits automatic introduction in an iframe by checking window.self against window.top. The shared React predicate also receives the suppressed flag.

The WGPU Shell predicate explicitly omits suppressed under a one-shell-per-surface assumption. The renderer's page-mounted bootFrameworkOsWgpu API nevertheless advertises independently rooted multiple mounts and is used by the existing Os coordination story. It has no explicit introduction suppression option and does not propagate an iframe suppression context.

The WGPU page boot disposal removes the canvas and disposes plugin actors, but its own documentation states the underlying Wasm event loop continues rendering to the detached canvas. Descriptor, host appearance, locale, storage, dynamic extension and agent bridge hooks are module/global doors. Their lifetime and owner scope require concrete inspection before multiple mounts can be considered independent; merely adding one global suppression boolean would not establish that property.

Source references: engine/🧱️elements/🏛️ShellHost/🟦️.tsx at the suppressAutoIntroduction interface and automatic introduction effect; engine/🧱️elements/🐚️Shell/🟦️.tsx shouldAutoStartIntroduction; engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs should_auto_start_introduction; engine/🎯️targets/🧊️wgpu/🎬️renderer-boot/🟦️.ts; Os 📖️stories/🧭️coordination/🟦️.tsx.

Next evidence must establish the real mount/worker lifetime, carry host policy in an owner-specific contract, and add a neutral fixture plus React oracle and physical mounted acceptance. No iframe or multi-shell parity claim is made from this audit.

## Executed Lifecycle Repair

Inspection found no current semioWgpuMount export anywhere in the Rust renderer. The advertised public library boot therefore calls an entry point that no longer exists. A new language-neutral two-root lifecycle fixture and schema, an independent React createRoot/unmount DOM oracle and a Worker-bound public boot test executed through Nx: the schema and React oracle passed; WGPU failed with the exact missing semioWgpuMount error.

Both page and library doors now use one domain-owned browser-host module. Every mount owns its dedicated frame Worker, transport, input, media overlay, accessibility mirror and lifecycle. Library mirrors carry distinct mount namespaces, and page-global introspection/interactive-job bindings are installed only by the ordinary page door. Retiring one mount closes its transport and waits for the Worker retirement acknowledgement, removes its own DOM and retains the other mount. The former direct Wasm mount path and its best-effort detached-canvas lifetime are removed from the library door.

The focused lifecycle gate executed green: all three tests passed, including two independent owners, unique DOM ids, exact plugin descriptors, independent retirement and repeated disposal. This is mounted DOM and transport contract evidence with controlled Worker replies; actual Wasm/GPU multi-mount acceptance is still required. Page source guards now inspect the entry plus shared host, retaining the same input, readiness, media, platform, introspection and authority invariants. Both sorted generator source authorities own the new host. Fresh browser generation and complete browser regression are executing.

Further requirements remain: authored module URLs must reach the owned Worker, root remount ownership must be explicit, boot failure/retirement must settle reliably, and iframe/background introduction policy must be carried and enforced per owner. The existing Storybook delivery/resolution path still references the old Trunk/cache topology and needs canonical current-artifact validation. None of those are declared complete by the three-test lifecycle result.


## Complete Retirement and Replacement Laws

The first expanded lifecycle run did not reach assertions because the unexcluded Nx dependency graph encountered the concurrent Flow/IO compilation closure. It is recorded as a prerequisite failure, not a renderer test red. The isolated registered Nx browser target then executed seven selected lifecycle tests: three passed and four failed on the current implementation. Failures prove that repeated disposal can return before Worker retirement, replacement mounts do not retire their previous owner, and crashed or unreachable-close workers cannot settle their retirement.

The repair serializes ownership through a root-scoped weak registry and keeps one disposal promise per mount. Normal disposal still awaits the Worker’s own bounded close acknowledgment. An actual Worker error or failed close-channel write terminates the failed owner and settles retirement. The focused green run is in progress; no live GPU claim follows from this controlled Worker seam.

Eight language-neutral introduction vectors and a strict schema now describe fresh, seen, replay, suppressed, iframe, suppressed replay, tutorial and absent-introduction cases. The browser test compiles the actual authored React predicate through the existing TypeScript oracle and separately exercises first-frame and live owner-scoped policy carriage. The corresponding native neutral-vector test is authored against the current unsuppressed predicate for a real fail-first run when the concurrent IO source closure compiles. Native implementation is unchanged at this checkpoint.


## Native Fail-First and Browser Oracle Follow-Up

The expanded lifecycle gate passed all seven selected tests after its four recorded failures. The initial introduction browser run recorded the missing boot-carrier defect (`[undefined, undefined]` instead of `[true, false]`). Its first React source oracle attempt failed on a jsdom URL/read boundary; that is a test harness failure, not a React predicate failure. The test now imports the authored source through the runner's raw-source loader and retains the TypeScript compiler oracle.

The coherent native union executed seventeen laws: fifteen passed, including theme grammar/guidance, viewer capability, heap ownership, picker/boot, notice and panel journal repairs. Two newly added laws failed at assertions: Display publication after one disclosure interaction and the embedded introduction's suppressed vector. The introduction repair now includes the native host-policy setter/reader, the complete six-term arming predicate and live policy transitions before chrome publication. The transition clears an active tour without writing its device answer; bringing an unseen shell forward can offer it again. A second native law exercises those transitions and preserves an answered introduction. Native green and live mounted GPU behavior are still due.

The first module-registry integration pulled its kernel preflight dependency into the page's source closure. The pure admission boundary is being split into an adjacent module so the UI isolate carries only bounded input admission/schema, while descriptor and dependency planning remain in the Worker. The literal-source guard remains enforced. No unrelated actor fixture or graph guard was changed to accommodate this split.

## Current Native and Embedded Source Receipts

The coordinated native union passed all 34 selected laws (1,581 filtered), including the complete Display suite and both host-introduction laws. The accepted Display sequence now publishes the newly measured panel document before body/glass geometry; the recorded one-click disclosure failure is closed by that gate. Live GPU confirmation remains pending the fresh Wasm publication.

The public mount now exposes six owner-local introspection methods. They probe only that mount's transport and reject after retirement. The first API test failed because the old mount returned no introspection handle. A later integrated run encountered a transient missing pure-admission import; the restored import is preserved. The descendant-shard lifecycle law then failed because frame retirement left its shard alive. Retirement now closes shard MessagePorts and terminates descendant Workers one per event-loop turn before resolving the mount's disposal promise. Late shard creation after close is refused. The combined embedded/transport gate passed 79 of 79 laws after reconciling two obsolete source assertions; these are controlled Worker and DOM contract receipts, not physical GPU evidence.

The explicit plugin registry packet passed its final eight selected laws with independent graphlib dependency-ordering evidence. The page still owns its generated registry; the library carries its supplied, bounded registry, and descriptor planning stays inside the Worker.

A schema-first third browser entry now owns the public embedded library. The initial neutral three-entry oracle was red against the former two-entry parser. The canonical producer now emits the library through the existing browser-build script, and its uncached Nx generation passed. Browser profile/input/output authority, package router and check route include the new exclusive producer. Physical route and full integration checks are in progress.

Two fresh Wasm attempts failed at the shared IO importer call with incompatible database ownership expectations during concurrent source evolution. Those are compile prerequisite failures. The canonical erased import contract now borrows the validated database, matching its handwritten trait and preserving the caller's database for subsequent inspection. A fresh coordinated Wasm attempt is executing; no refreshed host publication is claimed yet.

Actual React shell acceptance completed the full 57-vector interaction journey in English and German, each recording 235 trusted inputs and zero untrusted inputs/errors. The physical widget extension additionally exposed a detached resize-corner callback after the first geometry change. The window agent is repairing the captured interaction lifetime and verifying the full intended movement in both languages. The retained report remains authoritative for the exact passing and pending widget rows.

## Delivered Library Routes and Shared IO Contract Boundary

The neutral three-entry identity law passed against the updated profile, with its independent strict schema/emoji oracle. The completed-artifact route fixture first failed for both profiles because the public library route was absent; the repaired four-law route suite passed. The uncached plugin-registry generator refreshed the canonical editor launch file successfully, including widget and embedded acceptance commands. A new root-artifact fixture/schema row declares the public library's exclusive producer, and WGPU preparation now requests that producer alongside page boot, frame Worker and Wasm.

Two subsequent Wasm attempts encountered the immutable database borrow reverted at the call site while a crate was compiling against the earlier borrowed codec. The live store interface and call site later converged on an owned database contract in the independent IO development lane. The renderer fleet will preserve that current coherent ownership contract rather than continue changing an unrelated domain decision. The caller-retains-after-import assumption is outside the renderer parity requirement and is being removed from the repair packet. Subsequent publication must compile the current interface and caller together; the retained failure logs remain prerequisite failures, not GPU receipts.

The React widget extension passed all fourteen rows across English and German after its two recorded runtime failures. The complete corner movement and both neighboring-window consequences now hold without browser errors. Embedded runtime separately exposed two production layout layers using viewport dimensions inside a smaller host. The ShellHost base correction is applied; the shared Layout container is under repair from the physical red. Both the embedded oracle and ordinary full-page journeys must be reverified after those container changes.

## Fresh Native Publication and Integration Gates

The next coherent owned-IO Wasm build succeeded uncached in 3 minutes 38 seconds. The canonical compiler published the binding module and Wasm file to wasm-dev. This is the fresh native source publication containing the repaired Display layout sequence, viewer admission and host-introduction policy; it is not yet a physical browser parity receipt.

All 28 package integration laws passed after adding the third public library entry. The root-artifact authority suite passed all five tests after its existing editor-route assertion was reconciled to the current generated launch entry (the obsolete seed-only route was removed from the assertion). Its independent Ajv, TypeScript, crypto and native Go checks remain intact. The owned IO neutral source/SQLite proof passed two tests with 27 assertions after preserving the independent IO lane's current ownership design.

The public boot lifecycle packet recorded five failures out of six initial laws and then passed six of six. It introduces scoped progress and AbortSignal cancellation while retaining complete Worker retirement. An additional malformed-progress refusal law and the full mount regression are executing. Both exact generator source authorities now include its neutral schema.

After the two production container repairs, the ordinary full-page React Dock gate passed all eight physical laws again. The 57-vector English React journey is executing against the same corrected source. Find/palette keyboard and portal scope remain a separate physical embedded failure under repair. Browser bundle publication, host refresh and actual fresh WGPU acceptance follow the source-ready marker.
