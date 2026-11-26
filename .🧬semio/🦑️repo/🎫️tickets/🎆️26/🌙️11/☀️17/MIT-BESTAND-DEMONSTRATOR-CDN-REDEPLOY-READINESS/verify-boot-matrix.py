"""🌐 Verify the configured English and German cases on the static origin."""
from collections import Counter
import json
from pathlib import Path
import re
import sys
from urllib.parse import unquote, urlsplit

ROOT = Path(__file__).resolve().parents[7]
CASES = ROOT / "♻️mit-bestand/🧺️demonstrator/🔨️modules/🧪️e2e/📁️artifacts/🧫️cases.json"
BEHAVIOUR_CASES = ROOT / "♻️mit-bestand/🧺️demonstrator/🧪️tests/🎭️acceptance/🧭️behaviours/🧫️cases.json"


def coverage(report, discovery=False):
    if report["errors"]:
        raise ValueError("Global Playwright error")
    seen = Counter()
    suites = list(report["suites"])
    while suites:
        suite = suites.pop()
        suites.extend(suite.get("suites", []))
        for spec in suite["specs"]:
            for test in spec["tests"]:
                key = (test["projectName"], spec["title"])
                passed_once = len(test["results"]) == 1 and test["results"][0]["status"] == "passed"
                if test["expectedStatus"] != "passed" or (not discovery and not passed_once):
                    raise ValueError(f"Acceptance case did not pass: {key}")
                seen[key] += 1
    return seen


def require_coverage(seen, expected):
    extra = sorted(key for key, count in seen.items() if key not in expected or count != 1)
    missing = sorted(expected.keys() - seen.keys())
    if seen != expected or extra:
        raise ValueError(f"Wrong acceptance coverage: missing {missing}, extra or repeated {extra}")


def origin_requests(access):
    return [(method, unquote(urlsplit(url).path), int(status))
            for method, url, status in re.findall(r'"([A-Z]+) ([^ ]+) HTTP/[0-9.]+" ([0-9]{3}) ', access)]


def verify_origin(access):
    requests = origin_requests(access)
    if not requests or any(path.rstrip("/") == "/__semio/agent-bridge" for _, path, _ in requests):
        raise ValueError("Missing origin evidence or a request to the development agent bridge")
    failures = sorted({(status, path) for _, path, status in requests if status >= 400})
    if failures:
        raise ValueError(f"Unexpected origin responses ({len(failures)} unique): {failures[:8]}")
    return len(requests)


def verify(report, access):
    cases = json.loads(CASES.read_bytes())
    expected = Counter((project["name"], title) for title in cases["titles"] if title.endswith(" boot") for project in cases["projects"])
    if len(expected) != 16:
        raise ValueError("Incomplete boot roster")
    require_coverage(coverage(report), expected)
    print(f"{len(expected)} pane/language boots passed, {verify_origin(access)} origin requests, zero agent-bridge requests")


def discover_suite(discovery, projects, titles):
    if Counter(project["name"] for project in discovery["config"]["projects"]) != projects:
        raise ValueError("Wrong acceptance projects")
    discovered = coverage(discovery, discovery=True)
    required = Counter((project, title) for title in titles for project in projects)
    require_coverage(discovered, required)
    return discovered


def release_rosters(baseline, behaviours):
    cases = json.loads(CASES.read_bytes())
    projects = Counter(project["name"] for project in cases["projects"])
    baseline_cases = discover_suite(baseline, projects, cases["titles"])
    behaviour_cases = discover_suite(behaviours, projects, json.loads(BEHAVIOUR_CASES.read_bytes())["titles"])
    if sum(title.endswith(" boot") for _, title in baseline_cases) != 16:
        raise ValueError("Incomplete boot roster")
    overlap = baseline_cases.keys() & behaviour_cases.keys()
    if overlap:
        raise ValueError(f"Acceptance suites overlap: {sorted(overlap)}")
    return baseline_cases, behaviour_cases


def verify_release(rosters, baseline, behaviours, access):
    baseline_cases, behaviour_cases = rosters
    require_coverage(coverage(baseline), baseline_cases)
    require_coverage(coverage(behaviours), behaviour_cases)
    requests = verify_origin(access)
    print(f"{len(baseline_cases) + len(behaviour_cases)} acceptance cases passed "
          f"({len(baseline_cases)} baseline including 16 boots, {len(behaviour_cases)} behaviors), "
          f"{requests} origin requests, zero agent-bridge requests")


if __name__ == "__main__":
    report, access = map(Path, sys.argv[1:])
    verify(json.loads(report.read_bytes()), access.read_text())
