import hashlib, importlib, json, os, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import c4p34 as P
from specs.lib import DXFS
TAGS = 'png jpg avi pptx dxfbin dxfschema zlib'.split()
F = os.path.dirname(P.T)
JOURNAL = os.path.join(F, '🗑️generated', 'c4p34-installed.json')
SCHEMA = os.path.join(P.REPO, DXFS.rsplit('/', 1)[0], '🦀️.rs')
OLD = '            rust: include_str!("🔺️diff/🦀️.rs"),\n'


def sha(b):
    return hashlib.sha256(b).hexdigest()


def facet_new():
    sp = importlib.import_module('specs.dxfschema').SPEC
    parts = ['include_str!("🔺️diff/🦀️.rs")'] + ['include_str!("🔺️diff/%s/🦀️.rs")' % c['folder'] for c in sp['children']]
    return '            rust: concat!(' + ', '.join(parts) + '),\n'


def main():
    cmd = sys.argv[1]
    tags = sys.argv[2:] or TAGS
    if cmd == 'install':
        journal = {}
        for t in tags:
            sp = importlib.import_module('specs.' + t).SPEC
            target = os.path.join(P.REPO, sp['file'])
            before = open(target, 'rb').read()
            P.install(t, importlib.import_module('specs.' + t))
            journal[t] = dict(file=sp['file'], before=sha(before), after=sha(open(target, 'rb').read()), folders=P.rel_folders(sp))
        if 'dxfschema' in tags:
            text = open(SCHEMA, encoding='utf-8').read()
            assert text.count(OLD) == 1, ('facet anchor', text.count(OLD))
            open(SCHEMA, 'w', encoding='utf-8').write(text.replace(OLD, facet_new()))
            journal['facet'] = dict(file=os.path.relpath(SCHEMA, P.REPO), before=sha(text.encode('utf-8')), after=sha(open(SCHEMA, 'rb').read()))
        old = json.load(open(JOURNAL)) if os.path.exists(JOURNAL) else {}
        old.update(journal)
        json.dump(old, open(JOURNAL, 'w', encoding='utf-8'), ensure_ascii=False, indent=1)
        print('installed', ' '.join(tags))
    elif cmd == 'facet-preview':
        print(facet_new())


main()
