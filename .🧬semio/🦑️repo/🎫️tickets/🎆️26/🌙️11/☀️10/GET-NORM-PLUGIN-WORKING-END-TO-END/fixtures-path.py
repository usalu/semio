"""🧲️ Prints the relative path from a crate manifest directory to the run-time fixture reader crate."""
import os, sys
print(os.path.relpath("🧰️framework/🔨️modules/🧪️test/🧲️fixture-reader/📦️packages/🦀️rust", sys.argv[1]))
