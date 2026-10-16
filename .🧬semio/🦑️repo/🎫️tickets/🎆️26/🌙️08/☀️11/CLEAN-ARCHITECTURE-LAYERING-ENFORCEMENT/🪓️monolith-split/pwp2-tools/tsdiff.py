#!/usr/bin/env python3
# 🧾️ Runs the TS laws that read WP2 source files with `bun test ./<file>` and records pass/fail per test name; `tsdiff.py save <tag>` / `tsdiff.py diff <tagA> <tagB>`.
import json, os, re, subprocess, sys
ARGS = list(sys.argv[1:])
from common import *

OS = REPO + '/🧰️framework/🛍️products/💻️os/🔨️modules'
FILES = [
    OS + '/🔌️plugin/⚛️reactor/🎟️context/🌉️call/⚖️failure-debit/🧪️tests/🟦️.ts',
    OS + '/🔌️plugin/🎞️media/⚠️turn/🧪️tests/🟦️.ts',
    OS + '/🔌️plugin/🛠️tool-machine/🖐️gesture/🎟️issuer/🧪️tests/🟦️.ts',
    OS + '/🔌️plugin/🧵️retained-command/🧬️context/🧪️tests/🟦️.ts',
    OS + '/🔌️plugin/🧩️composition/📨️emission/📦️preparation/🎟️receipt/🧪️tests/🟦️.ts',
    OS + '/🔌️plugin/♻️lifecycle/🏪️stores/🧪️tests/🟦️.ts',
    OS + '/💡️inference/🧪️tests/🔬️unit/🟦️.ts',
    REPO + '/🧰️framework/🔨️modules/🛠️tool-machine/🧪️tests/🧪️history-closure-policy/🟦️.ts',
    REPO + '/🧰️framework/🔨️modules/🧵️job/🎟️context/🪙️loan/🧪️tests/🟦️.ts',
    OS + '/🏪️store/🧪️tests/🔬️unit/🟦️.ts',
]

def run(f):
    if not os.path.exists(f):
        return {'MISSING': 'file'}
    p = subprocess.run(['perl', '-e', 'alarm 180; exec @ARGV', '--', 'bun', 'test', './' + os.path.basename(f)], cwd=os.path.dirname(f), capture_output=True, text=True)
    out = p.stdout + p.stderr
    res = {}
    for m in re.finditer(r'^\s*\((pass|fail)\)\s+(.*?)(?:\s+\[[\d.]+m?s\])?$', out, re.M):
        res[m.group(2)] = m.group(1)
    m = re.search(r'(\d+) pass', out); n = re.search(r'(\d+) fail', out)
    res['__summary__'] = f"{m.group(1) if m else 0} pass {n.group(1) if n else 0} fail"
    return res

if __name__ == '__main__':
    if ARGS[0] == 'save':
        data = {os.path.relpath(f, REPO): run(f) for f in FILES}
        json.dump(data, open(WS + '/ts-' + ARGS[1] + '.json', 'w'), ensure_ascii=False, indent=1)
        for k, v in data.items():
            print(v.get('__summary__'), k[-90:])
    else:
        a, b = (json.load(open(WS + '/ts-' + t + '.json')) for t in ARGS[1:3])
        diff = 0
        for k in a:
            for name in sorted(set(a[k]) | set(b.get(k, {}))):
                if a[k].get(name) != b.get(k, {}).get(name):
                    diff += 1
                    print('DIFF', k[-70:], name, a[k].get(name), '->', b.get(k, {}).get(name))
        print('differences:', diff)
