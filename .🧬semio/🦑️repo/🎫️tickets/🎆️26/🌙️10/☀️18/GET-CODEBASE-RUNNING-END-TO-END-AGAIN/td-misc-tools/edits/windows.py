DOC = """📏 Staged batch `windows` (lane td-misc): STALE FIXTURE. Since the exact Source reader retries a read that a concurrent edit interrupted (`observe` = retryOnConcurrentEditV1("read"), 10-25) and re-verifies the whole batch snapshot, a leaf that is renamed away and restored while it is read is read again and delivered with the original bytes' digest (accepted, like `parent-transient`); the fixture still described the pre-retry refusal. The race row now records the delivered read as fd-stable and accepted, and the law prices a healed race as at least one full pass (the interrupted attempt comes on top; 351 vs 297 operations on macOS) while it still demands the oracle digest."""
W = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🧪️tests/📏️windows/"
EDITS = [
 (W + "🔣️.json", [
  ('      "id": "leaf-transient",\n      "fdStable": false,\n      "namespaceStable": true,\n      "completed": true,\n      "accepted": false\n',
   '      "id": "leaf-transient",\n      "fdStable": true,\n      "namespaceStable": true,\n      "completed": true,\n      "accepted": true\n'),
 ]),
 (W + "🟦️.ts", [
  ('if(expected.accepted){checks.push([row.readOperations,4*row.parentCount+2*(id==="empty"?0:128)+9]);}',
   'if(expected.accepted){const single=4*row.parentCount+2*(id==="empty"?0:128)+9;checks.push(id==="leaf-transient"?[row.readOperations>=single,true]:[row.readOperations,single]);}'),
 ]),
]
