import json,re,sys
S="/private/tmp/claude-501/-Users-ueli-Documents-semio/2c8171ff-0be5-4b23-8c30-2258199d4040/scratchpad"
o=[x for x in json.load(open(S+"/cur.json")) if re.search(r'unwrap|expect|panic!|unreachable',x[2]) and 'tickets' not in x[0]]
R="/Users/ueli/Documents/semio/"
for a in sys.argv[1].split(','):
    f,n,_=o[int(a)];L=open(R+f,encoding='utf8').read().split('\n')
    sigs=[]
    for i in range(n-1,-1,-1):
        for m in re.finditer(r'fn \w+[^{]*',L[i]) if i<n-1 else re.finditer(r'fn \w+[^{]*',L[i][:0]):pass
        ms=list(re.finditer(r'fn \w+[^{;]*',L[i]))
        if ms:
            print(a,f[-45:],n,'::',ms[-1].group(0)[:260]);break
