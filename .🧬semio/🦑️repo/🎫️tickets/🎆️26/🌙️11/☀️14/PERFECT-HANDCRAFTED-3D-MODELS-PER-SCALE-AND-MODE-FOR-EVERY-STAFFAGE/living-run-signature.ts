/** 🌿 Fingerprint runtime modules actually imported by the living audit. */
import {createHash} from 'node:crypto'
import {existsSync,readFileSync} from 'node:fs'
import {dirname,join,relative,resolve} from 'node:path'
const root=process.cwd(),mode=process.argv[2],staff=resolve(root,'🎓️teaching/🏛️architecture/🧍️staffage')
const seeds=mode==='glb'?[join(staff,'🧪️tests/🧊️model-catalog/🔍️oracles/🟦️.ts'),join(staff,'🔨️modules/📤️export/🧊️glb/🟦️.ts')]:[join(staff,'🔨️modules/🗂️catalog/🧊️volumes/🟦️.ts')]
const compile=new Bun.Transpiler({loader:'ts',target:'bun',minifyWhitespace:true,minifySyntax:true})
const hashes=new Map<string,string>()
function visit(path:string){
 if(hashes.has(path)||!existsSync(path))return
 const input=readFileSync(path,'utf8')
 const code=path.endsWith('.json')?JSON.stringify(JSON.parse(input)):compile.transformSync(input)
 hashes.set(path,createHash('sha256').update(code).digest('hex'))
 if(path.endsWith('.json'))return
 const imports=compile.scanImports(code).map(item=>item.path)
 for(const match of input.matchAll(/new URL\(\s*["'](\.[^"']+\.json)["']/g))imports.push(match[1]!)
 for(const target of imports){if(target.startsWith('.')&&/\.(?:ts|json)$/.test(target))visit(resolve(dirname(path),target))}
}
seeds.forEach(visit)
if(process.argv.includes('--trace'))console.error(hashes.size,[...hashes.keys()].map(path=>relative(root,path)).join('\n'))
const hash=createHash('sha256').update(Bun.version).update(readFileSync(join(import.meta.dir,'living-current.ts')))
for(const path of [...hashes.keys()].sort())hash.update(relative(root,path)).update(hashes.get(path)!)
for(const dep of ['three','gltf-validator','@gltf-transform/core','ajv']){
 const p=join(root,'node_modules',dep,'package.json');if(existsSync(p))hash.update(JSON.parse(readFileSync(p,'utf8')).version)
}
console.log(hash.digest('hex'))

