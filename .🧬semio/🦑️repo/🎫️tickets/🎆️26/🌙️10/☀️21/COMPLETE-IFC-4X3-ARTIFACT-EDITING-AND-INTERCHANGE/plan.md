# IFC 4x3 Artifact Plan

## Goal and Boundaries

Implement IFC4X3_ADD2 editing and interchange through the repository artifact infrastructure. Keep IFC4 and IFC2x3 as independently versioned standards. Do not change AGENTS files, modify git state, create worktrees, or take ownership of the concurrent build-system refactor.

## Fleet

The coordinator uses the requested Extra High setting. Three concurrent GPT 6.1 Sol workers exhaust the four available slots. Initial read-only audits use Low, the supported equivalent of the requested Light setting; implementation workers use High. Repeat independent audits after implementation.

## Work Streams

1. Official complete EXPRESS catalog, provenance, inherited attributes, type/select/enum metadata and schema validation in Rust and TypeScript.
2. STEP codec, official examples and independent parser comparison, explicit BIM/FEM 3D/GLB exchange contracts with preservation and loss diagnostics.
3. Accessible, localized details editing through the existing host details provider and central mutation path.
4. Coordinator: TypeScript mutation-produced sparse diffs, atomic central application, inverse diffs/mutations, transaction history and language-neutral test fixtures; integration and validation.

## Acceptance Evidence

- Enumerate every official IFC4x3 entity and type; distinguish syntax, schema validation, EXPRESS rule validation and MVD conformance without overstating coverage.
- Test entity, header, positional and nested edits, invalid targets, identity collisions, references, atomic failure and undo/redo against shared fixtures.
- Round-trip official IFC STEP examples and compare an independent implementation.
- Test exchange through the real artifact representations; surface unsupported geometry or semantic loss explicitly.
- Exercise details controls end to end through mutations and verify localization/accessibility.
- Register executable IFC4x3 capabilities and keep declarations honest.

## Infrastructure Findings

Read `repo://goals` through the cached repo MCP binary before opening this ticket, associated with `runningifc`. No matching earlier IFC4x3 ticket existed. Ticket metadata uses generic `codex` because the MCP allowlist rejects GPT 6.1 Sol; actual fleet models are recorded above.

The `repo` command is absent from PATH. `.🧬semio/🦑️repo/⚡️cache/🗃️bin/repo.exe` is MCP-only. Its sibling `semio-repo.exe` exposes the old domain CLI, with no script/run/file-read route. Necessary source reads and focused tests may require direct tool/runtime fallbacks until the other agents restore that route. No build infrastructure changes will be made to address this in the IFC ticket.

Latest continuation checkpoint: `current-state.md` records 401 passing tests / 11,639 assertions across all 28 current registered IFC4x3 suites, twelve genuine-output/environment-gated skips, and the still-active full objective. `template-ui-implementation.md` records the real complete released-template selector and remaining guided-form gap. `exchange-control-implementation.md` records five test-first original cancellation fixes, declaration-based geometry classification, and independent p-retry/IfcOpenShell evidence. `native-details-admission-independent-audit.md` is the latest limited source audit; the High execution worker resumed to close its real-grant test gap and remaining retained frontiers. No compiler ingress bypass, native execution claim, ticket close or goal completion was made.

## Current Integration Wave

The active goal and ticket remain open. Completed implementation slices and exact verification boundaries are documented in `schema-implementation.md`, `core-shape-fixes.md`, `core-ownership-fixes.md`, `binary-implementation.md`, `details-implementation.md`, `host-audit-fixes.md`, `details-reference-insertion.md`, `geometry-integration.md`, `exchange-followup-fixes.md`, `native-io-fixes.md`, `route-capabilities.md`, `mvd-implementation.md` and `native-express-ast.md`.

The coordinator's root facade/test registration includes public async core APIs, graph/source/host identities, complete scope metadata and stale-native-program detection. `IFC4X3_GRAPH_SCHEMA` is `stdio.ifc`; `IFC4X3_DOCUMENT_SCHEMA` consistently identifies the host envelope `s.stdio.ifc@4x3/*` in TypeScript and Rust. Missing empty complex lists are admitted and normalized by the shared-graph editing contract; they are not a separate compatibility API.

The current full fleet owns EXPRESS semantic execution/differential verification, native glTF/GLB host exchange and an independent host/MVD/native-AST audit. Further native semantic execution, complete MVD constraints, specialized geometry/interactions, native BIM/FEM routes and genuine host end-to-end verification remain required. Authentic native compiler ingress remains unavailable and is not bypassed. Generated oracle dependencies/results are shared with concurrent IFC workers and must remain until the entire ticket is done; retain all input scripts and Markdown audits permanently.

