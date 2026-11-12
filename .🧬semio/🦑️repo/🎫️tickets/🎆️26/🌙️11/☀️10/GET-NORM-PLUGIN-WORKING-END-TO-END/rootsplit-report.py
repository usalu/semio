import json,re,os,collections
T=os.path.dirname(os.path.abspath(__file__))
rows=json.load(open(T+'/🗑️generated/rootsplit-all-mapping.json'))
ren={"@semio-tech/ui-storybook":"repo-storybook","@semio-tech/schema-tooling":"repo-schema-commands"}
rows+=[(t,pp,d,'policy') for t,pp,d in json.load(open(T+'/🗑️generated/rootsplit-c-mapping.json'))]
seen=set(); out=[]
for t,p,d,k in rows:
    if t in seen: continue
    seen.add(t); p=ren.get(p,p)
    if d=='verify-video-render-export': p='@semio-tech/framework-os'
    out.append((t,p,d,k))
root=json.load(open('📋️project.json'))['targets']
verbs=collections.defaultdict(list)
for t,v in root.items():
    m=re.match(r'repo \./📜️script\.ts (\S+)',(v.get('options') or {}).get('command',''))
    verbs[m.group(1) if m else 'own script / aggregate'].append(t)
L=[]
A=L.append
A('# Root project split (rootsplit)\n')
A('Goal: move the per-domain targets of the repo-root `📋️project.json` (project `workspace`, 329 targets, all through one `📜️script.ts`) into the projects of their domains, split the root dispatch, update callers. Scripts and logs of this work: `rootsplit-*.py` and `🗑️generated/rootsplit-*` in this ticket folder.\n')
A('## Result\n')
A('- Root targets: 329 before, %d after. %d targets moved to %d domain projects or removed because the domain project already owned the same lane.'%(len(root),len(out),len({p for _,p,_,_ in out})))
A('- New projects (project-only, own `📜️script.ts`, no new bun package): `repo-storybook` (`🏗️build/📖️storybook`), `repo-schema-commands` (`🎛️commands/🧬️schema`). Existing projects extended: see table.')
A('- Root verify dispatch `🎛️commands/✅️verify/🟦️.ts`: 40 domain lanes (about 470 lines) moved into domain-owned `OwnedVerifyScript` modules `<domain>/🧪️tests/🔬️owned-verify/🟦️.ts` (new in framework os, replication, value, pack, job, kernel, ui, infinite-dag, block-3d; added to the existing owned verify modules of wires, rewriting, jack). The root keeps only workspace-wide gates. `verify <lane>` stays routable through the declarative `metadata.semio.workspaceCommand` of the owning project, so lane CLI names did not change. `reset-document-ownership` was split into `wires-reset-document-ownership` and `rewriting-reset-document-ownership` (one lane per owning crate).')
A('- Texts: the six refusal messages of the moved lanes live in domain catalogs (`verify-os`, `verify-ui`) and were deleted from the repo verify catalog.')
A('- Callers rewritten: `workspace:<target>` to `<project>:<target>` in READMEs, `.storybook`, dashboard authority fixture and test, build-surfaces / dashboard-control fixtures, `🏗️build/🧭️invocation` (cpp and storybook mapping), `🏗️build/📥️inputs/🦀️resolution` fixture, verify gate lane (`schema-parsers-check`), cache-contract test; `.storybook` catalog-integrity test now reads the `repo-storybook` project; `dependsOn` entries across all project files. No CI workflow, `.agents` skill or `.devcontainer` command referenced a moved name (checked with word grep). The message sample in `🕸️graph/🧾️collect-errors` was left as is (synthetic text).\n')
A('## Target to new project\n')
A('Old root names are gone (no aliases). "how": `duplicate` = the domain project already owned the lane under this name; `variant` = same lane with an argument, now its own target next to the lane; `moved` = definition moved with its `cwd` set to the workspace root and the command pointing at the domain script.\n')
A('| old root target | new project | new target |\n|---|---|---|')
for t,p,d,k in sorted(out,key=lambda r:(r[1],r[0])):
    A('| %s | %s | %s |'%(t,p,d))
A('\n## Kept at the root (%d), workspace-wide by nature\n'%len(root))
for v,ts in sorted(verbs.items()):
    A('- `%s` (%d): %s'%(v,len(ts),', '.join(ts) if len(ts)<12 else ', '.join(ts[:12])+', ...'))
res=T+'/🗑️generated/rootsplit-results.md'
if os.path.exists(res): A('\n'+open(res,encoding='utf8').read())
open(T+'/root-project-split.md','w',encoding='utf8').write('\n'.join(L)+'\n')
print(len(out),len(root))
