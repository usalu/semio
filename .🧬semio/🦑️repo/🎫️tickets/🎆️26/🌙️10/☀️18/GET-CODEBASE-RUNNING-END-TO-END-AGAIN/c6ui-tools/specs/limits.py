from specs.lib import *
F = U + '/🧬️contract/🛡️limits/🦀️.rs'
CH = [
    ch('📏', 'document-limits-and-text-estimates', 'document_limits_and_text_estimates', [(21, 253)],
       'Document limits and text estimates: the `UiDocumentLimits`, the intake step constants, the patch byte estimate, the text byte counts and the component, tree toolbar and row target validity checks.'),
    ch('🚫', 'contract-violations-and-validation', 'contract_violations_and_validation', [(257, 529)],
       'Contract violations and validation: the `UiContractViolation` catalogue and the snapshot validation walk.'),
    ch('🩹', 'patch-rejection-and-apply-arena', 'patch_rejection_and_apply_arena', [(533, 737)],
       'Patch rejection and apply arena: the `PatchRejection`, the apply phases, handles, retirement and slots and the process-wide patch apply arena.'),
    ch('🏭', 'patch-apply-producer', 'patch_apply_producer', [(739, 1164)],
       'Patch apply producer: the apply step and validate frame and the resumable `UiPatchApplyProducer`.'),
    ch('🎁', 'patch-apply-outcomes-and-retirement', 'patch_apply_outcomes_and_retirement', [(1166, 1349)],
       'Patch apply outcomes and retirement: the producer drop, the apply outcome, the rejected patch, the snapshot, patch and validation retirement and the quota kinds.'),
    ch('🔧', 'patch-ops-and-quotas', 'patch_ops_and_quotas', [(1351, 1470)],
       'Patch ops and quotas: the patch application, the single operation application, the mutation helper, the children and text quotas and the subtree removal.'),
]
SPEC = dict(orig='limits.rs', file=F, children=CH, wiring='block', external=mounted(F, orig='limits.rs'))
