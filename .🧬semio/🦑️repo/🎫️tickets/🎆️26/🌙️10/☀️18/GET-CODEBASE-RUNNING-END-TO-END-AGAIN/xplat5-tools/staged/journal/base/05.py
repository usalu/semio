"""🐍️ Independent oracle of the journal state directory: the worktree id and the layout per platform, recomputed from the account database. usage: python3 -I oracle.py < {vectors:[...], process:{root}}  -> {expected:[...], account:{uid, home, id}}"""
import hashlib, json, ntpath, os, posixpath, pwd, sys
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
entry = pwd.getpwuid(os.getuid()) if hasattr(pwd, "getpwuid") else None
json.dump({"expected": [expected(vector) for vector in data["vectors"]], "account": {"uid": str(os.getuid()), "home": entry.pw_dir, "id": identifier(data["root"])}}, sys.stdout)
