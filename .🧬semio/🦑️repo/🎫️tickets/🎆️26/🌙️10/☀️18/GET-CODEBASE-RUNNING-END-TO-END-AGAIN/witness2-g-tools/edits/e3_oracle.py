#!/usr/bin/env python3
"""♻️ e3: the plan-reuse oracle answers the `ignore` port with the production digest."""
import sys
F = sys.argv[1] + "/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/♻️plan-reuse/🧪️tests/🟦️.ts"
text = open(F, encoding="utf8").read()
def once(old, new):
    global text
    assert text.count(old) == 1, (text.count(old), old[:70])
    text = text.replace(old, new)
once('import {bazelPlanKeyV1}from"../🟦️.ts";', 'import {bazelPlanKeyV1}from"../🟦️.ts";\nimport {bazelIgnoreSourcesDigestV1}from"../../📜️script.ts";')
once('return worldWitnessGitDigestV1(listing)===expected;}});', 'return worldWitnessGitDigestV1(listing)===expected;},ignore:async(_subject:string,expected:string)=>(await bazelIgnoreSourcesDigestV1(root,signal))===expected});')
open(F, "w", encoding="utf8").write(text)
