#!/usr/bin/env python3
"""🥾️ F3 — independent oracle for the boot-budget law vectors: re-implements the resource-kind classification, the
per-kind payload sums and the budget verdict in Python (no shared code with the TS harness) and writes the vectors with
their expected values into `🧑‍💻dev/🧫️fixtures/🥾️boot-budget.json` (the budget block is kept as is).
usage: python3 f3-boot-budget-oracle.py [--check]   (--check: compare only, exit 1 on drift)
"""
import json
import re
import sys
from urllib.parse import unquote, urlparse

FIXTURE = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧫️fixtures/🥾️boot-budget.json"
KINDS = ["module", "json", "font", "style", "wasm", "other"]
MB = 1024 * 1024


def kind_of(url: str, initiator: str) -> str:
    path = unquote(urlparse(url).path)
    if path.endswith(".wasm"):
        return "wasm"
    if re.search(r"\.(woff2?|ttf|otf)$", path) or re.search(r"fonts?\.bin$", path):
        return "font"
    if path.endswith(".css"):
        return "style"
    if path.endswith(".json"):
        return "module" if initiator == "script" else "json"
    if re.search(r"\.(m?js|jsx|ts|tsx)$", path) or initiator == "script":
        return "module"
    return "other"


def payload(resources: list) -> dict:
    sums = {kind: [0, 0, 0] for kind in KINDS}
    for resource in resources:
        row = sums[kind_of(resource["url"], resource["initiatorType"])]
        row[0] += 1
        row[1] += resource["transferSize"]
        row[2] += resource["decodedBodySize"]
    return {kind: {"count": row[0], "transferMB": round(row[1] / MB + 1e-12, 2), "decodedMB": round(row[2] / MB + 1e-12, 2)} for kind, row in sums.items()}


def verdict(budget: dict, measured: dict) -> dict:
    judged = measured["load"] <= budget["loadCeilingPerCore"] * measured["cores"]
    violations = []
    for kind in KINDS:
        bound, row = budget["payload"][kind], measured["cold"]["payload"][kind]
        if row["count"] > bound["maxCount"]:
            violations.append(f"{kind}: {row['count']} files (bound {bound['maxCount']})")
        if row["decodedMB"] > bound["maxDecodedMB"]:
            violations.append(f"{kind}: {fmt(row['decodedMB'])} MB (bound {fmt(bound['maxDecodedMB'])} MB)")
    warm = round(sum(measured["warm"]["payload"][kind]["transferMB"] for kind in KINDS) + 1e-12, 2)
    if warm > budget["warm"]["maxTransferMB"]:
        violations.append(f"warm reload transferred {fmt(warm)} MB (bound {fmt(budget['warm']['maxTransferMB'])} MB)")
    for label in ("cold", "warm"):
        load = measured[label]
        if load["readyMs"] is None:
            violations.append(f"{label}: the shell never became ready")
        if load["plugins"]["total"] == 0 or load["plugins"]["loaded"] < load["plugins"]["total"]:
            violations.append(f"{label}: {load['plugins']['loaded']}/{load['plugins']['total']} plugins loaded")
    if judged:
        checks = [
            ("serve first answer", measured["serveReadyMs"], budget["serve"]["maxReadyMs"]),
            ("cold first contentful paint", measured["cold"]["fcpMs"], budget["cold"]["maxFcpMs"]),
            ("cold ready", measured["cold"]["readyMs"], budget["cold"]["maxReadyMs"]),
            ("cold all plugins loaded", measured["cold"]["pluginsLoadedMs"], budget["cold"]["maxPluginsLoadedMs"]),
            ("cold long tasks", measured["cold"]["longTaskMs"], budget["cold"]["maxLongTaskMs"]),
            ("warm ready", measured["warm"]["readyMs"], budget["warm"]["maxReadyMs"]),
        ]
        for label, value, bound in checks:
            if value is not None and value > bound:
                violations.append(f"{label} {value} ms (bound {bound} ms)")
    status = "fail" if violations else ("pass" if judged else "blocked")
    return {"status": status, "timingJudged": judged, "violations": violations}


def fmt(value: float) -> str:
    return str(int(value)) if float(value).is_integer() else repr(round(value, 2))


def resource(url: str, initiator: str, transfer: int, decoded: int) -> dict:
    return {"url": url, "initiatorType": initiator, "transferSize": transfer, "decodedBodySize": decoded}


def load(ready, fcp, plugins_loaded, total, loaded, long_ms, pay: dict) -> dict:
    return {"ttfbMs": 40, "fcpMs": fcp, "readyMs": ready, "pluginsLoadedMs": plugins_loaded, "plugins": {"total": total, "loaded": loaded}, "longTaskMs": long_ms, "longTasks": 1 if long_ms else 0, "payload": pay}


def kinds_payload(**rows) -> dict:
    return {kind: rows.get(kind, {"count": 0, "transferMB": 0, "decodedMB": 0}) for kind in KINDS}


