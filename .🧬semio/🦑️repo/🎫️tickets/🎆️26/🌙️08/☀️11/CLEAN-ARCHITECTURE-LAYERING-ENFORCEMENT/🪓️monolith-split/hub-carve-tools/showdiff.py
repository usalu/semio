import sys, difflib, os
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
REPO='/Users/ueli/Documents/semio'
def show(ws, rel, maxlines=80):
    p = os.path.join(REPO, rel) if not rel.startswith('/') else rel
    cur = ws.read(p)
    try: old = open(p).read()
    except FileNotFoundError: old = ''
    d = list(difflib.unified_diff(old.splitlines(), cur.splitlines(), 'a/'+rel[-50:], 'b/'+rel[-50:], lineterm='', n=1))
    print('\n'.join(l[:230] for l in d[:maxlines]))
    if len(d) > maxlines: print('... (%d more diff lines)' % (len(d)-maxlines))
