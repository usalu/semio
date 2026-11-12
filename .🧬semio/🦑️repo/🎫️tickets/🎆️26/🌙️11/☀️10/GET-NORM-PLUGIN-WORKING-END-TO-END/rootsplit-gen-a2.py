import json,os,re,sys,importlib.util,collections
T=os.path.dirname(os.path.abspath(__file__))
def mod(n,f):
    s=importlib.util.spec_from_file_location(n,T+'/'+f);m=importlib.util.module_from_spec(s);s.loader.exec_module(m);return m
LN=mod('ln','rootsplit-lanes.py'); SP=mod('sp','rootsplit-lanes-spec.py'); J=mod('j','rootsplit-jsontext.py')
APPLY='apply' in sys.argv
S='/tmp/claude-1000/-home-deck-semio/0ec9c8a1-06e8-45c6-b0fa-6d0c7cb86d6b/scratchpad'
names=json.load(open(S+'/names.json'))
VERIFYDIR=os.path.dirname(LN.V)
FWPROC=SP.FW+"/🔨️modules/🏃️process/📦️packages/🟦️typescript"
def rel(frm_dir,to):
    r=os.path.relpath(to,frm_dir)
    return r if r.startswith('.') else './'+r
def abs_of(old_rel): return os.path.normpath(os.path.join(VERIFYDIR,old_rel))
lanes=LN.lanes()
root=json.load(open('📋️project.json'))
rt=root['targets']
rootlane={}
for t,v in rt.items():
    m=re.match(r'repo \./📜️script\.ts verify (\S+)(.*)',(v.get('options') or {}).get('command',''))
    if m: rootlane.setdefault(m.group(1),[]).append((t,m.group(2).split(),v))
mapping=[]; removals=[]; newfiles={}; edits={}
def block_for(lane,existing,gid):
    src=lanes[lane][0][2] if lane in lanes else None
    return src
def transform(blk,moddir,existing):
    blk=blk.replace('this.root','context.repoRoot' if existing else 'this.repoRoot')
    blk=re.sub(r'^\s*const \{ (runCargo|runCmd) \} = await import\("\.\./\.\./🏃️process/(🦀️cargo/🏃️run/)?🟦️\.ts"\);\n','',blk,flags=re.M)
    def f(m):
        old=m.group(1)
        if not old.startswith('.'): return m.group(0)
        return 'import("'+rel(moddir,abs_of(old))+'")'
    blk=re.sub(r'import\("([^"]+)"\)',f,blk)
    return blk
def helper_imports(body,moddir,texts):
    L=[]
    L.append('import { BundleScript } from "%s";'%rel(moddir,FWPROC+'/🧭️routing/🟦️.ts'))
    ex=[x for x in('runCmd','runCargo') if re.search(r'\b'+x+r'\b',body)]
    if ex: L.append('import { %s } from "%s";'%(', '.join(ex),rel(moddir,FWPROC+'/🎛️execution/🟦️.ts')))
    st=[x for x in('startToolV1','toolExitV1') if re.search(r'\b'+x+r'\b',body)]
    if st: L.append('import { %s } from "%s";'%(', '.join(st),rel(moddir,FWPROC+'/🧰️tools/🟦️.ts')))
    if re.search(r'\bruntimeExecutableV1\b',body): L.append('import { runtimeExecutableV1 } from "%s";'%rel(moddir,SP.FW+'/🔨️modules/⚙️runtime/🏷️identity/🟦️.ts'))
    if re.search(r'\bjoin\(',body): L.append('import { join } from "node:path";')
    if texts:
        L.append('import { bindCatalogV1 } from "%s";'%rel(moddir,FWPROC+'/🏗️build/🗣️language/📖️catalog/🟦️.ts'))
        L.append('import { repoErrorV1, type RepoErrorCodeV1, type RepoErrorV1 } from "%s";'%rel(moddir,FWPROC+'/🏗️build/🗣️language/🚨️error/🟦️.ts'))
        L.append('import catalog from "./📖️texts/🔣️.json" with { type: "json" };')
    return L
