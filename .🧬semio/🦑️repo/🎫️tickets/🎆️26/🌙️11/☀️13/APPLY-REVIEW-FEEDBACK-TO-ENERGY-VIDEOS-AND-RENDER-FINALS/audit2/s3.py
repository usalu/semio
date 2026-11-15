import re, os, sys
HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = "C:/git/semio"
files = [l.strip() for l in open(os.path.join(HERE, "files.txt"), encoding="utf-8") if l.strip()]
UNI = re.compile("[ΔθϑΦρληΣψΘΛΩ]")
SPELLED = re.compile(r"Delta[ -]?Theta|Delta[ -]T\b|Q[ -]Punkt|Q[ -]Strich|\bTheta\b|\bRho\b|\bPhi\b|\bLambda\b|\bEta\b|\bSigma\b|\bDelta\b|c-Luft|Theta-i|Theta i|Rho a\b|Q Punkt|Delta x\b|Delta-x")
STR = re.compile(r"(?:r|f|rf|fr)?\"(?:[^\"\\]|\\.)*\"|'(?:[^'\\]|\\.)*'")
DOLLAR = re.compile(r"\$[^$]*\$")
mode = sys.argv[1]
out = []
for f in files:
    if "tests" in f and mode == "uni":
        continue
    p = os.path.join(ROOT, f)
    try:
        lines = open(p, encoding="utf-8").read().splitlines()
    except Exception:
        continue
    for i, l in enumerate(lines, 1):
        if not re.search(r"[\"']", l):
            continue
        for m in STR.finditer(l):
            s = m.group(0)
            if mode == "uni":
                t = DOLLAR.sub("", s)
                if UNI.search(t):
                    out.append(f"{f}:{i}: {s.strip()[:230]}")
                    break
            else:
                if SPELLED.search(DOLLAR.sub("", s)):
                    out.append(f"{f}:{i}: {s.strip()[:230]}")
                    break
print("\n".join(out))
print("TOTAL", len(out))
