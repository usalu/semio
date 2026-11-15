#!/usr/bin/env python3
"""Package O landing 15: units outside the checkout (registry crates) stay local: their source names in the metadata cannot be made equal to a local compile."""
import pathlib
B=pathlib.Path("/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-execution/🚀️offload")
p=B/"🗺️paths/🟦️.ts"; t=p.read_text()
o=' const inside=within(cwd,manifest),shipped='
assert t.count(o)==1
t=t.replace(o,' if(!within(cwd,manifest))return decline("not-portable","unit outside the checkout (registry crate): its source names differ from a local compile");\n'+o)
p.write_text(t)
