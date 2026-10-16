#!/usr/bin/env python3
"""🔐️ Adds or removes dependency edges in tracked Cargo.lock files, textually and sorted, so a manifest edit and its lock stay consistent without running cargo.

usage: lockfix.py add <package> <dependency> [--apply] [--create=<version>:<dep,dep>] | remove <package> <dependency> [--apply]
Only lock files that contain `name = "<package>"` and (for add) `name = "<dependency>"` are touched; ticket-folder locks are skipped.
"""
import re, subprocess, sys

ROOT = "/Users/ueli/Documents/semio"


def tracked_locks():
    out = subprocess.run(["git", "-c", "core.fsmonitor=false", "--no-optional-locks", "-c", "core.quotepath=false", "ls-files", "--", "*Cargo.lock"], cwd=ROOT, capture_output=True, text=True).stdout.splitlines()
    return [p for p in out if p and not p.startswith(".🧬semio/")]


def split_blocks(text):
    parts = re.split(r"(?m)^(?=\[\[package\]\]\n)", text)
    return parts


def block_name(block):
    m = re.search(r'(?m)^name = "([^"]+)"$', block)
    return m.group(1) if m else None


def dep_key(entry):
    return entry.split(" ")[0]


def main():
    mode, package, dependency = sys.argv[1:4]
    apply = "--apply" in sys.argv
    changed, skipped, untouched = [], [], 0
    for rel in tracked_locks():
        path = f"{ROOT}/{rel}"
        text = open(path, encoding="utf8").read()
        if f'name = "{package}"' not in text:
            untouched += 1
            continue
        blocks = split_blocks(text)
        names = {block_name(b) for b in blocks}
        if mode == "add" and dependency not in names:
            if not any(a.startswith("--create") for a in sys.argv):
                skipped.append((rel, f"{dependency} is not a package of this lock"))
                continue
            spec = [a for a in sys.argv if a.startswith("--create=")]
            version, sub = (spec[0].split("=", 1)[1].split(":", 1) + [""])[:2] if spec else ("0.1.0", "")
            subdeps = [d for d in sub.split(",") if d]
            if any(d not in names for d in subdeps):
                skipped.append((rel, f"{dependency} needs {subdeps} which the lock lacks"))
                continue
            deps_text = "dependencies = [\n" + "".join(f' "{d}",\n' for d in subdeps) + "]\n" if subdeps else ""
            created = f'[[package]]\nname = "{dependency}"\nversion = "{version}"\n{deps_text}\n'
            at = next((i for i, b in enumerate(blocks) if i > 0 and (block_name(b) or "") > dependency), len(blocks))
            if at == len(blocks) and not blocks[-1].endswith("\n\n"):
                blocks[-1] = blocks[-1] + "\n"
                created = created.rstrip("\n") + "\n"
            blocks.insert(at, created)
            names.add(dependency)
        edited = False
        for i, block in enumerate(blocks):
            if block_name(block) != package:
                continue
            m = re.search(r"(?ms)^dependencies = \[\n(.*?)^\]\n", block)
            if mode == "add":
                if m is None:
                    new = block.rstrip("\n") + f'\ndependencies = [\n "{dependency}",\n]\n'
                    blocks[i] = new + ("\n" if block.endswith("\n\n") else "")
                    edited = True
                    continue
                entries = re.findall(r'(?m)^ "([^"]+)",$', m.group(1))
                if any(dep_key(e) == dependency for e in entries):
                    continue
                entries.append(dependency)
                entries.sort(key=lambda e: (dep_key(e), e))
                body = "".join(f' "{e}",\n' for e in entries)
                blocks[i] = block[: m.start(1)] + body + block[m.end(1):]
                edited = True
            else:
                if m is None:
                    continue
                entries = re.findall(r'(?m)^ "([^"]+)",$', m.group(1))
                kept = [e for e in entries if dep_key(e) != dependency]
                if len(kept) == len(entries):
                    continue
                if kept:
                    body = "".join(f' "{e}",\n' for e in kept)
                    blocks[i] = block[: m.start(1)] + body + block[m.end(1):]
                else:
                    blocks[i] = block[: m.start()] + block[m.end():]
                edited = True
        if edited:
            changed.append(rel)
            if apply:
                open(path, "w", encoding="utf8").write("".join(blocks))
    print(f"{mode} {package} -> {dependency}: changed={len(changed)} skipped={len(skipped)} untouched={untouched} apply={apply}")
    for rel in changed:
        print("  changed", rel)
    for rel, why in skipped:
        print("  skipped", rel, why)


main()
