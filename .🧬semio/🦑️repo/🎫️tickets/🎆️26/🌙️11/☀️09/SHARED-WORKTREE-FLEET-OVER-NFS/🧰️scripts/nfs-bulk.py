#!/usr/bin/env python3
"""usage: nfs-bulk.py KIND ROLE TREE ARGS... -- timestamps (epoch seconds) for operations that are watched on another machine. KIND: create DIR N | delete DIR | rename N | long | exec; ROLE: W (do it and print t0), M (rename only), P (rename: wait until the source is complete) or R SECONDS (wait until it is visible and print the epoch). Join the two epochs with the clock offset (nfs-clock.py)."""
import json, os, shutil, stat, sys, time

kind, role, tree = sys.argv[1], sys.argv[2], sys.argv[3]
rest = sys.argv[4:]

def body(i): return "file %d %s\n" % (i, "x" * (i % 97))

def long_rel():
    glyphs = [chr(c) for c in range(0x1F300, 0x1F650)]
    parts = ["".join(glyphs[(k * 131 + j) % len(glyphs)] for j in range(63)) for k in range(3)]
    return "/".join(parts) + "/" + "".join(glyphs[(7 * j + 5) % len(glyphs)] for j in range(60)) + ".txt"

def wait(check, seconds):
    end = time.time() + seconds
    while time.time() < end:
        try:
            if check(): return time.time()
        except OSError:
            pass
        time.sleep(0.005)
    return None

def out(**kw): print(json.dumps(kw, ensure_ascii=False))

if kind == "create":
    d, n = rest[0], int(rest[1])
    if role == "W":
        os.makedirs(os.path.join(tree, d), exist_ok=True); t0 = time.time()
        for i in range(n):
            with open(os.path.join(tree, d, "f%04d.txt" % i), "w") as f: f.write(body(i))
        out(t0=t0, created=time.time())
    else:
        def all_there():
            if not os.path.isfile(os.path.join(tree, d, "f%04d.txt" % (n - 1))): return False
            return all(open(os.path.join(tree, d, "f%04d.txt" % i)).read() == body(i) for i in range(n))
        out(seen=wait(all_there, float(rest[2])))
elif kind == "delete":
    d = rest[0]
    if role == "W":
        t0 = time.time(); shutil.rmtree(os.path.join(tree, d)); out(t0=t0, removed=time.time())
    else:
        out(seen=wait(lambda: not os.path.exists(os.path.join(tree, d)), float(rest[1])))
elif kind == "rename":
    n = int(rest[0]); src, dst = "nfs-rename/src", "nfs-rename/dst"
    if role == "D":
        shutil.rmtree(os.path.join(tree, "nfs-rename"), ignore_errors=True); out(removed="nfs-rename")
    elif role == "W":
        for i in range(n):
            os.makedirs(os.path.join(tree, src, "d%d" % (i % 10)), exist_ok=True)
            with open(os.path.join(tree, src, "d%d" % (i % 10), "f%03d.txt" % i), "w") as f: f.write(body(i))
        out(created=time.time())
    elif role == "M":
        t0 = time.time(); os.rename(os.path.join(tree, src), os.path.join(tree, dst)); out(t0=t0)
    elif role == "P":
        out(seen=wait(lambda: os.path.isfile(os.path.join(tree, src, "d9", "f%03d.txt" % (n - 1))), float(rest[1])))
    else:
        def complete():
            return not os.path.exists(os.path.join(tree, src)) and sum(len(fs) for _, _, fs in os.walk(os.path.join(tree, dst))) == n
        out(seen=wait(complete, float(rest[1])))
elif kind == "long":
    rel = long_rel(); p = os.path.join(tree, rel)
    if role == "D":
        shutil.rmtree(os.path.join(tree, rel.split("/")[0]), ignore_errors=True); out(removed=rel.split("/")[0][:2])
    elif role == "W":
        os.makedirs(os.path.dirname(p), exist_ok=True); t0 = time.time()
        with open(p, "w") as f: f.write("long emoji path\n")
        out(t0=t0, relative_codepoints=len(rel), relative_utf8_bytes=len(rel.encode()), absolute_bytes=len(p.encode()), longest_component_bytes=max(len(x.encode()) for x in rel.split("/")))
    else:
        out(seen=wait(lambda: open(p).read() == "long emoji path\n", float(rest[0])))
elif kind == "exec":
    p = os.path.join(tree, "nfs-live/tool.sh")
    if role == "D":
        shutil.rmtree(os.path.join(tree, "nfs-live"), ignore_errors=True); out(removed="nfs-live")
    elif role == "W":
        os.makedirs(os.path.dirname(p), exist_ok=True)
        with open(p, "w") as f: f.write("#!/bin/sh\necho hi\n")
        os.chmod(p, 0o644); time.sleep(float(rest[0])); t0 = time.time(); os.chmod(p, 0o755); out(t0=t0)
    else:
        out(seen=wait(lambda: os.stat(p).st_mode & 0o111 == 0o111, float(rest[0])))
