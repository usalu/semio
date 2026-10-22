"""📊️ Independent Python oracle of the build report name rule (standard library only): a name is admitted when it matches the authored pattern in full. Requests arrive as JSON on stdin."""
import json, re, sys

PATTERN = re.compile(r"[A-Za-z0-9][A-Za-z0-9._-]{0,63}\.json")


def admitted(name, limit):
    return isinstance(name, str) and len(name) <= limit and PATTERN.fullmatch(name) is not None


request = json.load(sys.stdin)
print(json.dumps([admitted(name, request["limit"]) for name in request["names"]]))
