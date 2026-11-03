"""Static check: every field path a scene grip, dimension or marker of the EN 1992 scene binds is a SetFieldRule path of the mutation vocabulary (the runtime test `every_handle_binding_of_the_default_document_resolves_to_a_typed_kind` is the proof). Run with python3 -I."""
import pathlib, re, sys
ART = next(pathlib.Path("/home/deck/semio").glob("✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏛️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any"))
rules = set(re.findall(r'SetFieldRule \{ path: "([^"]+)"', (ART / "🧬️schema/🧬️mutations/🧭️edit-rules/🦀️.rs").read_text(encoding="utf-8")))
scene = "\n".join(p.read_text(encoding="utf-8") for p in (ART / "🎬️scene").rglob("🦀️.rs") if "🧪️tests" not in str(p))
fields = set(re.findall(r'(?:anchor_grip|punching_grip|length_grip)\([^;]*?"((?:members|anchors|bars|tendonPoints|studRings)?[A-Za-z.\[\]=]*?)"[^;]*?\)', scene))
direct = set(re.findall(r'"(span|width|height|effectiveDepth|cover|hEf|d|c1|headDiameter|columnWidth|columnDepth|stirrups\.spacing|fire\.axisDistance|prestress\.eccentricity)"', scene))
need = {"members[]." + f for f in ("span", "width", "height", "effectiveDepth", "cover", "longitudinal[].count", "longitudinal[].diameter", "stirrups.spacing", "fire.axisDistance", "prestress.eccentricity", "punching.columnWidth", "punching.columnDepth", "bars[].y", "bars[].z", "tendonPoints[].x", "tendonPoints[].z", "studRings[].radius")} | {"anchors[]." + f for f in ("hEf", "d", "c1", "headDiameter")}
missing = sorted(need - rules)
print("rules:", len(rules), "bound paths:", len(need), "missing:", missing)
sys.exit(1 if missing else 0)
