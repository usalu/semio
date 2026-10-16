"""🖥️ Slice D follow-up, os React board host: the generic Board2dHost helpers lose their puzzle 2d names (exports, callers, docs)."""
import re
from importlib import util

spec = util.spec_from_file_location("apply_board", __file__.rsplit("/", 1)[0] + "/apply-board.py")
board = util.module_from_spec(spec)
spec.loader.exec_module(board)
R = board.R
E = R + "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/"

RENAMES = {
    "Puzzle2dDropPayload": "BoardDropPayload",
    "Puzzle2dLiveMirrorMutations": "BoardLiveMirrorMutations",
    "beginPuzzle2dPeerGesture": "beginBoardPeerGesture",
    "collectPuzzle2dLiveMirrorMutations": "collectBoardLiveMirrorMutations",
    "endPuzzle2dPeerGesture": "endBoardPeerGesture",
    "notifyPuzzle2dPeersGestureEnded": "notifyBoardPeersGestureEnded",
    "parsePuzzle2dCatalogueDragPayload": "parseBoardCatalogueDragPayload",
    "pushPuzzle2dDropPreview": "pushBoardDropPreview",
    "pushPuzzle2dLiveMirrorMutations": "pushBoardLiveMirrorMutations",
    "puzzle2dDropPreviewJson": "boardDropPreviewJson",
    "puzzle2dEntityFlag": "boardEntityFlag",
    "puzzle2dPeerOwnsGesture": "boardPeerOwnsGesture",
    "puzzle2dScreenToWorld": "boardScreenToWorld",
    "puzzle2dWorldToScreen": "boardWorldToScreen",
}
TOKEN = re.compile(r"\b(" + "|".join(sorted(RENAMES, key=len, reverse=True)) + r")\b")
FILES = [
    E + "🧱️elements/🖥️Board2dHost/🟦️.tsx",
    E + "🎯️targets/⚛️react/🟦️.tsx",
    E + "🧪️tests/🔬️engine-contract/🟦️.ts",
    E + "🧪️tests/👕️canvas-presence/🟦️.ts",
    E + "🧱️elements/🖥️Board2dHost/🧪️tests/👻️catalogue-drop/🟦️.tsx",
    R + "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🧪️tests/🪪️session-factory/🟦️.ts",
    R + "🧰️framework/🔨️modules/🕹️interaction/👆️gesture/🟦️.ts",
]
EDITS = {
    E + "🧱️elements/🖥️Board2dHost/🟦️.tsx": [
        (" * `<sectionId>.<kindId>` (puzzle 2d: `puzzle2d-play-kinds.nodes.beam`), so the trailing dot segment — or\n * the whole id, for a bare-kind row like puzzle 3d's — is the candidate,",
         " * `<sectionId>.<kindId>` (for example `catalog-kinds.nodes.beam`), so the trailing dot segment — or\n * the whole id, for a bare-kind row — is the candidate,"),
        (" * pick or hover reports it under — the client twin of the guest's `puzzle2d_selection_targets`. A",
         " * pick or hover reports it under — the client twin of the owning guest's selection-target classifier. A"),
    ],
}


def apply_renames():
    for path in FILES:
        text = open(path, encoding="utf-8").read()
        new, count = TOKEN.subn(lambda m: RENAMES[m.group(1)], text)
        assert count > 0, path
        open(path, "w", encoding="utf-8").write(new)
        print(f"renamed {count:3} in", path.replace(R, ""))


if __name__ == "__main__":
    board.apply(EDITS)
    apply_renames()
