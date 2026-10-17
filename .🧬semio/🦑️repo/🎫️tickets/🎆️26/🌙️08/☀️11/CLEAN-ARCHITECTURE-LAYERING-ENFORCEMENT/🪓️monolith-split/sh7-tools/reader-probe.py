"""🔬 sh7 reader probe: resolve the `include_str!`/`concat!` facet-leaf expression of a descriptor file the way rustc would, and compare the text with the pre-split monolith.
usage: reader-probe.py <descriptor.rs> <facet-key> <head-blob-spec> <needle> [<needle> ...] [--index]
`--index` reads the descriptor from the git index (the state before this slice's edit) instead of the working tree.
Exit code 0 when every needle is found in the resolved leaf text and no non-wiring line of the monolith is missing."""
import collections, os, re, subprocess, sys

ROOT = "/Users/ueli/Documents/semio"


def git(*args):
    return subprocess.run(["git", "-c", "core.fsmonitor=false", "-c", "core.quotepath=false", *args], cwd=ROOT, capture_output=True, check=True).stdout.decode("utf-8")


def facet_expr(text, key):
    start = 0
    *anchors, leafkey = key.split(".")
    for a in anchors:
        start = text.index(a + ":", start)
    m = re.compile(r"\b" + re.escape(leafkey) + r":\s*").search(text, start)
    assert m, key
    i = m.end()
    depth, k = 0, i
    while k < len(text):
        c = text[k]
        if c == "(":
            depth += 1
        elif c == ")":
            depth -= 1
        elif c == "," and depth == 0:
            break
        elif c == "}" and depth == 0:
            break
        k += 1
    return text[i:k]


def main():
    args = [a for a in sys.argv[1:] if a != "--index"]
    use_index = "--index" in sys.argv
    desc, key, head_spec, needles = args[0], args[1], args[2], args[3:]
    text = git("show", ":" + desc) if use_index else open(ROOT + "/" + desc, encoding="utf-8").read()
    expr = facet_expr(text, key)
    paths = re.findall(r'include_str!\(\s*"([^"]+)"\s*\)', expr)
    base = os.path.dirname(ROOT + "/" + desc)
    leaf = "".join(open(os.path.normpath(os.path.join(base, p)), encoding="utf-8").read() for p in paths)
    print(f"facet {key}: {len(paths)} include_str! file(s), {len(leaf.splitlines())} lines, {len(leaf.encode())} bytes ({'index' if use_index else 'worktree'})")
    ok = True
    for n in needles:
        hit = n in leaf
        print(f"  needle {'FOUND  ' if hit else 'MISSING'} {n}")
        ok &= hit
    head = git("show", head_spec)
    strip = lambda t: collections.Counter(l.strip().replace("pub(super) ", "") for l in t.split("\n") if l.strip())
    lost = strip(head) - strip(leaf)
    wiring = re.compile(r'^(#\[path = ".*"\]|mod \w+;|pub use \w+::\*;|.*FacetLeaves \{.*|typescript: include_str!\(.*\),)$')
    real = {l: n for l, n in lost.items() if not wiring.match(l)}
    print(f"  monolith lines missing from the leaf text: {sum(lost.values())} total, {sum(real.values())} not mount wiring")
    for l, n in list(real.items())[:5]:
        print("    e.g.", n, l[:100])
    sys.exit(0 if ok and not real else 1)


main()
