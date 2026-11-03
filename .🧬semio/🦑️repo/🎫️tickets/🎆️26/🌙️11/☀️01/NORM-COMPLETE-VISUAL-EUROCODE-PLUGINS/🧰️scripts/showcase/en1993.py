"""Generates the EN 1993 showcase document `🧪️tests/🧫️fixtures/🎪️showcase/🔣️.json` (python3 -I en1993.py).

The compliant HEB 240 frame of the model, the populated row of every special part (SPEC `show` values of `../en1993-special-spec.py`), the end-plate and base-plate joints, the stainless member and the pipeline, the annex rows (ring-stiffened cylinder, corrugated web, orthotropic deck, restrained purlin, stiffened panel), the frame portal and the load/joint variants of the spec-trace tests (tension, torsion, parabolic moment diagram, category B bolts, fillet-welded joint). Every collection is populated, every entity is typed by the artifact schema."""
import copy, pathlib, re, sys
sys.path.insert(0, str(pathlib.Path(__file__).resolve().parent))
from common import artifact, clone, finish, load

ART = artifact("🔩️en1993")
SCRIPTS = pathlib.Path(__file__).resolve().parents[1]
exec(compile((SCRIPTS / "en1993-special-spec.py").read_text(encoding="utf8"), "spec", "exec"))
camel = lambda s: re.sub(r"_([a-z0-9])", lambda m: m.group(1).upper(), s)
BASE = load(ART / "🧪️tests/🧫️fixtures/🧬️mutations/🕸️change-frame-node-support/✅apply/🔣️.before.json")
doc = copy.deepcopy(BASE)
doc["annex"] = "De"
doc["reliabilityClass"] = "RC2"
by_id = lambda coll, rid: next(r for r in doc[coll] if r["id"] == rid)


def show(row, ent, only=None, skip=(), override=None):
    for fd in SPEC[ent]["fields"]:
        if (only is not None and fd["name"] not in only) or fd["name"] in skip:
            continue
        v = (override or {}).get(fd["name"], fd["show"])
        row[camel(fd["name"])] = v if fd["ty"] == "String" else (int(v) if fd["ty"] == "u32" else float(v))
    return row


doc["materials"] = [{"id": "mat-s355", "grade": "S355", "fy": 355.0e6, "fu": 510.0e6, "eModulus": 210.0e9, "gModulus": 81.0e9, "subgrade": "J2", "kind": "Carbon", "rambergOsgoodN": 0.0}]
doc["sections"] = [{"id": "sec-heb240", "designation": "HEB 240", "kind": "RolledI", "h": 0.240, "b": 0.240, "tw": 0.010, "tf": 0.017, "r": 0.021, "area": 0.0106, "shearAreaY": 0.00512, "shearAreaZ": 0.00240, "iy": 1.126e-4, "iz": 3.923e-5, "it": 1.037e-6, "iw": 4.762e-7, "wElY": 9.38e-4, "wElZ": 3.27e-4, "wPlY": 1.053e-3, "wPlZ": 4.985e-4, "areaNet": 0.0095, "fabrication": "Unset"}]
member = by_id("members", "member-b1")
member.update(label="Beam-column B1", sectionId="sec-heb240", materialId="mat-s355", length=4.0, bucklingLengthY=4.0, bucklingLengthZ=4.0, ltbLength=4.0, ltbRestraintSpacing=2.0)
show(member, "SteelMember")


def action(aid, mid, case, **forces):
    return {"id": aid, "memberId": mid, "loadCaseId": case, "action": {"n": 0.0, "vy": 0.0, "vz": 0.0, "my": 0.0, "mz": 0.0, "t": 0.0, **forces}}


doc["memberActions"] = [action("act-b1-g", "member-b1", "g-permanent", n=100000.0, vz=20000.0, my=40000.0), action("act-b1-q", "member-b1", "q-imposed", n=43333.0, vz=15333.0, my=17333.0)]

