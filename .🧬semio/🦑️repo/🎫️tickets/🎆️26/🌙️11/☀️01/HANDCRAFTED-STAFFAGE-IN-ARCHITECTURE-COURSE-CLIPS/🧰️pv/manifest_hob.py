import time
p="🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🔣️.json"
s=open(p,encoding="utf-8",newline="").read()
i=s.index('"id": "gas-hob"'); old='"viewpoints": [\n        "elevation"\n      ]'
j=s.index(old,i); assert j-i<4000
s=s[:j]+'"viewpoints": [\n        "elevation",\n        "plan"\n      ]'+s[j+len(old):]
for k in range(10):
    try:
        open(p,"w",encoding="utf-8",newline="").write(s); print("ok"); break
    except OSError as e:
        print("retry",e); time.sleep(1)
