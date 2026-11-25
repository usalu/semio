"""F34 stage: one shared build-holder predicate; a profile that compilers hold loses no unit. Dry run by default; --apply writes (anchored, idempotent). Run from /Users/ueli/Documents/semio. Optional --root <dir> applies to a scratch copy of the gc member."""
import json, os, subprocess, sys, tempfile

HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = "/Users/ueli/Documents/semio"
GC = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/🧹️gc"
apply = "--apply" in sys.argv
if "--root" in sys.argv:
    ROOT = sys.argv[sys.argv.index("--root") + 1]
U = f"{ROOT}/{GC}/🧱️units"
H = f"{ROOT}/{GC}/🫀️holder"
problems = []


def read(path):
    return open(path, encoding="utf8").read()


def write(path, text):
    if apply:
        os.makedirs(os.path.dirname(path), exist_ok=True)
        tmp = path + ".f34tmp"
        open(tmp, "w", encoding="utf8").write(text)
        os.replace(tmp, path)


def edit(path, pairs):
    text = read(path)
    for old, new in pairs:
        if new in text and old not in text:
            continue
        if text.count(old) != 1:
            problems.append(f"{path}: anchor count {text.count(old)} for {old[:70]!r}")
            continue
        text = text.replace(old, new)
    write(path, text)
    print("edit", path.replace(ROOT + "/", "")[-60:])


def create(path, source):
    if os.path.exists(path) and read(path) == read(source):
        return
    if os.path.exists(path):
        problems.append(f"{path}: exists and differs")
        return
    write(path, read(source))
    print("create", path.replace(ROOT + "/", "")[-60:])


create(f"{H}/🟦️.ts", f"{HERE}/holder.ts")
create(f"{H}/🧪️tests/🟦️.ts", f"{HERE}/holder-test.ts")
create(f"{H}/🧪️tests/🐍️.py", f"{HERE}/holder-oracle.py")

edit(f"{U}/🟦️.ts", [
 ('import {tryLockSessionV1,type IncrementalLockPortV1} from "../🌱️incremental/🟦️.ts";\n',
  'import {tryLockSessionV1,type IncrementalLockPortV1} from "../🌱️incremental/🟦️.ts";\nimport {buildHolderLiveV1} from "../🫀️holder/🟦️.ts";\n'),
 ("/** 🧱️ One Cargo build unit (`build/<crate>/<hash>`): its allocated bytes, last use, whether a running compiler holds its lock and whether a running compiler holds its profile. */\nexport type BuildUnitV1=Readonly<{path:string;crate:string;profile:string;bytes:number;lastUsedMs:number;held:boolean;busy:boolean}>;",
  "/** 🧱️ One Cargo build unit (`build/<crate>/<hash>`): its allocated bytes, last use and whether it has a live holder (`buildHolderLiveV1`: a running compiler holds its lock, its profile's build lease or its lane). */\nexport type BuildUnitV1=Readonly<{path:string;crate:string;profile:string;bytes:number;lastUsedMs:number;held:boolean}>;"),
 ("/** 🛡️ Whether a unit may go: never one a compiler holds; a unit of a profile that compilers hold only when it is not one of the newest variants and idle for the busy minimum, any other unit once it is older than the minimum age. */\nexport function unitEligibleV1(unit:BuildUnitV1,newest:ReadonlySet<string>,nowMs:number,rules:Pick<typeof policy,\"minimumAgeMilliseconds\"|\"busyMinimumAgeMilliseconds\">=policy):boolean{\n if(unit.held)return false;\n return unit.busy?!newest.has(unit.path)&&nowMs-unit.lastUsedMs>=rules.busyMinimumAgeMilliseconds:nowMs-unit.lastUsedMs>=rules.minimumAgeMilliseconds;\n}",
  "/** 🛡️ Whether a unit may go: never one with a live holder (a unit of a profile that compilers hold is held, however idle), any other unit once it is older than the minimum age. */\nexport function unitEligibleV1(unit:BuildUnitV1,nowMs:number,rules:Pick<typeof policy,\"minimumAgeMilliseconds\">=policy):boolean{\n return !unit.held&&nowMs-unit.lastUsedMs>=rules.minimumAgeMilliseconds;\n}"),
 ("const eligible=(unit:BuildUnitV1)=>unitEligibleV1(unit,newest,input.nowMs,rules),take=",
  "const eligible=(unit:BuildUnitV1)=>unitEligibleV1(unit,input.nowMs,rules),take="),
 ("/** 🔭️ Measures every unit below the profiles: allocated bytes, the last time anything in it changed (never an access time, which measuring itself would renew) and whether a running compiler holds its lock or a live build holds its lane. Without a lock backend a unit counts as held until the busy minimum age has passed. */",
  "/** 🔭️ Measures every unit below the profiles: allocated bytes, the last time anything in it changed (never an access time, which measuring itself would renew) and whether it has a live holder (`buildHolderLiveV1`): a running compiler holds its lock, compilers hold the build lease of its profile or a live build holds its lane. Without a lock backend a unit counts as held until the held grace has passed. */"),
 ("held:live||probe===\"held\"||probe===\"unsupported\"&&options.nowMs-used<policy.busyMinimumAgeMilliseconds,busy});",
  "held:buildHolderLiveV1({lane:live,lease:busy,lock:typeof probe===\"object\"?\"free\":probe,idleMilliseconds:options.nowMs-used},policy.heldGraceMilliseconds)});"),
 ("/** 🧹️ Measures, plans and (unless `dryRun`) removes stale build units. A profile whose lane has a live holder is never touched, whatever its units' age or the budget. Profiles are probed first: a profile whose build lease is free is held exclusively while its units go (compilers that start meanwhile queue), a profile compilers hold only loses superseded units idle for the busy minimum; every removal re-takes the unit's own lock and holds it while the unit goes.",
  "/** 🧹️ Measures, plans and (unless `dryRun`) removes stale build units. A unit with a live holder is never touched, whatever its age or the budget: not a unit of a lane with a live holder, not any unit of a profile whose build lease compilers hold (Cargo reuses a unit without locking it, so a lock alone does not cover the units a running build has planned), not a unit whose compiler lock is held. Profiles are probed first: a profile whose build lease is free is held exclusively while its units go (compilers that start meanwhile queue); every removal re-takes the unit's own lock and holds it while the unit goes."),
 ("crates=new Set<string>(),newest=newestUnitsV1(units),byPath=new Map(units.map(unit=>[unit.path,unit])),profileByDirectory=",
  "crates=new Set<string>(),profileByDirectory="),
 ("say(\"busyProfile\",{profile:profile.profile,directory:profile.buildDirectory,hours:Math.round(policy.busyMinimumAgeMilliseconds/3600000)});",
  "say(\"busyProfile\",{profile:profile.profile,directory:profile.buildDirectory});"),
 ("   const held_=await leases(profile),exclusive=held_!==\"busy\";\n   try{",
  "   const held_=await leases(profile);if(held_===\"busy\")continue;\n   try{"),
 ("     const unit=byPath.get(row.path)!;if(!exclusive&&!unitEligibleV1({...unit,busy:true},newest,nowMs))continue;\n", ""),
 ("   }finally{if(held_!==\"busy\")held_.release();}", "   }finally{held_.release();}"),
])

