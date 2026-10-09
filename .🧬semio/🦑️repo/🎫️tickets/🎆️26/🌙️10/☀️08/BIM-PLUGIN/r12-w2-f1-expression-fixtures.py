"""Writes the schema-first contracts and the language-agnostic fixtures of `semio-framework-expression`.

Outputs, under the module `🧮️expression` (names are discovered from sibling modules, never typed):
  <domain>/🧬️schema/🔣️.json        draft-07 schemas (tree = the AST contract; the others describe the fixtures)
  🌳️tree/🧬️schema/{🟦️.ts,🛰️.proto,🔗️.graphql}   the other facets of the AST
  🧫️fixtures/<domain>/🔣️.json      cases; every expectation is computed by the Python oracle in 🧪️tests/📜️conformance/🐍️.py

Run from the repo root: `.venv/Scripts/python.exe <this file>`; deterministic (seeded).
"""
import importlib.util
import json
import math
import random
import re
from decimal import Decimal
from pathlib import Path

REPO = Path(__file__).resolve().parents[7]


def child(parent, suffix):
    return next(p for p in parent.iterdir() if p.name.endswith(suffix))


FRAMEWORK = next(p for p in REPO.iterdir() if p.name.endswith("framework"))
MODULES = child(FRAMEWORK, "modules")
MODULE = child(MODULES, "expression")
VALUE_SCHEMA = child(child(MODULES, "value"), "schema")
SCHEMA_DIR = VALUE_SCHEMA.name
JSON_NAME = next(child(child(MODULES, "geometry"), "fixtures").glob("*/*.json")).name
TS_NAME = next(p.name for p in VALUE_SCHEMA.iterdir() if p.name.endswith(".ts"))
PROTO_NAME = next(p.name for p in VALUE_SCHEMA.iterdir() if p.name.endswith(".proto"))
GRAPHQL_NAME = next(p.name for p in VALUE_SCHEMA.iterdir() if p.name.endswith(".graphql"))
FIXTURES_NAME = child(child(MODULES, "geometry"), "fixtures").name
DOMAIN = {name: child(MODULE, name) for name in ("tree", "kinds", "evaluation", "parameters", "syntax")}
ORACLE_FILE = child(child(child(MODULE, "tests"), "conformance"), ".py")

spec = importlib.util.spec_from_file_location("expression_oracle", ORACLE_FILE)
oracle = importlib.util.module_from_spec(spec)
spec.loader.exec_module(oracle)

BASE = "https://semio.tech/framework/expression/"
LENGTH_UNITS = ["mm", "cm", "m", "km", "in", "ft"]
AREA_UNITS = ["mm2", "cm2", "m2"]
VOLUME_UNITS = ["mm3", "cm3", "m3", "l"]
ANGLE_UNITS = ["deg", "rad"]
KINDS = ["number", "length", "angle", "area", "volume", "bool", "text"]
KEYWORDS = {"if", "then", "else", "and", "or", "not", "true", "false"}
ALIAS = {"°": "deg", "m²": "m2", "m³": "m3", "cm²": "cm2", "cm³": "cm3", "mm²": "mm2", "mm³": "mm3"}
ALL_UNITS = set(LENGTH_UNITS + AREA_UNITS + VOLUME_UNITS + ANGLE_UNITS) | set(ALIAS)
UNIT_KIND = {**{u: "length" for u in LENGTH_UNITS}, **{u: "area" for u in AREA_UNITS}, **{u: "volume" for u in VOLUME_UNITS}, **{u: "angle" for u in ANGLE_UNITS}}


def num(v):
    return {"op": "number", "value": float(v)}


def qty(kind, v, unit):
    return {"op": kind, "value": float(v), "unit": unit}


def boolean(v):
    return {"op": "bool", "value": v}


def text(v):
    return {"op": "text", "value": v}


def par(name):
    return {"op": "param", "name": name}


def neg(x):
    return {"op": "unary", "operator": "negate", "operand": x}


def inv(x):
    return {"op": "unary", "operator": "not", "operand": x}


def binary(op, left, right):
    return {"op": "binary", "operator": op, "left": left, "right": right}


def compare(op, left, right):
    return {"op": "compare", "operator": op, "left": left, "right": right}


def conj(left, right):
    return {"op": "and", "left": left, "right": right}


def disj(left, right):
    return {"op": "or", "left": left, "right": right}


def cond(c, t, e):
    return {"op": "if", "condition": c, "then": t, "otherwise": e}


def call(function, *arguments):
    return {"op": "call", "function": function, "arguments": list(arguments)}


def length(v, u="m"):
    return qty("length", v, u)


def angle(v, u="deg"):
    return qty("angle", v, u)


def area(v, u="m2"):
    return qty("area", v, u)


def volume(v, u="m3"):
    return qty("volume", v, u)


def children(node):
    op = node["op"]
    if op == "unary":
        return ["operand"]
    if op in ("binary", "compare", "and", "or"):
        return ["left", "right"]
    if op == "if":
        return ["condition", "then", "otherwise"]
    return []


def map_children(node, fn):
    node = dict(node)
    if node["op"] == "call":
        node["arguments"] = [fn(i, a) for i, a in enumerate(node["arguments"])]
    for index, key in enumerate(children(node)):
        node[key] = fn(index, node[key])
    return node


def paths(node, prefix=()):
    yield list(prefix), node
    if node["op"] == "call":
        for i, a in enumerate(node["arguments"]):
            yield from paths(a, prefix + (i,))
    for i, key in enumerate(children(node)):
        yield from paths(node[key], prefix + (i,))


def replace_at(node, path, new):
    if not path:
        return new
    head, rest = path[0], path[1:]
    return map_children(node, lambda i, c: replace_at(c, rest, new) if i == head else c)


def number_text(v):
    if v != 0 and (abs(v) >= 1e16 or abs(v) < 1e-6):
        parts = Decimal(repr(v)).as_tuple()
        digits = "".join(map(str, parts.digits))
        point = parts.exponent + len(digits) - 1
        digits = digits.rstrip("0") or "0"
        return digits[0] + ("." + digits[1:] if len(digits) > 1 else "") + f"e{point}"
    plain = format(Decimal(repr(v)), "f")
    if "." in plain:
        plain = plain.rstrip("0").rstrip(".")
    return plain


PLAIN = re.compile(r"^[^\W\d]\w*$")


def escaped(value, quote):
    out = quote
    for c in value:
        if c == "\\":
            out += "\\\\"
        elif c == "\n" and quote == '"':
            out += "\\n"
        elif c == "\t" and quote == '"':
            out += "\\t"
        elif c == quote:
            out += "\\" + c
        else:
            out += c
    return out + quote


def name_text(name):
    return name if PLAIN.match(name) and name not in KEYWORDS else escaped(name, "`")


