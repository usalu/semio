import os
T = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
V = '️'
X = '✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any'
E = X + '/✏️editor'
EN = E + '/⚙️engine'


def ch(emoji, kebab, mod, ranges, doc, **kw):
    return dict(folder=emoji + V + kebab, mod=mod, ranges=ranges, doc='//! %s%s %s See [`super`].' % (emoji, V, doc), **kw)


def orig(name):
    return os.path.join(T, 'orig', name + '.rs.txt')
