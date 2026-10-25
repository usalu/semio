import re,os,sys,collections
errs=open('.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/🗑️generated/store-b/census-store-errors.txt').read().split('\n')
byfile=collections.defaultdict(set)
for e in errs:
    m=re.match(r'^(.*?):(\d+):(\d+): error\[E0061\]: this method takes 2 arguments but 1 argument was supplied',e)
    if m:byfile[os.path.normpath(m.group(1))].add((int(m.group(2)),int(m.group(3))))
HELPER='\nfn with_receiving<R>(operation: impl FnOnce(&mut semio_framework_value::NativeDecodeControl<\'_>) -> R) -> R { semio_framework_os_store_testing_prelude::FixtureStoreReceiving::new(semio_framework_value::testing::physical_test_close_grant()).receive(operation) }\n'
for f,sites in sorted(byfile.items()):
    if not os.path.exists(f):print('NOFILE',f);continue
    lines=open(f).read().split('\n');done=0;skipped=[]
    for l,c in sorted(sites,reverse=True):
        s=lines[l-1];i=c-1
        if not s[i:].startswith('advance('):skipped.append((l,c,s[i:i+30]));continue
        j=i+len('advance(');depth=1;k=j
        while depth:
            ch=s[k]
            if ch=='(':depth+=1
            elif ch==')':depth-=1
            k+=1
        args=s[j:k-1]
        # receiver
        r=i-1
        assert s[r]=='.'
        b=r
        while b>0 and (s[b-1].isalnum() or s[b-1] in '_.&'):b-=1
        recv=s[b:r]
        s=s[:b]+f'with_receiving(|receiving| {recv}.advance({args}, receiving))'+s[k:]
        lines[l-1]=s;done+=1
    t='\n'.join(lines)
    if done and 'fn with_receiving' not in t:t+=HELPER
    open(f,'w').write(t);print(f[-110:],done,'skipped',skipped)
