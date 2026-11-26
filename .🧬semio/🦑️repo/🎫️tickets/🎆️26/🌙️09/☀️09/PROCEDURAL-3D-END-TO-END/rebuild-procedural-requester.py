#!/usr/bin/env python3
"""🏗️ Drive the pinned compiler and retain the procedural requester's exact final link objects."""
import hashlib
import json
import os
from pathlib import Path
import shutil
import subprocess
import sys

ticket = Path(__file__).resolve().parent
generated = ticket / "🗑️generated/bridge-current"
build = generated / "b4"
crate = "semio_hub_procedural_component"


def write_json(path, value):
    pending = path.with_suffix(".pending")
    pending.write_text(json.dumps(value, ensure_ascii=False))
    pending.replace(path)


def sha(path):
    digest = hashlib.sha256()
    with path.open("rb") as source:
        for block in iter(lambda: source.read(1 << 20), b""):
            digest.update(block)
    return digest.hexdigest()


def build_core(command, output, compiler):
    host = subprocess.check_output([compiler, "-vV"], text=True).split("host: ", 1)[1].splitlines()[0]
    sysroot = Path(subprocess.check_output([compiler, "--print", "sysroot"], text=True).strip())
    linker = sysroot / "lib/rustlib" / host / "bin/rust-lld"
    core = build / "proc.core.wasm"
    if not core.exists():
        temp = core.with_suffix(".pending")
        command = list(command)
        command[command.index("-o") + 1] = str(temp)
        if command[:2] != ["-flavor", "wasm"]:
            command[:0] = ["-flavor", "wasm"]
        command.append("--threads=1")
        subprocess.run([linker, *command], check=True)
        temp.replace(core)
        print("[DEBUG] linked procedural core", core.stat().st_size, sha(core), flush=True)
    compact = core
    for round_number in (1, 2):
        reduced = build / f"proc.fold{round_number}.wasm"
        if not reduced.exists():
            subprocess.run([generated / "deduplicate-requester", compact, reduced], check=True)
        compact = reduced
    component = build / f"{crate}.wasm"
    if not component.exists():
        pending = component.with_suffix(".pending")
        subprocess.run([sys.executable, ticket / "repack-procedural-core.py", compact, pending, compiler], check=True)
        pending.replace(component)
    if output.resolve() != component.resolve():
        shutil.copyfile(component, output)


def replay_link(args):
    build.mkdir(parents=True, exist_ok=True)
    compiler = Path(os.environ["SEMIO_PROCEDURAL_REQUESTER_RUSTC"])
    output = Path(args[args.index("-o") + 1])
    saved = []
    for index, arg in enumerate(args):
        item = Path(arg)
        if arg.endswith(".o") and item.is_file() and item.name.startswith(crate):
            retained = build / f"code{index}.o"
            if not retained.exists() or sha(retained) != sha(item):
                shutil.copyfile(item, retained)
            args[index] = str(retained)
            saved.append({"path": retained.name, "sha256": sha(retained)})
    if not saved:
        raise ValueError("procedural requester linker call contained no final rustc objects")
    link_path = build / "link.json"
    previous = json.loads(link_path.read_text()) if link_path.exists() else None
    receipt = {"args": args, "compiler": str(compiler), "objects": saved}
    if previous and (previous["objects"] != saved or previous["args"] != args):
        for filename in ("proc.core.wasm", "proc.fold1.wasm", "proc.fold2.wasm", f"{crate}.wasm"):
            (build / filename).unlink(missing_ok=True)
    write_json(link_path, receipt)
    build_core(args, output, compiler)


def invoke_rustc(args):
    compiler = args[0]
    if "--crate-name" not in args or args[args.index("--crate-name") + 1] != crate:
        os.execv(generated / "retry-compiler", [str(generated / "retry-compiler"), *args])
    build.mkdir(parents=True, exist_ok=True)
    write_json(build / "rustc.json", args)
    os.environ["SEMIO_PROCEDURAL_REQUESTER_RUSTC"] = compiler
    os.execv(compiler, [*args, "-Clinker=" + str(Path(__file__).resolve())])


if __name__ == "__main__":
    arguments = sys.argv[1:]
    if arguments == ["--relink"]:
        receipt = json.loads((build / "link.json").read_text())
        build_core(receipt["args"], build / f"{crate}.wasm", receipt["compiler"])
    elif "SEMIO_PROCEDURAL_REQUESTER_RUSTC" in os.environ:
        replay_link(arguments)
    else:
        invoke_rustc(arguments)
