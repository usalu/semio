import json, os, re, shutil, subprocess, sys

WRITE = "--write" in sys.argv
ROOT = os.getcwd()
WINDOWS = [
    "✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🪟️windows/🗺️map/🎚️config",
    "✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/✒️main/🎚️config",
    "✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🗺️map/🎚️config",
    "✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🕸️canvas/🎚️config",
    "✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🕸️canvas/🫧️transient",
    "✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🌐️graph/🎚️config",
    "✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📊️results/🫧️transient",
    "✏️s/🔌️plugins/🧱️block/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🌐️world/🫧️transient",
]
NEW = {"✅": "✅️ok", "\U0001f7f0": "\U0001f7f0️nop"}
EXTRA_FILES = ["🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧫️fixtures/🧫️mutation-leaf-identity/🔣️.json"]


def subset_root(window):
    return window.split("/✳️any/")[0] + "/✳️any"


def plan():
    rows = []
    for window in WINDOWS:
        fixtures = f"{window}/🧪️tests/🧫️fixtures"
        for kind in sorted(os.listdir(fixtures)):
            kdir = f"{fixtures}/{kind}"
            if not os.path.isdir(kdir):
                continue
            for case in sorted(os.listdir(kdir)):
                if not os.path.isdir(f"{kdir}/{case}"):
                    continue
                head = case[0]
                if head in NEW and case != NEW[head]:
                    rows.append((window, kind, case, NEW[head]))
    return rows


def referencing_files(window, kind, case):
    rel = window.split("/✳️any/")[1]
    root = subset_root(window)
    oracle = f"{window}/🔮️oracles/🔣️.json"
    hits = []
    if os.path.exists(oracle) and f"{kind}/{case}" in open(oracle, encoding="utf-8").read():
        hits.append((oracle, f"{kind}/{case}"))
    needle = f"{rel}/🧪️tests/🧫️fixtures/{kind}/{case}"
    out = subprocess.run(["rg", "-l", "--no-ignore", "-F", needle, root], capture_output=True, text=True).stdout.split("\n")
    for f in out:
        if f and f != oracle:
            hits.append((f, needle))
    for extra in EXTRA_FILES:
        if os.path.exists(extra):
            text = open(extra, encoding="utf-8").read()
            marker = f'"scope": "{window}",\n      "path": "🧪️tests/🧫️fixtures/{kind}/{case}"'
            if marker in text:
                hits.append((extra, f"🧪️tests/🧫️fixtures/{kind}/{case}\""))
    return hits


def main():
    rows = plan()
    ledger = []
    for window, kind, case, new in rows:
        old_dir = f"{window}/🧪️tests/🧫️fixtures/{kind}/{case}"
        new_dir = f"{window}/🧪️tests/🧫️fixtures/{kind}/{new}"
        if os.path.exists(new_dir):
            raise SystemExit(f"target exists: {new_dir}")
        edits = []
        for f, needle in referencing_files(window, kind, case):
            text = open(f, encoding="utf-8").read()
            tail = needle.rsplit("/", 1)[0]
            if needle.endswith('"'):
                replaced = text.replace(needle, f"🧪️tests/🧫️fixtures/{kind}/{new}\"")
                count = text.count(needle)
            else:
                pattern = re.compile(re.escape(needle) + r"(?![\w\-])")
                replaced, count = pattern.subn(tail + "/" + new, text)
            if count:
                edits.append((f, replaced, count))
        ledger.append({"dir": old_dir, "to": new_dir, "edits": [(f, c) for f, _, c in edits]})
        print(f"{kind}/{case} -> {new}: {len(edits)} file(s) {[c for _, _, c in edits]}"); [print("     ", f.split("subsets/")[-1][-100:]) for f, _, _ in edits]; [print("     ", f.split("subsets/")[-1][-130:]) for f, _, _ in edits]
        if WRITE:
            for f, replaced, _ in edits:
                backup = os.path.join(os.environ["BACKUPS"], f.replace("/", "%"))
                if not os.path.exists(backup):
                    shutil.copy2(f, backup)
                mode = os.stat(f).st_mode
                tmp = f + ".rename-tmp"
                open(tmp, "w", encoding="utf-8").write(replaced)
                os.chmod(tmp, mode)
                os.replace(tmp, f)
            os.rename(old_dir, new_dir)
    json.dump(ledger, open(os.environ.get("LEDGER", "/dev/null"), "w"), ensure_ascii=False, indent=1)


main()
