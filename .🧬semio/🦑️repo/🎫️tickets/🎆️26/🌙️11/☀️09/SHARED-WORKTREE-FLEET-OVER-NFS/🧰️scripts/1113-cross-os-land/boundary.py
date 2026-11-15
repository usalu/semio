from lib import *
def code(t):
    s=read(src("🚧️boundary/🟦️.ts"))
    t=sub(t,'import {join,sep} from "node:path";\n','import {join,sep} from "node:path";\nimport foldTable from "./🔤️fold/🔣️.json" with {type:"json"};\n')
    t=sub(t,'COM[1-9\\u00b9\\u00b2\\u00b3]|LPT[1-9\\u00b9\\u00b2\\u00b3])$/u;','CONIN\\$|CONOUT\\$|CLOCK\\$|COM[1-9\\u00b9\\u00b2\\u00b3]|LPT[1-9\\u00b9\\u00b2\\u00b3])$/u;') if False else t
    t=sub(t,'NUL|COM[1-9\\u00b9\\u00b2\\u00b3]|LPT[1-9\\u00b9\\u00b2\\u00b3])$/u;','NUL|CONIN\\$|CONOUT\\$|CLOCK\\$|COM[1-9\\u00b9\\u00b2\\u00b3]|LPT[1-9\\u00b9\\u00b2\\u00b3])$/u;')
    old=between(t,"/** 🔤️ Folds a file name","const compare=")
    new=between(s,"const FOLD=new Map","const compare=")
    t=t.replace(old,new,1)
    t=sub(t,'function parseLine(raw:string,index:number):Rule|undefined{','function parseLine(raw:string,index:number,insensitive:boolean):Rule|undefined{')
    t=sub(t,'expression:new RegExp(`^(?:${source})$`,"su"),literal};','expression:new RegExp(`^(?:${source})$`,insensitive?"isu":"su"),literal:literal===undefined||!insensitive?literal:foldNameV1(literal)};')
    old=between(t,"/** 🧱️ Compiles gitignore text","\n const rules:Rule[]")
    new=between(s,"/** 🧱️ Compiles gitignore text","\n const rules:Rule[]")
    t=t.replace(old,new,1)
    t=sub(t,'const rule=parseLine(line,rules.length);','const rule=parseLine(line,rules.length,insensitive);')
    t=sub(t,'for(const rule of literals.get(name)??[])if(isDirectory||!rule.dirOnly)best=Math.max(best,rule.index);','for(const rule of literals.get(insensitive?foldNameV1(name):name)??[])if(isDirectory||!rule.dirOnly)best=Math.max(best,rule.index);')
    block=between(s,"/** 🪞️ Lists the shallowest paths","/** 🛂️ Names why a worktree-relative path")
    olddoc=between(t,"/** 🛂️ Names why a worktree-relative path","\nexport function refusePathV1")
    newdoc=between(s,"/** 🛂️ Names why a worktree-relative path","\nexport function refusePathV1")
    t=t.replace(olddoc,block+newdoc,1)
    t=sub(t,'if(path.length>256&&Array.from(path).length>256)return "too-long";','if(path.length>PATH_LIMIT_UNITS_V1)return "too-long";',2)
    return t
edit("🚧️boundary/🟦️.ts",code)
for p in ["🚧️boundary/🔤️fold/📜️script.ts","🚧️boundary/🔤️fold/🔣️.json","🚧️boundary/🧪️tests/🧫️fixtures/🪟️unused"][:2]: copy_new(p)
