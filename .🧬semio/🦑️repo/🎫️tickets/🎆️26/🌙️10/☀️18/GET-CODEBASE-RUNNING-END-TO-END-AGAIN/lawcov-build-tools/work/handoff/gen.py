"""📝 Authoring aid: writes the daemon handoff corpus (corpus.json); wires are given as text or as a generated filler of a given byte size."""
import json, os

W = os.path.dirname(os.path.abspath(__file__)) + "/"
BOUND = "Bounded original daemon handoff required"
CLOSED = "Closed original daemon handoff required"
BIRTH = "Closed original daemon birth descriptor required"


def row(name, wire=None, filler=None, error="", kind=None):
    return {"name": name, "wire": wire, "filler": filler, "error": error, "kind": kind}


c = {"version": 1, "envName": "SEMIO_NX_DAEMON_SERVICE_BIRTH", "maximumBytes": 1048576}
c["wires"] = [
    row("a missing wire", None, None, BOUND, "bound"),
    row("an empty wire is not JSON", "", None, "SyntaxError", "syntax"),
    row("text that is not JSON", "not json", None, "SyntaxError", "syntax"),
    row("a truncated document", '{"birth":', None, "SyntaxError", "syntax"),
    row("null is not a record", "null", None, CLOSED, "closed"),
    row("an array is not a record", "[]", None, CLOSED, "closed"),
    row("a string is not a record", '"x"', None, CLOSED, "closed"),
    row("a number is not a record", "1", None, CLOSED, "closed"),
    row("an empty record lacks both parts", "{}", None, CLOSED, "closed"),
    row("a record with only a birth", '{"birth":{}}', None, CLOSED, "closed"),
    row("a record with only a witness", '{"witness":{}}', None, CLOSED, "closed"),
    row("a record with a third key", '{"birth":{},"witness":{},"extra":1}', None, CLOSED, "closed"),
    row("a record with other keys", '{"a":1,"b":2}', None, CLOSED, "closed"),
    row("an empty birth is not a descriptor", '{"birth":{},"witness":{}}', None, BIRTH, "birth"),
    row("the key order does not matter", '{"witness":{},"birth":{}}', None, BIRTH, "birth"),
    row("a birth that is null", '{"birth":null,"witness":{}}', None, BIRTH, "birth"),
    row("exactly the byte bound is read as a document", None, {"bytes": 1048576, "wrapper": "{\"birth\":{},\"witness\":{},\"x\":\"%s\"}"}, CLOSED, "closed"),
    row("one byte more than the bound is refused unread", None, {"bytes": 1048577, "wrapper": "{\"birth\":{},\"witness\":{},\"x\":\"%s\"}"}, BOUND, "bound"),
    row("bytes are counted not characters", None, {"bytes": 1048577, "wrapper": "\"%s\"", "multibyte": True}, BOUND, "bound"),
]
json.dump(c, open(W + "corpus.json", "w", encoding="utf8"), ensure_ascii=False, indent=1)
