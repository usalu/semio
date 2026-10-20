import sys
sys.path.insert(0, __file__.rsplit("/", 1)[0])
import armspec
F = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🔨️modules/🏠️host/🧰️owned/🔄️initialization-turn/🦀️.rs"
B = lambda **k: {"binds": k}
rows = [
 (54, "turn_admit_owners", "Admits the arena owners of the initialization."),
 (137, "turn_start_initial_clone", "Starts the clone of the initial snapshot."),
 (219, "turn_seed_history", "Seeds one history lane edit.", B(edit="usize", lane="u8", index="usize")),
 (268, "turn_find_applied", "Finds the next applied edit of the history.", B(position="usize")),
 (289, "turn_apply_forward", "Applies one forward mutation of an applied edit.", B(position="usize", edit="usize", mutation="usize")),
 (361, "turn_return_forward", "Returns the forward mutation of an applied edit.", B(position="usize", edit="usize", mutation="usize")),
 (412, "turn_find_redo", "Finds the next redo edit of the history.", B(position="usize")),
]
armspec.build(F, "turn", 44, rows, "DrawingInitializationTurn", method={"self_param": "&mut self", "call": "self.{name}({args})"}, out=sys.argv[1])
