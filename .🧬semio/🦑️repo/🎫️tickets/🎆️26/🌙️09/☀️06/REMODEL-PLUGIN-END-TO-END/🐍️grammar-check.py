#!/usr/bin/env python3
"""🔎 Third-party (python) twin of the framework `dialect grammar` engine.

Mirrors `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🔍️lexer/🦀️.rs` (token alphabet) and
`.../📖️grammar/🦀️.rs` (`parse_grammar`, `Recognizer::{compile,recognize,uncovered_productions}`)
closely enough to be an independent oracle for the remodeling grammar leaves: same greedy,
commit-on-first-alternative matching, same terminal predicates, same full-token-consumption
requirement, same `split_text_preamble` body reduction the framework fixture sweep applies.

Usage: python3 🐍️grammar-check.py
"""
import os, re, sys

REPO = os.path.abspath(os.path.join(os.path.dirname(__file__), "..", "..", "..", "..", "..", "..", ".."))
ANY = os.path.join(REPO, "✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any")
SCHEMA = os.path.join(ANY, "🧬️schema")
EXAMPLES = os.path.join(ANY, "📚️examples")

# ── lexer ──────────────────────────────────────────────────────────────────────
IDENT_CONT = set("_-./")
SINGLE = {"=": "Equals", ",": "Comma", ":": "Colon", "@": "At", "^": "Caret", "{": "LBrace", "}": "RBrace",
          "[": "LBracket", "]": "RBracket", "(": "LParen", ")": "RParen", "<": "Lt", ">": "Gt",
          "&": "Amp", "$": "Dollar", ";": "Semicolon", "+": "Plus", "*": "Star", "/": "Slash"}


class Tok:
    __slots__ = ("kind", "text")

    def __init__(self, kind, text):
        self.kind, self.text = kind, text

    def __repr__(self):
        return f"{self.kind}({self.text!r})"


def lex(src, keep_newlines=False):
    out, i, n = [], 0, len(src)
    while i < n:
        c = src[i]
        if c == "\n":
            if keep_newlines:
                out.append(Tok("Newline", "\n"))
            i += 1
            continue
        if c in " \t\r":
            i += 1
            continue
        if c == "#":
            while i < n and src[i] != "\n":
                i += 1
            continue
        if c == '"':
            j, buf = i + 1, []
            while j < n and src[j] != '"':
                if src[j] == "\\" and j + 1 < n:
                    buf.append(src[j + 1])
                    j += 2
                    continue
                buf.append(src[j])
                j += 1
            out.append(Tok("Text", "".join(buf)))
            i = j + 1
            continue
        if c.isdigit() or (c == "-" and i + 1 < n and src[i + 1].isdigit()):
            j = i + 1
            while j < n and src[j].isdigit():
                j += 1
            is_float = False
            if j < n and src[j] == "." and j + 1 < n and src[j + 1].isdigit():
                is_float = True
                j += 1
                while j < n and src[j].isdigit():
                    j += 1
            if j < n and src[j] in "eE":
                k = j + 1
                if k < n and src[k] in "+-":
                    k += 1
                if k < n and src[k].isdigit():
                    is_float = True
                    j = k
                    while j < n and src[j].isdigit():
                        j += 1
            out.append(Tok("Float" if is_float else "Int", src[i:j]))
            i = j
            continue
        if c.isalpha() or c == "_":
            j = i
            while j < n and (src[j].isalnum() or src[j] in IDENT_CONT):
                if src[j] == "-" and j + 1 < n and src[j + 1] in "->":
                    break
                j += 1
            text = src[i:j]
            out.append(Tok("Placeholder" if text == "_" else "Ident", text))
            i = j
            continue
        if c == "-" and i + 1 < n and src[i + 1] == ">":
            out.append(Tok("Arrow", "->"))
            i += 2
            continue
        if c == "." and i + 1 < n and src[i + 1] == ".":
            out.append(Tok("DotDot", ".."))
            i += 2
            continue
        if c in SINGLE:
            out.append(Tok(SINGLE[c], c))
            i += 1
            continue
        out.append(Tok("Error", c))
        i += 1
    return out


