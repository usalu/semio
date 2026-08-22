# P10K Owned B-Rep And Repo Server Implementations

//#region 🎯️Scope

This packet owns the final six live JavaScript dependency-parity findings without adding dependency
rows, allowlists, suppressions, or third-party types to repository-facing signatures:

- three `brepjs` / `brepjs-opencascade` imports in the shared spatial-kernel B-Rep engine;
- three `pg` / `next` / `pg-boss` imports in the repo server library.

//#endregion 🎯️Scope

//#region 🧩️OwnedBoundaries

The CAD package now contains `🟦️brep-implementation.ts`, behind the deepest manifest that
already declares `brepjs` and `brepjs-opencascade`. It owns:

- opaque B-Rep edge, face, shape, solid, wire, shell, vertex, result, and mesh contracts;
- all B-Rep operation calls used by the shared spatial kernel;
- bundled and Node-test OpenCascade WASM location;
- OpenCascade initialization and its binding into brepjs.

The spatial-kernel consumer imports only these owned contracts and operations. Browser behavior is
preserved by keeping Node module resolution dynamically imported only on the Node-test branch.

The repo coordinator package now contains `🟦️server-implementations.ts`, behind the deepest
manifest that already declares `pg`, `next`, and `pg-boss`. It owns:

- a query/end database-pool interface;
- request and response contracts plus JSON response creation/recognition;
- durable job and job-queue contracts.

The repo server library now exposes owned signatures and retains its existing lazy database pool,
Next JSON response status/body behavior, pg-boss worker registration, and graceful stop behavior.

The CAD package task router's pre-existing invalid `join` / `resolve` imports from the repo script
library were corrected to the platform-owned `node:path` module. This unblocked the real CAD Nx test
runner; no repo-library export compatibility layer was added.

//#endregion 🧩️OwnedBoundaries

//#region 📊️Parity

Packet start:

- manifests: **83**
- external rows: **303**
- evidenced rows: **144**
- unowned rows: **159**
- undeclared imports: **6**

Packet end:

- manifests: **83**
- external rows: **302**
- evidenced rows: **145**
- unowned rows: **157**
- undeclared imports: **0**

The six findings were deleted without changing a manifest. No `bun install` was needed. The two
declaring manifests now each own direct source evidence for their implementation rows. A concurrent
`date-fns` identity removal accounts for the additional **303 -> 302** external-row and **146 -> 145**
evidenced-row change after this cohort reached zero undeclared imports.

//#endregion 📊️Parity

//#region ✅️Validation

- `bun ./📜️script.ts verify dependencies parity js`: **PASS**, clean with zero undeclared
  imports.
- `bun ./📜️script.ts verify dependencies`: **PASS**, baseline 238, current 179, removed 59,
  additions 0.
- `bun nx run @semio-tech/cad-js:lint --skip-nx-cache`: **PASS**, zero recorded breaches.
- Focused CAD Nx B-Rep volume test: **PASS**, 1 test passed and 29 non-matching tests skipped.
- B-Rep adapter runtime smoke: **PASS**; initialized OpenCascade through the owned boundary and
  verified a 2 x 3 x 4 box measures volume 24.
- `bun nx run @semio-tech/repo-coordinator:test-quick --skip-nx-cache`: **PASS**; the package currently
  has no discovered test files.
- Repo server implementation and library import smokes: **PASS**; owned JSON response status 401 and
  response recognition were verified without opening a database or job-queue connection.
- Prettier check: both new owned-boundary files, the repo server library, and the CAD task router are
  clean. The already non-clean large B-Rep component remains the only check warning; it was not
  bulk-reformatted in the shared worktree.

The complete B-Rep file Nx run reached **29 passed / 1 failed**. The sole failure is outside this
packet: `defaultModelDefinitionId()` throws `ReferenceError: defaultModelDefinitionIdCache is not
defined` in spatial geometry line 1492. The full CAD quick suite similarly reaches Vitest but is
currently blocked by the same concurrently edited spatial-geometry cache regression and related
missing caches. The focused B-Rep adapter behavior itself is green.

No Cargo command was run.

//#endregion ✅️Validation
