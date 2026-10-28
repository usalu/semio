# IFC4 Semantic Inspector Integration Plan

Read-only 2026-10-21. High owns implementation; no source changes made here. The strongest reusable selected-object example found in stdio is PDF page canvas, not a complete world3d artifact example. Use its concrete inspector/retained wiring with the existing IFC world3d instance identity; do not claim a sibling world3d semantic inspector was located.

## Stable IDs and Schema-First Projection

IFC4 geometry already publishes instance id ifc-PRODUCT-REPRESENTATION and interactionId ifc-PRODUCT. Keep product ids as canonical decimal text after ifc- across UI and action arguments: Rust u64 parses directly; TypeScript BigInt parses directly; never Number(id). Require canonical no-leading-zero positive decimal, bound to u64, and verify the current snapshot contains the targeted product before editing. Support ids beyond 2^53 and inspect the fixture 9007199254740993 explicitly. One product may have multiple representation instances; selection maps them all to the same product.

Define neutral descriptor schema + Rust/TypeScript projection for supported semantic fields, target entity predicate, positional argument index, kind, optionality, localized label and edit capability. Start product Name/Description/ObjectType and type-aware Tag; do not assume all IfcProduct subtypes have Tag at the same index. Expose stable root /ifc-ID/{descriptor-key}, read-only id/type fields, and reference navigation without making entity id/type mutable accidentally. Property values should retain IFC unset/typed wrappers rather than flattening unsupported kinds to strings. GlobalId is a String grammar value with additional semantics; leave read-only until a proper IFC GUID validator exists.

## Concrete Reusable Framework and Sibling APIs

All source paths below are rooted at C:/git/semio.

- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/✏️editor/🖼️page/🎬️actions/🦀️.rs: install_object_interaction at178 constructs InteractionDefinition, GranularityDefinition, HoverSpec, SelectionSpec; installs InteractionRef::new(domain) on the page window. select_effect dispatches INTERACTION_SELECT_ACTION_ID with domainId, targets string containing JSON [{granularity:"object",id}], merge,method:"pick". Canvas pointer selection is an Effect::DispatchAction, never an artifact mutation.
- Same PDF page/🔎️inspector/🦀️.rs: field_input at129 binds ui::input.value(...).commit("blur").draft_target(stable target).publication_revision(revision).try_on_with(Trigger::Commit,ActionId::try_v1(controller,action),args). Framework supplies new value; object/field identity stays explicit in static args. The semantic draft target isolates per-object/per-field uncommitted draft state across rerenders.
- PDF editor/🦀️.rs render_with_request_context around225 gets interaction.selection(OBJECT_DOMAIN).ids.first() and calls render_inspector with doc snapshot, selected id, controller, window_kit_artifact_publication_revision(doc) and locale. Its bounded_native_editing_editor macro returns Emit{artifact_mutations,..}; it never writes snapshot state directly.
- PDF page/🪟️windows/🦀️.rs defines inspector_window_definition and three-pane document_layout (canvas/object/details). Generic snapshot details remain available alongside selected semantic fields.
- 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📜️declaration/👁️view/🔍️lane-views/🦀️.rs defines InteractionView with selection(domain), hover(domain,channel), active_granularity, active_mode. IFC editor original request render receives this already but ignores it. Carry selection into main scene and inspector rendering there, preserving original StepContext for geometry.
- Plugin 🪟️window-kits/🌍️world3d/🦀️.rs world3d_selection_json_with_granularity(method,ids,hovered,granularity) publishes admitted selection overlay. It does not itself create domain membership; declare interaction domain in AppDefinition/window.

## Exact Domain Topology Requirement

Declare IFC product InteractionDefinition with one product granularity and HierarchyProvider::Topology, not Flat, if deleted-id pruning is required. SelectionSpec can begin Single/Multiple, Pick and Replace/Invertive; use existing supported rectangle selection only after renderer domain picking is verified. Keep broadcast behavior aligned with other artifact domains.

Implement ArtifactEditor::interaction_topology returning protocol::InteractionTopology {domains:BTreeMap}; the product domain value is protocol::DomainTopology {ordered:Vec<TopologyNode>}, each node {id:"ifc-ID",granularity:"product",parent:None}. Exact constructors occur in plugin/🕹️interaction/🧭️topology/🦀️.rs at81,85. The framework refreshes topology per live document/config generation.

Plugin 🧵️instance/🖼️render/📥️staging/🧭️interaction/🦀️.rs comments around120–140 explicitly state Flat domains are skipped during membership pruning and retain stale ids. HierarchyProvider::Topology with root nodes enables deletion cleanup, including an empty topology after all products are deleted. Use product membership from descriptors/graph, not only successfully tessellated meshes: unsupported representation products still need semantic inspection/navigation.

## Reuse Shared Snapshot Details Without New Actions

High proposes an artifact-owned SnapshotDetailsProvider semantic root {"ifc-ID":{name,description,...}}, rendered in a separate inspector body using editing::render_snapshot_details_provider_revisioned(provider,revision,locale,controller,TreeWindows). This is viable with TWO explicit hooks:

1. Override SnapshotEditingEditor::snapshot_edit_is_admitted for recognized semantic paths. Default admission refuses them before reduction: editing/🛠️tool-job-factories/🦀️.rs snapshot_edit_value_is_admitted at143–208 calls snapshot.value_shape_at_path; SetValue line185 and Remove line193 require a real persisted snapshot path. Retained extent line282 calls E::snapshot_edit_is_admitted. Therefore snapshot_edit_special alone is insufficient.
2. In snapshot_edit_special, validate canonical semantic path + descriptor + actual current entity, then translate SetValue to existing IfcMutation::SetEntityArg with id and concrete index. Optional RemoveValue becomes SetEntityArg(value:Unset), never RemoveEntityArg (which shifts EXPRESS tuple slots). Unsupported semantic Insert/Move/Rename must refuse explicitly. The hook returns Some(vec) so the generic edit-rules resolver does not treat semantic pointers as snapshot pointers. Keep current complex-edit handling for non-semantic paths.

Trait implementation is in editing/🧰️editor-traits-and-retained-copies/🦀️.rs: default admission lines9–11, snapshot_edit_emit29 invokes special before edit-rules.resolve, then check_publication_limits. Retained snapshot factory reduction calls E::snapshot_edit_emit at tool-job-factories line308. Existing generic factory, publication/inverse/store preparation can therefore remain in use: output is ordinary SetEntityArg, whose inverse is already supported. No separate direct-state path or fresh native action is needed for this narrow slice.

Admission must NOT become permissive for every semantic-looking path. Validate supported event kinds and bounded path/value depth/size before returning true, and fallback to generic snapshot_edit_value_is_admitted for ordinary persisted paths. Special resolution should return a named fault for malformed/unknown semantic paths, not fall through and risk an unrelated interpretation. Binding canonical entity id rather than current selection ensures a draft for product A cannot accidentally edit product B when selection changes.

## Minimum Tests

Neutral descriptor/extraction/edit cases across Rust/TypeScript; third-party IfcOpenShell property extraction/edited export readback; exact wide-id selection and edit; two representations one product; reference navigation; readonly type/id refused; optional clear emits Unset without shifting args; invalid semantic descriptor/value refused; retained admission succeeds for semantic paths and remains bounded; generic snapshot details and complex constituents still edit; undo restores previous arg; selection changes preserve draft target; stale selection is pruned after product deletion; no selection shows localized empty prompt; cancelled retained edit publishes nothing. UI source assertions alone do not establish end-to-end interaction success; record native/runtime verification blockers honestly.
