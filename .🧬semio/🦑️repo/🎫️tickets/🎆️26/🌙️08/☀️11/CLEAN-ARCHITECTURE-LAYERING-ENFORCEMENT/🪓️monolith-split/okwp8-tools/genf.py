"""🧪 Generates the fixtures crate sources (staging dict path -> text)."""
import sys, os, re, json, collections
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import wp8lib as L, pathrw, vis, buildf as B, themes, prelude as PR
FE = '️'
RSF = '🦀' + FE + '.rs'
FBASE = B.FBASE
TEST_BLOCK = re.compile(r'(?:\n?//#region 🧪' + FE + r'Tests\n)?#\[cfg\(test\)\]\n#\[path = "[^"]*"\]\nmod tests;\n?(?://#endregion 🧪' + FE + r'Tests\n?)?')
def mutation_files():
    out = []
    base = FBASE + '/🧬' + FE + 'mutations'
    for dp, dn, fn in os.walk(base):
        for f in fn:
            if f == RSF: out.append(os.path.join(dp, f))
    return sorted(out)
def edit_mutation_file(path):
    t = L.read(path)
    rw = pathrw.Rewriter('f', depth=99, super_ok=False)
    t2 = rw.rewrite(t)
    t2 = TEST_BLOCK.sub('', t2)
    t2 = re.sub(r'\bpub\(crate\) (enum|struct)\b', r'pub \1', t2)
    t2 = t2.rstrip('\n') + '\n'
    return t2, rw.unresolved
if __name__ == '__main__':
    import difflib
    for f in mutation_files():
        t, un = edit_mutation_file(f)
        o = L.read(f)
        d = list(difflib.unified_diff(o.split('\n'), t.split('\n'), lineterm='', n=0))
        print('###', f.split('🧬' + FE + 'mutations/')[-1], 'unresolved', un, 'difflines', len(d))
        if len(sys.argv) > 1 and sys.argv[1] in f: print('\n'.join(d))