def bp(node):
    op = node["op"]
    if op == "if":
        return 0
    if op == "or":
        return 10
    if op == "and":
        return 20
    if op == "unary":
        return 30 if node["operator"] == "not" else 70
    if op == "compare":
        return 40
    if op == "binary":
        return {"add": 50, "subtract": 50, "multiply": 60, "divide": 60, "power": 80}.get(node["operator"], 100)
    return 100


def canonical(node):
    op = node["op"]

    def wrap(child, minimum):
        inner = canonical(child)
        return f"({inner})" if bp(child) < minimum else inner

    if op == "number":
        return number_text(node["value"])
    if op in ("length", "angle", "area", "volume"):
        return f"{number_text(node['value'])} {node['unit']}"
    if op == "bool":
        return "true" if node["value"] else "false"
    if op == "text":
        return escaped(node["value"], '"')
    if op == "param":
        return name_text(node["name"])
    if op == "unary":
        return ("-" + wrap(node["operand"], 71)) if node["operator"] == "negate" else "not " + wrap(node["operand"], 30)
    if op == "binary":
        operator = node["operator"]
        if operator in ("min", "max"):
            return f"{operator}({canonical(node['left'])}, {canonical(node['right'])})"
        power = operator == "power"
        b = bp(node)
        left = wrap(node["left"], b + 1 if power else b)
        right = wrap(node["right"], b if power else b + 1)
        return f"{left} {oracle.SYMBOL[operator]} {right}"
    if op == "compare":
        return f"{wrap(node['left'], 41)} {oracle.SYMBOL[node['operator']]} {wrap(node['right'], 41)}"
    if op in ("and", "or"):
        b = bp(node)
        return f"{wrap(node['left'], b)} {op} {wrap(node['right'], b + 1)}"
    if op == "if":
        return f"if {canonical(node['condition'])} then {canonical(node['then'])} else {canonical(node['otherwise'])}"
    return f"{node['function']}({', '.join(canonical(a) for a in node['arguments'])})"


class Verbose:
    """Fully parenthesised rendering with every alias spelling, so that parsing it does not depend on precedence."""

    def __init__(self):
        self.tick = 0

    def flip(self):
        self.tick += 1
        return self.tick % 2 == 0

    def atom(self, node):
        return node["op"] in ("number", "length", "angle", "area", "volume", "bool", "text", "param") or (node["op"] == "call") or (node["op"] == "binary" and node["operator"] in ("min", "max"))

    def sub(self, node):
        inner = self.render(node)
        return inner if self.atom(node) else f"({inner})"

    def render(self, node):
        op = node["op"]
        if op in ("length", "angle", "area", "volume"):
            unit = node["unit"]
            if self.flip():
                unit = {"deg": "°", "m2": "m²", "m3": "m³"}.get(unit, unit)
            gap = " " if self.flip() else ""
            return f"{number_text(node['value'])}{gap}{unit}"
        if op in ("number", "bool", "text", "param"):
            return canonical(node)
        if op == "unary":
            return ("-" if node["operator"] == "negate" else "not ") + self.sub(node["operand"])
        if op == "binary":
            operator = node["operator"]
            if operator in ("min", "max"):
                return f"{operator}({self.render(node['left'])}, {self.render(node['right'])})"
            symbol = oracle.SYMBOL[operator]
            if operator == "multiply" and self.flip():
                symbol = "×"
            elif operator == "divide" and self.flip():
                symbol = "÷"
            elif operator == "subtract" and self.flip():
                symbol = "−"
            return f"{self.sub(node['left'])} {symbol} {self.sub(node['right'])}"
        if op == "compare":
            symbol = oracle.SYMBOL[node["operator"]]
            if self.flip():
                symbol = {"=": "==", "!=": "≠", "<=": "≤", ">=": "≥"}.get(symbol, symbol)
            return f"{self.sub(node['left'])} {symbol} {self.sub(node['right'])}"
        if op in ("and", "or"):
            return f"{self.sub(node['left'])} {op} {self.sub(node['right'])}"
        if op == "if":
            return f"if {self.sub(node['condition'])} then {self.sub(node['then'])} else {self.sub(node['otherwise'])}"
        return f"{node['function']}({', '.join(self.render(a) for a in node['arguments'])})"


def python_text(node):
    """Fully parenthesised Python rendering of a tree; units become `Q(value, 'unit')` and parameters `P('name')`."""
    op = node["op"]
    p = python_text
    if op == "number":
        return repr(float(node["value"]))
    if op in ("length", "angle", "area", "volume"):
        return f"Q({float(node['value'])!r}, {node['unit']!r})"
    if op == "bool":
        return "True" if node["value"] else "False"
    if op == "text":
        return repr(node["value"])
    if op == "param":
        return f"P({node['name']!r})"
    if op == "unary":
        return f"(-({p(node['operand'])}))" if node["operator"] == "negate" else f"(not ({p(node['operand'])}))"
    if op == "binary":
        operator = node["operator"]
        if operator in ("min", "max"):
            return f"{operator}({p(node['left'])}, {p(node['right'])})"
        symbol = {"add": "+", "subtract": "-", "multiply": "*", "divide": "/", "power": "**"}[operator]
        return f"(({p(node['left'])}) {symbol} ({p(node['right'])}))"
    if op == "compare":
        symbol = {"equal": "==", "not-equal": "!=", "less": "<", "less-equal": "<=", "greater": ">", "greater-equal": ">="}[node["operator"]]
        return f"(({p(node['left'])}) {symbol} ({p(node['right'])}))"
    if op in ("and", "or"):
        return f"(({p(node['left'])}) {op} ({p(node['right'])}))"
    if op == "if":
        return f"(({p(node['then'])}) if ({p(node['condition'])}) else ({p(node['otherwise'])}))"
    return f"{node['function']}({', '.join(p(a) for a in node['arguments'])})"


NUMBER = re.compile(r"(?:\d+\.?\d*|\.\d+)(?:[eE][+-]?\d+)?")
IDENT = re.compile(r"[^\W\d]\w*")


