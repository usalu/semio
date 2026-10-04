"""🚚️ Moves the trailing `#[cfg(test)] mod tests { … }` block of every quiz Rust core file into
`<owner>/🧪️tests/🔬️unit/🦀️.rs` and wires it back with `#[cfg(test)] #[path] mod tests;`.

One-shot refactoring input for the core-rust work package; run from the repo root:
`.venv/Scripts/python.exe .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️28/QUIZ-PRODUCT-AND-TEACHING-PROCTOR/core_rust_extract_tests.py`
"""

import pathlib
import re
import sys

ROOT = pathlib.Path("🧰️framework/🛍️products/❓️quiz")
OWNERS = {
    "🧬️schema": ("🩻️", "the schema twin: serde round trips of every shared fixture document and the shared test kit"),
    "🔨️modules/🎲️randomness": ("🪀️", "the randomness: reference vectors of FNV-1a and MT19937, rejection sampling and shuffle"),
    "🔨️modules/🃏️sheet": ("📑️", "the sheet: RNG consumption order, draws, rotation and the shared sheets; also the shared quiz builders"),
    "🔨️modules/✅️validation": ("🧐️", "the validation: issue paths and codes, answer validity and completeness, schema violations"),
    "🔨️modules/📏️scoring": ("🎳️", "the scoring: pair concordance, profile similarity, run means and the shared scores"),
    "🔨️modules/🏅️badges": ("🎀️", "the badges: every rule, held badges and the shared badge vectors"),
    "🔨️modules/🧾️lifecycle": ("🎞️", "the lifecycle: handles, roster and learner decision tables and the shared sequences; also the replay kit"),
    "🔨️modules/👁️views": ("📺️", "the views: catalog, learner and run views, leaderboard ordering and tags, shared views"),
}
BLOCK = re.compile(r"\n#\[cfg\(test\)\]\n(pub\(crate\) )?mod tests \{\n(.*)\n\}\n?\Z", re.S)


def main() -> int:
    for owner, (emoji, subject) in OWNERS.items():
        source = ROOT / owner / "🦀️.rs"
        text = source.read_text(encoding="utf-8")
        match = BLOCK.search(text)
        if not match:
            print(f"no trailing test block in {source}", file=sys.stderr)
            return 1
        visibility = match.group(1) or ""
        body = "\n".join(line[4:] if line.startswith("    ") else line for line in match.group(2).split("\n"))
        header = f"//! {emoji} Unit tests of {subject}.\n//!\n//! @see ../../🦀️.rs — the implementation under test\n\n"
        target = ROOT / owner / "🧪️tests" / "🔬️unit" / "🦀️.rs"
        target.parent.mkdir(parents=True, exist_ok=True)
        target.write_text(header + body + "\n", encoding="utf-8", newline="\n")
        wiring = f'\n#[cfg(test)]\n#[path = "🧪️tests/🔬️unit/🦀️.rs"]\n{visibility}mod tests;\n'
        source.write_text(text[: match.start()] + wiring, encoding="utf-8", newline="\n")
        print(f"moved {owner}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
