"""📝️ Rewrites the `adapter()` doc comment of every converted Rust adapter: the per-kind registration prose becomes the
outline-registration statement, every other sentence is kept, and the block is re-wrapped.

Usage: registration-docs.py [--write] <adapter.rs>..."""
import re, sys, textwrap
write = "--write" in sys.argv
OUTLINE = "Handlers are registered under the Scenario Outline base ids, which the host resolves for every Examples row, and plain scenarios under their own ids."
PATTERNS = [
    r"`mutate-<kind>`/`inverse-<kind>` share ONE handler per role across all \d+ kinds (?:--|—) the scenario id only selects which (?:fixture|Examples|`Examples`) row's `<id>`/`<params>` doc string the shared handler reads(?:, per `Adapter::oracle`/`subject`'s own per-scenario dispatch table)?\.",
    r"Registers by (?:the )?FULL expanded scenario id \(`mutate-<kind>` ?/ ?`inverse-<kind>`\),? (?:never the outline's base id — a missing registration is a hard error, never a skip|one loop iteration per declared `KINDS` entry, plus the standalone identity round trip)\.",
    r"Registration is by FULL expanded scenario id, so (?:the loop mirrors the feature's `Examples` tables exactly|the whole vocabulary is registered in one loop|the outline's rows are enumerated here rather than its base id|both roles are registered in one loop over the subset's own `KINDS` — the same list the oracle module's `kinds_matches_the_catalog_and_every_feature_row` pins against the catalog and the feature's `Examples` rows)\.",
    r"One `mutate-<kind>`/`inverse-<kind>` pair per declared kind, plus the standalone `identity-round-trip` scenario\.",
]
INLINE = [(r", by FULL expanded scenario id — the loop mirrors the feature's `Examples` tables exactly\.", "."), (r", by FULL expanded scenario id\.", ".")]
for path in [a for a in sys.argv[1:] if not a.startswith("--")]:
    text = open(path, encoding="utf-8").read()
    m = re.search(r"((?:[ \t]*///[^\n]*\n)+)(pub fn adapter\(\))", text)
    if not m or "for kind in" in text[m.end():]:
        continue
    prose = " ".join(line.strip()[3:].strip() for line in m.group(1).strip("\n").split("\n"))
    new = prose
    for pattern, replacement in INLINE:
        new = re.sub(pattern, replacement, new)
    for pattern in PATTERNS:
        new = re.sub(pattern, OUTLINE, new)
    if new == prose and new == re.sub(pattern, "", new):
        pass
    if OUTLINE not in new and new != prose:
        new = new.rstrip() + " " + OUTLINE
    elif new == prose and ("FULL" in prose or "<kind>" in prose):
        print("UNMATCHED", path.split("/🧪️tests/")[-1], prose[:160]); continue
    if new == prose:
        continue
    block = "\n".join("/// " + line for line in textwrap.wrap(new, 106, break_on_hyphens=False, break_long_words=False)) + "\n"
    text = text.replace(m.group(1), block, 1)
    print("rewrote", path.split("/🧪️tests/")[-1])
    if write:
        open(path, "w", encoding="utf-8").write(text)
