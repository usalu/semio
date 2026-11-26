"""🗜️ Preserve retired native compiler intermediates in individually verifiable compressed archives."""
import gzip
import hashlib
import os
from pathlib import Path
import shutil
import sys

def digest(stream):
    result = hashlib.sha256()
    for chunk in iter(lambda:stream.read(1024*1024),b""):
        result.update(chunk)
    return result.hexdigest()

def archive(source, base, destination):
    target = destination / source.relative_to(base)
    target = target.with_name(target.name+".gz")
    target.parent.mkdir(exist_ok=True, parents=True)
    if not target.exists():
        with source.open("rb") as original, gzip.open(target,"wb",compresslevel=1) as output:
            shutil.copyfileobj(original,output,1024*1024)
    with source.open("rb") as original,gzip.open(target,"rb") as copy:
        checksum=digest(original)
        if checksum!=digest(copy):
            raise RuntimeError(f"archive verification failed for {source}")
    size=source.stat().st_size
    source.unlink()
    print(checksum,size,source.relative_to(base),flush=True)

base=Path(sys.argv[1]).resolve()
if base.name!="g3d-brep-stable-n25":
    raise ValueError("only the retired private BRep compilation is eligible")
destination=Path(sys.argv[2]).resolve()
files=sorted((p for p in base.rglob("*") if p.suffix in (".rlib",".rmeta",".dylib",".o") and p.is_file() and not p.is_symlink() and p.stat().st_nlink==1 and p.stat().st_size>=256*1024), key=lambda p:-p.stat().st_size)
for source in files:
    archive(source,base,destination)
