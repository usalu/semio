"""📥 c4p43 installer for the include-fragment split (cut): proves the live file still equals the snapshot, writes fragments first, then the root atomically."""
import json, os, shutil, sys
HERE = os.path.dirname(os.path.abspath(__file__))
REPO = '/Users/ueli/Documents/semio'
spec = json.load(open(os.path.join(HERE, 'specs-frag', 'cut.json'), encoding='utf-8'))
live = os.path.join(REPO, spec['file'])
assert open(live, 'rb').read() == open(spec['original'], 'rb').read(), 'live file differs from the snapshot'
base = os.path.dirname(live)
stage_base = spec['stage'] + '/' + os.path.dirname(spec['file'])
for c in spec['chunks']:
    assert not os.path.exists(os.path.join(base, c['folder'])), ('exists', c['folder'])
for c in spec['chunks']:
    os.makedirs(os.path.join(base, c['folder']))
    shutil.copyfile(os.path.join(stage_base, c['folder'], '🦀️.rs'), os.path.join(base, c['folder'], '🦀️.rs'))
tmp = live + '.c4p43-tmp'
shutil.copyfile(spec['stage'] + '/' + spec['file'], tmp)
os.replace(tmp, live)
print('installed cut')
