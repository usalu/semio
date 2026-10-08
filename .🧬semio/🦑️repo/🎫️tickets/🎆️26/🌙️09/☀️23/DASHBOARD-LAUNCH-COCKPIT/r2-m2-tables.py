#!/usr/bin/env python3
"""M-2 helper: renders the `.claude/launch.json` table of r2-slice-m2.md from the M-1a coverage data and the M-2 overrides."""
import json
T = "C:/git/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️23/DASHBOARD-LAUNCH-COCKPIT/"
claude = json.load(open(T + "🗑️generated/declarations/coverage.json", encoding="utf8"))["claude"]
resolved = {json.loads(l)["name"]: json.loads(l) for l in open(T + "🗑️generated/m2/claude-resolve-a2.jsonl", encoding="utf8")}
OVERRIDE = {
    "terra-jco-spike-static": ("-", "intentionally dropped", "the script `🧰️framework/🛍️products/💻️os/🧪️testkit/🧩️jcoprobe/🌐️harness/📜️script.ts` no longer exists (the testkit folder is gone)"),
    "mit-bestand-demonstrator-supervised": ("`semio run ticket:26/08/28/DEMONSTRATOR-END-TO-END-ALL-APPS/demonstrator-supervised`", "declared now", "ticket `DEMONSTRATOR-END-TO-END-ALL-APPS` is open: `🎮️commands.json` created, tool `demonstrator-supervised`"),
    "map-harness": ("-", "intentionally dropped", "ticket `GIS-MAP-END-TO-END` is closed and the entry's `📜️harnessscript.ts` does not exist"),
    "verify-gis2d-envelope-gates": ("-", "intentionally dropped", "folder `26/09/16/GIS-2D-END-TO-END-BUILD` has no `🎫️ticket.json`, so it is no open ticket; the registry cannot index a `🎮️commands.json` there"),
    "puzzle5d-react-supervised": ("`semio run ticket:26/09/17/PUZZLE-2D-5D-FEATURE-PARITY-WITH-3D/serve-supervised --param variant=puzzle5d`", "declared now", "ticket is open: tool `serve-supervised` (choice `variant` puzzle5d/puzzle3d/puzzle2d carries variant and port)"),
    "puzzle3d-react-supervised": ("`semio run ticket:26/09/17/PUZZLE-2D-5D-FEATURE-PARITY-WITH-3D/serve-supervised --param variant=puzzle3d`", "declared now", "same tool"),
    "puzzle2d-react-supervised": ("`semio run ticket:26/09/17/PUZZLE-2D-5D-FEATURE-PARITY-WITH-3D/serve-supervised --param variant=puzzle2d`", "declared now", "same tool"),
    "puzzle5d-battery": ("`semio run ticket:26/09/17/PUZZLE-2D-5D-FEATURE-PARITY-WITH-3D/puzzle5d-battery`", "declared now", "ticket is open: tool `puzzle5d-battery`"),
    "puzzle5d-battery-explore": ("`semio run ticket:26/09/17/PUZZLE-2D-5D-FEATURE-PARITY-WITH-3D/puzzle5d-battery-explore`", "declared now", "ticket is open: tool `puzzle5d-battery-explore`"),
    "storybook-static": ("`semio run tool:workspace/storybook-static`", "declared now", "was `workspace:dev -- storybook-static` without a ready declaration (`--wait-ready` would hang); tool with ready 6010 / `STORYBOOK_PORT`"),
    "storybook-framework-os": ("`semio run workspace:dev-storybook-framework-os`", "exact", "the entry's port 6011 never matched: the command binds 6010 (`STORYBOOK_PORT`), which is the declared ready port"),
    "architektur-und-technologie-quizze-steady": ("`semio run @teaching/architecture-quiz:dev --param steady`", "declared now", "new flag `steady` = `TEACHING_ARCHITECTURE_QUIZ_WATCH=off`"),
    "architektur-und-technologie-quizze-beside": ("`semio run @teaching/architecture-quiz:dev --param stack=beside`", "declared now", "new choice `stack=beside` = ports 6063 / 8793 and `PROCTOR_DATA={workspace}/.🧬semio/🎓️teaching/proctor-beside`; the Nx `options.env` pin of 6061 was removed so the choice can take effect"),
    "framework-snapshot-sqlite-io": ("-", "intentionally dropped", "target `@semio-tech/framework-rs:test-snapshot-sqlite-io` no longer exists"),
    "host-count-component-sqlite": ("`semio run @semio-tech/framework-plugin-host:count-component-check [--env SEMIO_TEST_ARTIFACT_DIR=<dir>]`", "exact", "the artifact directory is an output location, passed with `--env` when wanted"),
    "pptx-outline-ownership-source": ("`semio run @semio-tech/stdio-pptx-rs:test-outline-ownership --param cache=skip-local [--env SEMIO_TEST_ARTIFACT_DIR=<dir>]`", "exact", "the artifact directory is an output location, passed with `--env` when wanted"),
    "process3d-react-lane-attach": ("-", "intentionally dropped", "port 6222 belongs to no catalog playground (a ticket lane)"),
    "puzzle5d-native": ("`semio run playground:puzzle5d --param renderer=wgpu-native`", "exact", "resolves to `framework-os-dev:run-puzzle5d-native-dev`, the target `renderer-wgpu:native -- puzzle5d` reduces to; no URL, so no ready"),
}
rows = []
records = []
for number, entry in enumerate(claude, 1):
    name = entry["name"]
    kind = entry["kind"]
    if name in OVERRIDE:
        command, status, note = OVERRIDE[name]
    elif kind == "attach-only":
        playground = entry["id"]
        renderer = entry["parameters"]["renderer"]
        ready = resolved.get(name)
        port = ready["ready"]["port"] if ready and ready.get("ready") else "?"
        command = f"no command: `semio tasks`, then `semio open {playground}` (start with `semio run {playground} --param renderer={renderer} --detach --wait-ready`)"
        status = "exact" if port == entry["port"] else "MISMATCH"
        note = f"port {entry['port']} = ready port {port} of `{playground}` ({renderer})"
    else:
        parts = [f"`semio run {entry['id']}"]
        for key, value in (entry.get("parameters") or {}).items():
            if entry["id"].startswith("@semio-tech/pets-react"):
                key = {"port": "stories-port"}.get(key, key)
            parts.append(f" --param {key}={value}")
        if entry.get("extraArgs"):
            parts.append(" -- " + " ".join(entry["extraArgs"]))
        command = "".join(parts) + "`"
        status = "exact"
        note = ""
        verdict = entry.get("verdict", "")
        if "owner default is wgpu" in verdict:
            note = "renderer now explicit (`react`); the old command had no `SEMIO_RENDERER` and bound the wgpu port"
        elif entry["name"].startswith("architecture-pets"):
            note = "second gallery: own port and menagerie"
        elif "PROCTOR_PORT=8791" in verdict:
            note = "`PROCTOR_PORT=8791` is the owner default"
        elif "output location" in verdict:
            note = ""
    rows.append(f"| {number} | `{name}` | {command} | {status} | {note} |")
    records.append({"number": number, "original": name, "kind": kind, "command": command.strip("`") if command.startswith("`") and command.endswith("`") else command, "status": status, "note": note})
open(T + "🗑️generated/m2/claude-table.md", "w", encoding="utf8").write("\n".join(rows) + "\n")
json.dump(records, open(T + "🗑️generated/m2/claude-mapping.json", "w", encoding="utf8"), ensure_ascii=False, indent=1)
print(len(rows), sum(1 for r in rows if "MISMATCH" in r))