# ── grammar file parser ────────────────────────────────────────────────────────
HEADER_WORDS = {"dialect", "grammar", "extension", "start", "comment", "string", "use"}


class Sym:
    def __init__(self, kind, value, quant=None):
        self.kind, self.value, self.quant = kind, value, quant


class Grammar:
    def __init__(self):
        self.dialect = None
        self.id = None
        self.start = None
        self.productions = {}
        self.order = []


def parse_grammar(text):
    toks = lex(text, keep_newlines=True)
    g, i = Grammar(), 0

    def parse_atom(i):
        t = toks[i]
        if t.kind == "Text":
            s, i = Sym("lit", t.text), i + 1
        elif t.kind == "LBrace":
            alts, i = parse_alts(i + 1)
            assert toks[i].kind == "RBrace", f"expected }} got {toks[i]}"
            s, i = Sym("group", alts), i + 1
        elif t.kind == "Ident":
            name, i = t.text, i + 1
            s = Sym("term" if name.isupper() else "ref", name)
        else:
            raise SyntaxError(f"expected a symbol, found {t}")
        if i < len(toks) and toks[i].kind in ("Question", "Star", "Plus"):
            s.quant = toks[i].kind
            i += 1
        elif i < len(toks) and toks[i].text in ("?", "*", "+") and toks[i].kind in ("Star", "Plus", "Error"):
            s.quant = {"?": "Question", "*": "Star", "+": "Plus"}[toks[i].text]
            i += 1
        return s, i

    def parse_seq(i):
        syms = []
        while i < len(toks) and toks[i].kind not in ("Newline", "RBrace") and toks[i].text != "|":
            s, i = parse_atom(i)
            syms.append(s)
        return syms, i

    def parse_alts(i):
        alts = []
        s, i = parse_seq(i)
        alts.append(s)
        while i < len(toks) and toks[i].text == "|":
            s, i = parse_seq(i + 1)
            alts.append(s)
        return alts, i

    while i < len(toks):
        t = toks[i]
        if t.kind == "Newline":
            i += 1
            continue
        if t.kind == "Ident" and t.text in HEADER_WORDS:
            word = t.text
            i += 1
            args = []
            while i < len(toks) and toks[i].kind != "Newline":
                args.append(toks[i].text)
                i += 1
            if word == "dialect":
                g.dialect = args[0]
            elif word == "grammar":
                g.id = args[0]
            elif word == "start":
                g.start = args[0]
            continue
        assert t.kind == "Ident", f"expected Ident, found {t}"
        name, i = t.text, i + 1
        assert toks[i].kind == "Equals", f"expected Equals after {name}, found {toks[i]}"
        alts, i = parse_alts(i + 1)
        g.productions[name] = alts
        g.order.append(name)
    return g


# ── recognizer ─────────────────────────────────────────────────────────────────
def terminal_matches(name, tok):
    u = name.upper()
    if u == "BOOL":
        return tok.kind == "Ident" and tok.text in ("true", "false")
    if u in ("IDENT", "PLACEHOLDER"):
        return tok.kind in ("Ident", "Placeholder")
    if u == "INT":
        return tok.kind == "Int"
    if u == "FLOAT":
        return tok.kind == "Float"
    if u in ("TEXT", "STRING"):
        return tok.kind == "Text"
    if u in ("EQUALS", "EQ"):
        return tok.kind == "Equals" or tok.text == "="
    if u == "QUANTITY":
        return tok.kind in ("Int", "Float")
    return tok.kind.upper() == u


