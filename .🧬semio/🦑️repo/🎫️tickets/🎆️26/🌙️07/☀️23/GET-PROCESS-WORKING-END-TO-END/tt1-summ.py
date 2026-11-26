import re,sys
t=open(sys.argv[1],encoding='utf-8').read()
for m in re.finditer(r"---- (\S+) stdout ----\n(.*?)(?=\n---- |\nfailures:)",t,re.S):
    name=m.group(1).split('::')[-1]
    lines=[l for l in m.group(2).split('\n') if l.strip() and 'note: run with' not in l]
    loc=re.search(r"panicked at (\S+)",lines[0]) if lines else None
    loc=loc.group(1).split('/')[-1] if loc else ''
    print(name[:70],'|',loc,'|',' '.join(lines[1:3])[:int(sys.argv[2]) if len(sys.argv)>2 else 300])
