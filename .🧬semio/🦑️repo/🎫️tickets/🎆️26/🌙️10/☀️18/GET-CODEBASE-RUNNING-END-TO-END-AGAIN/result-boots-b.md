# result-boots-b (executor boots-b, wave 1023b)
Tools `C/boots-b-tools/`, logs `C/🗑️generated/boots-b/`.

## Status (newest first)
- 19:20 forms activation: capacity wait done (~13 min), graph rebuilt, now queued 4 of 5 for the exclusive Bazel slot-picker lease behind pid 83278 (4300 s old request). Waiting.
- 18:47 forms activation launched (pid 99985, `activate.sh forms`, log `activate-forms.log/.out`).

## Targets (current names)
Project `@semio-tech/framework-os-dev`: `activate-<variant>-react-prototype`, `serve-<variant>-react-prototype` (warm, depends on `@semio-tech/plugin-registry:session-<variant>`), `dev-<variant>-react-prototype` (depends on activate). Variants and ports from `playground` rows in the Cargo manifests: puzzle3d react 6013, forms 6058, note 6080.

## Table
| plugin | activate | serve | probe |
|---|---|---|---|
| forms | running | - | - |
| note | - | - | - |
| puzzle3d | - | - | - |

## Blocker (19:50, for the coordinator)
Bazel is unusable for ~hours: all 3 Bazel server slots are held by the long play activation (pid 82896, 7.9 h), the demo activation (83234, 7.9 h) and 70621 (4.9 h); `@semio-tech/print:test` (pid 83278, `print-b/ptest5.raw`) holds the exclusive `bazel-slot-picker` lease for 80+ min while it waits for a free slot ("all 3 Bazel server slots are in use; waiting for one (6100 s)"), so every other Bazel run queues behind it (5 waiters). Product defect candidate: the picker lease is held across the slot wait (should queue by slot, not hold the picker). I did not kill anything of anyone else's. My own queued Bazel activation (pids 99985/99989/99990/25178) was cancelled at 19:52; I proceed with `--tool=nx` (allowed by the brief's command list) with the same targets.
