import os, sys, subprocess, shutil, importlib, json
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import pfa
REPO = pfa.REPO
SCR = pfa.SCR
PLUGINS = {
    'draw': ('✏️s/🔌️plugins/🖍️draw', {'drawing': '✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing'}),
}
EXTRA = ['🧰️framework/🔨️modules/🧪️test/🛬️original-snapshot-owners']


def sh(cmd, cwd=None, check=True):
    r = subprocess.run(cmd, shell=True, cwd=cwd, capture_output=True, text=True)
    if check and r.returncode != 0:
        raise SystemExit('FAILED: %s\n%s' % (cmd, r.stderr[-800:]))
    return r


def assert_scratch(path):
    top = sh('git rev-parse --show-toplevel', cwd=path).stdout.strip()
    assert os.path.realpath(top) == os.path.realpath(path), (top, path)
    assert os.path.realpath(path).startswith(os.path.realpath(SCR)), path


def build(plugin):
    pdir, arts = PLUGINS[plugin]
    root = os.path.join(SCR, 'ab', plugin, 'repo')
    shutil.rmtree(root, ignore_errors=True)
    os.makedirs(root)
    sh('git init -q', cwd=root)
    assert_scratch(root)
    sh("git -C '%s' -c core.fsmonitor=false -c core.quotepath=false archive HEAD '%s' | tar -x -C '%s'" % (REPO, pdir, root))
    for extra in EXTRA:
        src = os.path.join(REPO, extra)
        if os.path.isdir(src):
            sh("git -C '%s' -c core.fsmonitor=false -c core.quotepath=false archive HEAD '%s' | tar -x -C '%s'" % (REPO, extra, root), check=False)
    sh('git add -A && git -c user.name=c4p4 -c user.email=c4p4@example.invalid -c commit.gpgsign=false -c core.hooksPath=/dev/null commit -q -m base', cwd=root)
    return root


def overlay_wt(plugin, root):
    pdir, arts = PLUGINS[plugin]
    assert_scratch(root)
    sh("rsync -a --delete --exclude .git --exclude target --exclude node_modules '%s/%s/' '%s/%s/'" % (REPO, pdir, root, pdir))
    for extra in EXTRA:
        if os.path.isdir(os.path.join(REPO, extra)):
            sh("rsync -a --delete '%s/%s/' '%s/%s/'" % (REPO, extra, root, extra))


def revert_mine(plugin, root, tags):
    for tag in tags:
        sp = importlib.import_module('specs.' + tag).SPEC
        base = os.path.join(root, os.path.dirname(sp['file']))
        for c in sp['children']:
            shutil.rmtree(os.path.join(base, c['folder']), ignore_errors=True)
        shutil.copyfile(os.path.join(pfa.T, 'orig', sp['orig']), os.path.join(base, '🦀️.rs'))


def gate(root, art, label):
    import re
    sbx = os.path.join(SCR, 'ab', 'sbx-' + label)
    r = None
    for attempt in range(80):
        for suffix in ('', '-base', '-combined'):
            shutil.rmtree(sbx + suffix, ignore_errors=True)
        r = subprocess.run(['bun', os.path.join(pfa.T, 'abgate.ts'), root, art, sbx], capture_output=True, text=True, cwd=REPO)
        m = re.search(r'(?m)^error: Missing Rust source input: (.+)$', r.stderr + r.stdout)
        if m and not any(l.startswith('ABGATE ') for l in r.stdout.split('\n')):
            rel = m.group(1)
            src = os.path.join(REPO, rel)
            assert os.path.isfile(src), ('missing in the real repo too', rel)
            dst = os.path.join(root, rel)
            assert os.path.realpath(root).startswith(os.path.realpath(SCR))
            os.makedirs(os.path.dirname(dst), exist_ok=True)
            shutil.copyfile(src, dst)
            continue
        break
    line = [l for l in r.stdout.split('\n') if l.startswith('ABGATE ')]
    if not line:
        print('NO RESULT', (r.stderr + r.stdout)[-1500:])
        return None
    return json.loads(line[0][7:])


if __name__ == '__main__':
    plugin = sys.argv[1]
    arts_sel = sys.argv[2].split(',')
    tags = sys.argv[3].split(',')
    root = build(plugin)
    overlay_wt(plugin, root)
    pdir, arts = PLUGINS[plugin]
    res = {}
    for art in arts_sel:
        res[art + ':with-split'] = gate(root, arts[art], art + '-A')
    revert_mine(plugin, root, tags)
    for art in arts_sel:
        res[art + ':split-reverted'] = gate(root, arts[art], art + '-B')
    for k, v in res.items():
        if v:
            print(k, {m: (d['base'], d['combined'], d['onlyCombined']) for m, d in v['layout'].items()})
    out = os.path.join(os.path.dirname(pfa.T), '🗑️generated', 'c4p4-ab-%s.json' % plugin)
    json.dump(res, open(out, 'w', encoding='utf-8'), ensure_ascii=False, indent=1)
