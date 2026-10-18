import sys, hashlib, re
R = "/Users/ueli/Documents/semio"
G = R + "/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/🗑️generated"
label, f = sys.argv[1], sys.argv[2]
maxc = int(sys.argv[3]) if len(sys.argv) > 3 else 3500
if not f.startswith("🧰️"): f = "🧰️framework/🛍️products/🦑️repo/🔨️modules/" + f
k = hashlib.sha1(f.encode()).hexdigest()[:12]
t = open(f"{G}/tsl2-{label}/{k}.log", encoding="utf8", errors="replace").read()
t = re.sub(r"^\s*\d+ \|.{200,}$", lambda m: m.group(0)[:200] + "...", t, flags=re.M)
print(t[:maxc])
