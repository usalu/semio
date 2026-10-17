import sys, xml.etree.ElementTree as ET, collections
def msgs(p):
    out = collections.Counter()
    root = ET.parse(p).getroot()
    for tc in root.iter("testcase"):
        f = tc.find("failure")
        if f is None: f = tc.find("error")
        if f is not None:
            m = (f.get("message") or (f.text or "")).strip().split("\n")[0][:160]
            out[m] += 1
    return out
a, b = msgs(sys.argv[1]), msgs(sys.argv[2])
print("before:", dict(a)); print("after: ", dict(b)); print("SAME" if a == b else "DIFFERENT")
