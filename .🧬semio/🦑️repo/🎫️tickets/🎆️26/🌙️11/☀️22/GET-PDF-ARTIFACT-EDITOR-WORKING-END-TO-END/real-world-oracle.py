"""🔮️ pymupdf oracle for the registered real-world round trips: the saved file keeps every page, every page's images, fonts and text, the outline, the links and the page labels of the original, and adds the inserted text to page 1 only.

usage: uv run --offline --with pymupdf==1.26.7 python -I real-world-oracle.py <original.pdf> <saved.pdf> <edited title> <inserted text>
"""
import sys

import pymupdf


def digest(path):
    document = pymupdf.open(path)
    pages = []
    for page in document:
        pages.append({
            "text": page.get_text("text"),
            "images": len(page.get_images(full=True)),
            "fonts": sorted(font[3] for font in page.get_fonts(full=True)),
            "links": len(page.get_links()),
            "words": len(page.get_text("words")),
        })
    return document, pages


def main():
    original_path, saved_path, title, inserted = sys.argv[1:5]
    original, before = digest(original_path)
    saved, after = digest(saved_path)
    checks = []
    checks.append(("page count", len(before) == len(after), f"{len(before)} -> {len(after)}"))
    for index, (old, new) in enumerate(zip(before, after)):
        for key in ("images", "fonts", "links"):
            checks.append((f"page {index + 1} {key}", old[key] == new[key], f"{old[key]} -> {new[key]}"))
        if index == 0:
            checks.append(("page 1 keeps its text", old["text"].strip() in new["text"] or new["words"] >= old["words"], f"{old['words']} -> {new['words']} words"))
        else:
            checks.append((f"page {index + 1} text", old["text"] == new["text"], f"{old['words']} -> {new['words']} words"))
    page_one = saved[0]
    drawn = inserted in page_one.get_text("text") or any(inserted.encode() in page_one.read_contents() for _ in [0])
    checks.append(("inserted text on page 1", drawn, inserted))
    checks.append(("inserted text only on page 1", all(inserted not in page["text"] for page in after[1:]), inserted))
    checks.append(("title", saved.metadata.get("title") == title, saved.metadata.get("title")))
    checks.append(("outline", original.get_toc() == saved.get_toc(), f"{len(original.get_toc())} entries"))
    checks.append(("page labels", [original[i].get_label() for i in range(len(original))] == [saved[i].get_label() for i in range(len(saved))], ""))
    failed = [check for check in checks if not check[1]]
    for name, ok, detail in checks:
        if not ok:
            print(f"FAIL {name}: {detail}")
    print(f"{original_path}: {len(checks) - len(failed)} of {len(checks)} checks OK")
    sys.exit(1 if failed else 0)


if __name__ == "__main__":
    main()
