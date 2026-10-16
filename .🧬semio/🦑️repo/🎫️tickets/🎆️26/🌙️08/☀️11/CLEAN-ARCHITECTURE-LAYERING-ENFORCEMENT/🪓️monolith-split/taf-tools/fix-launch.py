import sys
old = "${workspaceFolder}/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts"
new = "${workspaceFolder}/✏️s/🧪️tests/🏘️plugin-composition/📏️layout-drawing/🪶️sqlite/🟦️.ts"
for p in sys.argv[1:]:
    t = open(p, encoding="utf8").read()
    assert t.count(old) == 1, (p, t.count(old))
    open(p, "w", encoding="utf8").write(t.replace(old, new))
    print("ok", p)
