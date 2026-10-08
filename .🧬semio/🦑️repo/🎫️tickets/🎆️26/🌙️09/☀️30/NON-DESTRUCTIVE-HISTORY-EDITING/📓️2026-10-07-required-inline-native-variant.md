# Required Inline Native Variant Role

The current strict35 native editor cohort j completed failed at 2026-10-07T07:02:24.164Z with three Layout role diagnostics and zero named editor assertions. PageFrameAdded physically owns inline Frame, a native tagged DslEnum. Its required statement field was incorrectly classified as scalar because the derive handled only boxed, optional and repeated statement owners. The bare native owner is preserved.

The domain-neutral derive now owns RequiredInlineStatements, classified after optional/boxed/list cases. It uses native DslVariants directly for ordinary and controlled construction/projection, exact-one cardinality, borrowed schema metadata, retained ordinal/key views and decoded variant retirement. It adds no native Box around the physical field and no representation adapter. Existing boxed/optional/repeated roles remain separate. Core confirmed no shared derive overlap.

Language-neutral JSON2020 schema and literal fixture admit two variant values (floating rectangle and NUL/Unicode text) and refuse six absent/null/list/unknown/extra-field cases. Independent Ajv2020 validates the same authored output. Actual source RED71966 ran 0/1 with 11 expects and 114ms, solely absent new role after all neutral vectors validated; full GREEN72421 ran 4/4,36 expects,99ms at07:06:09.967Z.

Native tests require independent syn parsing and actual field-role selection for inline/boxed/optional/repeated declarations. The original typed owner law requires print/parse roundtrip, controlled encode/decode, borrowed output measurement, zero/two-value refusal and explicit cancellation. Native72650/72655 and syntax72658 are pending. No native compile, generic editor assertion or allocator-bound production claim follows from source GREEN.

Physical source owners: DSL schema/derive main; adjacent neutral fixture/schema and ownership/native tests; schema native required-inline unit test and canonical unit module registration; Layout PageFrameAdded statement annotation. Reports and captured logs remain ticket-owned.


Actual compiled boundary: emitted role law72655 passed1/1 in15ms with visible independent-syn DEBUG. Initial runtime72650 stopped before assertions at existing public RefusingField missing BorrowedDslField. The narrow fixture now declares exact borrowed Text shape matching its real DslField::shape; all refusal assertions remain. After exact controller/selector absence, runtime74309 passed1/1 in10ms with visible actual typed native roundtrip/refusal/cancel/measurement DEBUG; Nx18.3s. Syntax72658 parsed4 actual Rust owners. This is a real generic role typing/runtime boundary, not a Layout editor assertion receipt.

Puzzle focused18 domain e67466 completed failed2026-10-07T07:04:45.793Z before assertions at eight unrelated current Store member-group integration errors. Core/UI are coordinating genuine missing group preparation. No native candidate/constructor proof is inferred. Current strict35 successor waits this concrete compile prerequisite, with all103 laws preserved.


The full regressions exposed separate old seams: native75482 executed105 laws,102 PASS/3 FAIL,1.302s. The three failed retained writer nested/container/depth laws call cold owned_retirement.close_step(1,3), and a real native Box demands more than three indivisible release bytes. These original tiny grants and all assertions are preserved; no budget was raised and no precredit was added. Source review confirms the Value raw frontier now truthfully refuses such release demand; the old erased writer retirement/finite Vec+Box schema owners do not have a proof of arbitrary exact bounded closure. This remains a distinct incomplete obligation.

Full emitter75483 executed2 laws,1PASS/1FAIL29ms. The old neutral composition fixture omitted actual required BorrowedDslRecord/BorrowedDslField/BorrowedDslVariants roles; its parsed emission assertion found the mismatch. The authored fixture now states these explicit generic metadata contracts and the actual extra two borrowed variant projection methods, preserving all ordinary methods/product-boundary exclusions. Its corrected complete native receipt is pending; no regression GREEN is claimed yet.


Corrected full neutral emission fixture now has an actual complete native receipt: full emission-b81648 passed at07:58:04.346Z, with both current native emission laws and successful DEBUG output. This verifies the corrected native borrowed trait/variant projection expectations together with RequiredInlineStatements emission. It does not repair or certify the three existing raw retained-writer3-byte closure failures; those remain explicit105-law regression obligations.
