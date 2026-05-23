from pathlib import Path

root = Path(r"c:\git\semio")
main = root / "repo" / "client" / "cli" / "main.go"
snippet_path = root / ".repo" / "\U0001f3ab" / "26" / "05" / "13" / "REFACTOR-LIB-RS-TO-GOLDEN-SCHEMA-PARTIAL" / "autofix_snippet.go"
text = main.read_text(encoding="utf-8")
snippet = snippet_path.read_text(encoding="utf-8")
# normalize garbled comment from mojibake export
snippet = snippet.replace(
    "// ÔûÂ´©ÅfindMatchingSectionStartName holds the data fields for a findMatchingSectionStartName record.",
    "// findMatchingSectionStartName locates the section start name for autofix helpers.",
)
marker = "\n\n// \U0001f4ecTicketOpen MUST return a non-nil error when the operation fails.\n// \u26abTicketOpen performs the ticket open operation on the repo context.\nfunc (c *repoContext) TicketOpen"
if marker not in text:
    raise SystemExit("marker not found")
pre, post = text.split(marker, 1)
if "func applyAutofixes" in pre:
    raise SystemExit("already inserted")
# trim trailing whitespace from pre then add snippet
pre = pre.rstrip() + "\n\n" + snippet.rstrip() + "\n"
text = pre + marker + post
main.write_text(text, encoding="utf-8", newline="\n")
print("inserted autofix helpers")
