import time
M = "🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🔣️.json"
G = ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/HANDCRAFTED-STAFFAGE-IN-ARCHITECTURE-COURSE-CLIPS/🗑️generated/sc/row.txt"
s = open(M, encoding="utf-8", newline="").read()
if '"id": "solar-thermal-collector"' in s:
    print("already")
else:
    rows = open(G, encoding="utf-8").read()
    key = '    {\n      "id": "evaporative-humidifier"'
    assert s.count(key) == 1
    s = s.replace(key, rows + key)
    for i in range(30):
        try:
            open(M, "w", encoding="utf-8", newline="").write(s)
            break
        except OSError:
            time.sleep(0.5)
    print("inserted")
