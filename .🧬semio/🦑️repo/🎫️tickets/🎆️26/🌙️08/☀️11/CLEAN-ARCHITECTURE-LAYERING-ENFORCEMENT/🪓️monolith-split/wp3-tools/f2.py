"""F2: AppInteractionState (struct, impl, text stream) and dispatch_normalized_event leave frame-build/app-runtime/winit-app for the frame-actions crate, so the runtime mailbox needs no upward edge to the app runtime (plan 5.3 I2 refined: the mailbox's runtime surface shrinks to the interaction checkout)."""
import os, sys, re
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import mv
R = os.environ['R']; T = os.environ['T']; V = '️'
fb = R + f'/🧾{V}frame-build/🦀{V}.rs'; ar = R + f'/🧠{V}app-runtime/🦀{V}.rs'; wa = T + f'/🪟{V}winit-app/🦀{V}.rs'; iw = T + f'/🎮{V}input-wire/🦀{V}.rs'
fa = R + f'/🛎{V}frame-actions/🦀{V}.rs'; pr = R + f'/🎨{V}presentation-raster/🦀{V}.rs'
d = R + f'/🛎{V}frame-actions/🎮{V}interaction-state'; dst = d + f'/🦀{V}.rs'
assert not os.path.exists(dst)
rd = lambda p: open(p, encoding='utf8').read()
t_fb, t_ar, t_wa, t_iw, t_fa, t_pr = map(rd, (fb, ar, wa, iw, fa, pr))
rest_fb, m_fb = mv.cut(t_fb, ['const TEXT_STREAM_CAPACITY', 'struct AppTextStream', 'struct AppInteractionState'])
rest_ar, m_ar = mv.cut(t_ar, ['impl AppInteractionState'])
rest_wa, m_wa = mv.cut(t_wa, ['fn dispatch_normalized_event'])
rest_wa, m_help = mv.cut(rest_wa, ['fn pointer_button_to_i16', 'fn event_modifiers_to_pointer', 'fn key_action_from_dispatch'])
assert (len(m_fb), len(m_ar), len(m_wa), len(m_help)) == (3, 1, 1, 3), (len(m_fb), len(m_ar), len(m_wa), len(m_help))
# helpers go to host-plumbing's input wire, widened to pub(crate)
help_txt = '\n\n'.join(re.sub(r'^fn ', 'pub(crate) fn ', x, flags=re.M) for x in m_help)
assert t_iw.endswith('\n')
t_iw2 = t_iw + '\n//#region 🎮' + V + 'InputNormalization\n' + help_txt + '\n//#endregion 🎮' + V + 'InputNormalization\n'
body = '\n\n'.join(m_fb + m_ar + m_wa)
body = re.sub(r'(?m)^(pub\(crate\) async fn dispatch_normalized_event)', r'\1', body)
ui_names = [n for n in ['CursorRequest', 'DispatchEvent', 'EventModifiers', 'ImeEvent', 'InvalidationReason', 'PointerButton', 'PointerInfo', 'TextEditTarget'] if n in mv.idents(body)]
need = {n for n in mv.names_private(body) if n in mv.idents(rest_fb) | mv.idents(rest_ar) | mv.idents(rest_wa) | mv.idents(t_pr)}
body = mv.widen(body, need)
hdr = (f'//! 🎮{V} Interaction state: the state a frame checks out to every worker (shell, input, pointer capture, text streams) with its keyboard and pointer entry points, and the normalized-event dispatch that drives it; the frame-actions crate of the renderer partition, below the runtime mailbox (plan section 5.3, inversion I2).\n'
       'use super::*;\n'
       'use crate::input_wire::{event_modifiers_to_pointer, key_action_from_dispatch, pointer_button_to_i16};\n'
       + (f'use ui_render::{{{", ".join(ui_names)}}};\n' if ui_names else '') + '\n//#region 📮' + V + 'RuntimeMailbox\n//#region 🎮' + V + 'AppInteractionState\n')
os.makedirs(d)
open(dst, 'w', encoding='utf8').write(hdr + body + '\n//#endregion 🎮' + V + 'AppInteractionState\n//#endregion 📮' + V + 'RuntimeMailbox\n')
a = f'mod runtime_apply;\npub(crate) use runtime_apply::*;\n'
assert t_fa.count(a) == 1
t_fa2 = t_fa.replace(a, a + f'\n#[path = "🎮{V}interaction-state/🦀{V}.rs"]\nmod interaction_state;\npub(crate) use interaction_state::*;\n', 1)
# winit-app keeps its callers working through explicit imports
imp = 'use crate::os_host::OsHost;\nuse crate::AppInteractionState;\n'
assert rest_wa.count(imp) == 1
rest_wa = rest_wa.replace(imp, imp + 'use crate::dispatch_normalized_event;\nuse crate::input_wire::{event_modifiers_to_pointer, key_action_from_dispatch, pointer_button_to_i16};\n', 1)
call = 'winit_app::dispatch_normalized_event(interaction,event)'
assert t_pr.count(call) == 1
t_pr2 = t_pr.replace(call, 'dispatch_normalized_event(interaction,event)')
w = lambda p, s: open(p, 'w', encoding='utf8').write(s)
w(fb, rest_fb); w(ar, rest_ar); w(wa, rest_wa); w(iw, t_iw2); w(fa, t_fa2); w(pr, t_pr2)
print('ui names', ui_names, 'widened', sorted(need))
print({k: (len(a.split('\n')), len(b.split('\n'))) for k, a, b in [('fb', t_fb, rest_fb), ('ar', t_ar, rest_ar), ('wa', t_wa, rest_wa)]}, 'new', len(body.split('\n')))
