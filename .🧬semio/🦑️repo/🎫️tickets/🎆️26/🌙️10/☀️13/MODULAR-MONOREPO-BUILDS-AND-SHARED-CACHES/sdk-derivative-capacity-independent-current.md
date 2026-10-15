# SDK Derivative Capacity Independent Audit

Read-only source review on 2026-10-15. No builds, Originals, runtime probes, or tests executed.

## Findings

No concrete production defect established in the current capacity issuer. The private issued map authenticates object/store identity; synchronous publication and copy consumption prevent concurrent double spending; real registry custody is checked before admission and copying. Accepted copy custody deliberately outlives the publication callback while the pending copy drains, and both callback failure paths settle accepted work before retiring their scope.

## Actionable Validation Gap

The physical derivative test in `sdk/🎟️capacity/🧪️tests/📤️derivative/🟦️.ts` exercises successful detached work but does not exercise the failure branches implemented at `sdk/🎟️capacity/📥️physical/🟦️.ts:29` and `:43-44`. Add an actual accepted copy followed by a publication callback throw, and independently an issuer callback throw after starting admitted work. Assert the original callback error is preserved, accepted work has settled before the outer rejection, and escaped copy/quote use is refused afterward. Current successful drain coverage does not falsify premature release on those error branches.

## Integration Limits

The projection implementation at `sdk/📥️source/🔗️derivative/🟦️.ts` was absent when inspected, so projection authentication, single-use enforcement, source-guard retention, immutable material validation, and its copy-side use of the custody guard remain pending integration review. This is concurrent work, not a defect finding.

The existing whole-snapshot issuer at capacity physical lines 52-53 still recognizes shared material through the binary-only verified map. General immutable shared-source hardlinks require the forthcoming verified source index integration; they are not considered validated by this audit.

Transient test import corrections and the detached test's fresh second projection are already assigned to Root and are not reported as unresolved findings. Formula tests compare Ajv acceptance and SQLite arithmetic, while the physical test uses independent Python SQLite exclusion and Node module/inode observations; all remain unexecuted here.