doc["fatigueDetails"] = [{"id": "fat-1", "memberId": "member-b1", "category": 71, "method": "DamageTolerant", "spectrum": [{"id": "fb-1", "deltaSigma": 50.0e6, "cycles": 1.0e6}, {"id": "fb-2", "deltaSigma": 40.0e6, "cycles": 1.0e6}], "stressHistory": ""}]
doc["fireExposures"] = [{"id": "fire-1", "memberId": "member-b1", "rating": "R60", "protectionThickness": 0.020, "sectionFactor": 150.0, "mu0": 0.50, "designTemperature": 550.0, "protectionConductivity": 0.20, "protectionDensity": 800.0, "protectionSpecificHeat": 1700.0}]

joint = by_id("joints", "joint-j1")
joint.update(boltClass="8.8", boltDiameter=0.020, boltRows=2, boltsPerRow=2, pitch=0.06, gauge=0.06, endDistance=0.04, edgeDistance=0.04, shearPlanes=1, plateThickness=0.010, plateFu=510.0e6, weldThroat=0.0, weldLength=0.0, weldFu=510.0e6, weldGrade="S355", category="A", frictionMu=0.5, preloadForce=0.0, slipFactorKs=1.0, frictionSurfaces=1,
             actions=[{"id": "jf-g", "loadCaseId": "g-permanent", "shear": 30000.0, "tension": 0.0}, {"id": "jf-q", "loadCaseId": "q-imposed", "shear": 23333.0, "tension": 0.0}])
template = copy.deepcopy(joint)

by_id("coldFormedMembers", "cf-1").update(bBar=0.080, thickness=0.003, kSigma=4.0, psi=1.0, fy=350.0e6, grossResistance=80000.0, actions=[{"id": "cf-g", "loadCaseId": "g-permanent", "force": 15000.0}, {"id": "cf-q", "loadCaseId": "q-imposed", "force": 10000.0}])
by_id("platedPanels", "pp-1").update(actions=[{"id": "pp-g", "loadCaseId": "g-permanent", "force": 80.0e6}, {"id": "pp-q", "loadCaseId": "q-imposed", "force": 40.0e6}])
by_id("siloShells", "silo-1").update(fy=235.0e6)
by_id("tensionComponents", "ten-1").update(fUk=650000.0, fK=480000.0, actions=[{"id": "ten-g", "loadCaseId": "g-permanent", "force": 120000.0}, {"id": "ten-q", "loadCaseId": "q-imposed", "force": 80000.0}])
by_id("bridgeFatigue", "br-1").update(**{"lambda": 1.0}, phi2=1.0, deltaSigmaP=35.0e6, category=71, method="DamageTolerant")
by_id("towerLegs", "tw-1").update(forceCoefficient=1.2, dynamicFactor=1.0, actions=[{"id": "tw-g", "loadCaseId": "g-permanent", "force": 80000.0}, {"id": "tw-w", "loadCaseId": "w-wind", "force": 40000.0}])
by_id("piles", "pile-1").update(sectionId="sec-heb240", materialId="mat-s355", actions=[{"id": "pile-g", "loadCaseId": "g-permanent", "force": 200000.0}, {"id": "pile-q", "loadCaseId": "q-imposed", "force": 100000.0}])
by_id("craneRunways", "crane-1").update(actions=[{"id": "cr-g", "loadCaseId": "g-permanent", "force": 20000.0}, {"id": "cr-q", "loadCaseId": "q-imposed", "force": 30000.0}])

ROWS = {"ColdFormedMember": ("coldFormedMembers", "cf-1"), "PlatedPanel": ("platedPanels", "pp-1"), "SiloShell": ("siloShells", "silo-1"), "TensionComponent": ("tensionComponents", "ten-1"), "BridgeFatigue": ("bridgeFatigue", "br-1"), "TowerLeg": ("towerLegs", "tw-1"), "SteelPile": ("piles", "pile-1"), "CraneRunway": ("craneRunways", "crane-1")}
for ent, (coll, rid) in ROWS.items():
    show(by_id(coll, rid), ent)

