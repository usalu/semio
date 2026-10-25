import json,re,subprocess,sys
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
log=open(sys.argv[1],errors="replace").read()
rows=re.findall(r'^\+\s+"([^:"]+):([^ "]+) ([^"]+)",$',log,re.M)
byproj={}
for proj,target,cmd in rows: byproj.setdefault(proj,[]).append((target,cmd.split(" ")))
files=subprocess.run(["git","ls-files","*📋️project.json"],capture_output=True,text=True).stdout.split("\n")
for f in files:
  if not f: continue
  try: raw=open(f,encoding="utf8").read(); d=json.loads(raw)
  except Exception: continue
  if d.get("name") not in byproj: continue
  ind=2
  fmt=json.dumps(d,indent=2,ensure_ascii=False)+"\n"==raw
  print(d["name"],f[-50:],"roundtrip",fmt,len(byproj[d["name"]]))
  if not fmt:
    text=raw
    for target,cmd in byproj[d["name"]]:
      first=cmd[0]; assert first.startswith("test-")
      start=text.index('"'+target+'"')
      pat=re.compile(r'"workspaceCommand"\s*:\s*\[([^\]]*)\]')
      m=pat.search(text,start); assert m and json.loads("["+m.group(1)+"]")==cmd,(target,m and m.group(0))
      new=json.dumps(["verify",first[5:]]+cmd[1:],ensure_ascii=False,separators=(",",":")) if "," in m.group(1) and ", " not in m.group(1) else json.dumps(["verify",first[5:]]+cmd[1:],ensure_ascii=False)
      text=text[:m.start()]+'"workspaceCommand":'+new+text[m.end():] if '"workspaceCommand":' in m.group(0) else text[:m.start()]+'"workspaceCommand": '+new+text[m.end():]
    if "--write" in sys.argv: open(f,"w",encoding="utf8").write(text)
    continue
  for target,cmd in byproj[d["name"]]:
    wc=d["targets"][target]["metadata"]["semio"]["workspaceCommand"]
    assert wc==cmd,(target,wc,cmd)
    d["targets"][target]["metadata"]["semio"]["workspaceCommand"]=["verify",cmd[0][5:]]+cmd[1:]
  if "--write" in sys.argv: open(f,"w",encoding="utf8").write(json.dumps(d,indent=2,ensure_ascii=False)+"\n")
