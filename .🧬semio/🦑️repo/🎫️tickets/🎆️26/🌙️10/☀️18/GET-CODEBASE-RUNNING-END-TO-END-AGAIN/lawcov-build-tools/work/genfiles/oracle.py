"""🗂️ Independent Python model of the generation files recheck window (standard library only): the operation numbered a multiple of the window triggers one recheck of the store before it is admitted; a refused recheck refuses exactly that operation; with concurrent operations at most one recheck runs and their number is bounded by the operations divided by the window. Requests arrive as JSON on stdin."""
import json, math, sys


def sequence(row):
    refused, rechecks = [], 0
    for number, _ in enumerate(row["operations"], start=1):
        if number % row["window"] == 0:
            rechecks += 1
            if rechecks in row["failRechecks"]:
                refused.append(number - 1)
    return {"rechecks": rechecks, "refused": refused}


def concurrent(row):
    total = row["workers"] * row["each"]
    return {"maximumRechecks": math.ceil(total / row["window"]), "peak": 1}


request = json.load(sys.stdin)
print(json.dumps({"sequences": [sequence(row) for row in request["sequences"]], "concurrent": [concurrent(row) for row in request["concurrent"]]}))
