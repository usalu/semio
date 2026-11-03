#!/usr/bin/env python3
"""Patch (apply after the freeze lifts): original_lifecycle_step must net the reserved return effects out of the posted delta.
Evidence: runtime-r46.txt backtrace -> original_lifecycle_step <- MountedTypedCommandFullOperation::drive_worker_step; whole receipt 1 item, children posted 2.
The worker pump verifies its receipt with original_worker_current_progress (after-before minus the reserved-return delta), the lifecycle loan did not."""
p="🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏛️ownership/⚠️failure/🦀️.rs"
s=open(p).read()
a="let before=cx.retained_progress();let fuel=cx.fuel_remaining();let result=advance(cx,grant);"
b="let before=cx.retained_progress();let reserved_before=cx.reserved_return_progress();let fuel=cx.fuel_remaining();let result=advance(cx,grant);"
assert s.count(a)==1;s=s.replace(a,b)
a="let remaining=original_lifecycle_unposted(progress,before,cx.retained_progress())?;"
b="let remaining=original_lifecycle_unposted(progress,before,original_lifecycle_net_after(cx.retained_progress(),reserved_before,cx.reserved_return_progress()))?;"
assert s.count(a)==1;s=s.replace(a,b)
a="/// 🧮️ Subtracts already posted child effects"
h='''/// 🧾️ Removes the reserved return effects the same turn posted from its raw recipient total, the way the worker pump qualifies its own receipt.
pub fn original_lifecycle_net_after(after:RetainedCloneProgress,reserved_before:RetainedCloneProgress,reserved_after:RetainedCloneProgress)->RetainedCloneProgress{
 let net=|total:usize,earlier:usize,later:usize|total.saturating_sub(later.saturating_sub(earlier));
 RetainedCloneProgress{copied_items:net(after.copied_items,reserved_before.copied_items,reserved_after.copied_items),copied_bytes:net(after.copied_bytes,reserved_before.copied_bytes,reserved_after.copied_bytes),retained_capacity_bytes:net(after.retained_capacity_bytes,reserved_before.retained_capacity_bytes,reserved_after.retained_capacity_bytes),released_bytes:net(after.released_bytes,reserved_before.released_bytes,reserved_after.released_bytes)}
}

'''
assert s.count(a)==1;s=s.replace(a,h+a)
open(p,'w').write(s)
print("applied")
