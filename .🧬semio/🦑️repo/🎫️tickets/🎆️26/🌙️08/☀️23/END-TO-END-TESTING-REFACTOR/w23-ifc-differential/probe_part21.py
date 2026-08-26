#!/usr/bin/env python3
"""🔬️ Probe: does an ifcopenshell re-serialization project identically to its own input?

Written from ISO 10303-21 clause 6 (lexical) and clause 8 (header) alone. No semio module is
imported and no semio binary is executed. Run under the runner's own cache-local interpreter.
"""
import json
import sys

# region 🔖️Lexer
WS = " \t\r\n"


def decode_string_literal(lexeme):
    """🔤️ ISO 10303-21 §6.4.2: one string LEXEME turned into the value it denotes."""
    chars = list(lexeme)
    out = []
    i = 0
    alphabet = "A"

    def hex_group(at, width):
        slice_ = "".join(chars[at:at + width])
        if len(slice_) != width or not all(c in "0123456789abcdefABCDEF" for c in slice_):
            raise ValueError("bad hex group %r in %r" % (slice_, lexeme))
        return chr(int(slice_, 16))

    while i < len(chars):
        if chars[i] != "\\":
            out.append(chars[i])
            i += 1
            continue
        nxt = chars[i + 1] if i + 1 < len(chars) else None
        if nxt == "\\":
            out.append("\\")
            i += 2
        elif nxt == "P":
            page = chars[i + 2] if i + 2 < len(chars) else None
            if page is None or not ("A" <= page <= "I") or (i + 3 >= len(chars) or chars[i + 3] != "\\"):
                raise ValueError("malformed \\P directive in %r" % lexeme)
            alphabet = page
            i += 4
        elif nxt == "S":
            if i + 2 >= len(chars) or chars[i + 2] != "\\":
                raise ValueError("malformed \\S directive in %r" % lexeme)
            if alphabet != "A":
                raise ValueError("\\S\\ on ISO 8859 page %s needs a mapping table this projection does not carry" % alphabet)
            if i + 3 >= len(chars):
                raise ValueError("truncated \\S directive in %r" % lexeme)
            out.append(chr(ord(chars[i + 3]) + 128))
            i += 4
        elif nxt == "X":
            width = chars[i + 2] if i + 2 < len(chars) else None
            if width in ("2", "4"):
                group = 4 if width == "2" else 8
                if i + 3 >= len(chars) or chars[i + 3] != "\\":
                    raise ValueError("malformed \\X%s directive in %r" % (width, lexeme))
                i += 4
                while True:
                    out.append(hex_group(i, group))
                    i += group
                    if chars[i:i + 4] == ["\\", "X", "0", "\\"]:
                        i += 4
                        break
                    if i >= len(chars):
                        raise ValueError("unterminated \\X%s run in %r" % (width, lexeme))
            elif width == "\\":
                out.append(hex_group(i + 3, 2))
                i += 5
            else:
                raise ValueError("malformed \\X directive %r in %r" % (width, lexeme))
        else:
            raise ValueError("unsupported control directive %r in %r" % (nxt, lexeme))
    return "".join(out)
# endregion 🔖️Lexer


# region 🔖️Parser
class Parser:
    def __init__(self, text):
        self.t = text
        self.n = len(text)
        self.i = 0

    def skip(self):
        while self.i < self.n:
            c = self.t[self.i]
            if c in WS:
                self.i += 1
            elif c == "/" and self.t.startswith("/*", self.i):
                end = self.t.find("*/", self.i + 2)
                self.i = self.n if end < 0 else end + 2
            else:
                return

    def value(self):
        self.skip()
        c = self.t[self.i]
        if c == "$":
            self.i += 1
            return {"t": "unset"}
        if c == "*":
            self.i += 1
            return {"t": "derived"}
        if c == "'":
            return {"t": "string", "v": decode_string_literal(self.string_lexeme())}
        if c == "#":
            j = self.i + 1
            while j < self.n and self.t[j].isdigit():
                j += 1
            num = int(self.t[self.i + 1:j])
            self.i = j
            return {"t": "reference", "v": num}
        if c == ".":
            j = self.t.index(".", self.i + 1)
            name = self.t[self.i + 1:j]
            self.i = j + 1
            return {"t": "enum", "v": name}
        if c == "(":
            self.i += 1
            items = []
            self.skip()
            if self.t[self.i] == ")":
                self.i += 1
                return {"t": "aggregate", "v": items}
            while True:
                items.append(self.value())
                self.skip()
                if self.t[self.i] == ",":
                    self.i += 1
                    continue
                if self.t[self.i] == ")":
                    self.i += 1
                    return {"t": "aggregate", "v": items}
                raise ValueError("expected , or ) at %d: %r" % (self.i, self.t[self.i:self.i + 40]))
        if c == '"':
            j = self.t.index('"', self.i + 1)
            lex = self.t[self.i + 1:j]
            self.i = j + 1
            return {"t": "binary", "v": lex}
        if c == "-" or c == "+" or c.isdigit():
            return self.number()
        if c.isalpha() or c == "_":
            j = self.i
            while j < self.n and (self.t[j].isalnum() or self.t[j] in "_-"):
                j += 1
            keyword = self.t[self.i:j]
            self.i = j
            self.skip()
            if self.i < self.n and self.t[self.i] == "(":
                inner = self.value()
                # 🧭️A typed parameter carries exactly one value inside its parentheses.
                payload = inner["v"][0] if len(inner["v"]) == 1 else inner
                return {"t": "typed", "name": keyword, "v": payload}
            raise ValueError("bare keyword %r at %d" % (keyword, self.i))
        raise ValueError("unexpected %r at %d" % (c, self.i))

    def number(self):
        j = self.i
        if self.t[j] in "+-":
            j += 1
        is_real = False
        while j < self.n and (self.t[j].isdigit() or self.t[j] in ".eE+-"):
            if self.t[j] == ".":
                is_real = True
            elif self.t[j] in "eE":
                is_real = True
            elif self.t[j] in "+-" and self.t[j - 1] not in "eE":
                break
            j += 1
        lex = self.t[self.i:j]
        self.i = j
        return {"t": "real", "v": float(lex)} if is_real else {"t": "integer", "v": int(lex)}

    def string_lexeme(self):
        assert self.t[self.i] == "'"
        j = self.i + 1
        out = []
        while True:
            c = self.t[j]
            if c == "'":
                if j + 1 < self.n and self.t[j + 1] == "'":
                    out.append("''")
                    j += 2
                    continue
                self.i = j + 1
                return "".join(out).replace("''", "'")
            out.append(c)
            j += 1


