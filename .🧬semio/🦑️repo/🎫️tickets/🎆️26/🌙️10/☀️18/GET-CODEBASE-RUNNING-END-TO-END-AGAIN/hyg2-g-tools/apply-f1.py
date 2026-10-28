import re, sys, os
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from docstrings import plan, name_of
EMOJI_START = re.compile(r'^[☀-➿\U0001F000-\U0001FAFF⬀-⯿©®‼-㊙]')
TEST_NAMES = {'tests', 'fixtures', 'oracle', 'oracles'}
def is_test_file(path):
    dirs = path.split('/')[:-1]
    return any(name_of(s) in TEST_NAMES for s in dirs)
def apply(path, emo, text):
    ext = os.path.splitext(path)[1]
    with open(path, encoding='utf-8', newline='') as f:
        raw = f.read()
    lines = raw.split('\n')
    idx = 0
    if ext == '.py' and lines and lines[0].startswith('#!'):
        idx = 1
    head = lines[idx] if idx < len(lines) else ''
    if ext == '.py':
        if head.startswith('"""'):
            body = head[3:]
            if EMOJI_START.match(body.lstrip()):
                raise SystemExit('already has emoji: ' + path)
            lines[idx] = '"""' + emo + ' ' + body
            action = 'prefix'
        else:
            lines.insert(idx, '"""' + emo + ' ' + text + '"""')
            action = 'insert'
    else:
        if head.startswith('/**') and not head.startswith('/***'):
            body = head[3:].lstrip()
            if EMOJI_START.match(body):
                raise SystemExit('already has emoji: ' + path)
            lines[idx] = '/** ' + emo + ' ' + body
            action = 'prefix'
        else:
            lines.insert(idx, '/** ' + emo + ' ' + text + ' */')
            action = 'insert'
    with open(path, 'w', encoding='utf-8', newline='') as f:
        f.write('\n'.join(lines))
    return action
if __name__ == '__main__':
    mode = sys.argv[1]
    rows = [l.rstrip('\n').split('\t') for l in open(sys.argv[2], encoding='utf-8')]
    for kind, emo, text, path in rows:
        if not is_test_file(path):
            print('\t'.join(['SKIP-NONTEST', kind, emo, text, path]))
            continue
        if mode == 'dry':
            print('\t'.join(['PLAN', kind, emo, text, path]))
        else:
            act = apply(path, emo, text)
            print('\t'.join([act, kind, emo, text, path]))
