import sys; sys.path.insert(0,'.')
import wrun3
from wrun3 import *
from wreg import *

def slug(s):
    s = re.sub(r'[^\x00-\x7f]', '', s).strip('-')
    return re.sub(r'[^a-z0-9]+', '_', s.lower()).strip('_')

def main4():
    crates, T, files, P = wrun3.main()
    OUT = wrun3.OUT
    libtexts = {}; testtexts = collections.defaultdict(list); targets = collections.defaultdict(list)
    for c in crates:
        t = c.text
        extra = ''
        if c.id == 'host':
            extra += open(f"{OUT}/🌍️world/{c.folder}/🧰️testkit/🦀️.rs", encoding='utf-8').read()
        if c.id == 'model':
            extra += open(f"{OUT}/🌍️world/{c.folder}/⏯️tool-run-trace/🦀️.rs", encoding='utf-8').read()
        libtexts[c.id] = t + extra
    for cid, path, n in files:
        if '/🧪️tests/' not in path: continue
        text = open(path, encoding='utf-8').read()
        testtexts[cid].append(text)
        d = path.split('/🧪️tests/')[1].split('/')[0]
        targets[cid].append((f"world_{cid}_{slug(d)}", f"../../🧪️tests/{d}/🦀️.rs"))
    siblings = {c.idx: set(c.cross) for c in crates}
    write_manifests(crates, OUT, siblings, libtexts, testtexts, targets)
    return crates, T, files, P, libtexts, testtexts

if __name__ == '__main__':
    crates, T, files, P, libtexts, testtexts = main4()
    for c in crates: print(c.id, sorted(CRATES[k][0] for k in c.cross))
    print(open(wrun3.OUT + '/🌍️world/' + crates[-1].folder + '/📦️packages/🦀️rust/Cargo.toml', encoding='utf-8').read())