BASE_SHOW = {"base_plate_length": 0.35, "concrete_fcd": 16.7e6, "concrete_modulus": 31.0e9, "concrete_area_a0": 0.098, "concrete_area_a1": 0.5, "joint_coefficient": 0.667, "anchor_area": 245.0e-6, "anchor_length": 0.2, "base_normal_force": -300000.0, "base_moment": 60000.0, "base_lever_left": 0.15, "base_lever_right": 0.15}
BP_SHARED = {"plate_fy", "plate_width", "bolt_to_web", "column_flange_thickness", "column_web_thickness", "column_fy", "bolt_stress_area"}
end_plate = show(clone(template, id="joint-ep", kind="EndPlate", boltClass="10.9", boltRows=2, boltsPerRow=2, pitch=0.09, gauge=0.11, endDistance=0.04, edgeDistance=0.04, plateThickness=0.02, weldThroat=0.006), "SteelJoint", skip=set(BASE_SHOW))
base_plate = show(clone(template, id="joint-bp", kind="BasePlate", boltClass="8.8", boltRows=1, boltsPerRow=2, edgeDistance=0.05, plateThickness=0.03), "SteelJoint", only=BP_SHARED | set(BASE_SHOW), override={**BASE_SHOW, "plate_width": 0.3, "bolt_to_web": 0.05})
doc["joints"] += [end_plate, base_plate]

stainless = {"id": "mat-1.4301", "grade": "1.4301", "fy": 210.0e6, "fu": 520.0e6, "eModulus": 200.0e9, "gModulus": 77.0e9, "subgrade": "", "kind": "Stainless", "rambergOsgoodN": 0.0}
ss_member = clone(member, id="member-ss", label="Stainless beam-column", materialId=stainless["id"], stainlessForming="ColdFormedOpen")
doc["materials"].append(stainless)
doc["members"].append(ss_member)
doc["memberActions"].append(action("act-ss-g", "member-ss", "g-permanent", n=40000.0, vz=10000.0, my=15000.0))
doc["fireExposures"].append(clone(doc["fireExposures"][0], id="fire-ss", memberId="member-ss", mu0=0.30, designTemperature=550.0))
pipeline = clone(by_id("siloShells", "silo-1"), id="pipe-1", radius=0.3, thickness=0.012, depth=1.0, length=0.0, fabricationClass="Unset", wallFriction=0.0, planArea=0.0, planPerimeter=0.0, liquidDensity=0.0, liquidHeight=0.0, courseElevation=0.0, internalPressure=4.0e6, externalPressure=0.1e6, temperatureChange=30.0)
doc["siloShells"].append(pipeline)

silo = by_id("siloShells", "silo-1")
ring = clone(silo, id="ring-1", radius=1.0, thickness=0.008, fy=355.0e6, length=3.0, fabricationClass="B", boundaryFactorHoop=1.0, externalPressure=0.5e6, internalPressure=0.0, meridionalStress=0.0, hoopStress=0.0, shellShearStress=0.0, wallFriction=0.0, planArea=0.0, planPerimeter=0.0, liquidDensity=0.0, liquidHeight=0.0, courseElevation=0.0, temperatureChange=0.0, ringSpacing=1.5, ringArea=0.0015, ringInertia=2.0e-6, ringOffset=0.08)
doc["siloShells"].append(ring)
pp = by_id("platedPanels", "pp-1")
doc["platedPanels"].append(clone(pp, id="corr-1", thickness=0.002, fy=355.0e6, shearForce=30000.0, corrugationFlat=0.14, corrugationProjection=0.04, corrugationDepth=0.05, webHeight=1.0))
doc["platedPanels"].append(clone(pp, id="stiff-1", a=3.0, b=1.5, thickness=0.010, fy=355.0e6, compressionStress=100.0e6, stiffenerArea=0.001, stiffenerInertia=8.333333333333334e-7, stiffenerEccentricity=0.055, stiffenerSpacing=0.375, alphaCr=0.0))
doc["bridgeFatigue"].append(clone(by_id("bridgeFatigue", "br-1"), id="deck-1", heavyVehicleWeight=400000.0, referenceWeight=480000.0, observedTraffic=0.5e6, designLife=100.0, secondLaneTraffic=0.0, secondLaneWeight=0.0, lambda1=1.0, lambdaMax=2.0, phi2=1.2, category=71, detailId="rib-to-deck-partial-penetration-plate", plateThickness=0.014, deckThickness=0.014, troughSpacing=0.6, troughWidth=0.3, troughThickness=0.008, crossbeamSpacing=3.0, surfacingThickness=0.07, wheelLoad=60000.0, wheelPatch=0.4))
doc["coldFormedMembers"].append(clone(by_id("coldFormedMembers", "cf-1"), id="purlin-1", thickness=0.002, fy=350.0e6, flangeWidth=0.06, lipLength=0.02, webHeight=0.2, sheetingStiffness=4000.0, momentResistance=15000.0, purlinSpan=6.0, purlinLoad=2500.0, purlinLateralRatio=0.15))

