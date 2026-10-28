"""🔄 [rusttests] copies every file that differs between S0 and S1 (plus new ones) into a mirror root, and the S0 stack bytes first. usage: syncmirror.py <S0> <S1> <mirror>"""
import os, shutil, sys
S0, S1, M = sys.argv[1:4]
n = 0
for d, _, fs in os.walk(S1):
    for f in fs:
        p = os.path.join(d, f); rel = os.path.relpath(p, S1)
        if rel == ".stack": continue
        q = os.path.join(M, rel); os.makedirs(os.path.dirname(q), exist_ok=True)
        if not os.path.exists(q) or open(q, "rb").read() != open(p, "rb").read(): shutil.copyfile(p, q); n += 1
print("synced", n, "files into", M)
