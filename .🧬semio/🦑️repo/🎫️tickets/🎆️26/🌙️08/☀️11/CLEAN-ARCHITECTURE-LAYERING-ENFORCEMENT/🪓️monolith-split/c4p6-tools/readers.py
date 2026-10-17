"""Anchored, journalled edits of the source-text readers and registries that name the c4p6 files.
Every edit re-reads its file, asserts that its anchor occurs exactly as expected, writes through a temporary file and logs to
F/🗑️generated/c4p6-edits.log.   usage: readers.py <edit-name>|all [--dry]"""
import json, os, re, sys, time

R = '/Users/ueli/Documents/semio'
F = R + '/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split'
LOG = F + '/🗑️generated/c4p6-edits.log'
DRY = '--dry' in sys.argv

P5 = '✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d'
E5 = P5 + '/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor'
E3 = '✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor'
LIB = '🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library'
COMPLETION = '🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧫️fixtures/⏳️completion/🔣️.json'
PUBAUTH = '✏️s/🔌️plugins/🧩️puzzle/🧪️tests/🧫️fixtures/🔏️publication-authority/🔣️.json'
SOURCE_ACCESS = LIB + '/🔍️discovery/📖️source-access/🟦️.ts'
RATCHET = LIB + '/🧪️tests/🧫️fixtures/📏️lib-wiring-ratchet/🔣️.json'


def read(rel):
    with open(os.path.join(R, rel), encoding='utf-8') as fh:
        return fh.read()


def write(rel, old, new):
    cur = read(rel)
    assert cur == old, ('file changed under the edit', rel)
    if DRY:
        print('DRY', rel, len(old), '->', len(new))
        return
    p = os.path.join(R, rel)
    tmp = p + '.c4p6-tmp'
    with open(tmp, 'w', encoding='utf-8') as fh:
        fh.write(new)
    os.replace(tmp, p)
    with open(LOG, 'a', encoding='utf-8') as fh:
        fh.write('%s\t%s\t%d->%d bytes\n' % (time.strftime('%H:%M:%S'), rel, len(old.encode()), len(new.encode())))
    print('edited', rel)


def once(text, anchor, label):
    assert text.count(anchor) == 1, (label, text.count(anchor), anchor[:100])
    return text


def edit_completion():
    t = read(COMPLETION)
    o = t
    old = E5 + '/🖌️session/🦀️.rs'
    for family, folder in (('puzzle5d-copy', '📋️clipboard'), ('puzzle5d-cut', '📋️clipboard'), ('puzzle5d-paste', '📌️paste'), ('puzzle5d-import', '📥️snapshot-import')):
        head = '"family": "%s",\n        "source": "%s"' % (family, old)
        once(t, head, family)
        t = t.replace(head, '"family": "%s",\n        "source": "%s/%s/🦀️.rs"' % (family, E5 + '/🖌️session', folder))
    for job in ('Paste', 'Import'):
        a = '"sourceStart": "\\npub(super) struct Puzzle5d%sJob {"' % job
        once(t, a, job)
        t = t.replace(a, '"sourceStart": "\\npub(in super::super) struct Puzzle5d%sJob {"' % job)
    write(COMPLETION, o, t)


def edit_pubauth():
    t = read(PUBAUTH)
    o = t
    block_start = t.index('"owner": "Puzzle5dPlayApp"')
    head, tail = t[:block_start], t[block_start:]
    m = re.search(r'"mounted": \[\n(.*?)\n      \]', tail, re.S)
    assert m, 'mounted block'
    rows = re.findall(r'"([^"]+)"', m.group(1))
    kids = {
        '🖌️session/🦀️.rs': ['📋️clipboard', '📌️paste', '📥️snapshot-import'],
        '👆️gestures/🦀️.rs': ['🎛️selection-and-weights', '🩹️patches', '🔗️fastener-edits', '🧲️proximity-connect'],
        '🧰️kit/🦀️.rs': ['➕️add-node-and-brush-part', '🧱️board-events-and-fasteners', '📚️example-and-brush-mesh', '💾️store-preparation'],
    }
    out = []
    for r in rows:
        out.append(r)
        for k in kids.get(r, []):
            out.append('%s/%s/🦀️.rs' % (r.split('/')[0], k))
    assert len(out) == len(rows) + 11, (len(out), len(rows))
    new_block = ',\n'.join('        "%s"' % r for r in out)
    tail = tail[:m.start(1)] + new_block + tail[m.end(1):]
    write(PUBAUTH, o, head + tail)


def edit_source_access():
    t = read(SOURCE_ACCESS)
    o = t
    anchor = '  "' + E5 + '/🦀️.rs",\n  "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs",\n];'
    once(t, anchor, 'split roots tail')
    add = '  "' + E5 + '/🧪️tests/🔬️unit/🦀️.rs",\n  "' + E3 + '/⏳️precompute/📐️geometry/🦀️.rs",\n'
    for line in add.strip('\n').split('\n'):
        assert t.count(line.strip().rstrip(',')) == 0, ('already registered', line)
    t = t.replace(anchor, anchor[:-2] + add + '];')
    write(SOURCE_ACCESS, o, t)


def lines_of(rel):
    return len(read(rel).split('\n'))


def edit_ratchet():
    t = read(RATCHET)
    o = t
    geo = '    "' + E3 + '/⏳️precompute/📐️geometry/🦀️.rs": '
    bru = '    "' + E3 + '/⏳️precompute/🖌️brush/🦀️.rs": '
    r5 = '    "' + P5 + '/🦀️.rs": '
    for pin in (geo, bru):
        m = re.search(re.escape(pin) + r'\d+,\n', t)
        assert m and len(re.findall(re.escape(pin), t)) == 1, pin[-60:]
        assert lines_of(pin[5:pin.index('": ')]) <= 150, ('root still above the wiring budget', pin[-60:])
        t = t[:m.start()] + t[m.end():]
    m = re.search(re.escape(r5) + r'(\d+),\n', t)
    assert m
    n = lines_of(P5 + '/🦀️.rs')
    assert n < int(m.group(1)), ('pinned roots only shrink', n, m.group(1))
    t = t[:m.start(1)] + str(n) + t[m.end(1):]
    write(RATCHET, o, t)


EDITS = dict(completion=edit_completion, pubauth=edit_pubauth, sourceaccess=edit_source_access, ratchet=edit_ratchet)

if __name__ == '__main__':
    names = [a for a in sys.argv[1:] if not a.startswith('--')]
    if names == ['all']:
        names = list(EDITS)
    for n in names:
        EDITS[n]()
