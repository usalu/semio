import json, re, sys, collections
E = json.load(open('ents.json')); ents = E['ents']; edges = E['edges']
NOISE={'_','bytes','Part','absent'}
edges={i:[j for j in v if ents[j]['name'] not in NOISE] for i,v in edges.items()}
KERNEL_FILES = {'kernel','k_clipboard','k_content','k_message','k_cold'}
ID = set('ArtifactHandle WindowHandle AssetHandle CapabilityToken PluginInstanceId InvocationId ActionId CommandId AppInstanceId SchemaVersion WindowKindId Rights ArtifactKind Scope CapabilityRequirement Capability CapabilityGrant ActionDef ActionInvocation CommandInvocation RequestId CapabilityId CapabilityRequest BrokerCapabilityGrant CapabilityChange QuotaSchema QuotaTree QuotaBreach FailureAction ExtensionDescriptor extensions_extending scope_capabilities_to_parent ActivationEvent Budget TurnStatus Usage InverseMutation KernelMutation EditRef UndoGroup AppEvent PhysicalSize Appearance WindowEvent WindowKindDef WindowInput RequestOutcome'.split())
HIST = set('history_severity_serde HistoryTransaction HistoryMutationMessage HistoryMutationEntry HistoryTimeTravelStage HistoryTimeTravelReview HistoryTimeTravelProblem HistoryTimeTravel HistoryEntry history_entry_count HistoryReprojectionKind HistoryReprojection HistoryPatch HISTORY_NOTICE_LABELS history_notice FaultNotice FRAMEWORK_FAULT_NOTICE_LABELS framework_fault_notice fault_notice HISTORY_REPROJECTION_LABELS HistoryReprojectionStatus history_reprojection_status'.split())
UIK = set('UiDirtyScope UiDirtySection union_body_keys'.split())
JOBS = set('FRAMEWORK_RESERVED_JOB_KIND SPAWNED_JOB_STEP_CEILING SPAWNED_JOB_FUEL SPAWNED_JOB_DEADLINE_MS SpawnedJobStep SpawnedJobCompletion SpawnedJobDriveError spawned_job_completion IconRenderExportItem MEDIA_EXPORT_BASE64_ENCODING MEDIA_EXPORT_UTF8_ENCODING MediaExportEncodingError media_export_bytes MEDIA_VIDEO_RENDER_CAPABILITY VIDEO_RENDER_PROGRAM_SCHEMA VIDEO_RENDER_PROGRAM_MAXIMUM_EDGE VIDEO_RENDER_PROGRAM_MAXIMUM_FPS VIDEO_RENDER_PROGRAM_MAXIMUM_FRAMES VideoRenderPath VideoRenderImage VideoRenderOp VideoRenderScene VideoRenderRun VideoRenderProgram VideoRenderProgramError video_render_unit video_render_path_is_well_formed video_render_image_url_is_admitted video_render_crop_is_admitted VideoRenderEncoderTier VideoRenderJobOutcome VideoRenderJobEvent VideoRenderJobRow VideoRenderJobEventError VideoRenderJobLedger IMPORT_CHUNK_BYTES IMPORT_ARGUMENT_PAYLOAD IMPORT_ARGUMENT_NAME IMPORT_ARGUMENT_CHUNK IMPORT_ARGUMENT_CHUNK_COUNT IMPORT_ARGUMENT_INDEX IMPORT_ARGUMENT_TOTAL ImportChunk import_payload_chunks import_chunk_arguments IMPORT_STAGING_MAXIMUM_CHUNKS IMPORT_STAGING_MAXIMUM_BYTES IMPORT_STAGING_RUNS ImportStagingRefusal ImportArguments ImportRun ImportStaging'.split())
def kernel_cluster(e):
    n = e['name']
    if n in ID: return 'kernel-identity'
    if n in HIST: return 'kernel-history'
    if n in UIK or n.startswith('UI_TURN_PATCH') or n.startswith('UiTurnPatch') or n.startswith('close_ui_turn_patch') or n in ('ui_patch_turn_bytes','with_ui_turn_patch_retire_arena','with_ui_turn_patch_transport_arena',): return 'kernel-ui'
    if n in JOBS: return 'kernel-media-jobs'
    return 'kernel'
