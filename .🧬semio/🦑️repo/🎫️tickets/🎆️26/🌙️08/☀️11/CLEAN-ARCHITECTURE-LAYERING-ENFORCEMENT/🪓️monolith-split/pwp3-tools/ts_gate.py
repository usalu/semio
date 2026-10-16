"""🧪️ Runs every TS test that reads plugin Rust sources (141 files under the plugin folder that name a `🦀️.rs` plus the extra readers found by item name) with `bun test`, one at a time, and records per file the pass/fail counts and failing test names. usage: python3 ts_gate.py <out.json>"""
import json, os, re, subprocess, sys, time
ROOT = '/Users/ueli/Documents/semio'
PLUG = '🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin'
def candidates():
    out = subprocess.run(['git', '-c', 'core.quotepath=false', 'grep', '-l', '-F', '🦀️.rs', '--', PLUG + '/*.ts'], cwd=ROOT, capture_output=True, text=True).stdout.split('\n')
    files = [f for f in out if f.endswith('.ts') and '/📦️packages/' not in f]
    extra = ['🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🧩️extensions/🕸️wasm/🧪️tests/🔬️unit/🟦️.ts']
    return sorted(set(files + extra))
def run(f):
    t = time.time()
    p = subprocess.run(['perl', '-e', 'alarm 240; exec @ARGV', '--', 'bun', 'test', './' + f], cwd=ROOT, capture_output=True, text=True, encoding='utf8', errors='replace')
    txt = p.stdout + p.stderr
    m_pass = re.search(r'\n\s*(\d+) pass', txt); m_fail = re.search(r'\n\s*(\d+) fail', txt)
    fails = sorted(set(re.findall(r'\(fail\) (.*?)(?: \[[\d.]+m?s\])?$', txt, re.M)))
    return dict(rc=p.returncode, passed=int(m_pass.group(1)) if m_pass else None, failed=int(m_fail.group(1)) if m_fail else None, fails=fails, seconds=round(time.time() - t, 1), tail=txt[-300:] if p.returncode not in (0, 1) else '')
if __name__ == '__main__':
    out = sys.argv[1]
    res = json.load(open(out)) if os.path.exists(out) and os.environ.get('RESUME') == '1' else {}
    only = sys.argv[2:]
    for f in (only or candidates()):
        if f in res: continue
        res[f] = run(f)
        print(f[-90:], res[f]['passed'], res[f]['failed'], res[f]['rc'], res[f]['seconds'], flush=True)
        json.dump(res, open(out, 'w'), ensure_ascii=False, indent=1)
