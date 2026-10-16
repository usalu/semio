import json, collections, sys, os
GEN=os.path.dirname(os.path.abspath(__file__))+'/../🗑️generated'
recs=json.load(open(GEN+'/pwp6-s0-recs.json'))
T2={'instance-app','instance-commands','instance-ladders','time-travel-turns','instance-typed-ops','tool-run-turns','instance-render'}
nodes=[x for x in recs if x['crate'] in T2]
def fam(x):
    c=x['crate']; f=x['file']; n=x['name']
    if c=='instance-app': return 'APP'
    if c=='time-travel-turns': return 'TT'
    if c=='tool-run-turns': return 'TR'
    if c=='instance-ladders': return 'LAD'
    return None

D = {}
def put(dom, names):
    for n in names.split():
        assert n not in D or D[n]==dom, (n, D.get(n), dom)
        D[n]=dom
put('hist','backfill_command_log build_history_view patch_history_view refresh_history_view history_patch refresh_cache history_command history_command_authors test_history')
put('emit','dispatch_emit dispatch_emit_inner test_dispatch_emit dispatch_emit_group commit_transaction_group transaction_group_history mount_original_emit_publication record_typed_operation_lane')
put('resv','framework_reserved_work_items framework_reserved_route_items dispatch_framework_reserved_action admit_framework_reserved_spawn step_framework_reserved_commit run_framework_reserved_commit_unit run_framework_reserved_commit_unit_admitted dispatch_chrome_history_action commit_framework_history_route begin_framework_revert_route commit_framework_clipboard_completion dispatch_reserved_original_producer commit_framework_shared_host_route dispatch_catalogue_example admit_reserved_emit_producer test_reserved_emit_admission_case test_reserved_emit_cancelled_worker_handoff test_reserved_emit_busy_completion_owner admit_original_tool_lease mount_original_reserved_owners')
put('act','dispatch_action dispatch_action_received dispatch_command dispatch_command_frame preview_addressed_action admit_agent_lane_preview_key begin_agent_lane_preview fill_agent_revisions')
put('host','deliver_host_event deliver_utility_host_events deliver_host_event_to_every_window deliver_base_moved dispatch_host_event_action')
put('arch','merge_document_archive merge_decoded_document_archive')
put('tret','retire_typed_operation_unit retire_typed_operation_granted retire_typed_operation_run')
put('grp','advance_private_child_group advance_private_child_group_granted publish_mounted_owned_child_operation_unit')
put('inter','interaction_selection_witness resolve_domain_topology app_interaction_topology build_full_interaction_topology revalidate_and_persist_interaction_state revalidate_interaction_on_document_change apply_interaction_writes dispatch_interaction_action apply_interaction_verb fold_inline_interaction_verbs overlay_leftover_ids_into_vortex leftover_interaction_view_from stamp_pending_render_interaction_ui')
put('grphist','dispatch_group_history_action')
put('rjob','build_artifact_reserved_action_job build_artifact_reserved_media_job')
put('gest','follow_tool_machines freeze_tool_machines admit_gesture_slot settle_gesture_slot end_window_gesture end_every_gesture take_gesture_ended request_gesture_cancellation begin_gesture_retirement gesture_retirement_pending gesture_retirement_demand gesture_retirement_step tool_overlay_retirement_pending tool_overlay_refold_owed tool_overlay_retirement_demand tool_overlay_retirement_step')
put('tmhi','admit_tool_dispatch commit_typing_before publish_typing_commits retire_tool_windows')
put('settle','settle_press settle_tool_operation rendered_snapshot rendered_config set_tool_clock_ms')
put('tpub','typed_operation_stall_streak advance_typed_operation_publication_one advance_typed_operation_publication_unit publish_mounted_typed_operation_run publish_mounted_typed_operation_run_admitted publish_mounted_typed_operation_pump mounted_typed_interaction_writes_are_next publish_mounted_typed_interaction_unit publish_mounted_typed_inline_interaction_unit advance_latest_wins_command_one publish_mounted_typed_child_operation_unit publish_mounted_typed_operation_unit retained_fields_terminal_is_empty')
put('tcmd','dispatch_typed_command_inner capture_typed_command_roots test_capture_presence_roots test_mount_typed_command start_typed_command_operation dispatch_typed dispatch_import_media dispatch_config_command_inner finish_configuration_command verb_dirties_history_panel finish_recorded')
put('LAD','child_root_retirement_step')
def dom(x):
    if x['name'] in D: return D[x['name']]
    if x['crate']=='instance-app': return 'APP'
    if x['crate']=='time-travel-turns': return 'TT'
    if x['crate']=='tool-run-turns': return 'TR'
    if x['crate']=='instance-ladders': return 'LAD'
    return D[x['name']]
