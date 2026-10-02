"""🧾️ S3-PUZZLE: rewrites the bare record lists of the puzzle example DSL fixtures into the canonical braced form.

The DSL grammar now reads and prints every `Shape::List(Shape::Record)` element braced (`[ { k=v } { k=v } ]`,
law `🗣️dsl/🧬️schema/🧪️tests/🧾️record-list`). The old printer wrote the same records bare (`[ k=v k=v ]`); the old
parser (`parse_record_fields`) ended a bare record at the first key that was already consumed (or unknown). This script
replays exactly that boundary rule on the token stream: inside a list whose first token pair is `ident =`, a key that
repeats within the current record starts the next record. Every other byte is kept. Fails loudly on any record shape it
cannot delimit (positional fields, block/table fields inside a list element). Bare nested record fields (`grip-2d=angle=…`)
stay bare, as the grammar still prints them; their extent follows the old rule (keys of the nested spec, `NESTED`, each once),
with key sets copied from `Puzzle5dGrip2d` / `Puzzle5dGrip3d`.

Usage: python3 🧪️s3-puzzle-brace-record-lists.py [--check] <file.dsl.semio>...
"""
import re
import sys

TOKEN = re.compile(r'\s+|"(?:[^"\\]|\\.)*"|[\[\]{}()=]|[^\s\[\]{}()="]+', re.S)
OPEN = {"[": "]", "{": "}", "(": ")"}
IDENT = re.compile(r"[A-Za-z_][A-Za-z0-9_-]*$")
NESTED = {
    "grip-2d": {"angle", "grip-kind", "radius"},
    "grip-3d": {"position", "direction", "radius", "label"},
}


def tokens(text):
    out, at = [], 0
    while at < len(text):
        match = TOKEN.match(text, at)
        if not match:
            raise SystemExit(f"untokenizable byte at {at}: {text[at:at + 40]!r}")
        out.append(match.group(0))
        at = match.end()
    return out


def significant(toks, at):
    while at < len(toks) and toks[at].isspace():
        at += 1
    return at


def is_key(toks, at):
    at = significant(toks, at)
    nxt = significant(toks, at + 1)
    return at < len(toks) and IDENT.match(toks[at]) is not None and nxt < len(toks) and toks[nxt] == "="


def group_end(toks, at):
    depth, close = 0, []
    for index in range(at, len(toks)):
        tok = toks[index]
        if tok in OPEN:
            close.append(OPEN[tok])
        elif close and tok == close[-1]:
            close.pop()
            if not close:
                return index
    raise SystemExit(f"unbalanced group at token {at}")


def convert(toks, start, end, stats):
    out, at = [], start
    while at < end:
        tok = toks[at]
        if tok == "[":
            close = group_end(toks, at)
            out.append(convert_list(toks, at, close, stats))
            at = close + 1
            continue
        out.append(tok)
        at += 1
    return "".join(out)


def pair(toks, at, stats):
    key = toks[at]
    equals = significant(toks, at + 1)
    value_start = significant(toks, equals + 1)
    if key in NESTED and is_key(toks, value_start):
        remaining, fields, cursor = set(NESTED[key]), [], value_start
        while True:
            cursor = significant(toks, cursor)
            if cursor >= len(toks) or not is_key(toks, cursor) or toks[cursor] not in remaining:
                break
            remaining.discard(toks[cursor])
            text, cursor = pair(toks, cursor, stats)
            fields.append(text)
        stats["nested"] += 1
        return key + "=" + " ".join(fields), cursor
    last = value_end(toks, value_start)
    return key + "=" + convert(toks, value_start, last + 1, stats), last + 1


def value_end(toks, at):
    at = significant(toks, at)
    tok = toks[at]
    if tok in OPEN:
        return group_end(toks, at)
    if tok in "])}=":
        raise SystemExit(f"empty value before {tok!r}")
    if is_key(toks, at):
        raise SystemExit(f"bare nested record value starting at {tok!r}; cannot delimit")
    return at


def convert_list(toks, start, close, stats):
    first = significant(toks, start + 1)
    if first >= close or not is_key(toks, first):
        return "[" + convert(toks, start + 1, close, stats) + "]"
    records, current, seen, at = [], [], set(), first
    while True:
        at = significant(toks, at)
        if at >= close:
            break
        if not is_key(toks, at):
            raise SystemExit(f"non-key token {toks[at]!r} inside a record list; cannot delimit")
        key = toks[at]
        if key in seen:
            records.append(current)
            current, seen = [], set()
        seen.add(key)
        text, at = pair(toks, at, stats)
        current.append(text)
    records.append(current)
    stats["lists"] += 1
    stats["records"] += len(records)
    return "[ " + " ".join("{ " + " ".join(fields) + " }" for fields in records) + " ]"


def main(argv):
    check = "--check" in argv
    paths = [path for path in argv if path != "--check"]
    for path in paths:
        text = open(path, encoding="utf-8").read()
        stats = {"lists": 0, "records": 0, "nested": 0}
        toks = tokens(text)
        rewritten = convert(toks, 0, len(toks), stats)
        stripped = re.sub(r"\[ \{ | \} \{ | \} \]", lambda m: {"[ { ": "[ ", " } { ": " ", " } ]": " ]"}[m.group(0)], rewritten)
        if stats["lists"] and stripped != text:
            raise SystemExit(f"{path}: removing the inserted braces does not give back the source")
        print(f"{path}: {stats['lists']} bare record lists, {stats['records']} records, {stats['nested']} bare nested records")
        if not check and rewritten != text:
            open(path, "w", encoding="utf-8").write(rewritten)


if __name__ == "__main__":
    main(sys.argv[1:])
