import sys
p=sys.argv[1];s=open(p,encoding='utf-8').read()
def r(a,b):
    global s
    assert s.count(a)==1,a[:80];s=s.replace(a,b)
r('''//! 🏡️ Real native Source custody precedes arena allocation and survives cancellation at every partial opening.''','''//! 🏡️ Real native Source custody precedes arena allocation and survives cancellation at every partial opening; the Source's resident return table is born first, as every native recipient requires, so the measured lifetime conserves only its own allocations.''')
r("let mut caller=OriginalJobFixtureCaller::admit(OperationId(841),Generation(1),grant,clock,10000000,s[\"fuel\"].as_u64().unwrap());","let mut caller=OriginalJobFixtureCaller::admit(OperationId(841),Generation(1),grant,clock,10000000,s[\"fuel\"].as_u64().unwrap());caller.admit_return_table();")
r("caller.cancel().cancel_now();while lease.is_some(){caller.turn(grant,|cx|semio_framework_job::OriginalNativeLease::return_slot_step(&mut lease,cx)).unwrap();}",
  "caller.cancel().cancel_now();while lease.is_some(){let(result,heap)=observe(||caller.turn(grant,|cx|semio_framework_job::OriginalNativeLease::return_slot_step(&mut lease,cx)));result.unwrap();let receipt=caller.receipt();assert_eq!(heap,(receipt.retained_capacity_bytes,receipt.released_bytes));born+=heap.0;freed+=heap.1;}")
open(p,'w',encoding='utf-8').write(s)
