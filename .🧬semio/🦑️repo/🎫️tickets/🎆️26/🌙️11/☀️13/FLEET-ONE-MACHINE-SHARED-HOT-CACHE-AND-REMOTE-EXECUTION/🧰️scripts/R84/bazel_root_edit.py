"""R84: the owner command law of the Bazel adapter: every owner command passes budget 0 and the issued phase control as its deadline (one control, one clock); the old rule derived a number from the phase."""
def edit(t):
    def one(old,new,count=1):
        nonlocal t
        assert t.count(old)==count,(old[:80],t.count(old))
        t=t.replace(old,new)
    one('expect(body).toContain("compilerAdmissionBudgetV1(phase)");','expect(body).toContain("deadline:phase");')
    one('assert(!argumentSources.has(owner));argumentSources.set(owner,budget.getText(tree));','assert(!argumentSources.has(owner));assert(/deadline:phaseControl/u.test(child.arguments[5]?.getText(tree)??""),owner);argumentSources.set(owner,budget.getText(tree));')
    i=t.index('for(const[owner,expression]of argumentSources){const argument=new Function(');j=t.index('assert.equal(argumentOracle.n,row.effects);}',i)+len('assert.equal(argumentOracle.n,row.effects);}')
    t=t[:i]+'for(const[owner,expression]of argumentSources)assert.equal(expression,"0",owner);'+t[j:]
    one('runOwnedCommand:async(_program:string,_args:readonly string[],_cwd:string,_label:string,budget:number,options:{env:unknown;signal:AbortSignal})=>{assert.equal(typeof budget,"number");assert(Number.isSafeInteger(budget)&&budget>=0);assert.equal(options.env,environment);assert.equal(options.signal.aborted,false);routing.requireIssuedResourceControlInvocationV1(actual,original);calls.push(budget);db.query("insert into calls values(?,?,?)").run(row.kind,calls.length,budget);if(row.kind==="root0")assert.equal(budget,0);else assert(budget>0&&budget<=1000);',
        'runOwnedCommand:async(_program:string,_args:readonly string[],_cwd:string,_label:string,budget:number,options:{env:unknown;signal:AbortSignal;deadline:typeof actual})=>{assert.equal(budget,0);assert.equal(options.deadline,actual);assert.equal(options.env,environment);assert.equal(options.signal.aborted,false);routing.requireIssuedResourceControlInvocationV1(actual,original);const left=leases.checkResourceLeaseControl(options.deadline),value=left===null?0:Math.ceil(left);calls.push(value);db.query("insert into calls values(?,?,?)").run(row.kind,calls.length,value);if(row.kind==="root0")assert.equal(value,0);else assert(value>0&&value<=1000);')
    one('/** ⏱️ Executes the actual owner initializer against real issuance and a command-observation-only port. */','/** ⏱️ Executes the actual owner initializer against real issuance and a command-observation-only port: every owner command passes budget 0 and the issued phase as its deadline, and the remaining time of that deadline is what drains. */')
    return t
