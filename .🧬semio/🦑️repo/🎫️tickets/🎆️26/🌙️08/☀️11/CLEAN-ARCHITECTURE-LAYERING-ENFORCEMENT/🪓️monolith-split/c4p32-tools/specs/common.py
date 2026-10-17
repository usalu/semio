import os
REPO = '/Users/ueli/Documents/semio'
V = '️'
SKIP = {'target', 'node_modules', '.git', '🗑️generated'}


def rs_under(rel_dir, exclude=()):
    out = []
    for dp, dn, fn in os.walk(os.path.join(REPO, rel_dir)):
        dn[:] = sorted(d for d in dn if d not in SKIP)
        for f in sorted(fn):
            if f.endswith('.rs'):
                rel = os.path.relpath(os.path.join(dp, f), REPO)
                if rel not in exclude:
                    out.append(rel)
    return out


def ch(emoji, kebab, mod, ranges, doc, **kw):
    return dict(folder=emoji + V + kebab, mod=mod, ranges=ranges, doc='//! %s%s %s See [`super`].' % (emoji, V, doc), **kw)
