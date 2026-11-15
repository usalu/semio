"""R4 payload description shared by the overlay builder and the landing script: the new files (destination relative to the worktree, source under new/) and the test files the landing runs."""
LIB="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build"
PROC="🧰️framework/🔨️modules/🏃️process/🏗️build"
GS=LIB+"/🪪️graph-state"
W=GS+"/🙋️wanted"
HC=PROC+"/🌡️host-capacity"
NEWFILES=[
 (W+"/🧪️tests/🍏️idle-grant/🟦️.ts","idle-grant.test.ts"),
 (LIB+"/🕸️graph/🧪️tests/🙋️fleet-read/🍏️macos-reading/🟦️.ts","macos-reading.test.ts"),
]
TESTS=[
 W+"/🧪️tests/🍏️idle-grant/🟦️.ts",
 LIB+"/🕸️graph/🧪️tests/🙋️fleet-read/🍏️macos-reading/🟦️.ts",
 LIB+"/🕸️graph/🧪️tests/🙋️fleet-read/🌍️small-booking/🟦️.ts",LIB+"/🕸️graph/🧪️tests/🙋️fleet-read/🟦️.ts",
 W+"/🧪️tests/🟦️.ts",W+"/🧪️tests/📖️read/🟦️.ts",W+"/🧪️tests/🛠️server/🟦️.ts",W+"/🧪️tests/🌍️two-platforms/🟦️.ts",W+"/💰️cost/🧪️tests/🟦️.ts",W+"/💰️cost/🧪️tests/🌍️measured-only/🟦️.ts",
 HC+"/🧪️tests/🔋️available/🟦️.ts",HC+"/🧪️tests/📡️sampling/🟦️.ts",
 LIB+"/🏡️resident/🧪️tests/🌡️pressure/🟦️.ts",
]
