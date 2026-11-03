#!/usr/bin/env python3
"""[DEBUG] diagnostic (apply only in a patch window; remove afterwards): says whether settlement died on should_yield (fuel/deadline) or on the turn cap, and prints fuel + runnable state."""
p="🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️testing/🧪️artifact-app-laws/🏗️registered-app-construction-and-typed-operation-settlement/🦀️.rs"
s=open(p).read()
a='if cx.should_yield()||turns>=maximum_turns{return Err('
b='if cx.should_yield()||turns>=maximum_turns{eprintln!("[DEBUG] settle stop: should_yield={} turns={} cap={} fuel={} runnable={} pending={} maint_empty={}",cx.should_yield(),turns,maximum_turns,cx.fuel_remaining(),app.has_runnable_typed_operations(),app.has_pending_typed_operations(),app.maintenance_terminal_is_empty());return Err('
assert s.count(a)==1;open(p,'w').write(s.replace(a,b));print("applied")
