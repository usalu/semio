import sys, json
sys.path.insert(0, __file__.rsplit("/", 1)[0])
import armspec
F = sys.argv[2]
lines = [int(x) for x in sys.argv[3].split(",")]
CT = "store::mounted_pack_rt::RetainedValueContainer"
rows = [
 (lines[0], "accept_begin_table", "Opens a table container of the retained record.", {"binds": {"count": "u64"}}),
 (lines[1], "accept_begin_list_or_tuple", "Opens a list or tuple container of the retained record.", {"binds": {"count": "u64"}}),
 (lines[2], "accept_field_id", "Binds the field id of the retained record.", {"binds": {"value": "u64"}}),
 (lines[3], "accept_f64", "Accepts the bits of one float of the retained record.", {"binds": {"bits": "u64"}}),
 (lines[4], "accept_inline_tag", "Accepts the inline tag of a retained flag or bit mask.", {}),
 (lines[5], "accept_wire_node_presence", "Accepts the presence marker of one wire node.", {"binds": {"presence": "u8"}}),
 (lines[6], "accept_end", "Closes the innermost container of the retained record.", {"binds": {"kind": CT}}),
 (lines[7], "accept_complete", "Completes the retained record and publishes its destination.", {}),
]
s = armspec.build(F, "accept", 9, rows, "Result<(), &'static str>", method={"self_param": "&mut self", "call": "self.{name}({args})"}, out=sys.argv[1],
  fall_tail="Ok(())", ensure_semicolon=True, call_suffix="?",
  prelude_ifs=[{"regex": r"\bContainer\b", "line": "use store::mounted_pack_rt::RetainedValueContainer as Container;"},
               {"regex": r"\bRole\b", "line": "use store::mounted_pack_rt::RetainedValueRole as Role;"},
               {"regex": r"\bToken\b", "line": "use store::mounted_pack_rt::RetainedValueToken as Token;"}])
