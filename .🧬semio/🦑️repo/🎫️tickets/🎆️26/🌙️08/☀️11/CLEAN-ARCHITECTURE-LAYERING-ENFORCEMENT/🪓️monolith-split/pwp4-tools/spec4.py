# 📐️ WP4 S0 specification: which chunk of which source module goes into which piece file of which crate directory.
from carve4 import *

RT = '⚛️reactor/🏃️runtime/🦀️.rs'
RC = '⚛️reactor/🦀️.rs'
PT_ = '⚛️reactor/🩹️patches/🦀️.rs'
D_LIFE = '⚛️reactor/📨️pending'
D_LIFE_RT = '⚛️reactor/🏃️runtime/📥️staging/♻️reactor-lifecycle'
D_APP_RT = '⚛️reactor/🏃️runtime/📥️staging/🔌️reactor-app'
D_INST = '⚛️reactor/🏃️runtime/📦️instances'
D_OPS = '⚛️reactor/🏃️runtime/🧰️ops'
D_EXCH = '⚛️reactor/🏃️runtime/🔄️exchange'
D_APP = '⚛️reactor/🔌️app'
D_TASKS = '⚛️reactor/⏲️tasks'
D_TURN = '⚛️reactor/🔄️turn'
D_WIT = '⚛️reactor/🧬️wit-bridge'
EXT_LIFT = '🧩️extension/🛬️lifted/🏃️runtime/🦀️.rs'
PRG_LIFT = '🏗️builder/🔌️program/🛬️lifted/🏃️runtime/🦀️.rs'
MACROS = '📣️macros/⚛️wp4-reactor/🦀️.rs'

def P(pid, crate, dir, emoji, name, desc, src, **kw):
    return Piece(pid, crate, dir, emoji + FE0F + name, f'//! {emoji}{FE0F} {desc}', src, **kw)

def R(lo, hi): return lambda c: c.hl >= lo and c.e <= hi
def N(*names): return lambda c: c.name in names and c.kind != 'use'
def ANY(*ps): return lambda c: any(p(c) for p in ps)
def K(kind): return lambda c: c.kind == kind

