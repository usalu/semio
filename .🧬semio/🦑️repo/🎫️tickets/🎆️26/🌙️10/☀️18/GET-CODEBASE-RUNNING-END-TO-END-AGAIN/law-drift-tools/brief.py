import json,sys,re,os
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
r1=[json.loads(l) for l in open(C+"/🗑️generated/law-drift-run/r1.jsonl")]
cur={}
for fn in ["r1","r2","r3"]:
    q=C+"/🗑️generated/law-drift-run/"+fn+".jsonl"
    if os.path.exists(q):
        for l in open(q): d=json.loads(l); cur[d['file']]=d
a,b=int(sys.argv[1]),int(sys.argv[2])
for i in range(a,b+1):
    r=cur[r1[i]['file']]
    print("###",i,r['file'].replace('🧰️framework/','FW/').replace('🛍️products/','P/').replace('🦑️repo/🔨️modules/📚️library','REPO-LIB'),r['pass_'],r['fail'])
    t=open(r['log'],encoding='utf-8').read() if r.get('log') else ''
    for m in list(re.finditer(r'\n(error:[^\n]*\n(?:(?!\n\(fail\)|\nerror:)[^\n]*\n){0,14}?\(fail\)[^\n]*)',t))[:3]:
        lines=[l for l in m.group(1).split('\n') if l.strip()]
        keep=[l[:230] for l in lines if re.match(r'(error:|Expected|Received|@@|[+-] |\(fail\)|\s+at <anonymous>)',l)]
        print("\n".join(keep[:9])); print('-')
