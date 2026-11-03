import os, re, sys
ROOT = "/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/"
art = sys.argv[1]
base = ROOT + art
snap = [os.path.join(d, f) for d, _, fs in os.walk(base) for f in fs if f == "🦀️.rs" and d.endswith("📸️snapshot") and "🧬️schema" in d and "tests" not in d and "sqlite" not in d and "io" not in d.split("/")[-3:]]
snap = [s for s in snap if s.endswith("🧬️schema/📸️snapshot/🦀️.rs")]
t = open(snap[0]).read()
enums = set(re.findall(r'pub enum (\w+)', t))
fields = {}
for sm in re.finditer(r'pub struct (\w+)\s*\{', t):
    i = sm.end(); d = 1; j = i
    while d:
        d += (t[j] == '{') - (t[j] == '}'); j += 1
    for fm in re.finditer(r'pub (\w+):\s*(?:Option<)?(\w+)>?,', t[i:j]):
        if fm.group(2) in enums: fields.setdefault(fm.group(1), set()).add((sm.group(1), fm.group(2)))
print(art, len(enums), "enums;", len(fields), "enum-typed fields")
bad = []
for d, _, fs in os.walk(base):
    if "target" in d: continue
    for f in fs:
        if not f.endswith(".rs"): continue
        p = os.path.join(d, f)
        if "🧬️schema/🧬️mutations" in p or "/🔺️diff/" in p or "sqlite" in p or p == snap[0]: pass
        s = open(p, errors="ignore").read()
        for fld in fields:
            for m in re.finditer(r'\.' + fld + r'\s*(==|!=)\s*"|\.' + fld + r'\.(as_str|is_empty|to_lowercase|to_uppercase|len|chars|starts_with|contains|eq_ignore_ascii_case|parse)\b|\.' + fld + r'\.clone\(\)', s):
                line = s.count("\n", 0, m.start()) + 1
                bad.append((p.replace(ROOT, ""), line, fld, s[m.start():m.start()+40].replace("\n", " "), sorted(fields[fld])[0]))
for b in bad: print(f"{b[0]}:{b[1]}\t{b[2]}\t{b[3]}\t{b[4]}")
print(len(bad))
