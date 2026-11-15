#!/usr/bin/env python3
"""Package O: provisioning stops writing the cache pointer (SEMIO_REMOTE_CACHE_ENDPOINT/ACCESS/INSTANCE). The fleet facts are the source of truth.
Usage: patch-env.py <shared-worktree dir> [--dry]. Anchored: every anchor must occur exactly the stated number of times, or nothing is written."""
import sys, pathlib, re
W = pathlib.Path(sys.argv[1]); DRY = "--dry" in sys.argv
TOP = W / "🗺️topology"
edits = {}

def read(p):
    return edits[p] if p in edits else p.read_text()

def sub(p, old, new, count=1):
    t = read(p)
    n = t.count(old)
    assert n == count, f"{p.name}: anchor occurs {n}x (want {count}): {old[:90]!r}"
    edits[p] = t.replace(old, new)

def cut(p, pattern, flags=0):
    t = read(p)
    new, n = re.subn(pattern, "", t, count=1, flags=flags)
    assert n == 1, f"{p.name}: pattern did not match once: {pattern[:90]!r}"
    edits[p] = new

# topology
T = TOP / "🟦️.ts"
cut(T, r'/\*\* 🌍️ Renders the environment of a machine of one group:[^\n]*\*/\nexport function machineEnvironmentV1\([^\n]*\n[^\n]*\n[^\n]*\n\}\n\n')
C = TOP / "⌨️command/🟦️.ts"
sub(C, "groupV1,machineEnvironmentV1,mountPartsV1", "groupV1,mountPartsV1")
sub(C, 'type Operation="env"|"mount-plan"', 'type Operation="mount-plan"')
sub(C, ' env:{valued:["--group","--shell"],plain:[]},\n', "")
cut(C, r'/\*\* 🌱️ Renders an environment as[^\n]*\*/\nexport function renderEnvironmentV1\([^\n]*\n[^\n]*\n\}\n\n')
sub(C, ' if(operation==="env")return renderEnvironmentV1(machineEnvironmentV1(topology,group),shell);\n', "")
sub(C, "returns the text to print: `env`, `mount-plan`", "returns the text to print: `mount-plan`")

# shell provisioning
S = TOP / "🥾️provision/🐚️.sh"
sub(S, '''ENVIRONMENT=$(fleet env --group "$GROUP_ID" --shell sh | grep '^export ')
ADDRESS_EXPORTS=$(fleet_address_env | sed 's/^/export /')
[ -z "$ADDRESS_EXPORTS" ] || ENVIRONMENT="$ENVIRONMENT
$ADDRESS_EXPORTS"
''', '''ENVIRONMENT=$(fleet_address_env | sed 's/^/export /')
''')
sub(S, 'say "step 5: cache environment"', 'say "step 5: machine environment (the cache endpoint comes from the fleet facts, never from a profile)"')
sub(S, '''# The build tools resolve their remote cache from SEMIO_REMOTE_CACHE_*: the profile files that step 5 writes belong to login shells, and the native bootstrap runs through sudo without one, so it gets the same variables and the write token of the account through a file that only the account can read.
native_environment() {
  local line token="${SEMIO_REMOTE_CACHE_TOKEN:-}" file
  while IFS= read -r line; do
    case $line in "export SEMIO_REMOTE_CACHE_"*=*) line=${line#export }; printf "%s='%s'\\n" "${line%%=*}" "${line#*=}";; esac
  done <<< "${ENVIRONMENT:-}"
''', '''# The build tools find the remote cache in the fleet facts of the group store; the native bootstrap runs through sudo without a login shell, so it gets the operator's write token of the account (when it has one) through a file that only the account can read.
native_environment() {
  local token="${SEMIO_REMOTE_CACHE_TOKEN:-}" file
''')

# PowerShell provisioning
P = TOP / "🥾️provision/🔵️.ps1"
cut(P, r"Say 'step 7: cache environment'\n.*?\n\}\n\n(?=Say 'step 7b)", re.S)

