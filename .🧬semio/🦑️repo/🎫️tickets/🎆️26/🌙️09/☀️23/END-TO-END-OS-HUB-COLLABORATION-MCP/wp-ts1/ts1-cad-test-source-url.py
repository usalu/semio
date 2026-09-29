"""🧭 TS1 one-off: cad + spatial-kernel suites locate their fixtures through the module URL, not Bun's `import.meta.dir`
(undefined under vitest's module runner). Registrations pass `{ url: import.meta.url }`, suites declare
`TestSource = { readonly url: string }` and read `new URL(relative, source.url)` (node:fs accepts file URLs).
Idempotent. usage: python3 ts1-cad-test-source-url.py [--dry-run]"""
import pathlib, subprocess, sys

R = pathlib.Path("/Users/ueli/Documents/semio")
DRY = "--dry-run" in sys.argv
changed = []
sources = subprocess.run(["git", "grep", "-l", "registerTests1(import.meta.vitest", "--", "✏️s/🔌️plugins/📐️cad", "✏️s/🔨️modules/🌐️spatial-kernel"], cwd=R, capture_output=True, text=True, check=True).stdout.split()


def edit(path, pairs):
    p = R / path
    before = p.read_text()
    text = before
    for old, new in pairs:
        if old not in text:
            assert new in text, (path, old[:80])
            continue
        text = text.replace(old, new)
    if text != before:
        changed.append(path)
        if not DRY:
            p.write_text(text)


for source in sources:
    edit(source, [("{ directory: import.meta.dir, url: import.meta.url }", "{ url: import.meta.url }")])
    text = (R / source).read_text()
    marker = 'await import("./🧪️tests/'
    start = text.index(marker) + len('await import(".')
    spec = text[start:text.index('"', start)]
    suite = str(pathlib.Path(source).parent) + spec
    edit(suite, [("type TestSource = { readonly directory: string; readonly url: string };", "type TestSource = { readonly url: string };")])

READERS = {
    "✏️s/🔨️modules/🌐️spatial-kernel/⚙️engine/📐️geometry/🧪️tests/🧪️semio-tech-cad-js-core-vec/🟦️.ts": "readFile",
    "✏️s/🔨️modules/🌐️spatial-kernel/⚙️engine/🧱️brepjs/🧪️tests/🧪️semio-tech-cad-js-brepjs/🟦️.ts": "readFile",
    "✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📺️renderer/🧪️tests/🧪️repluserfacingsuggestiondetail/🟦️.tsx": "readFileSync",
}
for path, reader in READERS.items():
    text = (R / path).read_text()
    pairs = [('      const { resolve } = await import("node:path");\n', "")]
    start = text.find("resolve(source.directory, ")
    if start >= 0:
        literal_start = start + len("resolve(source.directory, ")
        literal_end = text.index('")', literal_start) + 1
        literal = text[literal_start:literal_end]
        pairs.append((f"resolve(source.directory, {literal})", f"new URL({literal}, source.url)"))
    edit(path, pairs)

print(("would change: " if DRY else "changed: ") + "\n".join(changed or ["nothing"]))