class Rec:
    def __init__(self, g):
        self.g = g

    def match_sym(self, s, toks, pos, covered):
        if s.quant == "Question":
            r = self.match_base(s, toks, pos, covered)
            return pos if r is None else r
        if s.quant == "Star":
            cur = pos
            while True:
                nxt = self.match_base(s, toks, cur, covered)
                if nxt is None or nxt == cur:
                    break
                cur = nxt
            return cur
        if s.quant == "Plus":
            first = self.match_base(s, toks, pos, covered)
            if first is None:
                return None
            cur = first
            while True:
                nxt = self.match_base(s, toks, cur, covered)
                if nxt is None or nxt == cur:
                    break
                cur = nxt
            return cur
        return self.match_base(s, toks, pos, covered)

    def match_base(self, s, toks, pos, covered):
        if s.kind == "lit":
            return pos + 1 if pos < len(toks) and toks[pos].text == s.value else None
        if s.kind == "term":
            return pos + 1 if pos < len(toks) and terminal_matches(s.value, toks[pos]) else None
        if s.kind == "ref":
            alts = self.g.productions.get(s.value)
            if alts is None:
                return None
            return self.match_prod(s.value, alts, toks, pos, covered)
        if s.kind == "group":
            for alt in s.value:
                r = self.match_seq(alt, toks, pos, covered)
                if r is not None:
                    return r
            return None
        return None

    def match_seq(self, syms, toks, pos, covered):
        for s in syms:
            pos = self.match_sym(s, toks, pos, covered)
            if pos is None:
                return None
        return pos

    def match_prod(self, name, alts, toks, pos, covered):
        for alt in alts:
            r = self.match_seq(alt, toks, pos, covered)
            if r is not None:
                covered.add(name)
                return r
        return None

    def recognize(self, text):
        toks = lex(text)
        covered = set()
        alts = self.g.productions[self.g.start]
        r = self.match_prod(self.g.start, alts, toks, 0, covered)
        return (r is not None and r == len(toks)), (r or 0), len(toks), toks, covered


def dsl_body(text):
    """🪪 Framework `dsl_body_from_fixture`: `semio <plugin>.<artifact>.<component> vN` -> `<plugin>.<artifact>`."""
    if not text.lstrip().startswith("semio "):
        return text
    first, _, rest = text.partition("\n")
    m = re.match(r"^semio\s+(.+)\s+v\d+\s*$", first.strip())
    if not m:
        return text
    parts = m.group(1).split(".")
    return ".".join(parts[:-1]) + "\n" + rest.lstrip("\r\n")


def check(label, grammar_path, doc_texts):
    with open(grammar_path, encoding="utf-8") as fh:
        g = parse_grammar(fh.read())
    assert g.dialect == "grammar", f"{label}: dialect must be `grammar`, got {g.dialect}"
    assert g.start in g.productions, f"{label}: start production `{g.start}` missing"
    undefined = set()
    for alts in g.productions.values():
        stack = [s for alt in alts for s in alt]
        while stack:
            s = stack.pop()
            if s.kind == "ref" and s.value not in g.productions:
                undefined.add(s.value)
            elif s.kind == "group":
                stack.extend(x for alt in s.value for x in alt)
    if undefined:
        print(f"  {label}: UNDEFINED refs {sorted(undefined)}")
    rec, ok_all = Rec(g), True
    all_covered = set()
    for name, text in doc_texts:
        ok, consumed, total, toks, covered = rec.recognize(dsl_body(text))
        all_covered |= covered
        if ok:
            print(f"  {label} <- {name}: RECOGNIZED ({total} tokens)")
        else:
            ok_all = False
            ctx = " ".join(t.text for t in toks[max(0, consumed - 6):consumed + 6])
            print(f"  {label} <- {name}: FAILED at token {consumed}/{total}: …{ctx}…")
    uncovered = [n for n in g.order if n not in all_covered]
    if uncovered:
        print(f"  {label}: uncovered productions ({len(uncovered)}): {', '.join(uncovered)}")
    return ok_all, g, rec


# ── binary layout lint ─────────────────────────────────────────────────────────
# 🔬️ The record layout every 💾️binary triple must actually describe, read off the framework:
# `os_pack::encode_record_fields` = `field_count varint, (field_id varint, tagged value)*`
# (🎒️pack/🌱️value/🦀️.rs:342), sorted by field id, `Absent` omitted; tag alphabet 0x00..0x17;
# the value bridge is the one-field `value_bridge_spec()` (🏪️store/🦀️.rs:4970, field id 1,
# `Shape::Value`, `TAG_VALUE` 0x11). Anything short of naming these is the copy-pasted stub.
SNAPSHOT_FIELD_IDS = ["0", "schema", "1", "id", "2", "streams", "3", "assets", "4", "durable-artifacts", "5", "calibration", "6", "params", "7", "gcps", "8", "job", "9", "results"]
OP_ORDINAL_ENDS = ["create-stream", "commit-reconstruction"]