schema_path = f"{U}/🧬️schema/🔣️.json"
schema = json.loads(read(schema_path))
policy = schema["definitions"]["Policy"]["const"]
if "busyMinimumAgeMilliseconds" in policy:
    policy = {("heldGraceMilliseconds" if key == "busyMinimumAgeMilliseconds" else key): value for key, value in policy.items()}
    schema["definitions"]["Policy"]["const"] = policy
unit = schema["definitions"]["Unit"]
if "busy" in unit["required"]:
    unit["required"].remove("busy")
    del unit["properties"]["busy"]
messages = schema["definitions"]["Messages"]["const"]
messages["busyProfile"] = {
    "en": "[repo cache] profile {profile} of {directory} is held by running compilers: none of its units goes",
    "de": "[repo cache] Profil {profile} von {directory} wird von laufenden Compilern gehalten: keine seiner Einheiten geht",
}
new_schema_text = json.dumps(schema, indent=2, ensure_ascii=False) + "\n"
if new_schema_text != read(schema_path):
    write(schema_path, new_schema_text)
    print("edit schema")

scratch = tempfile.mkdtemp(prefix="f34-")
schema_file = os.path.join(scratch, "schema.json")
open(schema_file, "w", encoding="utf8").write(new_schema_text)

edit(f"{U}/🧪️tests/🐍️.py", [
 ('        age = now - unit["lastUsedMs"]\n        if unit["busy"]:\n            return unit["path"] not in newest and age >= policy["busyMinimumAgeMilliseconds"]\n        return age >= policy["minimumAgeMilliseconds"]\n',
  '        return now - unit["lastUsedMs"] >= policy["minimumAgeMilliseconds"]\n'),
])
oracle = f"{U}/🧪️tests/🐍️.py"
oracle_source = read(oracle) if os.path.exists(oracle) else ""

fixture_path = f"{U}/🧪️tests/🧫️fixtures/🔣️.json"
fixture = json.loads(read(fixture_path))
renames = {
 "a busy profile loses only superseded idle units, never its newest variants": "a profile that compilers hold loses no unit, superseded and idle or not",
 "a busy profile keeps units idle for less than the busy minimum": "a profile that compilers hold keeps units that are recent",
 "a busy profile gives pressure units idle for the busy minimum that are not among the newest": "a profile that compilers hold gives no unit to pressure however long it is idle",
}
for vector in fixture["vectors"]:
    vector["name"] = renames.get(vector["name"], vector["name"])
    for row in vector["input"]["units"]:
        if row.get("busy"):
            row["held"] = True
        row.pop("busy", None)
