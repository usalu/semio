import json,os,re,sys
T=os.path.dirname(os.path.abspath(__file__))
APPLY='apply' in sys.argv
LIB="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
DIR=LIB+"/⚖️laws"
P="repo-policy"
W="{workspaceRoot}/"
root=json.load(open('📋️project.json')); rt=root['targets']
sel=[t for t in rt if t.startswith('verify') or t in('mutation-outcome-law','semantic-vocabulary','test-abstraction-ownership','test-artifact-contract-ownership') or t.startswith('artifact-field-parity-')]
owner={t:P for t in sel}
named={
 "policyEngine":[W+LIB+"/⚖️laws/**/*",W+LIB+"/🚨️policy/**/*",W+LIB+"/🎛️commands/✅️verify/**/*",W+LIB+"/🔣️taxonomy.json","sharedGlobals"],
 "tsSources":[W+"**/*.{ts,tsx}",W+"**/package.json",W+"tsconfig.json",W+"bun.lock"],
 "rsSources":[W+"**/*.rs",W+"**/Cargo.toml",W+"Cargo.lock",W+"rust-toolchain.toml",W+"rustfmt.toml"],
 "projectFiles":[W+"**/📋️project.json",W+"📋️build-owners.json",W+"**/package.json"],
 "dependencyFiles":[W+"🔒️dependencies.json",W+"**/package.json",W+"**/Cargo.toml",W+"bun.lock",W+"Cargo.lock",W+"go.work",W+"go.work.sum",W+"**/go.mod",W+"pyproject.toml",W+"uv.lock"],
 "schemaFiles":[W+"**/🧬️schema/**/*",W+"**/🏅️standards/**/*.{ts,rs,json}",W+"**/Cargo.toml"],
}
def inputs(t):
    c=rt[t]['options'].get('command','')
    m=re.match(r'repo \./📜️script\.ts verify ?(\S*)',c); lane=m.group(1) if m else ''
    if t=='verify' or lane in('','gate','build-parity'): n=['policyEngine']
    elif lane=='taxonomy': n=['policyEngine','tsSources','rsSources']
    elif lane in('package-purity','typescript','canonical-architecture','layering','interactivity'): n=['policyEngine','tsSources']
    elif lane=='rust-warnings': n=['policyEngine','rsSources']
    elif lane=='dependencies': n=['policyEngine','dependencyFiles']
    elif lane in('build-owners','zero-touch'): n=['policyEngine','projectFiles']
    elif lane in('generated-corruption','semantic-vocabulary'): n=['policyEngine','tsSources','rsSources']
    else: n=['policyEngine','schemaFiles']
    if t=='semantic-vocabulary': n=['policyEngine','tsSources','rsSources']
    return n
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
new={}
for t in sel:
    v=json.loads(json.dumps(rt[t])); o=v['options']
    o['command']=o['command'].replace('repo ./📜️script.ts ','repo ./'+DIR+'/📜️script.ts ',1); o['cwd']='.'
    if 'dependsOn' in v: v['dependsOn']=[fix_dep(x,P) for x in v['dependsOn']]
    v['inputs']=inputs(t)+(v.get('inputs') or [] if False else [])
    new[t]=v
rootfiles=[W+"📋️project.json",W+"📜️script.ts",W+"package.json",W+"nx.json","sharedGlobals"]
rnamed={"rootFiles":rootfiles,"rootCommands":rootfiles+[W+LIB+"/🎛️commands/**/*"]}
spec={'generate-config-check':['rootCommands',W+"**/📋️project.json"],'generate-taxonomy-census':['rootCommands',W+"**/📋️project.json",W+LIB+"/🔣️taxonomy.json"],'generate-taxonomy-duplicates':['rootCommands',W+"**/📋️project.json",W+LIB+"/🔣️taxonomy.json"],
 'test-metrics':['rootCommands',W+"**/🧪️tests/**/*.ts"],'test-metrics-enforce':['rootCommands',W+"**/🧪️tests/**/*.ts"],
 'test-canvas-native':['rootCommands',W+"🧰️framework/🔨️modules/🖼️canvas/**/*"],'test-canvas-react':['rootCommands',W+"🧰️framework/🔨️modules/🖼️canvas/**/*"],
 'test-command-entrypoint':['rootCommands'],'test-outcome-law-gate':['rootCommands',W+"**/🧬️schema/**/*"],'examples':['rootCommands',W+"**/📚️examples/**/*"]}
def root_inputs(t,v):
    if v.get('executor')=='repo:noop': return None
    if t in spec: return spec[t]
    if v.get('inputs') is not None: return None
    return ['rootFiles']
mapping=[[t,P,t] for t in sel]
if __name__=='__main__':
    print(len(sel),sel[:3])
    if APPLY:
        os.makedirs(DIR,exist_ok=True)
        proj={"name":P,"$schema":os.path.relpath(LIB+"/🏗️build/🧬️schema/📋️project/🔣️.json",DIR),"namedInputs":named,"targets":new}
        open(DIR+'/📋️project.json','w',encoding='utf8').write(json.dumps(proj,indent=2,ensure_ascii=False)+'\n')
        rel=lambda p: os.path.relpath(p,DIR)
        open(DIR+'/📜️script.ts','w',encoding='utf8').write('#!/usr/bin/env repo\n/** ⚖️ repo-policy command router: the workspace-wide verification gates run against the workspace root. */\nimport { ScriptRouter, findWorkspaceRoot, receiveScriptProcessInvocation, runScriptMain } from "%s";\nimport { RootVerifyScript } from "%s";\n\nconst root = findWorkspaceRoot(import.meta.dirname);\nconst router = new ScriptRouter(root, root).register("verify", RootVerifyScript);\nawait receiveScriptProcessInvocation(process.env, original => runScriptMain(router, { invocation: original }));\n'%(rel("🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts"),rel(LIB+"/🎛️commands/✅️verify/🌳️root/🟦️.ts")))
        for t in sel: rt.pop(t)
        for t,v in rt.items():
            if 'dependsOn' in v: v['dependsOn']=[fix_dep(x,'workspace') for x in v['dependsOn']]
            ri=root_inputs(t,v)
            if ri is not None: v['inputs']=ri
        root['namedInputs']={"default":["{workspaceRoot}/📋️project.json","{workspaceRoot}/📜️script.ts","{workspaceRoot}/package.json","{workspaceRoot}/nx.json","sharedGlobals",W+"🧪️tests/🎚️config/🟦️.ts",W+"🧰️framework/🔨️modules/🧬️schema/🧪️tests/🎚️config/🟦️.ts"],**rnamed}
        open('📋️project.json','w',encoding='utf8').write(json.dumps(root,indent=2,ensure_ascii=False)+'\n')
        json.dump(mapping,open(T+'/🗑️generated/rootsplit-c-mapping.json','w'),ensure_ascii=False)
