# Wave 2 + 3 barrier — final combined-tree verification

Run serially by the coordinator after W2-A/B/C and W3-A/B reported (or, for the two pilots that stalled in poll
loops against external breakage, after reviewing their landed code directly).

## Gate results (all re-run by the coordinator, not taken from reports)

| Gate | Result |
|---|---|
| `cargo test -p semio-framework-os-kernel --lib` | **909 passed / 0 failed** |
| `cargo test -p semio-framework --lib` | **137 passed / 0 failed** |
| `cargo test -p semio-framework-plugin-host --lib` | **40 passed / 0 failed** |
| `cargo test -p semio-framework-plugin --lib` | 155 passed / 58 failed — every failure external (two other tickets' migrations; unchanged in kind and count from the W1 barrier). **This ticket's own tests inside it all pass**: transaction testkit 8/8, artifact contribution 3/3, plugin-builder dependency 5/5, mutation plan 3/3. |
| TS `bunx vitest run` (`💻️os/📦️packages/🟦️typescript`) | **292 passed / 2 failed** — the 2 are one test blocked on the repo's own broken wasm build target (`getrandom` for `wasm32-unknown-unknown`), not a path or logic defect. Was 244/4 at the W2 start. |
| `bun ./📜️script.ts policy` | `plugin-dependency/contribution-target`: **0**. `plugin-dependency/parity`: **61**, all the medium migration direction. `mutation-migration/triad-completeness`: **85**, unchanged — the new composite produces none. |
| `bun ./📜️script.ts check` (registry) | catalog and `.vscode/launch.json` fresh. |

## Coordinator repairs at this barrier

1. **Two missing rejection codes** (found by the contract-conformance audit): `transaction.dependency-missing`
   and `transaction.version-mismatch` existed in the frozen taxonomy but nowhere in code. Added
   `PluginGraph::contribution_block(contributor, owner)`, which distinguishes *contributor not loaded* /
   *dependency not declared* / *owner version excluded* / *ok*, and called it per contributed step in
   `🏃️run`'s transaction path — dispatch-time re-checking, because an owner can be unloaded or swapped long
   after load-time validation passed. The `🏃️run` site previously reported "contributor not loaded" as
   `contribution-not-permitted`, which pointed an operator at a declaration that was already correct.
2. **A bug in this ticket's own policy gate** (found by W3-B): `policyDependencyOwnerRoots` lists a plugin and
   its nested extensions as separate owners, but the file walk for each owner recursed into
   `🧩️extensions/**` — so a plugin inherited its extension's `.depends_on(...)` and was told to add a Cargo
   dependency it does not need. Fixed with `policyOwnerOwnComponentFiles`, which excludes nested extension
   subtrees. Verified: `aec-building`'s `.depends_on("cad")` no longer attributes to `cad`.
3. **Cross-lease compile fixes** so the guest SDK and run crate could build at all: a missing `BTreeSet`
   import, two `preflight` visibilities, and a double-`Result` unwrap in a builder test — all in code another
   session was mid-writing inside our lease files.

## Honest limitations

- **No `semio-s-plugin-*` crate can be compiled right now.** `semio-s-plugin-stdio` — which every plugin
  depends on — is mid-restructure by ticket `26/08/16/FULL-STDIO-…` (11 errors: an unresolved gltf inference
  module tree and two missing `include_str!` files). Both pilots' code is landed and reviewed, but
  `cargo test -p semio-s-plugin-flow` and `-p semio-s-plugin-cad-aec-building` **have not been observed to
  pass**, and are not claimed to.
- **The end-to-end composite transaction over real wasm is still not demonstrated.** W2-A's wasmtime e2e loads
  two real plugins and a real extension and proves graph/router/directory behaviour against them, but the
  full proposal→prepare→commit cycle is proven against a pure-Rust wire harness, because the pilots' guest
  components cannot be rebuilt while stdio is red. The pieces are in place — a shipped extension now really
  declares `.depends_on`/`.contributes`, and a shipped artifact really has a composite mutation — but the
  wasm round trip that joins them is the one claim this ticket does not get to make.
