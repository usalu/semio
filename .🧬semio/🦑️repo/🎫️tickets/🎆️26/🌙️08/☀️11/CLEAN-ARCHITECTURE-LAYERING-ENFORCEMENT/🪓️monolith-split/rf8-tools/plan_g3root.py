"""g3root: cut the viewer, editor structure, editor mode and editor command law wrappers of the generation3d test target into include fragments."""
import os, sys
sys.path.insert(0, os.path.dirname(__file__))
from fragsplit import *

EMOJIS = {l.split("\t")[0] for l in read(F + "/🗑️generated/rf8-folder-emojis.tsv").split("\n") if l}
C = lambda **k: k

SPEC = dict(
    target="g3root",
    path_dot_up=True,
    children=[
        C(folder="🔭️viewer-laws", ranges=[(72, 298)], doc="🔭️ Viewer laws: the preview-eval and viewer law wrappers, each mounting its unit test file."),
        C(folder="🪟️editor-mode-laws", ranges=[(299, 332), (936, 1017)], doc="🪟️ Editor mode laws: the panel and the generate and edit mode window law wrappers."),
        C(folder="🧭️editor-structure-laws", ranges=[(333, 935)], doc="🧭️ Editor structure laws: fold, interaction scope, generate interactions, example switch, domain, tick addressing, work capacity, mode panels and slider values."),
        C(folder="🎮️editor-command-laws", ranges=[(1018, 1366)], doc="🎮️ Editor command laws: one law wrapper per editor command and the selection laws."),
    ],
)

if __name__ == "__main__":
    for c in SPEC["children"]:
        assert c["folder"].split("️")[0] + "️" in EMOJIS, c["folder"]
    run(SPEC)
