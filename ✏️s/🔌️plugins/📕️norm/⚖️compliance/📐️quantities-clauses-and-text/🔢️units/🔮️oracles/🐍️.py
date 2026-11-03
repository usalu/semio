"""🐍️ Independent oracle for the norm unit vocabulary: derives every unit factor from SI base atoms and prefixes with exact rational arithmetic.

Protocol: one JSON request on stdin, one JSON answer on stdout. Run isolated: `python -I 🐍️.py < request.json`.
Request keys (all optional): units [symbol], conversions [{value, from, to}], numbers [{value, precision, locale}], parses [text].
No third-party package is imported; the unit algebra is reimplemented here on purpose so the Rust tables are checked against a different derivation.
"""
import json
import re
import sys
from fractions import Fraction as F

DIMS = ("m", "kg", "s", "K", "rad", "cd", "level", "byte", "person")


def dim(**kw):
    return tuple(F(kw.get(d, 0)) for d in DIMS)


PREFIX = {"k": F(10) ** 3, "M": F(10) ** 6, "G": F(10) ** 9, "m": F(1, 10 ** 3), "c": F(1, 100), "µ": F(1, 10 ** 6)}
YEAR = F(365) * 86400 + 6 * 3600
ATOM = {
    "1": (F(1), dim()),
    "m": (F(1), dim(m=1)),
    "g": (F(1, 1000), dim(kg=1)),
    "kg": (F(1), dim(kg=1)),
    "s": (F(1), dim(s=1)),
    "K": (F(1), dim(K=1)),
    "N": (F(1), dim(kg=1, m=1, s=-2)),
    "Pa": (F(1), dim(kg=1, m=-1, s=-2)),
    "J": (F(1), dim(kg=1, m=2, s=-2)),
    "W": (F(1), dim(kg=1, m=2, s=-3)),
    "Hz": (F(1), dim(s=-1)),
    "Wh": (F(3600), dim(kg=1, m=2, s=-2)),
    "L": (F(1, 1000), dim(m=3)),
    "l": (F(1, 1000), dim(m=3)),
    "bar": (F(100000), dim(kg=1, m=-1, s=-2)),
    "min": (F(60), dim(s=1)),
    "h": (F(3600), dim(s=1)),
    "d": (F(86400), dim(s=1)),
    "a": (YEAR, dim(s=1)),
    "t": (F(1000), dim(kg=1)),
    "in": (F(254, 10000), dim(m=1)),
    "ft": (F(3048, 10000), dim(m=1)),
    "rad": (F(1), dim(rad=1)),
    "lx": (F(1), dim(cd=1, m=-2)),
    "dB": (F(1), dim(level=1)),
    "B": (F(1), dim(byte=1)),
    "MiB": (F(2) ** 20, dim(byte=1)),
    "ppm": (F(1, 10 ** 6), dim()),
    "%": (F(1, 100), dim()),
    "‰": (F(1, 1000), dim()),
    "Person": (F(1), dim(person=1)),
    "g0": (F(980665, 100000), dim(m=1, s=-2)),
    "met": (F(5815, 100), dim(kg=1, s=-3)),
    "clo": (F(155, 1000), dim(K=1, kg=-1, s=3)),
}
SUP = {"²": "^2", "³": "^3", "⁴": "^4", "⁶": "^6", "½": "^(1/2)", "₀": "0"}


def normalise(symbol):
    s = symbol
    for k, v in SUP.items():
        s = s.replace(k, v)
    return s.replace("·", "*").replace("°C", "degC").replace("°", "deg")


def tokens(s):
    return re.findall(r"degC|deg|\^\(1/2\)|\^[0-9.]+|[A-Za-zµ%‰]+[0-9]*|[0-9]+|[*/()]", s)