def binary_layout_problems(facet, slug, proto, ksy, spicy, abnf):
    out = []
    if slug in ("snapshot", "op"):
        for label, body in (("🥋️", ksy), ("🌶️", spicy), ("🔠️", abnf)):
            if "field_count" not in body and "field-count" not in body:
                out.append(f"{label} never names the record field_count")
            if "field_id" not in body and "field-id" not in body:
                out.append(f"{label} never names the per-entry field_id")
        if "0x0d" not in ksy.lower():
            out.append("🥋️ no 0x0D record tag in the value alphabet")
        for token in SNAPSHOT_FIELD_IDS if slug == "snapshot" else OP_ORDINAL_ENDS:
            if token not in ksy:
                out.append(f"🥋️ missing {'field id' if slug == 'snapshot' else 'ordinal'} `{token}`")
        if slug == "op":
            for i, kw in enumerate(("create-stream", "commit-reconstruction")):
                probe = f"{0 if i == 0 else 34:2d} {kw}"
                if probe not in abnf or probe not in spicy:
                    out.append(f"ordinal row `{probe.strip()}` missing from 🔠️/🌶️")
    else:
        for label, body in (("🥋️", ksy), ("🌶️", spicy), ("🔠️", abnf)):
            # ABNF spells a byte `%x11`, ksy/spicy spell it `0x11` — accept either notation.
            if "0x11" not in body and "%x11" not in body:
                out.append(f"{label} never names TAG_VALUE 0x11")
            if "VALUE_BRIDGE_FIELD_ID" not in body and "value_bridge_spec" not in body:
                out.append(f"{label} never names the value bridge spec")
        for tag in ("12", "0C", "10", "07"):
            if f"%x{tag}" not in abnf.upper().replace("%X", "%x"):
                out.append(f"🔠️ DslValue tag 0x{tag} unlisted")
    return out


def read(path):
    with open(path, encoding="utf-8") as fh:
        return fh.read()


