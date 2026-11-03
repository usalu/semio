import json, glob, os, subprocess, sys
sys.stdout.reconfigure(encoding='utf8')
HERE = os.path.dirname(os.path.abspath(__file__))
F = os.path.join(HERE, '..', '🧰️F', 'orig')
REPO = 'C:/git/semio'
D = '🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🛋️furniture/'
for f in sorted(glob.glob(os.path.join(F, '*.json'))):
    a = os.path.basename(f)[:-5]
    raw = open(f, encoding='utf8', newline='').read()
    head = subprocess.run(['git', '-C', REPO, 'show', 'HEAD:' + D + a + '/🔣️.json'], capture_output=True).stdout.decode('utf8')
    for name, t in (('orig', raw), ('head', head)):
        if not t:
            print(a, name, 'empty')
            continue
        d = json.loads(t)
        r = None
        for ind in (1, 2):
            if json.dumps(d, ensure_ascii=False, indent=ind) + '\n' == t:
                r = ind
        print(a, name, 'roundtrip indent', r, len(t))
