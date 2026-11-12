from body import *
from decisions import *
A = Asset("leaning-person")
SK = {"eye", "brow", "ear-1", "ear-2", "ear-3", "mouth-line", "cuff-l", "cuff-r", "trouser-hem-l", "trouser-hem-r", "sole-l", "sole-r", "sole-toe-l", "sole-toe-r", "hoodie-hem", "hoodie-pocket-1", "hoodie-pocket-2",
      "beanie-rim-1", "beanie-rim-2", "beanie-rim-3", "jeans-seam-l", "wrist-skin-r", "head", "thumb-l", "thumb-r", "fingers-l-0", "fingers-l-1", "fingers-l-2", "fingers-r-0", "fingers-r-1", "fingers-r-2", "torso"}
humanize(A, skip=SK, bones=set(b for b in A.bones if is_human_bone(b)))
hd = loft_from(A.draft["head"], rxfn=pfn([(0.0, 0.05), (0.07, 0.052), (0.1, 0.065), (0.19, 0.075), (0.27, 0.04)]), amax=1.6, exp=2.0)
A.add(hd, like="head")
A.add(loft_from(A.draft["torso"], rxfn=front_halfwidth(A, "torso", "torso"), amax=2.4, exp=2.5), like="torso")
for sd in "lr":
    thumb(A, f"thumb-{sd}", f"hand-{sd}", glove=f"hand-{sd}")
    refinger(A, [f"fingers-{sd}-{i}" for i in range(3)], f"hand-{sd}")
tj = get(A, "torso")
eyes(A, u=0.174, w=0.03, rx=0.006, ry=0.006, rz=0.008, material="skin", tone=0.1, region="head-skin")
for sd, z in (("l", 1), ("r", -1)):
    vs = surface_v(hd, 0.192, z * 0.03) + 0.002
    A.add(capsule(f"brow-{sd}", "head", [0.192, vs, z * 0.012], [0.191, vs + 0.003, z * 0.05], 0.004, material="skin", role="detail", tone=0.2, region="head-skin"), like="brow", maxden=50)
    A.add(ellipsoid(f"ear-{sd}", "head", [0.024, 0.011, 0.007], [0.154, 0.019, z * 0.077], material="skin", role="detail", tone=0.7, region="head-skin"), like="ear")
    A.add(capsule(f"hood-string-{sd}", "torso", [0.302, surface_v(tj, 0.302, z * 0.03) + 0.002, z * 0.03], [0.449, surface_v(tj, 0.44, z * 0.03) + 0.002, z * 0.03], 0.003, material="cotton-clothing", role="detail", tone=0.2, region="torso-clothing"), like="hood-string", maxden=50)
A.add(ellipsoid("nose", "head", [0.015, 0.009, 0.008], [0.165, surface_v(hd, 0.165, 0.0) + 0.003, 0.0], material="skin", role="detail", tone=0.8, region="head-skin"), like="nose", maxden=50)
vm = surface_v(hd, 0.11, 0.0) + 0.001
A.add(capsule("mouth", "head", [0.11, vm, -0.017], [0.11, vm, 0.017], 0.003, material="skin", role="detail", tone=0.3, region="head-skin"), like="mouth-line", maxden=50)
patch(A, "hoodie-pocket", "torso", 0.03, surface_v(tj, 0.03, 0.0), 0.0, [0.097, 0.012, 0.24], "hoodie-pocket", "cotton-clothing", 0.2, "torso-clothing")
band(A, "beanie-rim", "beanie", 0.195, 0.21, 0.003, "wool-hat", 0.2, "hair", like="beanie-rim")
deltoids(A, ["upper-arm-l", "upper-arm-r"])
DEC = BODY + [
    d("The wall of the 1:50 views is a ground-role context shape (1.2 m wide, 1.9 m high, 0.12 m thick) and is not part of the figure volume or of its dimensions; the leaning pose's contact with it comes from the poses, not from solids. The phone is a rounded box with a glazing-role screen on the right hand.",
      "Die Wand der 1:50-Ansichten ist eine Kontextform mit Rolle ground (1,2 m breit, 1,9 m hoch, 0,12 m dick) und gehört weder zum Figurenvolumen noch zu dessen Massen; der Kontakt der angelehnten Haltung zu ihr ergibt sich aus den Posen, nicht aus Körpern. Das Telefon ist ein abgerundeter Quader mit einem Bildschirm der Rolle glazing an der rechten Hand."),
    d("The hoodie pocket (drawn as a two-segment outline) is a flat box on the chest, the hood strings are tubes on the chest surface, the beanie rim a thin ring; the three ears the draft produced by a repeat step are replaced by the two real ears.",
      "Die als zweisegmentige Kontur gezeichnete Kapuzenpullover-Tasche ist ein flacher Quader auf der Brust, die Kapuzenbänder sind Röhren auf der Brustfläche, der Mützenrand ein dünner Ring; die drei Ohren, die der Entwurf durch einen Wiederholungsschritt erzeugte, sind durch die zwei echten Ohren ersetzt."),
]
A.write(DEC)
