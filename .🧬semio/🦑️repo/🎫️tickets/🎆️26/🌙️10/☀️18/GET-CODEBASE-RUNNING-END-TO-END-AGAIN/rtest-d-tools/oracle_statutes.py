#!/usr/bin/env python3
"""Independent oracle of the statutes module: hashlib (SHA-256), zlib/gzip (RFC 1951/1952) and `re` models of the
spec-text scanners.  Writes (or with --check verifies) 🧪️tests/🧫️fixtures/🔬️unit-vectors.json of the statutes module.

The gzip members are written by CPython's zlib (levels, strategies, header flags): the hand-rolled inflater has to read
every one of them.  `--check` additionally inflates the stored Rust-written members (`rustMembers`) with zlib.
"""
import gzip
import hashlib
import json
import re
import struct
import sys
import zlib
from pathlib import Path

ROOT = Path("/Users/ueli/Documents/semio")
MODULE = ROOT / "🧰️framework/🛍️products/🦑️repo/🔨️modules/📜️statutes"
FIXTURES = MODULE / "🧪️tests/🧫️fixtures"
ORACLE_SCRIPT = str(Path(__file__).resolve().relative_to(ROOT))
TARGET = FIXTURES / "🔬️unit-vectors.json"

WORDS = ["alpha", "beta", "gamma", "delta", "epsilon", "zeta", "eta", "theta", "iota", "kappa", "lambda", "mu", "statute", "policy", "breach", "section"]


def lcg(seed):
    state = seed
    while True:
        state = (state * 1103515245 + 12345) & 0x7FFFFFFF
        yield state >> 8


def recipe_bytes(recipe):
    kind = recipe["kind"]
    if kind == "words":
        gen = lcg(recipe["seed"])
        out = []
        for index in range(recipe["count"]):
            out.append(WORDS[next(gen) % len(WORDS)])
            out.append("\n" if index % 11 == 10 else " ")
        return "".join(out).encode("utf8")
    if kind == "bytes":
        gen = lcg(recipe["seed"])
        return bytes(next(gen) & 0xFF for _ in range(recipe["count"]))
    if kind == "repeat":
        return bytes([recipe["byte"]]) * recipe["count"]
    if kind == "text":
        return recipe["text"].encode("utf8")
    raise ValueError(kind)


ALL = list(range(8))
FLAGS_ALL = (4, 8, 16, 2, 4 | 8 | 16 | 2)
RECIPES = [
    ({"kind": "text", "text": ""}, ALL, FLAGS_ALL, True),
    ({"kind": "text", "text": "a"}, ALL, FLAGS_ALL, True),
    ({"kind": "text", "text": "{\"entityId\":\"🦑️repo\",\"script\":\"lint\",\"breachs\":[]}"}, ALL, FLAGS_ALL, True),
    ({"kind": "words", "seed": 7, "count": 40}, ALL, FLAGS_ALL, True),
    ({"kind": "words", "seed": 11, "count": 3000}, [1, 2, 3, 4, 5, 6, 7], (4 | 8 | 16 | 2,), True),
    ({"kind": "bytes", "seed": 3, "count": 300}, ALL, (4 | 8 | 16 | 2,), True),
    ({"kind": "bytes", "seed": 5, "count": 65600}, [2], (), False),
    ({"kind": "repeat", "byte": 97, "count": 70000}, [1, 2, 3, 6], (), True),
    ({"kind": "repeat", "byte": 0, "count": 1}, ALL, (), False),
]
SHA_RECIPES = [{"kind": "text", "text": ""}, {"kind": "text", "text": "abc"}, {"kind": "text", "text": "abcdbcdecdefdefgefghfghighijhijkijkljklmklmnlmnomnopnopq"}, {"kind": "text", "text": "🧬semio 🦑️repo ünïcödé"}]
SHA_RECIPES += [{"kind": "repeat", "byte": 97, "count": n} for n in (1, 54, 55, 56, 57, 63, 64, 65, 119, 120, 121, 128, 1000, 1000000)]
SHA_RECIPES += [{"kind": "bytes", "seed": 9, "count": n} for n in (200, 4097)]

CONFIGS = [
    {"level": 0, "strategy": 0}, {"level": 1, "strategy": 0}, {"level": 6, "strategy": 0}, {"level": 9, "strategy": 0},
    {"level": 6, "strategy": 4}, {"level": 6, "strategy": 2}, {"level": 6, "strategy": 3}, {"level": 6, "strategy": 1},
]


def member(data, level, strategy):
    compressor = zlib.compressobj(level, zlib.DEFLATED, 31, 8, strategy)
    return compressor.compress(data) + compressor.flush()


