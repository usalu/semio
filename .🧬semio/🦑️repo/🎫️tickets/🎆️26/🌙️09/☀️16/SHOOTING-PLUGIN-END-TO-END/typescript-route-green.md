# TypeScript route green: blocked

- `repo run @semio-tech/shooting-js:test --lang=en --output=stream` exited 75 (no tests ran; 0 pass, 0 fail).
- Cause: project graph unreadable on this host. 277 stale projects need inference, fleet delivered 0 in 300 s; heavy projects (framework-process ~9854 MB, s-dev-composition ~3702 MB, ...) exceed ~3512 MB per project available. Output: `🗑️generated/ts-test.txt`.
- Static check: `parsePlaygroundBlock` (plugin registry playground discovery) already copies `devContribution`, and projection/catalog-view handle it; hub Cargo.toml:22 carries the value. No source defect found by reading.
- Files changed: none.
- Not done: test rerun, `s-dev-composition:generate` (needs the same graph, project needs ~3702 MB).
- Next: rerun on a machine with more free memory or when the fleet serves the graph.
