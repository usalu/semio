# Audit response

Two read-only audits ran against the finished tree: a **contract-conformance** audit (does the code match
`📋️contract-freeze.md`, judged by reading code, not reports) and an **evidence-honesty** audit (is every
claimed pass actually supported). Both findings lists are answered here.

## Contract conformance — 73/75 conform, 0 deviate, 2 missing

Everything in §1 (composite spine), §2 (channel), §3 (identity grammars), §4 (registration gates), §5
(transaction protocol) and §6 (WIT) was verified present and as-specified with file:line evidence, including
the four checks written into the contract precisely because they are the easy things to get wrong:

- `MutationDescriptor`'s fingerprint still derives from **only** id/schema_version/state_class/conflict_rule —
  the golden pin is intact and `contributor`/`artifact_kind` do not participate.
- Nothing added to `protocol`/`store` imports `semio_framework::*` or `io::*` — the dependency edge is not
  inverted.
- `ArtifactDeclaration`'s owner asserts for OWNED artifacts survive unweakened; contributions are an additive
  path, not a loosening.
- The owned-composition ownership check (`owner_of(child) == parent`) is intact, and only the `Peer` branch
  bypasses it — the owned path is unchanged.

**Both findings fixed.** `transaction.dependency-missing` and `transaction.version-mismatch` were in the frozen
taxonomy but nowhere in code. They are now raised from `PluginGraph::contribution_block`, called per
contributed step at dispatch time. The audit was right that their absence mattered: the existing code reported
"contributor not loaded" as `contribution-not-permitted`, which tells an operator to fix a declaration that is
already correct.

A note the fix produced: the *version-mismatch* branch cannot currently fire, because `PluginGraph::register`
re-validates the whole graph and refuses a swap that would break an existing requirement. Rather than write a
test that pretends to exercise it, the test asserts **why** it cannot fire and the branch stays as
defence-in-depth for any future load path that mutates the set without that re-validation.

## Evidence honesty — findings accepted

| Finding | Response |
|---|---|
| W1-B claimed 8 tests pass without ever running them | **Correct, and it said so explicitly** in its own report ("I have not personally observed any of them pass or fail"). The W1 barrier re-ran them: 8/8 genuinely pass. The lane's honesty is the reason this was catchable. |
| W2-A's wasmtime e2e does not prove composite mutations over real wasm | **Accepted and carried into the final summary as the one unmet claim.** The e2e proves graph/router/directory against real loaded components; the transaction cycle is proven against a pure-Rust harness. |
| W2-A ran against pre-built `.core.wasm`, fresh builds fail | **Accepted.** Fresh guest builds are blocked by `semio-s-plugin-stdio` being mid-restructure by another ticket, not by this ticket's changes. |
| `semio-framework-os-run` could not compile | **Fixed after the audit.** The blockers were a missing `BTreeSet` import and two private `preflight` methods in code another session was mid-writing inside our lease. The crate compiles; only stdio blocks it now. |
| W0-B never ran the Rust channel tests | **Correct.** The W0 barrier ran them and found two real golden-hex failures, which is exactly what a barrier is for. |
| TS counts differ between W2-B's report (290/4) and later runs (292/2) | **Explained**: the coordinator repaired two dead cross-language parity tests after W2-B reported — see `📓️coordinator-parity-repairs.md`. The delta is improvement, documented. |
| No misattributed failures found | Attribution was checked against the start commit throughout; the audit independently confirmed it. |

## Deliverable evidence, restated honestly

| Claim | Evidence | Strength |
|---|---|---|
| Mutations can call other mutations | 7 law tests on the spine + 8 transaction testkit tests + a real shipped composite (`👯️duplicate-widget`) whose plan calls two existing leaf kinds through one `Planner` | **Strong** |
| Plugins can depend on plugins | 18 manifest/graph unit tests + 6 host graph tests + a real shipped `.depends_on("cad", "^0.1.0")` on the aec-building extension + dependency-ordered boot in both hosts | **Strong** |
| Plugins **and extensions** can register mutations/inferences on another plugin's artifacts | 11 SDK tests + 4 host router tests + a real shipped contribution (one composite mutation and one inference contributed by an extension onto cad's artifact, gated by the declared dependency) | **Strong** |
| Works in the Rust wasmtime host | 40 host tests + a wasmtime e2e loading 2 real plugins and 1 real extension | **Good** — the transaction cycle itself is harness-proven, not wasm-proven |
| Works in the TypeScript browser host | 292 passing vitest tests incl. the full coordinator, pack cache, routers and graph | **Good** — same caveat: no real-wasm composite round trip |
| End-to-end composite transaction over real wasm | none | **Not achieved** — blocked by `semio-s-plugin-stdio` being uncompilable for the whole session |
