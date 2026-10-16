"""I1: Interpreter retained-ui-engine splits into the contracts lift (cell, close queue, pointer owners), the ui-engine-focus part (scene intents, clipboard io) and the ui-engine remainder (plan 5.2)."""
import os, sys, re
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import mv
E = os.environ['E']; T = os.environ['T']; V = '️'
DRY = '--dry' in sys.argv
src = E + f'/🧱{V}elements/🗣{V}Interpreter/🎯{V}targets/🧊{V}wgpu/🧩{V}retained-ui-engine/🦀{V}.rs'
text = open(src, encoding='utf8').read()
contracts_specs = ['static UI_ENGINE', 'const UI_ENGINE_LOCALE_UNRESOLVED', 'const TEST_UI_ENGINE_LOCALE', 'struct UiEngineCell', 'struct UiEngineGuard',
                   'impl std::ops::Deref for UiEngineGuard', 'impl std::ops::DerefMut for UiEngineGuard', 'fn initial_ui_engine', 'impl UiEngineCell', 'fn install_ui_engine_locale',
                   'struct UiDocumentCloseOwner', 'struct UiDocumentCloseQueue', 'impl UiDocumentCloseQueue', 'static UI_DOCUMENT_CLOSE_QUEUE',
                   'fn ui_document_close_pending', 'fn ui_document_close_pending_for', 'struct ScenePointerOwner', 'struct ScenePointerOwners', 'static SCENE_POINTER_OWNERS']
focus_specs = ['const SCENE_INTENT_CAPACITY', 'const SCENE_INTENT_ID_BYTES', 'enum SceneIntentEvent', 'struct SceneInteractionIntent', 'struct SceneIntentQueue',
               'impl Default for SceneIntentQueue', 'impl SceneIntentQueue', 'impl SceneInteractionIntent', 'static SCENE_INTENTS', 'fn apply_presented_scene_caret',
               'fn write_os_clipboard', 'struct ClipboardInvocationInput', 'struct MountedClipboardIo', 'struct MountedClipboardSlot', 'static MOUNTED_CLIPBOARD_IO',
               'fn clipboard_refusal', 'fn clipboard_demand_failure', 'fn clipboard_alias_failure', 'fn validate_clipboard_source', 'fn submit_clipboard_io',
               'fn deliver_clipboard_recipient', 'fn pump_clipboard_io_one', 'fn write_owned_os_clipboard', 'thread_local MOCK_CLIPBOARD_WRITES']
rest, m_con = mv.cut(text, contracts_specs, strict=False)
rest, m_foc = mv.cut(rest, focus_specs, strict=False)
print('contracts items', len(m_con), sum(len(x.split('\n')) for x in m_con), 'focus items', len(m_foc), sum(len(x.split('\n')) for x in m_foc))
for s in contracts_specs + focus_specs:
    n = sum(1 for it in mv.split_items(text) if it.kind != 'marker' and mv.matches(it, s))
    if n == 0: print('MISSING', s)
if DRY: sys.exit(0)
rd = lambda p: open(p, encoding='utf8').read()
w = lambda p, s: open(p, 'w', encoding='utf8').write(s)
def widen_super(body): return re.sub(r'\bpub\(super\)', 'pub(crate)', body)
def need_widen(body, *others):
    other = set()
    for o in others: other |= mv.idents(o)
    return mv.widen(widen_super(body), {n for n in mv.names_private(body) if n in other})
con_body = need_widen('\n\n'.join(m_con), rest, '\n'.join(m_foc))
foc_body = need_widen('\n\n'.join(m_foc), rest, '\n'.join(m_con))
lift_dir = T + f'/🤝{V}contracts/📥{V}lifted/🧩{V}retained-ui-engine'
lift = lift_dir + f'/🦀{V}.rs'
foc_dir = os.path.dirname(src) + f'/🎬{V}scene-intents-and-clipboard'
foc = foc_dir + f'/🦀{V}.rs'
assert not os.path.exists(lift) and not os.path.exists(foc)
os.makedirs(lift_dir); os.makedirs(foc_dir)
w(lift, f'//! 🤝{V} Retained UI engine cell: the per-worker retained UI engine, its document close queue and the scene pointer owners, lifted out of the Interpreter for the `contracts` crate of the renderer partition (plan section 5.2). The cell, its guard and the close queue keep their exact layout: they carry the reentrancy law the worker-cell test pins.\nuse super::*;\n\n//#region 🧩{V}RetainedUiEngine\n' + con_body + f'\n//#endregion 🧩{V}RetainedUiEngine\n')
w(foc, f'//! 🎬{V} Scene intents and clipboard io: the scene intent queue, the caret application and the mounted clipboard io of the retained UI engine; the ui-engine-focus crate of the renderer partition (plan section 5.2).\nuse super::*;\n\n//#region 🧩{V}RetainedUiEngine\n' + foc_body + f'\n//#endregion 🧩{V}RetainedUiEngine\n')
rel = os.path.relpath(lift, os.path.dirname(src))
assert len(os.path.relpath(lift, os.environ['ROOT']).encode()) <= 240
mount = f'\n#[path = "{rel}"]\nmod contracts_lift;\npub(crate) use contracts_lift::*;\n\n#[path = "🎬{V}scene-intents-and-clipboard/🦀{V}.rs"]\nmod scene_intents_and_clipboard;\npub(crate) use scene_intents_and_clipboard::*;\n'
w(src, rest.rstrip('\n') + '\n' + mount)
print('lines', len(text.split('\n')), '->', len(rest.split('\n')))
