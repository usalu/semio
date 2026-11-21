"""🌐 Verify that each English and German pane actually passed on the static origin."""
from collections import Counter
import json
from pathlib import Path
import re
import sys
from urllib.parse import unquote, urlsplit

ROOT = Path(__file__).resolve().parents[7]
CASES = ROOT / "♻️mit-bestand/🧺️demonstrator/🔨️modules/🧪️e2e/📁️artifacts/🧫️cases.json"


def verify(report, access):
    cases = json.loads(CASES.read_bytes())
    expected = {(lang, title) for title in cases["titles"] if title.endswith(" boot") for lang in ("en", "de")}
    if len(expected) != 16 or report["errors"]:
        raise ValueError("Incomplete boot roster or global Playwright error")
    seen = Counter()
    suites = list(report["suites"])
    while suites:
        suite = suites.pop()
        suites.extend(suite.get("suites", []))
        for spec in suite["specs"]:
            for test in spec["tests"]:
                key = (test["projectName"], spec["title"])
                if test["expectedStatus"] != "passed" or not test["results"] or test["results"][-1]["status"] != "passed":
                    raise ValueError(f"Boot did not pass: {key}")
                seen[key] += 1
    if seen != Counter(expected):
        raise ValueError(f"Wrong boot coverage: missing {sorted(expected - seen.keys())}, extra or repeated {sorted(k for k in seen if k not in expected or seen[k] > 1)}")
    requests = re.findall(r'"[A-Z]+ ([^ ]+) HTTP/[0-9.]+" [0-9]{3} ', access)
    if not requests or any(unquote(urlsplit(url).path).rstrip("/") == "/__semio/agent-bridge" for url in requests):
        raise ValueError("Missing origin evidence or a request to the development agent bridge")
    print(f"{len(seen)} pane/language boots passed, {len(requests)} origin requests, zero agent-bridge requests")


if __name__ == "__main__":
    report, access = map(Path, sys.argv[1:])
    verify(json.loads(report.read_bytes()), access.read_text())
