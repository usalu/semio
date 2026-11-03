import json,re,sys
S="/private/tmp/claude-501/-Users-ueli-Documents-semio/2c8171ff-0be5-4b23-8c30-2258199d4040/scratchpad"
o=[x for x in json.load(open(S+"/cur.json")) if re.search(r'unwrap|expect|panic!|unreachable',x[2]) and 'tickets' not in x[0]]
R="/Users/ueli/Documents/semio/"
w=int(sys.argv[2]) if len(sys.argv)>2 else 300
for a in sys.argv[1].split(','):
    f,n,t=o[int(a)];L=open(R+f,encoding='utf8').read().split('\n')
    s=L[n-1]
    print(f"#### {a} {f[-60:]}:{n}  (len {len(s)})")
    for m in re.finditer(r'\.unwrap\(\)|\.expect\(|panic!\(|unreachable!',s):
        print("   >>",s[max(0,m.start()-w):m.end()+w//2]);print()
