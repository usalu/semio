"""Staged (build-tooling freeze): the retained Graph snapshot reader must not treat ctime as identity.
Fresh snapshots under .cache/b/g/m/snapshots get a pure metadata change (ctime) up to ~15 s after their rename with ino, size, nlink and mtime unchanged
(observed 2026-10-26 03:57:43 -> 03:57:58, nx2-f snapshot-events.log); a reader whose lstat falls into that window fails `Retained Graph snapshot descriptor changed`
(course:test 02:39 and print:build 03:54 on Nx, every task green, rc 1). Content integrity stays pinned by the sha256 digest check of the same function.
Usage: python3 graph-snapshot-ctime.py   (refuses unless the file still has the staged base hash)"""
import hashlib, sys
FILE = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🟦️.ts"
BASE = "fe676032fa4359c314825fb8cd668b14eb731a41b4f045d47fdb903084feb658"
OLD = '["dev","ino","size","nlink","mtimeNs","ctimeNs"].some(key=>row[key as keyof typeof row]!==before[key as keyof typeof before])'
NEW = '["dev","ino","size","nlink","mtimeNs"].some(key=>row[key as keyof typeof row]!==before[key as keyof typeof before])'
root = "/Users/ueli/Documents/semio/"
data = open(root + FILE, "rb").read()
if hashlib.sha256(data).hexdigest() != BASE and OLD not in data.decode():
    sys.exit("CHANGED SINCE STAGING and anchor missing: merge by hand")
text = data.decode()
if text.count(OLD) != 1:
    sys.exit("anchor count %d != 1: merge by hand" % text.count(OLD))
open(root + FILE, "w").write(text.replace(OLD, NEW))
print("applied")
