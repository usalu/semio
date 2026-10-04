#!/usr/bin/env python3
"""🐍️ The `patch-snapshot` arm of the semio subsets' independent Python oracles (D4 of ticket
26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING): the verb joins the vocabulary tuple, `apply_mutation` answers the host's
`patched_snapshot` over the oracle's own snapshot reading, and `inverse_mutation` restores that reading through its own
`set-snapshot`. Edits anchor on each oracle's existing `set-snapshot` arms; idempotent; `--check` lists pending oracles.

@see ../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🖥️host/🐍️.py
"""
import pathlib
import re
import subprocess
import sys

ROOT = pathlib.Path(__file__).resolve().parents[7]
SUBSETS = "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/"
UNIFORM = ["🎞️animation", "🎬️video", "📐️cad", "📑️document", "🔊️audio"]
EXTERNAL = {"📊️table": True, "📦️object": True, "🔤️text": False, "🕸️graph": True, "🧊️brep": True, "🧰️kit": True}
IMPORT = "from semio_repo_test import Adapter, Context, Outcome, digest\n"


def oracle(subset: str, listed: list[str]) -> pathlib.Path:
    found = [name for name in listed if name.startswith(SUBSETS + subset + "/🧪️tests/") and "mutate" in name and name.endswith("/🐍️.py")]
    if len(found) != 1:
        raise SystemExit(f"{subset}: expected one mutate oracle, found {found}")
    return ROOT / found[0]


def armed(text: str) -> str:
    if '"patch-snapshot"' in text:
        return text
    if text.count(IMPORT) != 1 or text.count('\n    "set-snapshot",\n') != 1:
        raise SystemExit("an anchor (host import or vocabulary entry) is missing")
    text = text.replace(IMPORT, "from semio_repo_test import Adapter, Context, Outcome, digest, patched_snapshot\n")
    text = text.replace('\n    "set-snapshot",\n', '\n    "set-snapshot",\n    "patch-snapshot",\n')
    arms = [match for match in re.finditer(r'\n    if kind == "set-snapshot":\n        (return [^\n]+)\n', text)]
    if len(arms) != 2:
        raise SystemExit(f"expected the apply and inverse set-snapshot arms, found {len(arms)}")
    inverse = arms[1]
    restore = inverse.group(1).replace("[kind]", '["set-snapshot"]')
    text = text[: inverse.start()] + f'\n    if kind == "patch-snapshot":\n        {restore}' + text[inverse.start() :]
    apply = arms[0]
    return text[: apply.start()] + '\n    if kind == "patch-snapshot":\n        return patched_snapshot(snapshot, args["patch"])' + text[apply.start() :]


def armed_external(text: str, returns_list: bool) -> str:
    """🏷️ An externally tagged vocabulary (`{"Verb": {…}}`) without a whole-snapshot verb: the inverse of a patch is the
    host's exact inverse patch. Both vocabulary tables are extended where an oracle keeps both."""
    if '"patch-snapshot"' in text and ('\nTAG_OF_KIND = {' not in text or '"patch-snapshot": "PatchSnapshot"' in text):
        return text
    if '"patch-snapshot"' in text:
        tags = re.search(r'\nTAG_OF_KIND = \{\n(?:    "[^"\n]+": "[^"\n]+",\n)+(\})\n', text)
        return text[: tags.start(1)] + '    "patch-snapshot": "PatchSnapshot",\n' + text[tags.start(1) :]
    if text.count(IMPORT) != 1:
        raise SystemExit("the host import anchor is missing")
    text = text.replace(IMPORT, "from semio_repo_test import Adapter, Context, Outcome, digest, patched_snapshot, snapshot_patch_inverse\n")
    kinds = re.search(r'\nKINDS = \(([^)]*?)(,?\s*)\)\n', text)
    tags = re.search(r'\nTAG_OF_KIND = \{\n(?:    "[^"\n]+": "[^"\n]+",\n)+(\})\n', text)
    if kinds is None and tags is None:
        raise SystemExit("the vocabulary anchor (KINDS tuple or TAG_OF_KIND table) is missing")
    if tags is not None:
        text = text[: tags.start(1)] + '    "patch-snapshot": "PatchSnapshot",\n' + text[tags.start(1) :]
    if kinds is not None:
        kinds = re.search(r'\nKINDS = \(([^)]*?)(,?\s*)\)\n', text)
        text = text[: kinds.end(1)] + ', "patch-snapshot"' + text[kinds.end(1) :]
    split = "    tag, args = tagged(mutation)\n"
    apply_at = text.index(split, text.index("\ndef apply_mutation("))
    undo = '[{"PatchSnapshot": {"patch": snapshot_patch_inverse(document, args["patch"])}}]' if returns_list else '{"PatchSnapshot": {"patch": snapshot_patch_inverse(document, args["patch"])}}'
    inverse_at = text.index(split, text.index("\ndef inverse_mutation("))
    text = text[: inverse_at + len(split)] + f'    if tag == "PatchSnapshot":\n        return {undo}\n' + text[inverse_at + len(split) :]
    return text[: apply_at + len(split)] + '    if tag == "PatchSnapshot":\n        return patched_snapshot(document, args["patch"])\n' + text[apply_at + len(split) :]


def main() -> int:
    check = "--check" in sys.argv
    listed = subprocess.run(["git", "ls-files", "-z", "--", SUBSETS], cwd=ROOT, capture_output=True, check=True).stdout.decode().split("\0")
    pending = 0
    for subset in UNIFORM:
        path = oracle(subset, listed)
        text = path.read_text(encoding="utf-8")
        next_text = armed(text)
        if next_text != text:
            pending += 1
            if not check:
                path.write_text(next_text, encoding="utf-8")
            print(f"{'pending' if check else 'armed'}: {subset}")
    for subset, returns_list in EXTERNAL.items():
        path = oracle(subset, listed)
        text = path.read_text(encoding="utf-8")
        next_text = armed_external(text, returns_list)
        if next_text != text:
            pending += 1
            if not check:
                path.write_text(next_text, encoding="utf-8")
            print(f"{'pending' if check else 'armed'}: {subset}")
    print(f"{pending} oracle(s) {'pending' if check else 'armed'} of {len(UNIFORM) + len(EXTERNAL)}")
    return 1 if check and pending else 0


if __name__ == "__main__":
    sys.exit(main())
