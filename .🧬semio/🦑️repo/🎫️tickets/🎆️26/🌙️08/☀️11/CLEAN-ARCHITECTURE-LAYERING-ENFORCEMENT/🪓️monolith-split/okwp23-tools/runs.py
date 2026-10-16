# 📋 Reads Appendix C of PLAN-OS-KERNEL-CRATES.md (item runs of the multi-owner files) into {file: [(from, to, crate)]}; short store paths are expanded.
import os, re, sys
HERE = os.path.dirname(os.path.abspath(__file__))
PLAN = HERE + '/../PLAN-OS-KERNEL-CRATES.md'
R = '/Users/ueli/Documents/semio'
M = R + '/🧰️framework/🛍️products/💻️os/🔨️modules'

def expand(p):
    if p.startswith('S/'): return M + '/🏪️store/' + p[2:]
    if p.startswith('M/'): return M + '/' + p[2:]
    return p

def load():
    text = open(PLAN, encoding='utf8').read()
    i = text.index('## Appendix C.'); j = text.index('## Appendix D.')
    sec = text[i:j]
    out = {}
    cur = None
    for line in sec.split('\n'):
        m = re.match(r'\*\*`([^`]+)`\*\*', line)
        if m:
            cur = expand(m.group(1).replace('STORE ROOT ', '')); out[cur] = []; continue
        m = re.match(r'\|\s*(\d+)[–-](\d+)\s*\|\s*`([^`]+)`\s*\|', line)
        if m and cur: out[cur].append((int(m.group(1)), int(m.group(2)), m.group(3)))
    return out

if __name__ == '__main__':
    for f, rs in load().items():
        print(f.replace(M + '/', ''), len(rs))
