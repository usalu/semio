import sys, hashlib, re
R = "/Users/ueli/Documents/semio"
G = R + "/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/🗑️generated"
label = sys.argv[1]
maxper = int(sys.argv[2])
for f in sys.argv[3:]:
    if not f.startswith("🧰️"): f = "🧰️framework/🛍️products/🦑️repo/🔨️modules/" + f
    k = hashlib.sha1(f.encode()).hexdigest()[:12]
    lines = open(f"{G}/tsl2-{label}/{k}.log", encoding="utf8", errors="replace").read().split("\n")
    print("=====", f.replace("🧰️framework/🛍️products/🦑️repo/🔨️modules/", "P:").replace("🧰️framework/🔨️modules/🏃️process/","X:"))
    n = 0; i = 0
    block = []
    for l in lines:
        if l.startswith("(fail)"):
            n += 1
            if n <= maxper:
                keep = [b for b in block if b.startswith("error") or b.startswith("Expected") or b.startswith("Received") or b.startswith("- ") or b.startswith("+ ") or "^" in b]
                print("  FAIL", l[7:140])
                for b in keep[:7]: print("     ", b[:230])
                at = [b for b in block if b.strip().startswith("at ")]
                if at: print("      ", at[0].strip()[:230].replace(R, ""))
            block = []
        else:
            block.append(l)
            if len(block) > 80: block = block[-80:]
    print("  total fails:", n)
