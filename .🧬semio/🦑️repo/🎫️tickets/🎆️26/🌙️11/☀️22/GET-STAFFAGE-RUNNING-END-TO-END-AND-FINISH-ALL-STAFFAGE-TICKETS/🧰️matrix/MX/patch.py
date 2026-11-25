"""🩹 Applies named exact text replacements to one shared file in a single write: `python3 patch.py <file> <edits.json>`; every `old` must occur exactly once in the file as it is now, otherwise nothing is written."""
import json, pathlib, sys

def main() -> None:
    """✍️ Reads the file now, replaces, writes once."""
    target = pathlib.Path(sys.argv[1])
    text = target.read_text("utf8")
    for edit in json.loads(pathlib.Path(sys.argv[2]).read_text("utf8")):
        if text.count(edit["old"]) != 1:
            sys.exit(f"{edit['name']}: found {text.count(edit['old'])} times, nothing written")
        text = text.replace(edit["old"], edit["new"])
    target.write_text(text, "utf8")
    print(f"written {target.name}")

main()
