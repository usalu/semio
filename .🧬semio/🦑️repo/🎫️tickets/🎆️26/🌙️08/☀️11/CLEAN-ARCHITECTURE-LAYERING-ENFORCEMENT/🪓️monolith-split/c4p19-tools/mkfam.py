import json, os, subprocess
T = os.path.dirname(os.path.abspath(__file__))
POS = os.path.join(T, '..', 'pos-tools', 'pos2fam.py')
FAM = {
 'a': ['semio-s-artifact-norm-iso16757-diff'],
 'b': ['semio-s-artifact-norm-en1994-diff'],
 'c': ['semio-s-artifact-norm-din4108-checks'],
 'd': ['semio-s-artifact-norm-en1996-schema'],
 'e': ['semio-s-artifact-norm-en1995-schema'],
 'f': ['semio-s-artifact-norm-en1999-schema', 'semio-s-artifact-norm-en1999'],
 'g': ['semio-s-artifact-norm-en1998-inferences'],
 'h': ['semio-s-plugin-playbook-procedural'],
}
os.makedirs(os.path.join(T, 'families'), exist_ok=True)
for k, own in FAM.items():
    crates = list(own)
    for p in own:
        out = subprocess.run(['python3', POS, 'rdeps', p], capture_output=True, text=True, cwd='/Users/ueli/Documents/semio').stdout.split()
        for c in out:
            if c not in crates:
                crates.append(c)
    d = {'name': 'c4p19' + k, 'crates': crates, 'note': 'CENSUS4-PKG-19 ' + ','.join(own) + ' plus direct dependents', 'base': {'rev': 'b84bf526c7b', 'crates': crates}}
    json.dump(d, open(os.path.join(T, 'families', 'c4p19%s.json' % k), 'w', encoding='utf-8'), ensure_ascii=False, indent=1)
    print(k, len(crates))
