import os

root = "/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚖️en1990"
target = None
for dirpath, _, files in os.walk(root):
    if dirpath.endswith("snapshot") and "sqlite" in dirpath:
        for name in files:
            if name.endswith(".rs") and name.count(".") == 1:
                target = os.path.join(dirpath, name)
print(target)
text = open(target, encoding="utf-8").read()
old = [
    "bound.repeated(action.kind.len(), 24)",
    "bound.repeated(analysis.resistance_distribution.len(), 24)?; bound.repeated(analysis.action_distribution.len(), 24)?",
    "bound.repeated(series.quantity.len(), 24)?; bound.repeated(series.distribution.len(), 24)?",
]
new = [
    "bound.repeated(action.kind.as_str().len(), 24)",
    "bound.repeated(analysis.member_id.len(), 24)?; bound.repeated(analysis.resistance_distribution.as_str().len(), 24)?; bound.repeated(analysis.action_distribution.as_str().len(), 24)?; bound.repeated(analysis.second_action_distribution.as_str().len(), 24)?",
    "bound.repeated(series.quantity.len(), 24)?; bound.repeated(series.pairs.len(), 24)?; bound.repeated(series.distribution.as_str().len(), 24)?",
]
for a, b in zip(old, new):
    count = text.count(a)
    print(count, a[:40])
    if count != 1:
        raise SystemExit(1)
    text = text.replace(a, b)
open(target, "w", encoding="utf-8").write(text)
