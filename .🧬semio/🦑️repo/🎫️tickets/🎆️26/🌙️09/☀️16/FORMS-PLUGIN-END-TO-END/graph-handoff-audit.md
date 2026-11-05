# Graph Handoff And Canonical Name Audit

This was read-only inspection through repo tools. No alias, guard, pointer, cache, claim, receipt, or project declaration was changed.

## Exact Handoff Conditions

The claims registry prefetch loop prints “generation ... was published by a peer, nothing is left to prepare” when its served callback returns a generation. The graph host callback returns the current pointer generation whenever it differs from the request generation. This ends prefetch work; it is not a claim that every selected target is fresh. The generation loader then performs its normal attempt/builder admission and can still queue or yield its loader.

Custody claims conflict when either claim is structural, or when their sorted owner lists intersect. A held conflicting claim or an earlier queued ticket blocks admission. Thus a structural/full refresh can block an otherwise disjoint scoped refresh globally. The “refresh(es) go first ... whole workspace” text is selected when either claim is structural (or no overlapping sample can be named). Missing/withheld BDD nodes do not directly trigger this message; the plan’s structural/environment/membership demand and custody conflicts do.

Source owners inspected: graph/generations/claims, graph/generations/custody, graph/generations/load, and graph host served callback. The stamp hint is created after planning; no pre-plan age bug was established.

## Physical Declarations Versus Generation 898

The missing bare Flow names have actual current Cargo and project declarations under the Flow module. Representative current names semio-os-flow, semio-os-flow-host, and semio-os-flow-wasm differ from the framework-prefixed names stored at the same config paths in generation 898. WebGPU has a current Cargo package semio-ui-backend-webgpu and a framework-prefixed synthetic owner in that snapshot.

Digest-checked generation 898 fact index contains both historical and current file facts for the three representative Flow project configs. Their current modifications were observed at 2026-11-04T23:50:04 UTC. Manifest publication mtime was observed at 2026-11-05T03:12:26.825Z. The current SHA256 is present among captured facts for every inspected config. See 🗑️generated/graph-config-digests.json for exact hashes, metadata and timestamps. These observations establish mixed historical/current support in a partial generation; they do not establish which support a selected verified owner holds or a freshness guard defect.

## Selected Closure Read

A bounded read of digest-checked immutable generation 898 fragments found no missing bare Flow/WebGPU names in project-edge closures from Forms JS (625 visited owners), renderer React (624), or Play (1080). The workspace project has no project edges; its components target separately declares emitter build, plugin-web support, and UI generation prerequisites. This is project-edge evidence, not a replacement for actual target planning or source admission. See 🗑️generated/selected-graph-closure.json.

A direct selected-target discrepancy remains in that snapshot: the physical Play manifest declares prepare-forms-e2e, serve-forms-e2e, and test-e2e-forms, while generation 898’s stored Play node has none of those targets. Therefore the browser caller still requires a current scoped generation. An actual selected-command refusal would justify focused correction; no compatibility aliases or freshness relaxation were introduced from this inspection.

The later fleet snapshot showed generation 898 published, no graph computer/custody slot active, and descriptor 19405 proceeding through dependency/Nx integrity admission. The original renderer wait therefore was not proof of a permanent deadlock. Exact withheld BDD IDs and messages, including the Forms BDD entry, are retained in forms-native-workspace-scope.md.

## Retained Config Digest Evidence

| Current project | Historical captured SHA256 | Current SHA256 (also captured) | Current modified UTC |
|---|---|---|---|
| semio-os-flow | a1fa548b330a2afa5dd47fd4791b8fa0ce86bfcd1e538652aa5a8b640e1b66e9 | 5f8baa3962c07dab860e3805c0f444bdb9193f18e25d1090cf041220484462bf | 2026-11-04T23:50:04.121Z |
| semio-os-flow-host | 13b8b2e416fef6edc588ef8a16d11f8535450dee331539288d6e455fc644bdc2 | c71532372ea7fab912daebe59067b06c549c2c5a7b3476d07cbf1242e7ed9743 | 2026-11-04T23:50:04.195Z |
| semio-os-flow-wasm | 6f3b81784e4f1a1ff31c5fd5f54f4aad8c73da716fd5895d961b878a3c34aee3 | cd671c9ae2aac89ffc0d1f533b63e824dfa640b0549831197b0a7caf413eb2a7 | 2026-11-04T23:50:04.161Z |
