"""🎁️ Wraps every screen-selection call in a bounded mesh-query retry helper now that the selection returns mesh faults."""
import sys
def wrap(text, prefix, suffix):
    out, i = [], 0
    while True:
        hits = [j for j in (text.find("screen_select_components(", i), text.find("screen_select_instances(", i)) if j >= 0]
        if not hits:
            return "".join(out) + text[i:]
        j = min(hits); depth = 0
        for m in range(text.index("(", j), len(text)):
            depth += {"(": 1, ")": -1}.get(text[m], 0)
            if depth == 0:
                break
        out += [text[i:j], prefix, text[j:m + 1], suffix]
        i = m + 1
path, prefix, suffix = sys.argv[1:4]
with open(path) as source:
    wrapped = wrap(source.read(), prefix, suffix)
with open(path, "w") as target:
    target.write(wrapped)