def flagged_member(data, flags):
    """A gzip member with FEXTRA/FNAME/FCOMMENT/FHCRC header parts around a raw deflate stream."""
    raw = zlib.compressobj(6, zlib.DEFLATED, -15)
    body = raw.compress(data) + raw.flush()
    header = bytearray([0x1F, 0x8B, 8, flags, 0, 0, 0, 0, 0, 3])
    if flags & 4:
        extra = b"XY\x03\x00abc"
        header += struct.pack("<H", len(extra)) + extra
    if flags & 8:
        header += b"name.txt\x00"
    if flags & 16:
        header += b"a comment\x00"
    if flags & 2:
        header += struct.pack("<H", zlib.crc32(bytes(header)) & 0xFFFF)
    return bytes(header) + body + struct.pack("<II", zlib.crc32(data), len(data) & 0xFFFFFFFF)


def stored_member(data):
    """The member the hand-rolled encoder writes: fixed header, stored blocks of at most 65535 bytes, CRC-32 and length."""
    out = bytearray([0x1F, 0x8B, 8, 0, 0, 0, 0, 0, 4, 0xFF])
    if not data:
        out += bytes([1, 0, 0, 0xFF, 0xFF])
    else:
        chunks = [data[i:i + 65535] for i in range(0, len(data), 65535)]
        for index, chunk in enumerate(chunks):
            out += bytes([1 if index == len(chunks) - 1 else 0]) + struct.pack("<HH", len(chunk), len(chunk) ^ 0xFFFF) + chunk
    return bytes(out) + struct.pack("<II", zlib.crc32(data), len(data) & 0xFFFFFFFF)


def named_member(data):
    import io
    buffer = io.BytesIO()
    with gzip.GzipFile(filename="payload.json", mode="wb", fileobj=buffer, mtime=1234567890) as handle:
        handle.write(data)
    return buffer.getvalue()


SPEC_TEXTS = [
    "The tool MUST report a breach", "it should not", "SHOULD", "A SHALL", "MAY", "MAYBE not", "REQUIRED_FIELD", "x_MUST y", "MUST_NOT", "MUST NOT happen", "RECOMMENDED and OPTIONAL",
    "OPTIONAL.", "(MAY)", "must lowercase", "éMUST", "MUSTé", "", "nothing here", "SHALLOW", "a REQUIRED\nline", "UNMUST", "OPTIONALLY",
]
SYNTAX_TEXTS = [
    "plain prose", "use `code` here", "empty `` pair only", "``code``", "`` a`", "a `` b `x` c", "one ` backtick", "`a` and `b`", "call foo.bar(x) now", "call Foo(x) now", "MUST(x)", "MUST(x) then foo.bar(y)",
    "SHOULD( and Baz(", "MAY(y) Qux(z)", "a.b( c", "a.b(", "obj.method()", "_x._y(1)", "1abc.def(", "Foo.Bar(", "fooBar(", "fooBar (", "Abc(", "A(", "see foo.bar", "naming.Style rules",
    "SHALL(1) Other(2)", "x.y.z(1)", "prefixMUST(1)", "ÉA(", "the `x` and foo.bar(1)",
]
CONTRIBUTOR_TEXTS = [
    "2026 Ueli Saluz <ueli@semio-tech.com>", "2026 <a@b>", "2026  <a@b>", "2026   <a@b>", "2026 x<a@b>", "2026\tName\t<a.b-c@d>", "2026 Name <>", "2026 Name <a b>", "202 Name <a@b>", "20267 Name <a@b>",
    "(c) 2026 Ueli Saluz <ueli@semio-tech.com> ok", "2026Name <a@b>", "2026 Ünï <a@b>", "2026 a b c <a@b> trailing", "2026 Name (x) <a@b>", "", "2026 Name \n <a@b>", "2026 <", "1999 A <b@c>",
]

GO = re.ASCII
SPEC = re.compile(r"\b(MUST(\s+NOT)?|SHOULD(\s+NOT)?|SHALL(\s+NOT)?|MAY|REQUIRED|RECOMMENDED|OPTIONAL)\b", GO)
BACKTICK = re.compile(r"`[^`]+`", GO)
CALL = re.compile(r"[A-Za-z_]\w*\.\w+\(|[A-Z]\w+\(", GO)
CONTRIBUTOR = re.compile(r"\d{4}\s+[\w\s]+<[\w.@-]+>", GO)


def implementation(text):
    if BACKTICK.search(text):
        return "backtick-wrapped code"
    match = CALL.search(text)
    if match and match.group(0) not in ("MUST(", "SHOULD(", "SHALL(", "MAY("):
        return "function/method call: " + match.group(0)
    return None


