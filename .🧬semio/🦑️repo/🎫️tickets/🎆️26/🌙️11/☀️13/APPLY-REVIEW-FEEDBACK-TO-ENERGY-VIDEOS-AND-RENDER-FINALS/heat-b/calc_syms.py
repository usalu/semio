import sys

path = sys.argv[1]
text = open(path, encoding="utf-8", newline="").read()
marker = "class AnlagenVerluste"
head, tail = text.split(marker, 1)
start = head.index("class ReviewingHeatGains")
before, body = head[:start], head[start:]

replacements = [
    (r"\,\mathrm{{kWh/a}}", r"\,\frac{{\mathrm{{kWh}}}}{{\mathrm{{a}}}}"),
    (r"Q_{{\mathrm{{sol}}}}", r"Q_{{\mathrm{{S}}}}"),
    (r"Q_{{\mathrm{{int}}}}", r"Q_{{\mathrm{{i}}}}"),
    (r"Q_{{\mathrm{{Gewinn}}}}", r"Q_{{\mathrm{{g}}}}"),
    (r"Q_{{\mathrm{{Verlust}}}}", r"Q_{{\mathrm{{l}}}}"),
    (r"Q_{\mathrm{sol}}", r"Q_{\mathrm{S}}"),
    (r"Q_{\mathrm{int}}", r"Q_{\mathrm{i}}"),
    (r"Q_{\mathrm{Gewinn}}", r"Q_{\mathrm{g}}"),
    (r"Q_{\mathrm{Verlust}}", r"Q_{\mathrm{l}}"),
    (r"\eta_{{\mathrm{{h}}}}", r"\eta"),
    (r"\eta_{\mathrm{h}}", r"\eta"),
    (r"A_{{\mathrm{{Gewinn}}}}", r"Q_{{\mathrm{{g}}}}"),
    (r"A_{{\mathrm{{nutz}}}}", r"Q_{{\mathrm{{g,n}}}}"),
    (r"t\;[\mathrm{h}]", r"t\;\text{in}\;\mathrm{h}"),
    (r"P\;[\mathrm{kW}]", r"P\;\text{in}\;\mathrm{kW}"),
]
for old, new in replacements:
    body = body.replace(old, new)
open(path, "w", encoding="utf-8", newline="").write(before + body + marker + tail)
print("ok")
