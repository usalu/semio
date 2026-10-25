import re,os,collections
errs=open('.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/🗑️generated/store-b/census-store-errors.txt').read().split('\n')
by=collections.defaultdict(list)
for e in errs:
    m=re.match(r'^(.*?):(\d+):(\d+): error\[E0425\]: cannot find value `receiving_fixture`',e)
    if m:by[os.path.normpath(m.group(1))].append(int(m.group(2)))
DECL='let mut receiving_fixture=FixtureStoreReceiving::new(physical_test_close_grant());'
for f,ls in by.items():
    L=open(f).read().split('\n');ins=set()
    for l in sorted(set(ls)):
        i=l-1
        while i>=0 and not re.match(r'^\s*(pub(\([a-z]+\))? )?(async )?fn \w',L[i]):i-=1
        assert i>=0,(f,l)
        # find line with opening brace at depth end of signature
        j=i
        while '{' not in L[j]:j+=1
        body='\n'.join(L[j:l])
        if re.search(r'let (mut )?receiving_fixture',body):continue
        ins.add(j)
    for j in sorted(ins,reverse=True):
        indent=re.match(r'\s*',L[j]).group(0)+'    '
        # if the opening line has code after '{' (one-liner fn) insert inline
        k=L[j].rindex('{')
        if L[j].rstrip().endswith('{'):L.insert(j+1,indent+DECL)
        else:L[j]=L[j][:k+1]+DECL+L[j][k+1:]
    open(f,'w').write('\n'.join(L));print(f[-100:],len(ins),'fns')
