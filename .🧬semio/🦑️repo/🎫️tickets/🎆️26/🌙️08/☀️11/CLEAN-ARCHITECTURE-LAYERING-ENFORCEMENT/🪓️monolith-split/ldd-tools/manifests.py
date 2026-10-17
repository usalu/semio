"""📦️ Rewrites the layout manifests: drops every stdio dependency, adds the framework ports and pixels crates where the sources use them (text edit, asserts each removal)."""
import re

B = "✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/"
S = B + "🏅️standards/🔖️1/🪆️subsets/✳️any/"
PORTS = 'semio-framework-io-ports = { workspace = true }'
PORTS_TESTKIT = 'semio-framework-io-ports = { workspace = true, features = ["testkit"] }'
PIXELS = 'semio-framework-pixels = { workspace = true }'

PLAN = {
    S + "✏️editor/⚙️engine/📤️export/📦️packages/🦀️rust/Cargo.toml": dict(remove=["png", "semio-base", "semio-drawing"], deps=[PIXELS], dev=[PORTS_TESTKIT]),
    S + "✏️editor/⚙️engine/📤️export/🔣️json/📦️packages/🦀️rust/Cargo.toml": dict(remove=["semio-base", "semio-drawing"], deps=[PORTS], dev=[]),
    S + "✏️editor/⚙️engine/📦️packages/🦀️rust/Cargo.toml": dict(remove=["png", "semio-drawing", "semio-base"], deps=[PORTS, PIXELS], dev=[PORTS_TESTKIT]),
    S + "✏️editor/🎮️commands/📦️packages/🦀️rust/Cargo.toml": dict(remove=["png", "semio-base", "semio-drawing"], deps=[], dev=[PORTS_TESTKIT]),
    S + "✏️editor/📦️packages/🦀️rust/Cargo.toml": dict(remove=["semio-base", "semio-drawing"], deps=[PIXELS], dev=[PORTS_TESTKIT]),
    S + "🚪️io/📦️packages/🦀️rust/Cargo.toml": dict(remove=["dwg-geometry", "dwg-read", "dwg-snapshot", "dwg-write", "json", "semio-base", "semio-drawing", "semio-drawing-export", "semio-drawing-import", "svg", "png"], deps=[PORTS], dev=[]),
    S + "🧬️schema/🔺️diff/📦️packages/🦀️rust/Cargo.toml": dict(remove=["semio-drawing"], deps=[], dev=[]),
    S + "🧬️schema/🧬️mutations/🖼️frames/📦️packages/🦀️rust/Cargo.toml": dict(remove=["semio-drawing", "semio-base"], deps=[], dev=[PORTS_TESTKIT]),
    S + "🧬️schema/📦️packages/🦀️rust/Cargo.toml": dict(remove=["dwg-geometry", "semio-base", "semio-drawing", "svg"], deps=[PORTS], dev=[PORTS_TESTKIT]),
    B + "📦️packages/🦀️rust/Cargo.toml": dict(remove=["semio-drawing"], deps=[], dev=[PORTS_TESTKIT]),
}


def sections(text):
    out = []
    for m in re.finditer(r"(?m)^\[([^\]]+)\]\s*$", text):
        out.append((m.group(1), m.start(), m.end()))
    return out


def add_line(text, section, line):
    secs = sections(text)
    for i, (name, start, end) in enumerate(secs):
        if name == section:
            stop = secs[i + 1][1] if i + 1 < len(secs) else len(text)
            body = text[end:stop]
            stripped = body.rstrip("\n")
            trailing = body[len(stripped):]
            new_body = stripped + "\n" + line + trailing
            return text[:end] + new_body + text[stop:]
    raise AssertionError("section %s missing" % section)


def edit(path, remove, deps, dev):
    text = open(path, encoding="utf8").read()
    for suffix in remove:
        pat = re.compile(r"(?m)^semio-s-artifact-stdio-%s = \{ workspace = true \}\n" % re.escape(suffix))
        text, n = pat.subn("", text)
        assert n == 1, (path, suffix, n)
    for line in deps:
        name = line.split(" =")[0]
        assert name + " =" not in text.split("[dev-dependencies]")[0], (path, name)
        text = add_line(text, "dependencies", line)
    for line in dev:
        text = add_line(text, "dev-dependencies", line)
    assert "stdio" not in text.replace("semio-s-artifact-stdio-", "X") or "semio-s-artifact-stdio-" not in text, path
    open(path, "w", encoding="utf8").write(text)


if __name__ == "__main__":
    for path, plan in PLAN.items():
        edit(path, **plan)
    ws = B + "Cargo.toml"
    text = open(ws, encoding="utf8").read()
    block = re.compile(r"(?s)\n\[workspace\.dependencies\.semio-s-artifact-stdio-[a-z0-9-]+\]\npath = \"[^\"]+\"\n")
    text, n = block.subn("\n", text)
    assert n == 11, n
    text = re.sub(r"\n{3,}", "\n\n", text)
    anchor = '[workspace.dependencies.semio-framework-job]\npath = "../../../../../🧰️framework/🔨️modules/🧵️job/📦️packages/🦀️rust"\n'
    assert text.count(anchor) == 1
    text = text.replace(anchor, anchor + '\n[workspace.dependencies.semio-framework-io-ports]\npath = "../../../../../🧰️framework/🔨️modules/🚪️io/🔌️ports/📦️packages/🦀️rust"\n\n[workspace.dependencies.semio-framework-pixels]\npath = "../../../../../🧰️framework/🔨️modules/🔲️pixels/📦️packages/🦀️rust"\n')
    open(ws, "w", encoding="utf8").write(text)
