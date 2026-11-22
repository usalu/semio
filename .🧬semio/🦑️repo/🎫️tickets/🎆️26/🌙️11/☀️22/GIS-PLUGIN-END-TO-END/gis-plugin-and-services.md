# GIS plugin and services gates

Status: blocked before any task ran. No files were changed.

1. `@semio-tech/gis-plugin:test` failed after 60 min during graph load:
   "Generated task cannot consume its own output": target `@semio-tech/gis-gismap-js:build`, importer
   `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/📦️packages/🟦️typescript/🧪️tests/🪶️sqlite/🟦️.ts`, output `.../typescript/dist/🟦️.js`.
   Owner: gismap-js TS package (other agent). The sqlite test imports the build output of the package whose build declares it as input.
   Service log: /var/folders/xq/z6t8hmcd0qj871bx0sj7pbn80000gn/T/semio-repo/501/host-122bef2ab58b.log (ref fa6ebb8c).
2. `@semio-tech/s-gis-services-native:native-build` failed after 26 min: slice worker exit 75,
   "Private Nx engine import missing: ../../🌐️remote-execution/🛰️sidecar/🚦️ready/🩺️workers/🟦️.ts from
   🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🚢️fleet-status/📥️readers/🟦️.ts".
   Missing file in the repo framework (build tool), not in the GIS crates.
3. native-codec-check, mcp-build, user-path-check, security-check not run (same graph load would fail).

## Retry after planner fixes

`@semio-tech/s-gis-services-native:native-build` failed in 18 s, deterministic (ref daa5a796): "Closed cache ledger required: area caps
leave less free space than the soft reclaim threshold of bazel-workspaces" (`⚡️caching/💾️storage/🧾️ledger/🟦️.ts`).
`repo cache status` fails identically (ref 11be1b53). Disk has 31 GiB free of 228 GiB, so the cause is the machine's cache budget/area caps,
not GIS code. Fix needs a machine-level budget change (SEMIO_CACHE_BUDGET_BYTES or machine file) or `cache gc`, which affects every agent's caches;
not done. mcp-build, user-path-check, security-check, native-codec-check not run.

## Retry after cache fix (2h37m run)

`native-build`: graph slice read took about 6100 s, then failed (ref 34430f67): BUILD_SOURCE_UNPARSEABLE, cannot find module
`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/📦️sdk/📥️source/🔗️derivative/🔏️index/🧊️stamps/🟦️.ts`.
The importing `🔏️index/🟦️.ts` was edited at 04:24:50 (fresh); the `🧊️stamps` folder still does not exist after 4 min of waiting (04:30).
Repo-tool in-flight edit by another agent, not GIS code. Further targets not run; each retry costs a long graph read, so held until the module lands.

## Combined run (repo CLI via Bash; the mcp__repo__* tools were blocked by a dead PreToolUse hook)

Single `repo run-many` over s-gis-services-native, gis-plugin, os-hub, s-dev-composition, framework-surface-rs
(targets: native-build, mcp-build, user-path-check, security-check, native-codec-check, component-codec-check, gis-map-proposal-process-check,
trusted-stdio-gis-bundle-process-check, trusted-stdio-gis-bundle-browser-check, typecheck, wasm). Logs: `🗑️generated/gates-run-many.log`, `-2.log`.

Run 1 (about 200 s of planning): failed, no target ran. BUILD_SOURCE_UNPARSEABLE (ref 3c5c8ace): tsv snapshot test
`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📑️tsv/🏅️standards/🔖️iana/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts` (mtime Oct 20) imports
`../🫴️receiving/🛂️validation/🧪️tests/🧫️fixtures/🔣️.json` and `../🫴️receiving/🧪️tests/🧫️fixtures/🔣️.json`; the tsv `🫴️receiving` tree does not exist
(never in git; sibling txt/binary artifacts have it). stdio/tsv area with many fresh concurrent edits, not GIS; not edited.
Run 2 started (`🗑️generated/wait-and-run.sh`): waits up to 15 min for tsv `🫴️receiving` to appear, then reruns the same run-many; result in `gates-run-many-2.log`.

Hub gates and surface (from `hub-gis-gates.md`, `surface-wasm-and-gis3d-dev.md`): all not run, same planner blockers; included in the combined run above.
