"""Process-local patch of lib after the kind-convention rename: set-case-number/text are change-case-number/text."""
import lib
for old, new, emoji in (("set-case-number", "change-case-number", "🔢️"), ("set-case-text", "change-case-text", "🔤️")):
    lib.LEAF_DIRS = {(new if k == old else k): (emoji + new if k == old else v) for k, v in lib.LEAF_DIRS.items()}
    lib.KINDS[:] = [new if k == old else k for k in lib.KINDS]
    lib.RECORD = {(new if k == old else k): ("".join(w.capitalize() for w in (new if k == old else k).split("-"))) for k in lib.RECORD}
    lib.MODULE = {(new if k == old else k): (new if k == old else k).replace("-", "_") for k in lib.MODULE}
