import importlib.util,json,os,difflib,sys
spec=importlib.util.spec_from_file_location('stage','tsc-stage-tools/stage.py'); st=importlib.util.module_from_spec(spec); spec.loader.exec_module(st)
def hunks(base_text, edit_text):
    base_lines=base_text.splitlines(keepends=True); edit_lines=edit_text.splitlines(keepends=True)
    offsets=[0]
    for line in base_lines: offsets.append(offsets[-1]+len(line))
    out=[]
    for tag,i1,i2,j1,j2 in difflib.SequenceMatcher(None,base_lines,edit_lines,autojunk=False).get_opcodes():
        if tag=="equal": continue
        start,stop=offsets[i1],offsets[i2]
        old,new=base_text[start:stop],"".join(edit_lines[j1:j2])
        p=0; lim=min(len(old),len(new))
        while p<lim and old[p]==new[p]: p+=1
        s=0
        while s<lim-p and old[len(old)-1-s]==new[len(new)-1-s]: s+=1
        out.append((start+p,stop-s,new[p:len(new)-s]))
    return out
def build(base,edit,pad):
    ops=[]
    for start,stop,new in hunks(base,edit):
        left=right=pad
        while True:
            a=base[max(0,start-left):min(len(base),stop+right)]
            if a and base.count(a)==1: break
            left+=12; right+=12
        ops.append([a, base[max(0,start-left):start]+new+base[stop:min(len(base),stop+right)]])
    return ops
def robust(base,edit):
    for pad in (0,24,48,96,192,400):
        ops=build(base,edit,pad)
        try:
            if st.apply_ops(base,ops,'x')==edit and all(base.count(o[0])==1 for o in ops):
                # each anchor must also be unique in the fully edited text's predecessor states: apply_ops proved sequential success
                return ops
        except SystemExit: pass
    raise SystemExit('no robust ops')
if __name__=='__main__':
    index=json.load(open('tsc-stage-tools/staged/index.json'))
    patches=st.load()
    for flat in sys.argv[1:]:
        rel=index[flat]
        keys=[k for k in patches if k==rel or (os.path.basename(k)==st.SCRIPT and os.path.join(os.path.dirname(k),st.OWNER)==rel)]
        assert len(keys)==1,(flat,keys)
        k=keys[0]
        real=st.read(os.path.join(st.ROOT,rel)).decode('utf-8')
        assert st.sha(real.encode('utf-8'))==patches[k]['baseSha256'],('drift',flat)
        edited=open('tsc-stage-tools/staged/'+flat,encoding='utf-8').read()
        ops=robust(real,edited)
        patches[k]={'baseSha256':patches[k]['baseSha256'],'ops':ops}
        print(flat,len(ops),'ops')
    json.dump(patches,open('tsc-stage-tools/patches.json','w',encoding='utf-8'),ensure_ascii=False,indent=1)
