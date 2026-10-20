import os,sys
HERE=os.path.dirname(os.path.abspath(__file__))
ROOT=sys.argv[1] if len(sys.argv)>1 and not sys.argv[1].startswith("--") else None
if ROOT is None:
    p=HERE
    while p!=os.path.dirname(p):
        if os.path.isfile(os.path.join(p,"bunfig.toml")) and os.path.isfile(os.path.join(p,"bun.lock")):ROOT=p;break
        p=os.path.dirname(p)
rel="🧰️framework/🔨️modules/🏃️process/🔒️leases/🟦️.ts"
path=os.path.join(ROOT,rel);t=open(path,encoding="utf8").read()
if "systemLeaseLink" in t:print("already");sys.exit(0)
old='''function databasePath(directory: string, resource: string): string {
  const admitted = resourceLeasePathV1(directory, resource);
  for (let current = resolve(directory); ; current = dirname(current)) {
    try { const stat = lstatSync(current); if (!stat.isDirectory() || stat.isSymbolicLink()) throw new Error(`Invalid lease store directory: ${current}`); }
    catch (error) { if ((error as NodeJS.ErrnoException).code !== "ENOENT") throw error; }
    if (dirname(current) === current) break;
  }
  mkdirSync(directory, { recursive: true });
  const path = resourceLeasePathV1(realpathSync(directory), resource);
  if (path !== admitted) throw Error("Resource lease materialized path changed");'''
new='''/** 🔗️ Whether an ancestor symlink belongs to the operating system rather than to the store: a link directly below the filesystem root that only root could create (macOS `/var`, `/tmp`, `/etc`), or on Windows any link directly below the drive root. Such a link cannot be substituted by the store's users, so the strict no-symlink rule stands for every other ancestor. */
function systemLeaseLink(current: string, stat: ReturnType<typeof lstatSync>): boolean {
  const parent = dirname(current);
  if (process.platform === "win32") return dirname(parent) === parent;
  return stat.uid === 0 && (process.getuid?.() !== 0 || dirname(parent) === parent);
}
function databasePath(directory: string, resource: string): string {
  const admitted = resourceLeasePathV1(directory, resource);
  let system = false;
  for (let current = resolve(directory); ; current = dirname(current)) {
    try {
      const stat = lstatSync(current);
      if (stat.isSymbolicLink() && systemLeaseLink(current, stat) && statSync(current).isDirectory()) system = true;
      else if (!stat.isDirectory() || stat.isSymbolicLink()) throw new Error(`Invalid lease store directory: ${current}`);
    }
    catch (error) { if ((error as NodeJS.ErrnoException).code !== "ENOENT") throw error; }
    if (dirname(current) === current) break;
  }
  mkdirSync(directory, { recursive: true });
  const real = resourceLeasePathV1(realpathSync(directory), resource);
  if (!system && real !== admitted) throw Error("Resource lease materialized path changed");
  const path = system ? admitted : real;'''
assert t.count(old)==1,"anchor"
t=t.replace(old,new)
imp='import { appendFileSync, lstatSync,'
assert t.count(imp)==1
t=t.replace(imp,'import { appendFileSync, lstatSync, statSync,')
if "--write" in sys.argv:
    tmp=path+".conc-leases4.tmp";open(tmp,"w",encoding="utf8").write(t);os.replace(tmp,path);print("written")
else:print("edit",rel)
