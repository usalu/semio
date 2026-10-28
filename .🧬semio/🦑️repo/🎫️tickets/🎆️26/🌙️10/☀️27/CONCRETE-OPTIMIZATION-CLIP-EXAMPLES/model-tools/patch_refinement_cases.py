from pathlib import Path
path = Path("C:/git/semio/🎓️teaching/🏛️architecture/🎯️optimization/🎬️clip/🧪️cases/🔣️.json")
text = path.read_text(encoding="utf-8")
old = '''    "refinement": {
      "member": "w50-t12-d60",
      "dominates": [
        "w50-t12-d60",
        "w40-t20-d60",
        "w50-t20-d60",
        "w40-t30-d60",
        "w50-t30-d60"
      ]
    },'''
new = '''    "refinement": {
      "member": "w50-t30-d60",
      "dominates": [
        "w40-t30-d60",
        "w50-t30-d60"
      ]
    },'''
assert text.count(old) == 1
path.write_text(text.replace(old, new), encoding="utf-8")
