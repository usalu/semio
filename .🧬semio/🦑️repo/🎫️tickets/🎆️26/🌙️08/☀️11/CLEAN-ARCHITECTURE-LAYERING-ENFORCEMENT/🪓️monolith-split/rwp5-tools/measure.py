# 📏️ Size of every WP5 staging crate in a Shell tree: files, lines, bytes (the gate counts every line, comments and blanks included).
import os, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import cfg
def measure(root):
    out = {}
    for d, c, _ in cfg.CRATES:
        base = cfg.stg(root, d); n = l = b = 0
        if os.path.isdir(base):
            for dp, dn, fn in os.walk(base):
                for f in fn:
                    if f.endswith(".rs"):
                        data = open(os.path.join(dp, f), "rb").read(); n += 1; l += data.count(b"\n") + (0 if data.endswith(b"\n") else 1); b += len(data)
        out[c] = (n, l, b)
    return out
if __name__ == "__main__":
    root = sys.argv[1] if len(sys.argv) > 1 else cfg.NEW
    for c, (n, l, b) in measure(root).items(): print("%-22s files %3d  lines %5d  bytes %7d  %s" % (c, n, l, b, "OVER" if l > 3500 or b > 230000 or n > 27 else ""))
