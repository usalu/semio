import os
T = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
V = '️'
A = '✏️s/🔌️plugins/🗄️stdio/🗿️artifacts'
PNG = A + '/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🚪️io'
JPG = A + '/📸️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/🧾️document/🚪️io'
AVI = A + '/📼️avi/🏅️standards/🔖️1.0/🪆️subsets/🎛️hdrl/🚪️io'
PPTX = A + '/📽️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🔮️oracles'
DXFB = A + '/🖋️dxf/🏅️standards/🔖️r12/🪆️subsets/📰️header/🚪️io/💾️binary/🔺️diff'
DXFS = A + '/🖋️dxf/🏅️standards/🔖️r12/🪆️subsets/📰️header/🧬️schema/🔺️diff'
ZLIB = A + '/🗜️deflate/🏅️standards/🔖️rfc1950/🪆️subsets/✳️any/🚪️io/🧮️zlib'


def ch(emoji, kebab, mod, ranges, doc, **kw):
    return dict(folder=emoji + V + kebab, mod=mod, ranges=ranges, doc='//! %s%s %s See [`super`].' % (emoji, V, doc), **kw)


def orig(name):
    return os.path.join(T, 'orig', name + '.rs.txt')


def sub(d, skip=(), keep_root=False):
    root = '/Users/ueli/Documents/semio/' + d
    out = []
    for dp, dn, fn in os.walk(root):
        dn[:] = sorted(x for x in dn if x not in skip and x not in ('target', 'node_modules', '.git'))
        for f in sorted(fn):
            if f.endswith('.rs'):
                p = os.path.join(dp, f)
                if dp == root and f == '🦀️.rs' and not keep_root:
                    continue
                out.append(p)
    return out


def folders(children):
    return {c['folder'] for c in children}


def outside(tag):
    import json
    return json.load(open(os.path.join(T, 'pubusers.json'), encoding='utf-8'))[tag]
