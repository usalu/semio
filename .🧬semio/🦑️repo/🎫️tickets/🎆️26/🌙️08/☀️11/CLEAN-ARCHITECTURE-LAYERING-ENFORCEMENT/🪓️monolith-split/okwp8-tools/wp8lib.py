"""🧰 Shared helpers of the WP8 tools: snapshot-aware writer, relative paths, touched log."""
import os, shutil, json
R = '/Users/ueli/Documents/semio'
HERE = os.path.dirname(os.path.abspath(__file__))
GEN = HERE + '/../🗑️generated'
SCR = '/private/tmp/claude-501/-Users-ueli--claude-spares-spare-PBncC5/6c384ddf-f4e2-43bf-8d3c-c66daff05e49/scratchpad'
PRE = SCR + '/okwp8-pre'
TOUCHED = HERE + '/touched.txt'
M_REL = '🧰️framework/🛍️products/💻️os/🔨️modules'
K_REL = '🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust'
S_REL = M_REL + '/🏪️store'
def rel(p):
    return p[len(R) + 1:] if p.startswith(R + '/') else p
def snap(path):
    r = rel(path)
    dst = PRE + '/' + r
    if os.path.exists(R + '/' + r) and not os.path.exists(dst):
        os.makedirs(os.path.dirname(dst), exist_ok=True)
        shutil.copy2(R + '/' + r, dst)
def log_touch(path, how):
    r = rel(path)
    with open(TOUCHED, 'a', encoding='utf8') as f: f.write(how + '\t' + r + '\n')
def write(path, text, how='W'):
    assert path.startswith(R + '/'), path
    assert len(rel(path).encode('utf8')) <= 240, ('path too long', rel(path))
    snap(path)
    os.makedirs(os.path.dirname(path), exist_ok=True)
    existed = os.path.exists(path)
    open(path, 'w', encoding='utf8').write(text)
    log_touch(path, 'M' if existed else 'A')
def delete(path):
    snap(path)
    if os.path.exists(path):
        os.remove(path); log_touch(path, 'D')
def read(path): return open(path, encoding='utf8').read()
def relpath_from(src_file, tgt_file):
    return os.path.relpath(tgt_file, os.path.dirname(src_file))
