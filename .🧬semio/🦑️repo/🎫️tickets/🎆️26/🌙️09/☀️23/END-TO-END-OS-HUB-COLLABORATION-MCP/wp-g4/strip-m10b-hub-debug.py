import re, sys
p = "/Users/ueli/Documents/semio/🌎️hub/📄️documents/🦀️.rs"
s = open(p, encoding="utf-8").read()
new, n = re.subn(r'\n[ \t]*let _ = std::fs::OpenOptions::new\(\)\.create\(true\)\.append\(true\)\.open\("[^"]*wp-m10b[^"]*"\)\.and_then\(\|mut file\| \{\n[^\n]*use std::io::Write;\n[^\n]*writeln!\(file, "\[DEBUG\] m10b[^\n]*\n[ \t]*\}\);', '', s)
if n != 3 or "m10b" in new:
    sys.exit(f"unexpected n={n}")
open(p, "w", encoding="utf-8").write(new)
print("removed", n)
