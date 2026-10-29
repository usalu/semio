# Nx Selected External Inputs — 2026-10-29

The full normal course typecheck11724 exited1 before any target ran: Nx could not find externalDependency nx for @semio-tech/assets:build. Its graph had557 external nodes and no nx entry. Independent bounded inspection of the sealed generation found1625 external nodes, including canonical npm:nx version23.2.0 with its locked hash. Root package.json and bun.lock also pin23.2.0. This was not an absent package or SDK installation problem.

The selected generation retained external nodes reached by project edges and explicit external inputs. Two omissions combined: raw package inputs were matched only against exact keys (nx versus npm:nx), and the private Nx engine injected its nx/typescript inputs only after selection, while passing the selected external table through unchanged.

The fix declares both private executor families' package roots before the normal graph load. A neutral externalInputs option travels through the selected-generation reader and assembler. References resolve first to an exact sealed key, then to its root npm key, including scoped packages. Existing external edges close transitively; original nodes, versions and hashes are preserved. Unrelated external packages remain pruned. An unknown explicitly requested consumer root refuses rather than fabricating a node.

Canonical requested roots are recorded in graph admission and included in its closure digest. The normal selected reader assembles the view before content-addressed snapshot publication. Explicit pinned reuse checks require the same canonical admitted roots, not merely an external-table superset. No mutable installed-package supplement, blanket1625-node inclusion, private SDK modification, hasher/cache bypass or dropped dependency was introduced.

Schema limits external roots to65536 references of4096 Unicode code points each. Validation stops at the bound without expanding an unbounded string. Raw and canonical aliases normalize to the same root identity.

The focused regression publishes an in-memory sealed generation containing the actual canonical Bun lock external table, reads it through the real loadGenerationGraphV1 selected path, projects it through projectBuildGraphForNxV1, and invokes the qualified SDK's native HashPlanner. The old Source reproduced the exact11724 missing-nx error. The repaired path passes, preserves locked node data, prunes an unrelated sentinel, and distinguishes omitted root admission while treating canonical aliases identically. The canonical existing Nx oracle namespace fixture is used; initial attempts without its artifacts/namespace declaration were correctly refused and were not treated as the baseline.

A language-neutral reference corpus covers raw, canonical, scoped and exact custom keys. Ajv checks the authored schema; an independent SQLite recursive closure checks reference resolution and transitive pruning. The actual graph reader validates the new admission data. Positive and negative pinned-view checks exercise the implementation used by reuse.

Final focused result:3passed,0failed,45assertions,1.233s. Nearby existing assembly regressions:4passed,0failed,1filtered,71assertions,290ms, including the independent model, single-use stream, cancellation and batching. Bootstrap, graph loader and model TypeScript parsing reported no diagnostics. Logs: generated/nx-external-input-baseline.log, nx-external-input-fixed.log, nx-external-stream-regression.log.

Changed production Source:
- library/🟨️.mjs and 🟨️.d.mts: derive private engine external roots from both declared families.
- library/⚡️caching/🚀️bootstrap/📜️script.ts: request those roots before graph selection.
- library/🏗️build/🕸️graph/🟦️.ts: thread roots and validate explicit pinned reuse.
- library/🏗️build/🕸️graph/📋️model/🟦️.ts and library/🏗️build/🧬️schema/🔣️.json: canonical admission roots.
- library/🏗️build/🕸️graph/🕰️generations/🟨️.mjs and 🟨️.d.mts: bounded external-root protocol, canonical resolution and selected assembly.
- library/🏗️build/🕸️graph/🕰️generations/🧬️schema/🔣️.json: external-root request schema.
- library/🏗️build/🕸️graph/🕰️generations/🔌️load/🟨️.mjs and 🟨️.d.mts: selected view and closure identity.

New test Source:
- library/🏗️build/🕸️graph/🕰️generations/🧪️tests/🌊️stream/🔌️external/🟦️.ts
- Same directory: 🔣️.json and 📋️schema.json.

Here library means 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library at the repository root. No source files were copied into this ticket.

No full semantic retry was launched: last measured headroom3890MB was below the learned5378MB reservation. Parent now owns the normal persistent course preview; semantic retry remains deferred until its reservation is accounted for and capacity fits. This report does not claim TypeScript, unit, watcher, browser or persistent readiness success.

Parent dev64240 was admitted with3584MB/PID658431. Fresh authoritative capacity1793231295701 showed6645MB headroom/no waiters, fitting the last semantic5378MB reservation. Parent authorized one unchanged full semantic retry13235; it is live with English/NX_DAEMON=false/ticket/parallel1. Loggenerated/course-typecheck-nx-external-fixed.log. Preview handle remains parent-owned; no readiness or TSC success claim.

Independent fresh_runtime_path review found no blocking defect in the normal selected route: exact/npm/scoped resolution, transitive pruning, original locked node preservation, prepublication engine roots, canonical closure identity and exact pinned-root admission checks are present. It confirmed actual3/45 and4/71 logs; see nx-external-dependency-audit-2026-10-29.md. Production Source has remained frozen since before parent dev64240 and semantic13235 launched. The change intentionally rekeys private dependency implementation identity through library/🟨️.mjs; no implementation-proof guard or installed SDK bytes were edited. A separate root reported an implementation mismatch, but its causal diagnosis belongs to the bootstrap owner; this report does not attribute that failure to stale bytes without evidence. Semantic13235 is currently live in closure verification, with no logged error.

The full normal retry13235 exited1 before Nx target execution, after166/166 inference and SDK preparation. Its first cause was the selected Native descriptor inventory guard during retirement: selection filesystem, child status1/signalnull/reasonexit, stdout108bytes, stderr0bytes, endingHex746976652f6d000a, one guard with maximum3, expectedPid659329. Stack: assertNativeSelectedDescriptorCustodyV1 descriptors.ts156 → retirement contract.ts270/267/281/243. This was not an assets/HashPlanner or TypeScript result; operational passage of the missing-nx boundary remains pending. No Source guard was relaxed, no retry/cleanup was launched, and the parent-owned preview was untouched. No owned live handle remains. Loggenerated/course-typecheck-nx-external-fixed.log.
