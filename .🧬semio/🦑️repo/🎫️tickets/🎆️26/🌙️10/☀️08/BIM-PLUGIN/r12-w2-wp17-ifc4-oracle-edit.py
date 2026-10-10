import sys
p = sys.argv[1]
s = open(p, encoding="utf-8").read()
def rep(a, b):
    global s
    assert a in s, a[:80]
    s = s.replace(a, b, 1)
rep('''    "wall-depth": ["inferences", "wall-depth", "attic", "snapshot"],
}''', '''    "wall-depth": ["inferences", "wall-depth", "attic", "snapshot"],
}
EXAMPLES = {"example-house": ["assets", "house", "snapshot.json"], "example-office": ["assets", "office", "snapshot.json"]}
"""🏡️ The shipped examples: their snapshot lies in the assets next to the fixtures (the harness reads fixtures only, so these cases run standalone and in the Rust drift test)."""''')
rep('''def snapshot_of(root, case):
    """📸️ The committed snapshot of a case."""
    folder = Path(root)
    for name in CASES[case]:
        folder = child(folder, name)
    return json.loads(child(folder, ".json").read_text(encoding="utf-8"))''', '''def snapshot_of(root, case):
    """📸️ The committed snapshot of a case."""
    folder = Path(root) if case in CASES else Path(root).parent
    for name in CASES.get(case) or EXAMPLES[case]:
        folder = child(folder, name)
    return json.loads((folder if folder.is_file() else child(folder, ".json")).read_text(encoding="utf-8"))''')
rep("for case in arguments[2:] or CASES:", "for case in arguments[2:] or [*CASES, *EXAMPLES]:")
open(p, "w", encoding="utf-8").write(s)
print("ok")
