from specs.lib import *
TTI = PL + '/⏪️time-travel/🧵️instance'
_TXT = open(os.path.join(T, 'orig', 'tti.rs'), encoding='utf-8').read()
CUTS = [183, 318, 450, 608, 743]
CH = [
    ch('🧹', 'owners-and-retirement', 'owners_and_retirement', [(55, 183)],
       'Owners and retirement of the instance time-travel turns: the typed store command, the commit, the owners settle, the child refresh, the member subject retirement and the retire, close, defer and dismiss steps.'),
    ch('🔄', 'reprojection-turns-and-base-watch', 'reprojection_turns_and_base_watch', [(185, 321)],
       'Reprojection turns and base watch of the instance time-travel turns: the reprojection step, its cancel and resume, the change and dirty marks and the store base watch.'),
    ch('🎬', 'action-dispatch-and-effects', 'action_dispatch_and_effects', [(323, 456)],
       'Action dispatch and effects of the instance time-travel turns: the dispatched and applied time-travel actions, the event reduction and the performed effects.'),
    ch('🔎', 'preview-replay-and-session-start', 'preview_replay_and_session_start', [(458, 617)],
       'Preview, replay and session start of the instance time-travel turns: the preview, its adoption, the replay start, the commit, the member publication and the session start.'),
    ch('✍', 'draft-inputs', 'draft_inputs', [(619, 755)],
       'Draft inputs of the instance time-travel turns: the drafted input and the drafted selection of a history edit session.'),
    ch('🏃', 'turn-driving-and-replay-steps', 'turn_driving_and_replay_steps', [(757, 883)],
       'Turn driving and replay steps of the instance time-travel turns: the per-turn driver, the original review children and discard command, the patch preparation and the replay step.'),
    ch('📖', 'ref-reads', 'ref_reads', [(885, 1061)],
       'Ref reads of the instance time-travel turns: the read-only base, query, retirement demands, snap, option and reference label resolution and the pending work, position and busy views.'),
    ch('🪄', 'supersede-authoring-and-extension', 'supersede_authoring_and_extension', [(1063, 1253)],
       'Supersede authoring and extension of the instance time-travel turns: the supersede authoring steps and results, the history edit row revert, the frozen action fault and the time-travel extension that registers the turns with the instance.'),
]
SPEC = dict(orig='tti.rs', file=TTI + '/🦀️.rs', children=CH, wiring='each', pre=cut_impl(55, CUTS), pre_extra=cut_extra(55, CUTS, _TXT), external=mounted(TTI + '/🦀️.rs', orig='tti.rs'))
