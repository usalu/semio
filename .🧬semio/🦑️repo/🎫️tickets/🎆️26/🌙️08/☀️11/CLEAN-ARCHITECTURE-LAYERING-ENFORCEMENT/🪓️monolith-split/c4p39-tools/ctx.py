"""🔦 C4P39 boundary context: prints lines around candidate chunk starts. usage: ctx.py <n> <line>..."""
import sys
n = sys.argv[1]
ls = open(f"/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/c4p39-tools/orig/{n}.rs", encoding="utf-8").read().split("\n")
for a in sys.argv[2:]:
    a = int(a)
    print(f"--- {a}")
    for k in range(max(1, a - 6), a + 2):
        print(f"{k:5} {ls[k-1][:130]}")
