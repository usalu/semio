import re, sys, json, unicodedata, os
ROOT = "/Users/ueli/Documents/semio"
TICKET = os.path.join(ROOT, ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN")
listing = sys.argv[1]
files = [l.strip() for l in open(listing, encoding="utf-8") if l.strip()]
EXT = r"(?:ts|tsx|js|mjs|cjs|rs|json|toml|lock|md|py|sh|yml|yaml|go|bzl|bazel|css|html|wgsl|swift|kt|cs|ps1|cmd|txt|tex|svg|gradle|xml|proto|graphql|gql|sql|mdx|jsonc|cfg|ini)"
pat = re.compile(r"(?<![\w.\-/])((?:[^\s`'\"()<>,|;=\[\]{}]+/)+[^\s`'\"()<>,|;=\[\]{}/]+\.%s)(?![\w])" % EXT)
found = {}
for f in files:
    txt = open(os.path.join(TICKET, f), encoding="utf-8", errors="replace").read()
    for m in pat.finditer(txt):
        p = m.group(1).strip("./")
        found.setdefault(p, set()).add(f)
out = {p: sorted(v) for p, v in found.items()}
print(json.dumps(out, ensure_ascii=False, indent=0))
