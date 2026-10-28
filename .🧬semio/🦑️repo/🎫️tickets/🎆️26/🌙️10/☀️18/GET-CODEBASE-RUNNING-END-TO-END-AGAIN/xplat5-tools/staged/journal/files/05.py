"""🐍️ Independent oracle of the journal state directory: the worktree id and the layout per platform, recomputed from the account database where the platform has one (Windows has neither `pwd` nor `getuid`). usage: python3 -I oracle.py < {vectors:[...], process:{root}}  -> {expected:[...], account:{uid, home, id}}"""
import hashlib, json, ntpath, os, posixpath, sys
try:
    import pwd
except ImportError:
    pwd = None
data = json.load(sys.stdin)
def identifier(root): return hashlib.sha256(("semio-repo-service-v1\0" + root).encode("utf8")).hexdigest()[:12]
def expected(vector):
    root, environment, platform, user, home = vector["root"], vector["environment"], vector["platform"], vector["user"], vector["home"]
    ident = identifier(root)
    if platform == "win32":
        base = environment.get("LOCALAPPDATA") or ntpath.join(home, "AppData", "Local")
        return ntpath.join(base, "semio", "repo", ident, "journal")
    if environment.get("XDG_RUNTIME_DIR"): return posixpath.join(environment["XDG_RUNTIME_DIR"], "semio-repo-" + user, ident, "journal")
    return posixpath.join(home, "Library/Caches" if platform == "darwin" else ".cache", "semio", "repo", ident, "journal")
entry = pwd.getpwuid(os.getuid()) if pwd is not None and hasattr(os, "getuid") else None
account = {"uid": str(os.getuid()) if hasattr(os, "getuid") else "0", "home": entry.pw_dir if entry is not None else os.path.expanduser("~"), "id": identifier(data["root"])}
json.dump({"expected": [expected(vector) for vector in data["vectors"]], "account": account}, sys.stdout)