def translate(source):
    """Token-level translation of Semio syntax (without `if`) to Python, so Python's own parser decides precedence."""
    out, i = [], 0
    while i < len(source):
        c = source[i]
        if c.isspace():
            i += 1
        elif NUMBER.match(source, i):
            m = NUMBER.match(source, i)
            value = float(m.group())
            i = m.end()
            j = i
            while j < len(source) and source[j].isspace():
                j += 1
            unit = None
            if j < len(source) and source[j] == "°":
                unit, i = "deg", j + 1
            else:
                word = IDENT.match(source, j)
                if word and word.group() in ALL_UNITS:
                    unit, i = ALIAS.get(word.group(), word.group()), word.end()
            out.append(f"Q({value!r}, {unit!r})" if unit else repr(value))
        elif IDENT.match(source, i):
            m = IDENT.match(source, i)
            word = m.group()
            i = m.end()
            rest = source[i:].lstrip()
            if word in ("and", "or", "not"):
                out.append(word)
            elif word in ("true", "false"):
                out.append(word.capitalize())
            elif rest.startswith("("):
                out.append(word)
            else:
                out.append(f"P({word!r})")
        elif c == "`":
            end = source.index("`", i + 1)
            out.append(f"P({source[i + 1:end]!r})")
            i = end + 1
        elif c == '"':
            end = source.index('"', i + 1)
            out.append(repr(source[i + 1:end]))
            i = end + 1
        else:
            two = source[i:i + 2]
            mapping = {"==": "==", "!=": "!=", "<=": "<=", ">=": ">="}
            if two in mapping:
                out.append(mapping[two])
                i += 2
                continue
            out.append({"^": "**", "=": "==", "≠": "!=", "≤": "<=", "≥": ">=", "×": "*", "÷": "/", "−": "-"}.get(c, c))
            i += 1
    return " ".join(out)


def tree_from_source(source):
    return oracle.to_tree(oracle.parse_python(translate(source)))


ENV = {
    "w": ("length", 2.4),
    "h": ("length", 0.9),
    "a": ("angle", 30 * math.pi / 180.0),
    "n": ("number", 3.0),
    "zero": ("number", 0.0),
    "area": ("area", 2.16),
    "vol": ("volume", 1.296),
    "f": ("bool", True),
    "g": ("bool", False),
    "s": ("text", "oak"),
}
ENV_ROWS = {k: {"kind": kind, "value": value} for k, (kind, value) in ENV.items()}
KIND_ROWS = {k: kind for k, (kind, _) in ENV.items()}

LITERAL_NUMBERS = [0.0, 1.0, 2.0, 3.0, 0.5, 10.0, 100.0, 7.25, 0.25, 12.0]
LITERAL_LENGTHS = [(2.4, "m"), (90, "mm"), (250, "cm"), (1.5, "km"), (10, "in"), (3, "ft"), (0.5, "m"), (1200, "mm"), (4, "m")]
LITERAL_ANGLES = [(30, "deg"), (90, "deg"), (45, "deg"), (1.5, "rad"), (180, "deg"), (60, "deg")]
LITERAL_AREAS = [(2, "m2"), (10000, "cm2"), (1.5, "m2"), (250000, "mm2")]
LITERAL_VOLUMES = [(1, "m3"), (500, "l"), (2.5, "m3"), (1000000, "cm3")]
COMPARE_OPS = ["equal", "not-equal", "less", "less-equal", "greater", "greater-equal"]


class Generator:
    def __init__(self, seed):
        self.rng = random.Random(seed)

    def pick(self, items):
        return self.rng.choice(items)

    def literal(self, kind):
        r = self.rng
        if kind == "number":
            return num(self.pick(LITERAL_NUMBERS))
        if kind == "length":
            return length(*self.pick(LITERAL_LENGTHS))
        if kind == "angle":
            return angle(*self.pick(LITERAL_ANGLES))
        if kind == "area":
            return area(*self.pick(LITERAL_AREAS))
        if kind == "volume":
            return volume(*self.pick(LITERAL_VOLUMES))
        if kind == "bool":
            return boolean(r.random() < 0.5)
        return text(self.pick(["oak", "pine", "", "a b", "ß"]))

    def gen(self, kind, depth, pool):
        r = self.rng
        names = [n for n, k in pool.items() if k == kind]
        if depth <= 0 or r.random() < 0.18:
            if names and r.random() < 0.55:
                return par(self.pick(names))
            return self.literal(kind)
        d = depth - 1
        g = lambda k: self.gen(k, d, pool)
        if r.random() < 0.08:
            return cond(g("bool"), g(kind), g(kind))
        if kind == "number":
            c = r.randrange(11)
            if c == 0:
                return binary(self.pick(["add", "subtract", "multiply", "min", "max"]), g("number"), g("number"))
            if c == 1:
                return binary("divide", g("number"), g("number"))
            if c == 2:
                k = self.pick(["length", "angle", "area", "volume", "number"])
                return binary("divide", g(k), g(k))
            if c == 3:
                return binary("power", g("number"), num(self.pick([2.0, 3.0, 0.5, 1.0, 10.0])))
            if c == 4:
                return call(self.pick(["sqrt", "abs", "round", "floor", "ceil"]), g("number"))
            if c == 5:
                return call(self.pick(["sin", "cos", "tan"]), g("angle"))
            if c == 6:
                return neg(g("number"))
            if c == 7:
                return binary("multiply", g("number"), g("number"))
            if c == 8:
                return call("round", binary("divide", g("length"), length(self.pick([10, 50, 100]), "mm")))
            if c == 9:
                return binary("divide", g("length"), g("length"))
            return binary("add", g("number"), num(self.pick(LITERAL_NUMBERS)))
        if kind == "length":
            c = r.randrange(9)
            if c == 0:
                return binary(self.pick(["add", "subtract", "min", "max"]), g("length"), g("length"))
            if c == 1:
                return binary("multiply", g("number"), g("length"))
            if c == 2:
                return binary("multiply", g("length"), g("number"))
            if c == 3:
                return binary("divide", g("length"), g("number"))
            if c == 4:
                return binary("divide", g("area"), g("length"))
            if c == 5:
                return call("sqrt", g("area"))
            if c == 6:
                return call("abs", g("length"))
            if c == 7:
                return binary("divide", g("volume"), g("area"))
            return neg(g("length"))
        if kind == "angle":
            c = r.randrange(6)
            if c == 0:
                return binary(self.pick(["add", "subtract", "min", "max"]), g("angle"), g("angle"))
            if c == 1:
                return binary("multiply", g("number"), g("angle"))
            if c == 2:
                return binary("divide", g("angle"), g("number"))
            if c == 3:
                k = self.pick(["number", "length"])
                return call("atan2", g(k), g(k))
            if c == 4:
                return call("abs", g("angle"))
            return neg(g("angle"))
        if kind == "area":
            c = r.randrange(6)
            if c == 0:
                return binary(self.pick(["add", "subtract", "min", "max"]), g("area"), g("area"))
            if c == 1:
                return binary("multiply", g("length"), g("length"))
            if c == 2:
                return binary("power", g("length"), num(2.0))
            if c == 3:
                return binary("multiply", g("number"), g("area"))
            if c == 4:
                return binary("divide", g("volume"), g("length"))
            return binary("divide", g("area"), g("number"))
        if kind == "volume":
            c = r.randrange(5)
            if c == 0:
                return binary(self.pick(["add", "subtract", "min", "max"]), g("volume"), g("volume"))
            if c == 1:
                return binary("multiply", g("length"), g("area"))
            if c == 2:
                return binary("multiply", g("area"), g("length"))
            if c == 3:
                return binary("power", g("length"), num(3.0))
            return binary("multiply", g("number"), g("volume"))
        if kind == "bool":
            c = r.randrange(5)
            if c == 0:
                k = self.pick(["number", "length", "angle", "area", "volume"])
                return compare(self.pick(COMPARE_OPS), g(k), g(k))
            if c == 1:
                return conj(g("bool"), g("bool"))
            if c == 2:
                return disj(g("bool"), g("bool"))
            if c == 3:
                return inv(g("bool"))
            k = self.pick(["bool", "text"])
            return compare(self.pick(["equal", "not-equal"]), g(k), g(k))
        return cond(g("bool"), g("text"), g("text"))

    def spoil(self, node, pool):
        """Replaces one random node with an expression of another kind, producing ill-kinded trees."""
        target = self.rng.choice(list(paths(node)))
        other = self.gen(self.pick(KINDS), 1, pool)
        return replace_at(node, target[0], other)


