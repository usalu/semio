# FEM and Procedural Release, 2026-11-26

## State
- `native-queue.sh procedural` (JOBS=1) started 05:1x; log `🗑️generated/queue-procedural-1126.log`. It is waiting for the exclusive product lane q53457c, held by another agent's queue (puzzle playbook stdio sourcing energy cad gis flow). No compile started yet.
- `release-receipts.py status`: 12 of 33 confirmed; `fem` and `procedural` are staged but unconfirmed.

## Descriptor emitter graph (no actor-input host)
- Source already moved the concrete actor input behind the reactor context and the wire ABI declares the job decoder and Pack JSON receiver (`🔬️descriptor-emitter-host-dependencies.md`). The neutral ownership test exists and passed; pinned offline Cargo selected 599 host units with zero concrete actor-input units.
- The 19:03 Nov 25 failure in `browser-procedural.log` (actor_input 1,759 MB over 1,536 MB) predates that change. A repo-managed `cargo tree -i semio-plugin-actor-input` was started; plain cargo is refused by the compiler ingress law. The first lane run is the real proof.

## FEM
- The four mounted apps are already separate units in source: `🌎️hub/🧩️compositions/🏗️fem/🗃️apps/{◻️2d,🧊️3d}/{✏️editor,👁️viewer}` (each a `semio-hub-fem-*-app` crate, registered in `🌎️hub/Cargo.toml`), delegated from the thin `semio_hub_fem_router`.
- `verify-fem-app-carve.py` run from the repo root passes ("four independently owned app units"). The 1,871 MiB library peak predates this split; the later FEM failure (2D session registry 32-bit shell credit overflow) is also fixed in source (`🔬️fem-shell-credit.md`). No further source change needed before the lane frees; peak after the split is measured only by a real build.

## Generator
- `Flow Wasm exports and exact memory are required`: a correct runtime guard in `🌊️flow/🕸️wasm/🖥️host/🏃️runtime/🟨️.js`; the fix is the new standalone engine (15,884,064 bytes, seven exports), published, but it must be in the final Vite output.
- `scheduled start refused while the document loaded`: fixed in source (per-instance load gate, 32 DOM tests pass); browser replay pending.
- `plugin.completed.decode` (`⚛️reactor/🔄️turn/📥️receiving/📦️completed/⚠️failure`): raised for Json or Pack decode refusal of the completed input. Still OPEN: static reading cannot name the case-specific return tag; it needs the captured browser fault event (the failure already carries `format` and `original`). No speculative change made.

## plugin.completed.decode root cause (already fixed in source, awaiting guest rebuild)
- Cause (`🔬️completed-extension-frame.md`): the WIT completion protocol returns extension faults as Pack, but the guest's bounded fault receiver fed them to the JSON parser, so every Math/BREP fault surfaced as `plugin.completed.decode`. The browser fault writer also emitted non-schema integer fields.
- Fix in `⚛️reactor/🔄️turn/📥️receiving/📦️completed/🦀️.rs` (Nov 25 19:56): both completion outcomes now use the bounded Pack decoder; JSON stays only for the stored correlation request. Browser fault writer fixed with a 671-byte neutral fixture; TS suite red then green (29 assertions, SQLite and AJV cross-checks). Native tests exist in `🧪️tests/🦀️.rs` (Pack wire body, full fault zero-work paid close, parked future); their managed run was still pending.
- Chromium capture was not repeated: the only staged Procedural module (`modules/🌀️procedural`, 129,960,119-byte core, Nov 24) and the Nov 25 site both predate the fix, so a run would only re-show the diagnosed mismatch. The real return tag will be visible once the queued Procedural build restages the guest; then run `verify-generator-overlay.py <🗑️generated/f> <modules/🌀️procedural>`.

## Files changed
- This note only.
