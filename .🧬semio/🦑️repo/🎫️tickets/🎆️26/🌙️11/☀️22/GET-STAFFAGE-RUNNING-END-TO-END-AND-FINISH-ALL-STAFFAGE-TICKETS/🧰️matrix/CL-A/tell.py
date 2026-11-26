"""📣️ Tells lane MZ that an asset was rewritten: `python3 tell.py <asset> <start of the title of its closing section>` re-reads `requests.md` and appends the line `CL-A rewrote <asset> at <time>` at the end of that section (before the next `## ` heading)."""
import datetime, re, sys

path = "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/GET-STAFFAGE-RUNNING-END-TO-END-AND-FINISH-ALL-STAFFAGE-TICKETS/requests.md"
asset, title = sys.argv[1:3]
text = open(path, encoding="utf-8").read()
start = text.index("\n## " + title)
following = re.search(r"\n## ", text[start + 4:])
end = len(text) if following is None else start + 4 + following.start()
line = f"CL-A rewrote {asset} at {datetime.datetime.now():%Y-%m-%d %H:%M}"
body = text[start:end].rstrip("\n")
open(path, "w", encoding="utf-8").write(text[:start] + body + "\n\n" + line + "\n" + text[end:])
print("told MZ: " + line)
