"""♻️ Regenerates the generated `pub use <crate>::{…}` block of a facade file between the begin and end markers."""
import re, sys
from model import *
import facade_names as fn
BEGIN = '// 🏛️ generated facade: names of the carved crate that code outside it names'
def regen(facade_path, crate):
    t = open(facade_path, encoding='utf8').read()
    uses = fn.facade_uses(crate).rstrip('\n')
    ident = 'semio_framework_os_renderer_wgpu_' + crate.replace('-', '_')
    block = BEGIN + ' (' + crate + ')\n' + uses + '\n// 🏛️ end generated facade (' + crate + ')'
    pat = re.compile(re.escape(BEGIN + ' (' + crate + ')') + r'.*?// 🏛️ end generated facade \(' + re.escape(crate) + r'\)', re.S)
    if pat.search(t): t = pat.sub(lambda m: block, t)
    else:
        m = re.search(r'(?m)^(?:#\[[^\n]*\]\n)*pub use ' + re.escape(ident) + r'::\{[^\n]*\};\n(?:(?:#\[[^\n]*\]\n)*pub use ' + re.escape(ident) + r'::\{[^\n]*\};\n)*', t)
        assert m, 'no old block'
        t = t[:m.start()] + block + '\n' + t[m.end():]
    open(facade_path, 'w', encoding='utf8').write(t)
if __name__ == '__main__':
    regen(sys.argv[1], sys.argv[2])
