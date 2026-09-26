"""🧾️ Appends one entry (stdin) to the end of the Session 12 log in 📓️wp-c10.md (before the session-11 evidence note)."""
import sys
path = "/Users/ueli/Documents/semio/.tmp-ticket/📓️wp-c10.md"
text = sys.stdin.read().rstrip("\n") + "\n"
s = open(path, encoding="utf-8").read()
anchor = "\n**Evidence loss 12:2x:**"
i = s.index(anchor)
s = s[:i].rstrip("\n") + "\n" + text + s[i:]
open(path, "w", encoding="utf-8").write(s)
print("logged")
