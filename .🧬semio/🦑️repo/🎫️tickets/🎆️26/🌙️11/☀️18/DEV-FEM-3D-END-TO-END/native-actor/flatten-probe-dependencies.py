"""🧊️ Flatten the current Rust compiler inputs without changing the Cargo build lane."""

import os
from pathlib import Path
import shutil
import sys


def response_arguments(arguments, manifest, metadata, libraries, output):
    args = iter(arguments)
    search = []
    filtered = ["-L", f"dependency={metadata}", "-L", f"dependency={libraries}"]
    for flag in args:
        if flag == "-L":
            setting = next(args)
            if setting.startswith("dependency="):
                search.append(Path(setting.split("=", 1)[1]))
            else:
                filtered.extend([flag, setting])
        elif flag == "--out-dir":
            next(args)
            filtered.extend([flag, str(output)])
        elif flag == "-C":
            setting = next(args)
            if not setting.startswith("incremental="):
                filtered.extend([flag, setting])
        elif flag == "--extern":
            setting = next(args)
            name, separator, path = setting.partition("=")
            if separator and Path(path).suffix in (".rmeta", ".dll"):
                setting = f"{name}={metadata / Path(path).name}"
            filtered.extend([flag, setting])
        else:
            filtered.append(str((manifest / flag).resolve()) if flag.endswith(".rs") else flag)
    return list(dict.fromkeys(search)), filtered


def stage_file(source, target, hardlink):
    if len(str(target)) > 256:
        raise ValueError(f"Probe path exceeds 256 characters: {target}")
    if target.exists():
        if target.stat().st_size != source.stat().st_size:
            raise ValueError(f"Two units claim {target.name}")
    elif hardlink:
        os.link(source, target)
    else:
        shutil.copy2(source, target)


def stage(args_path, manifest, metadata, libraries, output, final_args):
    args = args_path.read_text(encoding="utf-8").splitlines()
    dep_dirs, args = response_arguments(args, manifest, metadata, libraries, output)
    for directory in (metadata, libraries, output):
        directory.mkdir(parents=True, exist_ok=True)
    for index, directory in enumerate(dep_dirs, 1):
        for source in directory.iterdir():
            extension = source.suffix
            if extension == ".rlib":
                stage_file(source, libraries / source.name, hardlink=True)
            elif extension in (".rmeta", ".dll"):
                stage_file(source, metadata / source.name, hardlink=False)
        if index % 100 == 0 or index == len(dep_dirs):
            print(f"[DEBUG] staged {index}/{len(dep_dirs)} current probe dependency directories", flush=True)
    final_args.write_text("\n".join(args) + "\n", encoding="utf-8")


if __name__ == "__main__":
    stage(*(Path(value) for value in sys.argv[1:]))
