"""✏️ One-line edit measurement: appends a probe comment to <file>, runs product.py <label>-edit <target>, removes the probe byte-exact, runs product.py <label>-revert <target>.
Usage: python3 -I edit-probe.py <label> <repo-relative file> <target> [comment-prefix]"""
import os, subprocess, sys
label, rel, target = sys.argv[1:4]
prefix = sys.argv[4] if len(sys.argv) > 4 else "//"
root = "/Users/ueli/Documents/semio"
path = os.path.join(root, rel)
product = os.path.join(os.path.dirname(os.path.abspath(__file__)), "product.py")
original = open(path, "rb").read()
probe = ("\n" if not original.endswith(b"\n") else "") + prefix + " gate probe " + label + "\n"
try:
    open(path, "ab").write(probe.encode())
    subprocess.run([sys.executable, product, label + "-edit", target])
finally:
    open(path, "wb").write(original)
subprocess.run([sys.executable, product, label + "-revert", target])
