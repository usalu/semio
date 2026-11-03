import time
p=r"C:\git\semio\🎓️teaching\🏛️architecture\⚡️energy\🏗️authoring\🎨️visuals\🧍️staffage\🔣️.json"
s=open(p,encoding="utf-8",newline="").read()
a='    "ventilation-need-mvhr":'
b='''    "ventilation-mechanical-mvhr": { "asset": "mvhr-unit", "clip": "boost-spin", "view": "thermal", "environment": "office-neutral", "fps": 15, "theme": "dark" },
    "ventilation-mechanical-pendulum-fan": { "asset": "decentralised-hrv-fan", "clip": "pendulum-cycle", "view": "thermal", "environment": "office-neutral", "fps": 15, "theme": "dark" },
'''
assert s.count(a)==1
for i in range(8):
    try:
        open(p,"w",encoding="utf-8",newline="").write(s.replace(a,b+a)); print("ok"); break
    except OSError:
        time.sleep(1)