## Current Verification Wave

The full fleet now owns native EXPRESS implementation and corpus tests, TypeScript semantic review fixes/controlled structural admission, and independent native domain-exchange review. Native BIM/FEM routes have reached a written clean audit point; supported mappings and explicit refusals are recorded in `native-domain-exchange-implementation.md`. These do not replace the outstanding authentic native compilation and end-to-end host checks.

The root completed retained reference admission and complex-control fixes (`host-mvd-fixes.md`), sparse identity-preserving entity type changes, declaration-aware domain reset and exact negative-zero whole-entity deltas (`host-type-switch.md`). Native test inputs are retained, and independent IfcOpenShell details vectors run successfully. Independent host re-audit is next when a fleet slot frees.

All 46 original official ADD2 specification examples are now permanent regression fixtures with pinned provenance, closed neutral metadata and IfcOpenShell 0.9.0.post1 baselines. Their 8,172 entities round-trip through owned STEP and central diff admission; both validators agree on 45 clean models and one original IfcPerson.Identification schema error. The new corpus suite passed 47 tests / 462 assertions after the controlled admission implementation (`official-corpus-results.md`).

Full goal acceptance remains open: native execution and retained/browser interactions; source-owned semantic diagnostic integration; broader supported geometry/domain exchange beyond the explicitly admitted subsets; complete MVD/formal/informal coverage with honest physical STEP boundaries; and any remaining independent audit findings. Written implementations and corpus tests alone do not close this ticket or goal.

## Current Authoring and Ownership Wave

The complete pinned property/quantity template catalog is present in both implementations: 760 templates and 3,988 entries, exhaustively matched against actual IfcOpenShell templates and independently parsed native literals (`property-set-implementation.md`). A High execution worker now owns selected-template application through the same mutation/diff/inverse/history infrastructure, all published carrier categories and all property/quantity forms. Catalog metadata alone is not authoring completion (`template-authoring-plan.md`).

Semantic P1/P2 fixes passed an independent focused re-audit (`native-express-reaudit.md`). Root registered derived-binding and native-execution parity tests in the package suite. Root added released MVD header declaration helpers and localized native presets, with owned central edits, neutral scenarios and independent IfcOpenShell header comparisons (`mvd-header-implementation.md`). Native source remains unexecuted.

Native details work now has contextual derivation repairs, compact type requests, source-owned preparation and catalog-derived row capacity. Independent follow-up found cancellation custody and retained GlobalId emission defects; High is fixing them (`host-derived-reaudit.md`). Remaining physical allocation/shift/metadata work is not represented as proved bounded execution. Native FEM owner receiving/pack decoding and a minimal original-control nested-phase fix are progressing; actual funded IFC projection and output codec binding remain required. Shared generated outputs must remain while these workers use them.

## Current Native Authoring and Codec Wave

TypeScript selected-template authoring now has all seven published carriers and twelve property/quantity forms, retained identities, central sparse transaction/inverse/history and full asynchronous semantic admission. Its independent population/EXPRESS checks and closed-capture repair are recorded in `template-authoring-implementation.md` and `template-capture-fixes.md`. High is implementing the same native command/receiving path while coordinating actual original command capture with the details worker.

Details custody/GlobalId findings were repaired (`host-custody-fixes.md`); High continues borrowed forward/inverse Store wire admission and original header/event custody rather than synchronous clone/encode fallbacks. Native FEM receiving, funded builders and registered route sources reached review; Low found a successful pack-close phase failure, a scoped-direction test defect and quadratic identity admission. High is fixing those and then the remaining generic IFC pack ownership boundaries (`native-domain-exchange-reaudit.md`).

Root replaced active TypeScript STEP scans/output with owned central admission, bounded chunk export, actual cancellation and normative print/control handling, retaining original offsets and independent-parser differences (`step-control-implementation.md`). The complete official corpus remains green. Native STEP lexical parity, broader semantics/geometry/domain mappings, full MVD constraints and authentic retained/native execution still remain; neither goal nor ticket is complete.

## Current Native Source Wave

Root added the complete pinned STEP alphabet contract (nine pages, 855 shifted positions), owned Rust/TypeScript interfaces and registered neutral/independent decoder tests (`step-alphabet-implementation.md`). Native reference lexer and IFC framing/origin scans now preserve ignored-character/print/PAGE semantics and original UTF-8 offsets. Written native source parity is not native execution, retained receiving or physical-work proof; isolated cold parsing/export helpers still require replacement.

