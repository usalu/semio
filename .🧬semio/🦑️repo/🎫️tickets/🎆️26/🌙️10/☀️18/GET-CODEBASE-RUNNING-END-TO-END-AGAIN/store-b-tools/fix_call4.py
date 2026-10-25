import re,os,collections
errs=open('.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/🗑️generated/store-b/census-store-errors.txt').read().split('\n')
by=collections.defaultdict(set)
for e in errs:
    m=re.match(r'^(.*?):(\d+):(\d+): error\[E0061\]: this method takes 4 arguments but 3 arguments were supplied',e)
    if m:by[os.path.normpath(m.group(1))].add((int(m.group(2)),int(m.group(3))))
def split_args(s):
    out=[];d=0;cur='';instr=False;i=0
    while i<len(s):
        c=s[i]
        if c=='"':
            instr=not instr
        if not instr:
            if c in '([{':d+=1
            elif c in ')]}':d-=1
            elif c==',' and d==0:out.append(cur);cur='';i+=1;continue
            elif c=='|' and False:pass
        cur+=c;i+=1
    out.append(cur);return out
for f,sites in sorted(by.items()):
    L=open(f).read().split('\n')
    # files shifted by +? handle by nearby search
    for l,c in sorted(sites,reverse=True):
        i=l-1;s=L[i];idx=c-1
        m=re.match(r'(step_derived_history_preview|step_derived_report_replay|advance_apply_batch)\(',s[idx:])
        if not m:
            print('SKIP',f[-60:],l,s[idx:idx+40]);continue
        name=m.group(1);j=idx+len(m.group(0));depth=1;k=j
        while depth:
            ch=s[k]
            if ch=='(':depth+=1
            elif ch==')':depth-=1
            k+=1
        args=split_args(s[j:k-1])
        # receiver: expression before ".name"
        r=idx-1;assert s[r]=='.';b=r
        while b>0 and (s[b-1].isalnum() or s[b-1] in '_.&'):b-=1
        recv=s[b:r]
        if name=='advance_apply_batch':
            # (publication, grant, receiving) -> (publication, body, grant, receiving)
            new=f'{recv}.advance_apply_batch({args[0].strip()}, semio_framework_value::testing::physical_test_close_grant().maximum_copy_bytes, {args[1].strip()}, {args[2].strip()})'
            s=s[:b]+new+s[k:]
        else:
            new=f'semio_framework_os_store_testing_prelude::FixtureStoreReceiving::new(semio_framework_value::testing::physical_test_close_grant()).receive(|control| {recv}.{name}({args[0].strip()}, {args[1].strip()}, control, {args[2].strip()}))'
            s=s[:b]+new+s[k:]
        L[i]=s
    open(f,'w').write('\n'.join(L));print(f[-100:],len(sites))
