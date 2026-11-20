import sys
NB="🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build"
p=f"mirror/{NB}/🟦️.ts"; s=open(p).read()
def rep(old,new):
    global s
    assert s.count(old)==1,(old[:60],s.count(old)); s=s.replace(old,new)
rep('from"./🧮️phase-ledger/🟦️.ts";\n','from"./🧮️phase-ledger/🟦️.ts";\nimport{restoreWidthV1}from"./🧱️restore-width/🟦️.ts";\n')
rep('if(!environment.MAKEFLAGS&&!environment.CARGO_MAKEFLAGS)environment.CARGO_BUILD_JOBS=String(facts.parallelism);',
    'if(!environment.MAKEFLAGS&&!environment.CARGO_MAKEFLAGS){const width=restoreWidthV1(facts.parallelism,environment);environment.CARGO_BUILD_JOBS=String(width.jobs);if(width.factor>1)environment.SEMIO_UNIT_CACHE_SLOTS=String(width.slots);process.stderr.write(`[restore-width] jobs=${width.jobs} compile-slots=${width.slots} factor=${width.factor}\\n`);phaseLedgerV1().add({phase:"cargo:jobs",seconds:0,count:width.jobs,bytes:null,within:true});}')
open(p,"w").write(s)
