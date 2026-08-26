"""📃️ One-off derivation (ticket 26/08/23/END-TO-END-TESTING-REFACTOR, wave 22).

Reads the REAL committed article
`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🌐️html/🧫️fixtures/🌐️zukunft-bau-entwerfen-mit-bestand.html` —
the real 150 KB TYPO3-published German page "Zukunft Bau: Entwerfen mit Bestand", already committed
as this repository's own HTML 5 fixture — with Python's own stdlib `html.parser`, and maps its real
inline structure onto the `s.stdio.semio.text` run/mark model: `<a href>` becomes a `link` mark
carrying the real href, `<strong>`/`<b>` a `bold` mark, `<em>`/`<i>` an `italic` mark, `<code>` a
`code` mark, and every element's nearest `lang` attribute (defaulting to the document element's
`lang="de"`) becomes the run's language. Nothing is invented: every run's content is a real text
node of the real page and every href is a real URL the page really links to.

Writes the pair into `mutate-semio-text/🧫️fixtures/` through the case's own INDEPENDENT Python
implementation of the carrier.
"""

import html.parser
import importlib.util
import sys
from pathlib import Path

ROOT = Path("/Users/ueli/Documents/semio")
SOURCE = ROOT / "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🌐️html/🧫️fixtures/🌐️zukunft-bau-entwerfen-mit-bestand.html"
CASE = ROOT / "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🧪️tests/mutate-semio-text"

MARK_OF_TAG = {"a": "link", "strong": "bold", "b": "bold", "em": "italic", "i": "italic", "code": "code"}
SKIP = {"script", "style", "head", "meta", "link", "title", "noscript", "svg", "path", "iframe"}


class Runs(html.parser.HTMLParser):
    """📖️ Collects one run per real text node, carrying the marks its open ancestors imply."""

    def __init__(self):
        super().__init__(convert_charrefs=True)
        self.stack = []
        self.language = ["de"]
        self.skip = 0
        self.runs = []

    def handle_starttag(self, tag, attrs):
        attributes = dict(attrs)
        if tag in SKIP:
            self.skip += 1
            return
        if self.skip:
            return
        if "lang" in attributes:
            self.language.append(attributes["lang"])
            self.stack.append((tag, None, True))
            return
        if tag in MARK_OF_TAG:
            self.stack.append((tag, {"kind": MARK_OF_TAG[tag], "href": attributes.get("href", "") if tag == "a" else ""}, False))

    def handle_endtag(self, tag):
        if tag in SKIP:
            self.skip = max(0, self.skip - 1)
            return
        if self.skip:
            return
        for at in range(len(self.stack) - 1, -1, -1):
            if self.stack[at][0] == tag:
                if self.stack[at][2]:
                    self.language.pop()
                del self.stack[at]
                return

    def handle_data(self, data):
        if self.skip:
            return
        content = " ".join(data.split())
        if not content:
            return
        self.runs.append({"language": self.language[-1], "content": content, "marks": [mark for _, mark, tagged in self.stack if mark is not None and not tagged]})


def load_oracle():
    stub = type(sys)("semio_repo_test")
    stub.Adapter = type("Adapter", (), {"__init__": lambda self, name: None, "oracle": lambda self, *a: self})
    stub.Context = object
    stub.Outcome = object
    stub.digest = lambda data: ""
    sys.modules["semio_repo_test"] = stub
    spec = importlib.util.spec_from_file_location("text_oracle", CASE / "🐍️component.py")
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def main():
    parser = Runs()
    parser.feed(SOURCE.read_text(encoding="utf-8", errors="replace"))
    document = {"schema": "s.stdio.semio.text", "runs": parser.runs}
    oracle = load_oracle()
    dsl = oracle.print_dsl(document)
    assert oracle.parse_dsl(dsl) == document, "the printed DSL does not re-parse to the same document"
    pack = oracle.pack_bytes(document)
    assert oracle.parse_pack(pack) == document, "the encoded pack does not re-decode to the same document"
    out = CASE / "🧫️fixtures"
    out.mkdir(exist_ok=True)
    (out / "🗣️zukunft-bau-entwerfen-mit-bestand.dsl.semio").write_text(dsl, encoding="utf-8")
    (out / "🎒️zukunft-bau-entwerfen-mit-bestand.pack.semio").write_bytes(pack)
    marked = [(i, r) for i, r in enumerate(document["runs"]) if r["marks"]]
    print("runs=%d marked=%d languages=%r" % (len(document["runs"]), len(marked), sorted({r["language"] for r in document["runs"]})))
    print("mark kinds:", sorted({m["kind"] for r in document["runs"] for m in r["marks"]}))
    print("dsl bytes=%d pack bytes=%d" % (len(dsl.encode("utf-8")), len(pack)))
    for i, r in marked[:6]:
        print("  marked run", i, r["language"], repr(r["content"][:60]), r["marks"])
    print("  last run", len(document["runs"]) - 1, document["runs"][-1])
    multi = [(i, r) for i, r in enumerate(document["runs"]) if len(r["marks"]) > 1]
    print("runs with >1 mark:", multi[:4])


if __name__ == "__main__":
    main()