doc["members"] += [clone(member, id="member-tension", label="Tension member", sectionId="sec-heb240"), clone(member, id="member-torsion", label="Member under torsion"), clone(member, id="member-parabolic", label="Member under uniform load", momentDiagram="Parabolic")]
doc["memberActions"] += [action("act-tn-g", "member-tension", "g-permanent", n=-100000.0, vz=20000.0, my=40000.0), action("act-tn-q", "member-tension", "q-imposed", n=-43333.0, vz=15333.0, my=17333.0),
                         action("act-to-g", "member-torsion", "g-permanent", n=100000.0, vz=20000.0, my=40000.0, mz=3000.0, t=5000.0), action("act-to-q", "member-torsion", "q-imposed", n=43333.0, vz=15333.0, my=17333.0, mz=3000.0, t=5000.0),
                         action("act-pb-g", "member-parabolic", "g-permanent", n=100000.0, vz=20000.0, my=40000.0), action("act-pb-q", "member-parabolic", "q-imposed", n=43333.0, vz=15333.0, my=17333.0)]
tension_joint = clone(template, id="joint-b1", category="B", actions=[clone(a, tension=20000.0) for a in template["actions"]])
welded_joint = clone(template, id="joint-w1", kind="Welded", weldThroat=0.005, weldLength=0.2, plateWidth=0.2, plateFy=355.0e6, columnFy=355.0e6, columnFlangeThickness=0.017, columnWebThickness=0.010)
doc["joints"] = [j for j in doc["joints"] if j["id"] != "joint-w1"] + [tension_joint, welded_joint]
V7 = {}
exec(compile((SCRIPTS / "en1993-v7-spec.py").read_text(encoding="utf8"), "spec-v7", "exec"), V7)


ENUM_TOKENS = {"stiffenerShape": {"": "None", "flat": "Flat", "tee": "Tee", "angle": "Angle"}, "warpingSupport": {"fork": "Fork", "fixed": "Fixed", "fixed-fork": "FixedFork", "cantilever": "Cantilever"}}


def show_v7(row, ent):
    for fd in V7["SPEC"][ent]["fields"]:
        key = camel(fd["name"])
        row[key] = ENUM_TOKENS.get(key, {}).get(fd["show"], fd["show"]) if fd["ty"] == "String" else float(fd["show"])
    return row


