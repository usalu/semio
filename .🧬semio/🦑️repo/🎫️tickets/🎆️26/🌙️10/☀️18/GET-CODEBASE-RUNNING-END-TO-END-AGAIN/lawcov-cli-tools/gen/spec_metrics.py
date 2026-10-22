from decimal import Decimal, ROUND_HALF_UP
import math

def rs(n, d=3):
    if not math.isfinite(n) or n == 0:
        return Decimal(0)
    x = Decimal(repr(abs(n)))
    exp = x.adjusted()
    q = Decimal(1).scaleb(exp - d + 1)
    r = x.quantize(q, rounding=ROUND_HALF_UP) if d - 1 - exp >= 0 else (x / q).quantize(Decimal(1), rounding=ROUND_HALF_UP) * q
    return r if n > 0 else -r

def plain(x):
    s = format(x.normalize(), "f")
    if "." in s:
        s = s.rstrip("0").rstrip(".")
    return s or "0"

def coeff(x):
    if x == 0:
        return "0"
    return plain(rs(float(x), 3))

def count(n, units):
    v = abs(n)
    if not math.isfinite(v) or v <= 0:
        return "0"
    r = rs(v, 3)
    if r == 0:
        r = rs(v, 1)
    for size, suffix in units:
        if r >= size:
            return coeff(r / size) + suffix
    return coeff(r) + units[-1][1] if False else coeff(r) + ("" if units is LOC else "B")

LOC = [(Decimal(1000000), "M"), (Decimal(1000), "k")]
SIZE = [(Decimal(1000000000), "GB"), (Decimal(1000000), "MB"), (Decimal(1000), "KB")]

def loc(n):
    return count(n, LOC)

def size(n):
    return count(n, SIZE)

def ratio(n):
    if not math.isfinite(n) or n <= 0:
        return "0"
    pct = n * 100
    v = rs(pct, 3)
    if v == 0:
        v = rs(pct, 1)
    if v >= 1:
        return coeff(v)
    places = 3 if v >= Decimal("0.1") else 2 if v >= Decimal("0.01") else 3 if v >= Decimal("0.001") else 4
    f = Decimal(10) ** places
    capped = (v * f).quantize(Decimal(1), rounding=ROUND_HALF_UP) / f
    if capped == 0:
        return coeff(1 / f)
    return coeff(capped)

H = "📊️metric"
SYM = {"net": "➕️", "edit": "✏️", "rem": "➖️", "sum": "\U0001f7f0️", "bloc": "\U0001f4af️", "up": "\U0001f4c8️", "down": "\U0001f4c9️", "ratio": "➗️"}
TOK = {"uloc": "📃uloc", "size": "💾size"}

def body_line(kind, code, added, edited, removed, le=None, ls=None):
    f = loc if kind == "uloc" else size
    line = H + (le + ls if le and ls else "") + TOK[kind]
    bloc = max(0, int(code + 0.5))
    if bloc > 0:
        line += SYM["bloc"] + f(bloc)
    net = added - removed
    if net != 0:
        line += (SYM["up"] if net > 0 else SYM["down"]) + f(abs(net))
        if bloc - net > 0:
            line += SYM["ratio"] + ratio(abs(net) / (bloc - net))
    for key, v in (("net", added), ("edit", edited), ("rem", removed)):
        if v > 0:
            line += SYM[key] + f(v)
    t = added + edited + removed
    if t > 0:
        line += SYM["sum"] + f(t)
    return line
