"""📦️ Seal a finite compiler-only source image without creating a development checkout."""
import concurrent.futures, hashlib, json, os, shutil, stat, subprocess, sys
from pathlib import Path

root=Path.cwd()
image=Path(sys.argv[1]).resolve()
ticket=Path(__file__).resolve().parent
generated=ticket/'🗑️generated'
assert image.is_mount() and image!=root and not (image/'.git').exists()
owners=('🧰️framework','✏️s','🌎️hub','.cargo','Cargo.toml','Cargo.lock','rust-toolchain.toml','package.json','📋️project.json','📋️build-owners.json','🔒️dependencies.json','📜️script.ts','nx.json','tsconfig.json')
def inventory():
    listing=subprocess.check_output(['git','ls-files','-z','--cached','--others','--exclude-standard','--',*owners])
    found={os.fsdecode(row) for row in listing.split(b'\0') if row}
    for build in ('bs','bre'):
        for dep in (generated/build).glob('*/debug/build/*/*/out/*.d'):
            try:
                row=dep.open(encoding='utf8').readline().split(': ',1)[1]
                for part in row.split():
                    path=Path(os.path.normpath(root / part))
                    if path.is_relative_to(root):
                        relative=path.relative_to(root)
                        if relative.parts and relative.parts[0] in owners: found.add(str(relative))
            except (OSError,ValueError,IndexError): continue
    return sorted(k for k in found if len(str(image/k))<=256 and (root/k).is_file() or len(str(image/k))<=256 and (root/k).is_symlink())
def stamp(path):
    value=path.lstat()
    return value.st_ino,value.st_size,value.st_mtime_ns,value.st_ctime_ns
def capture(name):
    source=root/name
    dest=image/name
    dest.parent.mkdir(parents=True,exist_ok=True)
    while True:
        before=stamp(source)
        if source.is_symlink():
            target=os.readlink(source)
            if target.startswith('/'): raise RuntimeError('absolute compiler input link '+name)
            dest.unlink(missing_ok=True);dest.symlink_to(target)
            digest=hashlib.sha256(os.fsencode(target)).hexdigest()
        else:
            data=source.read_bytes()
            digest=hashlib.sha256(data).hexdigest()
            dest.write_bytes(data)
            dest.chmod(source.stat().st_mode & 0o777)
        if stamp(source)==before: return name,(before,digest)
names=inventory()
print('Compiler input files',len(names),flush=True)
rows={}
with concurrent.futures.ThreadPoolExecutor(max_workers=5) as workers:
    for index,(n,r) in enumerate(workers.map(capture,names)):
        rows[n]=r
        if index and index%25000==0:print('Captured',index,flush=True)
    for attempt in range(10):
        current=set(inventory())
        for n in rows.keys()-current:
            (image/n).unlink(missing_ok=True);del rows[n]
        changed=[n for n in current if n not in rows or stamp(root/n)!=rows[n][0]]
        print('Final source observation',attempt,len(changed),flush=True)
        if not changed:break
        rows.update(workers.map(capture,changed))
    else: raise RuntimeError('compiler source observation did not converge')
manifest={n:r[1] for n,r in sorted(rows.items())}
data=json.dumps(manifest,ensure_ascii=False,separators=(',',':')).encode()
(generated/'compiler-image-inputs.json').write_bytes(data)
print('SHA-256 of observed path/digest manifest',hashlib.sha256(data).hexdigest(),flush=True)
