"""R2 payload description shared by the overlay builder and the landing script: the new files (destination relative to the worktree, source under new/) and the test files the landing runs."""
LIB="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build"
S=LIB+"/🧠️memory/🧽️settle"
NEWFILES=[
 (S+"/🟨️.mjs","settle.mjs"),
 (S+"/🟨️.d.mts","settle.d.mts"),
 (S+"/🧬️schema/🔣️.json","settle.schema.json"),
 (S+"/🧪️tests/🟦️.ts","settle.test.ts"),
 (LIB+"/🏡️resident/🧪️tests/🧽️plan-answered/🟦️.ts","plan-answered.test.ts"),
 (LIB+"/🕸️graph/📥️inference/🧭️script-inputs/🧪️tests/🧽️walk-maps/🟦️.ts","walk-maps.test.ts"),
]
R=LIB+"/🏡️resident/🧪️tests"
TESTS=[
 S+"/🧪️tests/🟦️.ts",R+"/🧽️plan-answered/🟦️.ts",LIB+"/🕸️graph/📥️inference/🧭️script-inputs/🧪️tests/🧽️walk-maps/🟦️.ts",
 LIB+"/🧠️memory/🪜️plan-stage/🧪️tests/🟦️.ts",R+"/🌐️host/🟦️.ts",R+"/🧭️plan/🟦️.ts",R+"/🪜️plan-refusal/🟦️.ts",R+"/🪓️plan-limit/🟦️.ts",
]
