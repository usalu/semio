import sys,json
rows=[json.loads(l) for l in open(sys.argv[1])]
rows.sort(key=lambda r:-r['peakKb'])
for r in rows[:int(sys.argv[2]) if len(sys.argv)>2 else 25]:
    print(r['peakKb']//1024,'MB',r['durS'],'s',r['cmd'][-170:],'|',(r['anc'] or [''])[0][-100:])
