import re, sys
text = open(sys.argv[1], errors="replace").read()
only = sys.argv[2] if len(sys.argv) > 2 else None
blocks = re.split(r"\n(?=---- )", text)
target = None
out = []
for line_block in re.split(r"(?m)^(?=\s+Running )", text):
    m = re.match(r"\s+Running (.*?) \((.*?)/out/(.*?)\)", line_block)
    name = (m.group(2).split("/")[-1] + ":" + m.group(3).split("-")[0]) if m else "?"
    for f in re.finditer(r"---- (\S+) stdout ----\n(.*?)(?=\n---- |\nfailures:)", line_block, re.S):
        body = f.group(2).strip().replace("\n", " | ")
        body = re.sub(r"thread '.*?' \(\d+\) panicked at ", "@", body)
        body = re.sub(r"note: run with.*", "", body)
        out.append((name, f.group(1), body[:int(sys.argv[3]) if len(sys.argv) > 3 else 260]))
for n, t, b in out:
    if only and only not in n: continue
    print(n, "::", t.split("::")[-1][:90], "::", b)
