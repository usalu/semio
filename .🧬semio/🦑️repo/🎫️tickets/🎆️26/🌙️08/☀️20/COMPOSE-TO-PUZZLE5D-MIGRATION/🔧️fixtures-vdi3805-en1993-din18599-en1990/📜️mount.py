"""🔌️ Appends one artifact's `#[cfg(test)] #[path = "."] mod fixture_tests { … }` block to its own
mutations-root component, so `📦️glue.rs` (shared with the other norm lanes) stays untouched."""
import io, os, sys

MARKER = "//#region 🧪️FixtureTests"


def mount(root_rs, lines, artifact_label):
    text = io.open(root_rs, encoding="utf-8").read()
    head = (text.split(MARKER)[0] if MARKER in text else text).rstrip("\n")
    block = [
        MARKER,
        "// 🧪️ Self-wired fixture cases for the {} mutation vocabulary: one handcrafted case per".format(artifact_label),
        "// triad leaf, mounted here rather than in `📦️glue.rs` because that file is shared by all",
        "// fifteen norm artifacts and several lanes edit it at once. `#[path = \".\"]` keeps the",
        "// inline module's own name out of the base directory, so every leaf path below is read",
        "// straight off this `🧬️mutations/` directory (ticket 26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION,",
        "// contract D1).",
        "#[cfg(test)]",
        '#[path = "."]',
        "mod fixture_tests {",
    ] + lines + ["}", MARKER.replace("//#region", "//#endregion"), ""]
    io.open(root_rs, "w", encoding="utf-8").write(head + "\n\n" + "\n".join(block))


if __name__ == "__main__":
    root_rs, label = sys.argv[1], sys.argv[2]
    rows = [line for line in sys.stdin.read().splitlines() if line.strip()]
    mount(root_rs, rows, label)
    print("mounted {} lines into {}".format(len(rows), root_rs))
