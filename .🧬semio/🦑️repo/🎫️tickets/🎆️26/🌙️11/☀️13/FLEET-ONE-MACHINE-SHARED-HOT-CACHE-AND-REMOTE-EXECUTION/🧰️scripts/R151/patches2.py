from lib import *
def patch_load2(t):
    t=once(t,'process.stderr.write("[graph-retry] "+JSON.stringify(diagnostic)+"\\n");}return undefined;};','process.stderr.write("[graph-retry] "+JSON.stringify(diagnostic)+"\\n");}else process.stderr.write("[graph-retry] "+JSON.stringify({version:1,reason,stale:((details.stale as {path:string}[]|undefined)??[]).map(row=>row.path)})+"\\n");return undefined;};',"retry line")
    t=once(t,"    const repairStarted=performance.now();stale.length=0;\n",'    const repairStarted=performance.now();process.stderr.write("[graph-repair] "+JSON.stringify({round:repair+1,stale:stale.map(row=>row.path)})+"\\n");stale.length=0;\n',"repair line")
    return t
TARGETS2={LOAD:patch_load2}
