import json,os,re,sys,importlib.util
T=os.path.dirname(os.path.abspath(__file__))
def mod(n,f):
    s=importlib.util.spec_from_file_location(n,T+'/'+f);m=importlib.util.module_from_spec(s);s.loader.exec_module(m);return m
J=mod('j','rootsplit-jsontext.py')
APPLY='apply' in sys.argv
S='/tmp/claude-1000/-home-deck-semio/0ec9c8a1-06e8-45c6-b0fa-6d0c7cb86d6b/scratchpad'
names=json.load(open(S+'/names.json'))
LIB="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
FWPROC="🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript"
SCHEMA=LIB+"/🏗️build/🧬️schema/📋️project/🔣️.json"
root=json.load(open('📋️project.json')); rt=root['targets']
def rx(pat): return [t for t in rt if re.match(pat,t)]
G=[
 dict(project="@semio-tech/stdio-js",dir="✏️s/🧑‍💻dev/🗄️stdio/📦️packages/🟦️typescript",new=False,targets=rx(r'stdio-'),reg=[("stdio",LIB+"/🎛️commands/🗄️stdio/🟦️.ts","StdioScript")]),
 dict(project="@semio-tech/schema-tooling",dir="🧰️framework/🔨️modules/🧬️schema/🛠️tooling",new=True,targets=rx(r'schema-'),reg=[("schema",LIB+"/🎛️commands/🧬️schema/🟦️.ts","SchemaScript")]),
 dict(project="@semio-tech/ui-storybook",dir="🧰️framework/🔨️modules/🖱️ui/📖️storybook",new=True,targets=rx(r'(dev-storybook|prepare-storybook|build-storybook|test-storybook|setup-storybook)'),reg=[("dev",LIB+"/🎛️commands/🛠️dev/🟦️.ts","DevScript"),("test",LIB+"/📦️packages/🟦️typescript/🎛️commands/🧪️test/🚦️root/🟦️.ts","RootTestScript"),("setup",LIB+"/🎛️commands/🔧️setup/🟦️.ts","SetupScript")]),
 dict(project="repo-mcp",dir="🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🔌️mcp",new=False,targets=['dev-mcp','dev-mcp-repo'],reg=[("dev",LIB+"/🎛️commands/🛠️dev/🟦️.ts","DevScript")]),
 dict(project="@semio-tech/repo-lib",dir=LIB+"/📦️packages/🟦️typescript",new=False,targets=['cpp','cpp-setup','cpp-configure','cpp-build','cpp-test','cpp-custody','cpp-test-toolchain','test-workspace-build','test-build-tool-bazel','test-build-tool-bazel-bootstrap','test-runtime-build-producer','test-dependency-readmission','test-graph-memory-reservation','build-canvas-font-assets'],reg=[]),
]
DUPS={'test-discover':'@semio-tech/repo-test-domain','test-doctor':'@semio-tech/repo-test-domain','test-contract':'@semio-tech/repo-test-domain','test-oracle':'@semio-tech/repo-test-domain','test-subject':'@semio-tech/repo-test-domain','test-parity':'@semio-tech/repo-test-domain','test-report':'@semio-tech/repo-test-domain','scale-fixture-check':'@semio-tech/framework-os-dev','runtime-fixture-graph':'os-hub'}
def rel(frm,to):
    r=os.path.relpath(to,frm); return r if r.startswith('.') else './'+r
moved={}
owner={}
for g in G:
    for t in g['targets']: owner[t]=g['project']
for t,p in DUPS.items(): owner[t]=p
def fix_dep(dep,own):
    if isinstance(dep,str):
        if ':' in dep or dep.startswith('^'): return dep
        if dep in owner and owner[dep]!=own: return owner[dep]+':'+dep
        if dep in rt and dep not in owner and own!='workspace': return 'workspace:'+dep
        return dep
    d=dict(dep); tg=d.get('target')
    if tg and ':' not in tg and 'projects' not in d:
        if tg in owner and owner[tg]!=own: d['projects']=[owner[tg]]
        elif tg in rt and tg not in owner and own!='workspace': d['projects']=['workspace']
    return d
plan=[]
newtargets={}
for g in G:
    nt={}
    for t in g['targets']:
        v=json.loads(json.dumps(rt[t]))
        o=v.setdefault('options',{}) if v.get('executor')!='repo:noop' else v.get('options')
        if o is not None and 'command' in o:
            c=o['command']
            if c.startswith('repo ./📜️script.ts '):
                assert g['reg'], (t,c)
                c='repo ./'+g['dir']+'/📜️script.ts '+c[len('repo ./📜️script.ts '):]
            o['command']=c
        if v.get('executor')=='repo:script' or (o is not None and 'command' in o):
            o['cwd']='.'
        if 'dependsOn' in v: v['dependsOn']=[fix_dep(x,g['project']) for x in v['dependsOn']]
        s=json.dumps(v,ensure_ascii=False).replace('{projectRoot}','{workspaceRoot}')
        v=json.loads(s)
        nt[t]=v; plan.append((t,g['project'],t))
    newtargets[g['project']]=nt
