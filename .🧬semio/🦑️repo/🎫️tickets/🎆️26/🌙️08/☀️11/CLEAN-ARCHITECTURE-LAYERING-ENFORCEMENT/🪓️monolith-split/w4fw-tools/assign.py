import re
from cfg import *
ID = set('ArtifactHandle WindowHandle AssetHandle CapabilityToken PluginInstanceId InvocationId ActionId CommandId AppInstanceId SchemaVersion WindowKindId Rights ArtifactKind Scope CapabilityRequirement Capability CapabilityGrant ActionDef ActionInvocation CommandInvocation RequestId CapabilityId CapabilityRequest BrokerCapabilityGrant CapabilityChange QuotaSchema QuotaTree QuotaBreach FailureAction ExtensionDescriptor extensions_extending scope_capabilities_to_parent ActivationEvent Budget TurnStatus Usage InverseMutation KernelMutation EditRef UndoGroup AppEvent PhysicalSize Appearance WindowEvent WindowKindDef WindowInput RequestOutcome'.split())
HIST = set('history_severity_serde HistoryTransaction HistoryMutationMessage HistoryMutationEntry HistoryTimeTravelStage HistoryTimeTravelReview HistoryTimeTravelProblem HistoryTimeTravel HistoryEntry history_entry_count HistoryReprojectionKind HistoryReprojection HistoryPatch HISTORY_NOTICE_LABELS history_notice FaultNotice FRAMEWORK_FAULT_NOTICE_LABELS framework_fault_notice fault_notice HISTORY_REPROJECTION_LABELS HistoryReprojectionStatus history_reprojection_status'.split())
FAULTDEF = set('FaultNoticeDefinition FaultNoticeError is_fault_notice_code fault_notice_placeholders validate_fault_notices fault_notice_text fill_fault_notice fault_notice_definitions'.split())
UIK = set('UiDirtyScope UiDirtySection union_body_keys ui_patch_turn_bytes with_ui_turn_patch_retire_arena with_ui_turn_patch_transport_arena'.split())
JOBS = set('FRAMEWORK_RESERVED_JOB_KIND SPAWNED_JOB_STEP_CEILING SPAWNED_JOB_FUEL SPAWNED_JOB_DEADLINE_MS SpawnedJobStep SpawnedJobCompletion SpawnedJobDriveError spawned_job_completion IconRenderExportItem MEDIA_EXPORT_BASE64_ENCODING MEDIA_EXPORT_UTF8_ENCODING MediaExportEncodingError media_export_bytes MEDIA_VIDEO_RENDER_CAPABILITY VIDEO_RENDER_PROGRAM_SCHEMA VIDEO_RENDER_PROGRAM_MAXIMUM_EDGE VIDEO_RENDER_PROGRAM_MAXIMUM_FPS VIDEO_RENDER_PROGRAM_MAXIMUM_FRAMES VideoRenderPath VideoRenderImage VideoRenderOp VideoRenderScene VideoRenderRun VideoRenderProgram VideoRenderProgramError video_render_unit video_render_path_is_well_formed video_render_image_url_is_admitted video_render_crop_is_admitted VideoRenderEncoderTier VideoRenderJobOutcome VideoRenderJobEvent VideoRenderJobRow VideoRenderJobEventError VideoRenderJobLedger IMPORT_CHUNK_BYTES IMPORT_ARGUMENT_PAYLOAD IMPORT_ARGUMENT_NAME IMPORT_ARGUMENT_CHUNK IMPORT_ARGUMENT_CHUNK_COUNT IMPORT_ARGUMENT_INDEX IMPORT_ARGUMENT_TOTAL ImportChunk import_payload_chunks import_chunk_arguments IMPORT_STAGING_MAXIMUM_CHUNKS IMPORT_STAGING_MAXIMUM_BYTES IMPORT_STAGING_RUNS ImportStagingRefusal ImportArguments ImportRun ImportStaging'.split())
MEDIA = set('MediaClass MediaForm OsMediaCapability ArtifactKindSpec HostedArtifactKind MediaType MediaWireFormat MediaPortDirection PortMultiplicity MediaPortSpec MediaCompat MEDIA_FORM_CONVERSIONS media_types_compatible ArtifactPresentation AppIo Media MediaPayload MediaFingerprint MediaError MediaConverter fingerprint Part bytes'.split())
APPSURF_FROM_VIEW = set('ConfigSpec CommandVariantSpec CommandGrammar'.split())
VIEWMODEL = lambda n: n.startswith('VIEW_CONTEXT') or n.startswith('PUBLIC_INVOCATION') or n in ('ViewModel','TreeWindowRequest','ViewWindowInstance','ViewSessionIdentity','MAX_SURFACE_VIEW_CONTEXT_BYTES','MAX_SURFACE_BODY_KEY_BYTES','public_invocation_char_cost','public_invocation_string_pages','UiRefreshSection','UI_REFRESH_SECTION_KEYS','UI_REFRESH_SECTION_BODY_KEYS')
CONTRIB = set('TopicContribution Version VersionParseError VersionPin VersionPinParseError PluginDependency ContributedMutationSemantics ContributedMutationMetadata ContributedInferenceMetadata InferencePayloadContract InferenceCommitBinding InferenceArtifactBinding INFERENCE_ARTIFACT_PACK_BASE64 ArtifactContributionDescriptor ProgramContributionEntry parse_contributions absent SPACE_INDEX_TOPIC SpaceIndexContribution tree_pin'.split())
PLUGIN = set('PackageRole PluginManifest ArtifactKindChoice SurfaceAppChoice encode_artifact_kind_choice decode_artifact_kind_choice encode_surface_app_choice decode_surface_app_choice artifact_kind_choices DependencyGraphError validate_dependency_graph resolve_load_order find_cycle_members dependents ExecutionMode ExtensionPointDeclaration AssetDeclaration DescriptorEntry FileTypeContribution IoEntryDirection IoEntryDescriptor ComposerEntryDescriptor ContributionSet ASSEMBLY_FAILED_PLUGIN_ID ExecutionProtocol PackageDescriptor CanonicalDescriptorValue canonical_members AgentContributions'.split())
INPUTS = set('INPUT_LABEL_GLOSSARY_JSON input_label_glossary mutation_input_defs InputSchemaAudit mutation_input_audit registered_input_schema_document mutation_input_instance input_union INPUT_NUMERIC_TRANSPORT_SCHEMA_JSON FRAMEWORK_INPUT_SCHEMA_DOCUMENTS INPUT_SHAPE_ANNOTATIONS input_shape input_numeric_transport InputVariant INPUT_UI_KEYS INPUT_UI_NUMBER_KEYS INPUT_UI_VECTOR_KEYS INPUT_REF_DEPTH input_error input_pointer input_localized_text input_type input_inferred_reference_kind ResolvedInput InputSchemaReader input_string input_finite input_positive input_options input_option_source input_number InputSchemaResolver InputSchemaErrorCode InputSchemaError'.split())
ADDRESS = set('Keybinding Platform PlatformKeybinding ActionRef ActionAddress ActionInvocation UtilityRef CommandOwnerAddress CommandAddress CommandInvocation CommandInvocationView ToolRef is_element_id element_id_segment child_element_id UI_NAVBAR_ELEMENT_ID UI_FOOTER_ELEMENT_ID window_element_id panel_tab_element_id panel_tab_first_draggable_element_id'.split())
VOCAB_HIGH = set('framework_fixed_audience framework_fixed_audience_violation history_edit_action_definitions history_edit_hidden_arg history_edit_finalize_dialog app_declares_tool_run tool_run_action_definitions tool_run_action_description tool_run_action_arg start_introduction_action_definition start_tutorial_action_definition record_tutorial_action_definition'.split())
ACTIONS_NEEDS_APP = set('interaction_window_bodies_by_domain panel_leaf_body_keys interaction_action_definitions ReservedContribution no_reserved_actions no_reserved_window_actions no_reserved_dialogs no_fixed_audience reserved_app_actions reserved_window_actions reserved_editor_dialogs reserved_audience_violation'.split())
KERNEL_FILES = {'kernel','k_clipboard','k_content','k_message','k_cold'}
def assign_entity(f, n, kind='fn'):
    """cluster id for a named top-level entity (impls attach to their target by the caller); None: drop"""
    if kind == 'mod' and n in ('program_contribution_tree', 'intrinsic_media'): return None
    if kind == 'mod' and f == 'manifest': return 'manifest-actions' if n == 'initial_window_utility' else None
    if f in KERNEL_FILES:
        if f != 'kernel': return 'kernel'
        if n in ID: return 'kernel-identity'
        if n in HIST: return 'kernel-history'
        if n in UIK or n.startswith('UI_TURN_PATCH') or n.startswith('UiTurnPatch') or n.startswith('close_ui_turn_patch'): return 'kernel-ui'
        if n in JOBS: return 'kernel-media-jobs'
        return 'kernel'
    if f == 'abi': return 'abi'
    if f == 'action_bus' or f == 'platform': return 'action-bus'
    if f == 'source_projection': return 'source-projection'
    if f.startswith('interaction'): return 'interaction'
    if f == 'root': return None
    if f == 'm_media': return 'manifest-media'
    if f in ('m_canonical', 'm_space'): return 'manifest-contribution'
    if f == 'm_utilinit': return 'manifest-actions'
    if f == 'onboarding': return 'onboarding'
    if f == 'ta_actions': return 'time-travel-actions'
    if f == 'tr_actions': return 'tool-run-actions'
    if n in FAULTDEF: return 'kernel-history'
    if f == 'm_view':
        if n in APPSURF_FROM_VIEW: return 'manifest-app-surface'
        if n in MEDIA: return 'manifest-media'
        if VIEWMODEL(n): return 'manifest-view-model'
        if n in PLUGIN: return 'manifest-plugin'
        return 'UNASSIGNED'
    if n in CONTRIB: return 'manifest-contribution'
    if f == 'm_plugin': return 'manifest-plugin'
    if f == 'm_argschema':
        if n in INPUTS: return 'manifest-mutation-inputs'
        if n == 'default_action_icon_id': return 'manifest-actions'
        return 'manifest-arg-schema'
    if f == 'm_actions':
        if n in ACTIONS_NEEDS_APP: return 'manifest-app-surface'
        return 'manifest-actions'
    if f == 'manifest':
        return 'manifest-address' if n in ADDRESS else 'manifest-actions'
    if f == 'm_surface':
        return 'manifest-arg-schema' if n == 'AppRole' else 'manifest-app-surface'
    return 'UNASSIGNED'
