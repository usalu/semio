# [aud-staged-g] file sets per staged batch (repo-relative, NFC) and pairwise overlaps; read-only
import json, os, re, subprocess, unicodedata, itertools, sys
R = '/Users/ueli/Documents/semio/'
T = R + '.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/'
BUILD = R + '🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/'
LIB = R + '🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/'
PROC = R + '🧰️framework/🔨️modules/🏃️process/'
N = lambda p: unicodedata.normalize('NFC', p.replace(R, ''))
S = {}
m = json.load(open(T + 'route2-g-tools/staged-base.json', encoding='utf-8'))
S['route2-g'] = [N((LIB if k.startswith('lib:') else BUILD) + k.replace('lib:', '', 1)) for k in m]
S['laws-6b'] = [N(BUILD + '🗂️suites/🔣️.json'), N(PROC + '🧪️testing/🗂️suites/🔣️.json'), N(PROC + '📋️project.json'),
                N(LIB + '⚡️caching/🗂️suites/🔣️.json'), N(LIB + '⚡️caching/📋️project.json')]
sf = set()
for f in ['store-g-tools/staged/apply-foldsafe-preflight.py', 'store-g-tools/staged/apply-managed-parent-guard.py']:
    for s in re.findall(r'"(🧰️framework/[^"]+?\.(?:ts|json|sh|py|mjs|md|ps1|rs|toml|yaml|txt))"', open(T + f, encoding='utf-8').read()):
        sf.add(N(R + s))
NS = R + '🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/'
sf = {N(NS + '📁️directory/🟦️.ts'), N(R + '🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/💾️storage/📁️directory/🟦️.ts'),
      N(LIB + '⚡️caching/🚀️bootstrap/🛠️tools/🏗️bazel/🩺️readiness/🟦️.ts'), N(LIB + '🏗️build/🩺️preflight/🟦️.ts'), N(LIB + '🏗️build/🩺️preflight/🧪️tests/🧮️matrix/🐍️oracle/🐍️.py'),
      N(LIB + '🏗️build/🩺️preflight/🧪️tests/🧮️matrix/🔣️.json'), N(LIB + '🏗️build/🩺️preflight/🧪️tests/🧮️matrix/🧬️schema/🔣️.json'), N(LIB + '🏗️build/🩺️preflight/🧬️schema/🔣️.json'),
      N(NS + '🍎️image/👤️owners/🟦️.ts'), N(NS + '🧬️schema/🔣️.json'), N(NS + '🧹️cleanup/🎛️command/🟦️.ts'), N(NS + '🧹️cleanup/📥️receiver/🧬️schema/🔣️.json'),
      N(NS + '🍎️image/👤️owners/🧪️tests/🚧️managed/🟦️.ts'), N(NS + '🧹️cleanup/🎛️command/🧪️tests/⏳️grace/🟦️.ts'), N(NS + '🧪️tests/🧭️imports/🟦️.ts')}
S['store-g'] = sorted(sf)
import ast, subprocess
st = subprocess.run(['python3', T + 'gram2-g-tools/staged/row-table/batch.py', 'status'], capture_output=True, text=True).stdout
S['gram2-row-table'] = [N(R + k) for k in ast.literal_eval(st.strip())['leaf']]
S['nxfix-core'] = [N(R + k) for k in json.load(open(T + 'nxfix-g-tools/staged/core/manifest.json', encoding='utf-8'))]
S['nxfix-nx-patch'] = [N(R + k) for k in json.load(open(T + 'nxfix-g-tools/staged/nx-patch/manifest.json', encoding='utf-8'))]
S['nxlock'] = [N(R + ' '.join(l.split()[1:])) for l in open(T + 'aud-staged-g-tools/out/nxlock.txt', encoding='utf-8') if l.split() and l.split()[0] in ('needs-core', 'pending', 'base', 'applied', 'base-changed')]
S['obsrows'] = [N(R + ' '.join(l.split()[1:])) for l in open(T + 'aud-staged-g-tools/out/obsrows.txt', encoding='utf-8') if l.split() and l.split()[0] in ('base', 'new')]
rb = json.load(open(T + 'rev3fix-g-tools/staged/rev3fix/bases.json', encoding='utf-8'))
S['rev3fix'] = [N(R + k) for k in rb] + [N(R + f['path']) for f in json.load(open(T + 'rev3fix-g-tools/staged/rev3fix/edits.json', encoding='utf-8'))['newFiles']]
S['setup-g'] = [N(R + k) for k in json.load(open(T + 'setup-g-tools/staged/index-edited.json', encoding='utf-8'))]
S['testmem2'] = [N(R + f['file']) for f in json.load(open(T + 'testmem2-g-tools/staged/batch.json', encoding='utf-8'))['files']]
S['gram2-index-chunks'] = [N(R + e['file']) for e in json.load(open(T + 'gram2-g-tools/staged/index-chunks/edits.json', encoding='utf-8'))['edits']]
S['walk-walks'] = [N(R + e['file']) for e in json.load(open(T + 'walk-g-tools/staged/walks/edits.json', encoding='utf-8'))['edits']]
S['walk-footprint'] = [N(R + e['file']) for e in json.load(open(T + 'walk-g-tools/staged/footprint/edits.json', encoding='utf-8'))['edits']]
S['xplat3'] = [N(R + line.rstrip('\n').split('\t', 1)[1]) for line in open(T + 'xplat3-g-tools/bases.tsv', encoding='utf-8') if line.strip()]
S['conv-g'] = [N(R + k) for k in json.load(open(T + 'conv-g-tools/base-hashes.json', encoding='utf-8'))]
S['self-g'] = [N(BUILD + '🏛️sealed/🔣️.json')]
S = {k: sorted(set(v)) for k, v in S.items()}
for k, v in S.items(): print(f'{k}: {len(v)} files')
print('--- overlaps (shared files) ---')
for a, b in itertools.combinations(S, 2):
    common = set(S[a]) & set(S[b])
    if common: print(f'{a} x {b}: {len(common)} -> ' + '; '.join(c[-60:] for c in sorted(common)[:4]))
json.dump({k: v for k, v in S.items()}, open(T + 'aud-staged-g-tools/out/filesets.json', 'w', encoding='utf-8'), ensure_ascii=False, indent=0)
