# WGPU27 Paired Browser Verification

## Artifact

The completed WASM27 build published renderer JS SHA-256 `daf1302c5545a2e158f0bf6e9b1a2a915de8c73dae8ec64bec71403eda6ee8fb` and 135939338-byte WASM SHA-256 `023df39f8cb317b14cd294d189f2ffaea6b73f02f9d119061ca98c3e97d8f0be`. All other twelve-file manifest entries match WGPU23. Manifest: generated/astra-runtime/gate34/paired-27-artifact-seal.json.

Server 27b is listening on 6014 with PID34168. The earlier restart waiting process exited after the previous Nx owner stopped; the fresh owned server then started normally. A stale browser error tab refused further navigation, so verification uses new IAB tab7. React reference is tab4, server6013, refreshed during this run. Both screenshots are 1280x720.

## Observations

- Both renderers load Concrete Forest with Top and Perspective panes. Native assets arrive over subsequent frames; early smaller framing is not treated as final output.
- Expanded WGPU Projection contains every expected named AX row, but its visible text and icons remain covered by opaque bordered strips. Atomic publication alone did not solve this. Root identified the generic activatable Stack paint on the tree identity rows and added a focused fix plus actual paint-output regressions (see tree-occlusion35 report).
- Choosing Curvilinear through the native AX row updates the selected row and pane title to Curvilinear; the world visibly changes projection.
- After independently reloading React and selecting Top→Curvilinear, the settled camera framing still differs: React roof occupies most of the sphere, while native roof is much smaller and more of the floor plan is visible. Right Perspective pane framing is similar after assets finish. This remains an open camera transition parity defect; do not claim Curvilinear parity from isolated projection math tests. Sol is auditing initial orthographic camera distance/zoom transport.
- Browser console capture contains Vite connection/freshness info and no reported warning/error, but it did not expose worker diagnostics. HTML correctly declares `semio-runtime-diagnostics=1`. The WGPU server still reports HMR on despite `SEMIO_VITE_HMR=0`; its independent server config does not read that variable. No debug-log-based full feature success is claimed.

The WGPU server config now respects the existing `SEMIO_VITE_HMR=0` switch exactly as the React dev server does. This is a one-line configuration repair for stable paired verification; its applied runtime setting will be checked on the next server restart. Current server27b predates this edit.

## Camera Transition Cause and Source Repair

The shell still enqueued a synthetic zero-delta wheel after a local projection selection. That creates a camera-settle obligation, whose `setCamera` report deliberately carries zoom1 for perspective cameras. The guest echo is then eligible to replace the native orbit zoom, undoing the locally preserved orthographic zoom. React `handleProjectionKindChange` only stores its pending projection and syncs title/icon; it does not dispatch a camera gesture.

Removed the synthetic wheel. The existing native projection-chip law had explicitly required the incorrect settle; it now requires no synthetic camera obligation. This is pending native and physical verification. The React projection Tree test also had a stale assumption that leaves lack buttons; now it checks disclosure semantics via aria-expanded, allowing the newly accessible leaf actions. Focused oracle run pending.

## Projection Pane React Receipt

The focused projection-pane React suite passed both tests on 2026-09-27 at approximately 23:31 UTC (Vitest 14.73 seconds; Nx 18.9 seconds, exit 0). The assertions distinguish disclosure buttons by `aria-expanded`, preserving accessible leaf buttons. Native camera-transition verification remains pending. The execution log is under `🗑️generated/astra-runtime/gate34/projection-pane-react35.log`.
