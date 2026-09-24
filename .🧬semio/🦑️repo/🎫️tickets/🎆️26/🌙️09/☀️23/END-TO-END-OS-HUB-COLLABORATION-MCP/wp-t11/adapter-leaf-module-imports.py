"""📥️ Imports the leaf modules (`remove_x::RemoveX {}`) a conformance-class adapter names bare into its existing
`…::schema::mutations::{…}` use, where the sweep left them unimported."""
import re, glob, sys
root = '✏️s/🔌️plugins/🗄️stdio/🗿️artifacts'
for c in sys.argv[1:]:
    p = glob.glob(f'{root}/*/🏅️standards/*/🪆️subsets/*/🧪️tests/{c}/🦀️.rs')[0]
    s = open(p, encoding='utf-8').read()
    m = re.search(r'(    use semio_s_artifact_stdio_\w+::standards::\w+::subsets::\w+::schema::mutations::\{)([^}]*)(\};)', s)
    have = {i.strip() for i in m.group(2).split(',')}
    bare = sorted({b for b in re.findall(r'(?<![:\w])([a-z_][a-z0-9_]*)::[A-Z]\w*', s) if b not in have and not b.startswith(('semio', 'std', 'crate', 'super', 'self'))})
    print(c, bare)
    if bare:
        items = sorted(have | set(bare), key=lambda n: (n[:1].isupper(), n))
        s = s[:m.start()] + m.group(1) + ', '.join(items) + m.group(3) + s[m.end():]
        open(p, 'w', encoding='utf-8').write(s)
