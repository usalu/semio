import re, sys
log, pat = sys.argv[1], sys.argv[2]
w = int(sys.argv[3]) if len(sys.argv) > 3 else 120
L = open(log, encoding="utf-8").read().split("\n")
head = re.compile(r"^(.+?)\((\d+),(\d+)\): error (TS\d+): (.*)$")
for i, l in enumerate(L):
    m = head.match(l)
    if not m or pat not in m.group(1):
        continue
    f, ln, col, code, msg = m.group(1), int(m.group(2)), int(m.group(3)), m.group(4), m.group(5)
    try:
        s = open("/Users/ueli/Documents/semio/" + f, encoding="utf-8").read().split("\n")[ln - 1]
    except Exception as e:
        s = "?"
    u = s.encode("utf-16-le")
    pre = u[: 2 * (col - 1)].decode("utf-16-le")
    post = u[2 * (col - 1):].decode("utf-16-le")
    print(f"{ln}:{col} {code} {msg[:230]}")
    k = i + 1
    while k < len(L) and L[k].startswith("  ") and k < i + 4:
        print("   " + L[k].strip()[:200]); k += 1
    print("   >> ..." + pre[-w:] + "<<<" + post[:w] + "...")
