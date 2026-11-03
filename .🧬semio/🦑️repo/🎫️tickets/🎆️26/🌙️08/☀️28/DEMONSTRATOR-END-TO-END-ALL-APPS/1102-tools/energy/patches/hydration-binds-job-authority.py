#!/usr/bin/env python3
"""Fail-closed patch: parent hydration must be created under the stepping job's authority (cx.operation, cx.generation), not the archive-load slot id and the live store generation."""
F='🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/📄️docs/📥️staging/🪜️maintenance-ladder/🦀️.rs'
s=open(F).read()
a="""                        semio_framework_job::OperationId(active.operation),
                        semio_framework_job::Generation(stores.store.generation_now()),
"""
b="""                        cx.operation(),
                        cx.generation(),
"""
assert s.count(a)==1,"anchor count"
open(F,'w').write(s.replace(a,b));print("applied")
