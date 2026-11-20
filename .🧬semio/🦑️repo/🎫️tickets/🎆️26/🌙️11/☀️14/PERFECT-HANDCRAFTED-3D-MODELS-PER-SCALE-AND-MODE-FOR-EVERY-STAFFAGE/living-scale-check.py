"""🌿 Check each living digital scale against its bones, thermal regions and authored mesh topology."""
import json
from pathlib import Path

catalog = next(Path.cwd().glob("*teaching/*architecture/*staffage/*catalog"))
files = sorted([*catalog.glob("*animals/*/*.json"), *catalog.glob("*vegetation/*/*.json")])
missing = []
for path in files:
    asset = json.loads(path.read_text())["asset"]
    bones = {b["id"] for b in asset["rig"]["bones"]}
    for den, volume in asset["volumes"].items():
        if volume["status"] != "authored":
            missing.append((asset["id"], den))
            continue
        body = {s["id"]: s for s in volume["body"]}
        for solid in [*body.values(), *volume["thermal"], *volume["structure"]]:
            if "of" in solid:
                assert solid["of"] in body, (path, den, solid["id"])
            elif "bone" in solid:
                assert solid["bone"] in bones, (path, den, solid["id"])
        if den == "500" and asset["id"] in {"horse", "dairy-cow", "sheep", "goat", "pig", "roe-deer", "labrador-dog"}:
            ends = {b for b in bones if b.startswith(("hoof-", "paw-"))}
            assert ends <= {s["bone"] for s in body.values()}, (path, ends - {s["bone"] for s in body.values()})
            assert ends <= {s.get("member") for s in volume["structure"] if s.get("part")=="member"}, (path, den)
        if asset["id"] == "small-leaved-lime":
            assert {"trunk","ld1"} <= {s["bone"] for s in body.values()}
            assert "TODO" not in str(volume["decisions"])
        if asset["representations"][den]["status"] != "unavailable":
            continue
        assert {s["region"] for s in volume["thermal"]} == {r["id"] for r in asset["thermal"]["regions"]}
        assert {s["tags"][1] for s in volume["thermal"] if (s.get("tags") or [""])[0] == "emitter"} == {e["id"] for e in asset["thermal"]["emitters"]}
        assert bones <= {s.get("member") for s in volume["structure"]}, (path, den, bones - {s.get("member") for s in volume["structure"]})
        for list_name in ("body", "thermal", "structure"):
            ids = [s["id"] for s in volume[list_name]]
            assert len(ids) == len(set(ids)), (path, den, list_name)
assert not missing, missing
print(f"{len(files) * 4} living asset scales passed")

