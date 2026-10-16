"""F4: frame-build/app-runtime/frame-presentation/presentation-raster regroup into the file shapes of the crates frame-transaction, app-runtime and runtime-mailbox (plan 5.2 rows for those crates)."""
import os, sys, re
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import mv
R = os.environ['R']; V = '️'
P = lambda name: R + f'/{name.replace("@", V)}/🦀{V}.rs'
rd = lambda p: open(p, encoding='utf8').read()
w = lambda p, s: open(p, 'w', encoding='utf8').write(s)
fb, ar, fp, pr, fa = P('🧾@frame-build'), P('🧠@app-runtime'), P('🖥@frame-presentation'), P('🎨@presentation-raster'), P('🛎@frame-actions')
t = {k: rd(p) for k, p in dict(fb=fb, ar=ar, fp=fp, pr=pr, fa=fa).items()}

def child(parent_path, folder, mod, header, bodies, uses='', region=True):
    d = os.path.dirname(parent_path) + '/' + folder
    assert not os.path.exists(d + f'/🦀{V}.rs'), d
    os.makedirs(d, exist_ok=True)
    reg = ('\n//#region 📮' + V + 'RuntimeMailbox\n', '\n//#endregion 📮' + V + 'RuntimeMailbox\n') if region else ('\n', '\n')
    w(d + f'/🦀{V}.rs', header + '\nuse super::*;\n' + uses + reg[0] + '\n\n'.join(bodies) + reg[1])
    return f'#[path = "{folder}/🦀{V}.rs"]\nmod {mod};\npub(crate) use {mod}::*;\n'

def widen_pair(body, *others):
    other = set()
    for o in others: other |= mv.idents(o)
    return mv.widen(body, {n for n in mv.names_private(body) if n in other})

# 1. frame transaction group out of frame-build, boot_runtime out of app-runtime
fb_rest, m_tx = mv.cut(t['fb'], ['struct FrameTransaction', 'enum AppFrameTransactionStep', 'enum FrameTransactionStage', 'impl FrameTransactionStage', 'enum AppFrameTransactionPhase', 'impl FrameTransaction'])
ar_rest, m_boot = mv.cut(t['ar'], ['fn boot_runtime'])
assert len(m_tx) == 6 and len(m_boot) == 1
tx_body = widen_pair('\n\n'.join(m_tx + m_boot), fb_rest, ar_rest, t['fp'], t['pr'], t['fa'])
mount_tx = child(fb, f'🧮{V}frame-transaction', 'frame_transaction', f'//! 🧮{V} Frame transaction: the transaction phases and steps of one frame and `boot_runtime`; the frame-transaction crate of the renderer partition, above the presenter (plan section 5.2).', [tx_body])

# 2. frame preparation group out of frame-presentation (types and impls move down beside the build cursors)
fp_rest, m_prep = mv.cut(t['fp'], ['enum FramePreparationStep', 'struct AppFramePreparation', 'fn frame_retirement_permits', 'impl AppFramePreparation', 'impl AppFramePresentation'])
assert len(m_prep) == 5
prep_body = widen_pair('\n\n'.join(m_prep), fp_rest, fb_rest, ar_rest, t['pr'])
mount_prep = child(fb, f'📋{V}frame-preparation', 'frame_preparation', f'//! 📋{V} Frame preparation: the preparation cursor and its presentation counterpart, beside the build cursors that own them so a type and every inherent impl of it share a crate; the app-runtime crate of the renderer partition (plan section 5.2).', [prep_body])

# 3. native asset stream out of app-runtime into the runtime mailbox transport (world_rejection_fault is frame-build logic and stays)
ar_rest, m_stream = mv.cut(ar_rest, ['fn native_renderer_asset_path', 'fn stream_native_renderer_asset', 'fn push_renderer_asset_page', 'fn stream_native_renderer_http_asset'])
assert len(m_stream) == 4
stream_body = widen_pair('\n\n'.join(m_stream), ar_rest, fb_rest, fp_rest, t['pr'])
mount_stream = child(pr, f'🌐{V}native-asset-stream', 'native_asset_stream', f'//! 🌐{V} Native asset stream: the file and http page streams that feed a world asset fetch; mailbox transport of the runtime-mailbox crate of the renderer partition (plan section 5.3).', [stream_body])

# 4. frame-job's carriers and the deadline capacity go down to frame-actions
fj = os.environ['T'] + f'/🧵{V}frame-job/🦀{V}.rs'
t_fj = rd(fj)
fj_rest, m_car = mv.cut(t_fj, ['struct FrameBuildInputs', 'struct FrameDirectives'])
assert len(m_car) == 2
cap_line = 'pub(crate) const WORLD3D_DEADLINE_CAPACITY: usize = 256;\n'
assert t['pr'].count(cap_line) == 1
pr_rest = t['pr'].replace(cap_line, '')

# 5. mounts and writes
def mount(text, anchor, add):
    assert text.count(anchor) == 1, anchor
    return text.replace(anchor, anchor + '\n' + add, 1)
fb_rest = mount(fb_rest, 'use super::*;\n', mount_tx + '\n' + mount_prep)
pr_rest = mount(pr_rest, 'use super::*;\n', mount_stream)
carriers = '\n//#region 📥' + V + 'FrameBuildInputs\n' + '\n\n'.join(m_car) + '\n//#endregion 📥' + V + 'FrameBuildInputs\n'
fa_text = t['fa'].rstrip('\n') + '\n' + cap_line + carriers
write_all = {fb: fb_rest, ar: ar_rest, fp: fp_rest, pr: pr_rest, fa: fa_text, fj: fj_rest}
for pth, s in write_all.items(): w(pth, s)
print({os.path.basename(os.path.dirname(k)): len(v.split('\n')) for k, v in write_all.items()})
