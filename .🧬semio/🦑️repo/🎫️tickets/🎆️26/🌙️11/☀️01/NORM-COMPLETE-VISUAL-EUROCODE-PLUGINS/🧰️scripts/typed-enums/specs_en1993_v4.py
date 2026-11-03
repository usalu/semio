import sys, re, json; sys.path.insert(0, ".")
import convert as C
from addspec import add
root = C.artifact_root("en1993")
t = open(root + "/✏️editor/🏷️field-meta/🦀️.rs", encoding="utf-8").read()
consts = {}
for m in re.finditer(r"const ([A-Z_0-9]+): &\[NormFieldChoice\] = &\[(.*?)\n?\];", t, re.S):
    consts[m.group(1)] = [(r.group(1), r.group(2), r.group(3)) for r in re.finditer(r'choice\("([^"]*)",\s*"((?:[^"\\]|\\.)*)",\s*"((?:[^"\\]|\\.)*)"\)', m.group(2))]
paths = {m.group(1): m.group(2) for m in re.finditer(r'\("([^"]+)", FieldSpec::choice\([^\n]*?, ([A-Z_0-9]+)\)\)', t)}
FIELDS = [
 ("SteelMember.member_type", "members[].memberType", "SteelMemberType", "🔩 Member type: beam or beam-column."),
 ("SteelMember.load_application", "members[].loadApplication", "LoadApplication", "⬇️ Level of the transverse load relative to the shear centre."),
 ("SteelMember.moment_diagram", "members[].momentDiagram", "MomentDiagram", "📈 Shape of the bending moment diagram between the lateral restraints."),
 ("SteelMember.analysis", "members[].analysis", "AnalysisKind", "🧮 Global analysis type (elastic or plastic)."),
 ("SteelMember.stainless_forming", "members[].stainlessForming", "StainlessForming", "🔩 Forming of a stainless steel member (EN 1993-1-4)."),
 ("SteelMember.torsion_load", "members[].torsionLoad", "TorsionLoad", "🌀 Application of the torsion action on a fork-supported member."),
 ("PlatedPanel.patch_type", "platedPanels[].patchType", "PatchLoadType", "📍 Patch load type of EN 1993-1-5 Figure 6.1."),
 ("SiloShell.fabrication_class", "siloShells[].fabricationClass", "FabricationToleranceClass", "🏭 Fabrication tolerance class of a silo shell."),
 ("SteelJoint.assumed_class", "joints[].assumedClass", "StiffnessClass", "🔗 Assumed stiffness class of a joint."),
 ("TowerLeg.lattice_kind", "towerLegs[].latticeKind", "LatticeKind", "🗼 Lattice member kind of a tower leg."),
 ("SteelPile.environment", "piles[].environment", "SheetPileEnvironment", "🌊 Corrosion environment of a sheet pile."),
 ("Restraint.start_support,Restraint.end_support", "restraints[].startSupport", "RestraintSupport", "⚓ Support condition at the end of a restrained member."),
 ("Restraint.load_distribution", "restraints[].loadDistribution", "RestraintDistribution", "📈 Load and moment distribution of a restrained member."),
 ("BuiltUpMember.chord_shape", "builtUpMembers[].chordShape", "ChordShape", "⬜ Chord section outline shape of a built-up member."),
 ("WeldGroup.pattern", "weldGroups[].pattern", "WeldPattern", "〰 Weld pattern of a weld group."),
 ("JointRow.row_type", "jointRows[].rowType", "JointRowType", "🔩 Row position of a bolt row."),
 ("FrameNode.support", "frameNodes[].support", "FrameNodeSupport", "⚓ Support of a frame node."),
 ("FrameBar.release_start,FrameBar.release_end", "frameBars[].releaseStart", "FrameBarRelease", "🔗 End connection of a frame bar."),
 ("FireExposure.rating", "fireExposures[].rating", "FireRating", "🔥 Fire resistance rating."),
 ("FatigueDetail.method,BridgeFatigue.method", "fatigueDetails[].method", "FatigueMethod", "♻ Fatigue assessment method."),
]
rows = []
for hosts, path, enum, doc in FIELDS:
    const = paths.get(path)
    if not const or const not in consts:
        print("MISSING", path); continue
    vs = []
    for tok, en, de in consts[const]:
        ident = "".join(w[:1].upper() + w[1:] for w in re.split(r"[^A-Za-z0-9]+", tok) if w) if tok else "Unset"
        name = ident if ident[:1].isalpha() else "V" + ident
        vs.append((name, en, de, list(dict.fromkeys([tok, tok.lower()]))))
    rows.append((hosts, enum, doc, vs[0][0], vs))
    print(enum, const, [v[0] for v in vs])
add("en1993", rows)
p = "specs/en1993.json"; d = json.load(open(p, encoding="utf-8")); d["wire"] = "camel"; json.dump(d, open(p, "w", encoding="utf-8"), ensure_ascii=False, indent=1)
