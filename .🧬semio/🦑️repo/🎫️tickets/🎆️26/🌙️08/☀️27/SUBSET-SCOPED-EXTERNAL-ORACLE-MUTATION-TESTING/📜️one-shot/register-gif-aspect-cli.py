#!/usr/bin/env python3
# 🧵️ One-shot: registers the giflib CLI screen-descriptor reader for one gif subset and repoints
# set-pixel-aspect-ratio off `-uncarried`. Usage: <script> <87a|89a>
import json, subprocess, sys

ver = sys.argv[1]
V = f"✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/🏅️standards/🔖️{ver}/🪆️subsets/✳️any"
PROBES = "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/🏅️standards/🔖️89a/🪆️subsets/✳️any/🔬️probes/📜️script.ts"
ORACLE = f"{V}/🧪️oracle/🔣️.json"
CAP, OID = f"gif-{ver}-mutate-screen", "giflib-gif-screen-cli"

d = json.load(open(ORACLE))
manifests = json.loads(subprocess.run(["bun", f"{V}/🏭️generator/📜️script.ts", "aspect-manifests"], capture_output=True, text=True).stdout)
kinds = [e["mutation"] for e in manifests]

d["oracles"].append({
    "id": OID, "kind": "third-party-cli", "ecosystem": "native", "package": "giflib", "version": "6.1",
    "source": {"repository": "https://sourceforge.net/projects/giflib/", "license": "MIT"},
    "engine": {"family": "giflib", "implementation": "giftext screen-descriptor dump; gifbuild text round trip", "version": "6.1"},
    "capabilities": [CAP], "comparisonProfiles": ["semantic-gif-screen-v1"],
    "license": "MIT", "testOnly": True, "productionReachable": False, "networkDuringExecution": False,
    "platforms": ["darwin-arm64", "darwin-x64", "linux-x64", "linux-arm64"],
    "homepage": "https://giflib.sourceforge.net",
    "rationale": (
        "📖️ The last kind in this artifact, and the one this ticket was most confident could not be "
        "closed.\n\n"
        "The recorded negative was thorough about LIBRARIES and correct on every point: `gif` 0.13.3 "
        "(`encoder.rs:345`) and 0.14.2 (`encoder.rs:401`) each write a hardcoded `0u8` for the aspect "
        "byte, NEITHER has any parse path for byte 12 of the logical screen descriptor, and Pillow "
        "surfaces only `background` and `version` after a round trip. Both versions were read directly.\n\n"
        "What that survey never questioned is its own INVENTORY. It enumerated cargo, npm and PyPI — "
        "libraries — and never the installed command-line tools, even though Protocol v2 lists "
        "`third-party-cli` as a qualifying oracle kind alongside `third-party-library`.\n\n"
        "giflib supplies both halves. `giftext` prints the descriptor including `Aspect = N`. "
        "`gifbuild -d` dumps a text description carrying `pixel aspect byte N`, and `gifbuild` writes a "
        "GIF back from it — byte-deterministic across runs, checked, and it preserves the GIF87a "
        "signature on the 87a side rather than upgrading it.\n\n"
        "The only authored step is one line of that text description, which is fixture authoring — the "
        "goal statement admits handcrafted fixtures — with giflib doing every byte of the encoding and a "
        "different giflib tool doing the judging. The generator asserts every OTHER descriptor field "
        "(width, height, colour resolution, bits per pixel, background, image count) is identical across "
        "the pair, so the fixture moves the aspect byte and nothing else."
    ),
})
EVID = ("Validated BOTH ways: (before,before) -> equal:true; (before,after) -> equal:false, aspect 0 -> 49. "
        "Every other screen-descriptor field is asserted identical at generation time, so the pair moves the "
        "aspect byte alone. gifbuild's output is byte-identical across repeated runs, and on the 87a side it "
        "preserves the GIF87a signature. giftext's output echoes the input PATH, which is filtered out of the "
        "projection so a fixture does not project differently because of where it lives.")
CRIT = [
    {"id": "accepts-a-known-good-pair", "met": True, "detail": "identical (before,before) compares equal:true"},
    {"id": "rejects-a-known-bad-pair", "met": True, "detail": "the kind's own (before,after) compares equal:false on aspect"},
    {"id": "witnesses-what-no-library-reaches", "met": True, "detail": "both vendored gif crate versions write a hardcoded 0u8 and have no parse path for byte 12; Pillow surfaces only background and version"},
    {"id": "the-pair-differs-in-nothing-else", "met": True, "detail": "width, height, colour resolution, bits per pixel, background and image count are all asserted identical at generation"},
    {"id": "projection-is-location-stable", "met": True, "detail": "giftext echoes the input path; that line is excluded so the projection does not depend on the fixture's location"},
    {"id": "offline", "met": True, "detail": "giftext and gifbuild are installed locally; neither touches the network"},
]
for pid in ("gif-screen-project", "gif-screen-compare"):
    d.setdefault("probes", []).append({
        "id": f"{pid}-{ver}", "kind": "external-process", "ecosystem": "native", "package": "giflib", "version": "6.1",
        "engine": {"family": "giflib", "implementation": "giftext screen-descriptor dump", "version": "6.1"},
        "capabilities": [f"gif.screen.{pid.split('-')[-1]}"], "outputSchema": "semio.repository-test.probe-report/v2",
        "deterministic": True, "license": "MIT", "testOnly": True, "productionReachable": False,
        "networkDuringExecution": False, "command": ["bun", PROBES, pid],
        "qualification": {"status": "qualified", "evidence": EVID, "checkedAt": "2026-08-28", "criteria": CRIT},
    })
d.setdefault("comparisonProfiles", []).append({"id": "semantic-gif-screen-v1", "description": "The logical screen descriptor as giflib reports it — width, height, colour resolution, bits per pixel, background index, pixel aspect ratio and image count. The tool echoes the input path; that line is excluded."})
pipeline = f"gif-{ver}-screen-compare-v1"
d.setdefault("comparisonPipelines", []).append({
    "id": pipeline, "description": "Screen-descriptor equality through giflib's own dump. GATING for the aspect byte no library reader parses.",
    "stages": [
        {"probe": f"gif-screen-project-{ver}", "description": "An independent CLI reader accepts both files and reports the descriptor.", "inputs": ["expected-before-gif", "expected-after-gif"], "assertions": {"bothImport": True}},
        {"probe": f"gif-screen-compare-{ver}", "description": "Equality over the screen descriptor — the operative equality.", "inputs": ["expected-before-gif", "expected-after-gif"], "assertions": {"equal": True}},
    ],
})
wanted = set(kinds); rep = 0
for m in d["mutationManifests"][0]["mutations"]:
    if m["id"] in wanted:
        m["oracleRequirements"] = [{"capability": CAP, "qualifyingKind": "third-party-cli", "oracle": OID}]; rep += 1
for e in manifests: e["comparisonPipeline"] = pipeline
ids = {e["id"] for e in manifests}
d["fixtureManifests"] = [f for f in d.get("fixtureManifests", []) if f["id"] not in ids] + manifests
json.dump(d, open(ORACLE, "w"), ensure_ascii=False, indent=2); open(ORACLE, "a").write("\n")
print(f"gif@{ver}: repointed {rep}, fixtureManifests {len(d['fixtureManifests'])}")
