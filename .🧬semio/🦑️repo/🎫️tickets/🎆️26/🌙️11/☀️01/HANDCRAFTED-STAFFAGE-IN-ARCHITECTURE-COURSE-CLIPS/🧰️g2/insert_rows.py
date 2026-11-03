import io
M = "🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🔣️.json"
G = ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/HANDCRAFTED-STAFFAGE-IN-ARCHITECTURE-COURSE-CLIPS/🗑️generated/g2/rows.txt"
s = open(M, encoding="utf-8", newline="").read()
if '"id": "gas-hob"' in s:
    print("already")
else:
    rows = open(G, encoding="utf-8").read()
    key = '    {\n      "id": "storage-crate-stack"'
    nl = "\r\n" if "\r\n" in s else "\n"
    key = key.replace("\n", nl)
    assert s.count(key) == 1
    rows = rows.replace("\n", nl)
    s = s.replace(key, rows + key)
    import time
    for i in range(30):
        try:
            open(M, "w", encoding="utf-8", newline="").write(s)
            break
        except OSError:
            time.sleep(0.5)
    print("inserted")
