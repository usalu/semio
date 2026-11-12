from body import *
from decisions import *
A = Asset("jogger")
FOLDS = {"torso-waistfold-0", "torso-waistfold-1-1", "torso-waistfold-1-2", "torso-chestfold", "sleeve-l-shoulder", "sleeve-r-shoulder", "armband-shoulder", "thigh-l-crease-0", "thigh-l-crease-1", "thigh-r-crease-0", "thigh-r-crease-1",
         "thigh-l-hipfold-1", "thigh-l-hipfold-2", "thigh-r-hipfold-1", "thigh-r-hipfold-2", "nostril-1", "nostril-2", "jaw-line-1", "jaw-line-2", "lower-lip", "sole-l", "sole-r", "sole-toe-l", "sole-toe-r"}
SK = FOLDS | {"eye", "brow", "headband-1", "headband-2", "headband-3", "shirt-hem", "shorts-stripe-r", "shorts-hem-r", "shorts-hem-l", "head", "thumb-l", "thumb-r"}
humanize(A, skip=SK, bones=set(b for b in A.bones if is_human_bone(b)))
hd = loft_from(A.draft["head"], rxfn=pfn([(0.0, 0.05), (0.07, 0.052), (0.105, 0.068), (0.14, 0.077), (0.28, 0.077)]), amax=1.4, exp=2.0)
A.add(hd, like="head")
for sd in "lr":
    thumb(A, f"thumb-{sd}", f"hand-{sd}", glove=f"hand-{sd}")
eyes(A, u=0.185, w=0.03, rx=0.006, ry=0.007, rz=0.009, material="skin", tone=0.1, region="head-skin")
for sd, z in (("l", 1), ("r", -1)):
    vs = surface_v(hd, 0.203, z * 0.03) + 0.002
    A.add(capsule(f"brow-{sd}", "head", [0.203, vs, z * 0.012], [0.2, vs + 0.003, z * 0.05], 0.004, material="skin", role="detail", tone=0.2, region="head-skin"), like="brow", maxden=50)
    A.add(capsule(f"shorts-stripe-{sd}", f"thigh-{sd}", [0.0, 0.0, z * 0.16], [0.197, 0.0, z * 0.16], 0.008, material="cotton-clothing", role="detail", tone=0.75, region="leg-clothing"), like="shorts-stripe", maxden=50)
    band(A, f"shorts-hem-{sd}", f"thigh-{sd}", 0.204, 0.213, 0.003, "cotton-clothing", 0.3, "leg-clothing", like="shorts-hem")
band(A, "headband", "head", 0.232, 0.262, 0.003, "hair", 0.9, "hair", like="headband", maxden=100)
band(A, "shirt-hem", "torso", -0.063, -0.051, 0.003, "cotton-clothing", 0.3, "torso-clothing", like="shirt-hem")
deltoids(A, ["sleeve-l", "sleeve-r"])
DEC = BODY + [
    d("The jogger's torso is a loft with the elevation depth and the front/plan width (0.17 m half width, flatter than deep); the head loft narrows to the 0.052 m neck in its lower part (the head bone carries the neck), the shorts are a loft over a skin thigh, the socks and sleeves lofts over the skin limbs.",
      "Der Rumpf der Läuferin ist ein Loft mit der Tiefe der Seitenansicht und der Breite aus Vorder- und Draufsicht (0,17 m Halbbreite, flacher als tief); der Kopf-Loft verjüngt sich im unteren Teil auf den Hals von 0,052 m (der Kopfknochen trägt den Hals), die Shorts sind ein Loft über einem Oberschenkel aus Haut, Socken und Ärmel Lofts über den Hautgliedern."),
    d("Cloth folds, creases, nostrils, jaw and lip lines and sole strokes are drawing aids of the 1:50 views and are not modelled as geometry; hems, the headband and the shirt hem are thin rings slightly larger than the garment, the shorts stripes are tubes on both outer thighs, the shoe slices drawn in the plan are covered by the shoe lofts.",
      "Stofffalten, Knitter, Nasenlöcher, Kiefer- und Lippenlinien sowie Sohlenstriche sind Zeichenhilfen der 1:50-Ansichten und nicht als Geometrie modelliert; Säume, das Stirnband und der Hemdsaum sind dünne Ringe etwas grösser als das Kleidungsstück, die Shorts-Streifen sind Röhren an beiden Aussenseiten der Oberschenkel, die in der Draufsicht gezeichneten Schuhscheiben sind durch die Schuh-Lofts abgedeckt."),
]
A.write(DEC)
