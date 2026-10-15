import sys,re,os
sys.path.insert(0,'.')
from wsan import *
from wscommon import *
stage=sys.argv[1]
types={}  # (file)-> name->vis for top-level type-like items
files={}
for c,folder,mod,emoji,doc in CHILDREN:
    p=f"{stage}/{folder}/🦀️.rs"
    t=open(p,encoding='utf-8').read();L=[l+'\n' for l in t.split('\n')][:-1]
    I=lex2(L);its=top_items(L,I)
    for it in its:classify(it)
    files[c]=(L,I,its)
bad=0
for c,(L,I,its) in files.items():
    typenames={it['name']:it['vis'] for it in its if it['kind'] in('struct','enum','type','trait','union') and it['name']}
    private={n for n,v in typenames.items() if v==''}
    for it in its:
        # signature text of widened pub(super) fn/struct field/method
        for x in range(it['head'],it['end']+1):
            line=L[x]
            m=re.match(r'^\s*pub\(super\)\s+((?:unsafe |async |const )*)fn\s+\w+',line)
            if m:
                sig=line
                k=x
                while '{' not in sig and ';' not in sig and k+1<len(L):
                    k+=1;sig+=L[k]
                sig=sig.split('{')[0]
                for n in private:
                    if re.search(r'\b'+n+r'\b',sig):
                        print("PRIVATE-IN-SIG",c,n,sig.strip()[:140]);bad+=1
            m=re.match(r'^\s*pub\(super\)\s+(\w+)\s*:\s*(.*)$',line)
            if m:
                for n in private:
                    if re.search(r'\b'+n+r'\b',m.group(2)):
                        print("PRIVATE-IN-FIELD",c,n,line.strip()[:120]);bad+=1
print("bad",bad)
