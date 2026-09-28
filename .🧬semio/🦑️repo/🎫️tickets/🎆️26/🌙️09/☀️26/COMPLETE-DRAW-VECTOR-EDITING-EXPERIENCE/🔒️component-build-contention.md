# Component Build Lock Contention

The Draw materialization handle 43904 was confirmed live repeatedly. Inspection found its cargo process 44003 and the Raster cargo process 46718 with no descendants, no active wasm-dev rustc processes, and 117 shared open crate lock files. One-second process samples show all compiler work threads blocked in Cargo prebuild_lock_exclusive / LockManager::lock / flock (five Draw stacks, two Raster stacks). These observations establish lock contention with no active compiler capable of releasing the work, rather than an elapsed-time timeout.

The Draw cargo command is owned by this chat’s describe/materialize run. Only process 44003 is being terminated to release that run’s locks. No other chat’s process or checkout is modified. Its existing tool handle must be polled to a terminal result. The other component build must be observed advancing before Draw materialization is reissued; do not immediately recreate the lock contention.

Raw samples remain under the ticket generated folder until ticket cleanup.

## Observed Recovery

Terminating only Draw cargo 44003 made handle 43904 exit 130. Immediately afterwards Raster cargo 46718 spawned rustc 98729 at 66.6% CPU; before the cancellation it had no children and only blocked flock workers. The Raster process subsequently disappeared from the authoritative process listing. Draw describe/materialize was then reissued, with its log at `🗑️generated/build-draw-lock-recovered.txt`. This recovery was based on sampled blocking stacks and resumed compiler activity, not on elapsed duration alone.

## Compilation After Recovery

Recovered materialization 67402 reached compiler diagnostics and ended with code 130 after a genuine compile failure (not another manual cancellation): the generic Vec value editor’s InsertAt rejection returned a Result inside a match whose other arms produce unit. The rejection now explicitly returns its error in `🧰️framework/🔨️modules/🌱️value/🔁️codec/🦀️.rs`. Only that failing arm changed; the other expression-valued matches retain their existing form. Fresh materialization 58527 is live with log `🗑️generated/build-draw-value-edit-fixed.txt`. It must be polled to completion before any rebuilt-browser claim.


Native renderer run 40003 terminated with three compilation errors in shared shell code. The icon-override test was already corrected by concurrent work when re-read. This task fixed the remaining `sync_dock` borrow conflict by capturing the immutable owner identity before retiring mutable surface documents; renderer validation continues in run 99880. Draw materialization run 58527 remains live after successfully completing component-dev and while building the describe host.


Build 58527 completed successfully (describe and materialize-dev). The serve implementation explicitly retains an existing activation receipt and only reports stale staged modules; materializing alone does not refresh that runtime. Therefore browser verification must run `@semio-tech/framework-os-dev:activate-draw-react-dev` after build 79374 succeeds, then reload. Old preview 33503 was stopped deliberately with Ctrl-C to free the shared Nx serve target. First stable-preview request 64642 merely joined that old target and exited; replacement 76268 is live on port 6065 with SEMIO_VITE_HMR=0. Its local hub startup is still pending and no listener was yet observed.


Renderer run 99880 terminated after 10m58s in semio-framework-ui: `stamp_tree_selected_item` passed `Vec<UiTreeSectionNode>` to a function accepting `UiTreeItemNode` slices (engine line 1661). The current source still contained the error. Corrected it to visit each section's `items`; the test target is rerunning. No stale renderer-cache errors from the earlier viewport migration were changed.