MEDIA = set('MediaClass MediaForm OsMediaCapability ArtifactKindSpec HostedArtifactKind MediaType MediaWireFormat MediaPortDirection PortMultiplicity MediaPortSpec MediaCompat MEDIA_FORM_CONVERSIONS media_types_compatible ArtifactPresentation AppIo Media MediaPayload MediaFingerprint MediaError MediaConverter fingerprint Part bytes'.split())
VIEW = lambda n: n.startswith('VIEW_CONTEXT') or n.startswith('PUBLIC_INVOCATION') or n in ('ViewModel','TreeWindowRequest','ViewWindowInstance','ViewSessionIdentity','MAX_SURFACE_VIEW_CONTEXT_BYTES','MAX_SURFACE_BODY_KEY_BYTES','public_invocation_char_cost','public_invocation_string_pages','UiRefreshSection','UI_REFRESH_SECTION_KEYS','UI_REFRESH_SECTION_BODY_KEYS')
CONTRIB = set('TopicContribution Version VersionParseError VersionPin VersionPinParseError PluginDependency ContributedMutationSemantics ContributedMutationMetadata ContributedInferenceMetadata InferencePayloadContract InferenceCommitBinding InferenceArtifactBinding INFERENCE_ARTIFACT_PACK_BASE64 ArtifactContributionDescriptor ProgramContributionEntry parse_contributions absent SPACE_INDEX_TOPIC SpaceIndexContribution'.split())
FAULTDEF = set('FaultNoticeDefinition FaultNoticeError is_fault_notice_code fault_notice_placeholders validate_fault_notices fault_notice_text fill_fault_notice fault_notice_definitions'.split())
RELOC_VOCAB = lambda n: (n.startswith('HISTORY_EDIT') or n.startswith('history_edit') or n.startswith('Introduction') or n.startswith('introduction') or n.startswith('Tutorial') or n.startswith('tutorial') or n.startswith('GUMBALL') or n.startswith('framework_fixed_audience') or n in ('app_declares_tool_run','tool_run_action_definitions','tool_run_action_description','tool_run_action_arg','START_INTRODUCTION_ACTION_ID','start_introduction_action_definition','START_TUTORIAL_ACTION_ID','start_tutorial_action_definition','RECORD_TUTORIAL_ACTION_ID','record_tutorial_action_definition','TUTORIAL_CONVERGE_MS','validate_tutorial','interpolate_tutorial_camera','apply_tutorial_ui_change','compose_tutorial_ui','TutorialSlice','tutorial_slice'))
PLUGIN = set('PluginManifest ArtifactKindChoice SurfaceAppChoice encode_artifact_kind_choice decode_artifact_kind_choice encode_surface_app_choice decode_surface_app_choice artifact_kind_choices DependencyGraphError validate_dependency_graph resolve_load_order find_cycle_members dependents ExecutionMode ExtensionPointDeclaration AssetDeclaration DescriptorEntry FileTypeContribution IoEntryDirection IoEntryDescriptor ComposerEntryDescriptor ContributionSet ASSEMBLY_FAILED_PLUGIN_ID ExecutionProtocol PackageDescriptor CanonicalDescriptorValue canonical_members AgentContributions PackageRole'.split())
INPUTS = set('INPUT_LABEL_GLOSSARY_JSON input_label_glossary mutation_input_defs InputSchemaAudit mutation_input_audit registered_input_schema_document mutation_input_instance input_union INPUT_NUMERIC_TRANSPORT_SCHEMA_JSON FRAMEWORK_INPUT_SCHEMA_DOCUMENTS INPUT_SHAPE_ANNOTATIONS input_shape input_numeric_transport InputVariant INPUT_UI_KEYS INPUT_UI_NUMBER_KEYS INPUT_UI_VECTOR_KEYS INPUT_REF_DEPTH input_error input_pointer input_localized_text input_type input_inferred_reference_kind ResolvedInput InputSchemaReader input_string input_finite input_positive input_options input_option_source input_number InputSchemaResolver InputSchemaErrorCode InputSchemaError'.split())
VOCAB_HIGH = set('framework_fixed_audience framework_fixed_audience_violation history_edit_action_definitions history_edit_hidden_arg history_edit_finalize_dialog app_declares_tool_run tool_run_action_definitions tool_run_action_description tool_run_action_arg start_introduction_action_definition start_tutorial_action_definition record_tutorial_action_definition'.split())
ADDRESS = set('Keybinding Platform PlatformKeybinding ActionRef ActionAddress ActionInvocation UtilityRef CommandOwnerAddress CommandAddress CommandInvocation CommandInvocationView ToolRef is_element_id element_id_segment child_element_id UI_NAVBAR_ELEMENT_ID UI_FOOTER_ELEMENT_ID window_element_id panel_tab_element_id panel_tab_first_draggable_element_id'.split())
ACTIONS_NEEDS_APP = set('interaction_window_bodies_by_domain panel_leaf_body_keys interaction_action_definitions'.split())
def assign(i):
    e = ents[i]; f = e['file']; n = e['name']
    if e['kind'] == 'mod': return None
    if f in KERNEL_FILES:
        if f == 'k_clipboard': return 'kernel'
        return kernel_cluster(e)
    if f == 'abi': return 'abi'
    if f == 'action_bus': return 'action-bus'
    if f == 'source_projection': return 'source-projection'
    if f == 'platform': return 'platform'
    if f.startswith('interaction'): return 'interaction'
    if f == 'root': return None
    if f == 'm_media': return 'manifest-media'
    if f == 'm_canonical': return 'manifest-contribution'
    if f == 'm_space': return 'manifest-contribution'
    if f == 'm_utilinit': return 'manifest-actions'
    if f == 'm_vocab':
        if n in VOCAB_HIGH: return 'manifest-product-actions'
        return 'manifest-product-vocabulary'
    if n in FAULTDEF: return 'kernel-history'
    if f == 'm_view':
        if n in ('ConfigSpec','CommandVariantSpec','CommandGrammar'): return 'manifest-app-surface'
        if n in MEDIA: return 'manifest-media'
        if VIEW(n): return 'manifest-view-model'
        if n in PLUGIN: return 'manifest-plugin'
        return 'UNASSIGNED'
    if n in CONTRIB: return 'manifest-contribution'
    if f == 'm_plugin':
        return 'manifest-plugin'
    if f == 'm_argschema':
        if n in INPUTS or n.startswith('<impl'): return 'manifest-mutation-inputs'
        if n == 'default_action_icon_id': return 'manifest-actions'
        return 'manifest-arg-schema'
    if f == 'm_actions':
        if n == 'history_action_definitions': return 'manifest-product-actions'
        if n in ACTIONS_NEEDS_APP: return 'manifest-app-surface'
        return 'manifest-actions'
    if f == 'manifest':
        if n in ADDRESS: return 'manifest-address'
        return 'manifest-actions'
    if f == 'm_surface':
        return 'manifest-app-surface'
    return 'UNASSIGNED'
