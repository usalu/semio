"""🔏️ Finish an observed compiler image and fingerprint its actual read-only contents."""
from pathlib import Path
import hashlib, json, os, signal, stat, subprocess, sys, time
root=Path.cwd(); image=Path('/Volumes/BIMInput')
out=Path(__file__).resolve().parent/'🗑️generated'
assert image.is_mount() and (image/'🌎️hub/Cargo.toml').is_file()
if sys.argv[1]=='finish':
    pid=int(sys.argv[2])
    command=subprocess.run(['ps','-p',str(pid),'-o','command='],capture_output=True,text=True).stdout
    if 'capture-compiler-inputs.py' in command: os.kill(pid,signal.SIGTERM)
    owners=['🧰️framework','✏️s','🌎️hub','.cargo','Cargo.toml','Cargo.lock','rust-toolchain.toml','package.json','📋️project.json','📋️build-owners.json','🔒️dependencies.json','📜️script.ts','nx.json','tsconfig.json']
    files=subprocess.check_output(['git','ls-files','-z','--cached','--others','--exclude-standard','--',*owners]).decode().split('\0')
    cutoff=(out/'bim-input.sparseimage').stat().st_birthtime-60
    copied=0
    for name in files:
        if not name or len(str(image/name))>256:continue
        src=root/name;dst=image/name
        try:s=src.lstat()
        except FileNotFoundError:
            if dst.is_file():dst.unlink()
            continue
        if s.st_mtime<cutoff and (dst.exists() or dst.is_symlink()):continue
        dst.parent.mkdir(parents=True,exist_ok=True)
        if stat.S_ISLNK(s.st_mode):
            link=os.readlink(src)
            assert not os.path.isabs(link)
            dst.unlink(missing_ok=True);dst.symlink_to(link)
        elif stat.S_ISREG(s.st_mode):
            while True:
                before=src.stat()
                data=src.read_bytes()
                if (before.st_ino,before.st_size,before.st_mtime_ns)==(lambda v:(v.st_ino,v.st_size,v.st_mtime_ns))(src.stat()):break
            if dst.is_symlink():dst.unlink()
            dst.write_bytes(data);dst.chmod(before.st_mode & 0o777)
        copied+=1
    print('Final compiler inputs refreshed',copied,flush=True)
else:
    manifest={}
    for directory,folders,files in os.walk(image):
        if Path(directory)==image: folders[:]=[n for n in folders if not n.startswith('.') or n=='.cargo']
        for name in files:
            p=Path(directory)/name
            if p.name.startswith('.DS_Store'):continue
            data=os.fsencode(os.readlink(p)) if p.is_symlink() else p.read_bytes()
            manifest[str(p.relative_to(image))]=hashlib.sha256(data).hexdigest()
    data=json.dumps(manifest,sort_keys=True,ensure_ascii=False,separators=(',',':')).encode()
    (out/'compiler-image-inputs.json').write_bytes(data)
    print('Sealed files',len(manifest),'SHA-256',hashlib.sha256(data).hexdigest(),flush=True)
