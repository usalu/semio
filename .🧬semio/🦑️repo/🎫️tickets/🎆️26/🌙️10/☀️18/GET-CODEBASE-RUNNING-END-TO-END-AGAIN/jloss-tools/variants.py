"""🧪 Builds the test-only watcher variants of jloss from one watcher source: noqos (default dispatch QoS), slow (noqos + 3 ms per callback, forces UserDropped), baseslow (same delay in a base source). Usage: python3 -I variants.py <new.rs> <base.rs> <outdir>"""
import sys
new, base, out = sys.argv[1:4]
CB = "        let paths = paths as *const *const c_char;\n        let mut changes: BTreeMap<String, Change> = BTreeMap::new();"
SLOW = "        std::thread::sleep(std::time::Duration::from_millis(3));\n"
QOS = "dispatch_queue_attr_make_with_qos_class(std::ptr::null(), QOS_CLASS_USER_INITIATED, 0)"
s = open(new).read()
assert s.count(QOS) == 1 and s.count(CB) == 1
noqos = s.replace(QOS, "std::ptr::null()")
open(out + "/noqos.rs", "w").write(noqos)
open(out + "/slow.rs", "w").write(noqos.replace(CB, SLOW + CB))
b = open(base).read()
assert b.count(CB) == 1
open(out + "/baseslow.rs", "w").write(b.replace(CB, SLOW + CB))
