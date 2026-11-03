"""🧫 Writes the EN 1993 check-outcome fixture `families-fail` (copy of the showcase with a loaded plated panel, a heavy fatigue spectrum, a thin fire protection, a loaded cold-formed member, heavy frame loads and a soft bracing system, transverse and combined weld forces)."""
import copy, json, pathlib
art = pathlib.Path("/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🔩️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures")
d = json.load(open(art / "🎪️showcase/🔣️.json", encoding="utf8"))
pp = d["platedPanels"][0]
pp.update(compressionStress=2.6e8, patchForce=1.2e6, shearForce=1.6e6, bendingMoment=1.5e6, plasticMoment=2.0e6, flangeMoment=1.0e6, patchMoment=1.4e6, transverseStiffenerInertia=5e-8, lateralPressure=4.0e5, flangeInducedK=0.3, flangeArea=0.02, stressX=2.5e8, stressZ=1.5e8, shearStress=1.2e8, alphaCr=1.5, halfWidth=1.2, equivalentSpan=6.0, hogging=1)
pp["actions"] = [{"id": "pp-g", "loadCaseId": "g-permanent", "force": 2.0e8}, {"id": "pp-q", "loadCaseId": "q-imposed", "force": 1.0e8}]
d["fatigueDetails"][0].update(category=36, method="SafeLife")
fire = d["fireExposures"][0]
fire.update(protectionThickness=0.002, mu0=0.8, designTemperature=300.0)
cf = d["coldFormedMembers"][0]
cf["actions"] = [{"id": "cf-g", "loadCaseId": "g-permanent", "force": 40000.0}, {"id": "cf-q", "loadCaseId": "q-imposed", "force": 30000.0}]
for n in d["frameNodes"]:
    if n["loadZ"]:
        n["loadZ"] = -5.0e6
storey = d["storeys"][0]
storey.update(verticalLoad=1.0e7, bracingLoad=400000.0, bracingLength=6.0, bracingDeflection=0.02)
d["restraints"][0].update(designMoment=4.5e5, generalMethod=1, designAxial=3.0e5, hingeToRestraint=4.5)
for a in d["memberActions"]:
    if a["memberId"] == "member-torsion":
        a["action"]["t"] = 40000.0
for j in d["joints"]:
    scale = {"joint-j1": 1.0, "joint-b1": 6.0}.get(j["id"])
    if scale:
        j["actions"] = [{**a, "tension": (40000.0 if a["id"] == "jf-g" else 30000.0) * scale} for a in j["actions"]]
by_id = {j["id"]: j for j in d["joints"]}
heavy_bolted = copy.deepcopy(by_id["joint-b1"])
heavy_bolted.update(id="joint-b2", plateThickness=0.008)
heavy_bolted["actions"] = [{"id": "jf-g2", "loadCaseId": "g-permanent", "shear": 400000.0, "tension": 360000.0}, {"id": "jf-q2", "loadCaseId": "q-imposed", "shear": 300000.0, "tension": 270000.0}]
thin_flange = copy.deepcopy(by_id["joint-w1"])
thin_flange.update(id="joint-w2", columnFlangeThickness=0.008, plateThickness=0.012, weldLength=0.2)
thin_flange["actions"] = [{"id": "jf-g3", "loadCaseId": "g-permanent", "shear": 300000.0, "tension": 0.0}, {"id": "jf-q3", "loadCaseId": "q-imposed", "shear": 200000.0, "tension": 0.0}]
transverse_weld = copy.deepcopy(by_id["joint-w1"])
transverse_weld.update(id="joint-w3")
transverse_weld["actions"] = [{"id": "jf-g4", "loadCaseId": "g-permanent", "shear": 0.0, "tension": 0.0}, {"id": "jf-q4", "loadCaseId": "q-imposed", "shear": 0.0, "tension": 0.0}]
transverse_weld.update(weldForceLongitudinal=0.0, weldForceTransverse=315000.0)
combined_weld = copy.deepcopy(by_id["joint-w1"])
combined_weld.update(id="joint-w4")
combined_weld["actions"] = [{"id": "jf-g5", "loadCaseId": "g-permanent", "shear": 0.0, "tension": 0.0}, {"id": "jf-q5", "loadCaseId": "q-imposed", "shear": 0.0, "tension": 0.0}]
combined_weld.update(weldForceLongitudinal=126000.0, weldForceTransverse=315000.0)
d["joints"] += [heavy_bolted, thin_flange, transverse_weld, combined_weld]
heavy_weld_group = copy.deepcopy(d["weldGroups"][0])
heavy_weld_group.update(id="weld-group-2", throat=0.003, pattern="Line")
d["weldGroups"].append(heavy_weld_group)
d["hollowJoints"][0].update(brace1Force=900000.0, brace2Force=900000.0)
d["tensionComponents"][0]["actions"] = [{"id": "ten-g", "loadCaseId": "g-permanent", "force": 400000.0}, {"id": "ten-q", "loadCaseId": "q-imposed", "force": 300000.0}]
out = art / "📄️oracle-documents/families-fail"
out.mkdir(exist_ok=True)
with open(out / "🔣️.json", "w", encoding="utf8") as f:
    json.dump(d, f, ensure_ascii=False, indent=2)
    f.write("\n")
