"""💬 Appends the answers of MX under their requests in `requests.md`, in one write: `python3 answer.py <requests.md> <answers.json> <time>`; an answer is `{ "heading": "<start of the ## line>", "text": "…" }`, placed at the end of the section of the one heading that starts so; a section that already carries the same text is left alone, and nothing is written when a heading is missing or ambiguous."""
import json, pathlib, sys


def main() -> None:
    """✍️ Reads the file now, places every answer, writes once."""
    target = pathlib.Path(sys.argv[1])
    lines = target.read_text("utf8").split("\n")
    placed = 0
    for answer in json.loads(pathlib.Path(sys.argv[2]).read_text("utf8")):
        starts = [i for i, line in enumerate(lines) if line.startswith("## ") and line[3:].startswith(answer["heading"])]
        if len(starts) != 1:
            sys.exit(f"{answer['heading']}: {len(starts)} headings, nothing written")
        end = next((i for i in range(starts[0] + 1, len(lines)) if lines[i].startswith("## ")), len(lines))
        if any(answer["text"] in line for line in lines[starts[0]:end]):
            continue
        while end > starts[0] + 1 and lines[end - 1].strip() == "":
            end -= 1
        lines[end:end] = ["", f"**Answer MX (2026-11-25 {sys.argv[3]}):** {answer['text']}"]
        placed += 1
    target.write_text("\n".join(lines), "utf8")
    print(f"{placed} answers placed")


main()
