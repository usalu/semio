"""🧱 Rebuilds the three staged batches of lane xplat5 from the files of the work mirror and the CURRENT live files (base = live now): python3 -I build.py <mirror-root> [journal|witnessenv|winframe ...]. Re-run after any drift of a base file; nothing live is written."""
import os, sys
HERE = os.path.dirname(os.path.abspath(__file__))
C = os.path.dirname(HERE)
sys.path.insert(0, os.path.join(C, "hotpath2-tools"))
import lib
LIVE = os.path.dirname(os.path.dirname(os.path.dirname(os.path.dirname(os.path.dirname(os.path.dirname(os.path.dirname(C)))))))
B = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/"
J = B + "🏡️resident/📜️journal/"
W = B + "🔭️witness/"
G = B + "🕸️graph/🕰️generations/"
BATCHES = {
    "journal": ("🔒️ Staged batch `journal` (lane xplat5): a journal directory whose owner file is not written yet is a starting service (kill(0) signals the process group on POSIX), and the Python oracle of the journal directory runs where Windows has no `pwd` and no `getuid`.", [
        J + "🟦️.ts",
        J + "🧪️tests/🔒️lock/🟦️.ts", J + "🧪️tests/🔒️lock/🐍️oracle/🐍️.py", J + "🧪️tests/🔒️lock/🧫️fixtures/🔣️.json",
        J + "🧪️tests/📍️directory/🟦️.ts", J + "🧪️tests/📍️directory/🐍️oracle/🐍️.py"]),
    "witnessenv": ("🖥️ Staged batch `witnessenv` (lane xplat5): the plan witness ignores the session variables of Linux, devcontainer, Windows and IDE hosts (not only macOS ones), compares names without regard to case, and declares the Windows process table probe of the stall watchdog.", [
        W + "🟨️.mjs", W + "🧬️schema/🔣️.json",
        W + "🧪️tests/🖥️session/🟦️.ts", W + "🧪️tests/🖥️session/🐍️oracle/🐍️.py", W + "🧪️tests/🖥️session/🧫️fixtures/🔣️.json"]),
    "winframe": ("🪟 Staged batch `winframe` (lane xplat5): the native scan frontier of Windows (inode 0, mode 0 = unknown) is compared with `lstat` without a phantom change, UNC roots keep a valid path, and the watcher pin follows.", [
        B + "🏡️resident/🔭️watch/🦀️rust/🦀️.rs", B + "🏡️resident/🔭️watch/🦀️rust/🔣️.json",
        G + "🟨️.mjs", G + "🟨️.d.mts", B + "🧭️invocation/🕸️graph/🧠️memo/🟦️.ts",
        G + "🧪️tests/🪟️native-frontier/🟦️.ts", G + "🧪️tests/🪟️native-frontier/🐍️oracle/🐍️.py", G + "🧪️tests/🪟️native-frontier/🧫️fixtures/🔣️.json",
        B + "🏡️resident/🧪️tests/🔭️watch/🟦️.ts"]),
}
def main():
    mirror = os.path.abspath(sys.argv[1]); wanted = sys.argv[2:] or list(BATCHES)
    for name in wanted:
        title, rels = BATCHES[name]
        batch = os.path.join(HERE, "staged", name); os.makedirs(batch, exist_ok=True)
        lib.build(batch, LIVE, mirror, title, rels, os.path.join(HERE, "verify-" + name + ".sh"))
        text = open(os.path.join(HERE, "stage.template.py"), encoding="utf8").read().replace("🌌 Staged batch `universe` (lane rustedit): print's Bazel plan becomes reusable although a dependency reads a root-anchored pattern.", title)
        open(os.path.join(batch, "stage.py"), "w", encoding="utf8").write(text)
main()
