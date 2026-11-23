# Cargo Build Script Stall (compile-slot self-deadlock)

## Evidence
- `repo cargo ... check` (cargo PID 92989, parent bun 91821) idle 42 min at 0% CPU; 9 bun children (93046, 93049, 93050, 93059, 93061, 93062, 93155, 93201, 93215) in kqueue, no CPU.
- `<unit-cache>/s/0.lock` (the only compile slot, `SEMIO_UNIT_CACHE_SLOTS=1`) was held by live PID 93049, a build-script wrapper.
- `<unit-cache>/q/` held 9 waiting tickets (the wrapper PIDs above plus 93103); ticket 93103 was the one waiting.
- PID 93049's child 93093 is `proc-macro2/.../build_script_build.real` (the compiled build script); its child 93103 is the rustc wrapper (`script.ts rustc --cfg=procmacro2_build_probe ...`) started through `RUSTC_WRAPPER` (env of 93093: `RUSTC_WRAPPER=.../rustc-wrapper-*`, `SEMIO_UNIT_CACHE_SLOTS=1`).
- `sample`/`lsof`: 93049 waits on its child (unix sockets 6u/8u to 93093); 93103 waits in `acquireSlotV1`.

## Root cause
`runOffloadedV1` (buildscript wrapper) takes a compile slot (`acquireSlotV1`) around the local run of the real build script. Build scripts such as `proc-macro2` probe the compiler through `$RUSTC_WRAPPER`; that nested wrapper invocation inherits `SEMIO_UNIT_CACHE_SLOTS=1` and calls `acquireSlotV1` for a slot its own ancestor holds. With N slots and N concurrently running build scripts all slots are held by parents whose children wait for a slot: a lock-order inversion (parent holds, child needs) that never resolves because holders are alive (live holders never go stale). The grant is FIFO by ticket, so every other wrapper queued behind it, which looked like all compilers idle.

## Fix
`🏗️buildscript/🟦️.ts`: new `slotHeldEnvironmentV1` sets `SEMIO_UNIT_CACHE_SLOTS=0` in the environment of a build script that runs under a held slot; the nested probe is accounted to the script run's slot. The local run receives it via `capture(slotHeld)`.

## Regression test
`🏗️buildscript/🧪️tests/🔓️slot-held/🟦️.ts`: pure environment test, and a run of a script under the only slot whose child acquires a slot when `SEMIO_UNIT_CACHE_SLOTS>0` (stalls before the fix; bounded to 20 s). Run: `bun test ./<path>` -> 2 pass.

## Not covered
The "Lock order violated" lease entries (artifact/cargo-coordination while image shared) and `--parallelism=2` "Undeclared original Repo build selection" are separate issues, not touched here.

## Verification
Stalled PIDs 91821/92989 killed after evidence capture. `repo cargo --store=shared --configuration=prototype -- check --manifest-path <pdf>/Cargo.toml` result: see below.

Result: no stall. All build scripts (proc-macro2, serde, ...) ran and the check kept compiling units (330+ crates, continuous progress, no idle bun wrappers) for 10+ minutes in two runs. The PDF plugin graph is large (hundreds of workspace crates) and the host image was near full (2.7 GiB free), so one run reached my own 590 s timeout while still advancing; a second run was still progressing when this was written. A single small-crate completion was not observed.
