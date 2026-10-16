import os, sys
root = "/Users/ueli/Documents/semio"
tsv, apply = sys.argv[1], sys.argv[2] == "apply"
rows = [line.rstrip("\n").split("\t") for line in open(tsv, encoding="utf-8") if line.startswith("MISSING")]
done = skipped = 0
for _, file, lineno, literal, normalized in rows:
    marker = "/🧫️fixtures/"
    if marker not in normalized:
        print("SKIP no-fixtures-segment", file, lineno, normalized); skipped += 1; continue
    index = normalized.rindex(marker)
    candidate = normalized[:index] + "/🧪️tests" + normalized[index:]
    if not os.path.exists(os.path.join(root, candidate)):
        print("SKIP candidate-missing", file, lineno, candidate); skipped += 1; continue
    assert literal.count(marker) == 1, literal
    new_literal = literal.replace(marker, "/🧪️tests/🧫️fixtures/")
    path = os.path.join(root, file)
    lines = open(path, encoding="utf-8").read().split("\n")
    line = lines[int(lineno) - 1]
    assert line.count(literal) == 1, (file, lineno)
    lines[int(lineno) - 1] = line.replace(literal, new_literal)
    if apply:
        open(path, "w", encoding="utf-8").write("\n".join(lines))
    print("OK", file, lineno, new_literal); done += 1
print("done", done, "skipped", skipped)
