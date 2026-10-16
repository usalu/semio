# 📏 Estimated size of every crate of a dump: lines, bytes and files of the output files under the crate's node directory (excluding test directories), plus unchanged files found on disk under the node that the crate root mounts are NOT counted (use census for the exact number).
import json, os, sys, collections
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)) + '/../okwp0-tools')
import okcrates as C
cr, model, cargo = C.build()
d = json.load(open(sys.argv[1]))
out = d['out']
R = C.R
for k in d['batch']:
    node = R + '/' + cr[k]['node']
    files = {p: t for p, t in out.items() if p.startswith(node + '/') and '/🧪️tests/' not in p.replace(node, '')}
    lines = sum(len(t.split('\n')) for t in files.values()); by = sum(len(t.encode()) for t in files.values())
    print(f'{k:36} files {len(files):3} lines {lines:5} bytes {by:7}')
