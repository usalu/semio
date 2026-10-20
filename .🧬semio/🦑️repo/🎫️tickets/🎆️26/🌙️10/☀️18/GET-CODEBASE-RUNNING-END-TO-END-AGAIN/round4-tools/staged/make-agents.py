#!/usr/bin/env python3
"""Merged copy of agents-fix for the live tree after round-3 mem-gov: the host-capacity gate lost its inherited-grant error and the `isCurrentHostGrantV1` import neighbour.
Edits of b2-host.mjs: the import anchor becomes `formatHostCapacityV1,isHostCapacityV1,`; the edit of the removed `Closed inherited host capacity grant identifier required` error is dropped
(the message stays in the schema). Everything else is the original; the copy lives in C/round4-tools/staged/agents-fix and its build.mjs writes its own staged/."""
import os, shutil
HERE = os.path.dirname(os.path.abspath(__file__))
C = os.path.dirname(os.path.dirname(HERE))
SOURCE, TARGET = C + "/agents-fix-tools", HERE + "/agents-fix"
shutil.rmtree(TARGET, ignore_errors=True)
shutil.copytree(SOURCE, TARGET, ignore=shutil.ignore_patterns("__pycache__", "staged"))
path = TARGET + "/b2-host.mjs"
text = open(path, encoding="utf8").read()
a = "  edit(H.gate, `formatHostCapacityV1,isCurrentHostGrantV1,`, `formatHostCapacityV1,hostCapacityErrorV1,isCurrentHostGrantV1,`);"
b = '  edit(H.gate, `throw Error("Closed inherited host capacity grant identifier required");`, `throw Error(schema.definitions.Messages.const[language].errors.grantIdentifierRequired);`);\n'
assert text.count(a) == 1 and text.count(b) == 1
text = text.replace(a, "  edit(H.gate, `formatHostCapacityV1,isHostCapacityV1,`, `formatHostCapacityV1,hostCapacityErrorV1,isHostCapacityV1,`);").replace(b, "")
open(path, "w", encoding="utf8").write(text)
print("merged agents-fix: 2 gate edits adapted to the round-3 host-capacity gate")
