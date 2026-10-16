"""✂️ Moves the supersede-fold fixture laws of the history crate into an integration target that uses the public API only."""
import re
from rlib import *
import rcrates as C
import rewrite

UNIT = R + "🔗️causal/🔀️transition/🧪️tests/🔬️unit/🦀️.rs"
LAWS = R + "🔗️causal/🔀️transition/🧪️tests/🔬️supersede-fold/🦀️.rs"
MOVE = ["expected_supersessions", "the_supersede_fold_fixture_matches_the_fold_law", "supersessions_are_a_function_of_the_event_set"]
COPY = ["fixture_transition", "fixture_hex", "fixture_replacement", "head", "none", "supersede_fold_fixture", "fixture_step_envelope"]
TARGET = {"name": "history_supersede_fold", "path": "../../🧪️tests/🔬️supersede-fold/🦀️.rs"}

def run():
    t = read_stage_or_base(UNIT)
    rest, moved = cut_items(t, MOVE)
    copies = [top_defs(t)[n]["text"] for n in COPY]
    body = "\n\n".join(c.strip("\n") for c in copies + moved)
    model = rewrite.Model({})
    hist = model.worlds["history"]
    ids_w = model.worlds["ids"]
    used = bare_ids(body)
    hist_names = sorted(n for n in used if hist.lookup(hist.root, n, mode=2) is not None and n not in COPY and n not in MOVE)
    ids_names = sorted(n for n in used if ids_w.lookup(ids_w.root, n, mode=2) is not None and n not in hist_names)
    head = "//! 🧪️ Supersede fold laws through the public API of the history crate only.\n\n"
    if hist_names: head += "use " + C.ident("history") + "::{" + ", ".join(hist_names) + "};\n"
    if ids_names: head += "use " + C.ident("ids") + "::{" + ", ".join(ids_names) + "};\n"
    write(UNIT, re.sub(r"\n{3,}", "\n\n", rest))
    write(LAWS, head + "\n" + body + "\n")
    return hist_names, ids_names

if __name__ == "__main__":
    print(run())
