"""🧾 Summarize the latest durable matrix cells and check their recorded scope."""

from argparse import ArgumentParser
from collections import Counter
from datetime import datetime, timezone
from hashlib import sha256
from html import escape
import json
import os
from pathlib import Path

TICKET = Path(__file__).resolve().parent
APPS = json.loads((TICKET / "matrix-cells.json").read_text(encoding="utf8"))["apps"]
LOCALES = ("en", "de")
WIDTHS = (1440, 768, 375)
HEIGHT = 900
MIN_STORIES = {"elements": 126, "os": 72}


def markdown(value):
    """📝 Escape report strings before placing them in Markdown text or a table."""
    return escape(str(value)).replace("|", "&#124;").replace("\n", " ")


def checkpoint_revision(app):
    """🔑 Identify the checkpoint version that the current story runner can resume."""
    previous = TICKET.parent.parent / "☀️17" / "FINISH-SHARED-BORDER-CONSOLIDATION-ACROSS-UI"
    script, fixture = ("sweep-boot.js", TICKET / "elements-live.tsx") if app == "elements" else (
        "os-sweep.js", previous / "round3-os-live.tsx"
    )
    digest = sha256()
    for source in (TICKET / "runner-bridge.js", TICKET / "silhouette-probe.js", TICKET / script, fixture):
        digest.update(source.read_bytes())
    return digest.hexdigest()[:12]


def read_cell(directory, app, locale, width):
    """📬 Read one persisted cell and account for absent or malformed results."""
    stem = f"{app}-{locale}-{width}-{HEIGHT}"
    file = directory / f"{stem}.json"
    def unfinished(default_status, metadata, detail):
        if app in MIN_STORIES:
            partial = directory / "checkpoints" / f"{stem}-{checkpoint_revision(app)}.json"
            try:
                count = len(json.loads(partial.read_text(encoding="utf8")))
                return "Pending", metadata, [*detail, f"Checkpoint contains {count} completed stories."]
            except (OSError, ValueError):
                pass
        return default_status, metadata, detail

    try:
        attempt_file = directory / "attempts" / f"{stem}.json"
        attempt = json.loads(attempt_file.read_text(encoding="utf8")) if attempt_file.exists() else {}
        data = json.loads(file.read_text(encoding="utf8")) if file.exists() else {}
        if not isinstance(data, dict) or not isinstance(attempt, dict):
            raise ValueError("Expected an object")
    except (OSError, ValueError) as error:
        return "Error", {}, [f"Result unreadable: {error}"]

    if attempt and data.get("attempt") != attempt.get("attempt"):
        metadata = {"at": attempt.get("at", "—")}
        state = attempt.get("state")
        message = attempt.get("message")
        if state in {"timeout", "error"}:
            return "Error", metadata, [message or "The latest attempt ended before a result was saved."]
        if state == "cancelled":
            return "Cancelled", metadata, [message or "The latest attempt was cancelled."]
        return unfinished("Pending", metadata, ["The current attempt is waiting for a result."])
    if not data:
        return unfinished("Missing", {}, [])

    issues = list(data.get("errors") or [])
    if (data.get("app"), data.get("locale"), data.get("width"), data.get("height")) != (app, locale, width, HEIGHT):
        issues.append("Persisted cell does not match its filename.")
    if issues:
        return "Error", data, issues

    if data.get("observedLocale") != locale:
        issues.append(f"Expected locale {locale} and observed {data.get('observedLocale') or 'none'}.")
    if (data.get("actualWidth"), data.get("actualHeight")) != (width, HEIGHT):
        issues.append(f"Viewport was {data.get('actualWidth')} × {data.get('actualHeight')}.")
    targets = data.get("targets")
    focus = data.get("focusChecks")
    if not isinstance(targets, int) or targets <= 0:
        issues.append("No measured targets.")
    if not isinstance(focus, int) or focus <= 0:
        issues.append("No focus-visible checks.")
    stories = data.get("stories") or []
    reports = data.get("report") or []
    if not isinstance(stories, list) or not all(
        isinstance(story, dict) and isinstance(story.get("name"), str) and
        isinstance(story.get("targets"), int) and isinstance(story.get("failing"), int)
        for story in stories
    ):
        return "Error", {}, ["The result has invalid story entries."]
    if not isinstance(reports, list) or not all(isinstance(report, dict) for report in reports):
        return "Error", {}, ["The result has invalid finding entries."]
    if targets != sum(story["targets"] for story in stories):
        issues.append("Story target totals disagree with the result.")
    if data.get("failures") != len(reports) or len(reports) != sum(story["failing"] for story in stories):
        issues.append("Finding totals disagree with the report.")
    if issues:
        return "Error", data, issues

    if app in MIN_STORIES and len(stories) < MIN_STORIES[app]:
        issues.append(f"Recorded {len(stories)} stories, below the previous {MIN_STORIES[app]}-story baseline.")
    if app == "play" and not all(any(story["name"] == stage and story["targets"] for story in stories) for stage in ("onboarding", "landing")):
        issues.append("Onboarding and post-Skip landing results were not recorded separately.")
    if issues:
        return "Incomplete", data, issues
    return ("Findings" if reports else "Pass"), data, []


