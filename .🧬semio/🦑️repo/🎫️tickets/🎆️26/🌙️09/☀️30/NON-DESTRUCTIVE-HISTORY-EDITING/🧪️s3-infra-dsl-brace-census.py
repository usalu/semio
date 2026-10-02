"""🧮️ S3-INFRA: census of DSL carriers that still hold bare record lists (`[ key=v … ]`).

The current decoder (`🗣️dsl/🧬️schema/🛬️decoding/🦀️.rs`, `Shape::List(Shape::Record)`) requires every record of a record list
braced (`[ { key=v } { key=v } ]`). A list whose first significant token pair is `ident =` is therefore a bare record list and
cannot parse. Carriers: every committed `*.dsl.semio`, plus every `include_str!` target of a Rust file that is not JSON/TOML/
Markdown/source code. Strings are skipped by the tokenizer; fenced embeds (```…```) are skipped explicitly.

Usage: python3 🧪️s3-infra-dsl-brace-census.py <out.tsv>   (cwd = repo root)
"""
import os
import re
import subprocess
import sys

TOKEN = re.compile(r'\s+|```.*?```|"(?:[^"\\]|\\.)*"|[\[\]{}()=]|[^\s\[\]{}()="`]+|`', re.S)
IDENT = re.compile(r"[A-Za-z_][A-Za-z0-9_-]*$")
INCLUDE = re.compile(r'include_str!\(\s*"([^"]+)"\s*\)')
SKIP_EXT = (".json", ".toml", ".md", ".rs", ".ts", ".tsx", ".js", ".py", ".wgsl", ".html", ".css", ".svg", ".xml", ".csv", ".txt", ".yaml", ".yml", ".wit", ".sql", ".glsl", ".obj", ".stl", ".ply", ".gltf", ".mtl", ".dxf", ".ifc", ".step", ".stp", ".graphql", ".proto")


def bare_lists(text):
    toks = [m.group(0) for m in TOKEN.finditer(text)]
    sig = [t for t in toks if not t.isspace()]
    count = 0
    for index, tok in enumerate(sig[:-2]):
        if tok == "[" and IDENT.match(sig[index + 1]) and sig[index + 2] == "=":
            count += 1
    return count


def tracked(pattern):
    out = subprocess.run(["git", "ls-files", "-co", "--exclude-standard", "--", pattern], capture_output=True, text=True, check=True).stdout
    return [line for line in out.split("\n") if line and "🎫️tickets" not in line and "/target/" not in line and "node_modules" not in line]


def main(out_path):
    carriers = {path: "dsl.semio" for path in tracked("*.dsl.semio")}
    for rust in tracked("*.rs"):
        try:
            source = open(rust, encoding="utf-8").read()
        except (OSError, UnicodeDecodeError):
            continue
        for match in INCLUDE.finditer(source):
            target = os.path.normpath(os.path.join(os.path.dirname(rust), match.group(1)))
            if target.endswith(SKIP_EXT) or target in carriers or not os.path.isfile(target):
                continue
            carriers[target] = "include_str:" + rust
    rows = []
    for path, origin in sorted(carriers.items()):
        try:
            text = open(path, encoding="utf-8").read()
        except (OSError, UnicodeDecodeError):
            continue
        rows.append((bare_lists(text), path, origin))
    bad = [row for row in rows if row[0]]
    with open(out_path, "w", encoding="utf-8") as out:
        for count, path, origin in rows:
            out.write(f"{count}\t{path}\t{origin}\n")
    print(f"[DEBUG] carriers {len(rows)} (dsl.semio {sum(1 for r in rows if r[2] == 'dsl.semio')}, include_str {sum(1 for r in rows if r[2] != 'dsl.semio')}) | with bare record lists {len(bad)} | bare lists {sum(r[0] for r in bad)}")
    for count, path, origin in bad:
        print(f"[DEBUG] {count}\t{path}\t{origin if origin == 'dsl.semio' else 'include_str'}")


if __name__ == "__main__":
    main(sys.argv[1])
