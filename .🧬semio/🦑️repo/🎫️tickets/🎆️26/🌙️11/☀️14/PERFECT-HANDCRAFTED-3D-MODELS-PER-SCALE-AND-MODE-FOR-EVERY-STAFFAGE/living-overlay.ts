import {entryOf} from "./volumes-lib.ts"
import {renderScale} from "./volumes-render.ts"
import {join} from "node:path"
for (const s of process.argv.slice(2)){
    const [name,number]=s.split(":")
    const {asset}=entryOf("vegetation/"+name)
    renderScale(asset,Number(number),{},["body","thermal","structure"],{az:24,el:20,cut:true,width:900},()=>join(import.meta.dir,"🗑️generated","living",s.replace(":","-")+"-overlay.png"))
}