show_v7(by_id("platedPanels", "pp-1"), "PlatedPanel")
hybrid_steel = {"id": "mat-s460", "grade": "S460", "fy": 460.0e6, "fu": 540.0e6, "eModulus": 210.0e9, "gModulus": 81.0e9, "subgrade": "", "kind": "Carbon", "rambergOsgoodN": 0.0}
doc["materials"].append(hybrid_steel)
doc["members"].append(clone(member, id="member-hybrid", label="Hybrid girder (S460 flanges, S355 web)", materialId="mat-s460", webMaterialId="mat-s355"))
doc["memberActions"] += [action("act-hy-g", "member-hybrid", "g-permanent", n=100000.0, vz=20000.0, my=40000.0), action("act-hy-q", "member-hybrid", "q-imposed", n=43333.0, vz=15333.0, my=17333.0)]
by_id("members", "member-torsion").update(torsionLoad="Point", torquePosition=0.5, warpingSupport="Fixed")
doc["fatigueDetails"].append({"id": "fat-rf", "memberId": "member-b1", "category": 71, "method": "DamageTolerant", "spectrum": [], "stressHistory": "0 80 10 60 20 70 0 90 30 50 0"})
chs_section = {"id": "sec-chs168", "designation": "CHS 168.3x6.3", "kind": "Chs", "h": 0.1683, "b": 0.1683, "tw": 0.0063, "tf": 0.0063, "r": 0.0, "area": 0.00320502197878802, "shearAreaY": 0.0020403803625691245, "shearAreaZ": 0.0020403803625691245, "iy": 1.0525747577816918e-05, "iz": 1.0525747577816918e-05, "it": 2.1051041230876456e-05, "iw": 0.0, "wElY": 0.00012508315600495443, "wElZ": 0.00012508315600495443, "wPlY": 0.00016532092085208303, "wPlZ": 0.00016532092085208303, "areaNet": 0.0028845197809092183, "fabrication": "HotFinished"}
angle_section = {"id": "sec-l100x10", "designation": "L 100x100x10", "kind": "Angle", "h": 0.1, "b": 0.1, "tw": 0.01, "tf": 0.01, "r": 0.0, "area": 0.001900000000000004, "shearAreaY": 0.001, "shearAreaZ": 0.001, "iy": 2.865833333333334e-06, "iz": 7.342543859649179e-07, "it": 6.197665427176748e-08, "iw": 0.0, "wElY": 4.052900367500895e-05, "wElZ": 1.810041991413472e-05, "wPlY": 6.38753125671848e-05, "wPlZ": 3.220282132653735e-05, "areaNet": 0.0017100000000000036, "fabrication": "Unset"}
doc["sections"] += [chs_section, angle_section]
tower = doc["towerLegs"][0]
for tag, label, section_id in (("chs", "Tower leg (CHS 168.3x6.3, hot finished)", "sec-chs168"), ("angle", "Tower leg (L 100x100x10)", "sec-l100x10")):
    doc["members"].append(clone(member, id=f"member-tower-{tag}", label=label, memberType="Column", sectionId=section_id, length=5.0, bucklingLengthY=5.0, bucklingLengthZ=5.0, ltbLength=5.0, ltbRestraintSpacing=0.0))
    doc["towerLegs"].append(clone(tower, id=f"tw-{tag}", memberId=f"member-tower-{tag}", forceCoefficient=1.0, actions=[{"id": f"tw-{tag}-g", "loadCaseId": "g-permanent", "force": 80000.0}]))
doc["placements"] = [{"id": "member-b1", "x": 0.0, "y": 0.0, "z": 0.0, "yaw": 0.0}]

committed = load(ART / "🧪️tests/🧫️fixtures/🎪️showcase/🔣️.json")
plate_ic = clone(next(r for r in committed["jointPlates"] if r["id"] == "plate-fin"), id="plate-ic", label="Fin plate (instantaneous centre)", boltsPerRow=2, gauge=0.1, eccentricity=0.15, groupMethod="InstantaneousCentre")
doc["jointPlates"] = [plate_ic]
doc["fatigueStresses"] = [{"id": "fstress-1", "detailId": "fat-1", "maxCaseId": "g-permanent", "minCaseId": "q-imposed", "fibreZ": 0.12, "fibreY": 0.0, "relief": "StressRelieved", "concentration": 1.0, "thickness": 0.017, "cycles": 1.0e7}]
doc["bucklingLengths"] = [{"id": "buckling-1", "memberId": "member-b1", "plane": "MajorAxis", "sway": "NonSway", "restraintA": "Framed", "columnStiffnessA": 5.63e-5, "beamStiffnessA": 5.63e-5, "restraintB": "Framed", "columnStiffnessB": 5.63e-5, "beamStiffnessB": 5.63e-5}]
doc["formingGains"] = [{"id": "gain-1", "memberId": "cf-1", "forming": "RollForming", "ultimateStrength": 420.0e6, "grossArea": 0.0006, "bends": 4, "usedYield": 370.0e6}]
doc["fastenedJoints"] = [{"id": "fastened-1", "memberId": "cf-1", "kind": "SelfTappingScrew", "diameter": 0.0055, "washerDiameter": 0.016, "count": 4, "perRow": 2, "headThickness": 0.00075, "baseThickness": 0.0015, "headStrength": 360.0e6, "baseStrength": 360.0e6, "endDistance": 0.03, "edgeDistance": 0.015, "pitch": 0.03, "gauge": 0.03, "sheetWidth": 0.1, "shearForce": 4000.0, "tensionForce": 1200.0, "shearResistance": 0.0}]
doc["materialDuctility"] = [{"id": "ductility-1", "materialId": "mat-s355", "elongation": 0.20, "ultimateStrain": 0.12}]
finish(doc, ART, "🧪️tests/🧫️fixtures/🎪️showcase/🔣️.json", expect_empty=("jointPlates", "foundationBlocks", "frameActions", "frameMemberBases"))
