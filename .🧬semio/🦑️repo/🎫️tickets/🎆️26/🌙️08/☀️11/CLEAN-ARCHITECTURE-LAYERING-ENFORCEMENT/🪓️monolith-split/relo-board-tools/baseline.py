"""🧯️ Rebuilds the pre-slice-D content of every file slice D touched, inside a sandbox root: first-pass pristine copies,
the scene files saved before the follow-up, and the exact inverse of the follow-up host edits for the rest."""
import os
import re
import shutil
from importlib import util

TOOLS = os.path.dirname(os.path.abspath(__file__))
GEN = os.path.join(os.path.dirname(TOOLS), "🗑️generated")


def _load(name):
    spec = util.spec_from_file_location(name.replace("-", "_"), os.path.join(TOOLS, name + ".py"))
    module = util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def _invert(text, pairs):
    for old, new in reversed(pairs):
        assert text.count(new) == 1, new[:80]
        text = text.replace(new, old)
    return text


def build(root):
    """🧯️ Writes the pre-slice-D version of every touched file under `root`; returns the restored relative paths."""
    hosts = _load("apply-wire-hosts")
    R = hosts.R
    first = os.path.join(GEN, "relo-board-baseline")
    restored = {}
    for name in os.listdir(first):
        restored[name.replace("__", "/")] = open(os.path.join(first, name), encoding="utf-8").read()
    second = os.path.join(GEN, "relo-board-baseline2")
    scene_prefix = "🧰️framework/🔨️modules/🖱️ui/🎬️scene/"
    for name in os.listdir(second):
        if name.startswith("S__"):
            restored[scene_prefix + name[3:].replace("__", "/")] = open(os.path.join(second, name), encoding="utf-8").read()
    reverse_family = re.compile(r"\bboard2d_(" + hosts.FAMILY.pattern.split("(", 1)[1].split(")\\b")[0] + r")\b")
    touched = set(hosts.EDITS) | set(hosts.FAMILY_FILES) | set(hosts.EVENT_SET_FILES) | {hosts.W + "🧪️tests/🔬️wgpu-board2d-engine/🦀️.rs", hosts.P + "🧪️tests/🔬️unit/🦀️.rs", hosts.P5 + "✏️editor/🎭️modes/✏️edit/🪟️windows/◻️2d/🧪️tests/🔬️unit/🦀️.rs"}
    before_ts = {}
    tsv = os.path.join(second, "renderer-ts-files.tsv")
    if os.path.exists(tsv):
        for line in open(tsv, encoding="utf-8").read().splitlines():
            name, rel = line.split("\t")
            before_ts[rel] = open(os.path.join(second, name), encoding="utf-8").read()
    for rel, text in before_ts.items():
        if R + rel not in touched and rel not in restored:
            restored[rel] = text
    for path in sorted(touched):
        rel = path.replace(R, "")
        if rel in restored:
            continue
        text = before_ts.get(rel) or open(path, encoding="utf-8").read()
        if path == hosts.W + "🧪️tests/🔬️wgpu-board2d-engine/🦀️.rs":
            block = hosts.WGPU_NAMING_TEST.lstrip("\n") + "\n"
            assert text.count(block) == 1
            text = text.replace(block, "")
        if path == hosts.P + "🧪️tests/🔬️unit/🦀️.rs":
            block = hosts.TEST_2D.lstrip("\n") + "\n"
            assert text.count(block) == 1
            text = text.replace(block, "")
        if path == hosts.P5 + "✏️editor/🎭️modes/✏️edit/🪟️windows/◻️2d/🧪️tests/🔬️unit/🦀️.rs":
            block = "\n" + hosts.TEST_5D
            assert text.endswith(block)
            text = text[: -len(block)] + "\n"
        if path in hosts.EVENT_SET_FILES:
            text = text.replace("BOARD_TRANSIENT_EVENT_NAMES", "PUZZLE2D_TRANSIENT_EVENT_NAMES").replace("BOARD_FLUSH_NOW_EVENT_NAMES", "PUZZLE2D_FLUSH_NOW_EVENT_NAMES")
        if path in hosts.FAMILY_FILES:
            text = reverse_family.sub(lambda m: "puzzle_board_" + m.group(1), text)
        if path in hosts.EDITS:
            text = _invert(text, hosts.EDITS[path])
        restored[rel] = text
    for rel, text in restored.items():
        target = os.path.join(root, rel)
        if os.path.isdir(os.path.dirname(target)):
            open(target, "w", encoding="utf-8").write(text)
    return sorted(restored)


if __name__ == "__main__":
    import sys
    paths = build(sys.argv[1])
    print(len(paths), "files restored")