# test mounts: (file, mod) -> cluster ; None = drop
TESTS = {
 ('root','test_allocation'): None, ('root','schema_metadata'): 'FACADE', ('kernel','spawned_job_drive_tests'): 'kernel',
 ('abi','tests'): 'abi', ('action_bus','wire_retirement_tests'): 'action-bus', ('action_bus','tests'): 'action-bus',
 ('source_projection','tests'): 'source-projection', ('platform','tests'): 'action-bus',
 ('interaction','tests'): 'interaction', ('interaction_gesture','tests'): 'interaction', ('interaction_schema','tests'): 'interaction',
 ('kernel','service_operation_tests'): 'kernel', ('kernel','media_export_encoding_tests'): 'kernel',
 ('kernel','video_render_program_tests'): 'kernel', ('kernel','video_render_job_tests'): 'kernel-media-jobs', ('kernel','file_open_import_tests'): 'kernel',
 ('kernel','ui_dirty_scope_tests'): 'kernel-ui', ('kernel','history_patch_tests'): 'kernel-history', ('kernel','history_notices_tests'): 'kernel-history',
 ('kernel','fault_notices_tests'): 'kernel-history', ('kernel','framework_notices_tests'): 'kernel-history', ('kernel','history_reprojection_tests'): 'kernel-history',
 ('kernel','return_content_dialect_tests'): 'kernel', ('kernel','return_content_framing_tests'): 'kernel', ('kernel','return_content_message_tests'): 'kernel',
 ('kernel','return_source_inventory_tests'): 'kernel', ('kernel','return_source_entries_tests'): 'kernel', ('kernel','return_source_entries'): 'kernel',
 ('kernel','ui_turn_patch_tests'): 'kernel-ui', ('kernel','extension_activation_tests'): 'kernel-identity',
 ('manifest','media_vocabulary_tests'): 'manifest-media', ('manifest','app_label_tests'): 'manifest-app-surface', ('manifest','example_picker_tests'): 'manifest-app-surface',
 ('manifest','resolved_host_context_tests'): 'manifest-view-model', ('manifest','argument_retirement_tests'): 'manifest-mutation-inputs',
 ('m_canonical','tests'): 'manifest-contribution', ('m_argschema','number_facets_tests'): 'manifest-arg-schema', ('m_argschema','mutation_inputs_tests'): 'manifest-mutation-inputs',
 ('m_actions','reserved_contribution_tests'): 'manifest-app-surface', ('onboarding','tests'): 'onboarding', ('ta_actions','tests'): 'time-travel-actions', ('tr_actions','tests'): 'tool-run-actions',
 ('m_surface','dialog_choices_tests'): 'manifest-app-surface', ('m_surface','fault_notices_tests'): 'kernel-history',
 ('m_plugin','plugin_dependency_tests'): 'manifest-plugin',
 ('m_view','window_view_context_tests'): 'manifest-view-model', ('m_view','view_context_capacity_tests'): 'manifest-view-model', ('m_view','view_context_integer_carrier_tests'): 'manifest-view-model',
 ('m_view','public_invocation_capacity_tests'): 'manifest-view-model', ('m_view','ui_refresh_section_tests'): 'manifest-view-model',
 ('m_view','package_descriptor_value_codec_tests'): 'manifest-plugin', ('m_view','agent_contributions_tests'): 'manifest-plugin',
}
RENAME_TESTS = {('m_surface','fault_notices_tests'): 'definition_fault_notices_tests'}

REGION_CLUSTER = {'🧩️ReservedContribution': 'manifest-app-surface'}
