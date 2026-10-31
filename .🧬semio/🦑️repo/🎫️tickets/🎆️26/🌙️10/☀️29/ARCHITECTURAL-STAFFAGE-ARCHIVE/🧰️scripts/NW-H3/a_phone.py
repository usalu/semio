"""standing-phone-user (NW-H3): rig, poses, clips."""
from h3lib import *
import sys
sex, MASS = "f", 59.4
rig, anchors, L = build_rig(sex, MASS, stature=1.69, sL=0.95, sF=0.88, sH=0.97, sA=0.95)
a = asset_stub(rig, anchors)
print(L)
