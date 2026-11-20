import { writeFileSync, appendFileSync, mkdirSync, readFileSync } from "node:fs"
import { join } from "node:path"
import { spawnSync } from "node:child_process"
import { checkVolumes } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🟦️.ts"
import { catalogEntries, loadAsset } from "./volumes-lib.ts"
const file = join(import.meta.dir,"🗑️generated","living-volume-check.jsonl")
mkdirSync(join(import.meta.dir,"🗑️generated"),{recursive:true})
const entries = catalogEntries().filter(e=>["vegetation","animals"].includes(e.category))
if (["animals","vegetation"].includes(process.argv[2] ?? "")) {
    for (const e of entries.filter(e=>e.category===process.argv[2])) {
        const c=spawnSync(process.execPath,[import.meta.filename,e.slug],{stdio:"inherit"})
        if(c.status!==0) process.exit(c.status || 1)
    }
} else if (process.argv[2]) {
    if(readFileSync(file,"utf8").trim().split("\n").filter(Boolean).some(l=>JSON.parse(l).id===process.argv[2])) process.exit(0)
    const entry=entries.find(e=>e.slug===process.argv[2])!
    const findings=checkVolumes(loadAsset(entry.path))
    appendFileSync(file,JSON.stringify({id:entry.slug,findings})+"\n")
    console.log(entry.slug,findings.length)
} else {
    writeFileSync(file,"")
    for (const e of entries){
        const c=spawnSync(process.execPath,[import.meta.filename,e.slug],{stdio:"inherit"})
        if(c.status!==0) process.exit(c.status || 1)
    }
}