IGNORE_PREFIX = "// compose-ignore-"


def parse_directives(content):
    found = {}
    for number, line in enumerate(content.split("\n"), 1):
        stripped = line.strip()
        if stripped.startswith(IGNORE_PREFIX):
            patterns = [part.strip() for part in stripped[len(IGNORE_PREFIX):].split(",") if part.strip()]
            if patterns:
                found[number] = patterns
    return found


def ignored(directives, line, kind):
    return any(line > at and line <= at + 100 and any(kind.startswith(p) for p in patterns) for at, patterns in directives.items())


def build():
    sha = [{"recipe": r, "sha256": hashlib.sha256(recipe_bytes(r)).hexdigest()} for r in SHA_RECIPES]
    members = []
    for recipe, config_indexes, flag_sets, named in RECIPES:
        data = recipe_bytes(recipe)
        row = {"recipe": recipe, "length": len(data), "crc32": zlib.crc32(data) & 0xFFFFFFFF, "members": []}
        for index in config_indexes:
            config = CONFIGS[index]
            row["members"].append({"how": "zlib level %d strategy %d" % (config["level"], config["strategy"]), "hex": member(data, config["level"], config["strategy"]).hex()})
        if named:
            row["members"].append({"how": "gzip file with name and mtime", "hex": named_member(data).hex()})
        for flags in flag_sets:
            row["members"].append({"how": "header flags %d" % flags, "hex": flagged_member(data, flags).hex()})
        members.append(row)
    vectors = json.loads((FIXTURES / "🙈️ignore-directives/🔣️vectors.json").read_text(encoding="utf8"))
    ignore = []
    for document in vectors["documents"]:
        directives = parse_directives(document["content"])
        ignore.append({
            "name": document["name"],
            "directives": {str(k): v for k, v in directives.items()},
            "probes": [{"line": p["line"], "kind": p["kind"], "ignored": ignored(directives, p["line"], p["kind"])} for p in document["probes"]],
        })
    envelope_vectors = json.loads((FIXTURES / "🗜️breach-cache-envelope/🔣️vectors.json").read_text(encoding="utf8"))
    envelope = envelope_vectors["envelope"].encode("utf8")
    stored = [{"recipe": r, "hex": stored_member(recipe_bytes(r)).hex()} for r, _, _, _ in RECIPES]
    return {
        "schema": "semio.repo.statutes.unit-vectors/1",
        "$comment": "🔬️ Expected answers for the cargo unit tests. `sha256` is hashlib's digest of the payload a recipe describes, `members` are gzip members written by CPython's zlib/gzip (levels, strategies, header flags) that the hand-rolled inflater must read back to the same payload, `storedMembers` the bytes a stored-block encoder must write (a Python model of the member layout, verified to inflate under zlib by `--check`), and the three text tables are answers of `re` models equal to the Go regular expressions. Regenerate with the oracle; never edit by hand.",
        "oracle": {"script": ORACLE_SCRIPT, "tool": "python3", "libraries": ["hashlib", "zlib", "gzip", "re"], "model": "SHA-256, gzip members, spec-text scanners"},
        "sha": sha,
        "gzip": members,
        "storedMembers": stored,
        "ignoreDirectives": ignore,
        "envelope": {"sha256": hashlib.sha256(envelope).hexdigest(), "memberHex": stored_member(envelope).hex(), "crc32": zlib.crc32(envelope) & 0xFFFFFFFF},
        "specText": [{"text": t, "spec": bool(SPEC.search(t))} for t in SPEC_TEXTS],
        "implementationSyntax": [{"text": t, "reason": implementation(t)} for t in SYNTAX_TEXTS],
        "contributorLines": [{"text": t, "contributor": bool(CONTRIBUTOR.search(t))} for t in CONTRIBUTOR_TEXTS],
    }


def check_stored_members(document):
    for row in document["storedMembers"]:
        if gzip.decompress(bytes.fromhex(row["hex"])) != recipe_bytes(row["recipe"]):
            print("STORED MEMBER DOES NOT INFLATE", row["recipe"])
            return False
    return True


def main():
    document = build()
    text = json.dumps(document, ensure_ascii=False, indent=1) + "\n"
    if "--check" in sys.argv:
        same = TARGET.exists() and TARGET.read_text(encoding="utf8") == text
        ok = check_stored_members(document)
        print("SAME" if same else "DIFFERENT", "stored-members-inflate" if ok else "stored-members-broken")
        sys.exit(0 if same and ok else 1)
    TARGET.write_text(text, encoding="utf8")
    print("written", TARGET, len(text))


main()
