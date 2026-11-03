#!/usr/bin/env python3
"""Fail-closed patch (supersedes energy/patches/hydration-binds-job-authority.py): archive-load parent hydration authority is the stepping maintenance job's identity, bound at creation and re-bound every step (the hydration is owned by the archive load, not by one job turn)."""
F='🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/📄️docs/📥️staging/🪜️maintenance-ladder/🦀️.rs'
s=open(F).read()
a="""                        semio_framework_job::OperationId(active.operation),
                        semio_framework_job::Generation(stores.store.generation_now()),
"""
b="""                        cx.operation(),
                        cx.generation(),
"""
c="""                let step=active.hydration.as_mut().expect("recursive document parent hydration remains retained").step(cx,RetainedCloneGrant{maximum_items:1,..grant});"""
d="""                let owner=active.hydration.as_mut().expect("recursive document parent hydration remains retained");
                owner.operation=cx.operation();
                owner.generation=cx.generation();
                let step=owner.step(cx,RetainedCloneGrant{maximum_items:1,..grant});"""
assert s.count(a)==1 and s.count(c)==1,"anchor count"
open(F,'w').write(s.replace(a,b).replace(c,d));print("applied")
