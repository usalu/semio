#!/usr/bin/env python3
"""🏛️ Flips the default implementation mode of the build tool between live and sealed in ONE anchored, atomic, byte-exact edit of the policy file.
usage: flip.py [apply|revert|status] [--root DIR] [--dry]   (default: apply, root = the repository of the working directory, else of this script)
Exit codes: 0 done / already in the wanted state is exit 3 (nothing written), 2 anchor or JSON problem. `status` prints the mode and exits 0 (sealed) or 1 (live).
Stdlib only; works on macOS, Linux and Windows; the replaced file keeps its mode. The anchor is a whole JSON line, so a policy that is not exactly one `defaultMode` is refused instead of guessed."""
import json, os, sys

POLICY = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏛️sealed/🔣️.json"
LINE = '  "defaultMode": "%s",'


def find_root(argv):
    if "--root" in argv:
        return os.path.abspath(argv[argv.index("--root") + 1])
    for here in (os.getcwd(), os.path.dirname(os.path.abspath(__file__))):
        while True:
            if os.path.isfile(os.path.join(here, POLICY)):
                return here
            up = os.path.dirname(here)
            if up == here:
                break
            here = up
    sys.exit("flip: no repository root found (pass --root)")


def main(argv):
    verb = next((a for a in argv if a in ("apply", "revert", "status")), "apply")
    root = find_root(argv)
    path = os.path.join(root, POLICY)
    if not os.path.isfile(path):
        print("flip: policy file not found: " + path)
        return 2
    with open(path, "rb") as handle:
        raw = handle.read()
    text = raw.decode("utf-8")
    mode = json.loads(text).get("defaultMode")
    if verb == "status":
        print(mode)
        return 0 if mode == "sealed" else 1
    want = "sealed" if verb == "apply" else "live"
    old, new = LINE % ("live" if want == "sealed" else "sealed"), LINE % want
    if mode == want:
        print("flip: already %s (nothing written)" % want)
        return 3
    if text.count(old) != 1:
        print("flip: the anchor %r occurs %d times in %s" % (old, text.count(old), path))
        return 2
    changed = text.replace(old, new)
    if json.loads(changed).get("defaultMode") != want or changed.replace(new, old) != text:
        print("flip: the edit is not exactly one line")
        return 2
    if "--dry" in argv:
        print("flip: would set defaultMode %s in %s" % (want, path))
        return 0
    mode = os.stat(path).st_mode & 0o7777
    temporary = path + ".flip.%d" % os.getpid()
    with open(temporary, "wb") as handle:
        handle.write(changed.encode("utf-8"))
        handle.flush()
        os.fsync(handle.fileno())
    os.chmod(temporary, mode)
    os.replace(temporary, path)
    print("ok defaultMode", want, path[len(root) + 1:])
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
