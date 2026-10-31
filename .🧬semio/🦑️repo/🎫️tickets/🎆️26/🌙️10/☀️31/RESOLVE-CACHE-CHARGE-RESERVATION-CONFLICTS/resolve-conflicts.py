from pathlib import Path

root = Path(__file__).resolve().parents[7]
reservation = root / "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/🏪️store/🎟️reservation"

implementation = """ const quote=()=>{if(!existsSync(receipt))return;const file=lstatSync(receipt);if(!file.isFile()||file.isSymbolicLink()||file.nlink!==1||file.size>2048)throw Error("Original cache charge receipt required");const row=JSON.parse(readFileSync(receipt,"utf8"));if(validateJsonSchemaSubset(schema,row).length||row.owner!==owner||row.bytes!==policy.maximumArtifactBytes)throw Error("Original cache charge disagrees with global policy");return row as{allocation:string;owner:string;bytes:number};},expected=quote(),storage=await acquireNativeStorageOwnerV1({policy,control,port:image.port,owner,kind:"artifact-publication",bytes:policy.maximumArtifactBytes,...(expected?{expectedAllocation:expected.allocation}:{})});
 if(storage.allocation.owner!==owner||storage.allocation.kind!=="artifact-publication"||storage.allocation.bytes!==policy.maximumArtifactBytes||storage.allocation.phase!=="committed")throw Error("Original cache allocation refused");await storage.recheck();
 const lease=await acquireQueuedResourceLease({directory:leaseDirectory,resource:"repository-cache-charge:"+image.root,mode:"exclusive",owner:randomBytes(8).toString("hex"),signal:control.signal,control});
 try{const current=quote();if(current&&current.allocation!==storage.allocation.id)throw Error("Original cache charge disagrees with durable allocation");if(!current){const temporary=requireNativeStoragePathV1(image.root,receipt+"~"+randomBytes(8).toString("hex"));try{writeFileSync(temporary,JSON.stringify({version:1,allocation:storage.allocation.id,owner,bytes:storage.allocation.bytes})+"\\n",{flag:"wx"});const fd=openSync(temporary,"r+");try{fsyncSync(fd);}finally{closeSync(fd);}renameSync(temporary,receipt);await syncNativeDirectoryMetadataV1(image.root,image.root,control);}finally{rmSync(temporary,{force:true});}}}finally{lease.release();}
 await storage.recheck();await image.recheck();checkResourceLeaseControl(control);return storage;
"""

def replace_conflict(path: Path, chosen: str) -> None:
    text = path.read_text(encoding="utf-8")
    start = text.index("<<<<<<<")
    end = text.index(">>>>>>>", start)
    end = text.index("\n", end) + 1
    path.write_text(text[:start] + chosen + text[end:], encoding="utf-8", newline="\n")

source = (reservation / "🟦️.ts").read_text(encoding="utf-8")
start = source.index("<<<<<<<")
end = source.index(">>>>>>>", start)
end = source.index("\n", end) + 1
resolved = source[:start] + implementation + source[end:]
if "<<<<<<<" in resolved or ">>>>>>>" in resolved or "=======" in resolved:
    raise SystemExit("implementation still conflicted")
(reservation / "🟦️.ts").write_text(resolved, encoding="utf-8", newline="\n")

fixture = (reservation / "🧪️tests/🔣️.json").read_text(encoding="utf-8")
fixture_start = fixture.index("<<<<<<<")
ours_end = fixture.index("=======", fixture_start)
ours = fixture[fixture.index("\n", fixture_start) + 1:ours_end]
if not ours.startswith("{"):
    ours = "{\n" + ours
if not ours.rstrip().endswith("}"):
    ours = ours.rstrip() + "\n}\n"
(reservation / "🧪️tests/🔣️.json").write_text(ours if ours.endswith("\n") else ours + "\n", encoding="utf-8", newline="\n")

test_path = reservation / "🧪️tests/🟦️.ts"
test = test_path.read_text(encoding="utf-8")
test_start = test.index("<<<<<<<")
test_ours_end = test.index("=======", test_start)
ours_test = test[test.index("\n", test_start) + 1:test_ours_end]
tail = test[test.index(">>>>>>>", test_ours_end):]
tail = tail[tail.index("\n") + 1:]
merged = test[:test_start] + ours_test + tail
needle = 'acquireNativeStorageOwnerV1:async(options:any)=>{native("owner");expect(options.expectedAllocation??null).toBe(row.receipt);if(selected.result==="refused")throw Error("owner quote refused");return storage;}}'
insert = 'acquireNativeStorageOwnerV1:async(options:any)=>{native("owner");expect(options.expectedAllocation??null).toBe(row.receipt);if(selected.result==="refused")throw Error("owner quote refused");return storage;},syncNativeDirectoryMetadataV1:async(directoryRoot:string,directory:string,actual:unknown)=>{expect(held).toBe(true);expect(directoryRoot).toBe(root);expect(directory).toBe(root);expect(actual).toBe(control);events.push("receipt:directory");}}'
if needle not in merged:
    raise SystemExit("owner port not found")
merged = merged.replace(needle, insert, 1)
success = 'expect(events.indexOf("native:owner")).toBeLessThan(events.indexOf("receipt:hold"));expect(events.at(-1)).toBe("native:image-recheck");'
success_next = 'expect(events.indexOf("native:owner")).toBeLessThan(events.indexOf("receipt:hold"));expect(events.indexOf("receipt:write")).toBeLessThan(events.indexOf("receipt:directory"));expect(events.indexOf("receipt:directory")).toBeLessThan(events.indexOf("receipt:release"));expect(events.at(-1)).toBe("native:image-recheck");'
if success not in merged:
    raise SystemExit("success assertion not found")
merged = merged.replace(success, success_next, 1)
displaced = 'expect(events).not.toContain("receipt:write");'
displaced_next = 'expect(events).not.toContain("receipt:write");expect(events).not.toContain("receipt:directory");'
if displaced not in merged:
    raise SystemExit("displaced assertion not found")
merged = merged.replace(displaced, displaced_next, 1)
if "<<<<<<<" in merged or ">>>>>>>" in merged:
    raise SystemExit("test still conflicted")
test_path.write_text(merged, encoding="utf-8", newline="\n")
print("resolved")
