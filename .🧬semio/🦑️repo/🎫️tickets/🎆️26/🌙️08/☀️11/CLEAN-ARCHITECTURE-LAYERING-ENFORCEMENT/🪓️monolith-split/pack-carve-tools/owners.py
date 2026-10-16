"""🗺️ Which carved crate owns each name that used to live under the old `pack` module paths."""
import re
from lib import *
from names import codec_flat, pub_names
import usetree
from crates import *

PACKERR = {"PackError","PackRefusal","PackRetryDisposition","PackTransportCategory","PackTransportContext","PackTransportPolicy","PackTransportProgress","PackTransportPhase","TransportAdmission","TransportContextRefusal","TransportContextRefusalCause"}
CODEC = codec_flat() - {"PackRefusal"}

def all_defined(text):
    """Every top-level defined name (any visibility) in a text."""
    out = set()
    for it in items(text):
        if it["kind"] in ("fn","struct","enum","const","static","type","trait","mod","macro_rules","union") and it["name"]:
            out.add(it["name"])
    return out

def format_owners():
    import fmtsplit
    base = read_base(P + "📐️format/🦀️.rs")
    before, after, inner = fmtsplit.region(base)
    core = before + "\n" + after
    ret_items, cat_items = fmtsplit.partition(inner)
    owners = {}
    for n in all_defined(core) | pub_names(core): owners[n] = "format"
    for t in ret_items: owners.update({n: "retained" for n in all_defined(t)})
    for t in cat_items: owners.update({n: "catalog" for n in all_defined(t)})
    owners["retained_inline_symbols"] = "catalog"
    owners["retained_page_close_demands"] = "retained"
    return owners
