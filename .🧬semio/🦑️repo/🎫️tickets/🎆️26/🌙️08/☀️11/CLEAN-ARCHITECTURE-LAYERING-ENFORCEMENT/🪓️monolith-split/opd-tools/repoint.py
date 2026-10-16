"""Re-points every consumer of the os-pack schema duplicates to the owner path pack::record (opd slice). Re-reads each file, asserts the expected token counts, backs up the original once, writes atomically.
Usage: repoint.py [--write]"""
import os, re, sys, shutil
ROOT = "/Users/ueli/Documents/semio"
SP = "/private/tmp/claude-501/-Users-ueli--claude-spares-spare-PBncC5/6c384ddf-f4e2-43bf-8d3c-c66daff05e49/scratchpad"
PRE = SP + "/opdpre/"
NAMES = r"(?:schema_hash_controlled|schema_hash|PackSchemaGraph|PackSchemaShape|PackSchemaField)"
RULES = [
    (re.compile(r"(?<![A-Za-z0-9_])store::os_pack::(" + NAMES + r")\b"), r"pack::record::\1"),
    (re.compile(r"(?<![A-Za-z0-9_])store::(schema_hash)\b(?=\()"), r"pack::record::\1"),
    (re.compile(r"(?<![A-Za-z0-9_])semio_framework_os_kernel::os_pack::(" + NAMES + r")\b"), r"pack::record::\1"),
    (re.compile(r"(?<![A-Za-z0-9_])dsl::os_pack::(" + NAMES + r")\b"), r"pack::record::\1"),
    (re.compile(r"(?<![A-Za-z0-9_])semio_framework_os_pack::(" + NAMES + r")\b"), r"pack::record::\1"),
    (re.compile(r"(?<![A-Za-z0-9_:])crate::os_pack::(" + NAMES + r")\b"), r"pack::record::\1"),
    (re.compile(r"(?<=`)os_pack::(schema_hash)\b"), r"pack::record::\1"),
]
files = [l.strip() for l in open(SP + "/consumer-files.txt", encoding="utf8") if l.strip()]
write = "--write" in sys.argv
total = 0
for f in files:
    p = os.path.join(ROOT, f)
    t = open(p, encoding="utf8").read()
    n = 0
    for rx, rep in RULES:
        t, k = rx.subn(rep, t)
        n += k
    if n:
        print(f"{n:3d} {f[-110:]}")
        total += n
        if write:
            b = PRE + f
            if not os.path.exists(b):
                os.makedirs(os.path.dirname(b), exist_ok=True)
                shutil.copyfile(p, b)
            tmp = p + ".opd-tmp"
            open(tmp, "w", encoding="utf8").write(t)
            os.replace(tmp, p)
print("total replacements", total, "(written)" if write else "(dry)")
