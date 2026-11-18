"""R84: Bazel adapter data and tests moved with the production rules they encode: owner commands carry budget 0 and the issued phase as deadline, the captured loader renders machine tokens when its snapshot carries no catalog, the caller capabilities carry codeRoot, the executor scope declares the roots its runtime trace opens, and the scheduling policy declares the five ports keyed code reaches."""
import json,re
def exact(t,old,new,count=1):
    assert t.count(old)==count,(old[:90],t.count(old),count)
    return t.replace(old,new)
def environment_test(t):
    t=exact(t,'let captured:NodeJS.ProcessEnv|undefined,budget:number|undefined,signal:AbortSignal|undefined,expectedSignal:AbortSignal|undefined;','let captured:NodeJS.ProcessEnv|undefined,budget:number|undefined,remaining:number|null|undefined,signal:AbortSignal|undefined,expectedSignal:AbortSignal|undefined;')
    t=exact(t,'maximum:number,options:{env:NodeJS.ProcessEnv;signal:AbortSignal;onProgress:()=>Promise<void>})=>{expect(root).toBe(row.root);captured=options.env;budget=maximum;signal=options.signal;await options.onProgress();}','maximum:number,options:{env:NodeJS.ProcessEnv;signal:AbortSignal;deadline:import("@semio-tech/process/leases").ResourceLeaseControl;onProgress:()=>Promise<void>})=>{expect(root).toBe(row.root);captured=options.env;budget=maximum;signal=options.signal;remaining=(await import("@semio-tech/process/leases")).checkResourceLeaseControl(options.deadline);expect(options.deadline.signal).toBe(options.signal);await options.onProgress();}')
    t=exact(t,'expect(budget).toBeLessThanOrEqual(corpus.maximumChildMilliseconds);expect(budget).toBeGreaterThan(0);','expect(budget).toBe(0);expect(remaining).not.toBeNull();expect(remaining!).toBeLessThanOrEqual(corpus.maximumChildMilliseconds);expect(remaining!).toBeGreaterThan(0);')
    return t
def birth_test(t):
    t=exact(t,'budget:number,options:{env:NodeJS.ProcessEnv;signal:AbortSignal;onProgress:()=>Promise<void>})=>{count++;','budget:number,options:{env:NodeJS.ProcessEnv;signal:AbortSignal;deadline:Parameters<typeof checkResourceLeaseControl>[0];onProgress:()=>Promise<void>})=>{count++;')
    t=exact(t,'expect(budget).toBeGreaterThan(0);expect(budget).toBeLessThanOrEqual(120000);expect(options.signal.aborted).toBe(false);','expect(budget).toBe(0);expect(options.deadline.signal).toBe(options.signal);const left=checkResourceLeaseControl(options.deadline);expect(left).not.toBeNull();expect(left!).toBeGreaterThan(0);expect(left!).toBeLessThanOrEqual(120000);expect(options.signal.aborted).toBe(false);')
    return t
def birth_factory(t):
    return exact(t,'capabilities={tool:variant==="parity-nx"?"nx"as const:"bazel"as const,workspaceRoot:root,arguments:request','capabilities={tool:variant==="parity-nx"?"nx"as const:"bazel"as const,workspaceRoot:root,codeRoot:root,arguments:request')
def implementation_test(t):
    return exact(t,'.rejects.toThrow("Uncaptured implementation import refused");','.rejects.toThrow(\'import: {"specifier":"./foreign.ts"\');')
def sorted_insert(t,anchor_key,new_items,indent):
    i=t.index(anchor_key);j=t.index("[",i);k=t.index("]",j)
    body=t[j+1:k];items=[json.loads(x.strip().rstrip(",")) for x in body.strip().split("\n") if x.strip()]
    for n in new_items:
        if n not in items:items.append(n)
    items=sorted(items)
    text="[\n"+",\n".join(indent+json.dumps(x,ensure_ascii=False) for x in items)+"\n"+indent[:-2]+"]"
    return t[:j]+text+t[k+1:]
def scopes_data(t):
    d0=json.loads(t)
    new=sorted_insert(t,'"roots"',["🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🦀️execution/🟦️.ts","🧰️framework/🔨️modules/🧬️schema/📄️source/🔗️closure/🟦️.ts"],"        ")
    d1=json.loads(new);assert len(d1["scopes"][0]["roots"])==len(d0["scopes"][0]["roots"])+2
    d1["scopes"][0]["roots"]=sorted(d1["scopes"][0]["roots"]);return new
def scheduling_data(t):
    d=json.loads(t)
    def add(t,domain,port):
        i=t.index('"id": "%s"'%domain);j=t.index('"ports": [',i);k=t.index("]",j)
        body=t[j+len('"ports": ['):k]
        assert port not in body
        last=body.rstrip();lines=[x for x in body.split("\n") if x.strip()]
        indent=re.match(r'^(\s+)',lines[0]).group(1)
        return t[:j+len('"ports": [')]+last+",\n"+indent+json.dumps(port,ensure_ascii=False)+body[len(last):]+t[k:]
    t=add(t,"process-supervision","🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/📥️json/🟦️.ts")
    t=add(t,"progress-reporting","🧰️framework/🔨️modules/🏃️process/📖️texts/🟦️.ts")
    t=add(t,"progress-reporting","🧰️framework/🔨️modules/🏃️process/🏗️build/🗣️language/📖️catalog/🟦️.ts")
    t=add(t,"capacity","🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🎫️gate/🟦️.ts")
    d1=json.loads(t);return t
