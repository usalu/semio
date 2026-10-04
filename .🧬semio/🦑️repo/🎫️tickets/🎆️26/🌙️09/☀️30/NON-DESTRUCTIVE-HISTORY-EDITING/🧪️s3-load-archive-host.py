"""🗃️ Independent oracle for the document archive load host corpus (S3-LOAD).

Validates `📡️spr/🧵️channel/🧫️fixtures/🧫️document-archive-load-host/🔣️.json` against its schema with the third-party
`jsonschema` library, then replays every case through a Python model of the host protocol written from
`📓️api-stepped-document-load.md` §2 alone (not from the Rust driver) and checks the scripted commands, operations and
outcomes. A case the guest admitted itself (`admittedByGuest`, the media import of a whole document) starts admitted under
`firstSequence`. `--negative` flips one scripted command and must exit 1.
"""

import json
import pathlib
import sys

import jsonschema

ROOT = pathlib.Path("/Users/ueli/Documents/semio")
CHANNEL = ROOT / "🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧵️channel"
TERMINAL = {"ready", "cancelled", "fault"}


class Host:
    def __init__(self, admitted=None):
        self.operation = admitted
        self.cancel_wanted = False
        self.cancel_sent = False
        self.terminal = None
        self.outcome = None
        self.last = None

    def cancel(self):
        self.cancel_wanted = True
        if self.operation is None and self.outcome is None:
            self.outcome = {"kind": "cancelled"}

    def send(self, seq):
        if self.outcome is not None:
            return None
        if self.operation is None:
            self.operation = seq
            kind = "loadDocumentArchive"
        elif self.terminal is not None:
            kind = "acknowledgeDocumentArchiveLoad"
        elif self.cancel_wanted and not self.cancel_sent:
            kind = "cancelDocumentArchiveLoad"
        else:
            kind = "pollDocumentArchiveLoad"
        self.last = kind
        return kind

    def take(self, answer, foreign):
        kind = self.last
        if answer["kind"] == "error":
            if kind == "cancelDocumentArchiveLoad":
                self.cancel_sent = True
                return None
            if kind == "acknowledgeDocumentArchiveLoad" and self.terminal and self.terminal["state"] == "cancelled":
                self.terminal = None
                return None
            return {"kind": "refused", "fault": answer["fault"]}
        if answer["kind"] == "done" and kind in ("loadDocumentArchive", "cancelDocumentArchiveLoad", "acknowledgeDocumentArchiveLoad"):
            if kind == "cancelDocumentArchiveLoad":
                self.cancel_sent = True
            if kind == "acknowledgeDocumentArchiveLoad":
                state = self.terminal["state"]
                self.outcome = {"kind": "fault", "fault": self.terminal.get("fault", "")} if state == "fault" else {"kind": state}
            return None
        if answer["kind"] == "status" and kind == "pollDocumentArchiveLoad" and not foreign:
            if answer["state"] in TERMINAL:
                self.terminal = answer
            return None
        return {"kind": "unanswered"}


def replay(law, flip):
    failures = []
    for number, case in enumerate(law["cases"]):
        admitted = case.get("admittedByGuest", False)
        host = Host(law["firstSequence"] if admitted else None)
        seq = law["firstSequence"] + (1 if admitted else 0)
        cancel = case["cancel"] or {}
        stopped = None
        for index, exchange in enumerate(case["exchanges"]):
            if cancel.get("beforeStep") == index:
                host.cancel()
            sent = host.send(seq)
            expected = exchange["sends"]
            if flip and number == 0 and index == 1:
                expected = "cancelDocumentArchiveLoad"
            if sent != expected:
                failures.append(f"{case['name']}: step {index} sends {sent}, scripted {expected}")
                break
            if host.operation != law["firstSequence"]:
                failures.append(f"{case['name']}: operation {host.operation}")
            if cancel.get("whileStepInFlight") == index:
                host.cancel()
            stopped = host.take(exchange["answer"], exchange["answer"]["kind"] == "foreignStatus")
            seq += 1
        if cancel.get("beforeStep") == len(case["exchanges"]):
            host.cancel()
        outcome = stopped or host.outcome
        if outcome is None or (stopped is None and host.send(seq) is not None):
            failures.append(f"{case['name']}: the host still sends after the script")
        elif outcome != case["outcome"]:
            failures.append(f"{case['name']}: outcome {outcome}, scripted {case['outcome']}")
        statuses = [(e["answer"]["completed"], e["answer"]["total"]) for e in case["exchanges"] if e["answer"]["kind"] == "status"]
        if any(c > t for c, t in statuses) or any(a[0] > b[0] or a[1] != b[1] for a, b in zip(statuses, statuses[1:])):
            failures.append(f"{case['name']}: progress {statuses} is not monotonic and bounded")
    return failures


def main():
    law = json.loads((CHANNEL / "🧫️fixtures/🧫️document-archive-load-host/🔣️.json").read_text())
    schema = json.loads((CHANNEL / "🧬️schema/🔣️document-archive-load-host/🔣️.json").read_text())
    jsonschema.Draft202012Validator.check_schema(schema)
    jsonschema.Draft202012Validator(schema).validate(law)
    failures = replay(law, "--negative" in sys.argv)
    for failure in failures:
        print(f"FAIL {failure}")
    print(f"{len(law['cases'])} cases, {len(failures)} failures")
    sys.exit(1 if failures else 0)


if __name__ == "__main__":
    main()