# other root targets: fix deps
for t,v in rt.items():
    if t in owner: continue
    if 'dependsOn' in v: v['dependsOn']=[fix_dep(x,'workspace') for x in v['dependsOn']]
for t in list(owner): 
    if t in DUPS: plan.append((t,DUPS[t],t))
if __name__=='__main__':
    print(len(plan)); 
    for g in G: print(g['project'],len(g['targets']))
    json.dump(plan,open(T+'/🗑️generated/rootsplit-b-mapping.json','w'),ensure_ascii=False,indent=1)
    if APPLY:
        for g in G:
            p=g['project']; d=g['dir']
            if g['new']:
                os.makedirs(d,exist_ok=True)
                proj={"name":p,"$schema":rel(d,SCHEMA),"targets":newtargets[p]}
                open(d+'/📋️project.json','w',encoding='utf8').write(json.dumps(proj,indent=2,ensure_ascii=False)+'\n')
            else:
                pf=names[p]; assert not set(newtargets[p])&set(json.load(open(pf))['targets']),p; out=J.add_targets(open(pf,encoding='utf8').read(),newtargets[p]); json.loads(out); open(pf,'w',encoding='utf8').write(out)
            if g['reg']:
                sp=d+'/📜️script.ts'
                imps=[]; regs=[]
                for cmd,mp,cls in g['reg']:
                    imps.append('import { %s } from "%s";'%(cls,rel(d,mp)))
                    regs.append('.register("%s", %s)'%(cmd,cls))
                if g['new']:
                    routing=rel(d,FWPROC+'/🧭️routing/🟦️.ts')
                    code='#!/usr/bin/env repo\n/** 🧭️ %s command router: each command runs against the workspace root. */\nimport { ScriptRouter, findWorkspaceRoot, receiveScriptProcessInvocation, runScriptMain } from "%s";\n%s\n\nconst root = findWorkspaceRoot(import.meta.dirname);\nconst router = new ScriptRouter(root, root)%s;\nawait receiveScriptProcessInvocation(process.env, original => runScriptMain(router, { invocation: original }));\n'%(p,routing,'\n'.join(imps),''.join(regs))
                    open(sp,'w',encoding='utf8').write(code)
                else:
                    st=open(sp,encoding='utf8').read()
                    adapters=''
                    for cmd,mp,cls in g['reg']:
                        adapters+='/** 🧭️ Runs the `%s` command against the workspace root. */\nclass Owned%s extends BundleScript {\n  async run(segments: string[]): Promise<void> { await new %s(this.repoRoot, this.repoRoot, this.invocation).run(segments); }\n}\n'%(cmd,cls,cls)
                    m=re.search(r'new ScriptRouter\(',st); k=m.end(); depth=1
                    while depth:
                        c=st[k]; depth+=(c=='(')-(c==')'); k+=1
                    # find start of statement containing router creation
                    ls=m.start()
                    while ls>0 and st[ls-1]!='\n': ls-=1
                    if st[ls:ls+1] in (' ','\t'):
                        ls=max(mm.start() for mm in re.finditer(r'^(?=\S)',st[:m.start()],re.M))
                    reg=''.join('.register("%s", Owned%s)'%(cmd,cls) for cmd,mp,cls in g['reg'])
                    st=st[:ls]+adapters+st[ls:m.start()]+st[m.start():k]+reg+st[k:] if False else st[:ls]+adapters+st[ls:k]+reg+st[k:]
                    lines=st.split('\n'); last=max(i for i,l in enumerate(lines) if l.startswith('import '))
                    for imp in reversed(imps): lines.insert(last+1,imp)
                    st='\n'.join(lines)
                    if 'BundleScript' not in st.split('\n')[1:12].__str__() and not re.search(r'import[^;]*\bBundleScript\b',st): st=st.replace('import ','import { BundleScript } from "%s";\nimport '%rel(d,FWPROC+'/🧭️routing/🟦️.ts'),1) if not st.startswith('#!') else st.replace('\nimport ','\nimport { BundleScript } from "%s";\nimport '%rel(d,FWPROC+'/🧭️routing/🟦️.ts'),1)
                    open(sp,'w',encoding='utf8').write(st)
        for t in owner: rt.pop(t,None)
        open('📋️project.json','w',encoding='utf8').write(json.dumps(root,indent=2,ensure_ascii=False)+'\n')
