# 📖️ WP4 first physical carve: `shell_chrome_string` (a pure EN/DE lookup) leaves the monolith into the real `shell-vocab` crate.
# usage: carve_vocab.py <t1 tree> <overlay out dir>   (sandbox only)
import re, sys, os, glob, shutil
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from model import *
from units import *

HUB_REL = ENG + '/🎯️targets/🧊️wgpu/🧊️renderer/📥️mounts/🐚️wp4-shell-state/🦀️.rs'
VOCAB_REL = ENG + '/🎯️targets/🧊️wgpu/📖️shell-vocab/🦀️.rs'

def main(tree, overlay):
    path = None
    for p in glob.glob(tree + '/📥️staging/*/*/🦀️.rs'):
        if 'fn shell_chrome_string' in open(p, encoding='utf-8').read(): path = p; break
    assert path, 'shell_chrome_string not found'
    src, units = split_file(path)
    u = [x for x in units if x.kind == 'fn' and x.name == 'shell_chrome_string']
    assert len(u) == 1
    u = u[0]
    body = list(src.lines[u.first:u.last + 1])
    for i, l in enumerate(body):
        if re.match(r'^(pub(\([^)]*\))?\s+)?fn shell_chrome_string', l):
            body[i] = re.sub(r'^(pub(\([^)]*\))?\s+)?fn', 'pub fn', l)
    lines = list(src.lines)
    del lines[u.first:u.last + 1]
    open(path, 'w', encoding='utf-8').write('\n'.join(lines))
    vocab = ['//! 📖️ Shell chrome strings, language axes, labels and refusal vocabulary.', '//!',
             '//! 🗣️ Holds the EN/DE chrome string table of the wgpu shell as a pure lookup (`shell_chrome_string`), carved out of the renderer monolith by RENDERER-WP4 as the first physical Shell crate: no state, no I/O, no first-party dependency.', '',
             '//#region 🗣️ChromeI18n'] + body + ['//#endregion 🗣️ChromeI18n', '', '#[cfg(test)]', '#[path = "🧪️tests/🦀️.rs"]', 'mod tests;', '']
    out = os.path.join(overlay, VOCAB_REL); os.makedirs(os.path.dirname(out), exist_ok=True)
    open(out, 'w', encoding='utf-8').write('\n'.join(vocab))
    hub_base = '/private/tmp/claude-501/-Users-ueli-Documents-semio/11914954-c49c-43e1-9499-97ab9f20751b/scratchpad/orig-hub-wp4.rs'
    hub = open(hub_base, encoding='utf-8').read()
    anchor = '#[path = "../../../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs"]\npub mod shell;'
    assert hub.count(anchor) == 1
    hub2 = hub.replace(anchor, 'extern crate semio_framework_os_renderer_wgpu_shell_vocab as shell_vocab;\n\n' + anchor)
    h = os.path.join(overlay, HUB_REL); os.makedirs(os.path.dirname(h), exist_ok=True)
    open(h, 'w', encoding='utf-8').write(hub2)
    pp = tree + '/🦀️.rs'
    pt = open(pp, encoding='utf-8').read()
    a2 = '//#endregion 📥️RendererWp4Staging'
    assert pt.count(a2) == 1
    pt = pt.replace(a2, 'pub(crate) use shell_vocab::shell_chrome_string;\n' + a2)
    open(pp, 'w', encoding='utf-8').write(pt)
    print('carved', len(body), 'lines into shell-vocab from', os.path.relpath(path, tree))

if __name__ == '__main__':
    main(sys.argv[1], sys.argv[2])