RUNTIME_PIECES = [
 P('life.app-cell', 'reactor-lifecycle', D_LIFE_RT, '🛎', 'app-cell', 'The per-instance app cell of the plugin runtime: host and background holds, refusals and the cell itself.', RT),
 P('life.cleanup-vocabulary', 'reactor-lifecycle', D_LIFE_RT, '🧹', 'cleanup-vocabulary', 'Cleanup fault, maintenance status and close status vocabulary of the runtime lifecycle.', RT),
 P('life.instance-registry', 'reactor-lifecycle', D_LIFE_RT, '🗃', 'instance-registry', 'Close worker state, fixed instance registry, actor authority and the typed plugin runtime.', RT),
 P('life.cleanup-pumps', 'reactor-lifecycle', D_LIFE_RT, '🧯', 'cleanup-pumps', 'Live and close cleanup pumps and jobs of the runtime, and the instance close admission.', RT),
 P('inst.wire-frames', 'runtime-instances', D_INST, '🖼', 'wire-frames', 'Fault frames, merge-conflict frames and the instance registry accessors of the runtime.', RT),
 P('inst.descriptor', 'runtime-instances', D_INST, '🪪', 'descriptor', 'Plugin bundle installation, manifest, descriptor extras and the wire mutation roster.', RT),
 P('inst.opening', 'runtime-instances', D_INST, '🚪', 'opening', 'Native instance opening and app creation with the close grant constants.', RT),
 P('inst.cleanup-turns', 'runtime-instances', D_INST, '🔁', 'cleanup-turns', 'Live and close cleanup turns, stall verdicts, destroy and the cooperative maintenance steps.', RT),
 P('ops.wire-limits', 'runtime-ops', D_OPS, '🛡', 'wire-limits', 'Public action wire limits, envelope validation and addressed action admission.', RT),
 P('ops.document', 'runtime-ops', D_OPS, '📄', 'document', 'Document operations, codecs, archive loads, backbone bindings and codec registration.', RT),
 P('ops.media', 'runtime-ops', D_OPS, '🎞', 'media', 'Media consumption, export and production operations of the runtime.', RT),
 P('ops.surface-render', 'runtime-ops', D_OPS, '🖥', 'surface-render', 'Surface mounting, rendering, UI refresh sections and the context menu.', RT),
 P('ops.ingress', 'runtime-ops', D_OPS, '📥', 'ingress', 'Opening relays, command ingress, the exchange output and the custody manifest receiving.', RT),
 P('ops.typed-operations', 'runtime-ops', D_OPS, '🧾', 'typed-operations', 'Typed operation driving, task resume, response dispatch, intents and reserved presence.', RT),
 P('exch.plugin-exchange', 'runtime-exchange', D_EXCH, '🔀', 'plugin-exchange', 'The plugin exchange dispatcher: one function over every command, action and operation.', RT),
 P('app.runtime-helpers', 'reactor-app', D_APP_RT, '🛠', 'runtime-helpers', 'Small runtime helpers the instance family names: turn grants, render section, addressed view, internal fault.', RT),
 Piece('ext.lift', 'extension', '', '', '', RT, existing=EXT_LIFT),
 Piece('prg.lift', 'plugin-program', '', '', '', RT, existing=PRG_LIFT),
 Piece('macros', 'facade', '', '', '', RT, existing=MACROS, mounted_elsewhere=True, header=False),
]
RUNTIME_PIECES[-3].mod = 'extension_runtime_lift'
RUNTIME_PIECES[-2].mod = 'program_runtime_lift'
RUNTIME_RULES = [
 ('macros', ANY(R(4650, 5188), R(5497, 5582))),
 ('ext.lift', ANY(R(838, 859), R(5189, 5496))),
 ('prg.lift', ANY(R(638, 659), R(679, 683), R(796, 806), R(877, 912))),
 ('app.runtime-helpers', ANY(R(669, 678), R(2099, 2105), R(2117, 2127), R(2556, 2559), R(2569, 2580), R(3168, 3171), R(5588, 5590))),
 ('life.app-cell', R(39, 205)),
 ('life.cleanup-vocabulary', R(206, 394)),
 ('life.instance-registry', R(395, 637)),
 ('life.cleanup-pumps', ANY(R(913, 917), R(1033, 1212), R(1313, 1469), R(1583, 1593), R(1601, 1608), R(1729, 1732), R(1736, 1805))),
 ('inst.wire-frames', ANY(R(660, 668), R(684, 738))),
 ('inst.descriptor', ANY(R(739, 795), R(807, 837), R(860, 876))),
 ('inst.opening', R(918, 1032)),
 ('inst.cleanup-turns', ANY(R(1213, 1312), R(1470, 1582), R(1594, 1600), R(1609, 1728), R(1733, 1735), R(1806, 1877))),
 ('ops.wire-limits', R(1878, 2127)),
 ('ops.document', R(2128, 2490)),
 ('ops.media', R(2491, 2555)),
 ('ops.surface-render', R(2556, 2908)),
 ('ops.ingress', ANY(R(2909, 3167), R(5591, 5594))),
 ('ops.typed-operations', R(3168, 3837)),
 ('exch.plugin-exchange', R(3838, 4649)),
]

