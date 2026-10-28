"""[hold-g] writes the closed corpus of the claim registry (inputs only, plus hand-written expectations for the named laws)."""
import json, random, sys
IMPL_A = "a" * 64
IMPL_B = "b" * 64
rng = random.Random(1026)
names = ["@semio-tech/p%02d" % i for i in range(40)] + ["@a/ü", "@a/z", "@a/😀", "@a/￮"]
def claim(token, owners, state, live=True, base=7, impl=IMPL_A):
    return {"token": token, "base": base, "implementation": impl, "owners": sorted(set(owners)), "state": state, "live": live}
chunks = []
def case(id, wanted, claims, base=7, impl=IMPL_A, chunk=4, tail=1, expected=None):
    row = {"id": id, "wanted": wanted, "claims": claims, "base": base, "implementation": impl, "chunk": chunk, "tail": tail}
    if expected: row["expected"] = expected
    chunks.append(row)
w = ["a", "b", "c", "d", "e", "f", "g", "h"]
case("nothing claimed takes the first chunk", w, [], expected={"open": w, "take": w[:4], "running": [], "prepared": 0, "covered": 0})
case("a tail of at most tail owners joins the chunk", w[:5], [], expected={"open": w[:5], "take": w[:5], "running": [], "prepared": 0, "covered": 0})
case("one past the tail splits", w[:6], [], expected={"open": w[:6], "take": w[:4], "running": [], "prepared": 0, "covered": 0})
case("a live running claim covers its owners and is waited for", w, [claim("0" * 24, ["a", "b"], "running")], expected={"open": w[2:], "take": w[2:6], "running": ["0" * 24], "prepared": 0, "covered": 2})
case("a dead running claim covers nothing", w, [claim("0" * 24, ["a", "b"], "running", live=False)], expected={"open": w, "take": w[:4], "running": [], "prepared": 0, "covered": 0})
case("done claims cover and count as prepared", w, [claim("1" * 24, ["a", "b", "c"], "done")], expected={"open": w[3:], "take": w[3:], "running": [], "prepared": 3, "covered": 3})
case("failed claims cover but are not prepared", w, [claim("2" * 24, ["a", "b"], "failed")], expected={"open": w[2:], "take": w[2:6], "running": [], "prepared": 0, "covered": 2})
case("claims of another plan are ignored", w, [claim("3" * 24, w, "done", base=6), claim("4" * 24, w, "running", base=8)], expected={"open": w, "take": w[:4], "running": [], "prepared": 0, "covered": 0})
case("claims of another implementation are ignored", w, [claim("5" * 24, w, "done", impl=IMPL_B)], expected={"open": w, "take": w[:4], "running": [], "prepared": 0, "covered": 0})
case("everything done leaves nothing open and nothing to wait for", w, [claim("6" * 24, w, "done")], expected={"open": [], "take": [], "running": [], "prepared": 8, "covered": 8})
case("everything claimed by a live peer is waited for", w, [claim("7" * 24, w[:4], "running"), claim("8" * 24, w[4:], "running")], expected={"open": [], "take": [], "running": ["7" * 24, "8" * 24], "prepared": 0, "covered": 8})
case("a live peer on unrelated owners is not waited for", ["a", "b"], [claim("9" * 24, ["x", "y"], "running")], expected={"open": ["a", "b"], "take": ["a", "b"], "running": [], "prepared": 0, "covered": 0})
case("duplicate wanted names are one owner", ["b", "a", "b", "a"], [], expected={"open": ["a", "b"], "take": ["a", "b"], "running": [], "prepared": 0, "covered": 0})
case("code point order, not UTF-16 order", ["@a/￮", "@a/😀", "@a/z"], [], chunk=1, tail=0, expected={"open": ["@a/z", "@a/￮", "@a/😀"], "take": ["@a/z"], "running": [], "prepared": 0, "covered": 0})
case("empty wanted", [], [claim("a" * 24, ["a"], "running")], expected={"open": [], "take": [], "running": [], "prepared": 0, "covered": 0})
case("chunk of one without tail", w, [], chunk=1, tail=0, expected={"open": w, "take": ["a"], "running": [], "prepared": 0, "covered": 0})
for n in range(48):
    wanted = rng.sample(names, rng.randint(0, 30))
    claims = []
    for k in range(rng.randint(0, 6)):
        owners = rng.sample(names, rng.randint(0, 12))
        claims.append(claim("%024x" % rng.getrandbits(96), owners, rng.choice(["running", "running", "done", "failed"]), live=rng.random() < 0.7, base=rng.choice([6, 7, 7, 7, 8]), impl=rng.choice([IMPL_A, IMPL_A, IMPL_A, IMPL_B])))
    case("random %02d" % n, wanted, claims, chunk=rng.choice([1, 2, 3, 5, 8, 128]), tail=rng.choice([0, 1, 3, 32]))
slots = []
def slot(id, mb, env=None): slots.append({"id": id, "memoryMegabytes": mb, "environment": env or {}})
for gb in (1, 4, 8, 12, 16, 24, 32, 64, 128, 1024): slot("%d GB" % gb, gb * 1024)
slot("forced 3", 16384, {"SEMIO_GRAPH_COMPUTERS": "3"})
slot("forced above the maximum is ignored", 16384, {"SEMIO_GRAPH_COMPUTERS": "9"})
slot("forced zero is ignored", 16384, {"SEMIO_GRAPH_COMPUTERS": "0"})
slot("forced text is ignored", 16384, {"SEMIO_GRAPH_COMPUTERS": "many"})
slot("forced fraction is ignored", 16384, {"SEMIO_GRAPH_COMPUTERS": "2.5"})
slot("zero memory still gets one slot", 0)
refusals = []
ok = {"version": 1, "token": "0" * 24, "base": 3, "implementation": IMPL_A, "owners": ["a", "b"], "state": "running", "startedMs": 5}
def bad(id, **changes):
    c = dict(ok); c.update(changes)
    for k in [k for k, v in c.items() if v is ...]: del c[k]
    refusals.append({"id": id, "claim": c})
bad("unknown key", extra=1)
bad("missing state", state=...)
bad("wrong version", version=2)
bad("token too short", token="abc")
bad("token upper case", token="A" * 24)
bad("base below minus one", base=-2)
bad("base text", base="3")
bad("implementation short", implementation="a" * 63)
bad("state unknown", state="abandoned")
bad("owners not sorted", owners=["b", "a"])
bad("owners duplicated", owners=["a", "a"])
bad("owner empty", owners=[""])
bad("owners not a list", owners="a")
bad("negative start", startedMs=-1)
bad("end not integer", endedMs=1.5)
json.dump({"version": 1, "chunks": chunks, "slots": slots, "refusals": refusals}, sys.stdout, ensure_ascii=False, indent=1)