def main():
    """🗂️ Render a full matrix and distinguish errors from silhouette findings."""
    parser = ArgumentParser(description=__doc__)
    parser.add_argument("--check", action="store_true", help="Exit unsuccessfully unless every cell passed.")
    options = parser.parse_args()
    directory = TICKET / "🗑️generated" / "matrix"
    outcomes = Counter()
    row_totals = Counter()
    rows = []
    details = []
    for app in APPS:
        for locale in LOCALES:
            for width in WIDTHS:
                status, data, issues = read_cell(directory, app, locale, width)
                outcomes[status] += 1
                stories = data.get("stories") or []
                if app == "play":
                    coverage = ", ".join(f"{story['name']} {story['targets']}" for story in stories) or "—"
                else:
                    coverage = str(len(stories)) if stories else "—"
                values = (app, locale, width, data.get("targets", "—"), data.get("focusChecks", "—"),
                          data.get("failures", "—"), coverage, status, data.get("at", "—"))
                rows.append("| " + " | ".join(map(markdown, values)) + " |")
                if status in {"Pass", "Findings"}:
                    row_totals["targets"] += data["targets"]
                    row_totals["focus"] += data["focusChecks"]
                    row_totals["findings"] += data["failures"]
                reports = data.get("report") or []
                if issues or reports:
                    details.extend([f"### {app} {locale} at {width} × {HEIGHT}", ""])
                    details.extend(f"- {markdown(issue)}" for issue in issues)
                    grouped = Counter()
                    for report in reports:
                        if "issue" in report:
                            grouped[report["issue"]] += 1
                        else:
                            label = f"{report.get('story')} / {report.get('mode')} / {report.get('label')}"
                            grouped[f"{label} · {' · '.join(report.get('issues') or [])}"] += 1
                    details.extend(f"- {markdown(finding)}" + (f" ({count} occurrences)" if count > 1 else "") for finding, count in grouped.items())
                    details.append("")
    total = sum(outcomes.values())
    counts = " · ".join(f"{outcomes[state]} {state.lower()}" for state in
                        ("Pass", "Findings", "Incomplete", "Error", "Cancelled", "Pending", "Missing"))
    measured = f"{row_totals['targets']} targets, {row_totals['focus']} focus-visible checks, {row_totals['findings']} findings in the validated cells."
    contents = [
        "# 🧪 Latest Live Silhouette Matrix", "",
        f"{total} expected cells across eight surfaces in English and German at height {HEIGHT}. {counts}.",
        measured, "", "## Results", "",
        "| App | Locale | Width | Targets | Focus | Findings | Stories or stages | Status | Record time UTC |",
        "| --- | --- | ---: | ---: | ---: | ---: | --- | --- | --- |",
        *rows, "", "## Findings and gaps", "",
        *(details or ["No findings recorded in the available cells.", ""]),
        "## Scope", "",
        "- Elements and OS exercise the story and showcase fixtures with the shared locale provider. Some fixture-supplied sample strings are static.",
        "- CAD exercises the CAD chrome fixture and GIS inference panel. Hub uses the authorized mock API and the rendered locale switch.",
        "- Play exercises the onboarding overlay and the real landing with inert pane contents.",
        "- The probe checks the rendered viewport, simulates pointer transitions and records focus-visible checks. Clipped controls and unvisited app states remain outside the measured scope.",
        "- Target limits are 30 per Elements story, 20 per OS story and 50 in Staffage.",
        "", f"Generated {datetime.now(timezone.utc).isoformat(timespec='seconds')}.",
    ]
    directory.mkdir(parents=True, exist_ok=True)
    draft = directory / f"live-final-{os.getpid()}.md"
    draft.write_text("\n".join(contents) + "\n", encoding="utf8")
    draft.replace(TICKET / "live-final.md")
    return 1 if options.check and outcomes["Pass"] != total else 0


if __name__ == "__main__":
    raise SystemExit(main())