catalog_all=json.load(open('🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/✅️verify/📖️texts/🔣️.json'))['messages']
GQL='''/** 🧾️ Parses each viewport GraphQL schema with the reference parser and refuses the first syntax error. */
async function parseViewportGraphqlSchemasV1(paths: readonly string[]): Promise<void> {
  const { parse } = await import("graphql");
  for (const path of paths) parse(readFileSync(path, "utf8"));
}
'''
for g in SP.GROUPS:
    pf=names[g['project']]; pdir=pf[:-len('/📋️project.json')]
    moddir=g['root']+'/🧪️tests/🔬️owned-verify'
    blocks=[]
    for lane in g['lanes']:
        src=lanes.get(lane)
        if lane in('wires-reset-document-ownership','rewriting-reset-document-ownership'):
            base=lanes['reset-document-ownership'][0][2]
            crate=dict(SP.RENAME['reset-document-ownership'])[lane]
            b=base.replace('"reset-document-ownership"','"%s"'%lane).replace('"-p", "semio-s-artifact-reasoning-wires", "-p", "semio-s-artifact-trinity-rewriting"','"-p", "%s"'%crate)
        else:
            b=src[0][2]
            removals.append((lane,src[0][0],src[0][1]))
        blocks.append((lane,b))
    if any(l=='wires-reset-document-ownership' for l,_ in blocks): removals.append(('reset-document-ownership',lanes['reset-document-ownership'][0][0],lanes['reset-document-ownership'][0][1]))
    body='\n'.join(transform(b,moddir,g.get('existing',False)) for _,b in blocks)
    keys=sorted(set(re.findall(r'refuseVerifyV1\("\w+",\s*"([^"]+)"',body)))
    if g.get('existing'):
        modpath=g['root']+'/🧪️tests/🔬️rust-package-commands/🟦️.ts'
        txt=open(modpath,encoding='utf8').read()
        marker="  throw new Error('Unknown owned verification"
        if marker not in txt: marker="  throw new Error(`Unknown owned verification" if "Unknown owned verification" in txt else None
        assert marker and marker in txt,(modpath)
        i=txt.rindex(marker)
        edits[modpath]=txt[:i]+body+'\n'+txt[i:]
    else:
        texts=bool(keys)
        imps=helper_imports(body,moddir,texts)
        helpers=''
        if texts:
            helpers+='const VERIFY_TEXTS_V1 = bindCatalogV1("verify-%s", catalog);\n\n/** 🚨️ Builds one refusal of the %s verification texts as data; the CLI boundary renders it in the resolved language. */\nconst refuseVerifyV1 = (code: RepoErrorCodeV1, key: keyof typeof catalog.messages): RepoErrorV1 => repoErrorV1(VERIFY_TEXTS_V1, code, key, {});\n\n'%(g['id'],g['id'])
            newfiles[moddir+'/📖️texts/🔣️.json']=json.dumps({"version":1,"id":"verify-"+g['id'],"languages":["en","de"],"messages":{k:catalog_all[k] for k in keys}},indent=2,ensure_ascii=False)+'\n'
        if 'parseViewportGraphqlSchemasV1' in body:
            imps.append('import { readFileSync } from "node:fs";'); helpers+=GQL+'\n'
        code='\n'.join(imps)+'\n\n'+helpers+'/** 🧪️ Runs the verification lanes owned by the %s domain: %s. */\nexport class OwnedVerifyScript extends BundleScript {\n  async run(segments: string[]): Promise<void> {\n%s\n    throw new Error(`Unknown owned verification ${segments.join(" ")}`);\n  }\n}\n'%(g['id'],', '.join(g['lanes']),body)
        newfiles[moddir+'/🟦️.ts']=code
        sp=pdir+'/📜️script.ts'
        st=open(sp,encoding='utf8').read()
        m=re.search(r'new ScriptRouter\(',st)
        if not m:
            st2=st.replace('{ snapshotSqliteTests:','{ commands: new ScriptRouter(import.meta.dirname).register("verify", OwnedVerifyScript), snapshotSqliteTests:'); assert st2!=st,sp
            st2='import { ScriptRouter } from "%s";\n'%rel(pdir,FWPROC+'/🧭️routing/🟦️.ts')+st2
        else:
            k=m.end(); depth=1
            while depth:
                c=st[k]; depth+= (c=='(') - (c==')'); k+=1
            st2=st[:k]+'.register("verify", OwnedVerifyScript)'+st[k:]
        imp='import { OwnedVerifyScript } from "%s";\n'%rel(pdir,moddir+'/🟦️.ts')
        lines=st2.split('\n'); last=max(i for i,l in enumerate(lines) if l.startswith('import '))
        lines.insert(last+1,imp.rstrip('\n'))
        edits[sp]='\n'.join(lines)
    d=json.load(open(pf))
    added={}
    for lane in g['lanes']:
        olds=[x for x in (rootlane.get(lane,[]) if lane not in ('wires-reset-document-ownership','rewriting-reset-document-ownership') else rootlane.get('reset-document-ownership',[]))]
        base='verify-'+lane
        tgt={"executor":"repo:script","cache":True,"outputs":[],"options":{"cwd":pdir,"command":"repo ./📜️script.ts verify "+lane,"forwardAllArgs":True},"metadata":{"semio":{"workspaceCommand":["verify",lane]}}}
        assert base not in d['targets'],base
        added[base]=tgt
        for t,args,v in olds:
            if lane.endswith('reset-document-ownership'): mapping.append((t,g['project'],base)); continue
            if not args: dest=base
            else:
                dest=base+'-'+'-'.join(args)
                if dest not in added:
                    nt=json.loads(json.dumps(tgt)); nt['options']['command']+=' '+' '.join(args); nt['metadata']['semio'].pop('workspaceCommand'); nt['metadata']['semio']or nt.pop('metadata'); 
                    if 'metadata' in nt and not nt['metadata']['semio']: nt.pop('metadata')
                    if 'cache' in v: nt['cache']=v['cache']
                    added[dest]=nt
            mapping.append((t,g['project'],dest))
    g['_added']=(pf,added)
if __name__=='__main__':
    print(len(newfiles),len(edits),len(removals),len(mapping))
    for k in newfiles: print('new',k[-70:])
    for k in edits: print('edit',k[-70:])
    print([r[0] for r in removals])
    json.dump(mapping,open(T+'/🗑️generated/rootsplit-a2-mapping.json','w'),ensure_ascii=False,indent=1)
    if APPLY:
        if '--resume' not in sys.argv:
            for k,v in newfiles.items():
                os.makedirs(os.path.dirname(k),exist_ok=True); open(k,'w',encoding='utf8').write(v)
            for k,v in edits.items(): open(k,'w',encoding='utf8').write(v)
        for g in SP.GROUPS:
            pf,added=g['_added']; out=J.add_targets(open(pf,encoding='utf8').read(),added); json.loads(out); open(pf,'w',encoding='utf8').write(out)
        gone={t for t,_,_ in mapping}
        for t in gone: rt.pop(t,None)
        open('📋️project.json','w',encoding='utf8').write(json.dumps(root,indent=2,ensure_ascii=False)+'\n')
        V=LN.V; src=open(V,encoding='utf8').read().split('\n')
        for lane,a,e in sorted(removals,key=lambda x:-x[1]): del src[a:e]
        open(V,'w',encoding='utf8').write('\n'.join(src))
