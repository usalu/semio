"""Batch `unitcache`: the rust unit cache holds its ledger budget. Three defects found on the live image (u = 26.6 GiB against a 24 GiB budget, rust 84 % used):
1. the watching collector only collected above the high-water mark (87.5 %), so between the low (75 %) and the high mark nobody ran the share-bounded maintenance (a live watcher polled the image all day without collecting);
2. the admission memos (u/a, 230 000 files, 2.4 GiB) had no retention or share at all, so the unit cache could never fit its 24 GiB: objects (o) 13.5 GiB of a 11.95 GiB share and lanes (l) 10.7 GiB of a 9.56 GiB share were over because of 1, memos were unbounded.
The shares themselves (objects 25 %, lanes 20 %, memos 3 %; the incremental 10 % lies inside the lanes) fit: 48 % of the 48 GiB image = 23.0 GiB <= 24 GiB; a law pins it.
Apply AFTER jobsrv-g (its whole-file base of the unit-cache schema); the schema edit is one anchored line."""
L = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
U = f"{L}/⚡️caching/🦀️cargo/🗃️unit-cache"

SCHEMA = f"{U}/🧬️schema/🔣️.json"
GC = f"{U}/🧹️gc/🟦️.ts"
GC_TEST = f"{U}/🧹️gc/🧪️tests/🟦️.ts"

EDITS = [
    (SCHEMA, [(
        '"casShare": 0.25, "casProtectedShare": 0.1, "laneShare": 0.2, "incrementalShare": 0.1,',
        '"casShare": 0.25, "casProtectedShare": 0.1, "laneShare": 0.2, "incrementalShare": 0.1, "admissionShare": 0.03, "admissionRetentionMilliseconds": 604800000, "admissionTemporaryMilliseconds": 3600000,',
    )]),
    (GC, [
        ('import {recordFillSampleV1} from "./📈️fill/🟦️.ts";',
         'import {recordFillSampleV1} from "./📈️fill/🟦️.ts";\nimport {pruneAdmissionsV1} from "./📖️admissions/🟦️.ts";'),
        ('  return{running:false,plan,applied};',
         '  await pruneAdmissionsV1(root,nowMs,{retentionMs:policy.admissionRetentionMilliseconds,temporaryMs:policy.admissionTemporaryMilliseconds,maximumBytes:Math.floor(capacityBytes*policy.admissionShare)},options.signal);\n  return{running:false,plan,applied};'),
        (' let rounds=0,collections=0,freedBytes=0,idle=0,quietSince:number|null=null,scanned=-Infinity;',
         ' let rounds=0,collections=0,freedBytes=0,idle=0,quietSince:number|null=null,scanned=-Infinity,maintained=-Infinity;'),
        ('between floor and target it only watches',
         'between floor and target it maintains (the share-bounded collection) once at its start and then every maintenance interval'),
        ('   }else{idle=0;quietSince=null;}',
         '   }else{\n    idle=0;quietSince=null;\n    if(at-maintained>=policy.maintenanceIntervalMilliseconds){maintained=at;await collect(root,capacityBytes,{nowMs:at,signal:options.signal,report:options.report});collections++;}\n   }'),
    ]),
    (GC_TEST, [
        ('free=9*gib;const alert=await watchUnitCachePressureV1(root,volume,{now:time.now,sleep:time.sleep,measure,collect});',
         'free=9*gib;const alert=await watchUnitCachePressureV1(root,volume,{now:time.now,sleep:time.sleep,measure,collect:(async()=>({running:false as const,plan:{removeLanes:[],removeIncremental:[],removeObjects:[]},applied:{lanes:0,incremental:0,objects:0,sessions:0,units:0}})) as never});'),
        ('expect(alert).toMatchObject({state:"done",reason:"expired",collections:0});',
         'expect(alert).toMatchObject({state:"done",reason:"expired",collections:Math.floor(rules.watchMaximumMilliseconds/rules.maintenanceIntervalMilliseconds)+1});'),
    ]),
]

NEW = [
    (f"{U}/🧹️gc/📖️admissions/🟦️.ts", "admissions.ts"),
    (f"{U}/🧹️gc/📖️admissions/🧪️tests/🟦️.ts", "admissions-test.ts"),
    (f"{U}/🧹️gc/🧪️tests/🎯️budget/🟦️.ts", "budget-law.ts"),
]