# tests
TT = TOP / "🧪️tests/🟦️.ts"
sub(TT, "groupV1,machineEnvironmentV1,mountPartsV1", "groupV1,mountPartsV1")
sub(TT, "PREFLIGHT_LIMITS_V1,renderEnvironmentV1,renderFactsV1", "PREFLIGHT_LIMITS_V1,renderFactsV1")
sub(TT, 'say "step 5: cache environment"', 'say "step 5: fleet environment', 0) if False else None
t = read(TT)
n = t.count('say "step 5: cache environment')
assert n >= 3, n
edits[TT] = t.replace('say "step 5: cache environment"', 'say "step 5: machine environment (the cache endpoint comes from the fleet facts, never from a profile)"').replace('say "step 5: cache environ', 'say "step 5: machine environ')
sub(TT, '"export SEMIO_REMOTE_CACHE_ENDPOINT=http://203.0.113.10:6090"', '"export SEMIO_FLEET_NFS_HOST=203.0.113.10"')
t = read(TT)
i = t.index('test("the environment of every group is its own cache endpoint, read-write access and instance"')
j = t.index('test("an unknown group, platform or shell is refused"')
edits[TT] = t[:i] + '''test("no provisioning path writes a cache pointer: the env operation is gone and neither script sets SEMIO_REMOTE_CACHE_ENDPOINT, _ACCESS or _INSTANCE; the fleet facts are the source of truth",()=>{
 expect(()=>runTopologyCommandV1(["env","--group","a1"],{platform:"darwin",topology:fixture.topology})).toThrow();
 expect(provisionShell).not.toMatch(/fleet env|SEMIO_REMOTE_CACHE_(ENDPOINT|ACCESS|INSTANCE)/u);
 expect(provisionPowerShell).not.toMatch(/'env'|SEMIO_REMOTE_CACHE_(ENDPOINT|ACCESS|INSTANCE)/u);
});

''' + t[j:]
sub(TT, ' expect(()=>machineEnvironmentV1(topology,"zz")).toThrow("Declared fleet group required");\n', "")
t = read(TT)
i = t.index('test.skipIf(!python)("the Python oracle accepts the committed topology and recomputes the same environments"')
j = t.index("\n});\n", i) + 5
body = t[i:j]
new = '''test.skipIf(!python)("the Python oracle accepts the committed topology and the fixture",()=>{
 const check=Bun.spawnSync([python!,"-I",oracle,"check"],{stdout:"pipe",stderr:"pipe"});
 expect(check.stdout.toString()+check.stderr.toString()).toBe("");
 expect(check.exitCode).toBe(0);
});
'''
edits[TT] = t[:i] + new + t[j:]
sub(TT, 'const environment="export SEMIO_REMOTE_CACHE_ENDPOINT=http://10.0.0.1:6090\\nexport SEMIO_REMOTE_CACHE_ACCESS=read-write\\nexport SEMIO_REMOTE_CACHE_INSTANCE=g1\\nexport UNRELATED=1",run=', 'const environment="export UNRELATED=1",run=')
sub(TT, 'expect(run.log).toContain("[semio] cache http://10.0.0.1:6090 read-write g1 token=abc-write");', 'expect(run.log).toContain("[semio] cache    token=abc-write");')
sub(TT, "the native bootstrap runs with the remote cache environment of the group and the write token of the account, and leaves no environment file behind", "the native bootstrap gets the operator's write token of the account (the endpoint comes from the fleet facts) and leaves no environment file behind")
WT = TOP / "🧪️tests/🔵️.ts"
sub(WT, " `repo topology env --group ${group} --shell powershell`,\n", "")
sub(WT, 'expect({...outcome.machine,Path:undefined}).toEqual({...(fixture.environments as Record<string,Record<string,string>>)["a1"],Path:undefined});', 'expect({...outcome.machine,Path:undefined}).toEqual({Path:undefined});')