def main():
    failures = []
    demo = read(os.path.join(EXAMPLES, "🎬️demo/🖼️assets/🗣️.dsl.semio"))
    orbit = read(os.path.join(EXAMPLES, "🛰️synthetic-orbit/🖼️assets/🗣️.dsl.semio"))
    docs = [("📚️examples/🎬️demo", demo), ("📚️examples/🛰️synthetic-orbit", orbit)]

    populated = read(os.path.join(os.path.dirname(__file__), "🗒️populated-snapshot.dsl.semio"))
    print("── snapshot grammar vs the committed examples + a populated print of every optional lane ──")
    ok, snap_g, snap_rec = check("📸️snapshot", os.path.join(SCHEMA, "📸️snapshot/📝️text/📖️.grammar.semio"), docs + [("🗒️populated-snapshot", populated)])
    if not ok:
        failures.append("snapshot")

    print("── ops grammar vs printed op lines ──")
    ops = read(os.path.join(os.path.dirname(__file__), "🗒️op-lines.txt")).strip().splitlines()
    with open(os.path.join(SCHEMA, "🧬️mutations/📝️text/📖️.grammar.semio"), encoding="utf-8") as fh:
        og = parse_grammar(fh.read())
    orec, ocov = Rec(og), set()
    for line in ops:
        if not line.strip():
            continue
        toks = lex(line)
        r = orec.match_prod(og.start, og.productions[og.start], toks, 0, ocov)
        if r is None or r != len(toks):
            failures.append(f"op:{line.split()[0]}")
            print(f"  🧬️mutations <- {line.split()[0]}: FAILED at {r}/{len(toks)}")
    print(f"  🧬️mutations: {len(ops)} op line(s) checked, {sum(1 for f in failures if f.startswith('op:'))} failure(s)")
    ouncov = [n for n in og.order if n not in ocov]
    if ouncov:
        print(f"  🧬️mutations: uncovered productions ({len(ouncov)}): {', '.join(ouncov)}")

    print("── diff / inference grammars vs printed value-bridge documents ──")
    for label, rel, fixture in (
        ("🔺️diff", "🔺️diff/📝️text/📖️.grammar.semio", "🗒️diff.dsl.semio"),
        ("💡️inferences", "💡️inferences/📝️text/📖️.grammar.semio", "🗒️inference.dsl.semio"),
    ):
        ok3, _, _ = check(label, os.path.join(SCHEMA, rel), [(fixture, read(os.path.join(os.path.dirname(__file__), fixture)))])
        if not ok3:
            failures.append(label)

    print("── cross-artifact rejection: every non-stdio peer fixture must be rejected ──")
    plugins = os.path.join(REPO, "✏️s/🔌️plugins")
    checked, accepted = 0, []
    for root, dirs, files in os.walk(plugins):
        if "📖️.grammar.semio" not in files or "/🧬️schema/📸️snapshot/📝️text" not in root:
            continue
        if "🗄️stdio" in root or "📸️remodel" in root:
            continue
        gtext = read(os.path.join(root, "📖️.grammar.semio"))
        if "OCTET" in gtext:  # framework `soft_skip`-equivalent: the untouched stub scaffold
            continue
        subset = os.path.dirname(os.path.dirname(os.path.dirname(root)))
        for ex_root, _, ex_files in os.walk(os.path.join(subset, "📚️examples")):
            for f in ex_files:
                if not f.endswith(".dsl.semio"):
                    continue
                ok, *_ = snap_rec.recognize(dsl_body(read(os.path.join(ex_root, f))))
                checked += 1
                if ok:
                    accepted.append(os.path.relpath(os.path.join(ex_root, f), plugins))
    print(f"  📸️snapshot vs {checked} non-stdio peer fixture(s): {len(accepted)} wrongly accepted")
    for a in accepted:
        failures.append(f"reject:{a}")
        print(f"    ACCEPTED (BAD): {a}")

    print("── binary specs: identity + dialect lint, and the 3 repeat-free protocol files vs the framework protocol grammar ──")
    proto_g = parse_grammar(read(os.path.join(REPO, "🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/📖️grammar/📡️protocol.grammar.semio")))
    proto_rec = Rec(proto_g)
    for facet, slug in (("📸️snapshot", "snapshot"), ("🧬️mutations", "op"), ("🔺️diff", "diff"), ("💡️inferences", "inference")):
        d = os.path.join(SCHEMA, facet, "💾️binary")
        proto = read(os.path.join(d, "📡️.protocol.semio"))
        ksy, spicy, abnf = read(os.path.join(d, "🥋️.ksy")), read(os.path.join(d, "🌶️.spicy")), read(os.path.join(d, "🔠️.abnf"))
        problems = []
        for label, body in (("📡️", proto), ("🥋️", ksy), ("🌶️", spicy), ("🔠️", abnf)):
            if "stdio" in body.split("doc:")[0].split("# ")[0] and label != "🥋️":
                problems.append(f"{label} still names stdio")
        if not proto.startswith("dialect protocol\n"):
            problems.append("📡️ missing `dialect protocol`")
        if f"protocol remodeling.{slug}" not in proto:
            problems.append(f"📡️ protocol id is not remodeling.{slug}")
        if "framing " not in proto:
            problems.append("📡️ no framing line")
        if f"id: remodeling_{slug}" not in ksy:
            problems.append(f"🥋️ meta.id is not remodeling_{slug}")
        if f"module Remodeling_{slug};" not in spicy:
            problems.append(f"🌶️ module is not Remodeling_{slug}")
        if not abnf.startswith("; abnf remodeling."):
            problems.append("🔠️ header comment is not remodeling.*")
        problems.extend(binary_layout_problems(facet, slug, proto, ksy, spicy, abnf))
        if "repeat " not in proto:
            ok, consumed, total, toks, _ = proto_rec.recognize(proto)
            if not ok:
                problems.append(f"📡️ not accepted by 📡️protocol.grammar.semio at {consumed}/{total}")
        print(f"  {facet}/💾️binary: {'ok' if not problems else '; '.join(problems)}")
        failures.extend(f"binary:{facet}:{x}" for x in problems)

    print()
    if failures:
        print(f"❌ {len(failures)} failure(s): {failures}")
        return 1
    print("✅ every remodeling grammar leaf recognizes its real documents and rejects the peers'")
    return 0


if __name__ == "__main__":
    sys.exit(main())