if apply or True:
    states_file = os.path.join(scratch, "states.json")
    patched_oracle = os.path.join(scratch, "oracle.py")
    source = oracle_source if oracle_source else read(oracle)
    if '"busy"' in source:
        source = source.replace('        age = now - unit["lastUsedMs"]\n        if unit["busy"]:\n            return unit["path"] not in newest and age >= policy["busyMinimumAgeMilliseconds"]\n        return age >= policy["minimumAgeMilliseconds"]\n', '        return now - unit["lastUsedMs"] >= policy["minimumAgeMilliseconds"]\n')
    open(patched_oracle, "w", encoding="utf8").write(source)
    run = subprocess.run(["python3", "-I", patched_oracle, schema_file], input=json.dumps([v["input"] for v in fixture["vectors"]]).encode(), capture_output=True)
    if run.returncode != 0:
        problems.append("oracle failed: " + run.stderr.decode()[:300])
    else:
        for vector, removal in zip(fixture["vectors"], json.loads(run.stdout)):
            vector["remove"] = removal
new_fixture_text = json.dumps(fixture, indent=2, ensure_ascii=False) + "\n"
if new_fixture_text != read(fixture_path):
    write(fixture_path, new_fixture_text)
    print("edit fixture")

edit(f"{U}/🧪️tests/🟦️.ts", [
 ("for(const crate of[\"a\",\"b\",\"c\",\"d\"].filter(()=>next(3)>0)){const busy=next(2)===0;for(let variant=0;variant<next(7);variant++)units.push({path:`/s/b/p/${profile}/build/${crate}/${variant.toString(16).padStart(16,\"0\")}`,crate,profile,bytes:next(500)*1048576,lastUsedMs:now-next(60*60)*60000,held:next(8)===0,busy:profile===\"debug\"?busy:!busy});}",
  "for(const crate of[\"a\",\"b\",\"c\",\"d\"].filter(()=>next(3)>0)){for(let variant=0;variant<next(7);variant++)units.push({path:`/s/b/p/${profile}/build/${crate}/${variant.toString(16).padStart(16,\"0\")}`,crate,profile,bytes:next(500)*1048576,lastUsedMs:now-next(60*60)*60000,held:next(8)===0});}"),
 ("test(\"the policy keeps what a live build needs: a busy profile never loses its newest variants and a unit younger than the minimum age never goes\",()=>{\n expect(policy.busyMinimumAgeMilliseconds).toBeGreaterThan(policy.minimumAgeMilliseconds);expect(policy.idleMilliseconds).toBeGreaterThan(policy.busyMinimumAgeMilliseconds);expect(policy.keepNewestPerCrate).toBeGreaterThanOrEqual(2);\n const now=500*hour,units:BuildUnitV1[]=[0,1,2,3].map(index=>({path:`/s/b/p/prototype/build/x/${index}`,crate:\"x\",profile:\"prototype\",bytes:1,lastUsedMs:now-(index===0?policy.minimumAgeMilliseconds/2:policy.busyMinimumAgeMilliseconds*(index+1)),held:false,busy:true}));\n expect(planUnitsV1({needBytes:1e12,nowMs:now,units}).map(row=>row.path)).toEqual([\"/s/b/p/prototype/build/x/3\",\"/s/b/p/prototype/build/x/2\"]);\n expect(planUnitsV1({needBytes:1e12,nowMs:now,units:units.map(unit=>({...unit,busy:false}))}).map(row=>row.path)).toEqual([\"/s/b/p/prototype/build/x/3\",\"/s/b/p/prototype/build/x/2\",\"/s/b/p/prototype/build/x/1\"]);\n});",
  "test(\"the policy keeps what a live build needs: a held unit never goes however old and a unit younger than the minimum age never goes\",()=>{\n expect(policy.heldGraceMilliseconds).toBeGreaterThan(policy.minimumAgeMilliseconds);expect(policy.idleMilliseconds).toBeGreaterThan(policy.heldGraceMilliseconds);expect(policy.keepNewestPerCrate).toBeGreaterThanOrEqual(2);\n const now=500*hour,units:BuildUnitV1[]=[0,1,2,3].map(index=>({path:`/s/b/p/prototype/build/x/${index}`,crate:\"x\",profile:\"prototype\",bytes:1,lastUsedMs:now-(index===0?policy.minimumAgeMilliseconds/2:policy.heldGraceMilliseconds*(index+1)),held:false}));\n expect(planUnitsV1({needBytes:1e12,nowMs:now,units}).map(row=>row.path)).toEqual([\"/s/b/p/prototype/build/x/3\",\"/s/b/p/prototype/build/x/2\",\"/s/b/p/prototype/build/x/1\"]);\n expect(planUnitsV1({needBytes:1e12,nowMs:now,units:units.map(unit=>({...unit,held:true}))})).toEqual([]);\n});"),
 ("expect(rows.every(row=>!row.held&&!row.busy)).toBe(true);\n  const unsupported=await measureUnitsV1(unitProfilesV1(store),{nowMs:now,busy:()=>true,lock:()=>\"unsupported\"});expect(unsupported.map(row=>row.held)).toEqual([false,false,true]);expect(unsupported.every(row=>row.busy)).toBe(true);",
  "expect(rows.every(row=>!row.held)).toBe(true);\n  const unsupported=await measureUnitsV1(unitProfilesV1(store),{nowMs:now,busy:()=>false,lock:()=>\"unsupported\"});expect(unsupported.map(row=>row.held)).toEqual([false,false,true]);\n  const leased=await measureUnitsV1(unitProfilesV1(store),{nowMs:now,busy:()=>true,lock:()=>\"missing\"});expect(leased.every(row=>row.held),\"a profile compilers hold holds every unit of it\").toBe(true);"),
 ("test(\"a profile that compilers hold loses only superseded units idle for the busy minimum, and pressure stops once the image has its free space\",async()=>{",
  "test(\"a profile that compilers hold loses no unit however idle or superseded, and pressure stops once the image has its free space\",async()=>{"),
 ("  expect(busy.busyProfiles).toBe(1);expect(busy.removed.map(row=>row.path)).toEqual([old,mid]);expect([fresh,newest].every(existsSync)).toBe(true);",
  "  expect(busy.busyProfiles).toBe(1);expect(busy.removed).toEqual([]);expect([old,mid,fresh,newest].every(existsSync)).toBe(true);expect(busy.held).toBe(4);"),
])

