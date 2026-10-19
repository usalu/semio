"""👀 C5B line viewer: show.py <file> <line> [width] prints the line (cut) and whether the file already imports readModuleTreeSource."""
import sys
f, n = sys.argv[1], int(sys.argv[2]); w = int(sys.argv[3]) if len(sys.argv) > 3 else 600
t = open(f, encoding="utf-8").read().split("\n")
print(f.split("/")[-3] + "/" + f.split("/")[-2], n, "| imports readModuleTreeSource:", any("readModuleTreeSource" in l and "import" in l for l in t))
print(t[n - 1][:w])