def uses_only_env(tree):
    return all(node["name"] in ENV for _, node in paths(tree) if node["op"] == "param")


def kinds_case(name, source, python, params):
    return {"name": name, "source": source, "python": python, "params": params, "expected": oracle.kind_only(python, params)}


def evaluation_case(name, source, python, env):
    return {"name": name, "source": source, "python": python, "env": env, "expected": oracle.run(python, env)}


CURATED = [
    "1 + 2 * 3", "(1 + 2) * 3", "10 - 4 - 3", "100 / 10 / 5", "2 ^ 3 ^ 2", "-2 ^ 2", "(-2) ^ 2", "2 ^ -1", "2 ^ -3 ^ 2", "-3 * -2", "1 - -1", "- - n",
    "n * 2 + w / h", "2 * n ^ 2", "-n * 2", "a * 2 - a / 2", "w ^ 2 + area", "w * h * w", "area / w", "vol / area", "vol / w / h",
    "w + 90 mm", "2.4 m + 90 mm", "2.4m + 90mm", "250 cm - 0.5 m", "1 km / 1000 m", "10 in + 1 ft", "30 deg + 45°", "90 deg - 1.5 rad", "180 deg / 2",
    "2 m ^ 2", "3 m² + 2 m2", "500 l + 1 m³", "2 m * 3 m * 4 m", "area ^ 0.5", "w ^ 3", "4 m2 ^ 0.5",
    "sqrt(16)", "sqrt(4 m2)", "sqrt(area)", "abs(-2 m)", "abs(-w)", "round(2.5)", "round(-2.5)", "round(2.4)", "floor(-2.5)", "ceil(2.1)", "round(w / 100 mm) * 100 mm", "ceil(w / 1 m)",
    "sin(90 deg)", "cos(60 deg)", "tan(45 deg)", "sin(a)", "cos(a) ^ 2 + sin(a) ^ 2", "atan2(1 m, 1 m)", "atan2(1, -1)", "atan2(w, h) * 2", "atan2(h, w) / n",
    "min(w, h)", "max(w, h, 3 m)", "min(3, 1, 2)", "max(min(w, 1 m), 50 cm)", "min(area, 3 m2)", "max(a, 1 rad)",
    "w = 2.4 m", "w = 240 cm", "w != h", "w < 3 m", "w <= 2.4 m", "w > h", "w >= 3 m", "0.1 + 0.2 = 0.3", "0.1 + 0.2 < 0.3", "0.1 + 0.2 <= 0.3", "1 m = 1.0000001 m", "n = 3", "2 m >= 200 cm",
    "s = \"oak\"", "s != \"oak\"", "s = \"pine\"", "f = true", "f = g", "f != g",
    "f and g", "f or g", "not f", "not g", "f and not g", "f or g and g", "f and g or f", "not f or g", "not f and g", "not w = h", "not (w = h)", "f and g and f", "g or g or f",
    "w > h and n > 2", "w < h or n = 3", "(w > h) = (n > 2)", "w + h > 3 m and not g",
    "zero != 0 and 1 / zero > 2", "zero = 0 or 1 / zero > 2", "n / zero", "1 / 0", "w / 0", "1 / (n - 3)", "0 ^ -1", "(-8) ^ 0.5", "sqrt(-1)", "sqrt(0 - area)", "10 ^ 400", "1e308 * 10",
    "1e-7 * 1e7", "1.5e3 m", "0.000001 m", "123456.789 mm", "1e20 + 1", "n ^ 2 ^ 2", "(n ^ 2) ^ 2",
    "w + a", "w + 1", "w * a", "area * area", "2 / w", "w / a", "w ^ 4", "w ^ n", "w ^ -1", "w ^ h", "f ^ 2", "-f", "not n", "f < true", "w < s", "w = 2", "n and f", "f or n", "sqrt(w)", "sin(n)", "round(w)", "abs(f)", "atan2(w, a)", "atan2(f, a)", "missing + 1", "w + (missing * 2)", "(w + a) * (h + n)", "min(w, a)", "max(f, g)",
    "w + h + w + h", "(((n)))", "((w + h) * (w - h)) / (w + h)", "n - 1 - 1 - 1", "n / 2 / 2 / 2", "2 * 3 + 4 * 5 - 6 / 2", "-(n + 1) * 2", "-(w + h)", "--n", "n * -1", "n - - - 1",
]
HAND_TREES = [
    ("if f then w else h", cond(par("f"), par("w"), par("h"))),
    ("if f then 1 else 2 + 3", cond(par("f"), num(1), binary("add", num(2), num(3)))),
    ("(if f then 1 else 2) + 3", binary("add", cond(par("f"), num(1), num(2)), num(3))),
    ("1 + (if f then 2 else 3)", binary("add", num(1), cond(par("f"), num(2), num(3)))),
    ("if f then if g then 1 else 2 else 3", cond(par("f"), cond(par("g"), num(1), num(2)), num(3))),
    ("if f then 1 else if g then 2 else 3", cond(par("f"), num(1), cond(par("g"), num(2), num(3)))),
    ("if w > h then w else h", cond(compare("greater", par("w"), par("h")), par("w"), par("h"))),
    ("if zero = 0 then 1 else 1 / zero", cond(compare("equal", par("zero"), num(0)), num(1), binary("divide", num(1), par("zero")))),
    ("if zero != 0 then 1 / zero else 7", cond(compare("not-equal", par("zero"), num(0)), binary("divide", num(1), par("zero")), num(7))),
    ("if f then w else a", cond(par("f"), par("w"), par("a"))),
    ("if n then 1 else 2", cond(par("n"), num(1), num(2))),
    ("sqrt(if f then 4 else 9)", call("sqrt", cond(par("f"), num(4), num(9)))),
    ("min(if f then 1 else 2, 3)", binary("min", cond(par("f"), num(1), num(2)), num(3))),
    ("if f then \"yes\" else \"no\"", cond(par("f"), text("yes"), text("no"))),
    ("if not f then 1 else 2", cond(inv(par("f")), num(1), num(2))),
    ("(if f then g else f) and g", conj(cond(par("f"), par("g"), par("f")), par("g"))),
    ("if f and g or f then 1 m else 2 m", cond(disj(conj(par("f"), par("g")), par("f")), length(1), length(2))),
    ("`frame width` * 2", binary("multiply", par("frame width"), num(2))),
    ("`if` + `true` + `and`", binary("add", binary("add", par("if"), par("true")), par("and"))),
    ("Höhe + 1", binary("add", par("Höhe"), num(1))),
    ("\"a\\\"b\\\\c\\nd\" = \"x\"", compare("equal", text("a\"b\\c\nd"), text("x"))),
    ("(a < b) = (c < d)", compare("equal", compare("less", par("a"), par("b")), compare("less", par("c"), par("d")))),
    ("(f = g) = f", compare("equal", compare("equal", par("f"), par("g")), par("f"))),
    ("not (f and g)", inv(conj(par("f"), par("g")))),
    ("(not f) = g", compare("equal", inv(par("f")), par("g"))),
    ("f and (g and f)", conj(par("f"), conj(par("g"), par("f")))),
    ("f or (g or f)", disj(par("f"), disj(par("g"), par("f")))),
    ("(f or g) and f", conj(disj(par("f"), par("g")), par("f"))),
    ("-(a ^ b)", neg(binary("power", par("a"), par("b")))),
    ("a ^ (b ^ c)", binary("power", par("a"), binary("power", par("b"), par("c")))),
    ("(a ^ b) ^ c", binary("power", binary("power", par("a"), par("b")), par("c"))),
    ("a - (b - c)", binary("subtract", par("a"), binary("subtract", par("b"), par("c")))),
    ("a / (b / c)", binary("divide", par("a"), binary("divide", par("b"), par("c")))),
    ("-(-a)", neg(neg(par("a")))),
    ("-(a * b)", neg(binary("multiply", par("a"), par("b")))),
]
ERRORS = [
    ("unexpected character", "1 + #", "unexpected-character", 4, 5),
    ("lone bang", "a ! b", "unexpected-character", 2, 3),
    ("number overflow", "1e999", "invalid-number", 0, 5),
    ("unterminated text", "\"abc", "unterminated-text", 0, 4),
    ("unterminated name", "`abc", "unterminated-name", 0, 4),
    ("empty name", "``", "empty-name", 0, 2),
    ("bad escape in text", "\"a\\q\"", "invalid-escape", 2, 4),
    ("bad escape in name", "`a\\n`", "invalid-escape", 2, 4),
    ("missing operand at end", "1 +", "unexpected-end", 3, 3),
    ("empty source", "", "unexpected-end", 0, 0),
    ("missing closing parenthesis", "(1 + 2", "unexpected-end", 6, 6),
    ("missing else", "if f then 1", "unexpected-end", 11, 11),
    ("missing then", "if f 1", "unexpected-token", 5, 6),
    ("operator where operand expected", "1 + * 2", "unexpected-token", 4, 5),
    ("two operands", "1 2", "unexpected-token", 2, 3),
    ("stray closing parenthesis", "1 + 2)", "unexpected-token", 5, 6),
    ("keyword as operand", "and", "unexpected-token", 0, 3),
    ("missing comma", "atan2(1 2)", "unexpected-token", 8, 9),
    ("unknown unit", "2 foo", "unknown-unit", 2, 5),
    ("unknown unit after expression", "w + 3 yd", "unknown-unit", 6, 8),
    ("unknown function", "1 + frob(2)", "unknown-function", 4, 8),
    ("too many arguments", "sqrt(1, 2)", "argument-count", 0, 10),
    ("too few arguments", "atan2(1)", "argument-count", 0, 8),
    ("no arguments", "sin()", "argument-count", 0, 5),
    ("min with one argument", "min(1)", "argument-count", 0, 6),
    ("chained comparison", "a < b < c", "chained-comparison", 6, 7),
    ("chained equality", "a = b = c", "chained-comparison", 6, 7),
    ("non-ascii position", "Höhe + #", "unexpected-character", 7, 8),
]


