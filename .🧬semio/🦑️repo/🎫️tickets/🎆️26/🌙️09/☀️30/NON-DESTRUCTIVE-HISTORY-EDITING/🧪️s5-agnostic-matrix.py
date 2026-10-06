#!/usr/bin/env python3
"""🧮️ S5-AGNOSTIC: writes `📓️s5-agnostic-matrix.md` — one row per plugin artifact that wires an acceptance law: does its lib-test
compile on the channel-22 tree, the acceptance law's verdict, its document-lane leaves, how many declare `"editable": false`
(withdraw-only, design §22.20), its bare inputs (the inputs gate's `inferred` + `refused` rows of its document-lane leaves), the
owner WP and the first failing `file:line`. Sources: the wiring census (`🗑️generated/s5-agnostic/census.json`), the runner's rows
(`acceptance-results.tsv`, last row per crate), my classification (`🧪️s5-agnostic-classification.tsv`), the leaf descriptors in the
git index, and `🗑️generated/s5-agnostic/inputs.json` (`schema mutation-inputs --inputs --json`; absent → the column says so).
Prints the headline (counts + owner list). Usage: no arguments."""
import collections, importlib.util, json, os, re, subprocess, time

T = os.path.dirname(os.path.abspath(__file__))
ROOT = "/Users/ueli/Documents/semio"
G = os.path.join(T, "🗑️generated", "s5-agnostic")
LANES = ("/🎚️config/", "/👥️presence/", "/🫧️transient/", "/🪟️window/", "/📝️draft/", "/🧫️fixtures/", "/🧪️tests/")
WAVE_B = "2026-10-05 06:14"
LAW_V3 = "2026-10-05 16:53"
LAW_V8 = "2026-10-06 00:58"
ORDER = ["raster", "draw", "layout", "note", "space", "mathematical", "imperative", "writer", "vcs", "trinity", "remodel", "process", "wfc", "fem", "lowpoly", "shooting", "energy", "forms", "gis", "procedural", "playbook", "norm", "stdio", "block", "animate", "architect", "demonstrator", "sourcing", "dag", "flow", "cad"]


def table_module():
    spec = importlib.util.spec_from_file_location("s5_table", os.path.join(T, "🧪️s5-agnostic-table.py"))
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def artifact_of(path):
    match = re.match(r"✏️s/🔌️plugins/([^/]+)/🗿️artifacts/([^/]+)/", path)
    return match.groups() if match else None


def leaves():
    listed = subprocess.run(["git", "-c", "core.quotepath=false", "ls-files", "--", "✏️s/🔌️plugins"], cwd=ROOT, capture_output=True, text=True).stdout.split("\n")
    total, withdraw = collections.Counter(), collections.Counter()
    for path in listed:
        if not re.search(r"/🧬️mutations/[^/]+/🔣️\.json$", path) or any(lane in path for lane in LANES):
            continue
        key = artifact_of(path)
        if key is None:
            continue
        try:
            descriptor = json.load(open(os.path.join(ROOT, path), encoding="utf-8"))
        except (OSError, ValueError):
            continue
        if not isinstance(descriptor, dict) or "semanticKind" not in descriptor:
            continue
        total[key] += 1
        if descriptor.get("editable") is False:
            withdraw[key] += 1
    return total, withdraw


def inputs():
    path = os.path.join(G, "inputs.json")
    if not os.path.isfile(path):
        return None
    counts = collections.defaultdict(collections.Counter)
    for row in json.load(open(path, encoding="utf-8")):
        if any(lane in row["path"] for lane in LANES):
            continue
        key = artifact_of(row["path"])
        if key is not None:
            counts[key][row["verdict"]] += 1
    return counts


