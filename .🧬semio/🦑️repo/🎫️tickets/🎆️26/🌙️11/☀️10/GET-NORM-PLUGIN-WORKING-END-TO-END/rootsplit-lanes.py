import re
V="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/✅️verify/🟦️.ts"
src=open(V,encoding='utf8').read().split('\n')
starts=[i for i,l in enumerate(src) if re.match(r'    if \(segments\[0\] === "',l)]
starts.append(len(src))
ids=['runCmd','runCargo','startToolV1','toolExitV1','runtimeExecutableV1','runtimeParseTomlV1','readFileSync','refuseVerifyV1','sayVerifyV1','parseViewportGraphqlSchemasV1','orchestratorBudgetOpts','semioBuildParallelFlag','abstractionOwnershipChecks']
def lanes():
    out={}
    for a,b in zip(starts,starts[1:]):
        n=re.search(r'"([^"]+)"',src[a]).group(1)
        e=b
        if b==len(src):
            e=a+1
            while not re.match(r'^  }',src[e]) : e+=1
        out.setdefault(n,[]).append((a,e,'\n'.join(src[a:e])))
    return out
if __name__=='__main__':
    for n,l in lanes().items():
        for a,e,blk in l:
            used=[i for i in ids if re.search(r'\b'+i+r'\b',blk)]
            print(n,a+1,e-a,used,set(re.findall(r'this\.\w+',blk))-{'this.root'})
