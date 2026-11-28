import os
R="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-execution/🛰️sidecar/🚦️ready/"
p=R+"🌲️process-tree/🟦️.ts"
s=open(p,encoding="utf8").read()
if "readyWaitedMillisecondsV1" not in s:
    a="/** 🕰️ Whether a stage that runs older code than the code now running"
    assert s.count(a)==1
    s=s.replace(a,'''/** ⏱️ How long the wait line says the stage has been waiting (`[capacity] waiting 15549s for command: ...`), in milliseconds; `null` for any other line. A supervisor that adopts a stage hours into its wait learns the age of the wait from the line itself instead of starting its own clock, so a worker whose supervisor is restarted by every landing still reaches the stale-waiting verdict. A pure function.
 * @see ./🧪️tests/🕰️stale-waiting */
export function readyWaitedMillisecondsV1(line:string):number|null{const found=/^\\[(?:capacity\\] waiting|Kapazität\\] wartet) (\\d{1,9}) ?s\\b/u.exec(line);return found===null?null:Number(found[1])*1000;}

'''+a,1)
    open(p,"w",encoding="utf8").write(s)
p=R+"🟦️.ts"
s=open(p,encoding="utf8").read()
if "readyWaitedMillisecondsV1" not in s:
    s=s.replace("import{readyCodeStopV1,","import{readyWaitedMillisecondsV1,readyCodeStopV1,",1)
    b="waitingNow=readyWaitLineV1(recent.at(-1)??\"\",treePolicy);"
    assert s.count(b)==1
    s=s.replace(b,b+"if(waitingNow&&!waitedSeeded){waitedSeeded=true;const waited=readyWaitedMillisecondsV1(recent.at(-1)??\"\");if(waited!==null)workedAt=Math.min(workedAt,Date.now()-waited);}",1)
    c="let staleWaitingCounter=-1;"
    assert s.count(c)==1
    s=s.replace(c,c+"let waitedSeeded=false;",1)
    open(p,"w",encoding="utf8").write(s)
p=R+"🧪️tests/🕰️stale-waiting/🟦️.ts"
s=open(p,encoding="utf8").read()
if "readyWaitedMillisecondsV1" not in s:
    s=s.replace("readyStaleWaitingV1,readyWaitLineV1}","readyStaleWaitingV1,readyWaitLineV1,readyWaitedMillisecondsV1}",1)
    s+='''
test("the age of a wait is read from its own line in both languages, nothing else gives one",()=>{
 expect(readyWaitedMillisecondsV1("[capacity] waiting 15549s for command: an earlier request is still waiting (need 2048 MB)")).toBe(15_549_000);
 expect(readyWaitedMillisecondsV1("[Kapazität] wartet 120s für command: x")).toBe(120_000);
 for(const line of["[capacity] waiting for nothing","built unit 3 of 99","","[capacity] admitted command: 1 job(s)"])expect(readyWaitedMillisecondsV1(line),line).toBeNull();
});
'''
    open(p,"w",encoding="utf8").write(s)
