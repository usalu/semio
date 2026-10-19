import os,sys,re
R="/Users/ueli/Documents/semio/"
D=R+"🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/"
dry="--apply" not in sys.argv
def rd(p): return open(p,encoding="utf8").read()
def wr(p,t):
    if not dry: open(p,"w",encoding="utf8").write(t)
def sub(t,old,new,count=1):
    assert t.count(old)==count,(old[:80],t.count(old))
    return t.replace(old,new)

leaf_dir=D+"🔌️retirement-backend/"
leaf='''/** 🔌️ Platform strategy of the native storage retirement checks: the persistent backend binds for darwin and the fixed backend for every other platform. The storage core and the creator claim dispatch through it, so neither depends on a backend and each backend depends on both. */
export interface NativeRetirementBackendV1 {
  requireSuccessfulChildRetirement(custody: unknown, cleanup: unknown, enrollment: unknown, request: unknown): void;
  requireFailedBirthRetirement(owner: unknown, enrollment: unknown, request: unknown): void;
  requireRecoveredClaim(custody: unknown, cleanup: unknown, enrollment: unknown, request: unknown, claim?: unknown, removed?: boolean): void;
  requireRemovedBirthClaim(owner: unknown, enrollment: unknown, claim: unknown, request: unknown): void;
}
export type NativeRetirementBackendKindV1 = "persistent" | "fixed";

const bound = new Map<NativeRetirementBackendKindV1, NativeRetirementBackendV1>();

/** 🔗️ Binds one backend's checks when its module loads. */
export function bindNativeRetirementBackendV1(kind: NativeRetirementBackendKindV1, backend: NativeRetirementBackendV1): void {
  bound.set(kind, backend);
}

/** 🪪️ The backend of this platform; an unbound backend is a wiring fault. */
export function nativeRetirementBackendV1(): NativeRetirementBackendV1 {
  const kind: NativeRetirementBackendKindV1 = process.platform === "darwin" ? "persistent" : "fixed";
  const backend = bound.get(kind);
  if (!backend) throw new Error("Native storage retirement backend is unbound: " + kind);
  return backend;
}
'''
if not dry:
    os.makedirs(leaf_dir,exist_ok=True); open(leaf_dir+"🟦️.ts","w",encoding="utf8").write(leaf)

# S
p=D+"🟦️.ts"; t=rd(p)
t=sub(t,'if(process.platform==="darwin")requirePersistentNativeFailedBirthRetirementV1(owner,enrollment,request);else requireFixedNativeFailedBirthRetirementV1(owner,enrollment,request);','nativeRetirementBackendV1().requireFailedBirthRetirement(owner,enrollment,request);')
t=sub(t,'if(process.platform==="darwin")requirePersistentNativeSuccessfulChildRetirementV1(retirement,owner,enrollment,request);else requireFixedNativeSuccessfulChildRetirementV1(retirement,owner,enrollment,request);','nativeRetirementBackendV1().requireSuccessfulChildRetirement(retirement,owner,enrollment,request);')
for old in ['import{requirePersistentNativeSuccessfulChildRetirementV1}from"./🧹️cleanup/🎛️command/🟦️.ts";\n','import{requireFixedNativeSuccessfulChildRetirementV1}from"./🧱️fixed/🟦️.ts";\n','import{requirePersistentNativeFailedBirthRetirementV1}from"./🧹️cleanup/🎛️command/🟦️.ts";\n']:
    t=sub(t,old,'')
t=sub(t,'import{requireFixedNativeFailedBirthRetirementV1}from"./🧱️fixed/🟦️.ts";\n','import{nativeRetirementBackendV1}from"./🔌️retirement-backend/🟦️.ts"\n')
wr(p,t)

# B
p=D+"🌱️birth/🧾️claim/🟦️.ts"; t=rd(p)
t=sub(t,'if(process.platform==="darwin")requirePersistentNativeRemovedBirthClaimV1(owner,enrollment,claim,scope.originalRequest);else requireFixedNativeRemovedBirthClaimV1(owner,enrollment,claim,scope.originalRequest);','nativeRetirementBackendV1().requireRemovedBirthClaim(owner,enrollment,claim,scope.originalRequest);')
t=sub(t,'backend=process.platform==="darwin"?requirePersistentNativeRecoveredClaimV1:requireFixedNativeRecoveredClaimV1;','backend=nativeRetirementBackendV1();')
t=sub(t,'backend(owner,cleanup,enrollment,request);','backend.requireRecoveredClaim(owner,cleanup,enrollment,request);')
t=sub(t,'backend(owner,cleanup,enrollment,request,claim,true);','backend.requireRecoveredClaim(owner,cleanup,enrollment,request,claim,true);')
for old in ['import{requirePersistentNativeRecoveredClaimV1}from"../../🧹️cleanup/🎛️command/🟦️.ts";\n','import{requireFixedNativeRecoveredClaimV1}from"../../🧱️fixed/🟦️.ts";\n','import{requirePersistentNativeRemovedBirthClaimV1}from"../../🧹️cleanup/🎛️command/🟦️.ts";\n']:
    t=sub(t,old,'')
t=sub(t,'import{requireFixedNativeRemovedBirthClaimV1}from"../../🧱️fixed/🟦️.ts";\n','import{nativeRetirementBackendV1}from"../../🔌️retirement-backend/🟦️.ts";\n')
wr(p,t)

# F bind
p=D+"🧱️fixed/🟦️.ts"; t=rd(p)
first=t.index("\n")
imp='import{bindNativeRetirementBackendV1}from"../🔌️retirement-backend/🟦️.ts";\n'
assert imp not in t
t=imp+t
t=t.rstrip("\n")+'\nbindNativeRetirementBackendV1("fixed",{requireSuccessfulChildRetirement:requireFixedNativeSuccessfulChildRetirementV1,requireFailedBirthRetirement:requireFixedNativeFailedBirthRetirementV1,requireRecoveredClaim:requireFixedNativeRecoveredClaimV1,requireRemovedBirthClaim:requireFixedNativeRemovedBirthClaimV1});\n'
wr(p,t)

# K bind
p=D+"🧹️cleanup/🎛️command/🟦️.ts"; t=rd(p)
imp='import{bindNativeRetirementBackendV1}from"../../🔌️retirement-backend/🟦️.ts";\n'
assert imp not in t
t=imp+t
t=t.rstrip("\n")+'\nbindNativeRetirementBackendV1("persistent",{requireSuccessfulChildRetirement:requirePersistentNativeSuccessfulChildRetirementV1,requireFailedBirthRetirement:requirePersistentNativeFailedBirthRetirementV1,requireRecoveredClaim:requirePersistentNativeRecoveredClaimV1,requireRemovedBirthClaim:requirePersistentNativeRemovedBirthClaimV1});\n'
wr(p,t)
print("dry" if dry else "applied")
