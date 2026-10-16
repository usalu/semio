import json, os, re, subprocess, sys, time
ROOT = '/Users/ueli/Documents/semio'
SCR = '/private/tmp/claude-501/-Users-ueli-Documents-semio/11914954-c49c-43e1-9499-97ab9f20751b/scratchpad'
def files():
    fs = [l.strip() for l in open(SCR + '/pwp6-ts-subset.txt', encoding='utf-8') if l.strip()]
    fs += ['🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🎬️media-slots/🎟️collect/🧪️tests/🟦️.ts', '🧰️framework/🛍️products/💻️os/🔨️modules/🚪️io/📸️typed-snapshot/🧪️tests/🟦️.ts']
    return fs
def run(f):
    t = time.time()
    p = subprocess.run(['perl', '-e', 'alarm 300; exec @ARGV', '--', 'bun', 'test', './' + f], cwd=ROOT, capture_output=True, text=True, encoding='utf8', errors='replace')
    txt = p.stdout + p.stderr
    m_pass = re.search(r'\n\s*(\d+) pass', txt); m_fail = re.search(r'\n\s*(\d+) fail', txt)
    fails = sorted(set(re.findall(r'\(fail\) (.*?)(?: \[[\d.]+m?s\])?$', txt, re.M)))
    return dict(rc=p.returncode, passed=int(m_pass.group(1)) if m_pass else None, failed=int(m_fail.group(1)) if m_fail else None, fails=fails, seconds=round(time.time() - t, 1), tail=txt[-1500:])
if __name__ == '__main__':
    out = sys.argv[1]
    res = {}
    for f in files():
        if not os.path.exists(ROOT + '/' + f): print('missing', f); continue
        res[f] = run(f)
        print(f.split('/')[-3] + '/' + f.split('/')[-2], res[f]['passed'], res[f]['failed'], res[f]['rc'], res[f]['seconds'], flush=True)
        json.dump(res, open(out, 'w'), ensure_ascii=False, indent=1)
