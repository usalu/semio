"""usage: patch-image-scratch.py — two more image tests (backends conformance, platform heal) fall back to an inherited TMPDIR when no artifact directory is set; a single-file run whose TMPDIR lies inside a managed native parent would mount there. They take their scratch from the per-run sandbox instead (testScratchRootV1 follows a configured artifact directory itself)."""
import os
root=os.path.abspath("🧰️framework/🔨️modules/🏃️process")
S=root+"/📦️artifacts/🏗️native-build/💾️storage/"
sandbox=root+"/🧪️testing/🏖️sandbox/🟦️.ts"
def patch(rel,old,new,route):
    p=S+rel;s=open(p).read()
    assert s.count(old)==1,(rel,old)
    s=s.replace(old,new)
    spec=os.path.relpath(sandbox,os.path.dirname(p))
    first=s.index("\n")+1
    s=s[:first]+'import{testScratchRootV1}from"'+spec+'";\n'+s[first:]
    open(p,"w").write(s)
patch("🌐️platform/🥾️boot/🩹️recovery/🧪️tests/🩺️heal/🟦️.ts",
 'const base=process.env.SEMIO_TEST_ARTIFACT_DIR?resolve(process.env.SEMIO_TEST_ARTIFACT_DIR):tmpdir();',
 'const base=testScratchRootV1("heal");',"heal")
patch("🔌️backends/🧪️tests/✅️conformance/🟦️.ts",
 'join(process.env.SEMIO_TEST_ARTIFACT_DIR||process.env.TMPDIR||tmpdir(),"sc-")',
 'join(testScratchRootV1("conformance"),"sc-")',"conformance")
