from pathlib import Path

here = Path(__file__).resolve().parent
head = (here / "main_head.go").read_text(encoding="utf-8").splitlines(keepends=True)
# 1-based inclusive line numbers from committed main.go
start, end = 27637, 28813
chunk = "".join(head[start - 1 : end])
out = here / "autofix_snippet.go"
out.write_text(chunk, encoding="utf-8", newline="\n")
print("wrote", out, "lines", len(chunk.splitlines()))
