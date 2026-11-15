## Step 0 of Round 25: `repo tools setup all`

- Start 2026-11-15T08:10:29Z, end 2026-11-15T08:31:21Z, wall time 1252 s, exit code 0
- Log `~/tools-setup-round-25.log` (116 lines without `[script]` rows and `still running` lines)
- Memory pressure, samples 10 s apart: level 1 in 125

Lines with `restored` / `installed` (1):

```
Funding the dependency store: the dependencies of every scope are installed into it now. A first funding takes many minutes on a small machine; the lines below report each scope
```

Last 20 lines (without `[script]` rows and `still running` lines):

```
Computing main repo mapping: 
Computing main repo mapping: 
Computing main repo mapping: 
Loading: 
Loading: 0 packages loaded
Analyzing: target //:setup (1 packages loaded, 0 targets configured)
Analyzing: target //:setup (1 packages loaded, 0 targets configured)

INFO: Analyzed target //:setup (5 packages loaded, 7 targets configured).
INFO: Found 1 target...
Target //:setup up-to-date:
  bazel-bin/ready.txt
INFO: Elapsed time: 5.584s, Critical Path: 0.01s
INFO: 2 processes: 2 internal.
INFO: Build completed successfully, 2 total actions
[bazel] fixed module catalog ready blobs=156
[bazel] ready binary=8.4.2 darwin/arm64 sha256=45e9388abf21d110 modules=156 blobs lock=06c085a17c9611ed
[bazel] readiness {"version":1,"bazel":{"version":"8.4.2","platform":"darwin","arch":"arm64","sha256":"45e9388abf21d1107e146ea366ad080eb93cb6a5f3a4a3b048f78de0bc3faffa","executable":"/Users/ueli/Documents/semio/.cache/b/t/m/bazel/bin/45e9388abf21d110/bazel"},"modules":{"lockSha256":"06c085a17c9611edb2348fa8f6eb9726db9d85a5f3a9edebd2d2958c688d2df0","blobs":156}}
[semio] The in-house build engine needs no external tool to provision.
[semio] Die hauseigene Build-Engine benötigt kein externes Werkzeug.
```
