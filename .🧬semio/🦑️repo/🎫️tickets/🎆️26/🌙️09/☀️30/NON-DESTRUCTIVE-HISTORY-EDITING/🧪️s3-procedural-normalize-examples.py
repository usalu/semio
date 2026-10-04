"""🧾️ One-time normalization of the procedural example assets to the materialized-defaults form (design §20.9).

Reads the captured output of the two temporary `[DEBUG]` printer tests (DEV `debug_print_normalized_examples` for
generation3d, `debug_print_normalized_example` for generation2d), where the codec's own printer emitted every example with
its operators' declared defaults recorded, and rewrites the bundled `🗣️.dsl.semio` assets from it.

A bare run is a DRY RUN that prints a unified diff per asset; `--write` writes the differing assets. An asset is written
only if every line the printer removed was an empty-params line (`params=[ ]` or an empty `params [key:TEXT value:BLOCK]`
table), so the rewrite can never drop authored content. Unknown arguments exit non-zero.

Usage: python3 🧪️s3-procedural-normalize-examples.py <captured-output.txt>... [--write]
"""

import difflib
import pathlib
import re
import sys

REPO = pathlib.Path(__file__).resolve().parents[7]
GEN3D = REPO / "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples"
GEN2D = REPO / "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples"
ASSETS = {
    "hexagonal-mushroom-column": GEN3D / "🍄️hexagonal-mushroom-column/🖼️assets/🍄️hexagonal-mushroom/🗣️.dsl.semio",
    "rectangle-extrude-volume": GEN3D / "📦️rectangle-extrude-volume/🖼️assets/📦️rectangle-extrude-volume/🗣️.dsl.semio",
    "sphere-cut-with-torus": GEN3D / "🍩️sphere-cut-with-torus/🖼️assets/🍩️sphere-cut-with-torus/🗣️.dsl.semio",
    "box-fillet-preview": GEN3D / "📐️box-fillet-preview/🖼️assets/📐️box-fillet-preview/🗣️.dsl.semio",
    "sphere-box-fuse": GEN3D / "🧲️sphere-box-fuse/🖼️assets/🧲️sphere-box-fuse/🗣️.dsl.semio",
    "face-sweep-extrude": GEN3D / "🧹️face-sweep-extrude/🖼️assets/🧹️face-sweep-extrude/🗣️.dsl.semio",
    "rectangle-wire-preview": GEN3D / "🪢️rectangle-wire-preview/🖼️assets/🪢️rectangle-wire-preview/🗣️.dsl.semio",
    "box-shell-preview": GEN3D / "🐚️box-shell-preview/🖼️assets/🐚️box-shell-preview/🗣️.dsl.semio",
    "mesh-workbench": GEN3D / "🥽️mesh-workbench/🖼️assets/🥽️mesh-workbench/🗣️.dsl.semio",
    "demo": GEN2D / "🎬️demo/🖼️assets/🗣️.dsl.semio",
}
BLOCK = re.compile(r"\[DEBUG\] BEGIN (?P<id>[a-z0-9-]+)\n(?P<text>.*?)\n\[DEBUG\] END (?P=id)", re.S)
EMPTY_PARAMS = re.compile(r"params=\[ \]|^\s*params \[key:TEXT value:BLOCK\] \{$|^\s*\}$")


def printed(captures):
    blocks = {}
    for capture in captures:
        for match in BLOCK.finditer(pathlib.Path(capture).read_text(encoding="utf-8")):
            blocks[match["id"]] = match["text"].rstrip("\n") + "\n"
    return blocks


def only_empty_params_removed(before, after):
    removed = [line[1:] for line in difflib.unified_diff(before.splitlines(), after.splitlines(), lineterm="", n=0) if line.startswith("-") and not line.startswith("---")]
    return all(EMPTY_PARAMS.search(line) or line.strip().startswith("neuron ") for line in removed)


def main(arguments):
    write = "--write" in arguments
    captures = [argument for argument in arguments if argument != "--write"]
    unknown = [argument for argument in captures if argument.startswith("--")]
    if unknown or not captures:
        print(f"usage: {pathlib.Path(__file__).name} <captured-output.txt>... [--write]; unknown: {unknown}", file=sys.stderr)
        return 1
    blocks = printed(captures)
    missing = sorted(set(ASSETS) - set(blocks))
    refused, changed = [], []
    for example_id, text in sorted(blocks.items()):
        path = ASSETS[example_id]
        before = path.read_text(encoding="utf-8")
        if before == text:
            continue
        sys.stdout.writelines(difflib.unified_diff(before.splitlines(True), text.splitlines(True), f"a/{example_id}", f"b/{example_id}"))
        if not only_empty_params_removed(before, text):
            refused.append(example_id)
            continue
        changed.append(example_id)
        if write:
            path.write_text(text, encoding="utf-8")
    print(f"{'wrote' if write else 'would write'} {len(changed)}: {changed}; refused (authored content would change) {len(refused)}: {refused}; not printed: {missing}")
    return 1 if refused else 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
