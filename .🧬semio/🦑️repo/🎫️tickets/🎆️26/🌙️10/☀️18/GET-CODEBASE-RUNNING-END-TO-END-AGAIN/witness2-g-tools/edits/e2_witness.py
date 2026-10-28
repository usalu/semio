#!/usr/bin/env python3
"""🔭️ e2: witness - observation kind `i` (git's answer about the ignore sources) with its `ignore` port, schema kind row, type."""
import sys
root = sys.argv[1]
W = root + "/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔭️witness/"
def edit(name, pairs):
    text = open(W + name, encoding="utf8").read()
    for old, new in pairs:
        assert text.count(old) == 1, (name, text.count(old), old[:70])
        text = text.replace(old, new)
    open(W + name, "w", encoding="utf8").write(text)
edit("🟨️.mjs", [('''  if (kind === "x") return ports?.runtime !== undefined && await ports.runtime(subject, expected);''', '''  if (kind === "i") return ports?.ignore !== undefined && await ports.ignore(subject, expected);
  if (kind === "x") return ports?.runtime !== undefined && await ports.runtime(subject, expected);''')])
edit("🧬️schema/🔣️.json", [('''  "g": "git index listing",''', '''  "g": "git index listing",
  "i": "git ignore sources, by the digest of git's own answers about config, excludes file and info/exclude",''')])
edit("🟨️.d.mts", [('''/** 🔌️ The checks only the caller can make: the git index listing and the output of a command. */
export interface WorldWitnessPortsV1 { git?(subject: string, expected: string): Promise<boolean>; runtime?''', '''/** 🔌️ The checks only the caller can make: the git index listing, git's answer about its ignore sources and the output of a command. */
export interface WorldWitnessPortsV1 { git?(subject: string, expected: string): Promise<boolean>; ignore?(subject: string, expected: string): Promise<boolean>; runtime?''')])
