# 🌐️ WP4 external overlay: rewrites every ShellState consumer outside the Shell targets tree into <overlay>/<path relative to the repo>.
import sys, os, re, json, subprocess, collections
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from t1ext import *
R = '/Users/ueli/Documents/semio'
E = '🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine'

def candidate_files():
    out = set()
    for needle in ('shell.', 'ShellState', 'persist_ui_preferences', 'UiPrefsSnapshot', 'tutorial_apply_', 'tutorial_capture_'):
        r = subprocess.run(['git', '-c', 'core.quotepath=false', '-c', 'core.fsmonitor=false', 'grep', '-l', '-F', needle, '--', E + '/*.rs', ':!' + E + '/🧱️elements/🐚️Shell/🎯️targets/*'], cwd=R, capture_output=True, text=True)
        out |= {l for l in r.stdout.split('\n') if l.endswith('.rs')}
    return sorted(out)

def main(overlay):
    report = []; changed = []
    for rel in candidate_files():
        p = os.path.join(R, rel)
        if not os.path.exists(p): continue
        so = os.path.join(os.environ.get('RWP4_EXT_BASE', '/nonexistent'), rel)
        raw = open(so if os.path.exists(so) else p, encoding='utf-8').read()
        new = rewrite_external(raw, report, rel)
        if new != raw:
            q = os.path.join(overlay, rel); os.makedirs(os.path.dirname(q), exist_ok=True)
            open(q, 'w', encoding='utf-8').write(new); changed.append(rel)
    return changed, report

if __name__ == '__main__':
    changed, report = main(sys.argv[1])
    print(len(changed), 'files changed', len(report), 'reports')
    for r in report[:40]: print(r)
    c = collections.Counter(f.split('/🧱️elements/')[-1].split('/')[0] if '🧱️elements' in f else 'target' for f in changed)
    print(dict(c))
    json.dump(changed, open(GEN + '/rwp4-ext-changed.json', 'w'), ensure_ascii=False, indent=0)