def syntax_fixture():
    cases = []
    for source in CURATED:
        if "if " in source:
            continue
        python = translate(source)
        tree = oracle.to_tree(oracle.parse_python(python))
        cases.append({"name": f"natural: {source}", "source": source, "canonical": canonical(tree), "python": python, "ast": tree})
    for source, tree in HAND_TREES:
        cases.append({"name": f"hand: {source}", "source": source, "canonical": canonical(tree), "python": python_text(tree), "ast": tree})
    generator = Generator(7)
    for index in range(150):
        pool = {n: k for n, k in KIND_ROWS.items()}
        tree = generator.gen(generator.pick(KINDS), 1 + index % 5, pool)
        if index % 3 == 0:
            tree = generator.spoil(tree, pool)
        verbose = Verbose()
        cases.append({"name": f"generated {index}", "source": verbose.render(tree), "canonical": canonical(tree), "python": python_text(tree), "ast": tree})
    cases.extend(
        {"name": f"unit {u}", "source": f"2.5 {u}", "canonical": canonical(qty(UNIT_KIND[u], 2.5, u)), "python": python_text(qty(UNIT_KIND[u], 2.5, u)), "ast": qty(UNIT_KIND[u], 2.5, u)}
        for u in LENGTH_UNITS + AREA_UNITS + VOLUME_UNITS + ANGLE_UNITS
    )
    for number in (0.0, 1.0, 1e-7, 1.5e-9, 1e15, 1e16, 1.2345678901234567e20, 1e300, 0.000001, 123456789.125, 0.1 + 0.2, 5e-324):
        tree = num(number)
        cases.append({"name": f"number {number!r}", "source": number_text(number), "canonical": number_text(number), "python": python_text(tree), "ast": tree})
    return {"cases": cases, "errors": [{"name": n, "source": s, "code": c, "start": a, "end": b} for n, s, c, a, b in ERRORS]}


def kinds_fixture():
    cases = []
    for source in CURATED:
        if "if " in source:
            continue
        cases.append(kinds_case(source, source, translate(source), KIND_ROWS))
    for source, tree in HAND_TREES:
        if uses_only_env(tree):
            cases.append(kinds_case(source, source, python_text(tree), KIND_ROWS))
    generator = Generator(11)
    for index in range(160):
        tree = generator.gen(generator.pick(KINDS), 1 + index % 5, KIND_ROWS)
        if index % 3 == 0:
            tree = generator.spoil(tree, KIND_ROWS)
        cases.append(kinds_case(f"generated {index}", canonical(tree), python_text(tree), KIND_ROWS))
    extra = {"x": "length", "y": "length", "z": "angle"}
    cases.append(kinds_case("other parameter set", "x * y + x ^ 2", translate("x * y + x ^ 2"), extra))
    cases.append(kinds_case("other parameter set mismatch", "x + z", translate("x + z"), extra))
    return {"cases": cases}


