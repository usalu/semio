from pathlib import Path

root = Path(r"c:\git\semio")
main = root / "repo" / "client" / "cli" / "main.go"
snippet_path = root / ".repo" / "\U0001f3ab" / "26" / "05" / "13" / "REFACTOR-LIB-RS-TO-GOLDEN-SCHEMA-PARTIAL" / "autofix_snippet.go"
lines = main.read_text(encoding="utf-8").splitlines(keepends=True)
snippet = snippet_path.read_text(encoding="utf-8")
snippet = snippet.replace(
    "// ÔûÂ´©ÅfindMatchingSectionStartName holds the data fields for a findMatchingSectionStartName record.",
    "// findMatchingSectionStartName locates the section start name for autofix helpers.",
)
insert_at = None
for i, line in enumerate(lines):
    if line.startswith("func (c *repoContext) TicketOpen(input TicketOpenInput)"):
        insert_at = i - 2  # keep the two TicketOpen comment lines immediately above func
        break
if insert_at is None or insert_at < 0:
    raise SystemExit("TicketOpen not found")
if any("func applyAutofixes" in line for line in lines):
    raise SystemExit("already patched")
new_lines = lines[:insert_at] + ["\n"] + snippet.splitlines(keepends=True) + ["\n"] + lines[insert_at:]
main.write_text("".join(new_lines), encoding="utf-8", newline="\n")
print("inserted at", insert_at)
