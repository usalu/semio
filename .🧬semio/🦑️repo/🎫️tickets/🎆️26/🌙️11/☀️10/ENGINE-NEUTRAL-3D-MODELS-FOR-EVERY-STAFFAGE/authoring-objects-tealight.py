import importlib.util, math, os
spec = importlib.util.spec_from_file_location("lib", os.path.join(os.path.dirname(os.path.abspath(__file__)), "authoring-objects-lib.py"))
lib = importlib.util.module_from_spec(spec); spec.loader.exec_module(lib)

A = lib.Asset("tealight", "aluminium-cup", 0.5, "cup")
A.rev("cup-floor", "cup", [0, 0, 0], [[0, 0], [0, 0.0155], [0.0005, 0.0155], [0.0005, 0]])
A.ring("cup-wall", "cup", [0, 0], 0.0005, 0.016, 0.0155 + 0.0035 * 0.0005 / 0.016, 0.019, 0.0004, n=48)
A.rev("wax", "wax", [0, 0, 0], [[0.001, 0], [0.001, 0.0152], [0.0138, 0.0178], [0.0144, 0]], material="paraffin", tone=0.85, region="wax")
A.cap("wick", "wick", [0, 0.0145, 0], [0, 0.0198, 0], 0.0007, material="wick-cotton", tone=0.1, role="detail", region="wick")
body = [[y, r] for r, y in lib.bez([0, 0], [0.0058, 0.003], [0.0058, 0.009], [0.0016, 0.0125], 8)] + [[y, r] for r, y in lib.bez([0.0016, 0.0125], [0.0008, 0.0133], [0.0003, 0.0137], [0, 0.014], 4)][1:]
A.rev("flame", "flame-body", [0, 0.0185, 0], body, material="flame", tone=0.9, region="flame")
tongue = [[y, r] for r, y in lib.bez([0, 0], [0.0016, 0.002], [0.0019, 0.007], [0.0007, 0.012], 8)] + [[y, r] for r, y in lib.bez([0.0007, 0.012], [0.0004, 0.0133], [0.0002, 0.0138], [0, 0.014], 4)][1:]
A.rev("flame-tongue", "flame-tip", [0, 0.0265, 0], tongue, material="flame", tone=0.97, region="flame")
A.save([
    ("The cup is hollow: a 0.5 mm floor disc and a 0.4 mm tapered wall ring (0.031 m at the floor to 0.038 m at the rim); the wax is a revolve filling the cup up to 13.8 mm with a slightly raised centre, the wick a thin capsule, the flame and its tongue two teardrop revolves following the drawn outlines. The dimensions describe the lit pose (flame tip at 40.5 mm); in the unlit pose the flame bones sink into the wick.",
     "Das Schälchen ist hohl: eine 0,5 mm Bodenscheibe und ein 0,4 mm starker konischer Wandring (0,031 m am Boden bis 0,038 m am Rand); das Wachs ist ein Revolve, der das Schälchen bis 13,8 mm füllt, mit leicht erhöhter Mitte, der Docht eine dünne Kapsel, Flamme und Zunge zwei tropfenförmige Revolves nach den gezeichneten Umrissen. Die Masse beschreiben die brennende Pose (Flammenspitze bei 40,5 mm); in der unbrennenden Pose sinken die Flammenknochen in den Docht."),
])
