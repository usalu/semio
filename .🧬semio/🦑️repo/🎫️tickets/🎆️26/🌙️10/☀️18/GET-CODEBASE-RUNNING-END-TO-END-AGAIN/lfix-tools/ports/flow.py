import sys, re, os
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from wrapper import *
D = '/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/'
CTX = D + '🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🧰️context/🦀️.rs'
CTORS = re.compile(r'\bflow_app(?:_closing|_with_registry|_with_actor)?\s*\(')
TOK = re.compile(r'(?<![A-Za-z0-9_])FlowApp(?![A-Za-z0-9_])')
for dp, dn, fn in os.walk(D):
    for f in fn:
        p = os.path.join(dp, f)
        if not f.endswith('.rs') or p == CTX or '🧪️' not in p:
            continue
        s = read(p)
        if not (TOK.search(s) or CTORS.search(s)):
            continue
        s2, c, r = user_port(p, 'FlowAppFixture', 'FlowApp', rename_raw=True, extra_roots=('app', 'reopened', 'fresh', 'instance_a', 'instance_b', 'sender', 'receiver', 'other'))
        print(p.replace(D, '')[-70:], c)
