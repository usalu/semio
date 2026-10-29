import importlib.util
spec = importlib.util.spec_from_file_location("lib", "/home/deck/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️29/ARCHITECTURAL-STAFFAGE-ARCHIVE/🧰️scripts/refine-ha-animals-lib.py")
lib = importlib.util.module_from_spec(spec); spec.loader.exec_module(lib)
D = lib.Doc(); A = "domestic-cat"
def sit(o):
    for k, v in {"tail-1": 80, "tail-2": 42, "tail-3": -2, "tail-4": 0, "tail-5": 35}.items(): o["bones"][k] = {"angleDeg": v}
    o["description"] = {"en": "Sitting upright on the sitting bones with the hind feet flat in front of the haunches, straight forelegs, head level, and the tail laid along the near flank and the ground, wrapping forward around the hind and fore paws with its tip curled up.", "de": "Aufrechtes Sitzen auf den Sitzbeinhöckern, die Hinterfüße flach vor der Hinterhand, gerade Vorderbeine, Kopf waagerecht und der Schwanz an der nahen Flanke und am Boden entlang nach vorn um Hinter- und Vorderpfoten gelegt, die Spitze aufgebogen."}
    return o
print(D.edit(A, "poses", lambda o: o["id"] == "sitting-tail-wrapped", sit)); D.save()
D = lib.Doc()
print(D.edit(A, "anchors", lambda o: o["id"] in ("contact-hn", "contact-hf", "contact-fn", "contact-ff"), lambda o: {**o, "y": -0.0145})); D.save()