The three High execution workers continue actual editor event/value receiving and publication custody, native selected-template commands/planning/semantic admission, and original IFC pack receiving/encoding. Native FEM review fixes now cover explicit successful writer draining, scoped parent direction and sorted identity indexes; genuine source execution remains unavailable. A retained native EXPRESS evaluator remains required after authoring integration reaches its next review point. Keep all shared generated oracle outputs/dependencies while this fleet uses them.

Native IFC Pack ownership/normal routes reached independent Low review (`native-pack-boundary-implementation.md`, `original-pack-independent-audit.md`); High is fixing a native completed-work progress gap without changing authority. Root's native STEP emitter and original-owned UTF-8 publication source are wired to the actual native route with neutral and custody tests written (`native-step-writing-progress.md`). Rust remains unexecuted, physical allocator witnesses and independent source review are pending, and original-owned STEP receiving is not implemented yet. Details additionally identified impossible large-body clone fixtures under their actual authored grant and is correcting evidence/ownership strategy; shrinking fixtures will not constitute large-edit completion.

## Current Interchange Review Wave

Independent Low review accepted the Pack completed-work progress repair in source and found no new blocking source issue in root's original-owned native STEP export (`native-step-independent-audit.md`). Real allocator/receipt/cancellation-prefix fixtures are written, without native execution evidence. High now implements the original-owned TXT-to-IFC import on actual TXT lines/endings, preserving the existing export and normal route registration. Its all-alphabet, lexical, original-offset, official-corpus, five-axis denial, heap/custody and actual-route tests are written; the genuine-output-gated independent comparison must remain skipped until the authentic native producer runs.

Root completed GLB framing and represented-transform parity, plus original cancellation capture for all four public TypeScript glTF/GLB routes (`gltf-framing-implementation.md`). Shared cases cover future chunks, legal JSON whitespace, invalid padding and exact determinant signs across binary64 extremes. Official Khronos validator, three.js and mpmath comparisons run; combined glTF tests pass 31 tests / 70 assertions and focused strict checking passes. Native test source is syntax-checked only. Cold glTF JSON/buffer work and retained geometry custody remain open.

Native details and selected-template workers are reaching review checkpoints while preserving explicit refusals where retained source admission is unavailable. Progressive mutation preimages, large unchanged source ownership, synchronous metadata/semantic frontiers, production grant policy, actual native host execution, full formal MVD constraints and broader exchange/geometry coverage remain required. This goal and ticket stay active, and shared generated dependencies remain available to all IFC workers.

## Current Retained Semantic and Geometry Wave

Independent Low source review found no additional confirmed P1/P2 issue in selected-template/host publication or native original-owned STEP import (`native-template-host-independent-audit.md`, `native-step-import-independent-audit.md`). These are source audits, not compilation/runtime evidence. The High schema worker now owns a genuine retained EXPRESS machine with static borrowed declarations and source/mutation overlay; the High details worker owns sparse progressive preimages and stable borrowed Store wire identity; the High interchange worker owns original-native glTF/GLB receiving, geometry construction and publication. Original command and source authority, all five work axes, cancellation and retirement must stay intact.

Root added dense glTF accessor alignment/extent/role/byte checks, bounded scalar decoding and interior cancellation (`gltf-accessor-implementation.md`), then four exact original Khronos samples with complete coordinate/index comparisons, pinned licensing/provenance and official validator source/re-export checks (`gltf-samples-implementation.md`). All focused tests and strict checking pass; native tests are written and syntax-checked only. No goal/ticket completion or native conformance claim is made.

Root also retained natural glTF JSON last-member/integral-number/BOM behavior and fixed shared current-version admission across read/write/direct import (`gltf-json-research.md`). The current entire IFC4x3 TypeScript sweep passes **351 tests / 11,319 assertions across 26 files**, with ten genuine-output/environment-gated checks skipped and zero failures (39.90 seconds). This is an integration checkpoint on then-current shared source, not native execution. Progressive borrowed inverse source now replaces the temporary repeated-slot refusal (`host-progressive-preimages.md`); the details worker continues initial retained admission, and semantic/publication completion remains required.

The preceding sweep was directory-discovered coverage. Root now executes every current explicit IFC4x3 registration, including the nonstandard-filename central editing suite. Two stale harness failures were corrected without production enforcement changes; the complete current registered run passes **388 tests / 11,570 assertions across 27 suites**, ten gated skips, zero failures (`registered-test-scope.md`). Latest focused glTF work adds bounded byte copying/admission and interior cancellation with 55 tests / 161 assertions passing. `current-state.md` records the latest complete boundaries and active fleet assignments.
