V = '️'


def ch(emoji, kebab, mod, ranges, doc, **kw):
    return dict(folder=emoji + V + kebab, mod=mod, ranges=[tuple(r) for r in ranges], doc='//! %s%s %s See [`super`].' % (emoji, V, doc), **kw)
