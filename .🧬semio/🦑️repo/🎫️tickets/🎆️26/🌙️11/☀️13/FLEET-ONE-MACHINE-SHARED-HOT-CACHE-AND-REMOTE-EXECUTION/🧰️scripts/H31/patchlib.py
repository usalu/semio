import os, sys, hashlib, tempfile

def sha(path):
    return hashlib.sha256(open(path, "rb").read()).hexdigest()[:12]

def rep(path, old, new, count=1):
    text = open(path, encoding="utf-8").read()
    found = text.count(old)
    if found != count:
        raise SystemExit("anchor count %d != %d in %s: %r" % (found, count, path, old[:80]))
    text = text.replace(old, new)
    atomic(path, text)

def atomic(path, text):
    directory = os.path.dirname(path)
    handle, temporary = tempfile.mkstemp(dir=directory, prefix=".h31-")
    with os.fdopen(handle, "w", encoding="utf-8") as stream:
        stream.write(text)
    os.chmod(temporary, os.stat(path).st_mode & 0o777)
    os.replace(temporary, path)
