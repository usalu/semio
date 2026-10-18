import re, sys, os
# 🔁️ repoint_decode.py <file>... : moves OriginalNativeDecode* imports/paths from semio_framework_job to semio_framework_job_native_decode (single-line use lists, inline paths).
R = "/Users/ueli/Documents/semio"
NEW = "semio_framework_job_native_decode"
def fix_use(m):
    indent, body = m.group(1), m.group(2)
    toks = [t.strip() for t in body.split(",") if t.strip()]
    moved = [t for t in toks if t.startswith("OriginalNativeDecode")]
    kept = [t for t in toks if not t.startswith("OriginalNativeDecode")]
    if not moved: return m.group(0)
    sep = ", " if ", " in body else ","
    new = f"{indent}use {NEW}::{{{sep.join(moved)}}};"
    if kept: return f"{indent}use semio_framework_job::{{{sep.join(kept)}}};\n{new}"
    return new
for f in sys.argv[1:]:
    p = os.path.join(R, f)
    t = open(p, encoding="utf8").read()
    o = t
    t = re.sub(r"(?m)^([ \t]*)use semio_framework_job::\{([^}\n]*)\};", fix_use, t)
    t = re.sub(r"(?m)^([ \t]*)use semio_framework_job::(OriginalNativeDecode\w+);", rf"\1use {NEW}::\2;", t)
    t = re.sub(r"semio_framework_job::(OriginalNativeDecode\w+)", rf"{NEW}::\1", t)
    if t != o:
        open(p, "w", encoding="utf8").write(t)
        print("edited", f[-60:])
    else:
        print("UNCHANGED", f[-60:])
