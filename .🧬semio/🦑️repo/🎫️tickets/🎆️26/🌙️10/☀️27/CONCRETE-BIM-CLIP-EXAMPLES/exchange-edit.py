import sys, time
P = r"C:/git/semio/🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🔌️exchange/🐍️.py"
def edit(pairs):
    s = open(P, encoding="utf-8").read()
    miss = 0
    for a, b in pairs:
        if a not in s:
            print("MISSING:", a[:110]); miss += 1
            continue
        s = s.replace(a, b, 1)
    for _ in range(10):
        try:
            open(P, "w", encoding="utf-8").write(s)
            break
        except OSError as e:
            print("retry", e); time.sleep(1)
    print("edits applied, missing", miss)
