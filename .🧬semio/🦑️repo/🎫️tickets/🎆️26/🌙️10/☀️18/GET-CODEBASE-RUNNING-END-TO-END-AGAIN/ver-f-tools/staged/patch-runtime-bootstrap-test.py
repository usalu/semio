"""usage: patch-runtime-bootstrap-test.py <runtime-bootstrap test file> — the bootstrap script calls the launcher (`repo run repo-mcp:install`, the EnsureLauncher region provides `repo` before the native command exists) since Oct 22; the recording model still expected `bun repo run repo-mcp:install`. The `repo` stub now answers the installer call itself (and makes the native command available), the `bun` stub refuses everything."""
import sys
p=sys.argv[1];s=open(p).read()
a='"bun() { [ \\"$*\\" = \\"repo run repo-mcp:install\\" ] || return 92; printf \'installer\\\\n\' >&3; [ \\"$FAIL_AT\\" != installer ] || return 9; native_available=1; }",'
b='"bun() { return 92; }",'
assert s.count(a)==1,"bun stub"
s=s.replace(a,b)
c='"repo() { [ \\"$native_available\\" = 1 ] || return 91; case'
d='"repo() { if [ \\"$*\\" = \\"run repo-mcp:install\\" ]; then printf \'installer\\\\n\' >&3; [ \\"$FAIL_AT\\" != installer ] || return 9; native_available=1; return 0; fi; [ \\"$native_available\\" = 1 ] || return 91; case'
assert s.count(c)==1,"repo stub"
s=s.replace(c,d)
open(p,'w').write(s)
