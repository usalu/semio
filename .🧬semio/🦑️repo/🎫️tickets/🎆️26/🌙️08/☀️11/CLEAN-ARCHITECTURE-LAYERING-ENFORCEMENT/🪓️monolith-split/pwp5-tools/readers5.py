"""📖️ Source-text readers of the cut files: for every TS file that names `⏪️time-travel/🦀️.rs`, `⏯️tool-run/🦀️.rs`, `🛠️tool-machine/🦀️.rs`, `🎮️command/🦀️.rs` or `👷️job/🦀️.rs` (git grep, tracked plus new files), lists the string literals (12+ chars) that occur in the ORIGINAL text of a cut file (pre-edit snapshot) but not in the current module tree text of the same file (root plus every mounted child inside its directory, the way `readModuleTreeSource` reads it). A hit is a source-text law whose text moved out of the reader's view. usage: python3 readers5.py"""
import os, re, subprocess, sys
HERE = os.path.dirname(os.path.abspath(__file__)); sys.path.insert(0, HERE)
import lib5 as L
CUT = ['⏪️time-travel/🦀️.rs', '⏯️tool-run/🦀️.rs', '🛠️tool-machine/🦀️.rs', '🎮️command/🦀️.rs', '👷️job/🦀️.rs']
LIT = re.compile(r'#\[\s*path\s*=\s*"([^"]+)"\s*\]|\binclude!\(\s*"([^"]+\.rs)"\s*\)')
def tree_text(path, seen=None, root_dir=None):
    seen = seen if seen is not None else set(); root_dir = root_dir or os.path.dirname(path)
    if path in seen or not os.path.exists(path): return ''
    seen.add(path); t = L.rd(path); out = [t]
    for m in LIT.finditer(t):
        rel = m.group(1) or m.group(2)
        if not rel.endswith('.rs'): continue
        q = os.path.normpath(os.path.join(os.path.dirname(path), rel))
        if not q.startswith(root_dir + '/'): continue
        if '/🧪️tests/' in q: continue
        out.append(tree_text(q, seen, root_dir))
    return '\n'.join(out)
def original_tree_text(rel):
    snap = L.SNAP + '/' + rel
    base = snap if os.path.exists(snap) else L.PLUG + '/' + rel
    seen = set(); root_dir = os.path.dirname(L.PLUG + '/' + rel); out = [L.rd(base)]
    for m in LIT.finditer(out[0]):
        r = m.group(1) or m.group(2)
        if not r.endswith('.rs'): continue
        q = os.path.normpath(os.path.join(root_dir, r))
        if q.startswith(root_dir + '/') and '/🧪️tests/' not in q:
            sq = L.SNAP + '/' + os.path.relpath(q, L.PLUG)
            out.append(L.rd(sq) if os.path.exists(sq) else (L.rd(q) if os.path.exists(q) else ''))
    return '\n'.join(out)
def main():
    files = set()
    for n in CUT:
        r = subprocess.run(['git', '-c', 'core.quotepath=false', 'grep', '-l', '-F', n, '--', '*.ts', '*.tsx'], cwd=L.ROOT, capture_output=True, text=True).stdout.split('\n')
        files |= {f for f in r if f and not f.startswith(('.🧬semio', '.vscode'))}
    others = subprocess.run(['git', '-c', 'core.quotepath=false', 'ls-files', '--others', '--exclude-standard', '--', L.PLUG[len(L.ROOT) + 1:]], cwd=L.ROOT, capture_output=True, text=True).stdout.split('\n')
    files |= {f for f in others if f.endswith('.ts') and any(n in L.rd(L.ROOT + '/' + f) for n in CUT if os.path.exists(L.ROOT + '/' + f))}
    newtexts = {n: tree_text(L.PLUG + '/' + n) for n in CUT}
    oldtexts = {n: original_tree_text(n) for n in CUT}
    total = 0
    for f in sorted(files):
        p = L.ROOT + '/' + f
        if not os.path.exists(p): continue
        t = L.rd(p)
        named = [n for n in CUT if n in t]
        if not named: continue
        lits = set()
        for m in re.finditer(r'"((?:[^"\\\n]|\\.){12,})"|`((?:[^`\\]|\\.){12,})`', t):
            s = (m.group(1) or m.group(2)).encode().decode('unicode_escape', 'ignore') if False else (m.group(1) or m.group(2))
            lits.add(s.replace('\\"', '"').replace('\\\\', '\\'))
        hits = []
        for s in lits:
            for n in named:
                if s in oldtexts[n] and s not in newtexts[n]: hits.append((n, s[:100]))
        print(f.split('🔌️plugin/')[-1][:80], '| names', [n.split('/')[0] for n in named], '| literals', len(lits), '| moved-out hits', len(hits))
        for h in hits[:8]: print('     ', h)
        total += len(hits)
    print('TOTAL moved-out literals', total)
main()
