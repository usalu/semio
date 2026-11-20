import sys
t=open(sys.argv[1]).read()
def sub(t,a,b):
    assert t.count(a)==1,(a[:70],t.count(a));return t.replace(a,b)
t=sub(t,',unitsBefore=readBuildCountersV1(this.root).unit;',',unitsBefore=readBuildCountersV1(this.root).unit,runStartedMs=Date.now();')
t=sub(t,'for(const line of unitLedgerLinesV1(unitLedgerV1(unitsBefore,readBuildCountersV1(this.root).unit)))process.stderr.write(line+"\\n");}catch{}','for(const line of unitLedgerLinesV1(unitLedgerV1(unitsBefore,readBuildCountersV1(this.root).unit)))process.stderr.write(line+"\\n");const{phaseLedgerRunLinesV1}=await import("../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🧮️phase-ledger/🟦️.ts");for(const line of phaseLedgerRunLinesV1(this.root,runStartedMs,(Date.now()-runStartedMs)/1000))process.stderr.write(line+"\\n");}catch{}')
open(sys.argv[2],'w').write(t);print("ok")
