from pathlib import Path
import json
import sys

ticket = Path(__file__).parent
root = ticket
while not (root / "🧰️framework/🛍️products/📓️print").exists():
    root = root.parent
assignment = root / "🧰️framework/🛍️products/📓️print/🧾️template/📝️assignment"
for language in ("de", "en"):
    content = (assignment / f"parametrization-{language}.content.tex").read_text(encoding="utf-8")
    assert content.count(r"\section{") == 1
    assert content.count(r"\subsection{") == 2
    assert content.count(r"\clearpage") == 2
    assert content.count(r"\AssignmentWorkedExampleTable") == 1
    assert content.count(r"\AssignmentWorkedExampleGraph") == 1
    assert content.count(r"\AssignmentWorkedExampleLedger") == 1
    assert content.count(r"\AssignmentExerciseTable") == 2
    assert content.count(r"\AssignmentCompleteGraph") == 1
    assert content.count(r"\AssignmentSelectedLedger") == 1
    assert "WorkedBox" not in content
    assert all(token not in content for token in (r"\maketitle", r"\tableofcontents", r"\SemioAssignmentFrontMatter"))
    assert content.count(r"\Delta L=+1") == 2
    assert r"\Delta L=+2" in content and r"\sum\Delta L=-1" in content
    assert ("positive reelle" if language == "de" else "positive real") in content
    assert ("Kategoriepfeile die Richtung" if language == "de" else "numeric signs and categorical arrows") in content
    assert ("genau einer aktiv" if language == "de" else "exactly one active") in content
if len(sys.argv) > 1:
    import pypdfium2 as pdfium
    document = pdfium.PdfDocument(sys.argv[1])
    assert len(document) == 3, len(document)
    texts = []
    for page in document:
        textpage = page.get_textpage()
        texts.append(" ".join(textpage.get_text_range().split()))
        textpage.close()
        page.close()
    document.close()
    assert "Example 1.1" in texts[0]
    assert "Dwelling Capsules 1.2" in texts[1]
    assert "Table A" in texts[1] and "Table B" in texts[1]
    assert all(word in texts[0] for word in ("Box", "Pyramid", "elliptic cylinder", "positive real", "categorical", "Unique.", "Consistent.", "Minimal."))
    assert "20 marked pairs" in texts[2] and "16 ring edges" in texts[2]
    assert "numeric signs and categorical arrows" in texts[2]
    assert "exactly one active" in texts[0]
    assert "1.3" not in " ".join(texts)
    print(json.dumps({"pages": 3, "text_characters": [len(text) for text in texts], "checks": "Mixed example and preserved English exercise structure passed"}))
else:
    print(json.dumps({"languages": ["de", "en"], "checks": "Mixed example source contracts and preserved structure passed"}))
