import json, os, re, sys
REPO = '/Users/ueli/Documents/semio'
C = sys.argv[1]
FE = '️'
def norm(s): return s.replace(FE, '')
suites = []
for sj, base in [
  ('🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🗂️suites/🔣️.json', '🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build'),
  ('🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🗂️suites/🔣️.json', '🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching'),
  ('🧰️framework/🔨️modules/🏃️process/🧪️testing/🗂️suites/🔣️.json', '🧰️framework/🔨️modules/🏃️process/🧪️testing')]:
    d = json.load(open(os.path.join(REPO, sj), encoding='utf-8'))
    for s in d['suites']:
        for t in s.get('trees', []):
            suites.append((norm(base + '/' + t).rstrip('/'), s['id']))
def route(p):
    for t, sid in suites:
        if p == t or p.startswith(t + '/'):
            return sid
    return ''
PAT = {
 'skip': re.compile(r"\.skip\(|skipIf|\bxit\(|\bxdescribe\(|#\[ignore|pytest\.mark\.skip|t\.skip\(|\bit\.todo|\bdescribe\.skip"),
 'envgate': re.compile(r"if\s*\(\s*!process\.env|process\.env\.\w+\s*\)\s*\{?\s*$|SEMIO_[A-Z_]*\s*(?:===|!==)|os\.environ\.get|env::var"),
 'assert': re.compile(r"\bexpect\(|\bassert(_eq|_ne|!|\.|\s*\()|\bt\.(equal|ok|deepEqual|strictEqual)\(|\bok\(|\bstrictEqual\(|\bdeepStrictEqual\(|\bequal\(|\bassertEquals\b"),
 'oracle': re.compile(r"python3?\b|\.py['\"`]|oracle|Oracle|sqlite|numpy|\bspawn\w*\(|execFile|Popen|subprocess"),
 'mock': re.compile(r"vi\.mock\(|jest\.mock\(|vi\.fn\(|jest\.fn\(|\bmock\w*\(|\bstub\w*\("),
 'neg': re.compile(r"negative control|red before|fails? before|before the fix|temporarily|mutant|mutation|must (?:fail|refuse|be red)|control:|NEGATIVE|\bcontrol\b", re.I),
 'import_self': re.compile(r"^import .* from ['\"]\.\/[^'\"]*(?:🟦️)?\.ts['\"]"),
}
rows = []
for p in [l.strip() for l in open(os.path.join(C, '🗑️generated/aud-laws-g/matched.txt'), encoding='utf-8')]:
    path = p.split(' | ')[0].strip()
    who = p.split(' | ')[1].strip() if ' | ' in p else ''
    full = os.path.join(REPO, path)
    if not os.path.isfile(full):
        rows.append([path, 'MISSING', '', '', '', '', '', '', '', who]); continue
    size = os.path.getsize(full)
    if size > 2_000_000:
        rows.append([path, 'BIG', str(size), '', '', '', '', '', '', who]); continue
    cnt = {k: 0 for k in PAT}
    tests = 0
    with open(full, encoding='utf-8', errors='replace') as h:
        for line in h:
            if re.search(r"\b(it|test)\(|#\[test\]|\bdef test_", line): tests += 1
            for k, rx in PAT.items():
                if rx.search(line): cnt[k] += 1
    # result-file mentions with negative-control words near the file/dir
    dirkey = path.split('/')[-2] if '/' in path else ''
    nmention = 0
    for rf in who.split(','):
        if not rf: continue
        txt = norm(open(os.path.join(C, rf), encoding='utf-8', errors='replace').read())
        for line in txt.splitlines():
            if (path.split('/')[-1] in line or dirkey and dirkey in line) and re.search(r'red before|RED|fails? before|temporarily|mutant|negative control|verified by|before:', line, re.I):
                nmention += 1
    rows.append([path, route(norm(path)) or '-', str(size), str(tests), str(cnt['assert']), str(cnt['oracle']), str(cnt['skip']+cnt['envgate']), str(cnt['mock']), str(nmention), who])
hdr = ['path','route','size','tests','asserts','oracle_hits','skip_env','mock','neg_mentions','result_files']
with open(os.path.join(C, '🗑️generated/aud-laws-g/scores.tsv'), 'w', encoding='utf-8') as o:
    o.write('\t'.join(hdr)+'\n')
    for r in rows: o.write('\t'.join(r)+'\n')
print(len(rows), 'rows')
