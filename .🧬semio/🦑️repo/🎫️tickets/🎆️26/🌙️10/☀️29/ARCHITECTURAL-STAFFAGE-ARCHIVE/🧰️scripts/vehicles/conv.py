"""Scratch: convert rest-pose world coordinates (m, y up) to the local frame of a bone.
usage: conv.py BONES.json  BONE  "M x,y L x,y C x,y x,y x,y Z"   (bones: {id:[parent,x,y,angleDeg]})"""
import json, math, re, sys
def world(bones, bid):
    p, x, y, a = bones[bid]
    if p is None: return (x, y, a)
    px, py, pa = world(bones, p)
    c, s = math.cos(math.radians(pa)), math.sin(math.radians(pa))
    return (px + c*x - s*y, py + s*x + c*y, pa + a)
def local(bones, bid, d):
    ox, oy, a = world(bones, bid)
    c, s = math.cos(math.radians(a)), math.sin(math.radians(a))
    out = []
    for tok in d.replace(",", " ").split():
        out.append(tok)
    res, nums = [], []
    def flush():
        for i in range(0, len(nums), 2):
            dx, dy = nums[i]-ox, nums[i+1]-oy
            res.append("%g,%g" % (round(c*dx + s*dy, 3), round(-s*dx + c*dy, 3)))
        nums.clear()
    for t in out:
        if t.isalpha(): flush(); res.append(t)
        else: nums.append(float(t))
    flush()
    return " ".join(res)
if __name__ == "__main__":
    bones = json.load(open(sys.argv[1]))
    print(world(bones, sys.argv[2]))
    for d in sys.argv[3:]: print(local(bones, sys.argv[2], d))
