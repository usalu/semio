import json,sys,re
C=sys.argv[1];G=C+'/🗑️generated/law-sweep2';f=sys.argv[2] if len(sys.argv)>2 else 'verify'
lo=int(sys.argv[3]) if len(sys.argv)>3 else 0
rows={}
for l in open(G+'/results-%s.jsonl'%f):
    r=json.loads(l);rows[r['i']]=r
for i,r in sorted(rows.items()):
    if i<lo or r.get('rc') in (0,None): continue
    t=open(G+'/log-%03d.txt'%i).read() if f=='verify' else ''
    ls=[x for x in t.split('\n') if re.search(r'^(error|ENOENT|Error|AssertionError|TypeError|\S*error TS|.*Cannot find module|.*\(fail\)|TIMEOUT|.*SyntaxError)',x)]
    print(i,r['law'].replace('verify ',''),'rc',r['rc'],'|',(ls[0] if ls else t[-150:].replace('\n',' '))[:230].replace('/Users/ueli/Documents/semio/',''))
