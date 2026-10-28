import sys,os,json,re,subprocess
root=sys.argv[1]
files=subprocess.run(["git","-C",root,"ls-files","-z","--","🧰️framework","🚀️launcher"],capture_output=True).stdout.decode().split("\0")
if not files[0]: files=[os.path.relpath(os.path.join(d,n),root) for top in ("🧰️framework","🚀️launcher") for d,_s,ns in os.walk(os.path.join(root,top)) if "node_modules" not in d for n in ns]
files=[f for f in files if f.endswith(".json") and not re.search("tests|fixtures|🧪|🧫",f)]
translit=re.compile(r"\b\w*(ueber|fuer\b|pruef|zurueck|waehrend|koenn|muess|aender|gross\b|groess|Groess|Aender|loesch|Loesch|oeffn|Oeffn|verfueg|Verfueg|natuerlich|Fehlerf|ausfuehr|Ausfuehr|ergaenz|Ergaenz|waehl|Waehl|laeuft|Laeuft|haelt|Haelt|gueltig|Gueltig|Schluessel|Verknuepf|unterstuetz|Unterstuetz|benoetig|Benoetig|moeglich|Moeglich|aelter|Aelter|spaeter|Spaeter|Prioritaet|Qualitaet|Aktualitaet|Kapazitaet|Verfuegbar|vergroess)\w*",re.I)
english=re.compile(r"\b(the|of|for|is|are|not|was|were|has|have|with|from|and|or|waiting|running|failed|because|until|after|before|when|already|still|only|more|memory|cannot|can't|must|should|reserved|granted|queue|lease|build)\b")
def leaves(o,p=""):
    if isinstance(o,str): yield p,o
    elif isinstance(o,dict):
        for k,v in o.items(): yield from leaves(v,p+"/"+k)
    elif isinstance(o,list):
        for i,v in enumerate(o): yield from leaves(v,p+f"[{i}]")
def walk(o,path,f,out):
    if isinstance(o,dict):
        if "de" in o and ("en" in o or True) and isinstance(o["de"],(dict,str)) and not ({"type","properties"}&set(o)):
            for lp,s in leaves(o["de"],path+"/de"):
                t=translit.findall(s) and translit.search(s)
                if t: out.append((f,lp,"translit",t.group(0),s[:140]))
                words=english.findall(re.sub(r"\{[^}]*\}","",s))
                if len(words)>=2: out.append((f,lp,"english?",",".join(words[:5]),s[:140]))
        for k,v in o.items(): walk(v,path+"/"+k,f,out)
    elif isinstance(o,list):
        for i,v in enumerate(o): walk(v,path+f"[{i}]",f,out)
res=[]
for f in files:
    try: d=json.load(open(os.path.join(root,f)))
    except Exception: continue
    walk(d,"",f,res)
for r in res: print(json.dumps(r,ensure_ascii=False))
