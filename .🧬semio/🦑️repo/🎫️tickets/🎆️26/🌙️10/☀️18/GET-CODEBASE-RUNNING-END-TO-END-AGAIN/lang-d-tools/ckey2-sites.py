import re, sys, json
LIT = r'"(?:[^"\\\n]|\\.)*"|\'(?:[^\'\\\n]|\\.)*\'|`(?:[^`\\]|\\.)*`'
SITE = re.compile(r'(?P<head>(?:new\s+|throw\s+)*)\bError\(\s*(?P<lit>' + LIT + r')(?P<rest>)')
def sites(text):
    out = []
    for m in SITE.finditer(text):
        lit = m.group("lit")
        out.append((m.start(), m.end(), lit))
    return out
if __name__ == "__main__":
    p = sys.argv[1]
    t = open(p, encoding="utf8").read()
    for s, e, lit in sites(t):
        # show following 40 chars to see '+expr'
        print(lit, "##", t[e:e+45].replace("\n", " "))
