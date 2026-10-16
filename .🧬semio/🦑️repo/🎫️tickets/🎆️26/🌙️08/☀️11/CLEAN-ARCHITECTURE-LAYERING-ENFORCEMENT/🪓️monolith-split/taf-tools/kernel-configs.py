K = "/Users/ueli/Documents/semio/🧰️framework/"
# kernel config
p = K + "🔨️modules/🎠️kernel/🧪️tests/🎚️config/🟦️.ts"
t = open(p, encoding="utf8", newline="").read()
a = 'include: ["🧪️tests/🔬️scope-contributions/🟦️.ts", "🧪️tests/🏷️history-entry-label/🟦️.ts", "🧪️tests/🧪️history-patch/🟦️.ts", "🧪️tests/🧪️history-notices/🟦️.ts", "🧪️tests/🧪️framework-notices/🟦️.ts", "🧪️tests/🧪️history-reprojection/🟦️.ts", "🧪️tests/📦️package-assets/🟦️.ts"],'
assert t.count(a) == 1
t = t.replace(a, a[:-2] + ', "🧪️tests/🧪️createturnoutcomebroadcast/🟦️.ts"],')
b = 'includeSource: ["📡️turn-outcome-broadcast/🟦️.ts", "🧭️app-router/🟦️.ts", "🎛️activation-registry/🟦️.ts", "🗂️plugin-registry/🟦️.ts", "🚪️io-router/🟦️.ts", "🔌️plugin-source/🟦️.ts", "📤️return/📦️content/🟦️.ts"],'
assert t.count(b) == 1
t = t.replace(b, 'includeSource: ["📤️return/📦️content/🟦️.ts"],')
c = "/** 🧪️ Runs canonical kernel cases and existing production-to-test registration bridges. */"
assert t.count(c) == 1
t = t.replace(c, "/** 🧪️ Runs canonical kernel cases and the kernel children's behaviour suites; the only in-source file left is the content return module. */")
open(p, "w", encoding="utf8", newline="").write(t)
# framework config
p = K + "🧪️tests/🎚️config/🟦️.ts"
t = open(p, encoding="utf8", newline="").read()
kids = ["📡️turn-outcome-broadcast", "🧭️app-router", "🎛️activation-registry", "🗂️plugin-registry", "🚪️io-router", "🔌️plugin-source"]
ins = ", ".join('"🔨️modules/🎠️kernel/%s/🟦️.ts"' % k for k in kids)
assert t.count(ins + ", ") == 2, t.count(ins + ", ")
i0 = t.index("includeSource:")
head, tail = t[:i0], t[i0:]
assert tail.count(ins + ", ") == 1
tail = tail.replace(ins + ", ", "")
assert head.count("    include: [],") == 1
head = head.replace("    include: [],", '    include: ["🔨️modules/🎠️kernel/🧪️tests/🧪️createturnoutcomebroadcast/🟦️.ts"],')
old_c = """    // 🩹️ `include` MUST stay empty: this is an in-source (`import.meta.vitest`) suite collected via
    // `includeSource`. Listing the same file in BOTH keys made vitest collect it twice and report
    // double the real test count. Add new in-source files to `includeSource`/`coverage.include` only.
"""
new_c = """    // 🩹️ `include` lists plain test files only; in-source (`import.meta.vitest`) files go to `includeSource`.
    // Listing the same file in BOTH keys made vitest collect it twice and report double the real test count.
"""
assert head.count(old_c) == 1
head = head.replace(old_c, new_c)
open(p, "w", encoding="utf8", newline="").write(head + tail)
print("ok")
