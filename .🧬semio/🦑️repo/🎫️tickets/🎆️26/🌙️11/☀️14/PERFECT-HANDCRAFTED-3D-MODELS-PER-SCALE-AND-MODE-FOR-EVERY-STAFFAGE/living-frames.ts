import {writeFileSync} from "node:fs"
import {join} from "node:path"
import {entryOf,poseState} from "./volumes-lib.ts"
const frames={}
for(const name of ["apple-tree","english-oak","japanese-maple","lombardy-poplar","london-plane-pollarded"]){
    const {asset}=entryOf("vegetation/"+name)
    frames[name]=poseState(asset,500).frames
}
writeFileSync(join(import.meta.dir,"🗑️generated","living-frames.json"),JSON.stringify(frames))

