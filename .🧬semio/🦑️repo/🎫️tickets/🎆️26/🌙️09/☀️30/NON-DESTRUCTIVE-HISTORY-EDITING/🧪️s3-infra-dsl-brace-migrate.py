"""🧾️ S3-INFRA: braces every bare record list of a DSL carrier (`[ k=v k=v ]` → `[ { k=v } { k=v } ]`), byte-preserving.

Same boundary rule as S3-PUZZLE (`🧪️s3-puzzle-brace-record-lists.py`), which is the old parser's rule: inside a list whose
first significant token pair is `ident =`, a key that repeats within the current record starts the next record. Block fields
(`crop { … }`), table fields (`runs [cols] { rows }`) and bare variant tags (`document schema=…`) count as keys. Only `{ ` before a record and ` }` after it are inserted; every source byte is kept, so the
diff is exactly the braces. Bare nested record values (`target=artifact-id=… artifact-kind=…`) need their key set via
`--nested key=a,b,c`; without it the script fails loudly. Strings, fenced embeds and `#` comment lines are skipped.

Usage: python3 🧪️s3-infra-dsl-brace-migrate.py [--check] [--nested key=a,b,…]… <file>…
"""
import re
import sys

TOKEN = re.compile(r'\s+|#[^\n]*|```.*?```|"(?:[^"\\]|\\.)*"|[\[\]{}()=]|[^\s\[\]{}()="`#]+|`|#', re.S)
OPEN = {"[": "]", "{": "}", "(": ")"}
IDENT = re.compile(r"[A-Za-z_][A-Za-z0-9_-]*$")


class Carrier:
    def __init__(self, text, nested):
        self.toks = [m.group(0) for m in TOKEN.finditer(text)]
        if "".join(self.toks) != text:
            raise SystemExit("tokenizer lost bytes")
        self.nested, self.before, self.after, self.spans, self.lists, self.records = nested, set(), set(), [], 0, 0

    def sig(self, at):
        while at < len(self.toks) and (self.toks[at].isspace() or self.toks[at].startswith("#")):
            at += 1
        return at

    def key_kind(self, at):
        at = self.sig(at)
        if at >= len(self.toks) or not IDENT.match(self.toks[at]):
            return None
        nxt = self.sig(at + 1)
        if nxt < len(self.toks) and self.toks[nxt] == "=":
            return "pair"
        if nxt < len(self.toks) and self.toks[nxt] == "{":
            return "block"
        if nxt < len(self.toks) and self.toks[nxt] == "[":
            return "table"
        return "tag"

    def group_end(self, at):
        close = []
        for index in range(at, len(self.toks)):
            tok = self.toks[index]
            if tok in OPEN:
                close.append(OPEN[tok])
            elif close and tok == close[-1]:
                close.pop()
                if not close:
                    return index
        raise SystemExit(f"unbalanced group at token {at}")

    def walk(self, start, end):
        at = start
        while at < end:
            if self.toks[at] == "[":
                close = self.group_end(at)
                self.bare_list(at, close)
                at = close + 1
            else:
                at += 1

    def value_end(self, at, key):
        at = self.sig(at)
        tok = self.toks[at]
        if tok in OPEN:
            close = self.group_end(at)
            self.walk(at, close + 1)
            return close
        if tok in "])}=":
            raise SystemExit(f"empty value of {key!r} before {tok!r}")
        if self.key_kind(at) == "pair":
            keys = self.nested.get(key)
            if keys is None:
                raise SystemExit(f"bare nested record value of {key!r} starting at {tok!r}; pass --nested {key}=…")
            remaining, last, cursor = set(keys), None, at
            while True:
                cursor = self.sig(cursor)
                if cursor >= len(self.toks) or self.key_kind(cursor) != "pair" or self.toks[cursor] not in remaining:
                    break
                remaining.discard(self.toks[cursor])
                last = self.field_end(cursor)
                cursor = last + 1
            return last
        return at

    def field_end(self, at):
        key, kind = self.toks[at], self.key_kind(at)
        if kind == "tag":
            return at
        if kind in ("block", "table"):
            group = self.sig(at + 1)
            close = self.group_end(group)
            if kind == "table":
                body = self.sig(close + 1)
                if body < len(self.toks) and self.toks[body] == "{":
                    close = self.group_end(body)
            self.walk(group, close + 1)
            return close
        equals = self.sig(at + 1)
        return self.value_end(equals + 1, key)

    def bare_list(self, start, close):
        first = self.sig(start + 1)
        if first >= close or self.key_kind(first) != "pair":
            self.walk(start + 1, close)
            return
        seen, at, record_start, record_end = set(), first, first, None
        while True:
            at = self.sig(at)
            if at >= close:
                break
            if self.key_kind(at) is None:
                raise SystemExit(f"non-key token {self.toks[at]!r} inside a record list; cannot delimit")
            key = self.toks[at]
            if key in seen:
                self.before.add(record_start)
                self.after.add(record_end)
                self.spans.append((start, record_start, record_end))
                self.records += 1
                seen, record_start = set(), at
            seen.add(key)
            record_end = self.field_end(at)
            at = record_end + 1
        self.before.add(record_start)
        self.after.add(record_end)
        self.spans.append((start, record_start, record_end))
        self.records += 1
        self.lists += 1

    def render(self):
        out = []
        for index, tok in enumerate(self.toks):
            if index in self.before:
                out.append("{ ")
            out.append(tok)
            if index in self.after:
                out.append(" }")
        return "".join(out)


def main(argv):
    check, nested, paths, at = False, {}, [], 0
    while at < len(argv):
        arg = argv[at]
        if arg == "--check":
            check = True
        elif arg == "--nested":
            key, keys = argv[at + 1].split("=", 1)
            nested[key] = keys.split(",")
            at += 1
        else:
            paths.append(arg)
        at += 1
    for path in paths:
        text = open(path, encoding="utf-8").read()
        carrier = Carrier(text, nested)
        carrier.walk(0, len(carrier.toks))
        rewritten = carrier.render()
        if len(rewritten) - len(text) != 4 * carrier.records:
            raise SystemExit(f"{path}: inserted {len(rewritten) - len(text)} bytes for {carrier.records} records")
        print(f"[DEBUG] {path}: {carrier.lists} bare record lists, {carrier.records} records")
        if not check and rewritten != text:
            open(path, "w", encoding="utf-8").write(rewritten)


if __name__ == "__main__":
    main(sys.argv[1:])
