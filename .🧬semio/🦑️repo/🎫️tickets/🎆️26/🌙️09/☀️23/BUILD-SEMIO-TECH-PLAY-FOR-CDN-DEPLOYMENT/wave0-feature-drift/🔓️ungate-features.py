"""🔓️ Makes cargo features unconditional in Rust sources: drops `#[cfg(...)]` attributes that only require the named
features, and deletes the item each negated gate guards (attribute, docs, item through its balanced braces).
usage: python3 🔓️ungate-features.py <feature>[,<feature>...] <file.rs>..."""
import re, sys
features = sys.argv[1].split(",")
alt = "|".join(re.escape(f) for f in features)
one = rf'feature\s*=\s*"(?:{alt})"'
positive = re.compile(rf'^\s*#\[cfg\((?:{one}|any\((?:test,\s*)?{one}(?:,\s*{one})*\))\)\]\s*$')
negative = re.compile(rf'^\s*#\[cfg\(not\((?:{one}|any\({one}(?:,\s*{one})*\))\)\)\]\s*$')
inline_positive = re.compile(rf'(?:{one}|any\({one}(?:,\s*{one})*\)),\s*')
for path in sys.argv[2:]:
    lines = open(path).read().split("\n")
    out, i, dropped, deleted = [], 0, 0, 0
    while i < len(lines):
        line = lines[i]
        if positive.match(line):
            dropped += 1; i += 1; continue
        if negative.match(line):
            j = i + 1
            while j < len(lines) and re.match(r'^\s*(///|#\[)', lines[j]): j += 1
            depth, k = 0, j
            while True:
                depth += lines[k].count("{") - lines[k].count("}")
                if depth <= 0 and (lines[k].rstrip().endswith(";") or lines[k].rstrip().endswith("}")): break
                k += 1
            deleted += 1; i = k + 1; continue
        if re.search(rf'#\[cfg\(all\({one}', line) or re.search(rf'#\[cfg\(all\(any\({one}', line):
            line = re.sub(rf'#\[cfg\(all\((?:{one}|any\({one}(?:,\s*{one})*\)),\s*(.*)\)\)\]', r'#[cfg(\1)]', line)
        out.append(line); i += 1
    text = "\n".join(out)
    if re.search(one, text): print(f"REMAINING {path}: " + "; ".join(l.strip() for l in text.split("\n") if re.search(one, l))[:600])
    open(path, "w").write(text)
    print(f"{path}: dropped {dropped} gates, deleted {deleted} negated items")
