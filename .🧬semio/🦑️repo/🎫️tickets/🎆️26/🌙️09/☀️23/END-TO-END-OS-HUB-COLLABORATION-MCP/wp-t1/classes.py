import re, sys, collections
RULES = [
    ("go-test-in-package", r"Go test declaration is outside"),
    ("rust-test-outside-canonical", r"Rust (test attribute|self-test declaration) is outside"),
    ("rust-inline-test-module", r"Inline Rust test modules must move"),
    ("rust-test-wiring", r"External Rust test modules must use #\[path\]|Rust test includes must use"),
    ("test-source-depth", r"Executable test sources must be direct children"),
    ("adapter-filename", r"implementation filename must be one of"),
    ("test-registration", r"Test registration is outside a canonical"),
    ("vitest-wiring", r"import\.meta\.vitest wiring"),
    ("self-test-declaration", r"Self-test declaration .* is outside"),
    ("delivery-scope-owner", r"test owner is below delivery scope"),
    ("test-case-name", r"test case directory must use one canonical emoji|Test case directory .* needs"),
    ("js-test-outside-canonical", r"(TypeScript|JavaScript|Vitest|Bun|test framework).* is outside a canonical|outside a canonical test implementation"),
    ("legacy-test-filename", r"Legacy test filename pattern"),
    ("legacy-test-directory", r"Legacy test directory"),
    ("obsolete-test-category", r"Obsolete testing category"),
    ("oracle-in-production", r"Production source imports the registered oracle"),
]
def rows(path):
    for line in open(path, encoding="utf-8"):
        m = re.match(r"^  (testing/\S+)  (\S+)  (.*)$", line.rstrip("\n"))
        if m: yield m.groups()
def cls(detail):
    for name, pattern in RULES:
        if re.search(pattern, detail): return name
    return "other"
def count(path):
    c = collections.Counter(cls(d) for _, _, d in rows(path)); return c
if len(sys.argv) == 2:
    c = count(sys.argv[1]); print(sum(c.values()))
    for k, n in c.most_common(): print(f"{n:6d} {k}")
else:
    a, b = count(sys.argv[1]), count(sys.argv[2])
    print(f"total {sum(a.values())} -> {sum(b.values())}")
    for k in sorted(set(a) | set(b), key=lambda k: -a[k]): print(f"{a[k]:6d} -> {b[k]:6d}  {b[k]-a[k]:+6d}  {k}")
