"""😀 c4p18 docstring-emoji check: the first `//!` line emoji of every staged (or live, LIVE=1) new file must be unique among the first-line emoji of all .rs files of the artifact directory.
usage: emojis.py <tag>..."""
import os, re, subprocess, sys, importlib, collections
T = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, T)
import c4p18run as R

ROOT = R.REPO
FIRST = re.compile(r'^//[!/]? (\S+)')
ART = {'envelope': '✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d', 'gisedit': '✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap', 'owned': '✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap', 'gisroot': '✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap', 'vcsedit': '✏️s/🔌️plugins/🌿️vcs/🗿️artifacts/🌿️vcs', 'animation': '✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️presentation', 'video': '✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️presentation', 'scene': '✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️presentation'}
cache = {}


def firsts_of(art):
    if art in cache:
        return cache[art]
    out = subprocess.run(['git', '-c', 'core.fsmonitor=false', '-c', 'core.quotepath=false', 'ls-files', '--cached', '--others', '--exclude-standard', '--', art + '/**/*.rs', art + '/*.rs'], cwd=ROOT, capture_output=True, text=True).stdout.split('\n')
    m = collections.defaultdict(list)
    for rel in out:
        if not rel or '/node_modules/' in rel:
            continue
        try:
            with open(os.path.join(ROOT, rel), encoding='utf-8') as fh:
                line = fh.readline().rstrip('\n')
        except OSError:
            continue
        mm = FIRST.match(line)
        if mm and not mm.group(1).isascii():
            m[mm.group(1)].append(rel)
    cache[art] = m
    return m


bad = 0
for tag in sys.argv[1:]:
    sp = importlib.import_module('specs.' + tag).SPEC
    files, _ = R.read_stage(tag)
    base = os.path.dirname(sp['file'])
    mine = {os.path.normpath(os.path.join(base, rel)) for rel in files if rel != '🦀️.rs'}
    m = firsts_of(ART[tag])
    for rel in sorted(files):
        if rel == '🦀️.rs':
            continue
        first = FIRST.match(files[rel].split('\n')[0])
        e = first.group(1) if first else None
        if e is None or e.isascii():
            print('  NO-EMOJI-DOC', tag, rel)
            bad += 1
            continue
        full = os.path.normpath(os.path.join(base, rel))
        others = [p for p in m.get(e, []) if p != full and p not in mine]
        sib = [r2 for r2 in files if r2 != rel and r2 != '🦀️.rs' and (FIRST.match(files[r2].split('\n')[0]) or [0, None])[1] == e]
        if others or sib:
            print('  CLASH', tag, rel.split('/')[-2], e, 'with', [o.split('/')[-2] + '/' + o.split('/')[-1] for o in others[:3]], sib[:3])
            bad += 1
print('emoji clashes', bad)