def gate_findings():
    """Per artifact: the inputs gate's findings on its DOCUMENT-lane leaves — (undeclared inputs, input-less editable leaves, catalogue staleness)."""
    path = os.path.join(G, "inputs-findings.json")
    if not os.path.isfile(path):
        return None
    counts = collections.defaultdict(collections.Counter)
    for entry in json.load(open(path, encoding="utf-8")).get("diagnostics", []):
        if any(lane in entry["path"] for lane in LANES):
            continue
        key = artifact_of(entry["path"])
        if key is None:
            continue
        code = entry["detail"].split(" ")[0]
        counts[key]["inputless" if code == "inputless" else "stale" if code in ("leafUncatalogued", "malformed") else "undeclared"] += 1
    return counts


def auto_class(kind, summary):
    """A provisional class for a row nobody reviewed yet (the unattended batch writes rows while no agent reads them)."""
    if kind == "COMPILE-RED":
        return "COMPILE-RED (auto: wave-B fallout → S5-CHANNEL)" if "description" in summary else "COMPILE-RED (auto)"
    if kind == "ABORT":
        return "FAIL (auto: the test process died — unreviewed)"
    for needle, cls in (
        ("no committed case exercises an input of control kind", "FAIL (auto: plugin — an input-control kind no committed case round-trips)"),
        ("exercises a history edit end to end", "FAIL (auto: plugin — no usable committed or shipped case)"),
        ("the seed gesture", "FAIL (auto: plugin — the child law's seed action faults)"),
        ("the app's own example route", "FAIL (auto: plugin — its example route faults)"),
        ("the example does not finish loading", "FAIL (auto: plugin — its example route faults)"),
        ("breaks the generic mechanism", "FAIL (auto: MECHANISM CANDIDATE — unreviewed)"),
        ("does not reload identically", "FAIL (auto: reload — unreviewed)"),
        ("does not survive save and load", "FAIL (auto: reload — unreviewed)"),
    ):
        if needle in summary:
            return cls
    return f"{kind} (auto: unreviewed)"


def cascade_of(summary, failed_text):
    """The conflict law's outcome as the matrix shows it."""
    if "the conflict law on" in failed_text:
        return "FAIL"
    line = re.search(r"\[history-edit-cascade\] ([^|]*)", summary)
    if not line:
        return ""
    text = line.group(1)
    if "no parent-lane leaf" in text:
        return "not driven (child lane)"
    if "no dependents" in text:
        return "no dependents"
    return "resolved" + (" (withdraw + edit)" if "editing its input" in text else " (withdraw)")


def first_location(summary):
    match = re.search(r"((?:/Users/ueli/Documents/semio/)?[^\s|:]+\.(?:rs|json|toml)):(\d+):(\d+)", summary)
    if not match:
        return ""
    return f"`{match.group(1).replace('/Users/ueli/Documents/semio/', '')}:{match.group(2)}`"