def evaluation_fixture():
    cases = []
    for source in CURATED:
        if "if " in source:
            continue
        cases.append(evaluation_case(source, source, translate(source), ENV_ROWS))
    for source, tree in HAND_TREES:
        if uses_only_env(tree):
            cases.append(evaluation_case(source, source, python_text(tree), ENV_ROWS))
    generator = Generator(13)
    for index in range(260):
        tree = generator.gen(generator.pick(KINDS), 1 + index % 5, KIND_ROWS)
        if index % 5 == 0:
            tree = generator.spoil(tree, KIND_ROWS)
        cases.append(evaluation_case(f"generated {index}", canonical(tree), python_text(tree), ENV_ROWS))
    return {"cases": cases}


def pset(**formulas):
    return {name: {"source": source, "python": translate(source)} for name, source in formulas.items()}


def pcase(name, params, overrides=None, declared=None):
    overrides = overrides or {}
    declared = declared or {}
    expected = oracle.resolve({n: p["python"] for n, p in params.items()}, {n: p["python"] for n, p in overrides.items()}, declared)
    return {"name": name, "params": params, "overrides": overrides, "declared": declared, "expected": expected}


def parameters_fixture():
    cases = [
        pcase("empty", {}),
        pcase("single literal", pset(width="2.4 m")),
        pcase("chain", pset(width="2.4 m", height="width / 2", area="width * height", half="area / 2", big="area > 2 m2")),
        pcase("diamond", pset(a="1", b="a + 1", c="a + 2", d="b * c")),
        pcase("sorted by name within a layer", pset(z="1", m="2", a="3", total="z + m + a")),
        pcase("layers follow the longest chain", pset(a="1", b="a", c="b", d="a + c", e="a")),
        pcase("reference to an absent name", pset(a="missing + 1", b="a")),
        pcase("self reference", pset(a="a + 1", b="1")),
        pcase("two cycle", pset(a="b + 1", b="a + 1", c="a * 2", d="c", e="5")),
        pcase("three cycle with a tail", pset(a="b", b="c", c="a", d="c + 1", g="d + 1", f="1")),
        pcase("two overlapping cycles are one component", pset(a="b", b="a + c", c="b")),
        pcase("separate cycles", pset(a="b", b="a", c="d", d="c", e="1")),
        pcase("division by zero blocks dependants", pset(zero="0", bad="1 / zero", later="bad + 1", indep="zero + 5")),
        pcase("first failed dependency by name", pset(x="1 / 0", y="1 / 0", z="y + x")),
        pcase("kind error in a formula", pset(w="2 m", t="30 deg", bad="w + t", ok="w * 2")),
        pcase("unknown parameter", pset(a="typo * 2", b="1")),
        pcase("override replaces the formula", pset(width="2 m", double="width * 2"), pset(width="3 m")),
        pcase("override with a different kind propagates", pset(width="2 m", double="width * 2", sum="width + 1 m"), pset(width="30 deg")),
        pcase("override of a missing parameter", pset(a="1"), pset(b="2")),
        pcase("override breaking a cycle", pset(a="b + 1", b="a + 1", c="a"), pset(a="10")),
        pcase("override creating a cycle", pset(a="1", b="a + 1"), pset(a="b + 1")),
        pcase("override adding a dependency order", pset(a="1", b="2", c="a + b"), pset(a="b + 1")),
        pcase("declared kinds hold", pset(w="2 m", h="1 m", a="w * h"), declared={"w": "length", "h": "length", "a": "area"}),
        pcase("declared kind mismatch", pset(w="2 m", n="w / 1 m", m="n * 2"), declared={"w": "length", "n": "length"}),
        pcase("declared kind for a failing parameter", pset(a="1 / 0", b="2"), declared={"a": "number"}),
        pcase("conditional parameters", pset(wide="w > 2 m", w="2.4 m", pick="w * 2 + w"), None),
        pcase("lazy branch still resolves", pset(zero="0", safe="zero != 0 and 1 / zero > 1", other="zero = 0 or 1 / zero > 1")),
        pcase("text and bool parameters", pset(name="\"oak\"", same="name = \"oak\"", flag="not same")),
        pcase("trigonometry parameters", pset(turn="90 deg", s="sin(turn)", c="cos(turn)", t="atan2(s, c)")),
        pcase("rounding to a grid", pset(span="2.43 m", cells="round(span / 100 mm)", snapped="cells * 100 mm")),
    ]
    generator = Generator(17)
    for index in range(30):
        count = 4 + index % 7
        names = [f"p{i}" for i in range(count)]
        kinds = {n: generator.pick(["number", "length", "angle", "area", "bool"]) for n in names}
        params = {}
        for i, n in enumerate(names):
            earlier = {m: kinds[m] for m in names[:i]}
            pool = dict(earlier)
            if generator.rng.random() < 0.18:
                pool.update({m: kinds[m] for m in names[i:]})
            tree = generator.gen(kinds[n], 2, pool)
            python = python_text(tree)
            params[n] = {"source": canonical(tree), "python": python}
        overrides = {}
        if index % 3 == 0:
            target = generator.pick(names)
            replacement = generator.gen(kinds[target], 1, {m: kinds[m] for m in names if m != target})
            overrides[target] = {"source": canonical(replacement), "python": python_text(replacement)}
        declared = {n: kinds[n] for n in names if generator.rng.random() < 0.3}
        cases.append(pcase(f"generated {index}", params, overrides, declared))
    return {"cases": cases}


def number_schema():
    return {"type": "number", "minimum": 0}


