"""One-line string edit of the print theme leaf file, byte backup + byte-exact revert (rustedit-g).

usage: edit.py apply|revert|status      backup: backup/theme.rs.orig + backup/theme.rs.sha256
"""
import hashlib, os, sys

HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = "/Users/ueli/Documents/semio"
TARGET = os.path.join(ROOT, "🧰️framework/🛍️products/📓️print/🧬️schema/💡️inferences/🎨theme/🦀️.rs")
BACKUP = os.path.join(HERE, "backup", "theme.rs.orig")
SHA = os.path.join(HERE, "backup", "theme.rs.sha256")
OLD = b'format!("invalid colour: {value}")'
NEW = b'format!("invalid colour value: {value}")'


def sha(data):
    return hashlib.sha256(data).hexdigest()


def main():
    mode = sys.argv[1]
    current = open(TARGET, "rb").read()
    if mode == "status":
        print("current", sha(current), "backup", open(SHA).read().strip() if os.path.exists(SHA) else "none")
    elif mode == "apply":
        if os.path.exists(BACKUP):
            raise SystemExit("backup exists: revert first")
        if current.count(OLD) != 1:
            raise SystemExit("anchor not unique: " + str(current.count(OLD)))
        open(BACKUP, "wb").write(current)
        open(SHA, "w").write(sha(current) + "\n")
        open(TARGET, "wb").write(current.replace(OLD, NEW))
        print("applied", sha(current), "->", sha(open(TARGET, "rb").read()))
    elif mode == "revert":
        if not os.path.exists(BACKUP):
            print("nothing to revert")
            return
        original = open(BACKUP, "rb").read()
        if current != original:
            if current.replace(NEW, OLD) != original:
                raise SystemExit("peer changed the file meanwhile, left as is")
        open(TARGET, "wb").write(original)
        assert sha(open(TARGET, "rb").read()) == open(SHA).read().strip()
        os.remove(BACKUP)
        print("reverted byte-exact", sha(original))


main()
