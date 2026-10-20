import sys, json, re
sys.path.insert(0, __file__.rsplit("/", 1)[0])
import armspec
F = sys.argv[2]
rows = [(a, "close_lane_%d" % n, "Closes lane %d of the close ladder." % n) for a, n in ((143, 7), (161, 8), (202, 11), (230, 12), (245, 13), (277, 15), (303, 19), (318, 20))]
CF = "std::ops::ControlFlow"
armspec.build(F, "close_step", 59, rows, "Result<%s<(bool, RetainedCloneProgress), (bool, usize, usize)>, semio_framework_job::OriginalTurnFailure>" % CF,
  method={"self_param": "&mut self", "call": "self.{name}({args})"}, types={"child": "RetainedCloneGrant", "maximum_bytes": "usize", "grant": "RetainedCloneGrant"}, out=sys.argv[1],
  flow_option=True, flow_wrap=CF + "::Break", wrap=["Ok(%s::Continue({" % CF, "}))"], call_suffix="?")
d = json.load(open(sys.argv[1], encoding="utf8"))
d["context"] = ["cx", "grant", "maximum_bytes", "child"]
d["call_wrap"] = "match {call} { %s::Continue(value) => value, %s::Break(step) => return Ok(step) }" % (CF, CF)
json.dump(d, open(sys.argv[1], "w", encoding="utf8"), ensure_ascii=False, indent=1)