def tree_schema():
    expr = {"$ref": "#/definitions/Expr"}

    def node(op, properties, required=None):
        props = {"op": {"const": op}, **properties}
        return {"type": "object", "required": ["op", *(required or properties)], "additionalProperties": False, "properties": props}

    unit = lambda units: {"enum": units}
    definitions = {
        "LengthUnit": unit(LENGTH_UNITS),
        "AngleUnit": unit(ANGLE_UNITS),
        "AreaUnit": unit(AREA_UNITS),
        "VolumeUnit": unit(VOLUME_UNITS),
        "UnaryOperator": {"enum": ["negate", "not"]},
        "BinaryOperator": {"enum": ["add", "subtract", "multiply", "divide", "power", "min", "max"]},
        "CompareOperator": {"enum": COMPARE_OPS},
        "Function": {"enum": ["sqrt", "abs", "round", "floor", "ceil", "sin", "cos", "tan", "atan2"]},
        "Number": node("number", {"value": number_schema()}),
        "Length": node("length", {"value": number_schema(), "unit": {"$ref": "#/definitions/LengthUnit"}}),
        "Angle": node("angle", {"value": number_schema(), "unit": {"$ref": "#/definitions/AngleUnit"}}),
        "Area": node("area", {"value": number_schema(), "unit": {"$ref": "#/definitions/AreaUnit"}}),
        "Volume": node("volume", {"value": number_schema(), "unit": {"$ref": "#/definitions/VolumeUnit"}}),
        "Bool": node("bool", {"value": {"type": "boolean"}}),
        "Text": node("text", {"value": {"type": "string"}}),
        "Param": node("param", {"name": {"type": "string", "minLength": 1}}),
        "Unary": node("unary", {"operator": {"$ref": "#/definitions/UnaryOperator"}, "operand": expr}),
        "Binary": node("binary", {"operator": {"$ref": "#/definitions/BinaryOperator"}, "left": expr, "right": expr}),
        "Compare": node("compare", {"operator": {"$ref": "#/definitions/CompareOperator"}, "left": expr, "right": expr}),
        "And": node("and", {"left": expr, "right": expr}),
        "Or": node("or", {"left": expr, "right": expr}),
        "If": node("if", {"condition": expr, "then": expr, "otherwise": expr}),
        "Call": node("call", {"function": {"$ref": "#/definitions/Function"}, "arguments": {"type": "array", "items": expr}}),
    }
    names = ["Number", "Length", "Angle", "Area", "Volume", "Bool", "Text", "Param", "Unary", "Binary", "Compare", "And", "Or", "If", "Call"]
    ops = ["number", "length", "angle", "area", "volume", "bool", "text", "param", "unary", "binary", "compare", "and", "or", "if", "call"]
    definitions["Expr"] = {
        "type": "object",
        "required": ["op"],
        "properties": {"op": {"enum": ops}},
        "allOf": [{"if": {"properties": {"op": {"const": op}}, "required": ["op"]}, "then": {"$ref": f"#/definitions/{n}"}} for op, n in zip(ops, names)],
    }
    return {"$schema": "http://json-schema.org/draft-07/schema#", "$id": BASE + "tree.json", "title": "Expr", "description": "The closed typed expression tree of semio-framework-expression; literals are non-negative, lengths in the unit they were authored in.", "definitions": definitions, "$ref": "#/definitions/Expr"}


def fixture_schema(domain, title, body, definitions=None):
    shared = {
        "Kind": {"enum": KINDS},
        "Value": {"type": "object", "required": ["kind", "value"], "additionalProperties": False, "properties": {"kind": {"$ref": "#/definitions/Kind"}, "value": {"type": ["number", "boolean", "string"]}}},
        "Failure": {"type": "object", "required": ["code", "path"], "additionalProperties": False, "properties": {"code": {"type": "string", "minLength": 1}, "path": {"type": "array", "items": {"type": "integer", "minimum": 0}}}},
        "Tree": {"$ref": BASE + "tree.json"},
    }
    return {"$schema": "http://json-schema.org/draft-07/schema#", "$id": BASE + domain + ".json", "title": title, "definitions": {**shared, **(definitions or {})}, **body}


def obj(properties, required=None):
    return {"type": "object", "required": required or list(properties), "additionalProperties": False, "properties": properties}


def schemas():
    ref = lambda name: {"$ref": f"#/definitions/{name}"}
    outcome = {"oneOf": [ref("Value"), obj({"error": ref("Failure")})]}
    kind_outcome = {"oneOf": [obj({"kind": ref("Kind")}), obj({"error": ref("Failure")})]}
    text = {"type": "string"}
    syntax_case = obj({"name": text, "source": text, "canonical": text, "python": text, "ast": ref("Tree")})
    syntax_error = obj({"name": text, "source": text, "code": text, "start": {"type": "integer", "minimum": 0}, "end": {"type": "integer", "minimum": 0}})
    param_error = {"type": "object", "required": ["code"], "properties": {"code": text, "path": {"type": "array", "items": {"type": "integer", "minimum": 0}}, "members": {"type": "array", "items": text}, "name": text, "declared": ref("Kind"), "found": ref("Kind")}, "additionalProperties": False}
    param_formula = obj({"source": text, "python": text})
    param_case = obj(
        {
            "name": text,
            "params": {"type": "object", "additionalProperties": param_formula},
            "overrides": {"type": "object", "additionalProperties": param_formula},
            "declared": {"type": "object", "additionalProperties": ref("Kind")},
            "expected": obj({"order": {"type": "array", "items": text}, "values": {"type": "object", "additionalProperties": ref("Value")}, "errors": {"type": "object", "additionalProperties": param_error}}),
        }
    )
    return {
        "tree": tree_schema(),
        "kinds": fixture_schema("kinds", "KindFixtures", obj({"cases": {"type": "array", "items": obj({"name": text, "source": text, "python": text, "params": {"type": "object", "additionalProperties": ref("Kind")}, "expected": kind_outcome})}})),
        "evaluation": fixture_schema("evaluation", "EvaluationFixtures", obj({"cases": {"type": "array", "items": obj({"name": text, "source": text, "python": text, "env": {"type": "object", "additionalProperties": ref("Value")}, "expected": outcome})}})),
        "parameters": fixture_schema("parameters", "ParameterFixtures", obj({"cases": {"type": "array", "items": param_case}})),
        "syntax": fixture_schema("syntax", "SyntaxFixtures", obj({"cases": {"type": "array", "items": syntax_case}, "errors": {"type": "array", "items": syntax_error}})),
    }


