import os,sys
HERE=os.path.dirname(os.path.abspath(__file__))
ROOT=sys.argv[1] if len(sys.argv)>1 and not sys.argv[1].startswith("--") else None
if ROOT is None:
    p=HERE
    while p!=os.path.dirname(p):
        if os.path.isfile(os.path.join(p,"bunfig.toml")) and os.path.isfile(os.path.join(p,"bun.lock")):ROOT=p;break
        p=os.path.dirname(p)
L="🧰️framework/🔨️modules/🏃️process/🔒️leases/"
def rd(rel):return open(os.path.join(ROOT,rel),encoding="utf8").read()
def once(t,o,n):
    assert t.count(o)==1,o[:60]
    return t.replace(o,n)
out={}
c=rd(L+"🎛️control/🟦️.ts")
if "isSystemLeaseLinkV1" not in c:
    c=once(c,'import { resolve } from "node:path";','import { dirname, resolve } from "node:path";')
    c+='''
/** 🔗️ Whether an ancestor symlink belongs to the operating system rather than to the store: a link directly below the filesystem root that only root could create (macOS `/var`, `/tmp`, `/etc`), or on Windows any link directly below the drive root. Such a link cannot be substituted by the store's users, so the strict no-symlink rule stands for every other ancestor. */
export function isSystemLeaseLinkV1(current: string, stat: Readonly<{ uid: number }>): boolean {
  const parent = dirname(current);
  if (process.platform === "win32") return dirname(parent) === parent;
  return stat.uid === 0 && (process.getuid?.() !== 0 || dirname(parent) === parent);
}
'''
    out[L+"🎛️control/🟦️.ts"]=c
t=rd(L+"🟦️.ts")
if "function systemLeaseLink" in t:
    a=t.index("/** 🔗️ Whether an ancestor symlink");b=t.index("function databasePath")
    t=t[:a]+t[b:]
    t=t.replace("systemLeaseLink(current, stat)","isSystemLeaseLinkV1(current, stat)")
    t=once(t,'import {checkResourceLeaseControl,requireMaterializedPathV1,type LeaseWait','import {checkResourceLeaseControl,isSystemLeaseLinkV1,requireMaterializedPathV1,type LeaseWait')
    out[L+"🟦️.ts"]=t
r=rd(L+"♻️retirement/🟦️.ts")
if "isSystemLeaseLinkV1" not in r:
    r=once(r,'import {lstatSync,type Stats} from "node:fs";','import {lstatSync,statSync,type Stats} from "node:fs";\nimport {isSystemLeaseLinkV1} from "../🎛️control/🟦️.ts";')
    r=once(r,'const row=lstatSync(path);if(!row.isDirectory()||row.isSymbolicLink())throw Error("Original retirement directory required");held??=row;','const row=lstatSync(path),system=row.isSymbolicLink()&&isSystemLeaseLinkV1(path,row)&&statSync(path).isDirectory();if(!system&&(!row.isDirectory()||row.isSymbolicLink()))throw Error("Original retirement directory required");held??=row;')
    out[L+"♻️retirement/🟦️.ts"]=r
for k in out:print("edit",k)
if "--write" in sys.argv:
    for k,v in out.items():
        p=os.path.join(ROOT,k);open(p+".t","w",encoding="utf8").write(v);os.replace(p+".t",p)
    print("written")
