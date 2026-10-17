import os
T = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
V = '️'
M = '🧰️framework/🔨️modules'
FMT = M + '/🎒️pack/📐️format'
JTU = M + '/🎒️pack/🔤️json/🧪️tests/🔬️unit'
ONB = M + '/🎓️onboarding'
MUT = M + '/📡️replication/🎮️mutation'
ING = M + '/📡️replication/📡️wire/🎮️command/📥️ingress'
PRS = M + '/📡️replication/📡️wire/👥️presence'
WIRE = M + '/📡️replication/📡️wire'
EXP = M + '/🔄️machine/✨️derive/⚙️expansion'


def ch(emoji, kebab, mod, ranges, doc, **kw):
    return dict(folder=emoji + V + kebab, mod=mod, ranges=ranges, doc='//! %s%s %s See [`super`].' % (emoji, V, doc), **kw)
