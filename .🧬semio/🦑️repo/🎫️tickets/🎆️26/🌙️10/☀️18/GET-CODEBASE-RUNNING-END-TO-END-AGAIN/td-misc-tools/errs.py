"""🔎 [td-misc] pairs each `(fail)` line of a bun test log with the first error line printed before it. usage: python3 -I errs.py <log>..."""
import re, sys
for p in sys.argv[1:]:
    print("##", p.split("/")[-1]); err = None
    for line in open(p, encoding="utf8", errors="replace"):
        s = line.rstrip("\n")
        if re.match(r"^(error|TypeError|RangeError|ReferenceError|SyntaxError|Error)\b", s) or re.match(r"^\w*Error: ", s):
            err = err or s[:230]
        if s.startswith("(fail)"):
            print("  FAIL", s[7:120], "\n     <-", err); err = None
        if s.startswith("(pass)"): err = None
