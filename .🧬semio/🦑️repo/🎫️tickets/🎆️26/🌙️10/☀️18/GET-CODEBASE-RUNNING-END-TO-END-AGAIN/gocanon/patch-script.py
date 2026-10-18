#!/usr/bin/env python3
"""patch-script.py <📜️script.ts>...: route `go test ...` through runCanonicalGoTests."""
import re, sys

LINE = re.compile(r'await runRepositoryTestCommand\("go", \["test", "\.\.\.", (?P<args>.*?)\], \{ cwd: (?P<cwd>[^,}]+?)(?:, env: (?P<env>.+?))? \}\);')
GOPLAN = re.compile(r'import \{ (?P<names>[^}]*) \} from "(?P<path>[^"]*🐹️go-plan/🟦️\.ts)";')
RRTC = re.compile(r'import \{ runRepositoryTestCommand \} from "(?P<path>[^"]*)🎛️repository-command/🟦️\.ts";\n')


def patch(path):
    s = open(path, encoding='utf8').read()
    m = LINE.search(s)
    if not m:
        print('no go test line:', path)
        return False
    env = f', {{ env: {m.group("env")} }}' if m.group('env') else ''
    s = s[:m.start()] + f'await runCanonicalGoTests({m.group("cwd")}, [{m.group("args")}]{env});' + s[m.end():]
    g = GOPLAN.search(s)
    if g:
        names = [n.strip() for n in g.group('names').split(',')]
        if 'runCanonicalGoTests' not in names:
            names.append('runCanonicalGoTests')
        s = s[:g.start()] + f'import {{ {", ".join(names)} }} from "{g.group("path")}";' + s[g.end():]
        if 'runRepositoryTestCommand(' not in s:
            s = RRTC.sub('', s, count=1)
    else:
        r = RRTC.search(s)
        assert r, path
        if 'runRepositoryTestCommand(' in s:
            s = s[:r.end()] + f'import {{ runCanonicalGoTests }} from "{r.group("path")}🐹️go-plan/🟦️.ts";\n' + s[r.end():]
        else:
            s = s[:r.start()] + f'import {{ runCanonicalGoTests }} from "{r.group("path")}🐹️go-plan/🟦️.ts";\n' + s[r.end():]
    open(path, 'w', encoding='utf8').write(s)
    return True


for p in sys.argv[1:]:
    print(p.split('/')[-4:-2], patch(p))
