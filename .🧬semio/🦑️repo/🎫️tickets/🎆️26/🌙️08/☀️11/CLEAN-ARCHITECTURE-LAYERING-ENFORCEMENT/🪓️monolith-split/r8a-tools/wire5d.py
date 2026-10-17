import sys,re
A,S=sys.argv[1:3]
hub=S+'/✏️editor/📦️packages/🦀️rust/Cargo.toml'
t=open(hub,encoding='utf8').read()
if '[features]' not in t:
    feat='[features]\ntestkit = ["semio-framework-plugin/artifact-app-testing", "semio-framework-os-kernel/protocol-laws", "semio-framework-os-kernel/mutation-testing", "semio-s-plugin-puzzle-jobs/testing"]\n\n'
    t=t.replace('[dependencies]\n',feat+'[dependencies]\n',1)
    open(hub,'w',encoding='utf8').write(t)
for p in (A+'/🏅️standards/🔖️1/📦️packages/🦀️rust/Cargo.toml', S+'/✏️editor/🧪️tests/🔬️host-admission/📦️packages/🦀️rust/Cargo.toml'):
    t=open(p,encoding='utf8').read()
    if 'features = ["testkit"]' in t and 'puzzle-5d-puzzle5d' in t.split('[dev-dependencies]')[-1]: continue
    m=re.search(r'^semio-s-artifact-puzzle-5d-puzzle5d = \{ path = "([^"]+)" \}\n',t,re.M)
    line='semio-s-artifact-puzzle-5d-puzzle5d = { path = "%s", features = ["testkit"] }\n'%m.group(1)
    t=t.replace('[dev-dependencies]\n','[dev-dependencies]\n'+line,1)
    open(p,'w',encoding='utf8').write(t); print('wired',p[-60:])
