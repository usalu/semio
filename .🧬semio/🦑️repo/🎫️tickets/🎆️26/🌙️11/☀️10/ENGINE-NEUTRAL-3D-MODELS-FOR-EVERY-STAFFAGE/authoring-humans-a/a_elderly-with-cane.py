from body import *
from decisions import *
A = Asset("elderly-with-cane")
SK = {"fingers-l-0", "fingers-l-1", "fingers-l-2", "fingers-r-0", "fingers-r-1", "fingers-r-2", "thumb-l", "thumb-r", "shoe-sole-l", "shoe-sole-r", "jacket-pocket", "jacket-button", "eye", "glasses-1", "glasses-2",
      "jacket", "jacket-hump", "cane-shaft", "cane-crook", "cane-ferrule"}
humanize(A, skip=SK, bones=set(b for b in A.bones if is_human_bone(b)))
A.add(loft_from(A.draft["jacket"], rxfn=front_halfwidth(A, "torso", "torso"), amax=2.6, exp=2.5), like="jacket")
A.add(loft_from(A.draft["jacket-hump"], lat=(-0.12, 0.12), exp=2.4), like="jacket-hump")
for sd in "lr":
    thumb(A, f"thumb-{sd}", f"hand-{sd}", glove=f"hand-{sd}")
    refinger(A, [f"fingers-{sd}-{i}" for i in range(3)], f"hand-{sd}")
jk = get(A, "jacket")
head = get(A, "head")
eyes(A, u=0.05, w=0.03, rx=0.008, ry=0.005, rz=0.009, material="skin", tone=0.1)
for i, u in enumerate((-0.045, 0.043, 0.132)):
    A.add(ellipsoid(f"jacket-button-{i}", "torso", [0.008, 0.005, 0.008], [u, surface_v(jk, u, 0.012) + 0.001, 0.012], material="tweed", role="detail", tone=0.1, region="jacket"), like="jacket-button", maxden=50)
for sd, z in (("l", 1), ("r", -1)):
    patch(A, f"jacket-pocket-{sd}", "torso", -0.07, surface_v(jk, -0.07, z * 0.115), z * 0.115, [0.05, 0.012, 0.07], "jacket-pocket", "tweed", 0.38, "jacket")
    A.add(ellipsoid(f"glasses-lens-{sd}", "head", [0.016, 0.003, 0.022], [0.064, surface_v(head, 0.064, z * 0.03) + 0.0005, z * 0.03], material="skin", role="detail", tone=0.1, region="face-skin"), like="glasses-1", maxden=50)
    A.add(capsule(f"glasses-temple-{sd}", "head", [0.065, surface_v(head, 0.065, z * 0.052), z * 0.052], [0.066, 0.013, z * 0.075], 0.003, material="skin", role="detail", tone=0.1, region="face-skin"), like="glasses-2", maxden=50)
A.add(capsule("glasses-bridge", "head", [0.064, surface_v(head, 0.064, 0.0), -0.01], [0.064, surface_v(head, 0.064, 0.0), 0.01], 0.003, material="skin", role="detail", tone=0.1, region="face-skin"), like="glasses-1", maxden=50)
A.add(ellipsoid("cap-brim", "head", [0.006, 0.022, 0.05], [0.103, -0.095, 0.0], material="tweed", role="body", tone=0.32, region="cap"), like="cap-brim")
deltoids(A, ["sleeve-l", "sleeve-r"])
A.add(capsule("cane-shaft", "cane", [0.05, 0.0, -0.04], [0.84, 0.0, -0.04], 0.0115, 0.0095, material="ash-wood", role="body", tone=0.45, region="cane-wood"), like="cane-shaft")
tube_path(A, "cane-crook", "cane", [(-0.03, 0.0), (-0.052, 0.008), (-0.062, 0.03), (-0.058, 0.055), (-0.045, 0.068), (-0.032, 0.064)], 0.0125, "ash-wood", 0.4, "cane-wood", w=-0.04, like="cane-crook")
A.add(capsule("cane-ferrule", "cane", [0.82, 0.0, -0.04], [0.858, 0.0, -0.04], 0.013, 0.012, material="rubber", role="body", tone=0.05, region="cane-wood"), like="cane-ferrule")
DEC = BODY + [
    d("The cane is a round ash shaft on its own bone with a rubber ferrule and a crook of five tube segments following the centre line of the drawn hook; its lateral position (0.225 m right of the body axis) comes from the front and plan views.",
      "Der Stock ist ein runder Eschenschaft auf eigenem Knochen mit Gummizwinge und einer Krücke aus fünf Rohrsegmenten entlang der Mittellinie des gezeichneten Hakens; seine seitliche Lage (0,225 m rechts der Körperachse) stammt aus Vorder- und Draufsicht."),
    d("The jacket is wider than the plan trunk circle: its half width follows the front outline (about 0.19 m), the kyphotic back hump is a separate loft over the upper back, buttons and pocket flaps lie on the front surface, and the glasses are two flat lens ellipsoids, a bridge and two temple arms placed on the head surface.",
      "Die Jacke ist breiter als der Rumpfkreis der Draufsicht: ihre Halbbreite folgt dem Umriss der Vorderansicht (etwa 0,19 m), der kyphotische Rückenbuckel ist ein eigener Loft über dem oberen Rücken, Knöpfe und Taschenklappen liegen auf der Vorderfläche, und die Brille besteht aus zwei flachen Linsen-Ellipsoiden, einem Steg und zwei Bügeln auf der Kopfoberfläche."),
]
A.write(DEC)