BASE = "http://127.0.0.1:6700"
RESOURCES = [
    resource(f"{BASE}/@fs/Users/x/%F0%9F%A7%B1%EF%B8%8Felements/%F0%9F%8F%9B%EF%B8%8FShellHost/%F0%9F%9F%A6%EF%B8%8F.tsx", "script", 2_200_000, 2_200_000),
    resource(f"{BASE}/node_modules/.vite/deps/three-stdlib.js?v=1a2b", "script", 4_700_000, 4_700_000),
    resource(f"{BASE}/@id/virtual:semio-boot", "script", 1_000, 1_000),
    resource(f"{BASE}/plugins/%F0%9F%97%84%EF%B8%8Fstdio/%F0%9F%94%A3%EF%B8%8F.json", "fetch", 1_780_000, 1_780_000),
    resource(f"{BASE}/plugins/note/descriptor.json?t=5", "fetch", 0, 640_000),
    resource(f"{BASE}/@fs/Users/x/%F0%9F%A7%AB%EF%B8%8Ffixtures/%F0%9F%94%A3%EF%B8%8F.json?import", "script", 12_000, 12_000),
    resource(f"{BASE}/cursors/pointer.svg", "css", 700, 700),
    resource(f"{BASE}/%F0%9F%AA%9E%EF%B8%8Fvendor/%F0%9F%94%A4%EF%B8%8Fguestslim-typst-fonts.bin", "fetch", 8_760_000, 8_760_000),
    resource(f"{BASE}/fonts/inter.woff2", "css", 90_000, 90_000),
    resource(f"{BASE}/src/app.css", "link", 300, 1_040_000),
    resource(f"{BASE}/styles/theme", "css", 20_000, 20_000),
    resource(f"{BASE}/pkg/semio_guest_bg.wasm", "fetch", 3_000_000, 3_000_000),
    resource(f"{BASE}/_semio/hub/status", "fetch", 400, 400),
    resource(f"{BASE}/icons/app.png", "img", 5_000, 5_000),
]
WITHIN = kinds_payload(module={"count": 462, "transferMB": 40.3, "decodedMB": 40.27}, json={"count": 120, "transferMB": 17.64, "decodedMB": 17.61}, font={"count": 16, "transferMB": 9.34, "decodedMB": 9.34}, style={"count": 3, "transferMB": 0.36, "decodedMB": 0.36}, other={"count": 8, "transferMB": 0.01, "decodedMB": 0.01})
WARM = kinds_payload(module={"count": 462, "transferMB": 0.11, "decodedMB": 40.27}, json={"count": 120, "transferMB": 0.03, "decodedMB": 17.61}, style={"count": 3, "transferMB": 0.01, "decodedMB": 0.36})
OVER = kinds_payload(module={"count": 512, "transferMB": 46.2, "decodedMB": 46.2}, json={"count": 120, "transferMB": 17.67, "decodedMB": 17.64}, font={"count": 16, "transferMB": 9.34, "decodedMB": 9.34}, style={"count": 13, "transferMB": 1, "decodedMB": 0.99})
UNCACHED = kinds_payload(module={"count": 465, "transferMB": 40.67, "decodedMB": 40.65}, json={"count": 120, "transferMB": 17.67, "decodedMB": 17.64})


def vectors(budget: dict) -> list:
    cases = [
        ("quiet machine within budget", {"load": 3.2, "cores": 10, "serveReadyMs": 3782, "cold": load(8717, 6188, 8480, 60, 60, 393, WITHIN), "warm": load(2721, 1024, 2308, 60, 60, 215, WARM)}),
        ("saturated machine: timings over but not judged", {"load": 45.6, "cores": 10, "serveReadyMs": 65000, "cold": load(44000, 44600, 50700, 60, 60, 4000, WITHIN), "warm": load(9000, 3000, 5400, 60, 60, 900, WARM)}),
        ("quiet machine over every timing bound", {"load": 9.9, "cores": 10, "serveReadyMs": 15001, "cold": load(15001, 10001, 20001, 60, 60, 2001, WITHIN), "warm": load(5001, 1000, 5000, 60, 60, 0, WARM)}),
        ("payload over budget and an uncached warm reload", {"load": 45.6, "cores": 10, "serveReadyMs": None, "cold": load(8717, 6188, 8480, 60, 60, 393, OVER), "warm": load(2721, 1024, 2308, 60, 60, 215, UNCACHED)}),
        ("the shell never became ready", {"load": 2.0, "cores": 10, "serveReadyMs": None, "cold": load(None, 6188, None, 0, 0, 0, WITHIN), "warm": load(2721, 1024, 2308, 60, 58, 0, WARM)}),
    ]
    out = []
    for index, (name, measured) in enumerate(cases):
        resources = RESOURCES if index == 0 else RESOURCES[:4]
        out.append({"name": name, "resources": resources, "expectedKinds": [kind_of(r["url"], r["initiatorType"]) for r in resources], "expectedPayload": payload(resources), "measurement": measured, "expectedVerdict": verdict(budget, measured)})
    return out


def main() -> int:
    fixture = json.load(open(FIXTURE, encoding="utf-8"))
    expected = vectors(fixture["budget"])
    if "--check" in sys.argv:
        drift = expected != fixture["vectors"]
        print("drift" if drift else "vectors match the oracle")
        return 1 if drift else 0
    fixture["vectors"] = expected
    open(FIXTURE, "w", encoding="utf-8").write(json.dumps(fixture, ensure_ascii=False, indent=1) + "\n")
    for vector in expected:
        print(f"{vector['name']}: {vector['expectedVerdict']['status']} ({len(vector['expectedVerdict']['violations'])} violations), kinds {vector['expectedKinds']}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
