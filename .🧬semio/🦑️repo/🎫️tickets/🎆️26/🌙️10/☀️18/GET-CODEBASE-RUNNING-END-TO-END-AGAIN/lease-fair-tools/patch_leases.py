import sys,re
path=sys.argv[1]
s=open(path,encoding='utf8').read()
def once(old,new):
    global s
    assert s.count(old)==1,(s.count(old),old[:80])
    s=s.replace(old,new)
once('import { lstatSync, mkdirSync, readdirSync, readFileSync, realpathSync, rmSync, statfsSync, unlinkSync, writeFileSync } from "node:fs";',
     'import { lstatSync, mkdirSync, readdirSync, readFileSync, realpathSync, renameSync, rmSync, statfsSync, unlinkSync, writeFileSync } from "node:fs";')
once('import queueSchema from "./🧬️schema/📜️ticket.json" with { type: "json" };\n',
     'import queueSchema from "./🧬️schema/📜️ticket.json" with { type: "json" };\nimport waitPolicySchema from "./⏳️wait/🧬️schema/📜️policy.json" with { type: "json" };\n')
start=s.index('/** 🚦️ Serves original kernel-held tickets in arrival order;')
end=s.index('/** 🪢️ Orders a resource set consistently')
new=open(sys.argv[2],encoding='utf8').read()
s=s[:start]+new+s[end:]
open(path,'w',encoding='utf8').write(s)
