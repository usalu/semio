#!/usr/bin/env python3
"""📎️ Rewrites `include!/include_str!/include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "<rel>"))` into a file-relative literal.

The native Source admission refuses macro-expanded include paths (they need compiler ownership), and a manifest-relative
path silently changes meaning when a crate cut moves the file into another package. Each target is resolved against the
manifest of the package that compiles the file today and re-expressed relative to the including file.

usage: 📎️literal-includes.py [--write] <root>...
"""
import os, re, sys

HERE = os.path.dirname(os.path.abspath(__file__))
REPO = os.path.abspath(os.path.join(HERE, *[".."] * 8))
ns = {"__file__": os.path.join(HERE, "🧾️consumer-manifests.py")}
exec(compile(open(ns["__file__"], encoding="utf-8").read().rsplit("\nmain()", 1)[0], ns["__file__"], "exec"), ns)
owner = ns["owner"]
PATTERN = re.compile(r'(?P<macro>include(?:_str|_bytes)?)!\s*\(\s*concat!\s*\(\s*env!\s*\(\s*"CARGO_MANIFEST_DIR"\s*\)\s*,\s*"(?P<rel>[^"]+)"\s*\)\s*\)')


def main():
    write = "--write" in sys.argv
    roots = [a for a in sys.argv[1:] if not a.startswith("--")]
    for base in roots:
        for dirpath, dirnames, filenames in os.walk(os.path.join(REPO, base)):
            dirnames[:] = [d for d in dirnames if d not in ("node_modules", "target", "🗑️generated", ".git")]
            for filename in filenames:
                if not filename.endswith(".rs"):
                    continue
                path = os.path.join(dirpath, filename)
                text = open(path, encoding="utf-8").read()
                if "CARGO_MANIFEST_DIR" not in text or not PATTERN.search(text):
                    continue
                manifest = owner(path)
                if manifest is None:
                    for includer_dir, _, includer_files in os.walk(os.path.dirname(os.path.dirname(os.path.dirname(path)))):
                        for candidate in includer_files:
                            candidate_path = os.path.join(includer_dir, candidate)
                            if candidate.endswith(".rs") and candidate_path != path:
                                body = open(candidate_path, encoding="utf-8").read()
                                for inc in re.finditer(r'include!\s*\(\s*"([^"]+)"\s*\)', body):
                                    if os.path.normpath(os.path.join(includer_dir, inc.group(1))) == os.path.normpath(path):
                                        manifest = manifest or owner(candidate_path)
                if manifest is None:
                    artifact = re.match(r"(.*/🗿️artifacts/[^/]+)/", path)
                    fallback = os.path.join(artifact.group(1), "📦️packages", "🦀️rust", "Cargo.toml") if artifact else None
                    if not fallback or not os.path.exists(fallback):
                        print(f"UNOWNED {os.path.relpath(path, REPO)}")
                        continue
                    print(f"UNMOUNTED {os.path.relpath(path, REPO)} (resolved against the artifact package)")
                    manifest = fallback
                def literal(m):
                    rel = re.sub(r"\\u\{([0-9a-fA-F]+)\}", lambda e: chr(int(e.group(1), 16)), m.group("rel"))
                    target = os.path.normpath(os.path.join(os.path.dirname(manifest), rel.lstrip("/")))
                    if not os.path.exists(target) and "/🧫️fixtures/" in target and os.path.exists(target.replace("/🧫️fixtures/", "/🧪️tests/🧫️fixtures/", 1)):
                        target = target.replace("/🧫️fixtures/", "/🧪️tests/🧫️fixtures/", 1)
                    if not os.path.exists(target):
                        raise SystemExit(f"missing include target {target} from {path}")
                    return f'{m.group("macro")}!("{os.path.relpath(target, os.path.dirname(path))}")'
                new = PATTERN.sub(literal, text)
                print(f"rewrite {os.path.relpath(path, REPO)}")
                if write:
                    open(path, "w", encoding="utf-8").write(new)


main()