def parse_document(text):
    """📥️ Header records and DATA entity instances of one exchange structure."""
    header = {}
    entities = []
    hstart = text.index("HEADER;") + len("HEADER;")
    hend = text.index("ENDSEC;", hstart)
    p = Parser(text[hstart:hend])
    while True:
        p.skip()
        if p.i >= p.n:
            break
        j = p.i
        while j < p.n and (text[hstart + j].isalnum() or text[hstart + j] == "_"):
            j += 1
        name = p.t[p.i:j]
        p.i = j
        args = p.value()
        header.setdefault(name, args["v"])
        p.skip()
        if p.i < p.n and p.t[p.i] == ";":
            p.i += 1
    dstart = text.index("DATA;", hend) + len("DATA;")
    dend = text.rindex("ENDSEC;")
    body = text[dstart:dend]
    q = Parser(body)
    while True:
        q.skip()
        if q.i >= q.n:
            break
        if q.t[q.i] != "#":
            break
        j = q.i + 1
        while q.t[j].isdigit():
            j += 1
        eid = int(q.t[q.i + 1:j])
        q.i = j
        q.skip()
        assert q.t[q.i] == "=", q.t[q.i:q.i + 20]
        q.i += 1
        q.skip()
        k = q.i
        while q.t[k].isalnum() or q.t[k] == "_":
            k += 1
        ename = q.t[q.i:k]
        q.i = k
        args = q.value()
        entities.append({"id": eid, "name": ename.upper(), "args": args["v"]})
        q.skip()
        if q.i < q.n and q.t[q.i] == ";":
            q.i += 1
    return header, entities


FILE_NAME_ATTRS = ["name", "timestamp", "author", "organization", "preprocessorVersion", "originatingSystem", "authorization"]
FILE_DESCRIPTION_ATTRS = ["description", "implementationLevel"]


def header_object(values, attrs):
    return {name: (values[i] if i < len(values) else None) for i, name in enumerate(attrs)}


def project(text):
    header, entities = parse_document(text)
    schema = []
    fs = header.get("FILE_SCHEMA", [])
    if fs and fs[0]["t"] == "aggregate":
        schema = [v["v"] for v in fs[0]["v"] if v["t"] == "string"]
    entities.sort(key=lambda e: e["id"])
    return {
        "fileSchema": schema,
        "fileDescription": header_object(header.get("FILE_DESCRIPTION", []), FILE_DESCRIPTION_ATTRS),
        "fileName": header_object(header.get("FILE_NAME", []), FILE_NAME_ATTRS),
        "entityCount": len(entities),
        "entities": entities,
    }
# endregion 🔖️Parser


# region 🔖️Probe
def first_diff(path, a, b):
    if isinstance(a, dict) and isinstance(b, dict):
        for k in a:
            if k not in b:
                return "%s.%s missing on the right" % (path, k)
            d = first_diff("%s.%s" % (path, k), a[k], b[k])
            if d:
                return d
        for k in b:
            if k not in a:
                return "%s.%s missing on the left" % (path, k)
        return None
    if isinstance(a, list) and isinstance(b, list):
        if len(a) != len(b):
            return "%s length %d != %d" % (path, len(a), len(b))
        for i, (x, y) in enumerate(zip(a, b)):
            d = first_diff("%s[%d]" % (path, i), x, y)
            if d:
                return d
        return None
    if isinstance(a, float) or isinstance(b, float):
        try:
            if abs(float(a) - float(b)) <= 1e-6:
                return None
        except (TypeError, ValueError):
            pass
    if a != b:
        return "%s: %r != %r" % (path, a, b)
    return None


if __name__ == "__main__":
    import ifcopenshell

    src = sys.argv[1]
    with open(src, "r", encoding="utf-8", errors="strict") as h:
        original = h.read()
    a = project(original)
    print("original entities:", a["entityCount"], "schema:", a["fileSchema"])
    f = ifcopenshell.open(src)
    rt = f.to_string()
    b = project(rt)
    print("ifcopenshell re-serialized entities:", b["entityCount"], "schema:", b["fileSchema"])
    ignore = {"timestamp", "preprocessorVersion", "originatingSystem", "authorization"}
    for key in ignore:
        a["fileName"][key] = None
        b["fileName"][key] = None
    d = first_diff("$", a, b)
    print("first divergence:", d if d else "NONE — projections agree")
# endregion 🔖️Probe
