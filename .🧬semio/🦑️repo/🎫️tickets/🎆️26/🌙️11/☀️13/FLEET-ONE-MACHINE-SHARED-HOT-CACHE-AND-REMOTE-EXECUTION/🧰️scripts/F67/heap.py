import os,tempfile,sys
B=sys.argv[1]
def edit(p,pairs):
    s=open(p).read()
    for a,b in pairs:
        assert s.count(a)==1,(p,a[:50],s.count(a))
        s=s.replace(a,b)
    fd,t=tempfile.mkstemp(dir=os.path.dirname(p),prefix=".f67.");os.write(fd,s.encode());os.close(fd);os.replace(t,p)
edit(B+"/🔣️.json",[('"typeRoots": "node_modules/@types"\n','"typeRoots": "node_modules/@types",\n    "heap": {"maximumMegabytes": 12000, "minimumMegabytes": 4096, "hostShare": 0.6}\n')])
edit(B+"/🧬️schema/🔣️.json",[('"required": ["script", "typeRoots"],\n          "properties": {"script": {"type": "string", "minLength": 1}, "typeRoots": {"type": "string", "minLength": 1}}',
 '"required": ["script", "typeRoots", "heap"],\n          "properties": {\n            "script": {"type": "string", "minLength": 1},\n            "typeRoots": {"type": "string", "minLength": 1},\n            "heap": {\n              "type": "object",\n              "additionalProperties": false,\n              "required": ["maximumMegabytes", "minimumMegabytes", "hostShare"],\n              "properties": {"maximumMegabytes": {"type": "integer", "minimum": 512}, "minimumMegabytes": {"type": "integer", "minimum": 512}, "hostShare": {"type": "number", "exclusiveMinimum": 0, "maximum": 1}}\n            }\n          }')])
edit(B+"/🟦️.ts",[
 ('timeoutMs:number;signal?:AbortSignal}>)=>','timeoutMs:number;heapMegabytes:number;signal?:AbortSignal}>)=>'),
 ('/** ⚙️ Runs the compiler script under','/** 🧠️ The old-space heap the compiler may use in megabytes: the declared maximum, cut to the declared share of the host memory, never below the declared minimum.\n * @see ./🧬️schema/🔣️.json */\nexport function typecheckHeapMegabytesV1(limits:TypecheckPolicyV1=rules,hostMegabytes:number=Math.floor(totalmem()/1048576)):number{\n const heap=limits.compiler.heap;\n return Math.max(heap.minimumMegabytes,Math.min(heap.maximumMegabytes,Math.floor(hostMegabytes*heap.hostShare)));\n}\n\n/** ⚙️ Runs the compiler script under'),
 ('env:{...process.env,NO_COLOR:"1"}});\n let output=""','env:{...process.env,NO_COLOR:"1",NODE_OPTIONS:/--max-old-space-size/u.test(process.env.NODE_OPTIONS??"")?process.env.NODE_OPTIONS:((process.env.NODE_OPTIONS??"")+" --max-old-space-size="+request.heapMegabytes).trim()}});\n let output=""'),
 ('timeoutMs:limits.budgetMs,signal:ports.signal','timeoutMs:limits.budgetMs,heapMegabytes:typecheckHeapMegabytesV1(limits),signal:ports.signal'),
])
