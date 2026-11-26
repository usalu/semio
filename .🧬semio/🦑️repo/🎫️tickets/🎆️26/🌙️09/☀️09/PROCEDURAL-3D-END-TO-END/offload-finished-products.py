"""📦 Preserve completed compiler outputs by digest before freeing their local filesystem extents."""
from pathlib import Path
import os,hashlib,shutil
g=Path(__file__).resolve().parent/"🗑️generated/bridge-current"
remote=(g/"cold-compiler.tar").resolve(strict=True).parent/"verified-products"
def digest(path):
 h=hashlib.sha256()
 with path.open("rb") as f:
  for block in iter(lambda:f.read(1<<20),b""):h.update(block)
 return h.hexdigest()
candidates=list((g/"requester-final-objects").glob("*"))+list((g/"math-variadic-wasi2").glob("*.wasm"))+list((g/"current-math-graph-variadic").glob("*.rlib"))
for p in sorted(candidates,key=lambda x:-x.stat().st_size):
 if p.is_symlink() or not p.is_file() or p.stat().st_size<5<<20:continue
 sig=p.stat()
 target=remote/p.relative_to(g);target.parent.mkdir(exist_ok=True,parents=True)
 if not target.exists():
  staged=target.with_suffix(".writing");shutil.copy2(p,staged);staged.replace(target)
 checksum=digest(p)
 if checksum!=digest(target) or sig.st_mtime_ns!=p.stat().st_mtime_ns:raise ValueError(str(p))
 link=p.with_suffix(".verified-link");link.symlink_to(target);os.utime(link,ns=(sig.st_atime_ns,sig.st_mtime_ns),follow_symlinks=False);link.replace(p)
 print(checksum,sig.st_size,p.name,flush=True)
