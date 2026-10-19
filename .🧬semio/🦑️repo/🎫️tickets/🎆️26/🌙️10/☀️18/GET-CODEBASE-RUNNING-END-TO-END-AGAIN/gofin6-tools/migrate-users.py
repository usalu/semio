#!/usr/bin/env python3
"""usage: migrate-users.py <file>... -- rewrites workspace.<harness symbol> to sandbox.<symbol>, adds the sandbox import and drops the workspace import when nothing else uses it"""
import re, sys
MAP = [
    ('RunSandboxed', 'Run'), ('NewRootGuard', 'NewGuard'), ('RootGuard', 'Guard'), ('SandboxProblems', 'Problems'),
    ('UnownedRootLookups', 'UnownedRootLookups'), ('NewFixtureRepo', 'NewRepository'), ('BindRootForTest', 'BindRoot'),
    ('WriteFixtureFile', 'WriteFile'), ('CopyFixtureTree', 'CopyTree'), ('FixtureT', 'T'), ('OriginPath', 'OriginPath'),
    ('SandboxOrigin', 'Origin'), ('SandboxRoot', 'Root'), ('PathWithin', 'PathWithin'),
]
IMP = 'github.com/usalu/semio/repo/workspace'
SB = IMP + '/sandbox'
for path in sys.argv[1:]:
    s = open(path, encoding='utf8').read()
    o = s
    for old, new in MAP:
        s = re.sub(r'\bworkspace\.' + old + r'\b', 'sandbox.' + new, s)
    if s == o:
        continue
    if SB not in s:
        m = re.search(r'^(\s*)(\w+ )?"' + re.escape(IMP) + r'"\n', s, re.M)
        if not m:
            print('NOIMPORT', path)
            continue
        s = s[:m.end()] + m.group(1) + 'sandbox "' + SB + '"\n' + s[m.end():]
    if not re.search(r'\bworkspace\.', s):
        s = re.sub(r'^\s*(\w+ )?"' + re.escape(IMP) + r'"\n', '', s, count=1, flags=re.M)
    open(path, 'w', encoding='utf8').write(s)
    print('migrated', path)