ROOT_PIECES = [
 P('tasks.fixed-slots', 'reactor-tasks', D_TASKS, '🧮', 'fixed-slots', 'The fixed-slot primitive of the reactor state and its slot counts.', RC),
 P('tasks.instance-metadata', 'reactor-tasks', D_TASKS, '📇', 'instance-metadata', 'The per-instance metadata registry that gates the task quota.', RC),
 P('tasks.task-lane', 'reactor-tasks', D_TASKS, '🧵', 'task-lane', 'Task records, resumes, quota, spawn and cancellation of the actor-local task lane.', RC),
 P('tasks.extension-continuation', 'reactor-tasks', D_TASKS, '🔗', 'extension-continuation', 'Host handles and the extension invocation continuation over the actor request registry.', RC),
 P('tasks.render-bindings', 'reactor-tasks', D_TASKS, '🧷', 'render-bindings', 'The job render binding registry keyed by job and instance.', RC),
 P('life.reactor-close', 'reactor-lifecycle', D_LIFE, '🔒', 'reactor-close', 'The reactor close registry and the close participant steps of the lifecycle owner.', RC),
 P('inst.metadata-owner', 'runtime-instances', D_INST, '🏷', 'metadata-owner', 'Hands the retained opening app id into its original close slot.', RC),
 P('turn.close-step', 'reactor-turn', D_TURN, '🏁', 'close-step', 'One bounded reactor close opportunity driven by the turn.', RC),
 P('wit.bridge', 'reactor-wit-bridge', D_WIT, '🪢', 'bridge', 'The wasm32-wasip2 translation between the WIT reactor world and the kernel types.', RC, cfg='#[cfg(all(target_arch = "wasm32", target_env = "p2"))]', unwrap='wit_bridge', mod='wit_bridge', reexport=''),
]
ROOT_RULES = [
 ('tasks.fixed-slots', ANY(N('REACTOR_TASK_SLOTS', 'REACTOR_FIXED_WORDS', 'PLUGIN_REACTOR_INSTANCE_SLOTS'), lambda c: c.kind == 'struct' and c.name == 'ReactorFixedSlots')),
 ('tasks.instance-metadata', N('PLUGIN_REACTOR_APP_ID_BYTES', 'InstanceMetadata', 'InstanceMetadataRegistry')),
 ('tasks.task-lane', N('TaskRecord', 'TaskRecordRegistry', 'TaskResumeOutcome', 'PendingResume', 'REACTOR_RESUME_SLOTS', 'REACTOR_RESUME_BYTES', 'REACTOR_RESUME_ACTOR_BYTES', 'FixedResumeQueue', 'REACTOR_TASK_KEY_BYTES', 'REACTOR_TASK_LABEL_BYTES', 'REACTOR_TASK_RESTART_BYTES', 'instance_task_quota', 'spawn_task', 'encode_mutation_lane', 'cancel_instance_tasks_step', 'cancel_instance_tasks')),
 ('tasks.extension-continuation', N('host_for_instance', 'queue_extension_invocation', 'drain_queued_effects', 'extension_response_args', 'take_extension_response', 'append_extension_response_page', 'host')),
 ('tasks.render-bindings', N('JobRenderBinding', 'JobRenderBindingRegistry', 'job_render_binding_tests')),
 ('life.reactor-close', N('ReactorCloseState', 'ReactorCloseRegistry', 'preflight_reactor_close', 'reserve_reactor_close', 'reactor_close_fault', 'activate_reactor_close', 'reactor_close_complete', 'release_reactor_close')),
 ('inst.metadata-owner', N('take_original_instance_metadata_app_id')),
 ('turn.close-step', N('step_reactor_close')),
 ('wit.bridge', lambda c: c.kind == 'mod' and c.name == 'wit_bridge'),
]

PATCH_PIECES = [
 P('life.patch-slots', 'reactor-lifecycle', D_LIFE, '🩹', 'patch-slots', 'The patch tracker state, slots and terminals that the pending patch authority and the close owner share.', PT_),
]
PATCH_RULES = [
 ('life.patch-slots', N('READY_PATCH_CAPACITY', 'SurfaceSlot', 'MountedTreeProducer', 'MountedTreeTerminal', 'TerminalSlot', 'RejectedSlot', 'ReadySlot', 'UnadmittedSlot', 'ClosingInstance', 'PatchTrackerState', 'fixed_slots', 'pending_render_ready', 'has_work')),
]

CUTS = [(RT, RUNTIME_PIECES, RUNTIME_RULES), (RC, ROOT_PIECES, ROOT_RULES)]
