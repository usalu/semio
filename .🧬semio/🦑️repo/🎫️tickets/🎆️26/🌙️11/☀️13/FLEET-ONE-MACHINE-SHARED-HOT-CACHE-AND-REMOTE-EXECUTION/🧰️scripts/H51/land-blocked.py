import os
B="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🤝️shared-worktree/"
def edit(path,pairs):
    s=open(B+path,encoding="utf8").read()
    for o,n in pairs:
        assert s.count(o)==1,(path,o[:60])
        s=s.replace(o,n)
    tmp=B+path+".h51.tmp";open(tmp,"w",encoding="utf8").write(s);os.replace(tmp,B+path)
edit("🔁️replica/🛟️supervisor/🟦️.ts",[
('earlier=readJson(restartFile(root)) as {count?:unknown;wallMs?:unknown}|undefined;','earlier=readJson(restartFile(root)) as {count?:unknown;wallMs?:unknown;activity?:unknown;longestActivity?:unknown}|undefined;'),
('holder[Symbol.for("semio.fleet.blockedRestart")]={count:earlier.count,lastMs:earlier.wallMs};','{const where=earlier.longestActivity??earlier.activity;holder[Symbol.for("semio.fleet.blockedRestart")]={count:earlier.count,lastMs:earlier.wallMs,...typeof where==="string"&&/^[a-z]{1,12}$/u.test(where)?{activity:where}:{}};}'),
])
edit("🛡️resilience/🟦️.ts",[
('const holder=globalThis as unknown as Record<symbol,{count:number;lastMs:number}|undefined>,found=holder[Symbol.for("semio.fleet.blocked")],ended=holder[restartSymbol];','const holder=globalThis as unknown as Record<symbol,{count:number;lastMs:number;activity?:string}|undefined>,found=holder[Symbol.for("semio.fleet.blocked")],ended=holder[restartSymbol];'),
('...ended===undefined||ended.count===0?[]:[{kind:"loop-blocked-restart",count:ended.count,lastMs:ended.lastMs}]];','...ended===undefined||ended.count===0?[]:[{kind:"loop-blocked-restart",count:ended.count,lastMs:ended.lastMs},...ended.activity===undefined?[]:[{kind:"loop-blocked-in-"+ended.activity,count:ended.count,lastMs:ended.lastMs}]]];'),
('/** 🧱️ The blocks of the event loop that this process recorded (`kind` `loop-blocked`, how often and when last: the boot stage counts every wake-up that came `blockMs` or more too late) and the ends that the boot stage made of earlier daemons of this machine (`kind` `loop-blocked-restart`, how often and when last). Empty when none happened. */','/** 🧱️ The blocks of the event loop that this process recorded (`kind` `loop-blocked`, how often and when last: the boot stage counts every wake-up that came `blockMs` or more too late) and the ends that the boot stage made of earlier daemons of this machine (`kind` `loop-blocked-restart`, how often and when last, and `loop-blocked-in-<activity>` for the activity the last blocked daemon sat in, so the fleet report names what blocks a machine without reading its disk). Empty when none happened. */'),
])
edit("🛡️resilience/🧪️tests/🟦️.ts",[
(' try{expect(blockedFailuresV1()).toEqual([{kind:"loop-blocked-restart",count:3,lastMs:42}]);}finally{delete holder[key];}\n',' try{expect(blockedFailuresV1()).toEqual([{kind:"loop-blocked-restart",count:3,lastMs:42}]);}finally{delete holder[key];}\n holder[key]={count:91,lastMs:7,activity:"walk"};\n try{expect(blockedFailuresV1()).toEqual([{kind:"loop-blocked-restart",count:91,lastMs:7},{kind:"loop-blocked-in-walk",count:91,lastMs:7}]);expect(blockedFailuresV1().every(row=>/^[a-z][a-z0-9-]{0,39}$/u.test(row.kind))).toBe(true);}finally{delete holder[key];}\n'),
])
print("ok")
