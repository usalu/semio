# Private root build of the procedural component (2026-11-23)

Result: NOT staged by this lane. The private root was prepared and provisioned up to the dependency store, then stopped by resource blockers and repo tool bugs (below). The private root was removed. The live build (`🗑️generated/native-component-procedural-4.txt`) was left untouched and was still progressing (descriptor emitter, 4040+ compiler artifacts, about 10600 s). The live modules dir still holds only `raster`, `shard`, `vendor`. 6118 serves HTTP 200 (boot still refuses with plugin-registry-invalid); 6018 is down.

## Commands (scripts in this ticket)
1. Reclaim: `repo cache gc --dry-run` found only 1.1 MiB + 27 MiB (another collector held the store); free space 18 to 22 GiB, fluctuating with peers.
2. Watchdog (kills anything whose command line names the private root when free space is under 5 GiB): `private-root-watchdog.sh $HOME/.sinc/g3d`.
3. `env -u REPO_ROOT -u SEMIO_MACHINE_STATE_DIRECTORY -u CARGO_TARGET_DIR -u CARGO_BUILD_BUILD_DIR -u NX_WORKSPACE_ROOT repo incremental prepare --root=$HOME/.sinc/g3d --fresh --lang=en`. Clone of 172291 paths took about 3 min and 4.8 GB. Bootstrap passed preflight and tools setup, then failed in `dependencies sync --all`: the dependencies image is sized as 20 percent of free host space (3785 MiB), the install needs 6400 MiB.
4. Fix inside the private clone only: `partitions.dependencies` in `...budget/🔣️policy.json` set to a fixed 12 GiB (no `hostSharePercent`); detach and delete `.cache/b/d`; rerun `private-env.sh <live bun> $P/.../📜️script.ts repo --tool=nx dependencies sync --all`. First try stalled in the ownership walk (exit 124, 10 min without progress at load 25+); the retry completed (adopted from the fleet donor, about 13 GB image).
5. `tools setup all` failed twice in the native dependencies script: `Actual inherited control restriction required` from `recoverCompactCompilerHeadroomV1` -> `requireCompilerWriterCustodyV1` (repo tool bug on the headroom recovery path).
6. `run semio-hub-procedural-component:component -c prototype --exclude-dependencies` inside P: the graph service could not start (`Native storage capacity exhausted` in the compiler image reader). The rust image was elastic again (4 GiB = 66 percent of free host space vs a 34 GiB minimum); my fixed 16 GiB policy edit was not honoured by that path.

## Isolation checks
`private-env.sh` unsets the live `REPO_ROOT`, machine state dir, cargo dirs, `NX_*`, `SEMIO_CALLER*`; sets `SEMIO_WORKSPACE_ROOT=$P`, `SEMIO_BUILD_SERVICE=off`, `BUN_RUNTIME_TRANSPILER_CACHE_PATH=0`. Own images were mounted below P only (`mount | grep .sinc/g3d`), all detached with `hdiutil detach` before `rm -rf $HOME/.sinc/g3d`. The live bun binary was only executed, nothing was written to the live `.cache`. The post-build dep-info and `-links +1` checks were not reached (no build).

## Why stopped
- Disk: 12 to 22 GiB free, moving by 5 GiB within minutes from peers; the earlier WGPU wasm target alone was 29 GB. A private rust image large enough cannot be guaranteed above the 5 GiB floor.
- Memory: the machine-wide capacity gate queued each private command behind the live builds.
- Repo tool: the headroom-recovery control bug and the elastic rust sizing need a source fix first.

## How to restart
- Private route: free at least 40 GiB (live build finished or stopped), fix the two repo tool issues, repeat steps 3 to 6 with `private-env.sh`.
- Live route (preferred while it progresses): watch `ls .cache/b/p/m/browser/prototype/modules` for `🌀️procedural`; then `bun wgpu-native-activate.ts` (6118 screen `g3dwgpu` is running), `repo run @semio-tech/framework-os-dev:activate-generation3d-react-prototype --exclude-dependencies`, `screen -dmS g3dreact "$TF/serve-generation3d-native.sh"` for 6018, then `wgpu-native-smoke.mjs` and a Playwright load of `http://127.0.0.1:6018/?plugin=generation3d`.

## Files
`private-root-watchdog.sh`, `private-env.sh`, `🗑️generated/private-*.txt` (logs).
