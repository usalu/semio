from specs.lib import *
F = FILES['fs']
H1 = 'impl MountedModelBuild {'
H2 = 'impl MountedState {'
B1 = [709]
B2 = [1226, 1550]
def pre(t):
    return split_impl(H1, B1)(split_impl(H2, B2)(t))
CH = [
    ch('🪪', 'session-constants-and-owner-catalog', 'session_constants_and_owner_catalog', [(29, 192)],
       'Session constants and owner catalog: the session capacities and limits, the mounted owner class and claim and the process owner catalog.'),
    ch('🏗', 'model-build-job', 'model_build_job', [(194, 709)],
       'Model build job: the bounded string helpers, the mounted identity and stages, the model build stages and the mounted model build with its stepping.'),
    ch('🏁', 'model-build-close', 'model_build_close', [(711, 925)],
       'Model build close: the completion take and the close stepping of the mounted model build.'),
    ch('🗂', 'mounted-state-and-preparation', 'mounted_state_and_preparation', [(929, 1229)],
       'Mounted state and preparation: the mounted state, the child end and failure helpers, the close constants and the state construction, progress, domain preparation and visual drive.'),
    ch('🔁', 'mounted-state-step', 'mounted_state_step', [(1231, 1556)],
       'Mounted state step: the stepping of the mounted state through the model build, mesh, assembly and solve stages.'),
    ch('🧹', 'mounted-state-close', 'mounted_state_close', [(1558, 1909)],
       'Mounted state close: the close stepping and the terminal emptiness check of the mounted state.'),
    ch('📍', 'snapshot-admission-and-registry', 'snapshot_admission_and_registry', [(1911, 2250)],
       'Snapshot admission and registry: the current session, the pending admissions, the snapshot admission cursor and the mounted registry.'),
    ch('💼', 'job-bridge-and-live-mount', 'job_bridge_and_live_mount', [(2258, 2666)],
       'Job bridge and live mount: the bounded job bridge with the input codec and the live mount entries that initialize, prepare, reconcile, retire and close the session.'),
]
SPEC = dict(orig='fs.rs', file=F, children=CH, wiring='block', external=mounted(F, orig='fs.rs'), pre=pre, expect_extra=split_extra(H1, B1) + split_extra(H2, B2))
