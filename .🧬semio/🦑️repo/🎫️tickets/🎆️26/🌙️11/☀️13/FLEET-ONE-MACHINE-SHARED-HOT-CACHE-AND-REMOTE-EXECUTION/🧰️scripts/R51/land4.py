#!/usr/bin/env python3
"""R51 landing 4: puts the standing landing test of the tool-closure law at the head of landings.md (takes landing.lock; the file is rewritten atomically with the header prepended)."""
import os,sys
T="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION"
lock=T+"/landing.lock"
try:os.mkdir(lock)
except FileExistsError:sys.exit("held by "+open(lock+"/owner").read().strip())
open(lock+"/owner","w").write("R51-%d\n"%os.getpid())
try:
    p=T+"/landings.md";t=open(p,encoding="utf8").read()
    head="""# Landing protocol: standing tests (added by R51, 2026-11-15)

Every landing script runs, besides the test files of the members it touches, these standing files (a red one blocks the landing; the tree was read for 90 minutes once before a refusal in a declared tool closure showed up):
- `LIB/🏗️build/🕸️graph/📥️inference/🧭️script-inputs/🔌️provider-closures/🧪️tests/🛃️preflight/🟦️.ts`: walks every declared tool closure and provider port of the real tree and fails on a generated import (`closure.provider-generated-import`), the way the group read would; about 30 s. A tool row whose closure reaches a generated output (for example through `🛂️manifest`) must not be declared: remove the row, the nodes then walk the script themselves.
- `LIB/🏗️build/🕸️graph/📥️inference/🧭️script-inputs/🪆️heavy-questions/🧪️tests/🟦️.ts`: a refusal raised in a heavy-question child reaches the parent with code, key and values in both languages.
- A read also runs the same walk in a capped child at its start (`preflightToolClosuresInChildV1`, prime stage), so a refused closure stops it in seconds with its tool and targets.

"""
    if t.startswith("# Landing protocol: standing tests"):sys.exit("already there")
    open(p+".r51tmp","w",encoding="utf8").write(head+t);os.replace(p+".r51tmp",p)
finally:
    os.remove(lock+"/owner");os.rmdir(lock)
