import sys,os
R="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-execution/🛰️sidecar/🚦️ready/🟦️.ts"
s=open(R,encoding="utf8").read()
old1="input.mirror?.(record.stage,logTailV1(record.log,P.mirrorLines,P.mirrorLineCharacters),ownerOf(record.stage,record.startedMs));"
new1="input.mirror?.(record.stage,mirrorLinesV1(record.log),ownerOf(record.stage,record.startedMs));"
old2="mirror=()=>input.mirror?.(inflight.stage,logTailV1(inflight.log,P.mirrorLines,P.mirrorLineCharacters),owner);"
new2="mirror=()=>input.mirror?.(inflight.stage,mirrorLinesV1(inflight.log),owner);"
anchor="function logTailV1(log:string,count:number=P.tailLines,width:number=P.tailCharacters):string[]{"
fn='''/** 🪞️ The live mirror of a stage log: the last `mirrorLines` lines without the machine rows and heartbeat sentences of `failureNoise`, so a stage that waits for hours keeps its last real words in the mirror instead of a wall of `still running after N s`, followed by the newest raw line, which shows that the stage still lives. */
export function mirrorLinesV1(log:string):string[]{
 const raw=logTailV1(log,1,P.mirrorLineCharacters),quiet=failureLinesV1(log,P.mirrorLines-1,P.mirrorLineCharacters,[]),last=raw[0];
 return last===undefined||quiet.at(-1)===last?quiet:[...quiet,last];
}

'''
for o in (old1,old2,anchor):
    assert s.count(o)==1,o
s=s.replace(old1,new1).replace(old2,new2).replace(anchor,fn+anchor)
tmp=R+".h51.tmp";open(tmp,"w",encoding="utf8").write(s);os.replace(tmp,R)
print("ok")
