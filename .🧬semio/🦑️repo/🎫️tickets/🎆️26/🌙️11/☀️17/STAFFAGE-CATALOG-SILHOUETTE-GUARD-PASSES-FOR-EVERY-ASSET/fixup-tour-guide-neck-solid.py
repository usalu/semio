P="/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/👤️humans/tour-guide/🔣️.json"
s=open(P,encoding="utf8").read()
old='''              "id": "neck",
              "bone": "head",
              "role": "body",
              "z": 3.8,
              "filled": true,
              "tone": 0.84,
              "region": "head-skin",
              "material": "skin"
            },'''
new=old[:-15]+''',
              "solid": {
                "kind": "revolved",
                "axisBone": "head",
                "axis": "u",
                "axisX": 0,
                "axisZ": 0,
                "mirror": true
              }
            },'''
assert s.count(old)==1
open(P,"w",encoding="utf8").write(s.replace(old,new))
