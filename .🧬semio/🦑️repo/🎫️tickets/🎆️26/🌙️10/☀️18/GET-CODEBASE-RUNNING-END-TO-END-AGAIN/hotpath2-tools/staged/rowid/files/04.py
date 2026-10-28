"""🐍️ Independent oracle of the runtime row identity: shlex words and the digest of os.stat fields, recomputed from the schema as data.
usage: python3 -I oracle.py <schema.json>  < {platform, arch, words:[text], identities:[{tool, arguments, source, prefix, paths:[...]}]}   -> {words:[...|null], identities:[hex]}"""
import hashlib, json, os, shlex, sys
policy = json.load(open(sys.argv[1], encoding="utf8"))["definitions"]["Policy"]["const"]
data = json.load(sys.stdin)
def accepted(text):
    quoted = False
    for character in text:
        if quoted:
            if character == '"': quoted = False
            elif character in policy["quoted"]: return False
        elif character == '"': quoted = True
        elif character in policy["unquoted"]: return False
    return not quoted
def stat_row(path):
    real = os.path.realpath(path)
    info = os.stat(real)
    fields = {"size": info.st_size, "mtimeNs": info.st_mtime_ns, "ctimeNs": info.st_ctime_ns, "ino": info.st_ino, "mode": info.st_mode}
    return json.dumps([path, real] + [str(fields[name]) for name in policy["statFields"]], separators=(",", ":"), ensure_ascii=False)
def identity(row):
    document = {"version": policy["version"], "platform": data["platform"], "arch": data["arch"], "tool": row["tool"], "arguments": row["arguments"], "source": row["source"], "prefix": row["prefix"], "files": [stat_row(path) for path in row["paths"]]}
    return hashlib.sha256(json.dumps(document, separators=(",", ":"), ensure_ascii=False).encode("utf8")).hexdigest()
json.dump({"words": [shlex.split(text) if accepted(text) else None for text in data["words"]], "identities": [identity(row) for row in data["identities"]]}, sys.stdout)
