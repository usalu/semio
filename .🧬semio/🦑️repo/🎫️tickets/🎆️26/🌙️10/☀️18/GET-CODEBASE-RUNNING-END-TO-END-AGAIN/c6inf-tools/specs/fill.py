from specs.lib import *
F = NRM + '/🪣️fill/🦀️.rs'
H = 'impl BoardFillJob {'
CUTS = [('fn prepare_source', H), ('fn accept_candidate', H), ('fn reject_candidate', H)]


def pre(t):
    t, _ = cut_impls(t, CUTS)
    return t


CH = [
    ch('🏷', 'naming-capacities-and-pages', 'naming_capacities_and_pages', [(23, 237)],
       'Naming, capacities and pages of the board fill: the neutral naming, the fill capacity constants, the fill text and the page and fixed page containers.'),
    ch('📸', 'snapshot-and-capture-stages', 'snapshot_and_capture_stages', [(239, 448)],
       'Snapshot and capture stages of the board fill: the node, handle, template, kind and rule snapshots with their capture stages, the fill snapshot, its capture and ingress records and the capture step.'),
    ch('🎯', 'source-candidate-and-preview', 'source_candidate_and_preview', [(450, 624)],
       'Source, candidate and preview of the board fill: the fill source, virtual nodes and handles, candidates, the job state, the preview, verdicts, candidate events and the placement record.'),
    ch('📍', 'placement-checkpoint-and-commit-records', 'placement_checkpoint_and_commit_records', [(626, 773)],
       'Placement, checkpoint and commit records of the board fill: the placement and checkpoint impls and drops, the fill result and the commit handle, shape, placement and candidate records.'),
    ch('🧬', 'commit-encoder-and-payload', 'commit_encoder_and_payload', [(775, 1179)],
       'Commit encoder and payload of the board fill: the commit layout constants, the encode stage, the commit encoder, the commit payload and the commit candidate impl.'),
    ch('📥', 'fill-job-and-snapshot-capture', 'fill_job_and_snapshot_capture', [(1181, 1631)],
       'Fill job and snapshot capture of the board fill: the fill job struct, the offer enum, the snapshot begin function and the snapshot capture impl with its close protocol.'),
    ch('🚪', 'snapshot-ingress', 'snapshot_ingress', [(1633, 2066)],
       'Snapshot ingress of the board fill: the node, handle, kind and rule publication byte by byte and the ingress close protocol.'),
    ch('🧵', 'job-operation-and-state', 'job_operation_and_state', [(2068, 2207)],
       'Job operation and state of the board fill: the operation scope, checkpoint hand over, event takes, seeds and box overlap of the fill job.'),
    ch('🔍', 'job-source-and-candidate-scan', 'job_source_and_candidate_scan', [(2209, 2510)],
       'Job source and candidate scan of the board fill: source preparation, target and candidate selection, compatibility, preview construction and collision scans.'),
    ch('✅', 'job-accept', 'job_accept', [(2512, 2704)],
       'Job accept of the board fill: accepting a candidate with its node, edge, kind, handle, icon and virtual node publication.'),
    ch('🚫', 'job-reject-preview-and-complete', 'job_reject_preview_and_complete', [(2706, 2929)],
       'Job reject, preview and complete of the board fill: rejecting a candidate or target, the preview, completion, fault outcome, the next offer and its present step.'),
    ch('🏁', 'job-close-and-interactive', 'job_close_and_interactive', [(2931, 3148)],
       'Job close and interactive of the board fill: the granted close, the interactive job impl, the job drop and the send assertions.'),
]
SPEC = dict(orig='fill.rs', file=F, children=CH, wiring='block', external=mounted(F, orig='fill.rs'), pre=pre, pre_extra=['}', H] * len(CUTS))
