"""📝 Authoring aid: writes the custody pool scenario corpus (corpus.json)."""
import json, os

W = os.path.dirname(os.path.abspath(__file__)) + "/"
w = lambda id: {"op": "with", "id": id}
r = lambda id: {"op": "release", "id": id}
f = lambda id: {"op": "fail", "id": id}
a = lambda ms: {"op": "advance", "ms": ms}
close = {"op": "close"}
fail_open = {"op": "failNextOpen"}


def s(name, steps, idle=100, maximum=1000, final=None):
    return {"name": name, "idleMilliseconds": idle, "maximumLifetimeMilliseconds": maximum, "steps": steps, "final": final}


c = {"version": 1}
c["scenarios"] = [
    s("a warm request reuses the session", [w("a"), r("a"), w("b"), r("b")], final={"opens": 1, "closes": 0, "live": True}),
    s("an idle session is closed after the idle time", [w("a"), r("a"), a(99), a(1)], final={"opens": 1, "closes": 1, "live": False}),
    s("a request just before the idle time keeps the session", [w("a"), r("a"), a(99), w("b"), r("b"), a(99)], final={"opens": 1, "closes": 0, "live": True}),
    s("concurrent requests share one session", [w("a"), w("b"), w("c"), r("a"), r("b"), r("c")], final={"opens": 1, "closes": 0, "live": True}),
    s("the idle timer waits for the last request", [w("a"), w("b"), r("a"), a(500), r("b"), a(100)], final={"opens": 1, "closes": 1, "live": False}),
    s("an old idle session is replaced by the next request", [w("a"), r("a"), a(99), w("b"), r("b"), a(99), w("c"), r("c")], idle=100, maximum=150, final={"opens": 2, "closes": 1, "live": True}),
    s("a busy session is not replaced however old it is", [w("a"), a(5000), w("b"), r("a"), r("b")], idle=100, maximum=1000, final={"opens": 1, "closes": 0, "live": True}),
    s("a failed request closes the session and the next one reopens", [w("a"), f("a"), w("b"), r("b")], final={"opens": 2, "closes": 1, "live": True}),
    s("a failure among concurrent requests lets the others finish on the old session", [w("a"), w("b"), f("a"), r("b"), w("c"), r("c")], final={"opens": 2, "closes": 1, "live": True}),
    s("an open that fails leaves nothing behind and the next request reopens", [fail_open, w("a"), w("b"), r("b")], final={"opens": 2, "closes": 0, "live": True}),
    s("closing the pool closes its session and a later request reopens", [w("a"), r("a"), close, w("b"), r("b")], final={"opens": 2, "closes": 1, "live": True}),
    s("closing an idle pool twice closes once", [w("a"), r("a"), close, close], final={"opens": 1, "closes": 1, "live": False}),
    s("closing an empty pool does nothing", [close], final={"opens": 0, "closes": 0, "live": False}),
    s("the idle timer is cancelled by a close", [w("a"), r("a"), close, a(1000)], final={"opens": 1, "closes": 1, "live": False}),
    s("alternating requests and idle periods open each time", [w("a"), r("a"), a(100), w("b"), r("b"), a(100), w("c"), r("c"), a(100)], final={"opens": 3, "closes": 3, "live": False}),
    s("two concurrent failures close one session once", [w("a"), w("b"), f("a"), f("b")], final={"opens": 1, "closes": 1, "live": False}),
    s("three concurrent failures close one session once and the next request reopens", [w("a"), w("b"), w("c"), f("a"), f("b"), f("c"), w("d"), r("d")], final={"opens": 2, "closes": 1, "live": True}),
    s("a failure after a close does not close the session again", [w("a"), close, f("a")], final={"opens": 1, "closes": 1, "live": False}),
    s("a failed request after an idle close reopens once", [w("a"), r("a"), a(100), w("b"), f("b"), w("c"), r("c")], final={"opens": 3, "closes": 2, "live": True}),
]
c["invalid"] = [{"idle": 0, "maximum": 10}, {"idle": -1, "maximum": 10}, {"idle": 10, "maximum": 9}, {"idle": "x", "maximum": 5}, {"idle": 5, "maximum": "x"}]
c["valid"] = [{"idle": 1, "maximum": 1}, {"idle": 0.5, "maximum": 0.5}, {"idle": 1, "maximum": 1000000}]

json.dump(c, open(W + "corpus.json", "w", encoding="utf8"), ensure_ascii=False, indent=1)
