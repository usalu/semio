# Verification

All runs were native on Windows. The commands set `--config 'build.rustc="rustc"' --config 'build.rustc-wrapper=""'` and
`RUST_MIN_STACK=134217728`, with separate target dirs. The scripts are in this folder.

| Check | Script | Result |
| --- | --- | --- |
| `semio-framework-tool-machine` with tests, type check | `check.sh` | rc=0 |
| `semio-plugin-instance-render`, `-typing-edit`, `-instance-typed-ops`, type check | `check.sh` | rc=0 |
| tool-machine lib laws, including the new `every_host_abort_during_a_retained_read_ends_its_slot_once_the_read_returns` (14 cases) | `law-test.sh` | 12 passed |
| companion ledger laws (gesture-drive, typing, scrub, …) | `law-test.sh` | 40 passed |
| plugin follow laws `semio-plugin-tests-01 follow`, including the new `a_host_abort_during_a_follow_read_ends_its_slot_once_the_follow_returns_the_read` | `law-test.sh` | 4 passed |
| FEM wasm with the change (`semio_hub_fem_component.wasm`, sha256 `849acedc…`) | `fem-smoke.sh wasm` | attempt 1, rc=0 (92 min) |
| FEM 2D browser smoke | `fem-smoke.sh materialize/serve/smoke` | rc=0: Converged 36 / 512, then 49 / 512 after the edit; Live Load Ty 12000 N |

Notes:

- The first plugin-law run failed to compile `semio-plugin-ownership`, because another session was editing it at the
  same moment. The re-run was green.
- The first smoke failed at boot with `original-turn.succession-unoffered`: the served shard worker was stale. The parent
  FEM session regenerated `.cache/b/p/m/browser/prototype/modules/🧵️shard/🟨️shard-worker.js`, and the smoke then passed
  here. The parent session independently confirmed 2D rc=0 and 3D `Case: dead` on the same wasm.