sub(TT, 'expect(()=>runTopologyCommandV1(["env","--group","a1","--shell","fish"],{topology:fixture.topology})).toThrow("--shell must be one of");', 'expect(()=>runTopologyCommandV1(["facts","--group","a1","--shell","fish"],{topology:fixture.topology})).toThrow("--shell must be one of");')
sub(TT, 'expect(parseTopologyArgumentsV1(["env","--group","a1","--shell=sh"])).toEqual({operation:"env",values:{"--group":"a1","--shell":"sh"},flags:new Set()});', 'expect(parseTopologyArgumentsV1(["facts","--group","a1","--shell=sh"])).toEqual({operation:"facts",values:{"--group":"a1","--shell":"sh"},flags:new Set()});')
sub(TT, 'parseTopologyArgumentsV1(["env","--group","a1","--group","b2"])', 'parseTopologyArgumentsV1(["facts","--group","a1","--group","b2"])')
sub(TT, 'parseTopologyArgumentsV1(["env","--platform","linux"])).toThrow("Unknown option")', 'parseTopologyArgumentsV1(["server-plan","--platform","linux"])).toThrow("Unknown option")')
sub(TT, 'parseTopologyArgumentsV1(["env","--reachable"])', 'parseTopologyArgumentsV1(["facts","--reachable"])')
sub(TT, 'parseTopologyArgumentsV1(["env","--group"])', 'parseTopologyArgumentsV1(["facts","--group"])')
sub(TT, 'parseTopologyArgumentsV1(["env","--group","--shell"])', 'parseTopologyArgumentsV1(["facts","--group","--shell"])')
sub(TT, 'runTopologyCommandV1(["env"],{topology:fixture.topology})', 'runTopologyCommandV1(["facts"],{topology:fixture.topology})')
sub(TT, '["env","--group",group,"--shell","sh"],', '')
# python oracle
Y = TOP / "🧪️tests/🐍️.py"
sub(Y, 'NAMES={"endpoint":"SEMIO_REMOTE_CACHE_ENDPOINT","access":"SEMIO_REMOTE_CACHE_ACCESS","instance":"SEMIO_REMOTE_CACHE_INSTANCE"}\n', "")
cut(Y, r'def environment\(topology,group_id\):\n(?:    [^\n]*\n)+\n')
t = read(Y)
lines = t.split("\n")
keep = []
dropped = 0
skip_next = 0
for k, line in enumerate(lines):
    if skip_next:
        skip_next -= 1; dropped += 1; continue
    if 'environment(committed,n)!=' in line or 'if fixture["variables"]!=NAMES' in line or 'if sorted(fixture["environments"])' in line:
        dropped += 1; continue
    if line.strip() == 'for group_id,expected in fixture["environments"].items():':
        skip_next = 1; dropped += 1; continue
    if 'sys.argv[1]=="env"' in line:
        skip_next = 1; dropped += 1; continue
    keep.append(line)
assert dropped == 8, dropped
edits[Y] = "\n".join(keep)
sub(Y, "(valid accepted, every invalid case refused, environments recomputed)", "(valid accepted, every invalid case refused)")
t = read(Y)
assert "usage: 🐍️.py check" in t
t = re.sub(r"       🐍️\.py env <topology\.json\|fixture\.json> <group>[^\n]*\n", "", t, count=1)
edits[Y] = t
F = TOP / "🧪️tests/🧫️fixtures/🔣️.json"
cut(F, r' "variables": \{.*?\n \},\n', re.S)
cut(F, r' "environments": \{.*?\n \},\n', re.S)
t = read(F)
assert "REMOTE_CACHE" not in t, "fixture still names the variables"

# documentation
R = W / "README.md"
sub(R, "| `env` | `--group [--shell sh\\|powershell]` | `SEMIO_REMOTE_CACHE_ENDPOINT`, `SEMIO_REMOTE_CACHE_ACCESS=read-write`, `SEMIO_REMOTE_CACHE_INSTANCE`. |\n", "")
sub(R, "hands it the `SEMIO_REMOTE_CACHE_*` variables of `env` and the write token of the account", "hands it the write token of the account")
sub(R, "the cache environment of every machine, the scripts' checks", "the scripts' checks")
t = read(R)
i = t.index("Provisioning writes the machine environment of `env`")
j = t.index("**Zero-touch: the fleet facts.**")
edits[R] = t[:i] + '''Provisioning no longer writes any cache pointer (the former `env` operation is gone): `SEMIO_REMOTE_CACHE_ENDPOINT`, `_ACCESS` and
`_INSTANCE` come from the fleet facts in the group store (`<store>/v1/fleet/cache.json`), which the build tooling reads on every
machine (`📚️library/🏗️build/🌐️remote-cache`). A leftover pointer in `/etc/zshenv`, `/etc/profile`, `/etc/profile.d/semio-fleet.sh` or the
Windows Machine environment, written by an earlier version of the scripts, is ignored whenever healthy facts name another endpoint,
unless `SEMIO_REMOTE_CACHE_ENDPOINT_OVERRIDE` is set; running the script again rewrites the block without it. The provisioning scripts
write only the fleet address variables (`SEMIO_FLEET_GROUP`, `SEMIO_FLEET_NFS_HOST`, `SEMIO_FLEET_NETWORK`). The bearer token, where an
operator still uses one, is read from the variable named by `SEMIO_REMOTE_CACHE_TOKEN_ENVIRONMENT`, default `SEMIO_REMOTE_CACHE_TOKEN`.

''' + t[j:]

for p in edits:
    print("edit", p.relative_to(W))
if not DRY:
    for p, t in edits.items():
        p.write_text(t)
