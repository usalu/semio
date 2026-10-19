#!/usr/bin/env python3
import re, sys, glob, os
M='/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/⌨️cli/🧪️tests'
report = len(sys.argv) > 1 and sys.argv[1] == 'report'
pat_root = re.compile(r'\tcwd, _ := os\.Getwd\(\)\n\trepoRoot := findTestRepoRoot\(cwd\)\n\tSetRootDir\(repoRoot\)\n')
for f in sorted(glob.glob(M + '/*/🐹️.go')):
    s = open(f, encoding='utf8').read()
    parts = re.split(r'(?m)^(?=func )', s)
    out = []
    for p in parts:
        if p.startswith('func TestExhaustive'):
            q = p
            q = pat_root.sub('\trepoRoot := withExhaustiveRepo(t)\n', q)
            q = q.replace('\tcwd, _ := os.Getwd()\n\tSetRootDir(findTestRepoRoot(cwd))\n', '\twithExhaustiveRepo(t)\n')
            q = q.replace('\troot := findTestRepoRoot(".")\n\tSetRootDir(root)\n', '\troot := withExhaustiveRepo(t)\n')
            q = q.replace('\tcwd, _ := os.Getwd()\n\toldRoot := rootDir\n\trootDir = findTestRepoRoot(cwd)\n\tdefer func() { rootDir = oldRoot }()\n\tInvalidateTechnologyCache()\n', '\twithExhaustiveRepo(t)\n')
            q = q.replace('setupToolTest(t)', 'withExhaustiveRepo(t)')
            q = q.replace('getTestExecutor(t)', 'newExecutorFor(t, withExhaustiveRepo(t))')
            if report:
                name = p.split('(')[0]
                hits = [l.strip() for l in q.split('\n') if re.search(r'findTestRepoRoot|Getwd|rootDir|GetRootDir|testRepoRootOverride|withTempRepo|withMonorepoFixture|os\.Remove|WriteFile', l)]
                if hits: print(os.path.basename(os.path.dirname(f)), name, hits)
            p = q
        out.append(p)
    n = ''.join(out)
    if not report and n != s:
        open(f, 'w', encoding='utf8').write(n)
        print('converted', os.path.basename(os.path.dirname(f)))
