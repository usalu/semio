import json
ids=["worker-carrying-box","parent-with-stroller","jogger","person-with-umbrella","leaning-person","tour-guide"]
doc={"$schema":"../../🧬️schema/🔣️.json#/$defs/CategoryDocument","version":"semio.staffage/v1","category":"humans","assets":[json.load(open(f"out/{i}.json")) for i in ids]}
json.dump(doc,open("../humans-b.json","w"),ensure_ascii=False,indent=1)
print("merged",len(ids))
