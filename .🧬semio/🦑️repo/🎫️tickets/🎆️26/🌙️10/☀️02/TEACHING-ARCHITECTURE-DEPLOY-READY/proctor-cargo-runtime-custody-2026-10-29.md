# Cargo Server Runtime Custody — 2026-10-29

Read-only inspection while retaining actual server41024. No Source changes, new Native/compiler job, lease release or service stop occurred.

## Actual Retention

Canonical repo MCP leases_status observed PID1053085 holding semio.compiler.cpu/v1/0 exclusively and Cargo custody for release under unit-cache lane p77031d/b. Its build/store profile leases for p77031d/b and p77031d/t are shared. No matching CPU or custody waiter was reported in that timed observation. Registered Node reports12available host cores. This is a real retained compiler reservation during service runtime, not evidence the server is still compiling.

The launcher explains the lifetime: library/🏃️process/🦀️cargo/🚪️launcher/🟦️.ts183 awaits runOwnedCommand cargo until the command exits. For run -- ... serve, Cargo waits on its server child. Lines195–211 surround that execute call with acquired Cargo build custody and withCompilerCapacityConsumptionV1, releasing only after execute returns. Consumption/🟦️.ts4 correctly releases CPU before its later consume callback, but the Cargo server never reaches that callback while serving. Lane ownership is likewise retained until body settlement (launcher101–107).

## Qualified Build Interaction

Proctor product owner product/🟦️.ts384–418 invokes the SAME issued shared Cargo launcher with build --locked --package teaching-proctor --bin proctor and mode from actual Original. It is not a rawbinary bypass. The managed course dev target explicitly depends on @teaching/proctor:build, and its later runDevStack420–428 reuses a ready8791server. That runtime reuse does not skip the prerequisite build.

A release compile is not forced onto the server lane: unit-cache lane pool/🟦️.ts29–34 considers only unclaimed lanes for warm/LRU reuse, creates another lane below the bound, and waits only when every allowed live lane is occupied. Authored bounds are16ordinary/32live lanes. A live claimed p77031d cannot be selected as a free warm lane. Cargo custody resource includes actual buildDirectory and profile (native-build/lease/custody19–20); acquireCargoBuildLease65 takes that exact key. Another lane therefore has distinct custody, even in release; prototype also has a distinct profile.

Compiler CPU admission is counted rather than whole-host: qualified build leaves parallelism planned, launcher197 requests compileUnitsForHostV1.request. Units72 creates kind:available with maximum/minimum; current policy plans at most50percent of host cores and starts at50percent of that (12cores gives at most6preferred/3minimum before stricter memory/pressure limits). Capacity lease76 attempts each numbered core nonwaiting, skips a busy core, and accepts an available request at minimum77; it does not wait on CPU0 alone while ignoring others. The server retains1of12slots. That alone does not prevent this supported build from admission on the other slots.

## Boundary

The retained reservation reduces available compiler capacity byone and retains its lane/storage until server exit. It can contribute to genuine saturation (all admissible cores or32live lanes occupied), and exclusive whole-host/maintenance operations can legitimately wait. The timed registry and static contracts do not promise future CPU/memory/lane availability or a successful qualified build. They do establish no unavoidable same-release-lane or CPU0 blockage caused solely by this server. Preserve ordinary governor/custody admission; no special release, guard bypass, unowned stop or Source fix is justified for the current frontend prerequisite.

Generated cargo-runtime-held-leases.json contains only selected resource/PID/mode/state metadata, excluding argv/environment.
