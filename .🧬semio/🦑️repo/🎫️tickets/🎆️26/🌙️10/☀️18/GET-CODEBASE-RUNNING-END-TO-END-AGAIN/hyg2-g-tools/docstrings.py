import re, sys, os
SKIP = {'tests', 'fixtures', 'oracle', 'oracles'}
EMO = re.compile(r'^([^A-Za-z0-9\-_.]+)')
def emoji_of(seg):
    m = EMO.match(seg)
    return m.group(1) if m else ''
def name_of(seg):
    return re.sub(r'^[^A-Za-z0-9]+', '', seg)
def plan(path, kind):
    parts = path.split('/')
    dirs = parts[:-1]
    ext = os.path.splitext(path)[1]
    emo = ''
    for seg in reversed(dirs):
        if name_of(seg) in SKIP or name_of(seg) == 'tests':
            continue
        emo = emoji_of(seg)
        if emo:
            break
    if not emo:
        emo = '🧪️'
    idx = [i for i, s in enumerate(dirs) if name_of(s) in ('tests', 'fixtures', 'oracle', 'oracles')]
    cut = idx[0] if idx else len(dirs)
    before = [name_of(s) for s in dirs[:cut] if name_of(s) not in SKIP][-2:]
    after = [name_of(s) for s in dirs[cut:] if name_of(s) not in SKIP]
    subject = ' '.join(before + after) or 'module'
    if ext == '.py':
        text = 'Independent oracle for ' + subject + '.'
    elif ext == '.mjs':
        text = 'Fixture build for ' + subject + '.'
    elif any(name_of(s) == "fixtures" for s in path.split("/")[:-1]):
        text = 'Fixture for ' + subject + '.'
    else:
        text = 'Tests for ' + subject + '.'
    return emo, text
if __name__ == '__main__':
    for line in sys.stdin:
        kind, t, path = line.rstrip('\n').split('\t')
        emo, text = plan(path, kind)
        print(kind + '\t' + emo + '\t' + text + '\t' + path)
