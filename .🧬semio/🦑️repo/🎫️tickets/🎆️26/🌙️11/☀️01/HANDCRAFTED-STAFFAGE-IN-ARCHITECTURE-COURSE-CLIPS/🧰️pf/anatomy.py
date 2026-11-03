"""📊 Byte anatomy of one SVG frame: metadata, defs, body, numbers."""
import re, sys
s = open(sys.argv[1], encoding="utf-8").read()
meta = re.search(r"<metadata>.*?</metadata>", s, re.S)
print("total", len(s), "metadata", len(meta.group(0)) if meta else 0)
body = s[meta.end():] if meta else s
print("body", len(body), "paths", len(re.findall(r"<path", body)), "digits>4", len(re.findall(r"\d+\.\d{5,}", body)))
print(body[:700])
print(re.sub(r"\s+", " ", meta.group(0))[-1500:])
