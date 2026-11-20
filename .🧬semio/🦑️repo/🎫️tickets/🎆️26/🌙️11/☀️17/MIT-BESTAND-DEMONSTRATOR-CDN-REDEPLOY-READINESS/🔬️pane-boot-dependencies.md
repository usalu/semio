# Pane Boot Dependency Audit

The runtime catalog and generated playground definitions identify the selected guest, while observations below do not establish the complete request set for every successful boot.

| Pane | Runtime variant | Primary plugin ID | App | Evidence |
|---|---|---|---|---|
| generator | generation3d | procedural | s.procedural.generation3d@1/*#editor | generator-baseline-trace.log records procedural ingress and setContributions fault before ready |
| koordinator | koordinator | demonstrator | s.cad.cad@1/*#editor | boot-en.log records demonstrator component fault |
| aggregator | aggregator | demonstrator | s.puzzle.puzzle3d@1/*#editor | acceptance failed; complete request inventory unavailable |
| aussuchen | aussuchen | demonstrator | s.sourcing.curation@1/*#editor | acceptance failed; complete request inventory unavailable |
| bearbeiten | bearbeiten | demonstrator | s.process.process3d@1/*#editor | boot-en.log records demonstrator component fault |
| verfolgen | verfolgen | demonstrator | s.gis.gismap@1/*#editor | boot-en.log records demonstrator component fault |
| energie | energy | energy | s.energy.model@1/*#editor | energy-baseline and energy-new-host-baseline record descriptor, bridge, JS, wasm, shared shard and vendor; actor ready but old guest stalled |
| statik | fem3d | fem | s.fem.fem3d@1/*#editor | generated catalog identity; complete request inventory unavailable |

Sources: demonstrator runtime 🔣️.json and 🪧️brand.ts; generated playgrounds and 🔌️plugins.json under ✏️s/🧑‍💻dev/🧩️catalog/🤖️generated. The shared shard worker and vendor shims are positively observed for energy and must remain in staging. The four primary module directories and receipts exist in the current site; presence does not establish freshness or execution. Site sourceTreeDigest prefixes are demonstrator f4c1604c, procedural 423d9c0e, energy de4217d6, fem b0d2cae5. Prior static acceptance was 0/16.

The generated registry declares demonstrator depends on cad, gis, procedural, process, puzzle, sourcing and flow-extension-bim/brep/dictionary/list/logic/math/primitive/text. Procedural declares flow-extension-bim/brep/dictionary/draw/list/logic/math/primitive/text; energy and fem declare no dependsOn. These are staging closure declarations, not proof each module is fetched at boot. No complete retained per-pane request capture establishes which extensions are needed before readiness. Treat them as catalog/staging closure with boot necessity unobserved, not as safely removable. The site builder stages the complete runtimeComponentClosure in 🔨️modules/🧩️runtime/🟦️.ts.

Prioritize procedural, energy, demonstrator and fem for direct diagnosis, then the declared closure and remaining stale modules for a complete deployment. Individual rebuilt guests cannot validate the whole package. Procedural currently encounters the concurrent-input issue documented in 🔬️cargo-stale-dependency-view.md. Rebuild and retain the full closure, then verify both languages for all eight panes with complete request capture and the 16/16 acceptance gate. No builds or tests were run for this audit.
