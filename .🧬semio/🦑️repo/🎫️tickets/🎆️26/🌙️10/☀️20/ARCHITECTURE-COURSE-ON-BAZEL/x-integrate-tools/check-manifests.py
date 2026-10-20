#!/usr/bin/env python3
"""Semantic invariants of the two manifests after the series (the keys several sets touch). usage: check-manifests.py <root>"""
import json, os, sys

root = os.path.abspath(sys.argv[1])
COURSE = "🎓️teaching/🏛️architecture/📚️course/📦️packages/🟦️typescript/📋️project.json"
PROCTOR = "🎓️teaching/🛂️proctor/📦️packages/🦀️rust/📋️project.json"
course, proctor = (json.load(open(os.path.join(root, rel), encoding="utf8")) for rel in (COURSE, PROCTOR))
ct, pt = course["targets"], proctor["targets"]
RECEIPT = {"dependentTasksOutputFiles": "**/receipt.json", "transitive": False}
failures = []


def expect(label, condition):
    if not condition:
        failures.append(label)


def cache_class(target):
    return ((target.get("metadata") or {}).get("semio") or {}).get("cache")


expect("course build: cached with the store receipt as its only output", ct["build"].get("cache") is True and ct["build"].get("outputs") == ["{projectRoot}/.🧾️store/build.json"])
expect("course build: no managed-output decision beside cache true", cache_class(ct["build"]) is None)
expect("course build: closure over the site tsconfig", ct["build"]["metadata"]["semio"]["closure"] == {"typescript": ["{projectRoot}/tsconfig.site.json"]})
expect("course build: PROCTOR_URL keys the artifact", {"env": "PROCTOR_URL"} in ct["build"]["inputs"])
expect("course publish: uncached, depends on build", ct["publish"].get("cache") is False and "@teaching/architecture-course:build" in ct["publish"]["dependsOn"])
expect("course publish: side-effecting decision kept", (cache_class(ct["publish"]) or {}).get("class") == "side-effecting")
expect("course test: cached, generators as dependencies, declared closure", ct["test"].get("cache") is True and len(ct["test"]["dependsOn"]) == 6 and ct["test"]["metadata"]["semio"]["closure"]["modules"])
expect("course typecheck: cached with a declared tsconfig closure", ct["typecheck"].get("cache") is True and ct["typecheck"]["metadata"]["semio"]["closure"]["typescript"] == ["{projectRoot}/tsconfig.json"])
expect("course check: cached, depends on the proctor product", ct["check"].get("cache") is True and "@teaching/proctor:build" in ct["check"]["dependsOn"])
expect("course check: keys on the proctor receipt", RECEIPT in ct["check"]["inputs"])
expect("course check: no whole-repo proctor glob, no root Cargo globs, no redundant Rust sources", not any(isinstance(i, str) and (i.endswith("🛂️proctor/**/*") or i.endswith("Cargo.toml") or i.endswith("Cargo.lock") or i in ("proctorRustSources", "proctorCargoManifests")) for i in ct["check"]["inputs"]))
expect("course named inputs: proctorRustSources gone, proctorCargoManifests kept for test", "proctorRustSources" not in course["namedInputs"] and "proctorCargoManifests" in course["namedInputs"] and "proctorCargoManifests" in ct["test"]["inputs"])
expect("course default: ui glob replaced by derived closures", "{workspaceRoot}/🧰️framework/🔨️modules/🖱️ui/**/*" not in course["namedInputs"]["default"])
expect("course deploy-check: typecheck, publish and the proctor product as graph tasks", all(task in ct["deploy-check"]["dependsOn"] for task in ("@teaching/architecture-course:typecheck", "@teaching/architecture-course:publish", "@teaching/proctor:build")) and ct["deploy-check"].get("cache") is False)
for verb in ("dev", "test-e2e", "docker-image-build"):
    expect(f"course {verb}: depends on the proctor product", "@teaching/proctor:build" in ct[verb]["dependsOn"])
for verb in ("dev", "dev-site"):
    parameters = ct[verb]["metadata"]["semio"]["dashboard"]["parameters"]
    expect(f"course {verb}: stack parameter", any(p.get("id") == "stack" for p in parameters))
expect("course dev-site: ready declaration kept", ct["dev-site"]["metadata"]["semio"]["dashboard"]["ready"]["port"] == 6061)
expect("proctor dev: ready and stack parameter", pt["dev"]["metadata"]["semio"]["dashboard"]["ready"]["port"] == 8791 and any(p.get("id") == "stack" for p in pt["dev"]["metadata"]["semio"]["dashboard"]["parameters"]))
expect("proctor build: cached, outputs both mode directories", pt["build"].get("cache") is True and pt["build"]["outputs"] == ["{projectRoot}/dist/build/prototype", "{projectRoot}/dist/build/release"])
expect("proctor build: key is cargoSources with the proctorRust closure", pt["build"]["inputs"] == ["cargoSources"] and "proctorRust" in proctor["namedInputs"]["cargoSources"] and "^cargo" not in proctor["namedInputs"]["cargoSources"])
for verb in ("dev", "check", "rebuild", "health", "backup", "restore", "erase", "prune", "capacity"):
    expect(f"proctor {verb}: depends on build", "build" in pt[verb]["dependsOn"])
expect("proctor check: cached, keys on the receipt", pt["check"].get("cache") is True and RECEIPT in pt["check"]["inputs"])
for owner, targets in (("course", ct), ("proctor", pt)):
    for name, target in targets.items():
        cached, decision = target.get("cache") is True, cache_class(target)
        expect(f"{owner} {name}: a cached target carries no uncached decision", not (cached and decision is not None and decision.get("class") != "deterministic"))
for failure in failures:
    print("FAIL", failure)
print(f"manifest invariants: {len(failures)} failure(s)")
sys.exit(1 if failures else 0)
