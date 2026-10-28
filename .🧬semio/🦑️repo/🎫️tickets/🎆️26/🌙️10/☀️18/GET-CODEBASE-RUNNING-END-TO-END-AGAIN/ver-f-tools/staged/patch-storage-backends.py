"""usage: patch-storage-backends.py [root] — makes the native-storage backend laws green without growing the pinned entanglements:
 1. NativeStorageGeometryV1 gains `invalidate()` and `mapped(directories, control)`; the registry contract exports `invalidateNativeStorageRostersV1` and `nativeStorageMappedV1`.
 2. The mounted-image backend answers both (roster invalidation, `hdiutil info -plist` mapping check moved out of the neutral retirement module), the directory-partition backend answers `false`/no-op.
 3. `🧹️whole` asks the registry instead of naming the host tool, `📤️publication/🎛️owner` invalidates through the registry.
 4. The foreign-mount refusal text no longer names the host tool (the device is still named); its test follows.
 5. The two remaining `🧹️whole` -> directory-partition edges are pinned in the law fixture (debt of the retirement module; they cannot be dispatched without a retention port)."""
import os,sys
root=os.path.abspath(sys.argv[1]) if len(sys.argv)>1 else os.getcwd()
S=os.path.join(root,"🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage")

def edit(rel,pairs):
    path=os.path.join(S,rel);s=open(path).read()
    for old,new in pairs:
        assert s.count(old)==1,(rel,old[:80],s.count(old))
        s=s.replace(old,new)
    open(path,"w").write(s)

edit("🔌️backends/📋️contract/🔌️operations/🟦️.ts",[(
 " space(root:string,declared:number):Readonly<{bsize:number;bavail:number}>;\n}",
 " space(root:string,declared:number):Readonly<{bsize:number;bavail:number}>;\n invalidate():void;\n mapped(directories:readonly string[],control:ResourceLeaseControl):Promise<boolean>;\n}")])

edit("🔌️backends/📋️contract/🟦️.ts",[
 ('import {existsSync} from "node:fs";','import {existsSync} from "node:fs";\nimport type {ResourceLeaseControl} from "../../../../../🔒️leases/🟦️.ts";'),
 ("/** 🪣️ Reports allocation unit and free units of the backend that owns the root's backing file. */",
  "/** 🧹️ Drops the cached host attachment facts of every bound backend after the physical state changed. */\nexport const invalidateNativeStorageRostersV1=():void=>{for(const value of geometries.values())value.invalidate();};\n/** 🗺️ Tells whether the host still maps the backing file or the volume root of any given directory, by the backend that owns the first directory's backing file. */\nexport const nativeStorageMappedV1=(directories:readonly string[],control:ResourceLeaseControl):Promise<boolean>=>geometry(nativeStorageBackendOfDirectoryV1(directories[0]!).id).mapped(directories,control);\n/** 🪣️ Reports allocation unit and free units of the backend that owns the root's backing file. */")])