class Parser:
    def __init__(self, toks):
        self.t, self.i = toks, 0

    def peek(self):
        return self.t[self.i] if self.i < len(self.t) else None

    def take(self):
        self.i += 1
        return self.t[self.i - 1]

    def expr(self):
        left = self.power()
        while self.peek() in ("*", "/"):
            op = self.take()
            left = combine(left, self.power(), 1 if op == "*" else -1)
        return left

    def power(self):
        base = self.atom()
        while self.peek() is not None and self.peek().startswith("^"):
            raw = self.take()[1:]
            exp = F(1, 2) if raw == "(1/2)" else F(raw)
            factor = base[0] ** int(exp) if exp.denominator == 1 else F(1)
            base = (factor, tuple(x * exp for x in base[1]), base[2])
        return base

    def atom(self):
        t = self.take()
        if t == "(":
            value = self.expr()
            if self.take() != ")":
                raise ValueError("unbalanced")
            return value
        return unit_atom(t)


def combine(a, b, sign):
    factor = a[0] * b[0] if sign > 0 else a[0] / b[0]
    return (factor, tuple(x + sign * y for x, y in zip(a[1], b[1])), a[2] or b[2])


def unit_atom(t):
    if t == "deg":
        return (None, dim(rad=1), "deg")
    if t == "degC":
        return (F(1), dim(K=1), "degC")
    if t in ATOM:
        return ATOM[t] + (None,)
    if len(t) > 1 and t[0] in PREFIX and t[1:] in ATOM:
        factor, d = ATOM[t[1:]]
        return (factor * PREFIX[t[0]], d, None)
    raise ValueError("unknown atom " + t)


PI = 3.141592653589793


def unit(symbol):
    parser = Parser(tokens(normalise(symbol)))
    f, d, tag = parser.expr()
    if parser.peek() is not None:
        raise ValueError("trailing tokens in " + symbol)
    factor = float(f) if f is not None else PI / 180
    offset = 273.15 if tag == "degC" else 0.0
    return factor, offset, [str(x) for x in d]


def number_text(value, precision, locale):
    text = f"{value:.{precision}f}"
    neg = text.startswith("-")
    body = text[1:] if neg else text
    integer, _, frac = body.partition(".")
    if len(integer) >= 5:
        groups = []
        while integer:
            groups.insert(0, integer[-3:])
            integer = integer[:-3]
        integer = " ".join(groups)
    if neg and not any(c in "123456789" for c in body):
        neg = False
    mark = "," if locale == "de" else "."
    return ("-" if neg else "") + integer + (mark + frac if frac else "")


def parse_number(text):
    t = re.sub(r"[    ]", "", text)
    dots, commas = t.count("."), t.count(",")
    if dots and commas:
        mark = max(t.rfind("."), t.rfind(","))
    elif dots == 1 and not commas:
        mark = t.rfind(".")
    elif commas == 1 and not dots:
        mark = t.rfind(",")
    else:
        mark = -1
    out = "".join("." if i == mark else c for i, c in enumerate(t) if c not in ".," or i == mark)
    return float(out)


def parse(text):
    m = re.match(r"^\s*([+-]?[0-9][0-9.,\u2009\u202f\u00a0 ]*(?:[eE][+-]?[0-9]+)?)\s*(\S.*)?$", text)
    if not m:
        return None
    value = parse_number(m.group(1))
    sym = (m.group(2) or "").strip()
    return {"value": value, "unit": sym or None}


def main():
    req = json.load(sys.stdin)
    out = {}
    if "units" in req:
        out["units"] = [dict(zip(("factor", "offset", "dim"), unit(s))) for s in req["units"]]
    if "conversions" in req:
        res = []
        for c in req["conversions"]:
            ff, fo, _ = unit(c["from"])
            tf, to, _ = unit(c["to"])
            res.append(((c["value"] * ff + fo) - to) / tf)
        out["conversions"] = res
    if "numbers" in req:
        out["numbers"] = [number_text(n["value"], n["precision"], n["locale"]) for n in req["numbers"]]
    if "parses" in req:
        res = []
        for t in req["parses"]:
            p = parse(t)
            if p is not None and p["unit"] is not None:
                p["factor"], p["offset"], _ = unit(p["unit"]) if is_symbol(p["unit"]) else (None, None, None)
            res.append(p)
        out["parses"] = res
    json.dump(out, sys.stdout, ensure_ascii=False)


def is_symbol(text):
    try:
        unit(text)
        return True
    except (ValueError, AssertionError, KeyError):
        return False


if __name__ == "__main__":
    main()
