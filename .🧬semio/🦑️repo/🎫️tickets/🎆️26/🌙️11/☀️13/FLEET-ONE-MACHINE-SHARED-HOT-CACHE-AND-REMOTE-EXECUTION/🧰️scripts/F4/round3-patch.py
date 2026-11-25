import os
base="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-execution/🛰️sidecar/🚦️ready/"
def sub(p,old,new):
    s=open(base+p).read(); assert s.count(old)==1,(p,old[:60],s.count(old)); open(base+p,'w').write(s.replace(old,new))
sub('🟦️.ts','grownInodes=upper("growthInodes"),needInodes=grownInodes===null?inodes:','grownInodes=upper("growthInodes"),needInodes=grownInodes===null?derived:')
sub('🟦️.ts','const need=readyDiskNeedV1(input.stage,input.runs,facts.totalBytes,facts.totalInodes);','const need=readyDiskNeedV1(input.stage,input.runs,facts.totalBytes,facts.totalInodes,input.derivedInodes);')
sub('🟦️.ts','facts(path:string):ReadyFilesystemFactsV1|null;runs?:readonly ReadyTimingV1[]}>):ReadyDiskShortV1|null{','facts(path:string):ReadyFilesystemFactsV1|null;runs?:readonly ReadyTimingV1[];derivedInodes?:number|null}>):ReadyDiskShortV1|null{')
sub('🟦️.ts','/** 💽️ Whether the paths a stage writes to hold what it needs','''/** 🧮️ The inodes a stage creates before it has ever been measured, derived from its own input: the dependency generations hold about `entriesPerLockfileLine` entries per line of the lockfile of each scope (the root and every top-level folder with a `bun.lock`; calibrated on the six real generations of this repository, 3.20 to 3.52, rounded up), so the cold need is the sum of the lockfile lines times that factor. `null` for any other stage or when no lockfile is readable. A pure function of the workspace folder.
 * @see ./🧪️tests/💽️disk-recovery/🟦️.ts */
export function readyDerivedInodesV1(stage:string,root:string):number|null{
 const rule=P.diskDerivation;
 if(stage!==rule.stage)return null;
 let lines=0;
 const count=(path:string):void=>{try{const bytes=readFileSync(path);let at=-1;while((at=bytes.indexOf(10,at+1))!==-1)lines++;}catch{}};
 count(join(root,rule.lockfile));
 try{for(const entry of readdirSync(root,{withFileTypes:true}))if(entry.isDirectory()&&!entry.name.startsWith(".")&&entry.name!=="node_modules")count(join(root,entry.name,rule.lockfile));}catch{}
 return lines===0?null:Math.ceil(lines*rule.entriesPerLockfileLine);
}

/** 💽️ Whether the paths a stage writes to hold what it needs''')
sub('🟦️.ts','facts:input.facts??readyFilesystemFactsV1,runs:recentRuns()}),','facts:input.facts??readyFilesystemFactsV1,runs:recentRuns(),derivedInodes:readyDerivedInodesV1(stage,input.root)}),')
sub('🟦️.ts','mkdirSync,openSync,readFileSync,readSync,','mkdirSync,openSync,readFileSync,readSync,readdirSync,')
