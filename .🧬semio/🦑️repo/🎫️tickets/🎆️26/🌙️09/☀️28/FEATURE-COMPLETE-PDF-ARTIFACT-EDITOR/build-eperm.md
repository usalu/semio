# Build EPERM (`MutationLeaf source authority failed: Operation not permitted (os error 1)`)

## Mechanism

`repo cargo` offloads individual rustc units to fleet workers (`build/remote-execution`, worker `👷️worker`). A worker runs each action in a private input root under `.cache/f/remote-execution/work/<worker>/<task>/r` inside the macOS sandbox profile of `🔒️sandbox` (`deny file-read-data` on `/Users`, `/Volumes`, `/private/var/folders`, ... except the input root). The input tree of an offloaded unit holds only the crate sources (`🚀️offload/🗺️paths`).

The dsl derive (`dsl/✨️derive/🔖️mutation-source-authority`) finds its workspace by walking the ancestors of the source file for `package.json` plus `📋️project.json`, then reads the taxonomy projection and descriptors. Inside the input root those files are not inputs, so the walk climbs out of the root into the real worktree (`/Users/<u>/Documents/semio`, an ancestor of the worker directory), and `fs::read(package.json)` is refused by the sandbox with EPERM. The macro's `compile_error!` has no rustc error code, so `classifyOffloadExitV1` takes it for the compiler's own answer and returns it instead of compiling the unit locally. It is intermittent because it only happens when the unit was offloaded to an idle worker. The `xcrun_db ... Operation not permitted` warnings in the same logs are the same sandbox (the linker of the proc-macro dylibs) and are harmless.

Not the cause: seatbelt read oracle (only the `inputs` CLI), `chflags uchg` leases (lease directories only), replica daemon atomic replace, TCC, xattrs.

## Fix

- Worker names the real private input root to the action: `SEMIO_OFFLOAD_INPUT_ROOT` (`REMOTE_EXECUTION_INPUT_ROOT_ENV_V1`, `👷️worker/🟦️.ts`).
- `native-build/📥️resources/🧮️compiler`: `input_root_v1`, `inside_input_root_v1`; the workspace walk stops at the input root (`take_while`).
- A failed source authority inside an offloaded input root expands to `extern crate semio_offload_inputs_incomplete;` (`E0463`, already in `INCOMPLETE_INPUT_CODES`), so the client compiles the unit locally and the real result (or real error) comes from the original worktree.

## Tests

- `👷️worker/🧪️tests/🟦️.ts`: the action receives its real input root.
- `compiler/🦀️.rs` `tests::nothing_above_the_offloaded_input_root_may_be_read`.
- `dsl-derive-expand` checks (source boundary 262144 bytes kept: logic lives in the compiler-resources crate).

Not verified end to end: a fresh offloaded run of `semio-plugin-interaction` (needs an idle worker and a rebuilt derive dylib).

## Verification

- Deterministic repro (`🗑️generated/eperm/repro.ts`, real worker sandbox profile, input root under the worktree): the old ancestor walk anchors at `/Users/ueli/Documents/semio` (the real worktree) and `cat package.json` fails with `Operation not permitted`.
- `repo cargo check -p semio-plugin-interaction --lib` rebuilt the derive and finished green with no EPERM (8 min), but the fleet took no unit, so every `[DEBUG]` walk line showed `root=None` (local compile). A real offloaded run was not observed.
- Client side: `🚀️offload/🧪️tests/🩺️failure` now covers the exact `E0463` diagnostic the macro provokes (`semio_offload_inputs_incomplete`) as `incomplete-inputs`, which `🚀️offload/🟦️.ts` turns into a local compile.
- Temporary `[DEBUG]` log removed.
