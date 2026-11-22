from pathlib import Path
import json
import os
import re
import subprocess
import sys


ticket = Path(__file__).resolve().parent
generated = ticket / "🗑️generated"
lane = generated / "qr"
target = sys.argv[1] if len(sys.argv) > 1 else "526cc1907456600d"
artifacts = generated / "aw-cache"
artifacts.mkdir(exist_ok=True)
toolchain = re.search(r'^channel = "([^"]+)"$', Path("rust-toolchain.toml").read_text(), re.M)[1]
rust = ["repo", "tools", "exec", "--light", "rustup", "--", "run", toolchain, "rustc"]
cache = Path.home() / ".cache/semio/cas/v1/o"
units = Path(".cache/b/r/m/u/e")
editor = lane / f"libsemio_s_artifact_gis_gisterrain_editor-{target}.rmeta"
listing = subprocess.check_output([*rust, "-Zls=root", str(editor)], text=True)
names = {name: bytes.fromhex(hash_)[::-1] for name, hash_ in re.findall(r"(?m)^\d+ (\S+-[a-z0-9]+) hash ([a-f0-9]{32}) ", listing)}

def valid(path, hash_):
    if not path.is_file():
        return False
    with path.open("rb") as file:
        prefix = file.read(16)
        if prefix.startswith(b"rust"):
            file.seek(max(16, int.from_bytes(prefix[8:], "little") - 4096))
        return hash_ in file.read()

missing = {name: hash_ for name, hash_ in names.items() if (lane / f"lib{name}.rmeta").is_file() and not valid(lane / f"lib{name}.rmeta", hash_)}
print(f"[DEBUG] closure {len(names)} metadata entries; hash mismatch {len(missing)}", flush=True)
for directory, _, files in os.walk(units):
    for file in files:
        if not file.endswith(".json"):
            continue
        path = Path(directory) / file
        try:
            unit = json.loads(path.read_bytes())
        except (FileNotFoundError, json.JSONDecodeError):
            continue
        crate = unit.get("crate", "")
        if not any(name.startswith(f"{crate}-") for name in missing):
            continue
        outputs = unit.get("outputs", [])
        for output in outputs:
            filename = output["name"]
            if not filename.endswith(".rmeta") or not filename.startswith("lib"):
                continue
            name = filename[3:-6]
            if name not in missing:
                continue
            digest = output["sha256"]
            blob = cache / digest[:2] / digest[2:]
            if filename.startswith("libsemio_os_kernel-"):
                print(f"[DEBUG] kernel cache candidate {filename} {digest} exists={blob.exists()}", flush=True)
            if not valid(blob, missing[name]):
                continue
            related = {item["name"]: item["sha256"] for item in outputs if item["name"] in (filename, f"lib{name}.rlib")}
            if len(related) != 2 or any(not (cache / dep[:2] / dep[2:]).is_file() for dep in related.values()):
                continue
            for dependency, dep in related.items():
                link = artifacts / dependency
                if not link.is_symlink():
                    link.symlink_to(cache / dep[:2] / dep[2:])
            print(f"[DEBUG] recovered {name}", flush=True)
            del missing[name]
    if not missing:
        break
print(f"[DEBUG] unrecovered metadata ({len(missing)}): {', '.join(missing)}", flush=True)
