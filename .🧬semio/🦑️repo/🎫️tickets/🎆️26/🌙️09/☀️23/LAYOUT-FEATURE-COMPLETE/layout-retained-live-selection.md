# Live Retained Mutation Selection

Source inspection found two distinct Layout paths.

1. The interactive editor artifact lane is already explicit: LayoutPlayApp.build_artifact_store_one_item_preparation_factory returns store::mutation_apply_preparation_factory::<LayoutSnapshot,LayoutMutation>(). Framework store/📬️paged-one-item/🧬️mutation-apply/🦀️.rs binds M:RetainedMutation<S> and instantiates MutationApplyEdit<S,M>. Therefore adding the partial23-kind RetainedMutation implementation makes it the selected semantic owner for this existing interactive factory. There is no runtime auto-specialization question here. The remaining36 structural/splice/Drawing variants currently refuse in that owner, so full59 coverage is still required for complete normal Layout editing.

2. Document codec declaration remains bare: Layout declaration().document_codec::<EditorApp<LayoutPlayApp>>() calls DocumentCodecSpec::of<A>(), which constructs store::bare_artifact_codec::<A::Snapshot,A::Mutation>. It does not select an explicit retained native issuer source. No full retained document codec registration is claimed.

Interchange was notified directly and owns the correction/full semantic coverage. Root did not introduce a cold fallback or duplicate mutation adapter. Native execution remains unavailable, so this is a concrete source-selection finding rather than GUI behavior verified at runtime.
