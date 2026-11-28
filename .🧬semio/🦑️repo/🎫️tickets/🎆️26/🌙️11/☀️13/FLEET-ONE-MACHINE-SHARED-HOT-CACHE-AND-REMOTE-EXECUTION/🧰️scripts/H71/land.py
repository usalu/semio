import os,sys,tempfile
ROOT=sys.argv[1] if len(sys.argv)>1 else "/Users/ueli/Documents/semio/🧰️framework"
DESC="🔨️modules/📁️filesystem/🧾️observation/📁️descriptor/🟦️.ts"
PLAT="🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/📤️publication/🌐️platform/🟦️.ts"
INIT="🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/📤️publication/🎛️owner/🌅️initial/🟦️.ts"
PROG="🔨️modules/🏃️process/🏗️build/🗣️language/📣️progress/🟦️.ts"
PROGS="🔨️modules/🏃️process/🏗️build/🗣️language/📣️progress/🧬️schema/🔣️.json"
READY="🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-execution/🛰️sidecar/🚦️ready/🟦️.ts"
READYS="🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-execution/🛰️sidecar/🚦️ready/🧬️schema/🔣️.json"
edits={
DESC:[
("export interface ArtifactRelativeDirectoryV1{",
 "/** 📐️ The optional per-call read bound of `withFile`: a declared size from 1 up to `RelativeProtocol.maximumFileReadBytes`; absent, the default `RelativeProtocol.maximumReadBytes` applies. */\nexport type ArtifactRelativeReadOptionsV1=Readonly<{maximumReadBytes?:number}>;\nexport interface ArtifactRelativeDirectoryV1{"),
("withFile<T>(name:string,body:(file:ArtifactReadFileV1)=>Promise<T>):Promise<T>;","withFile<T>(name:string,body:(file:ArtifactReadFileV1)=>Promise<T>,options?:ArtifactRelativeReadOptionsV1):Promise<T>;"),
("/** ⚙️ Loads only fixed operating-system ABI names after the actual Windows receiver guard. */",
 "/** 📐️ The read bound of one `withFile` call: the default, or the size the caller declared, a positive safe integer up to `maximumFileReadBytes`; any other declaration is refused with the file and the size, in English and German. */\nfunction artifactRelativeReadBoundV1(policy:Readonly<{maximumReadBytes:number;maximumFileReadBytes:number}>,options:ArtifactRelativeReadOptionsV1|undefined,path:()=>string):number{\n const declared=options?.maximumReadBytes;\n if(declared===undefined)return policy.maximumReadBytes;\n if(!Number.isSafeInteger(declared)||declared<1||declared>policy.maximumFileReadBytes)throw Error(`Declared relative file read bound required: ${path()} declared ${String(declared)} bytes, allowed 1 to ${policy.maximumFileReadBytes} / Deklarierte Lesegrenze der relativen Datei erforderlich: ${path()} erklärte ${String(declared)} Bytes, erlaubt 1 bis ${policy.maximumFileReadBytes}`);\n return declared;\n}\n/** 🧱️ The refusal of one read longer than its bound: names the file, the size and the bound in English and German, and the two ways out (a declared bound, or reading in chunks). */\nfunction artifactRelativeReadRefusalV1(kind:string,path:string,length:number,bound:number):Error{\n return Error(`Bounded ${kind} file read required: ${path} read ${length} bytes, above the bound of ${bound} bytes; declare a larger bound with the maximumReadBytes option of withFile or read in chunks / Begrenztes Lesen der Datei (${kind}) erforderlich: ${path} liest ${length} Bytes, über der Grenze von ${bound} Bytes; größere Grenze mit der Option maximumReadBytes von withFile deklarieren oder in Teilen lesen`);\n}\n/** ⚙️ Loads only fixed operating-system ABI names after the actual Windows receiver guard. */"),
("withFile<R>(name:string,run:(file:ArtifactReadFileV1)=>Promise<R>){live(this);const bytes=artifactRelativeNameV1(name);",
 "withFile<R>(name:string,run:(file:ArtifactReadFileV1)=>Promise<R>,options?:ArtifactRelativeReadOptionsV1){live(this);const bytes=artifactRelativeNameV1(name),bound=artifactRelativeReadBoundV1(policy,options,()=>place+\"/\"+name);"),
("||length>policy.maximumReadBytes||!Number.isSafeInteger(position+length)||offset+length>buffer.byteLength)throw Error(\"Bounded relative file read required\");",
 "||length>bound||!Number.isSafeInteger(position+length)||offset+length>buffer.byteLength)throw length>bound&&Number.isSafeInteger(length)?artifactRelativeReadRefusalV1(\"relative\",place+\"/\"+name,length,bound):Error(\"Bounded relative file read required\");"),
("withFile<V>(name:string,callback:(file:ArtifactReadFileV1)=>Promise<V>){live(this);artifactWindowsRelativeNameV1(name);",
 "withFile<V>(name:string,callback:(file:ArtifactReadFileV1)=>Promise<V>,options?:ArtifactRelativeReadOptionsV1){live(this);artifactWindowsRelativeNameV1(name);const bound=artifactRelativeReadBoundV1(policy,options,()=>artifactRelativeWindowsPathV1.join(framePath(),name));"),
("||length>policy.maximumReadBytes||!Number.isSafeInteger(position+length)||offset+length>buffer.byteLength)throw Error(\"Bounded Windows relative file read required\");",
 "||length>bound||!Number.isSafeInteger(position+length)||offset+length>buffer.byteLength)throw length>bound&&Number.isSafeInteger(length)?artifactRelativeReadRefusalV1(\"Windows relative\",artifactRelativeWindowsPathV1.join(framePath(),name),length,bound):Error(\"Bounded Windows relative file read required\");"),
],
PLAT:[("throw Error(\"Staged immutable input content changed\");});const after=await directory.entry(name);","throw Error(\"Staged immutable input content changed\");},{maximumReadBytes:chunk.decoded});const after=await directory.entry(name);")],
INIT:[("const read=file.read(bytes,offset,Number(receiptEntry.size)-offset,offset);","const read=file.read(bytes,offset,Math.min(65536,Number(receiptEntry.size)-offset),offset);")],
PROGS:[("\"definitions\": {\n    \"TypedRefusals\": {","\"definitions\": {\n    \"FaultDetail\": {\n      \"description\": \"A stage of the ready supervisor sets `environment` to `enabled`: every internal fault at the CLI boundary then also writes one line `prefix` + reference + the JSON of its cause chain (name, message and the first `maximumStackLines` stack frames of each of at most `maximumErrors` faults, cut at `maximumCharacters`) before the internal-error line, so the failed-stage log in the fleet store names the throwing frame. A terminal run without the variable never prints a stack.\",\n      \"const\": {\"environment\": \"SEMIO_FAULT_DETAIL\", \"enabled\": \"1\", \"prefix\": \"[DATA] Fault detail / Fehlerdetail \", \"maximumErrors\": 8, \"maximumStackLines\": 10, \"maximumCharacters\": 3600}\n    },\n    \"TypedRefusals\": {")],
PROG:[
("/** 🧯️ Turns anything that is not a typed refusal",
 "/** 🔬️ The line of the cause chain of an unexpected fault for the failed-stage log of a fleet worker: reference, then name, message and top stack frames of the fault, of its causes and of the faults it aggregates, bounded by `FaultDetail`. */\nexport const faultDetailLineV1 = (error: unknown): string => {\n  const policy = schema.definitions.FaultDetail.const, chain: object[] = [];\n  const visit = (value: unknown): void => {\n    if (chain.length >= policy.maximumErrors) return;\n    chain.push(value instanceof Error ? { name: value.name, message: value.message, stack: (value.stack ?? \"\").split(\"\\n\").slice(1, 1 + policy.maximumStackLines).map((row) => row.trim()) } : { name: typeof value, message: String(value), stack: [] });\n    if (value instanceof AggregateError) for (const member of value.errors) visit(member);\n    else if (value instanceof Error && value.cause !== undefined) visit(value.cause);\n  };\n  visit(error);\n  return `${policy.prefix}${faultReferenceV1(error)} ${JSON.stringify(chain)}`.slice(0, policy.maximumCharacters);\n};\n\n/** 🧯️ Turns anything that is not a typed refusal"),
("  if (!reportRepoErrorV1(error, options)) reportRepoErrorV1(refuseInternalV1(error), options);\n}",
 "  if (reportRepoErrorV1(error, options)) return;\n  const detail = schema.definitions.FaultDetail.const;\n  if ((options.environment ?? process.env)[detail.environment] === detail.enabled) (options.write ?? processWriteV1)(faultDetailLineV1(error));\n  reportRepoErrorV1(refuseInternalV1(error), options);\n}"),
],
READY:[("[P.capacityPriorityEnvironment]:P.capacityPriority};","[P.capacityPriorityEnvironment]:P.capacityPriority,[P.faultDetailEnvironment]:P.faultDetailEnabled};")],
READYS:[
("        \"tailBytes\": 16384,\n","        \"tailBytes\": 16384,\n        \"faultDetailEnvironment\": \"SEMIO_FAULT_DETAIL\",\n        \"faultDetailEnabled\": \"1\",\n"),
("\"^\\\\[DATA\\\\] Store dependency installer (?:failed|stderr) \"],","\"^\\\\[DATA\\\\] Store dependency installer (?:failed|stderr) \", \"^\\\\[DATA\\\\] Fault detail / Fehlerdetail \"],"),
("\"^(?:Internal error: )?Current expected JavaScript dependency binding required","\"^\\\\[DATA\\\\] Fault detail / Fehlerdetail \",\"^(?:Internal error: )?Current expected JavaScript dependency binding required"),
],
}
plan=[]
only=os.environ.get('H71_ONLY')
for rel,rows in edits.items():
    if only and rel not in {'desc':[DESC],'ready':[READY,READYS],'process':[PLAT,INIT,PROG,PROGS]}[only]:continue
    path=os.path.join(ROOT,rel);s=open(path,encoding="utf8").read()
    for old,new in rows:
        assert s.count(old)==1,(rel,s.count(old),old[:90])
        s=s.replace(old,new)
    plan.append((path,s))
if "--check" in sys.argv:print("anchors ok");sys.exit(0)
for path,s in plan:
    fd,tmp=tempfile.mkstemp(dir=os.path.dirname(path),prefix=".h71-");os.write(fd,s.encode("utf8"));os.close(fd)
    os.chmod(tmp,os.stat(path).st_mode&0o7777);os.replace(tmp,path);print("landed",path[-70:])
