import os, sys
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), "groups"))
from _gen import emit, live
HERE = os.path.dirname(os.path.abspath(__file__))
FW = "🧰️framework"
NB = FW + "/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage"
LIB = FW + "/🛍️products/🦑️repo/🔨️modules/📚️library"
FILES = [
 NB + "/📤️publication/📚️witness/🧪️tests/🟦️.ts",
 NB + "/🧱️fixed/🧪️tests/📥️receipt/🟦️.ts",
 NB + "/🧹️cleanup/📥️receiver/🧪️tests/📥️release/🟦️.ts",
 FW + "/🔨️modules/🏃️process/📦️artifacts/💾️cache/🧪️tests/🟦️.ts",
 FW + "/🔨️modules/🏃️process/📦️artifacts/💾️cache/🧹️retirement/🧪️tests/🔣️.json",
 FW + "/🔨️modules/🏃️process/💾️state/🧪️tests/🔣️.json",
 LIB + "/⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/📤️publication/🧪️tests/🏗️native-build/📦️owner/🟦️.ts",
 LIB + "/⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/📤️publication/🧪️tests/🏗️native-build/🟦️.ts",
 LIB + "/⚡️caching/💾️storage/🔒️enforcement/🔌️nx/📦️graph/🧪️tests/🟦️.ts",
]
edits = []
for path in FILES:
    count = live(path).count("fixed.img")
    assert count > 0, path
    edits.append((path, "fixed.img", "fixed.dir", count))
edits.append((FW + "/🔨️modules/🏃️process/📦️artifacts/💾️cache/🧹️retirement/🧪️tests/🟦️.ts",
 'expect(text).toContain("request.parentIdentity!==parent.identity");expect(text).toContain("same(last.root,underlying)");',
 'expect(text).toContain(\'bindNativeBackingIdentityV1(request.parentIdentity,parent.identity)==="mismatch"\');expect(text).toContain(\'bindNativeIdentitiesV1([last.root],[underlying])==="mismatch"\');'))
emit("g7", "tests name the directory partition anchor fixed.dir instead of the retired image file fixed.img; the retirement source law follows the relation-bound identities", [], edits, [], os.path.join(HERE, "groups"))
print("g7 written", len(edits))
