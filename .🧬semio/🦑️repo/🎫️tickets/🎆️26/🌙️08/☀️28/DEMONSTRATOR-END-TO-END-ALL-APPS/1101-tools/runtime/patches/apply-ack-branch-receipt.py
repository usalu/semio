#!/usr/bin/env python3
"""Patch 2 (needs coordinator approval, r47 evidence): after patch 1 the lifecycle error is "child receipt differs from whole producer receipt" with whole = 0.
The `worker_outcome_pending` branch of drive_worker_step posts the outcome acknowledgement to the context but returns Progress(Default); it must report what it posted (net of the reserved-return delta), like the admit_pending branch."""
p="🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️retained-command/🔄️full-operation/🦀️.rs"
s=open(p).read()
a="""        if self.worker_outcome_pending{
            if session.acknowledge_checked_out_outcome(cx)?{self.worker_outcome_pending=false;self.worker_resume_pending=true;}
            return Ok(PluginLifecycleStep::Progress(Default::default()));
        }"""
b="""        if self.worker_outcome_pending{
            let before=cx.retained_progress();let reserved=cx.reserved_return_progress();
            if session.acknowledge_checked_out_outcome(cx)?{self.worker_outcome_pending=false;self.worker_resume_pending=true;}
            let after=semio_framework_plugin_ownership::bounded_ownership::original_lifecycle_net_after(cx.retained_progress(),reserved,cx.reserved_return_progress());
            return Ok(PluginLifecycleStep::Progress(semio_framework_plugin_ownership::bounded_ownership::original_lifecycle_delta(before,after)?));
        }"""
assert s.count(a)==1;open(p,'w').write(s.replace(a,b))
q="🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏛️ownership/🦀️.rs"
t=open(q).read();x="pub use original_failure::{original_lifecycle_grant,"
assert t.count(x)==1;open(q,'w').write(t.replace(x,"pub use original_failure::{original_lifecycle_net_after,original_lifecycle_grant,"));print("applied")
