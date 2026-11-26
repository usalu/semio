"""🧊 Stage every actual source input of the six changed BRep compiler units."""
from pathlib import Path
import hashlib
import json
import os
import shutil
import subprocess

root = Path.cwd()
snapshot = root / ".cache/g3d-native/b"
ticket = Path(__file__).resolve().parent
output = ticket / "🗑️generated/bridge-current"
family = Path("🧰️framework/🔨️modules/🧊️3d/📐️brep")
owners = [
    "📸️representation/🏄️surface",
    "📸️representation",
    "⚙️engine/🏗️solid-topology",
    "⚙️engine/🧠️native-registry/🏗️solid-transaction",
    "⚙️engine/🧠️native-registry",
    "⚙️engine/📦️mesh-io",
]
manifest = "📦️packages/🦀️rust/Cargo.toml"
selector = r"""
import { readRustSourceClosureV1 } from "./🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts";
import { readFile } from "node:fs/promises";
import { dirname, resolve } from "node:path";
const { root, owners, cfg } = JSON.parse(process.argv[1]);
const ledger = {};
for (const owner of owners) {
    const manifest = resolve(root, owner, "📦️packages/🦀️rust/Cargo.toml");
    const files = await readRustSourceClosureV1(
        [owner + "/🦀️.rs"], root, cfg,
        async file => {
            try { return await readFile(resolve(root,file)); }
            catch (error) { if (error.code === "ENOENT") return null; throw error; }
        },
        { manifestDirectory: dirname(manifest), generated: () => undefined },
    );
    ledger[owner] = files.map(({path,digest,bytes}) => ({path,digest,bytes}));
}
process.stdout.write(JSON.stringify(ledger));
"""
bun = root / ".cache/b/t/m/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun"
rustc = subprocess.check_output(["rustup", "which", "rustc"], text=True).strip()
cfg = subprocess.check_output([rustc, "--print", "cfg", "--target", "wasm32-wasip2"], text=True)
cfg = [row.replace('="', "=").rstrip('"') for row in cfg.splitlines()]
paths = [str(family / owner) for owner in owners]

def closure(directory):
    payload = {"root": str(directory), "owners": paths, "cfg": cfg}
    return json.loads(subprocess.check_output([bun, "-e", selector, json.dumps(payload)],text=True))

def digest(data):
    return hashlib.sha256(data).hexdigest()

expected = closure(root)
inputs = {}
for rows in expected.values():
    for row in rows:
        relative = Path(row["path"])
        if relative.is_absolute():
            relative = relative.relative_to(root)
        prior = inputs.setdefault(relative, row["digest"])
        if prior != row["digest"]:
            raise RuntimeError(f"source changed across owner closures: {relative}")
for owner in paths:
    relative = Path(owner) / manifest
    inputs[relative] = digest((root / relative).read_bytes())

changed = 0
for relative, sha in sorted(inputs.items()):
    source = root / relative
    private = snapshot / relative
    data = source.read_bytes()
    if digest(data) != sha:
        raise RuntimeError(f"source changed before staging: {relative}")
    if private.exists() and data == private.read_bytes():
        continue
    private.parent.mkdir(parents=True, exist_ok=True)
    pending = private.with_name(private.name + ".new")
    shutil.copy2(source, pending)
    if digest(pending.read_bytes()) != sha:
        raise RuntimeError(f"source changed during staging: {relative}")
    os.replace(pending, private)
    changed += 1
    print("[DEBUG]", sha, relative, flush=True)

actual = closure(snapshot)
assert actual == expected, "private compiler closure differs from its captured live source"
(output / "boolean-browser-staged.json").write_text(json.dumps(expected, ensure_ascii=False))
print(f"[DEBUG] verified all {len(inputs)} inputs across {len(owners)} private BRep units ({changed} updated)", flush=True)
