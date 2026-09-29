"""💬️ C12: trinity jack's `textEdit` declares its agent-facing description (en + de) — S19's os-mcp manifest audit
(`s14-s19-logs/mcp-search-long-1.txt`: `trinity.s.trinity.jack@1/*#editor.textEdit [missing] declares no description`).
`textEdit` became a DOCUMENT verb with C12's T6 row 13 (the query is document content, coalesced typing = one undo step).
Guest-linked (jack editor manifest) → REGEN: trinity `describe` (the descriptor embeds action semantics).
Idempotent; usage: python3 c12-jack-textedit-description.py [--apply]   (default dry run; C12_REPO overrides the tree)"""
import os
import sys

REPO = os.environ.get("C12_REPO", "/Users/ueli/Documents/semio")
EDITOR = "✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs"
OLD = """            .action_describe("formatDocument", LocalizedLabel::native("""
NEW = """            .action_describe("textEdit", LocalizedLabel::native("Replaces the document's Jack query with the given text; consecutive edits undo as one step and the graph changes only when the query runs.", "Ersetzt die Jack-Abfrage des Dokuments durch den angegebenen Text; aufeinanderfolgende Änderungen werden als ein Schritt rückgängig gemacht, der Graph ändert sich erst beim Ausführen der Abfrage."))
""" + OLD
path = os.path.join(REPO, EDITOR)
text = open(path, encoding="utf-8").read()
if NEW in text:
    print("present textEdit description; 0 problems")
elif text.count(OLD) != 1:
    print("problem: anchor count", text.count(OLD))
    sys.exit(1)
else:
    print("edit    textEdit description; 0 problems", "mode=" + ("apply" if "--apply" in sys.argv else "dry-run"))
    if "--apply" in sys.argv:
        open(path, "w", encoding="utf-8").write(text.replace(OLD, NEW))
        print("written")
