#!/usr/bin/env python3
"""Track 3 Wave A item #4: replaces each crate's `impl store::ConfigRecord for X {}` +
`impl protocol::OperationDiff<X> for X { fn apply(...) -> X { self.clone() } fn absorb(...) {
*self = other; } }` pair with one `store::impl_whole_record_config!(X);` call. Only touches files
where the OperationDiff body EXACTLY matches the trivial whole-record shape (verified via a strict
regex over the whitespace-collapsed impl block) — anything else is left untouched and reported."""
import re
import subprocess

ROOT = "/Users/ueli/Documents/semio/"

MARKER_RE = re.compile(r'impl store::ConfigRecord for (\w+) \{\}\n')


def find_matching_brace(text, open_idx):
    depth = 0
    i = open_idx
    while i < len(text):
        if text[i] == "{":
            depth += 1
        elif text[i] == "}":
            depth -= 1
            if depth == 0:
                return i
        i += 1
    raise ValueError("unbalanced braces")


def process(path):
    with open(path, "r", encoding="utf-8") as f:
        text = f.read()
    m = MARKER_RE.search(text)
    if not m:
        return None
    ty = m.group(1)
    marker_start, marker_end = m.start(), m.end()

    # Look for the OperationDiff impl starting right after the marker, tolerating an intervening
    # doc-comment block.
    after = text[marker_end:]
    od_re = re.compile(r'\A(?:\n)*(?:/// .*\n)*impl protocol::OperationDiff<' + re.escape(ty) + r'> for ' + re.escape(ty) + r' \{\n')
    od_m = od_re.match(after)
    if not od_m:
        return f"SKIP ({ty}): no adjacent OperationDiff impl found"

    # od_m's match ends with the literal "{\n" of the impl block's own opening brace, so that
    # brace sits 2 chars before the match end — NOT found by searching forward from od_m.end()
    # (which would instead find the first *inner* fn body's brace).
    od_open_abs = marker_end + od_m.end() - 2
    assert text[od_open_abs] == "{", "expected impl block opening brace"
    od_close_abs = find_matching_brace(text, od_open_abs) + 1

    body = text[od_open_abs : od_close_abs]
    collapsed = re.sub(r'\s+', '', body)
    expected = f"{{fnapply(&self,_base:&{ty})->{ty}{{self.clone()}}fnabsorb(&mutself,other:Self){{*self=other;}}}}"
    if collapsed != expected:
        return f"SKIP ({ty}): OperationDiff body doesn't match trivial whole-record shape"

    new_text = text[:marker_start] + f"store::impl_whole_record_config!({ty});\n" + text[od_close_abs:]
    with open(path, "w", encoding="utf-8") as f:
        f.write(new_text)
    return f"FIXED ({ty})"


def main():
    result = subprocess.run(
        ["grep", "-rl", "impl store::ConfigRecord for", ROOT + "✏️s"],
        capture_output=True, text=True,
    )
    files = [f for f in result.stdout.splitlines() if f.endswith(".rs") and "node_modules" not in f]
    fixed = 0
    for f in sorted(files):
        outcome = process(f)
        rel = f[len(ROOT):]
        print(f"  {outcome}: {rel}" if outcome and not outcome.startswith("FIXED") else f"  {outcome}: {rel}")
        if outcome and outcome.startswith("FIXED"):
            fixed += 1
    print(f"\n{fixed}/{len(files)} migrated")


if __name__ == "__main__":
    main()