def facets():
    union = ["Number", "Length", "Angle", "Area", "Volume", "Bool", "Text", "Param", "Unary", "Binary", "Compare", "And", "Or", "If", "Call"]
    ts = ['/** 🌳️ Generated by `r12-w2-f1-expression-fixtures.py` from `🔣️.json`: the closed typed expression tree of `semio-framework-expression`. */', ""]
    ts += [f'export type LengthUnit = {" | ".join(json.dumps(u) for u in LENGTH_UNITS)};', f'export type AngleUnit = {" | ".join(json.dumps(u) for u in ANGLE_UNITS)};']
    ts += [f'export type AreaUnit = {" | ".join(json.dumps(u) for u in AREA_UNITS)};', f'export type VolumeUnit = {" | ".join(json.dumps(u) for u in VOLUME_UNITS)};']
    ts += ['export type UnaryOperator = "negate" | "not";', 'export type BinaryOperator = "add" | "subtract" | "multiply" | "divide" | "power" | "min" | "max";']
    ts += [f'export type CompareOperator = {" | ".join(json.dumps(u) for u in COMPARE_OPS)};', 'export type FunctionName = "sqrt" | "abs" | "round" | "floor" | "ceil" | "sin" | "cos" | "tan" | "atan2";', ""]
    ts += [
        'export type Expr =',
        '  | { op: "number"; value: number }',
        '  | { op: "length"; value: number; unit: LengthUnit }',
        '  | { op: "angle"; value: number; unit: AngleUnit }',
        '  | { op: "area"; value: number; unit: AreaUnit }',
        '  | { op: "volume"; value: number; unit: VolumeUnit }',
        '  | { op: "bool"; value: boolean }',
        '  | { op: "text"; value: string }',
        '  | { op: "param"; name: string }',
        '  | { op: "unary"; operator: UnaryOperator; operand: Expr }',
        '  | { op: "binary"; operator: BinaryOperator; left: Expr; right: Expr }',
        '  | { op: "compare"; operator: CompareOperator; left: Expr; right: Expr }',
        '  | { op: "and"; left: Expr; right: Expr }',
        '  | { op: "or"; left: Expr; right: Expr }',
        '  | { op: "if"; condition: Expr; then: Expr; otherwise: Expr }',
        '  | { op: "call"; function: FunctionName; arguments: Expr[] };',
        "",
    ]
    proto = ['// 🌳️ Generated by `r12-w2-f1-expression-fixtures.py` from `🔣️.json`: the closed typed expression tree of `semio-framework-expression`.', 'syntax = "proto3";', "package semio.framework.expression;", ""]
    for enum, values in (("LengthUnit", LENGTH_UNITS), ("AngleUnit", ANGLE_UNITS), ("AreaUnit", AREA_UNITS), ("VolumeUnit", VOLUME_UNITS), ("UnaryOperator", ["negate", "not"]), ("BinaryOperator", ["add", "subtract", "multiply", "divide", "power", "min", "max"]), ("CompareOperator", COMPARE_OPS), ("Function", ["sqrt", "abs", "round", "floor", "ceil", "sin", "cos", "tan", "atan2"])):
        prefix = re.sub(r"(?<!^)(?=[A-Z])", "_", enum).upper()
        proto.append(f"enum {enum} {{")
        proto.append(f"  {prefix}_UNSPECIFIED = 0;")
        proto += [f"  {prefix}_{v.upper().replace('-', '_')} = {i + 1};" for i, v in enumerate(values)]
        proto += ["}", ""]
    proto += [
        "message Expr {",
        "  oneof node {",
        "    double number = 1;",
        "    Length length = 2;",
        "    Angle angle = 3;",
        "    Area area = 4;",
        "    Volume volume = 5;",
        "    bool bool = 6;",
        "    string text = 7;",
        "    string param = 8;",
        "    Unary unary = 9;",
        "    Binary binary = 10;",
        "    Compare compare = 11;",
        "    Logic and = 12;",
        "    Logic or = 13;",
        "    Conditional if = 14;",
        "    Call call = 15;",
        "  }",
        "}",
        "",
        "message Length { double value = 1; LengthUnit unit = 2; }",
        "message Angle { double value = 1; AngleUnit unit = 2; }",
        "message Area { double value = 1; AreaUnit unit = 2; }",
        "message Volume { double value = 1; VolumeUnit unit = 2; }",
        "message Unary { UnaryOperator operator = 1; Expr operand = 2; }",
        "message Binary { BinaryOperator operator = 1; Expr left = 2; Expr right = 3; }",
        "message Compare { CompareOperator operator = 1; Expr left = 2; Expr right = 3; }",
        "message Logic { Expr left = 1; Expr right = 2; }",
        "message Conditional { Expr condition = 1; Expr then = 2; Expr otherwise = 3; }",
        "message Call { Function function = 1; repeated Expr arguments = 2; }",
        "",
    ]
    gql = ['"""🌳️ Generated by `r12-w2-f1-expression-fixtures.py` from `🔣️.json`: the closed typed expression tree of `semio-framework-expression`."""', ""]
    for enum, values in (("LengthUnit", LENGTH_UNITS), ("AngleUnit", ANGLE_UNITS), ("AreaUnit", AREA_UNITS), ("VolumeUnit", VOLUME_UNITS), ("UnaryOperator", ["negate", "not"]), ("BinaryOperator", ["add", "subtract", "multiply", "divide", "power", "min", "max"]), ("CompareOperator", COMPARE_OPS), ("ExpressionFunction", ["sqrt", "abs", "round", "floor", "ceil", "sin", "cos", "tan", "atan2"])):
        gql += [f"enum {enum} {{"] + [f"  {v.upper().replace('-', '_')}" for v in values] + ["}", ""]
    gql += [
        "type NumberExpr { value: Float! }",
        "type LengthExpr { value: Float! unit: LengthUnit! }",
        "type AngleExpr { value: Float! unit: AngleUnit! }",
        "type AreaExpr { value: Float! unit: AreaUnit! }",
        "type VolumeExpr { value: Float! unit: VolumeUnit! }",
        "type BoolExpr { value: Boolean! }",
        "type TextExpr { value: String! }",
        "type ParamExpr { name: String! }",
        "type UnaryExpr { operator: UnaryOperator! operand: Expr! }",
        "type BinaryExpr { operator: BinaryOperator! left: Expr! right: Expr! }",
        "type CompareExpr { operator: CompareOperator! left: Expr! right: Expr! }",
        "type AndExpr { left: Expr! right: Expr! }",
        "type OrExpr { left: Expr! right: Expr! }",
        "type IfExpr { condition: Expr! then: Expr! otherwise: Expr! }",
        "type CallExpr { function: ExpressionFunction! arguments: [Expr!]! }",
        "",
        "union Expr = NumberExpr | LengthExpr | AngleExpr | AreaExpr | VolumeExpr | BoolExpr | TextExpr | ParamExpr | UnaryExpr | BinaryExpr | CompareExpr | AndExpr | OrExpr | IfExpr | CallExpr",
        "",
    ]
    return "\n".join(ts), "\n".join(proto), "\n".join(gql)


def write(path, content):
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(content, encoding="utf-8", newline="\n")


def dump(value):
    return json.dumps(value, ensure_ascii=False, indent=1) + "\n"


def main():
    for domain, schema in schemas().items():
        write(DOMAIN[domain] / SCHEMA_DIR / JSON_NAME, dump(schema))
    ts, proto, gql = facets()
    write(DOMAIN["tree"] / SCHEMA_DIR / TS_NAME, ts)
    write(DOMAIN["tree"] / SCHEMA_DIR / PROTO_NAME, proto)
    write(DOMAIN["tree"] / SCHEMA_DIR / GRAPHQL_NAME, gql)
    fixtures = {"syntax": syntax_fixture(), "kinds": kinds_fixture(), "evaluation": evaluation_fixture(), "parameters": parameters_fixture()}
    for domain, document in fixtures.items():
        write(MODULE / FIXTURES_NAME / DOMAIN[domain].name / JSON_NAME, dump(document))
    for domain, document in fixtures.items():
        print(domain, len(document["cases"]))
    ok = sum("kind" in c["expected"] for c in fixtures["evaluation"]["cases"])
    print("evaluation values", ok, "errors", len(fixtures["evaluation"]["cases"]) - ok)


if __name__ == "__main__":
    main()
