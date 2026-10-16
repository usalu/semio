import sys
old = '"name": "⚖️test-worker-law-suites🖥️os🟦️",\n      "type": "node-terminal",\n      "request": "launch",\n      "command": "bun repo run @semio-tech/framework-os:test -- quick ../../🔨️modules/🏪️store/👷️worker/🟦️.ts",'
new = old.replace("quick ../../🔨️modules/🏪️store/👷️worker/🟦️.ts", "quick ../../🧪️tests/📚️worker-law-suites/🟦️.ts")
for p in sys.argv[1:]:
    t = open(p, encoding="utf8", newline="").read()
    assert t.count(old) == 1, (p, t.count(old))
    open(p, "w", encoding="utf8", newline="").write(t.replace(old, new))
    print("ok", p)
