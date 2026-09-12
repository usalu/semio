# Generator Renderer Prerequisites

The coordinator read all four current generator command sources and executed the existing read-only graph/schema/UI checks before the queued Sol extraction. No product source or generated projection was changed by this investigation.

## Executed Production Checks

| Command owner | Existing direct route | Observed result |
| --- | --- | --- |
| Graph Rust package | `check-generated` | Nine manifests fresh; existing Bun suite passed three tests and 70 assertions, including declared output identity, exact publication/symlink refusal and native generated registry loading. |
| Schema Rust package | `check` | All current projections byte-fresh for 58 entity kinds; source SHA `f237cdd640bd1ac2b4546726f78f33c90aaf0c2ed11e1f6115294c8e9c3be242`. |
| UI Rust package | `check` | Both current projections byte-fresh for two locales and two terminologies. |

These were direct package command routes, not newly executed registered Nx or native Rust tests. Actor was inspected without executing its destructive typegen cleanup or unbounded preview process. Graph's check invokes its existing isolated publication controls; it does not publish the live graph output.

## Native Declaration Closure

Installed TypeScript parsed all four sources with zero syntax diagnostics. Its symbol checker, restricted to each current source, produced the exact local declaration closures retained in `🗑️generated/coordinator/generator-renderer-declaration-closures.json`. Imports are external boundaries; overlapping closures are not summed as independent work.

- Graph: catalog admission two declarations/52 lines, rendering twenty/309, output inventory two/16, publication five/43. The existing suite imports the parser and writer from the command file and must bind their real neutral owners after extraction.
- Schema: source reading four/12; TypeScript renderer five/36; Rust renderer five/34; Go renderer five/49; complete generated-target plan eleven/117. The common provenance/source authority should remain shared rather than duplicated across renderers.
- UI: source reading five/7; Rust enum rendering two/44; Rust projection four/58; TypeScript projection three/22; complete plan ten/88. Preserve actual axes and consumer identity while separating source loading, projection and publication.
- Actor: output coordinate one/3; native exporter execution two/9; generation class closure four/21; preview class closure three/26. This source is not proven task-only by its 89-line size: preview constructs an owned output protocol, inventories stale output and executes Cargo in temporary roots, while generation removes all sibling output entries. Those are actual publication/preview behavior requiring precise ownership and bounded cancellation/deadline treatment.

All line counts and source hashes in the raw report are review provenance only. Do not turn them into permanent source-body snapshots.

## Additional Schema Type Consumer Defect

The inspected generated TypeScript entity catalog imported its declared `EntityKind` and `EntityKindCatalog` from `../🟦️`, despite living at `schema/🤖️generated/🏷️entity-kinds/🟦️.ts`. The producer emitted that same relative string, so byte freshness alone preserved the bad binding. Native TypeScript module resolution is captured in `🗑️generated/coordinator/schema-entity-type-import-resolution.json`; the correct schema owner is two parents above the generated leaf. Root subsequently repaired the producer and regenerated the projection: native resolution now reaches the real schema owner and the existing six-case entity suite passes. See `📓️coordinator-entity-catalog-type-binding-repair-2026-09-12.md` for exact changes and registered verification status. Preserve that fixed producer/consumer relationship during extraction.

## Pending Acceptance

Use `📓️generator-renderer-script-extraction-packet-2026-09-12.md` for scope, including the authored Rust manifest conversion and genuinely generated Rust entity catalog rename. Retain canonical output protocol/byte provenance and exact output membership. Re-read concurrent source state, add portable ownership/type controls first, execute the real producer and native consumers, and audit every new ancestor context. No generator extraction or full taxonomy acceptance is claimed here.
