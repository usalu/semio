import re, sys, subprocess, collections
p = sys.argv[1]
cur = open(p, "rb").read()
head = subprocess.run(["git", "show", "HEAD:" + p], capture_output=True).stdout
for name, d in (("head", head), ("cur", cur)):
    print(name, collections.Counter(re.findall(rb"\r*\n", d)))
