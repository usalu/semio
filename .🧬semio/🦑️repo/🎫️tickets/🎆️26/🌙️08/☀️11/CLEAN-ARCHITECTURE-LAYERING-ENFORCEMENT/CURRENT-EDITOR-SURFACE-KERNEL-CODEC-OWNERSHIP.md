# Editor Surface Kernel Codec Ownership

Actual General library entries retain OS-kernel aliases: Editor names it store and mounts component; Surface names it dsl/store and publicly exports dsl::os_dsl before mounting paint, terrain, node_graph and tiled_map. Removing the normal Kernel declaration alone is incoherent. Editor color_bridge bodies already use canonical Value traits; their dsl_core import is not used in the bodies, irrespective of stale derive comments.

EditorError and NodeGraphError wrap store::PackError with actual From implementations. Both binary decoders call pack_rt::decode_wire_value then decode_pack_value fallback and dsl_value_to_json. Scene normalization calls scene_field_json_text in eight node-graph slots and the Editor helper. These are executable mounted obligations.

General Pack actual library mounts 🌱️value as public record and explicitly exports PackError/PackRefusal. Canonical decode_value_record_body_exact and document codecs return PackRefusal. Kernel pack_rt remains a distinct field/spec/options, pk: base64 and serde_json bridge with an upper PackError role. Replacement needs source-owned format policy and error-role decisions, not alias renaming. DslValue to serde_json conversion is the actual Value From implementation and is separable.

Surface imports DomainHover/DomainSelection/SelectionMethod through Kernel, but definitions are General Replication wire and General Interaction reexports them. Viewport2d is General UI viewport and Kernel reexports it. These are concrete neutral candidates. Surface public os_dsl, remaining module mounts, conditional tests and complete normal-provider declarations still need closure before Kernel retirement.

Proof: 🗑️generated/neutral-render-owner/independent-editor-surface-current-kernel-codec-frontier-1.json contains 13 full current caller/library/defining frames. Render remains NOT deletion/API Ready. No source/runtime changes.
