"""🧫️ Builds the fixture of the acceptance laws from the recorded gate logs of 10-26/10-27 (g34-g46): every line a tool reads is kept verbatim (consecutive identical lines run-length encoded, the chatter of the compilers in between elided with a count), with the numbers the gate recorded for it and the verdicts the budgets must give, written down by hand here."""
import json, os, re, sys
HERE = os.path.dirname(os.path.abspath(__file__))
RUN = os.path.join(HERE, "..", "gate-g-tools", "run")
POLL = {"kind": "poll", "grainMs": 5000}


KEEP = re.compile(r"^(\$ repo |INFO: |graph: |\[bazel\] |\[capacity\] |\[dashboard\] |\[repository:command\] |\[script\] |\[\d+ / \d+\]|Target |Analyz|Loading|Computing main repo|Executed |ERROR|FAIL)")


def lines(label):
    out, elided = [], 0

    def flush():
        nonlocal elided
        if elided:
            out.append("... %d lines of tool output elided (no observer reads them)" % elided)
            elided = 0

    with open(os.path.join(RUN, label + ".log"), encoding="utf8", errors="replace") as handle:
        for raw in handle:
            line = raw.rstrip("\n").rstrip("\r")[:400]
            if not KEEP.search(line):
                elided += 1
                continue
            flush()
            if out and isinstance(out[-1], list) and out[-1][0] == line:
                out[-1][1] += 1
            elif out and out[-1] == line:
                out[-1] = [line, 2]
            else:
                out.append(line)
    flush()
    return out


def case(identifier, title_en, title_de, label, scenario, role, expect_executed, expect_served, rows):
    record = json.load(open(os.path.join(RUN, label + ".json"), encoding="utf8"))
    return {"id": identifier, "title": {"en": title_en, "de": title_de}, "source": "gate-g-tools/run/%s.{log,json} (target %s)" % (label, record["target"]),
            "scenario": scenario, "role": role, "exit": record["exit"], "wallMs": round(record["wall_s"] * 1000), "clock": POLL,
            "peakResidentMb": record["peak_tree_mb"], "log": lines(label), "expect": {"executed": expect_executed, "served": expect_served, "rows": rows}}


def build():
    return {"version": 1, "about": "Recorded official runs of the gate (poll clock of 5 s, resident memory only): hot no-ops, one-line edits with their reverts and a run that re-executes, each with the verdicts the declared budgets must give. A breach and a pass of every product family.", "cases": [
        case("course-hot-pass", "Hot course no-op that holds its budgets (g41)", "Heißer Kurs-Leerlauf, der seine Budgets hält (g41)", "g41-hot2", "course-noop", "hot", 0, True,
             {"course-noop-wall": "pass", "course-noop-footprint": "pass", "course-noop-executed": "pass"}),
        case("dashboard-hot-pass", "Hot dashboard no-op: the freshness worker is the one allowed action (g41)", "Heißer Dashboard-Leerlauf: der Frische-Worker ist die eine erlaubte Aktion (g41)", "g41-dash3", "dashboard-noop", "hot", 1, True,
             {"dashboard-noop-wall": "pass", "dashboard-noop-footprint": "pass", "dashboard-noop-executed": "pass"}),
        case("print-hot-pass", "Hot print no-op served by the action cache (g41)", "Heißer Druck-Leerlauf aus dem Aktions-Cache (g41)", "g41-print3", "print-noop", "hot", 0, True,
             {"print-noop-wall": "pass", "print-noop-footprint": "pass", "print-noop-executed": "pass"}),
        case("course-hot-breach", "Hot course no-op of 138 s with a lost journal (g43): the time breaks the budget, the resident set proves nothing", "Heißer Kurs-Leerlauf von 138 s mit verlorenem Journal (g43): die Zeit verletzt das Budget, der resident Speicher beweist nichts", "g43-hot3", "course-noop", "hot", 0, False,
             {"course-noop-wall": "fail", "course-noop-footprint": "unproven", "course-noop-executed": "pass"}),
        case("course-hot-grain", "Hot course no-op of 10.1 s on a 5 s poll clock (g42): within one grain of the budget", "Heißer Kurs-Leerlauf von 10,1 s auf einer 5-s-Abfrage (g42): innerhalb einer Stufe des Budgets", "g42-hot2", "course-noop", "hot", 0, True,
             {"course-noop-wall": "unproven", "course-noop-footprint": "pass", "course-noop-executed": "pass"}),
        case("course-edit-pass", "Course one-line edit in 50.5 s (g42c)", "Einzeilige Kurs-Änderung in 50,5 s (g42c)", "g42c-edit", "course-edit", "edit", 1, False,
             {"course-edit-wall": "pass"}),
        case("course-revert-pass", "Byte-exact revert of the course edit in 40.3 s, two Bazel invocations (g42c)", "Byteidentische Rücknahme der Kurs-Änderung in 40,3 s, zwei Bazel-Aufrufe (g42c)", "g42c-revert", "course-edit", "revert", 1, True,
             {"course-revert-wall": "pass"}),
        case("course-edit-breach", "Course one-line edit of 133 s under load (g43c)", "Einzeilige Kurs-Änderung von 133 s unter Last (g43c)", "g43c-edit", "course-edit", "edit", 1, True,
             {"course-edit-wall": "fail"}),
        case("print-edit-pass", "Print Rust one-line edit in 105.9 s (g36q)", "Einzeilige Rust-Änderung des Drucks in 105,9 s (g36q)", "g36q-edit", "print-edit", "edit", 2, False,
             {"print-edit-wall": "pass"}),
        case("print-edit-breach", "Print Rust one-line edit of 161 s (g36p)", "Einzeilige Rust-Änderung des Drucks von 161 s (g36p)", "g36p-edit", "print-edit", "edit", 2, False,
             {"print-edit-wall": "fail"}),
        case("peer-quiet-pass", "Peer-edit run that executes nothing: a hot run read as the run the edit lands in (g41)", "Lauf mit fremder Änderung, der nichts ausführt: ein heißer Lauf als der Lauf gelesen, in dem die Änderung eintrifft (g41)", "g41-hot2", "course-peer-unrelated", "peer", 0, True,
             {"course-peer-unrelated-peer-executed": "pass"}),
        case("peer-reexecuting-breach", "Peer-edit run that re-executes a worker action: an edit run read as the run the peer edit lands in (g42c)", "Lauf mit fremder Änderung, der eine Worker-Aktion erneut ausführt: ein Änderungslauf als der Lauf gelesen, in dem die fremde Änderung eintrifft (g42c)", "g42c-edit", "course-peer-unrelated", "peer", 1, False,
             {"course-peer-unrelated-peer-executed": "fail"}),
    ]}


if __name__ == "__main__":
    out = sys.argv[1]
    data = build()
    with open(out, "w", encoding="utf8") as handle:
        json.dump(data, handle, ensure_ascii=False, indent=1)
        handle.write("\n")
    print(out, os.path.getsize(out), "bytes", len(data["cases"]), "cases")
