"""🧲️ Replaces compile-time include_str!/include_bytes! of a test file by the run-time fixture reader of the crate whose manifest directory is given, and adds the dev-dependency.
usage: to-fixture-text.py <manifest-dir> <test.rs>..."""
import os, re, sys
root = os.getcwd()
manifest = sys.argv[1]
reader = os.path.relpath("🧰️framework/🔨️modules/🧪️test/🧲️fixture-reader/📦️packages/🦀️rust", manifest)
used = set()
for path in sys.argv[2:]:
    text = open(path, encoding="utf8").read()
    def swap(match):
        kind = "text" if match.group(1) == "str" else "bytes"
        used.add("fixture_" + kind)
        target = os.path.normpath(os.path.join(os.path.dirname(path), match.group(2)))
        return f'fixture_{kind}!("{os.path.relpath(target, manifest)}")'
    text = re.sub(r'include_(str|bytes)!\("([^"]+)"\)', swap, text)
    names = sorted(used)
    line = "use semio_framework_test_fixtures::" + (names[0] if len(names) == 1 else "{" + ", ".join(names) + "}") + ";\n"
    if "semio_framework_test_fixtures" not in text:
        m = re.search(r"^use .*;\n", text, re.M)
        text = text[:m.end()] + line + text[m.end():] if m else line + text
    open(path, "w", encoding="utf8").write(text)
cargo = os.path.join(manifest, "Cargo.toml")
body = open(cargo, encoding="utf8").read()
if "semio-framework-test-fixtures" not in body:
    entry = f'semio-framework-test-fixtures = {{ path = "{reader}" }}\n'
    if "[dev-dependencies]" in body:
        body = body.replace("[dev-dependencies]\n", "[dev-dependencies]\n" + entry, 1)
    else:
        body = body.rstrip("\n") + "\n\n[dev-dependencies]\n" + entry
    open(cargo, "w", encoding="utf8").write(body)
print("used", sorted(used))
