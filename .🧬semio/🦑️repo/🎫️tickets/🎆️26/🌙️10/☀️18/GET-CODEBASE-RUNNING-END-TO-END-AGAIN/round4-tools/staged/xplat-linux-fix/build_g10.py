import os, sys
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), "groups"))
from _gen import emit, read_abs, text_span
HERE = os.path.dirname(os.path.abspath(__file__))
C = os.path.dirname(HERE)
GS = os.path.join(C, "cache-gc-tools", "staged")
L = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
RUN, TST = L + "/🏗️build/🧹️cache-gc/🏃️runner/🟦️.ts", L + "/🏗️build/🧹️cache-gc/🧪️tests/🟦️.ts"
SCHEMA = L + "/⚡️caching/💾️storage/🧾️ledger/🧬️schema/🔣️.json"
FACADE = "🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/💾️storage/🌐️platform/📋️contract/🟦️.ts"
rel = os.path.relpath("/Users/ueli/Documents/semio/" + FACADE, os.path.dirname("/Users/ueli/Documents/semio/" + RUN))
runner, test, schema = read_abs(os.path.join(GS, "gc-runner.ts")), read_abs(os.path.join(GS, "gc-test.ts")), read_abs(os.path.join(GS, "ledger-schema.json"))
edits = []
edits.append((RUN, 'import {chmod,lstat,readdir,rmdir,statfs,unlink} from "node:fs/promises";', 'import {chmod,lstat,readdir,rmdir,unlink} from "node:fs/promises";\nimport {nativeImageBackingFilenameV1,nativeImageDirectoryHostV1,nativePartitionOfV1,nativeStatfsV1} from "' + rel + '";'))
edits.append((RUN, ' sqliteFree(file:string):Promise<boolean>;\n', ' sqliteFree(file:string):Promise<boolean>;\n platform?:NodeJS.Platform;\n'))
old = text_span(runner, " if(child?.isDirectory()&&child.dev!==outer.dev){", "\n const measure=await measureCacheTreeV1(inner", False)
new = ''' const platform=ports.platform??process.platform,backing=nativeImageBackingFilenameV1(platform),mounted=nativeImageDirectoryHostV1(platform)?nativePartitionOfV1(inner,platform)?.attached===true:child?.isDirectory()===true&&child.dev!==outer.dev;
 if(mounted){const row=await nativeStatfsV1(inner,{platform});return{state:"mounted",usedBytes:(row.blocks-row.bfree)*row.bsize,capacityBytes:row.blocks*row.bsize,partial:false};}
 if(await exists(join(directory,backing)))return{state:"detached",usedBytes:null,capacityBytes:null,partial:false};'''
edits.append((RUN, old, new))
edits.append((RUN, 'if(info.nlink>1){const key=info.dev+":"+info.ino;if(seen.has(key))continue;seen.add(key);}', 'if(info.nlink>1){const exact=Number.isSafeInteger(info.ino)?info:await lstat(full,{bigint:true}).catch(()=>info),key=exact.dev+":"+exact.ino;if(seen.has(key))continue;seen.add(key);}'))
edits.append((RUN, '/** 📈️ Reads the ledger\'s status from the disk without taking a lease: statfs for mounted images, bounded walks for everything else. */', '/** 📈️ Reads the ledger\'s status from the disk without taking a lease: the envelope and the measured tree for mounted partitions (statfs of the mounted image on macOS, the declared envelope of a directory partition on Linux and Windows), bounded walks for everything else. */'))
edits.append((SCHEMA, '        "imageFile": "fixed.dmg",\n', ''))
edits.append((TST, 'writeFileSync(join(workspace,".cache/b/a/fixed.dmg"),"");', 'writeFileSync(join(workspace,".cache/b/a",nativeImageBackingFilenameV1()),"");'))
header = test.split("\n", 1)[0]
edits.append((TST, header, header + "\n" + 'import {nativeImageBackingFilenameV1} from "' + os.path.relpath("/Users/ueli/Documents/semio/" + FACADE, os.path.dirname("/Users/ueli/Documents/semio/" + TST)) + '";'))
new_files = [L + "/🏗️build/🧹️cache-gc/🧪️tests/📁️directory/🟦️.ts"]
emit("g10", "cache status and gc know directory partitions: mounted means attached, capacity is the declared envelope, the backing name comes from the native protocol", [RUN], edits, new_files, os.path.join(HERE, "groups"))
print("g10 written", len(edits))
