# Selected Graph Growth Follow-Up

Read-only Source audit after bounded working-set collection. This is a concrete remaining Source task, not a Native runtime or performance qualification. The broad goal remains active.

The actual selected backend still derives roots by filtering every produced project and searching every produced project's compile dependencies. With V projects and E compile edges, that expression can perform O(V*(V+E)) comparisons. It also searches the accumulating dependency array before inserting each generated dependency, which can make graph grafting quadratic in edge count. These are structural bounds from the current loops; no end-to-end timing or CPU measurement is claimed.

The next implementation should extract pure repository-owned graph assembly helpers, derive referenced project names in one pass and retain source/target membership while grafting edges. Preserve existing explicit semantics for reference parsing and aggregate roots. Use schema-authored DAG vectors with an independent SQLite edge/root oracle before replacing the actual controller calls. Include overlapping selections, disjoint roots, shared ancestors, existing edges, malformed owned DATA and bounded cancellation/progress. No legacy layer or new dependency is needed.

Do not weaken final Source observation or actual Store/namespace ownership to optimize this. Physical quota/Nx/compiler/platform and whole-repository acceptance remain separate.

Source: `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🦀️rust/🟦️.ts`, SHA256`c258ad8faee77336fe0c2332a62e7c5f35a6783532a5a85b01a7467bbc789f7b`, 18997 bytes. Relevant observed declarations:

- Line57: `generated dependency membership and grafting`.
- Line58: `root derivation and aggregate edge insertion`.

## Resolved Source Implementation

The actual controller now uses one seeded live pair index and ordered DAG/root projection; both audited repeated scans are removed. Current checkpoint `rust-graph-assembly-checkpoint-current.md`, SHA2564bb22ed14247813e90aa1611cfecae46311a46699a1da79ba78c79f2ad807626, 9232bytes, records the real65,536project frontier, independent SQLite/Ajv/JSON oracles, exact DATA limits, genuine lifetime/failure/epoch/cancellation tests, final both-mode gates and revision qualification. This historical audit remains as evidence of the original problem. Native/compiler/cache-size/platform/full-goal acceptance is not implied.
