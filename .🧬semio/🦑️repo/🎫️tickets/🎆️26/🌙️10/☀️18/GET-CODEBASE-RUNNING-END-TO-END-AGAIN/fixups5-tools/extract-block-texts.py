import json,subprocess,textwrap
blk=json.load(open('blocks.json'))
root='/Users/ueli/Documents/semio/'
def git(*a):
    return subprocess.run(['git','-C',root]+list(a),capture_output=True,text=True).stdout
out=[]
for i,r in enumerate(blk):
    tl=[x.strip() for x in r['t'].split('\n')]
    k0=0
    while tl[k0] in ('/**','*'): k0+=1
    key=tl[k0].lstrip('/*').strip()[:50]
    off=k0
    scope=r['f'].split('🏛️ShellHost/')[0] if '🏛️ShellHost/' in r['f'] else None
    ELEM='🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements'
    hits=git('grep','-n','-F',key,'fd14da55f18^','--',r['f']).split('\n')
    hits=[h for h in hits if h]
    if not hits: hits=git('grep','-n','-F',key,'fd14da55f18^','--',ELEM).split('\n')
    hits=[h for h in hits if h]
    res=None
    for h in hits:
        rev,rest=h.split(':',1)[0],h.split(':',1)
        # format rev:path:line:text
        s=h[len('fd14da55f18^:'):]
        path,ln,txt=s.split(':',2)
        lines=git('show','fd14da55f18^:'+path).split('\n')
        k=int(ln)-1-off
        j=k
        while '*/' not in lines[j]: j+=1
        res={'path':path,'text':'\n'.join(lines[k:j+1]),'after':lines[j+1:j+4],'before':lines[max(0,k-2):k]}
        # prefer same-file hit
        if path==r['f']: break
    out.append({'i':i,'nhits':len(hits),'res':res})
    print(i,len(hits),res and res['path'][-70:])
json.dump(out,open('full.json','w'),ensure_ascii=False,indent=1)
