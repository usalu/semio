import time
def patch(p, a, b):
    s = open(p, encoding="utf-8", newline="").read()
    assert s.count(a) == 1, a
    for _ in range(8):
        try:
            open(p, "w", encoding="utf-8", newline="").write(s.replace(a, b)); return
        except OSError:
            time.sleep(1)
V = r"C:\git\semio\🎓️teaching\🏛️architecture\⚡️energy\🏗️authoring\🎨️visuals"
patch(V + r"\🧍️staffage\🔣️.json", '"asset": "laptop", "pose": "open", "view": "thermal", "environment": "office-neutral", "theme": "dark"', '"asset": "laptop", "clip": "work-and-sleep", "view": "thermal", "environment": "office-neutral", "fps": 15, "theme": "dark"')
patch(V + r"\🐍️.py", '{"toaster": 60.0,', '{"laptop": 6.0, "toaster": 60.0,')
print("ok")
