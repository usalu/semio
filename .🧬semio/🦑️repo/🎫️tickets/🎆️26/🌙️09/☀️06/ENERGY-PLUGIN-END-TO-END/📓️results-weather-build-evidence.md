# Results and Weather Build Evidence

The native annual four-case comparison remains verified in `📓️native-annual-comparison.md`: all 16 acceptance metrics pass using actual EnergyBatchRun and EnergyPlus SQL. No physics or tolerance relaxation was used.

The publication source oracle now follows the actual Rust owned module tree (excluding cfg(test) fixtures). Its existing negative checks remain. Bun publication oracle passes with independent Ajv/SQLite/Buffer vectors.

## Mesh Ownership Compilation Repair

The canonical mesh decoder State retains a ControlledRetirement containing a non-Sync erased retirement cursor. Its derive emitted an impossible Sync receiving constructor. The State now uses the existing artifact_retire_struct macro over every original field; no unsafe Send/Sync or stronger generic requirement was introduced. Focused existing original_mesh_canonical_decoder tests pass 2/2, including failed source custody and exact physical allocation/release receipts. Log: generated mesh-retirement-test.log.

## Member Capability Compilation Repair

The already authored ReferenceSpaceMember facet was not included at its owning crate, while its 19 methods were duplicated in the production SpaceMember trait, implementation, NoMembers and enum macro. The facet is now included from its existing canonical source and exported. The duplicate cold methods were removed; original publication, immutable read and retirement methods remain in SpaceMember. History timestamp reads use the actual source log directly. No production Clone requirement was added. The first focused check reaches only 11 native engine wrapper errors caused by broad Clone impl bounds; integration owns those engine wrappers. The reference implementation itself compiles.

## Work in Verification

Real retained linked EPW requests, original canonical Pack decoding, actual artifact URI selection and clearing, and fail-closed bound-weather handling are implemented but their tests have not yet passed the shared Store compilation frontier. Production host LinkResolve routing remains a parent-owned outstanding task. Do not treat source fixtures or refusal alone as production browser resolution.

Heating/cooling/facility progress counters and localized panel rows are implemented. Full named hourly publication and bounded results navigation remain in progress. No claim of their verification is made here.

## Original presentation and native job boundary (2026-10-21)

The common presentation suite passed five actual ownership tests. The standalone `🧪️native-tool-run` harness references the checked-in production native receiver source by path; it passed two tests. The first receives 65,565 bytes over two ranged publications, preserves all nine counters and immutable partial captures, checks fixed reads across a 64 KiB page boundary, and retires producer/capture in both orders. The second drives a genuine `InteractiveJob` and `RetainedJobPublication` into `PreviewReady`, retains its original descriptor and physical producer pages, publishes canonical tick data through the native presentation slot, acknowledges/removes the original descriptor with funding, and verifies a genuine subsequent producer step reaches `Complete`. Both tests compare each turn's actual heap births/releases with the original caller receipt, use a finite total source stock, and close the original decoder. This verifies the production receiver/presentation boundary; it does not yet prove the mounted application ledger or browser UI.

The mounted driver now lends its original context to that receiver rather than constructing another root cancellation or payload authority. Native presentation retirement precedes producer retirement. Owned command admission captures one funded strong presentation alias instead of emitting a cloned legacy result view. Actual model rendering uses the original retained ESF2 source and fixed binary-search reads for each face; it builds no result decode map or repaint quantile sort. The renderer caption uses the actual selected English/German locale.

The full SDK compile remains pending the shared ownership publication frontier. A focused check currently stops at the integration agent's newly declared native transient publication source before checking mounted ToolRun integration. No mounted/browser pass is claimed. Named hourly series publication is still outstanding.

The native receiver harness now passes **three** tests; the added case cancels during native tick receiving, hides discarded views immediately, preserves the same physical producer source until paid cursor closure, and starts the next generation using the same native slot. Common payload tests pass **three** tests with the actual trace heap allocator enabled; presentation tests remain **five** passing. The results read oracle passes against independent SQLite dense ranks, Buffer wire values and Ajv validation for **20,003 rows / 480,148 bytes / eight chunks**, including forty repeated repaint rounds and explicit English/German captions. The actual scene renderer regression is authored but remains unrun pending SDK compilation.
