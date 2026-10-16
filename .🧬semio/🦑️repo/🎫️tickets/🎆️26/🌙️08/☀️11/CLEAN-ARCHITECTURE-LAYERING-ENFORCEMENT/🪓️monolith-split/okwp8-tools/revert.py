"""↩️ Restores the pristine tree for every file touched by the WP8 tools (snapshots in the scratchpad), so the generator can run again from original sources."""
import sys, os, shutil
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import wp8lib as L
def main():
    if not os.path.exists(L.TOUCHED): return
    rows = [l.rstrip('\n').split('\t') for l in open(L.TOUCHED, encoding='utf8') if l.strip()]
    seen = {}
    for how, rel in rows: seen.setdefault(rel, how)
    for rel, how in seen.items():
        pre = L.PRE + '/' + rel; dst = L.R + '/' + rel
        if os.path.exists(pre):
            os.makedirs(os.path.dirname(dst), exist_ok=True)
            shutil.copy2(pre, dst)
        elif how == 'A':
            if os.path.exists(dst): os.remove(dst)
    # remove directories created empty
    for rel, how in seen.items():
        d = os.path.dirname(L.R + '/' + rel)
        while d.startswith(L.R + '/') and os.path.isdir(d) and not os.listdir(d):
            os.rmdir(d); d = os.path.dirname(d)
    os.replace(L.TOUCHED, L.TOUCHED + '.prev') if os.path.exists(L.TOUCHED) else None
    print('reverted', len(seen))
if __name__ == '__main__': main()
