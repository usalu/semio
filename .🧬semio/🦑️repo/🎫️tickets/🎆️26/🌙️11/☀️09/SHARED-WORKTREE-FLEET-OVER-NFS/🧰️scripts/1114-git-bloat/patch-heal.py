import sys
p=sys.argv[1]+"/🔁️replica/🧪️tests/🩹️heal.ts";s=open(p).read()
s=s.replace('function repack(who:Machine):{old:string[];fresh:string[]}{','async function repack(who:Machine):Promise<{old:string[];fresh:string[]}>{')
a=' return {old:old.filter(path=>!now.includes(path)),fresh:now.filter(path=>!old.includes(path))};'
assert a in s
s=s.replace(a,' const result={old:old.filter(path=>!now.includes(path)),fresh:now.filter(path=>!old.includes(path))},stem=(path:string)=>path.slice(path.lastIndexOf("/")+1).replace(/\\.[a-z]+$/u,"");\n await authorisePacksV1(who.options.worktree,{publish:result.fresh.map(stem),retire:[...result.old.map(stem),"multi-pack-index"]});\n return result;')
assert s.count("const {old,fresh}=repack(server)")==2
s=s.replace("const {old,fresh}=repack(server)","const {old,fresh}=await repack(server)")
s=s.replace('import {gitBrokenV1,gitStateV1} from "../../🌿️git/🟦️.ts";','import {gitBrokenV1,gitStateV1} from "../../🌿️git/🟦️.ts";\nimport {authorisePacksV1} from "../🧷️packgate/🟦️.ts";')
open(p,'w').write(s)