with open(f"{U}/🧪️tests/🟦️.ts", encoding="utf8") as handle:
    tests = handle.read()
marker = "test(\"a real compiler lock keeps its unit, a real exclusive build lease makes a profile busy"
extra = '''test("a collector that runs while a compiler holds the build lease of the profile leaves every unit of it, idle, superseded, unlocked, over budget and under pressure alike, and takes them once the compiler is gone",async()=>{
 if(!["darwin","linux"].includes(process.platform))return;
 const {root,store,leases}=privateWorkspace("units-holder-");
 try{
  mkdirSync(join(store,"u","locks"),{recursive:true});
  const paths=["a","b","c","d","e"].map((hash,index)=>unit(store,"b/p/prototype","pack-http",hash,65536,200-index)),profile=unitProfilesV1(store)[0]!;
  const compiler=await acquireResourceLease({directory:leases,resource:profileLeaseResourceV1(profile),mode:"shared",signal:new AbortController().signal,timeoutMs:0});
  try{
   const dry=await collectUnitsV1(store,{dryRun:true,needBytes:()=>10**12,budgetBytes:0});expect(dry.removed).toEqual([]);expect(dry.held).toBe(paths.length);expect(dry.busyProfiles).toBe(1);
   const applied=await collectUnitsV1(store,{dryRun:false,needBytes:()=>10**12,budgetBytes:0});expect(applied.removed).toEqual([]);expect(paths.every(existsSync)).toBe(true);
   await reclaimUnitStorageV1(store,10**12,{capacityBytes:1});expect(paths.every(existsSync),"the reclaim before a build leaves them too").toBe(true);
   const swept=await collectGarbageSingleFlightV1(join(store,"u"),1,{nowMs:Date.now()+365*24*hour,volume:()=>({volumeBytes:2**40,freeBytes:1})});if(!swept.running)expect(swept.applied.units).toBe(0);expect(paths.every(existsSync),"the pressure sweep leaves them too").toBe(true);
  }finally{compiler.release();}
  const gone=await collectUnitsV1(store,{dryRun:false,needBytes:()=>10**12,budgetBytes:0});expect(gone.removed.length).toBe(paths.length);expect(paths.some(existsSync)).toBe(false);
 }finally{rmSync(root,{recursive:true,force:true});}
},60000);

'''
if "a collector that runs while a compiler holds the build lease" not in tests:
    if tests.count(marker) != 1:
        problems.append("tests: marker anchor count %d" % tests.count(marker))
    else:
        write(f"{U}/🧪️tests/🟦️.ts", tests.replace(marker, extra + marker))
        print("edit tests (new holder test)")

print("PROBLEMS" if problems else "anchors ok", *problems, sep="\n")
print("applied" if apply and not problems else "dry run" if not apply else "applied with problems")
sys.exit(1 if problems else 0)
