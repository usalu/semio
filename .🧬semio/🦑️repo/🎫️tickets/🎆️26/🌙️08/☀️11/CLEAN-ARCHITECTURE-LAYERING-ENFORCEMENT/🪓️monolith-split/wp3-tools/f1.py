"""F1: RuntimeApply and the three maintenance/execution owners leave frame-actions for the runtime-mailbox crate (plan 5.3 I2)."""
import os, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import mv
R = os.environ['R']; V = '️'
src = R + f'/🛎{V}frame-actions/🦀{V}.rs'
dstdir = R + f'/🛎{V}frame-actions/🔁{V}runtime-apply'
dst = dstdir + f'/🦀{V}.rs'
assert not os.path.exists(dst)
text = open(src, encoding='utf8').read()
specs = ['struct FrameMaintenanceOwner', 'impl FrameMaintenanceOwner', 'struct FrameDeferredExecutionOwner', 'impl FrameDeferredExecutionOwner', 'struct FrameMaintenanceRefusal', 'impl FrameMaintenanceRefusal', 'enum RuntimeApply', 'impl RuntimeApply']
rest, moved = mv.cut(text, specs)
assert len(moved) == 8, len(moved)
body = '\n\n'.join(moved)
priv_new = mv.names_private(body); priv_rest = mv.names_private(rest)
need = {n for n in priv_new if n in mv.idents(rest)} 
need_rest = {n for n in priv_rest if n in mv.idents(body)}
body = mv.widen(body, need)
pass
hdr = f'//! 🔁{V} Runtime apply: the completion variants a worker hands back to the runtime, with the maintenance and deferred execution owners they carry; the runtime-mailbox crate of the renderer partition (plan section 5.3, inversion I2).\nuse super::*;\n\n//#region 📮{V}RuntimeMailbox\n'
os.makedirs(dstdir)
open(dst, 'w', encoding='utf8').write(hdr + body + '\n//#endregion 📮' + V + 'RuntimeMailbox\n')
anchor = 'use super::*;\n'
assert rest.count(anchor) == 1
rest = rest.replace(anchor, anchor + f'\n#[path = "🔁{V}runtime-apply/🦀{V}.rs"]\nmod runtime_apply;\npub(crate) use runtime_apply::*;\n', 1)
open(src, 'w', encoding='utf8').write(rest)
print('widened new:', sorted(need), 'rest:', sorted(need_rest))
print(len(text.split('\n')), '->', len(rest.split('\n')), '+', len(body.split('\n')))
