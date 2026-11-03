"""One-off repair of the en1998 leaf run: record names were written hyphenated (`Change-building-ns-q`); rewrites them to PascalCase and the camelCase mutation discriminators."""
import os, re, sys
ART = "/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998"
proto = open(ART + "/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/📡️.protocol.semio", encoding="utf-8").read()
kinds = re.findall(r"^record (\S+) tag=(\d+)", proto, re.M)
kinds = [k for k, t in kinds if int(t) >= 110]
assert len(kinds) == 28, len(kinds)
pas = lambda k: "".join(x[:1].upper() + x[1:] for x in k.split("-"))
cam = lambda k: pas(k)[:1].lower() + pas(k)[1:]
n = 0
for d, _, fs in os.walk(ART):
    if "/target" in d: continue
    for f in fs:
        if not f.endswith((".rs", ".json", ".ts", ".py", ".feature", ".semio", ".toml", ".lock")): continue
        p = os.path.join(d, f)
        try: t = open(p, encoding="utf-8").read()
        except Exception: continue
        u = t
        for k in kinds:
            B = "C" + k[1:]
            u = re.sub(r"(?<![A-Za-z0-9-])%s(?![A-Za-z0-9-])" % re.escape(B), pas(k), u)
            u = re.sub(r'("mutation"\s*:\s*")%s(")' % re.escape(k), lambda m: m.group(1) + cam(k) + m.group(2), u)
            u = re.sub(r'(const"\s*:\s*")%s(")' % re.escape(k), lambda m: m.group(1) + cam(k) + m.group(2), u)
            u = re.sub(r'(mutation: )"%s"' % re.escape(k), lambda m: m.group(1) + '"' + cam(k) + '"', u)
            u = re.sub(r'(normWireLiteral\()"%s"' % re.escape(k), lambda m: m.group(1) + '"' + cam(k) + '"', u)
            u = re.sub(r"^(\s+)%s: parse" % re.escape(k), lambda m: m.group(1) + cam(k) + ": parse", u, flags=re.M)
            u = re.sub(r"parse(?:Change)?-%s" % re.escape(k[7:]), "parse" + pas(k), u)
        if u != t:
            open(p, "w", encoding="utf-8").write(u); n += 1
print("fixed files", n)
