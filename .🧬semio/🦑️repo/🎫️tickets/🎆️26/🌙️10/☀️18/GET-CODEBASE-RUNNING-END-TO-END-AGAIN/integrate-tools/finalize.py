#!/usr/bin/env python3
# 📦 Turns the folded sets into the ordered, anchored apply series: one cumulative merge per group (merge.py --order prefix), the generated files regenerated from the group's own code (regen.sh), and per group the files that change against the previous state, with content hashes of the live preimage and of the result.
# usage: finalize.py [--groups groups.json] [--out DIR]    writes final/manifest.json and final/<group>/<NNNN><ext>
import hashlib, json, os, shutil, subprocess, sys

HERE = os.path.dirname(os.path.abspath(__file__))
BASE0, TMP = "/private/tmp/ig/base0", "/private/tmp/ig"


def sha(path):
    return hashlib.sha256(open(path, "rb").read()).hexdigest()


def tree(root):
    out = {}
    for base, dirs, names in os.walk(root):
        dirs[:] = [d for d in dirs if d != "_conflicts"]
        for name in names:
            if base == root and name == "_report.json":
                continue
            path = os.path.join(base, name)
            out[os.path.relpath(path, root)] = path
    return out


def main():
    args = sys.argv[1:]
    opt = lambda flag, default: args[args.index(flag) + 1] if flag in args else default
    groups = json.load(open(opt("--groups", os.path.join(HERE, "groups.json"))))
    out = opt("--out", os.path.join(HERE, "final"))
    shutil.rmtree(out, ignore_errors=True)
    os.makedirs(out)
    previous, order, manifest = None, [], {"version": 1, "groups": []}
    for group in groups:
        order += group["sets"]
        merged = f"{TMP}/merged-{group['id']}"
        run = subprocess.run([sys.executable, f"{HERE}/merge.py", "--order", ",".join(order), "--out", merged], capture_output=True, text=True)
        sys.stdout.write(run.stdout[-600:])
        if run.returncode:
            sys.stderr.write(run.stderr[-2000:])
            raise SystemExit(f"group {group['id']}: unresolved conflicts or failed patches")
        overlay = f"{TMP}/ov-{group['id']}"
        subprocess.run([f"{HERE}/mkoverlay.sh", overlay, merged], check=True, capture_output=True)
        subprocess.run([f"{HERE}/regen.sh", overlay, merged], check=True)
        shutil.rmtree(overlay, ignore_errors=True)
        state, before = tree(merged), tree(previous) if previous else {}
        live = {rel: BASE0 + "/" + rel for rel in tree(BASE0)} if previous is None else None
        files, index = [], 0
        os.makedirs(f"{out}/{group['id']}")
        for rel, path in sorted(state.items()):
            if previous is None:
                pre = live.get(rel)
            else:
                pre = before.get(rel) or (BASE0 + "/" + rel if os.path.exists(BASE0 + "/" + rel) else None)
            result = sha(path)
            if pre is not None and sha(pre) == result:
                continue
            ext = os.path.splitext(rel)[1]
            name = f"{index:04d}{ext}"
            shutil.copyfile(path, f"{out}/{group['id']}/{name}")
            files.append({"rel": rel, "op": "create" if pre is None else "replace", "baseSha256": None if pre is None else sha(pre), "resultSha256": result, "file": f"{group['id']}/{name}"})
            index += 1
        manifest["groups"].append({"id": group["id"], "title": group["title"], "sets": group["sets"], "verify": group.get("verify", []), "commands": group.get("commands", []), "files": files})
        print(f"group {group['id']}: {len(files)} files ({sum(1 for f in files if f['op'] == 'create')} created)")
        previous = merged
    json.dump(manifest, open(f"{out}/manifest.json", "w", encoding="utf8"), ensure_ascii=False, indent=1)


main()
