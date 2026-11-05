# Host Staged Action Integration

The existing StagedFormSession and OwnedStagedControls enforce exact original resident node, publication, controller scope, token generation, locale and window provenance. Their constructors are currently used by tests only. Session executionArgs currently returns the raw draft argument object.

ActionDefinition already owns optional staging metadata and with_staging(body_key). Ordinary host action routing does not consume this metadata. Generic action argument rails render static controls and execute their effective arguments independently.

The reactor StagedAuxiliaryInput already owns request decoding, actual window context capture, controller donation, render source transfer and bounded retirement. Its module explicitly declares STAGED_AUXILIARY_TRANSPORT_ADMITTED=false; there is no actual admitted caller. Enabling this constant alone would not implement the transport. Real actor/Pack ingress and resident auxiliary publication must connect before claiming completion.

Schema-first typed execution contract: required Status.commitArgs is null while unready and an argument object while ready. Source-owned native representation is Option<DslValue>; no omitted field or draft fallback. The exact current resident payload is the only executable argument owner. StagedFormResponse remains the generic field model. Neutral commit-cases.json plus independent Ajv test and a session regression were authored before production changes. Focused red handle 49844 is waiting for ordinary machine capacity; no limits were changed.

Norm producer ownership belongs to runtime_full. It will mark insertItem and a distinct switchVariant action with norm.inspector.form, derive actual mutation schemas and typed candidates, and publish Some commit_args only after full native candidate validation.

Native matrix and scalar/UI gates remain pending new core/form coherence. Original shared native baseline was 152 passes and one diagrams duplicate-key failure; stable diagram text keys were repaired but not yet reverified.
