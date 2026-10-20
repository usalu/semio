import json,re,sys,collections
root='/Users/ueli/Documents/semio/'
blk=json.load(open('blocks.json')); full=json.load(open('full.json'))
DROP={6,11,14,17,31}
ANCHOR={0:'function WorldCanvas',1:'pollHeartbeatSab',2:'function useEditorSessionPresence',3:'function useWorldMarqueeAndGumball',
4:'function useOperationCompletionSubscriptions',5:'function useOperationCompletionSubscriptions',
7:'function useOperationProgressSubscriptions',8:'function useOperationProgressSubscriptions',
9:'function useFocusedProgramAndIntroductionState',10:'function useFocusedProgramAndIntroductionState',12:'function useFocusedProgramAndIntroductionState',
13:'function useIntroductionRevealEffects',15:'function usePanelPathsAndOverrideReveals',16:'function useToolTabSelectionEffects',
18:'function useArtifactOpening',19:'function useContributionsAndRefresh',20:'function useHubPortInferenceAndDocumentBackbone',
21:'function resolveDispatchTarget',22:'function useCanvasAndFooter',23:'function useCanvasAndFooter',24:'function useCanvasAndFooter',25:'function useCanvasAndFooter',
26:'function useShellUriAndDocuments',27:'function usePluginLifecycleState',28:'function useDirectoryAndRoute',
29:'function useSpawnedSession',30:'function useSpawnedSession',32:'function useHubCommandCallbacks',33:'function useHubCommandCallbacks',34:'function buildOsCommands'}
NEWDOC={
'useArtifactOpening':('Opens artifacts with an app and keeps the session aware of the app router and its conflicts.',['🚪','📂','📬','🗃']),
'useContributionsAndRefresh':('Publishes contributions, runs the coalesced ui refresh pass and keeps window titles in the active locale.',['🔄','♻','🔁','🪄']),
'useCanvasAndFooter':('Builds the canvas and footer items and exposes the readiness beacon, the dev catalog probes and the shell context menu.',['🧰','🧭','🏞','🖥']),
'useShellUriAndDocuments':('Applies shell uris, opens and closes documents and rehydrates the local catalog documents into the landing app.',['🔗','📑','🗂','📚']),
'useDirectoryAndRoute':('Routes directory events and space administration through the backbone worker.',['🛣','🧭','🗺','📍']),
'useSpawnedSession':('Seeds the canvas of the focused program, grabs focus for newly opened programs and publishes the service contributions of spawned sessions.',['🌱','🪴','🧬','🎯']),
}
byfile=collections.defaultdict(list)
for i,r in enumerate(blk):
    if i in DROP: continue
    byfile[r['f']].append(i)
def paragraph(text):
    ls=text.split('\n')
    ls[0]=re.sub(r'^\s*/\*\*\s?','',ls[0])
    ls[-1]=re.sub(r'\s*\*/\s*$','',ls[-1])
    out=[]
    for l in ls:
        l=l.strip()
        if l.startswith('*'): l=l[1:]
        if l.startswith(' '): l=l[1:]
        out.append(l.rstrip())
    while out and out[0]=='': out.pop(0)
    while out and out[-1]=='': out.pop()
    return out
def emoji_in(txt,e): return e in txt
dry=len(sys.argv)>1 and sys.argv[1]=='dry'
for f,idxs in byfile.items():
    path=root+f
    lines=open(path).read().split('\n')
    groups=collections.OrderedDict()
    for i in sorted(idxs,key=lambda x:blk[x]['l']):
        groups.setdefault(ANCHOR[i],[]).append(i)
    plan=[]
    for a,ii in groups.items():
        cand=[k for k,l in enumerate(lines) if re.search(r'(^|\s)'+re.escape(a)+r'\b',l) and (l.startswith('export') or l.startswith('  '+a) or l.startswith(a) )]
        cand=[k for k in cand if not lines[k].lstrip().startswith(('//','*','/*'))]
        if len(cand)!=1: print('ANCHOR PROBLEM',f[-60:],a,cand); sys.exit(1)
        plan.append((cand[0],a,ii))
    for k,a,ii in sorted(plan,reverse=True):
        indent=re.match(r'\s*',lines[k]).group(0)
        j=k-1
        paras=[paragraph(full[i]['res']['text']) for i in ii]
        if j>=0 and lines[j].rstrip().endswith('*/'):
            e=j
            while '/**' not in lines[j]: j-=1
            old=lines[j:e+1]
            if len(old)==1:
                body=[re.sub(r'\s*\*/\s*$','',re.sub(r'^\s*/\*\*\s?','',old[0]))]
            else:
                body=[]
                for n,l in enumerate(old):
                    t=l.strip()
                    if n==0: t=re.sub(r'^/\*\*\s?','',t)
                    if n==len(old)-1: t=re.sub(r'\s*\*/$','',t)
                    if t.startswith('*'): t=t[1:]
                    if t.startswith(' '): t=t[1:]
                    body.append(t.rstrip())
                while body and body[0]=='': body.pop(0)
                while body and body[-1]=='': body.pop()
            start,end=j,e+1
        else:
            name=a.split()[-1]
            if name not in NEWDOC and not a.startswith('pollHeartbeat'):
                print('NO DOC AND NO SUMMARY',a); sys.exit(1)
            if a.startswith('pollHeartbeat'):
                body=[]
            else:
                summ,cands=NEWDOC[name]
                ftxt='\n'.join(lines)
                emo=next((c for c in cands if c not in ftxt),None)
                if emo is None: print('NO EMOJI',a); sys.exit(1)
                body=[f'{emo}️ {summ}']
            start,end=k,k
        new=body[:]
        for p in paras:
            if new: new.append('')
            new+=p
        doc=[indent+'/**']+[(indent+' * '+t).rstrip() if t else indent+' *' for t in new]+[indent+' */']
        lines[start:end]=doc
    if dry: print('would edit',f[-70:],len(plan))
    else:
        open(path,'w').write('\n'.join(lines)); print('edited',f[-70:],len(plan))