edit("🔌️backends/🍎️mounted-image/📐️geometry/🟦️.ts",[
 ('import {lstatSync,statfsSync} from "node:fs";','import {lstatSync,statfsSync} from "node:fs";\nimport {dirname,join} from "node:path";\nimport {captureOwnedCommandV1} from "../../../../../../🎛️owned-execution/🟦️.ts";\nimport {nativeImageBackingFilenameV1,nativeImageRootV1} from "../../../📋️contract/🟦️.ts";\nimport {nativeImageInformationV1} from "../../../🍎️image/📥️information/🟦️.ts";\nimport {invalidateNativeImageRosterV1} from "../../../🍎️image/📡️roster/🟦️.ts";\nimport storageSchema from "../../../🧬️schema/🔣️.json" with {type:"json"};'),
 (" space:root=>{const filesystem=statfsSync(root);return Object.freeze({bsize:filesystem.bsize,bavail:filesystem.bavail});}\n};",
  " space:root=>{const filesystem=statfsSync(root);return Object.freeze({bsize:filesystem.bsize,bavail:filesystem.bavail});},\n invalidate:invalidateNativeImageRosterV1,\n mapped:async(directories,control)=>{\n  const protocol=storageSchema.definitions.ImageProtocol.const,images=directories.map(directory=>join(directory,nativeImageBackingFilenameV1())),roots=directories.map(directory=>nativeImageRootV1(directory)),text=await captureOwnedCommandV1(\"hdiutil\",[\"info\",\"-plist\"],dirname(directories[0]!),{control,output:{maximumBytes:protocol.informationMaximumBytes,maximumLines:protocol.informationMaximumLines}}),information=nativeImageInformationV1(text);\n  if(!Array.isArray(information.images))return true;\n  return (information.images as Record<string,unknown>[]).some(row=>images.includes(row[\"image-path\"] as string)||Array.isArray(row[\"system-entities\"])&&(row[\"system-entities\"] as Record<string,unknown>[]).some(entity=>roots.includes(entity[\"mount-point\"] as string)));\n }\n};")])

edit("🔌️backends/📁️directory-partition/📐️geometry/🟦️.ts",[
 ("};\nbindNativeStorageGeometryV1(\"directory-partition\"",",\n invalidate:()=>{},\n mapped:async()=>false\n};\nbindNativeStorageGeometryV1(\"directory-partition\"")])

whole="🧹️whole/🟦️.ts"
path=os.path.join(S,whole);s=open(path).read()
a=s.index('if(process.platform==="darwin"){const{captureOwnedCommandV1}')
tail='throw Error("Unmapped terminal Native backing required");}'
b=s.index(tail,a)+len(tail)
s=s[:a]+'if(await nativeStorageMappedV1([directory,originalDirectory],control))throw Error("Unmapped terminal Native backing required");'+s[b:]
old='import{selectNativeStorageBackendV1}from"../🔌️backends/📋️contract/🟦️.ts";'
assert s.count(old)==1
s=s.replace(old,'import{selectNativeStorageBackendV1,nativeStorageMappedV1}from"../🔌️backends/📋️contract/🟦️.ts";')
open(path,"w").write(s)

edit("📤️publication/🎛️owner/🟦️.ts",[
 ('import{invalidateNativeImageRosterV1}from"../../🍎️image/📡️roster/🟦️.ts";\nimport{nativeStorageBackendOfDirectoryV1,','import{invalidateNativeStorageRostersV1,nativeStorageBackendOfDirectoryV1,'),
 ('invalidateNativeImageRosterV1();checkBacking(token)','invalidateNativeStorageRostersV1();checkBacking(token)')])

edit("🧹️cleanup/📥️receiver/🧬️schema/🔣️.json",[
 ("Detach it with `/usr/bin/hdiutil detach {device}` (a test sandbox","Detach the device {device} (a test sandbox"),
 ("Mit `/usr/bin/hdiutil detach {device}` aushaengen (eine Test-Sandbox","Das Geraet {device} aushaengen (eine Test-Sandbox")])
edit("🧹️cleanup/🎛️command/🧪️tests/🟦️.ts",[('expect(text).toContain("hdiutil detach /dev/disk23s1");','expect(text).toContain("/dev/disk23s1");')])

fixture=os.path.join(S,"🔌️backends/🧪️tests/⚖️laws/🧫️fixtures/🔣️.json");s=open(fixture).read()
anchor='    "🧾️metadata/🟦️.ts -> directory-partition: 📁️directory/🟦️.ts",'
assert s.count(anchor)==1
s=s.replace(anchor,'    "🧹️whole/🟦️.ts -> directory-partition: 📁️directory/🟦️.ts",\n    "🧹️whole/🟦️.ts -> directory-partition: 🧱️fixed/🧬️schema/🔣️.json",\n'+anchor)
open(fixture,"w").write(s)
