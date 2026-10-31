STATUS: BLOCKED (no browser proof yet). `activate-puzzle3d-react-prototype` (bazel, driver C/1030-tools/smoke/act.sh) failed 3 times, no server was started, no pid to kill.

- Attempt 1 (09:08): source admission ENOENT on `borrower/📦️metadata` (peer mid-edit; gone by 09:21).
- Attempt 2 (09:29-11:45): reached 22/25, then `puzzle-plugin:wasm:prototype` hit ENOSPC on the rust image (transient, 20 GB free afterwards) and `os-plugin-describe-rs:build:prototype` exited 1.
- Attempt 3 (11:46-15:19, ~90 min of it in the capacity queue): reached 19/25, then `os-plugin-describe-rs:build:prototype` exit 1 again. This is now the only failed action, so it is a real fault, not ENOSPC. Cause not extracted yet: the full tail is in `C/🗑️generated/1030/smoke-activate.txt` (ERROR at line ~12192) and the saved bazel output under `.cache/b/p/m/bazel/workspaces/s1/`. The log is full of "crate::ValueError" unqualify suggestions, which are warnings or lint noise, not necessarily the error.
- FOR coordinator or the lane owning os-plugin-describe-rs: find the real error in that saved output. It blocks the activation of every lane.
- Probe ready, not run: `C/1030-tools/smoke/probe.ts`. It checks shell ready, no shell error, console clean, add object, undo, and a second add. Use `PROBE_BASE_URL` (default http://127.0.0.1:6013/?plugin=puzzle3d, serve port unconfirmed) and `PROBE_OUT`.
- Next on "resume": read the describe-rs error, fix or hand off, rerun act.sh (bazel caches the finished actions), then `serve-puzzle3d-react-prototype` with nohup, then the probe.
