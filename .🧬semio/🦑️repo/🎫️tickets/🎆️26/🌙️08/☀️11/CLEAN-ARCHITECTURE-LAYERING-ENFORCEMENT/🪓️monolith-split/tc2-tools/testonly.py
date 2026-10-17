import sys,glob,os,re
base="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/🗑️generated/tc2-baseline"
ROOT="/Users/ueli/Documents/semio/"
CRATES="draw-drawing-scene draw-drawing-transform draw-drawing-editor draw-drawing-style draw-drawing-editor-commands draw-drawing-write draw-drawing-editor-interaction draw-drawing-model draw-drawing-host draw-drawing-structure draw-drawing-geometry layout-layout-mutations-frames layout-layout-mutations-content layout-layout-editor-commands block-2d-editor block-5d-editor".split()
def load(f): return [l.rstrip("\n").split("\t") for l in open(f) if l.strip()]
def testonly(short):
    t=load(glob.glob(f"{base}/tc2-inputs-semio-s-artifact-{short}-lib-test-*.txt")[0])
    n={r[3] for r in load(f"{base}/tc2-inputs-{short}-normal.txt")}
    return [r for r in t if r[3] not in n]
if __name__=="__main__":
    short=sys.argv[1]
    for r in testonly(short):
        p=r[3].replace(ROOT,"")
        print(r[0][:3],r[1],r[2],p.split("subsets/")[-1] if "subsets/" in p else p)
