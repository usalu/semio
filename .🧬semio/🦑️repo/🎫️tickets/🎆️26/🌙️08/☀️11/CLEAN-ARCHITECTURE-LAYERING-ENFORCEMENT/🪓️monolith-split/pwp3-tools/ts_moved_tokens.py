"""🧪️ Finds string literals of TS source-text tests that were in a lifted Rust file before (snapshot) and are no longer in it (live): the assertions that must read the lift file instead. usage: python3 ts_moved_tokens.py"""
import json, os, re, subprocess, sys
ROOT = '/Users/ueli/Documents/semio'
SCR = '/private/tmp/claude-501/-Users-ueli-Documents-semio/11914954-c49c-43e1-9499-97ab9f20751b/scratchpad'
PL = '🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/'
FILES = ['🧩️composition/📨️emission/📦️preparation/🦀️.rs', '💡️inference/🦀️.rs', '💡️inference/🚪️gateway/🦀️.rs', '🎮️command/🦀️.rs', '🧵️retained-command/🧬️context/🦀️.rs', '🧵️retained-command/🧬️context/📸️fields/🦀️.rs', '🎞️media/⚠️turn/🦀️.rs', '🎞️media/🛫️intrinsic/🦀️.rs', '👷️job/🏛️reserved/🦀️.rs', '🧵️retained-command/🔄️full-operation/🦀️.rs', '🧩️composition/♻️root/👁️read/🦀️.rs']
before = {}; after = {}
for f in FILES:
    before[f] = open(SCR + '/pwp3-snap/' + f, encoding='utf8').read() if os.path.exists(SCR + '/pwp3-snap/' + f) else open(ROOT + '/' + PL + f, encoding='utf8').read()
    after[f] = open(ROOT + '/' + PL + f, encoding='utf8').read() if os.path.exists(ROOT + '/' + PL + f) else ''
cands = subprocess.run(['git', '-c', 'core.quotepath=false', 'grep', '-l', '-F', '🦀️.rs', '--', PL + '*.ts'], cwd=ROOT, capture_output=True, text=True).stdout.split('\n')
LIT = re.compile(r'"((?:[^"\\\n]|\\.)*)"|\'((?:[^\'\\\n]|\\.)*)\'|`((?:[^`\\]|\\.)*)`')
def unesc(s):
    try: return bytes(s, 'utf8').decode('unicode_escape').encode('latin-1').decode('utf8') if '\\' in s else s
    except Exception: return s
out = []
for c in sorted(x for x in cands if x.endswith('.ts') and '/📦️packages/' not in x):
    t = open(ROOT + '/' + c, encoding='utf8').read()
    for m in LIT.finditer(t):
        s = m.group(1) or m.group(2) or m.group(3) or ''
        s = unesc(s)
        if len(s) < 14 or '🦀️.rs' in s: continue
        for f in FILES:
            if s in before[f] and s not in after[f]:
                out.append((c.replace(PL, '')[:80], f[-40:], s[:90])); break
seen = set()
for o in out:
    if o in seen: continue
    seen.add(o); print(o)
print(len(seen), 'literals that moved out of their file')
