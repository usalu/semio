# Renderer Slot Layout Owner Review

The retained whole Renderer56959 runtime RED measures EngineSurfaceSlot at 85,696 bytes. The fixture declares 82,656 bytes. A fixture correction is **not yet justified**: the current physical layout is confirmed independently, but the exact previous constituent responsible for the 3,040-byte delta has not been established. No production, fixture, budget, generator or compiler operation was performed during this review.

## Exact current constituents

LLVM objdump of the retained compiled arm64 Renderer test binary independently exposes EngineSurfaceRegistry::default's slot factory. Its two memcpy lengths are 29,064 (Option<EngineSurface>) and 56,360 (Option<EngineSurfaceRetirement>); generation is 8 bytes, Option<EngineSurfaceId> is 260 bytes, exhausted is 1 byte and trailing alignment is 3 bytes.

`29,064 + 56,360 + 8 + 260 + 1 + 3 = 85,696`.

The three table measurements are EngineSurfaceRegistry capacity256/element85,696/owner32, StagedEngineScenes256/352/24, EngineCanvasBuildContext256/384/72. The latter two are exact against the fixture. Registry ownership remains a Box<[EngineSurfaceSlot;256]> made through boxed_fixed_slots; the 32-byte owner has not become an inline table.

The physical table is 21,938,176 bytes. The stale declared table is 21,159,936 bytes, a 778,240-byte difference at capacity256. Neither is the permitted thread stack. The 1,048,576-byte bounded thread stack, 2,097,152-byte pool stack and 65,536-byte conversion threshold stay unchanged.

## Owner relationships and causal limits

EngineSurfaceSlot contains both optional live EngineSurface and optional EngineSurfaceRetirement. The latter retains source hosts plus close owners concurrently. NodeGraphEngine stores GraphHost or FlowHost inline; NodeGraphEngineRetirement stores GraphHostRetirement or FlowHostRetirement inline. FlowHost and FlowHostRetirementState each embed Option<FlowStore>, and FlowStore is the first-party ArtifactStore specialization.

The live and retirement EngineCanvas type declarations, FlowHost declarations and GraphHost declarations are byte-identical to their declarations in the early retained complete OS source capture. Therefore a recent direct EngineSurfaceSlot/EngineSurface/EngineSurfaceRetirement/FlowHost/GraphHost field addition is not supported by that comparison. Nested owners can change these Rust layouts without changing the containing declarations. The Store owner does have current foreign field changes (mirror/prefix dirt, fold frontier, typed replay budgets); its complete old/current bodies are retained separately. This is a candidate dependency chain, **not a proved attribution of the exact 3,040-byte increase**. The early OS capture is not established as the fixture's authored layout epoch.

The original September boxed-slot report recorded an even earlier 67,528-byte element and 16-byte owner. It explains the heap construction requirement and third-party objdump methodology, but does not establish the intermediate 82,656-byte constituent layout.

## Schema, oracle and execution scope

The full shared JSON fixture contains nine exact physical table rows and the three unchanged scalar limits. No separate boxed-fixed-slots JSON schema was located in its owner; the TypeScript interface types the fixture. The language-neutral oracle validates arithmetic, unique owners, safe-positive counts, heap structure and thread limits. Its four Vitest laws do **not** independently derive Rust field layout. Treating that arithmetic oracle as proof of a changed Rust constituent would be incorrect.

The shared Rust assertion checks exact measured rows first, then structural inequalities, then builds all three owners on the explicitly bounded stack. Renderer56959 failed the first equality check. It did not execute the bounded-stack construction phase, so current bounded-stack success is unproven.

Before a manual canonical fixture update, retain measured constituent layouts for the relevant nested source and retirement owners in the existing whole law and compare them to an established previous compiled layout or owned field change. Any resulting fixture correction must preserve all nine table rows, exact equality, heap construction, and all scalar limits. No blind fixture proposal was authored in this review.

## Evidence

- Original native full handoff: `🗑️generated/current-native-worker/sqlite-native-ledger-production-renderer-1-slot-layout-runtime-red-handoff-1.json`.
- Full current/early-source EngineCanvas, FlowHost and GraphHost pairs: `🗑️generated/current-native-worker/renderer-slot-layout-root-and-host-current-pairs-1.json`.
- Full Store pair, early capture explicitly identified: `🗑️generated/current-native-worker/renderer-slot-layout-store-current-pair-1.json`.
- LLVM slot factory disassembly: `🗑️generated/current-native-worker/renderer-slot-layout-slot-closure-disassembly-1.log`.
- Retained symbol catalog and bounded retirement disassembly: `renderer-slot-layout-nm-1.log`, `renderer-slot-layout-retirement-disassembly-1.log` in the same generated directory.

Native lane remains idle awaiting Root's fresh source authority.
