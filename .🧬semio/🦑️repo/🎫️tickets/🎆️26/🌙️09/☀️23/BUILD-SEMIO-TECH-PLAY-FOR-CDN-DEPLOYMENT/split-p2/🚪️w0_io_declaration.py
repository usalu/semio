#!/usr/bin/env python3
"""🚪️ Wave 0: moves the subset's io registration glue — the derive-generated builder/analyzer/composer facets, the
native composer row and `io()` with its tests — from the io facet root into the subset root, so the io crate (scene
conversions and codecs) sits below the import/export leaves that `io()` registers (lane P2)."""
import os, re
REPO = os.path.abspath(os.path.join(os.path.dirname(os.path.abspath(__file__)), *[".."] * 8))
A = os.path.join(REPO, "✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any")
IO, SUB = os.path.join(A, "🚪️io/🦀️.rs"), os.path.join(A, "🦀️.rs")
io = open(IO, encoding="utf-8").read()
a = io.index("//#region 🎹️DerivedComposition\n"); b = io.index("//#region 🧪️Tests\n")
declaration = io[a:b]
c = io.index("//#endregion 🧪️Tests\n") + len("//#endregion 🧪️Tests\n")
tests = io[b:c]
d = io.index("pub mod derived_construction {")
facets = io[d:]
io = io[:a] + io[c:d].rstrip("\n") + "\n"
open(IO, "w", encoding="utf-8").write(io)
moved = declaration + facets.rstrip("\n") + "\n\n" + tests.replace('#[path = "🧪️tests/🔬️io/🦀️.rs"]', '#[path = "🚪️io/🧪️tests/🔬️io/🦀️.rs"]')
moved = moved.replace("crate::standards::v1::subsets::any::io::RemodelingAnalyzer", "crate::standards::v1::subsets::any::RemodelingAnalyzer")
moved = moved.replace("crate::standards::v1::subsets::any::io::RemodelingComposer", "crate::standards::v1::subsets::any::RemodelingComposer")
sub = open(SUB, encoding="utf-8").read().rstrip("\n") + "\n\n" + moved
sub = sub.replace("        io: io::io(),\n", "        io: io(),\n")
open(SUB, "w", encoding="utf-8").write(sub)
print("moved", moved.count("\n"), "lines")