def main():
    table = table_module()
    census = json.load(open(os.path.join(G, "census.json"), encoding="utf-8"))
    results = table.tsv(os.path.join(G, "acceptance-results.tsv"), 1)
    classes = table.tsv(os.path.join(T, "🧪️s5-agnostic-classification.tsv"), 0)
    leaf_total, leaf_withdraw = leaves()
    input_counts = inputs()
    gates = gate_findings()
    fault_lines = collections.defaultdict(list)
    tally, compiled, owners = collections.Counter(), collections.Counter(), collections.defaultdict(list)
    payload_red, swept, swept_of, faults, owed = [], 0, 0, [], collections.defaultdict(list)
    lines = ["| Plugin | Artifact crate | Laws | Lib-test compiles | Acceptance law | G12 / inputs / reload / child | Conflict (cascade, law v8) | Leaves exercised (census) | Control kinds proven | Payload laws ok·fail | Leaves | Withdraw-only | Undeclared inputs (gate) | Input-less editable leaves (gate) | Bare inputs (inferred + refused of total) | Owner WP | First failing file:line — reason |", "|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|"]
    for row in census:
        crate, key = row["crate"], (row["plugin"], row["artifact"])
        owner = table.OWNERS.get(row["plugin"], "main (no S5 owner)")
        result, cls = results.get(crate), classes.get(crate)
        laws = location = reason = proven = kinds = payloads = cascade = ""
        if result is None:
            compiles, verdict = "not built by this WP", "NOT RUN"
            reason = cls[3] if cls else ""
        else:
            when, _, _, kind, g12, edit_inputs, reload, child, payload, summary = (result + [""] * 10)[:10]
            epoch = "" if when >= WAVE_B else " — PRE-channel-22 tree"
            if kind == "COMPILE-RED":
                compiles, verdict = f"NO ({when[11:16]})", (cls[1] if cls else auto_class(kind, summary))
            elif kind == "NOT-RUN":
                compiles, verdict = (f"yes ({when[11:16]}, build only)" if cls and cls[1].startswith("NOT RUN (built") else f"not built ({when[11:16]})"), "NOT RUN"
            else:
                compiles = f"yes ({when[11:16]}{epoch})"
                counted = re.search(r"(\d+) of (\d+) editable leaves exercised", summary)
                proven = f"{counted.group(1)} of {counted.group(2)}" if counted else ("" if when >= LAW_V3 else "base law only")
                if counted:
                    swept, swept_of = swept + int(counted.group(1)), swept_of + int(counted.group(2))
                    shipped = re.search(r"(\d+) swept only on a shipped document", summary)
                    proven += f" ({shipped.group(1)} on a shipped document)" if shipped and shipped.group(1) != "0" else ""
                listed = re.search(r"control kinds: ([^|]*)", summary)
                if listed:
                    marks = {"proven": "✓", "not": "—", "UNPROVEN": "✗"}
                    kinds = " ".join(f"{part.split()[0]} {marks.get(part.split()[1], '?')}" for part in listed.group(1).split(", ") if len(part.split()) > 1)
                payloads = payload.replace("/", "·")
                if payload.split("/")[-1] not in ("", "0"):
                    breach = re.search(r'"(op \d+ \([^"]*)"', summary)
                    payload_red.append((crate.replace("semio-s-artifact-", ""), owner, breach.group(1) if breach else "see the family log"))
                laws = f"{g12} / {edit_inputs} / {reload} / {child}"
                verdict = "PASS" + epoch if kind == "PASS" else (cls[1] if cls else auto_class(kind, summary))
                cascade = cascade_of(summary, summary) if when >= LAW_V8 else "not run (pre-v8)"
            if kind != "PASS":
                owner = (cls[2] if cls and cls[2] else owner)
                location = first_location(summary)
                reason = (cls[3] if cls else summary)[:260]
            elif payload_red and payload_red[-1][0] == crate.replace("semio-s-artifact-", ""):
                reason = "payload law (derive-emitted, not the acceptance law) FAILS: " + payload_red[-1][2]
        head = verdict.split(" (")[0]
        tally[verdict if verdict.startswith("FAIL") else head.replace(" — PRE-channel-22 tree", " (pre-channel-22 tree only)")] += 1
        compiled[compiles.split(" (")[0]] += 1
        if not verdict.startswith("PASS") and not verdict.startswith("NOT RUN") and result is not None:
            owners[owner].append(crate.replace("semio-s-artifact-", ""))
            faults.append(f"- **{owner}** — `{crate.replace('semio-s-artifact-', '')}` {verdict}: {(cls[3] if cls else reason)[:700]}")
            fault_lines[owner].append(f"- `{crate.replace('semio-s-artifact-', '')}` — {verdict} — {location or 'no file:line'} — {((cls[3] if cls else summary) or '').replace(chr(10), ' ')[:420]}")
        if cascade == "FAIL":
            fault_lines[owner].append(f"- `{crate.replace('semio-s-artifact-', '')}` — conflict law (cascade) FAIL (unreviewed) — {(re.search(r'the conflict law on [^|]*', summary) or [''])[0][:420]}")
        if not verdict.startswith("PASS"):
            owed[crate.split("-")[3]].append(crate)
        if input_counts is None:
            bare = "inputs gate not run"
        else:
            counts = input_counts.get(key, {})
            total = sum(counts.values())
            bare = f"{counts.get('inferred', 0)} + {counts.get('refused', 0)} of {total}" if total else "0 of 0 (no catalogued leaf)"
        gate = (gates or {}).get(key, {}) if gates is not None else None
        undeclared, inputless = ("gate not run", "gate not run") if gate is None else (str(gate.get("undeclared", 0)) + (f" (+{gate['stale']} catalogue-stale)" if gate.get("stale") else ""), str(gate.get("inputless", 0)))
        detail = " — ".join(part for part in (location, reason.replace("|", "¦").replace("\n", " ")) if part)
        lines.append(f"| {row['plugin']} | `{crate.replace('semio-s-artifact-', '')}` | {'+'.join(row['laws'])} | {compiles} | {verdict} | {laws} | {cascade} | {proven} | {kinds} | {payloads} | {leaf_total[key]} | {leaf_withdraw[key]} | {undeclared} | {inputless} | {bare} | {owner} | {detail} |")
    headline = [
        f"Acceptance law over {len(census)} artifact crates / {len({row['plugin'] for row in census})} plugins: " + ", ".join(f"{name} {count}" for name, count in sorted(tally.items())) + ".",
        "Lib-test compiles: " + ", ".join(f"{name} {count}" for name, count in sorted(compiled.items())) + ".",
        f"Document-lane leaves {sum(leaf_total.values())}, withdraw-only (`editable: false`) {sum(leaf_withdraw.values())}" + ("" if input_counts is None else f"; bare inputs {sum(c.get('inferred', 0) + c.get('refused', 0) for c in input_counts.values())} of {sum(sum(c.values()) for c in input_counts.values())} document-lane inputs") + ".",
        "Owners of the non-passing rows: " + ("; ".join(f"{owner}: {', '.join(names)}" for owner, names in sorted(owners.items())) or "none") + ".",
        f"Census over the crates that ran law v3: {swept} of {swept_of} editable leaves exercised end to end by a case (the rest are named with their skip reason in the family logs).",
        "Payload-law failures (derive-emitted `semio_payload_law_*`, not the acceptance law): " + ("; ".join(f"{crate} → {owner}: {breach}" for crate, owner, breach in payload_red) or "none") + ".",
    ]
    body = [
        "# 📓️ S5-AGNOSTIC matrix — is history editing artifact-agnostic?",
        "",
        f"Generated {time.strftime('%F %T')} by `🧪️s5-agnostic-matrix.py` (S5-AGNOSTIC; evidence and method: `📓️s2-agnostic-report.md` § Session 5).",
        "One row per plugin artifact crate that wires `history_edit_acceptance_law!` (G12), `composed_reload_law!` (reload) or",
        "`composed_child_history_law!` (child). A verdict is a run of this WP on the tree of the stated time; nothing is inferred from",
        "other reports. Rows before 06:14 ran on the pre-wave-B (channel 21) tree and say so. \"Bare inputs\" = the inputs gate's",
        "`inferred` + `refused` rows among the artifact's document-lane leaf inputs (every depth); the gate reads catalogued leaves only.",
        "",
        "## Headline",
        "",
        *[f"- {line}" for line in headline],
        "",
        "## What has actually run (honest state)",
        "",
        "- 10 of 96 artifact crates have run the acceptance laws on the live tree (build B2, channel 23, 2026-10-05 17:25–19:13), each in a private cargo folder, one cargo at a time: 7 PASS (puzzle 2d, 3d, 5d; stdio pdf, gltf; sequence; wires), 3 FAIL, all classified PLUGIN (dag, flow, cad). No failure was a framework mechanism failure. 4 more crates (raster, drawing, layout, note) BUILT their lib-test but their laws were not executed. 82 crates were never built by this WP.",
        "- A PASS proves, through the generic verbs only: withdraw → accept → restore leaves zero trace; the preview right after Begin equals the document as of the edited mutation (downstream not applied); a schema-valid input change → accept → Report replay → finalize as overwrite AND as a new alternative, each head a fresh fold of the edited log, both reloads alike, row label en/de; one passing scenario per input-control kind the editors offer; no committed case that breaks the mechanism. For a composed plugin (sequence, wires, flow's child lane) the same session runs on the member store (`store` argument) and the seeded, overwritten and alternative documents reload identically.",
        "- A PASS does NOT prove every leaf: the census column says how many editable leaves a case exercised end to end (154 of 294 so far); the others are named with their skip reason in `🗑️generated/s5-agnostic/family-*.test.txt` (fixture content: a seed that does not apply on the shipped document, an index input that blocks the replay, no changeable input). Preview-as-of and withdraw/restore are not asserted on the child lane.",
        "- Every `NOT RUN` is owed, not presumed green.",
        "",
        "## How to run",
        "",
        "One family per call. The runner builds in the private folder `🗑️generated/s5-agnostic/target` (both `CARGO_TARGET_DIR` and `CARGO_BUILD_BUILD_DIR`, `CARGO_BUILD_JOBS=4`, `RUST_MIN_STACK=268435456`), starts nothing while `🗑️generated/coord/activation.flag` exists or below 12 GiB free (exit 5 — owed), and keeps the folder between families until it exceeds 12 GiB. Re-issue the same call when the 10-minute cap of a Bash call moves it to the background (it waits, reports, and is the disk watchdog). Measured: 15 min cold for the first family (3 GiB), 5–8 min for a following one; the folder reaches 12 GiB after about 15 crates — set `S5_DROP_DIR=1` on the last family. A family whose build ended with less than 12 GiB free is NOT executed: free disk and re-issue the call (the build is then a no-op).",
        "",
        "## Faults by owner (every row that ran and did not pass)",
        "",
        *(faults or ["- none"]),
        *[f"- **{owner}** — `{crate}` payload law (derive-emitted, not the acceptance law): {breach}" for crate, owner, breach in payload_red],
        "",
        "## Remaining families — exact commands (every crate without a PASS; one call per line, in this order of value)",
        "",
        "```",
        "cd /Users/ueli/Documents/semio",
        "T=.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️30/NON-DESTRUCTIVE-HISTORY-EDITING",
        *[f"zsh $T/🧪️s5-agnostic-run-family.sh {plugin}{'' if len(chunks) == 1 else '-' + str(index + 1)} {' '.join(chunk)}" for plugin, chunks in ((plugin, [crates[start:start + 9] for start in range(0, len(crates), 9)]) for plugin, crates in sorted(owed.items(), key=lambda item: ORDER.index(item[0]) if item[0] in ORDER else len(ORDER))) for index, chunk in enumerate(chunks)],
        "python3 $T/🧪️s5-agnostic-matrix.py",
        "```",
        "",
        "## Matrix",
        "",
        *lines,
        "",
    ]
    open(os.path.join(T, "📓️s5-agnostic-matrix.md"), "w", encoding="utf-8").write("\n".join(body))
    for crate, owner, breach in payload_red:
        fault_lines[owner].append(f"- `{crate}` — payload law (derive-emitted, not the acceptance law) — plugin — {breach}")
    faults_body = [
        "# 📓️ S5 — every editor: faults by owner",
        "",
        f"Generated {time.strftime('%F %T')} by `🧪️s5-agnostic-matrix.py` (re-run by the unattended acceptance batch after every family). One line per crate that ran and did not pass, grouped by owner: class — first failing `file:line` — first message. A class marked `auto` was assigned by pattern while no agent was reading; `MECHANISM CANDIDATE` rows are the ones to review first. Classes: mechanism (the framework's time-travel / store path), plugin (a leaf, a fixture, an action or an example of that plugin), harness (the law's own driver — S5-AGNOSTIC), compile-red (the lib-test does not build), peer-in-flight (a peer's uncommitted edit).",
        "",
        *([line for owner in sorted(fault_lines) for line in [f"## {owner}", "", *fault_lines[owner], ""]] or ["No crate that ran has failed."]),
    ]
    open(os.path.join(T, "📓️s5-every-editor-faults.md"), "w", encoding="utf-8").write("\n".join(faults_body) + "\n")
    print("\n".join(headline))


if __name__ == "__main__":
    main()
