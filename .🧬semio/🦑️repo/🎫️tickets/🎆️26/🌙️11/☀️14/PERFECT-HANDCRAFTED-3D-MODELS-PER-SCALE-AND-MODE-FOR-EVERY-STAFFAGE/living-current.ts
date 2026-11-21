/** 🌿 Check one catalog revision at its four scales, retaining only compact verification facts. */
import {createHash} from "node:crypto"
import {mkdirSync,readFileSync,writeFileSync,renameSync} from "node:fs"
import {join} from "node:path"
import {checkVolumes} from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🟦️.ts"
import {catalogEntries} from "./volumes-lib.ts"

const [id, mode = "strict", only] = process.argv.slice(2)
const scale = only === undefined ? undefined : Number(only) as 50 | 100 | 200 | 500
if (scale !== undefined && (![50, 100, 200, 500].includes(scale) || mode !== "strict")) throw Error("Only volume checks accept a scale")
const entry = catalogEntries().find(e => e.slug === id)
if (!entry || !["strict", "glb"].includes(mode)) throw Error("Supply an asset slug and strict or glb")
const raw = readFileSync(entry.path, "utf8")
const hash = createHash("sha256").update(raw).digest("hex")
const asset = JSON.parse(raw).asset
const directory = join(import.meta.dir, "🗑️generated", "living-current")
mkdirSync(directory, {recursive:true})
let findings: unknown[] = []
let scenes: unknown, bytes: number | undefined, payload: Record<string, number> | undefined

try {
    if (mode === "strict") {
        findings = [...checkVolumes(asset, scale === undefined ? {} : {scale})]
    } else {
        const {auditGeometry, checkGlb, expectedScenes, BUDGET} = await import("../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧊️model-catalog/🔍️oracles/🟦️.ts")
        const {writeGlb} = await import("../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/📤️export/🧊️glb/🟦️.ts")
        {
            const report = await auditGeometry(asset)
            findings.push(...report.findings)
            scenes = report.facts.scenes
        }
        Bun.gc(true)
        const data = await writeGlb(asset, {pose:asset.presentation.defaultPose})
        bytes = data.byteLength
        const jsonBytes = new DataView(data.buffer, data.byteOffset+12, 4).getUint32(0, true)
        const manifest = JSON.parse(new TextDecoder().decode(data.subarray(20,20+jsonBytes)))
        payload = {json:jsonBytes}
        const used = new Set<number>()
        function account(accessor:number, role:string) {
            const view = manifest.accessors[accessor].bufferView
            if (used.has(view)) return
            used.add(view)
            payload![role] = (payload![role] ?? 0) + manifest.bufferViews[view].byteLength
        }
        for (const mesh of manifest.meshes) for (const primitive of mesh.primitives) {
            for (const [role, accessor] of Object.entries(primitive.attributes)) account(accessor as number,role)
            if (primitive.indices !== undefined) account(primitive.indices,primitive.mode === 1 ? "edges" : "triangles")
        }
        if (bytes > BUDGET.glbBytes) findings.push({check:"budget",item:id,message:bytes+" bytes in presentation GLB, limit "+BUDGET.glbBytes})
        findings.push(...(await checkGlb(data, id + ".glb", id, expectedScenes(asset))).findings)
    }
} catch (error) {
    findings.push({error:String(error)})
}
const output = join(directory,[id, mode, only].filter(Boolean).join("-")+".json")
const partial = output+"."+process.pid+".tmp"
writeFileSync(partial,JSON.stringify({id,hash,code:process.env.LIVING_AUDIT_CODE,scale,scenes,bytes,payload,findings}))
renameSync(partial,output)
console.log("[DEBUG]",id,mode,findings.length)

