# BIM And FEM Late Caller Fault Independent Audit

Confirmed P2 in actual endpoints after shared graph capture correction. Authored cross-platform bim-fem-progress-fault-probe.ts uses existing actual exchange fixture drivers and connected FEM fixture. Generated/bim-fem-progress-fault.json records12actual runs. No production edits or native execution claim.

All four routes swallow original caller Error and IfcStepError(exchange.number) in their late semantic catches: BIM→IFC bim-sites; IFC→BIM second bim-import callback; FEM→IFC second fem-export callback; IFC→FEM fem-materials. They resolve unsupported/refusal diagnostics rather than reject exact caller value. Original caller IfcStepError(io.cancelled) rethrows same object in each route, showing current catch classification depends on code rather than caller authority. Eight swallowed faults, four exact cancellation-shaped rethrows.

Existing structural.ifc fixture refused before chosen FEM material phase, so the final probe instead obtains real supported IFC source from actual connected FEM exporter before attaching import control. Final runs all hit intended phases; no absence of callback was counted as success.

Affected catches: bim.ts66/142 and fem.ts126/250. Shared exchange-source scopes preserve failures only while source capture runs; outer endpoint semantic callbacks still need original scope across entire operation, and must rethrow caller failure before domain diagnostics/cancel-code conversion. Use same captureIfcIoControlScope rule already applied to triangles, with tests for undefined/primitive/IfcEditError(cancelled) as well as these actual Error/domain-shaped cases. Preserve genuine domain refusals and signal cancellation outcomes; no broad class-only rethrow workaround.

This is a narrow original fault identity finding. It does not validate complete BIM/FEM semantics, original units/control traversal or native parity. Parent owns unit/traversal refactor; this audit modified no production source.

## Pre-fix integration review

Current endpoint source still uses captureIfcIoControl without caller-failure scope. Minimal complete correction must start scope before external/IFC capture, not only late semantic try/catch. captureIfcExternalSource currently catches IfcEditError and may resolve unsupported or translate cancelled before outer catch sees it. Therefore check scope.rethrowCallerFailure immediately after awaited source capture even if it returned no artifact, and on capture rejection, or scope the generic external helper itself as done for exchange-source. Retain late catch rethrow before all class/code conversion. Existing actual12 baseline is retained; no post-fix execution was claimed at this point. Parent notified before implementation.

## Actual endpoint and source-capture closure

Reread stable fix: allfour entrypoints create original scope before capture; scope.run rethrows caller failure on both resolved source outcome and rejected source work. external-source itself scopes callback before domain conversion. Every semantic catch rethrows scoped caller failure first. Separate failed boolean preserves undefined and original class-shaped values; genuine domain diagnostics and original signal cancellation still follow their existing branches.

Independently reran original12latephase probe:12/12 hit intended callback and reject exact original object; before output preserved, after in bim-fem-progress-fault-after.json. Added cross-platform bim-fem-capture-fault-probe.ts with24actual constructed source cases across4routes×Error/IfcStepError/IfcEditError/IfcEditError(cancelled)/undefined/number.24/24 hit source capture and reject exact original value; all24 source models/snapshots remain byteword-equivalent under BigInt-aware before/after comparison. Generated capture-fault-after JSON records results. Reported P2 is closed at source and these actual endpoint probes. Root broader combined123/708 remains parent evidence, not rerun here.
