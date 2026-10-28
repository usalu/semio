"""🐍️ Independent oracle of the Nx failure tail: replays a transcript (chunks per channel, with optional repeat) the way the contract states it, without sharing a line with the implementation, and prints per transcript the text a reader gets: the bounded evidence lines (every complete line, decided when it arrives per channel, that names a refusal class by one of the declared messages or by the exhausted text, distinct, newest 64, cut to 2048) that the tail no longer holds, then the last `maximum` characters. usage: 🐍️.py <refusals.json> <churn schema.json> <tail schema.json> <vectors.json>"""
import json
import re
import sys


def main():
    table = json.load(open(sys.argv[1], encoding="utf-8"))["definitions"]["Table"]["const"]
    churn = json.load(open(sys.argv[2], encoding="utf-8"))["definitions"]["Protocol"]["const"]
    policy = json.load(open(sys.argv[3], encoding="utf-8"))["definitions"]["Policy"]["const"]
    vectors = json.load(open(sys.argv[4], encoding="utf-8"))["transcripts"]
    messages = [message["text"] for row in table["classes"] for message in row["messages"] if message["text"]]
    exhausted = [[part for part in re.split(r"\{[a-z]+\}", language["exhausted"])] for language in churn["texts"].values()]

    def names_class(line):
        if any(message in line for message in messages):
            return True
        return any(all(part in line for part in parts if part) and re.search(".+?".join(re.escape(part) for part in parts), line) for parts in exhausted)

    out = {}
    for vector in vectors:
        evidence, pending, kept = [], {"stdout": "", "stderr": ""}, ""

        def keep(line):
            if not names_class(line):
                return
            line = line[:policy["maximumEvidenceLineCharacters"]]
            if line in evidence:
                return
            if len(evidence) >= policy["maximumEvidenceLines"]:
                evidence.pop(0)
            evidence.append(line)

        for chunk in vector["chunks"]:
            text = chunk["text"] * chunk.get("repeat", 1)
            channel = chunk.get("channel", "stdout")
            kept = (kept + text)[-vector["maximum"]:]
            *done, pending[channel] = (pending[channel] + text).split("\n")
            for line in done:
                keep(line)
            if len(pending[channel]) > policy["maximumPendingLineCharacters"]:
                keep(pending[channel])
                pending[channel] = ""
        unfinished = [line[:policy["maximumEvidenceLineCharacters"]] for line in (pending["stdout"], pending["stderr"]) if names_class(line)]
        missing = [line for line in dict.fromkeys(evidence + [item for item in unfinished if item not in evidence]) if line not in kept]
        out[vector["id"]] = "\n".join(missing) + "\n" + kept if missing else kept
    print(json.dumps(out, ensure_ascii=False, sort_keys=True))


main()
