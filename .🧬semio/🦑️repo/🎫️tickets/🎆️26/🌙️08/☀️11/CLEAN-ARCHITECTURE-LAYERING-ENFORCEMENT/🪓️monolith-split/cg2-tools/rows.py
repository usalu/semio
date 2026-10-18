import re,sys
rows=[]
for l in open(sys.argv[1]):
    m=re.match(r'crate (\S+)\s+(\S+)\s+lib (\S+)\s+test (\S+)\s+deps',l)
    if not m: continue
    def r(s):
        f,b,l_=map(int,s.split('/')); return max(f/32,b/262144,l_/4000)
    rows.append((max(r(m.group(3)),r(m.group(4))),m.group(1).replace('semio-s-artifact-bim-model-',''),m.group(3),m.group(4)))
rows.sort(reverse=True)
print(len(rows), 'over0.9:',sum(1 for x in rows if x[0]>0.9))
for x in rows[:int(sys.argv[2]) if len(sys.argv)>2 else 20]: print('%.3f'%x[0],x[1],x[2],x[3])
