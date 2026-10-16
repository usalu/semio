import sys,collections; sys.path.insert(0,'.')
import bxfile, bxinfer, bdec, bmap, dmap
meta=bxfile.load_meta(); specs=bxfile.specs()
R="/Users/ueli/Documents/semio/"
SC="/private/tmp/claude-501/-Users-ueli-Documents-semio/11914954-c49c-43e1-9499-97ab9f20751b/scratchpad/"
files=[f for f in open(SC+'board-users.txt',encoding='utf-8',errors='replace').read().split('\n') if f and '♾️infinite' not in f and '🎫️tickets' not in f]
moved={h:{n for n,m in meta[h][0].items() if m.mode!='owner'} for h in meta}
fl={'BoardHost':set(bmap.F),'DagHost':set(dmap.F)}
comps={'BoardHost':set(bmap.ORDER),'DagHost':set(dmap.ORDER)}
SUS={'host','h','dag','board','catalog_host','empty_host','puzzle'}
def run(files=files):
  for f in files:
    t=open(R+f,encoding='utf-8',errors='replace').read()
    hs=[h for h in ('BoardHost','DagHost') if h in t]
    if not hs: continue
    nt,plan,haz,flags,bad=bxinfer.rewrite_file(t,f,meta,specs)
    toks=bdec.ctoks(nt); left=collections.Counter()
    for i,x in enumerate(toks):
        if x.k!='id' or toks[i-1].s!='.': continue
        for h in hs:
            if (x.s in moved[h] and toks[i+1].s=='(') or (x.s in fl[h] and toks[i+1].s!='('):
                ch=bxinfer.chain_before(toks,i)
                if not ch: continue
                last=toks[ch[1]].s
                if last in SUS and last not in comps[h]:
                    left[(nt[toks[ch[0]].a:toks[ch[1]].b], x.s, h)]+=1
    if left: print(f.replace('✏️s/🔌️plugins/','P/').replace('🧰️framework/🛍️products/💻️os/🔨️modules/','OS/')[-72:], dict(left.most_common(8)))
if __name__=='__main__': run()
