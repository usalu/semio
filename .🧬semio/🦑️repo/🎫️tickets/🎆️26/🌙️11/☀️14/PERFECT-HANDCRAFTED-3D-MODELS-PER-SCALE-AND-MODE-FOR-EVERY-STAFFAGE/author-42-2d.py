"""(second pass: the front feet strokes only; the first pass shortened elevation/front treads, platform, elevation feet) Repairs at the root in the stepladder asset (author 42): shorten the 1:500 pen strokes by the half pen at each end so that the widened strokes (rule volumes-views-disagree) end where the drawn part ends."""
import json, re
P = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🪑️objects/stepladder/🔣️.json"
HALF = 0.0325
L = open(P, encoding="utf-8").read().split("\n")
a = next(i for i, l in enumerate(L) if l == '      "500": {')
b = next(i for i in range(a + 1, len(L)) if re.match(r'      "\d+": \{|    \}', L[i]))
view = None
out = 0
i = a
while i < b:
    m = re.match(r'          "(elevation|front|plan)": \[', L[i])
    if m:
        view = m.group(1)
    m = re.match(r'              "id": "(foot-front-[lr])",', L[i])
    if m and view in ("elevation", "front"):
        s = i
        while 'points' not in L[s]:
            s += 1
        e = s
        while L[e] != "              ],":
            e += 1
        txt = "\n".join(L[s:e + 1])
        nums = [float(x) for x in re.findall(r"-?\d+\.?\d*(?:e-?\d+)?", txt.split(":", 1)[1])]
        if len(nums) == 4 and abs(nums[1] - nums[3]) < 1e-9:
            x0, y0, x1, y1 = nums
            lo, hi = sorted([x0, x1])
            if hi - lo > 2 * HALF:
                lo, hi = round(lo + HALF, 6), round(hi - HALF, 6)
                pts = [[lo, y0], [hi, y1]] if x0 <= x1 else [[hi, y0], [lo, y1]]
                L[s:e + 1] = ['              "points": ' + json.dumps(pts) + ","]
                b -= (e - s)
                out += 1
                print(view, m.group(1), pts)
    i += 1
t = "\n".join(L)
json.loads(t)
open(P, "w", encoding="utf-8").write(t)
print("changed", out)
