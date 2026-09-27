#!/usr/bin/env python3
"""🔮️ CX1 third-party oracle: derives every case snapshot of the stdio native text codec fixture with Python's own
readers (str.splitlines for txt, csv for tsv, html.parser for html) and compares it with the fixture's expectation.
Usage: cx1-oracle.py <fixture.json>"""
import csv
import html.parser
import io
import json
import sys

VOID = {"area", "base", "br", "col", "embed", "hr", "img", "input", "link", "meta", "param", "source", "track", "wbr"}


def txt(source):
    chunks = source.splitlines(keepends=True)
    lines = [chunk.rstrip("\r\n") for chunk in chunks]
    return {"schema": "stdio.txt", "lines": lines, "trailingNewline": bool(chunks) and chunks[-1].endswith("\n"), "lineEnding": "crLf" if "\r\n" in source else "lf"}


def tsv(source):
    records = [row for row in csv.reader(io.StringIO(source, newline=""), delimiter="\t", quoting=csv.QUOTE_NONE)]
    return {"schema": "stdio.tsv", "records": records, "trailingNewline": source.endswith("\n"), "lineEnding": "crlf" if "\r\n" in source else "lf"}


class Tree(html.parser.HTMLParser):
    def __init__(self):
        super().__init__(convert_charrefs=True)
        self.doctype = None
        self.stack = [{"kind": "element", "name": "#document", "children": []}]

    def handle_decl(self, decl):
        self.doctype = decl

    def element(self, tag, attrs):
        node = {"kind": "element", "name": tag}
        if attrs:
            node["attributes"] = [{"name": name} if value is None else {"name": name, "value": value} for name, value in attrs]
        self.stack[-1].setdefault("children", []).append(node)
        return node

    def handle_starttag(self, tag, attrs):
        node = self.element(tag, attrs)
        if tag not in VOID:
            self.stack.append(node)

    def handle_startendtag(self, tag, attrs):
        self.element(tag, attrs)

    def handle_endtag(self, tag):
        if tag not in VOID:
            assert self.stack[-1]["name"] == tag, f"unbalanced </{tag}>"
            self.stack.pop()

    def handle_data(self, data):
        self.stack[-1].setdefault("children", []).append({"kind": "text", "text": data})

    def handle_comment(self, data):
        self.stack[-1].setdefault("children", []).append({"kind": "comment", "text": data})


def html_snapshot(source):
    tree = Tree()
    tree.feed(source)
    tree.close()
    roots = tree.stack[0]["children"]
    assert len(tree.stack) == 1 and len(roots) == 1, "one root element"
    snapshot = {"schema": "stdio.html"}
    if tree.doctype is not None:
        snapshot["doctype"] = tree.doctype
    snapshot["root"] = roots[0]
    return snapshot


def main():
    fixture = json.load(open(sys.argv[1], encoding="utf-8"))
    assert fixture["schema"] == "semio.stdio.native-text-codecs/v1"
    derive = {"s.stdio.txt": txt, "s.stdio.tsv": tsv, "s.stdio.html": html_snapshot}
    failures = 0
    for case in fixture["cases"]:
        actual = derive[case["artifactKind"]](case["source"])
        ok = actual == case["snapshot"]
        failures += not ok
        print(f"{'ok  ' if ok else 'FAIL'} {case['id']}" + ("" if ok else f"\n  oracle:  {json.dumps(actual, ensure_ascii=False)}\n  fixture: {json.dumps(case['snapshot'], ensure_ascii=False)}"))
    print(f"cx1-oracle: {len(fixture['cases']) - failures}/{len(fixture['cases'])} cases equal the Python stdlib readers (csv, html.parser, str.splitlines)")
    sys.exit(1 if failures else 0)


main()
