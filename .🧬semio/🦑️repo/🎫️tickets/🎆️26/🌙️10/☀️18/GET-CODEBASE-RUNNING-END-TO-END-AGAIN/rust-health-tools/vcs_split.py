import re,json,sys
S='🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/'
p=S+'🕰️history/🎮️command/🦀️.rs'
s=open(p).read()
start=s.index("impl<P,M> ArtifactStore<P,M>where")
head_end=s.index('{',start)
header=s[start:head_end]
bounds=header[header.index('where')+5:].strip()
body=s[head_end+1:]
def scan(text):
    i=0;depth=0;toks=[]
    n=len(text)
    while i<n:
        c=text[i]
        if text.startswith('//',i):
            j=text.find('\n',i); i=n if j<0 else j; continue
        if c=='"':
            i+=1
            while i<n and text[i]!='"': i+= 2 if text[i]=='\\' else 1
            i+=1; continue
        if c=="'":
            mm=re.match(r"'(\\.|[^\\'])'",text[i:i+5])
            if mm: i+=len(mm.group(0)); continue
            i+=1; continue
        if c=='{':
            if depth==0: toks.append(('open',i))
            depth+=1
        elif c=='}':
            if depth==0: toks.append(('end',i)); break
            depth-=1
            if depth==0: toks.append(('close',i))
        i+=1
    return toks
toks=scan(body)
end=[t for t in toks if t[0]=='end'][0][1]
opens=[t[1] for t in toks if t[0]=='open'];closes=[t[1] for t in toks if t[0]=='close']
assert len(opens)==len(closes),(len(opens),len(closes))
prev=0;methods=[]
for o,c in zip(opens,closes):
    methods.append((body[prev:o],body[o+1:c]));prev=c+1
tail=body[prev:end]
rest=body[end+1:]
print(len(methods),repr(tail[:60]),repr(rest[:60]))
json.dump({'pre':s[:start],'bounds':bounds,'methods':methods,'tail':tail,'rest':rest},open(sys.argv[1],'w'))
for lead,b in methods: print('SIG:',lead.strip().split('\n')[-1][:140])
