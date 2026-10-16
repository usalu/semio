import sys,json,re,os
FILES=[".vscode/launch.json",".vscode/🧩️launch.seed.jsonc"]
def entry(e):
    env=json.dumps(e["env"],ensure_ascii=False)
    return ('    {\n'
     f'      "name": {json.dumps(e["name"],ensure_ascii=False)},\n'
     '      "type": "node-terminal",\n'
     '      "request": "launch",\n'
     f'      "command": {json.dumps(e["command"],ensure_ascii=False)},\n'
     '      "cwd": "${workspaceFolder}",\n'
     f'      "env": {env},\n'
     '      "presentation": {\n'
     f'        "group": {json.dumps(e["group"])},\n'
     f'        "order": {e["order"]}\n'
     '      }\n'
     '    },\n')
def insert(spec):
    anchor=spec["after"]; entries=spec["entries"]
    for f in FILES:
        t=open(f,encoding="utf8",newline="").read()
        for e in entries:
            if f'"name": {json.dumps(e["name"],ensure_ascii=False)}' in t: raise SystemExit(f"{f}: already registered {e['name']}")
        key=f'      "name": {json.dumps(anchor,ensure_ascii=False)},\n'
        assert t.count(key)==1,(f,anchor,t.count(key))
        i=t.index(key)
        j=t.index("\n    },\n",i)+len("\n    },\n")
        t=t[:j]+"".join(entry(e) for e in entries)+t[j:]
        open(f,"w",encoding="utf8",newline="").write(t)
        print("inserted",len(entries),"into",f)
if __name__=="__main__":
    insert(json.load(open(sys.argv[1],encoding="utf8")))