if __name__ == '__main__':
    cl = {i: assign(i) for i in E['order']}
    cl = {i: c for i, c in cl.items() if c}
    # drop dup interaction aliases
    sizes = collections.defaultdict(lambda: [0,0,0])
    for i, c in cl.items():
        sizes[c][0]+=1; sizes[c][1]+=ents[i]['lines']; sizes[c][2]+=ents[i]['bytes']
    for c, s in sorted(sizes.items()): print(c, s)
    # cluster graph
    cg = collections.defaultdict(lambda: collections.defaultdict(list))
    for i, c in cl.items():
        for j in edges.get(i, []):
            d = cl.get(j)
            if d and d != c: cg[c][d].append((ents[i]['name'], ents[j]['name']))
    json.dump({c: {d: v for d, v in m.items()} for c, m in cg.items()}, open('cgraph.json','w'), indent=0, ensure_ascii=False)
    print()
    for c in sorted(cg):
        print(c, '->', ', '.join(f"{d}({len(v)})" for d, v in sorted(cg[c].items())))
    # cycles via SCC on cluster graph
    sys.setrecursionlimit(10000)
    nodes = set(cl.values())
    idx={}; low={}; st=[]; on=set(); out=[]; cnt=[0]
    def sc(v):
        idx[v]=low[v]=cnt[0]; cnt[0]+=1; st.append(v); on.add(v)
        for w in cg.get(v,{}):
            if w not in idx: sc(w); low[v]=min(low[v],low[w])
            elif w in on: low[v]=min(low[v],idx[w])
        if low[v]==idx[v]:
            comp=[]
            while True:
                w=st.pop(); on.discard(w); comp.append(w)
                if w==v: break
            out.append(comp)
    for v in sorted(nodes):
        if v not in idx: sc(v)
    print('\nCYCLES:', [c for c in out if len(c)>1])
