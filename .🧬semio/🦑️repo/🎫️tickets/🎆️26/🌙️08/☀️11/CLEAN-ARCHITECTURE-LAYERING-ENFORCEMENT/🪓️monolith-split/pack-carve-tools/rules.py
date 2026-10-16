"""📐 Path rewrite rules from the old pack-root namespace into the carved crates."""
import re
from lib import *
from owners import *
from crates import *
from paths import rewrite_crate_paths

class Rules:
    def __init__(self, owners_by_module):
        """owners_by_module: {'format': {name: crateKey}, 'record': {...}, ...} for names reachable as crate::<module>::name."""
        self.ob = owners_by_module

    def rule_for(self, here):
        ob = self.ob
        def rule(segs):
            s0 = segs[0]
            if s0 == "codec": return (1, "protocol::codec")
            if s0 == "value": return (1, "protocol::value")
            if s0 in CODEC: return (1, "protocol::codec::" + s0)
            if s0 in PACKERR: return (1, "semio_framework_pack_error::" + s0)
            if s0 == "test_allocation": return None
            if s0 in ("format", "record") and len(segs) >= 2:
                owner = ob[s0].get(segs[1])
                if owner is None: raise KeyError((s0, segs[1], here))
                return (2, "crate::" + segs[1] if owner == here else ident(owner) + "::" + segs[1])
            if s0 in ("format", "record"):
                return None
            if s0 in ("async_", "http", "io", "intrinsic"):
                key = {"async_": "async", "http": "http", "io": "io", "intrinsic": "intrinsic"}[s0]
                if key == here: return (1, "crate")
                return (1, ident(key))
            return None
        return rule

    def rewrite(self, text, here):
        return rewrite_crate_paths(text, self.rule_for(here))

import usetree
from lib import map_leaves

USE_RX = re.compile(r"(?m)(\bpub(?:\([^)]*\))?\s+)?\buse\s+(crate::[^;]*);")

def _rewrite_uses(self, text, here):
    rule = self.rule_for(here)
    def sub(m):
        vis = (m.group(1) or "")
        leaves = usetree.parse(m.group(2))
        mapped = map_leaves(leaves, rule)
        lines = usetree.emit(mapped)
        return " ".join((vis + l) if vis else l for l in lines)
    return USE_RX.sub(sub, text)

def _rewrite(self, text, here):
    return rewrite_crate_paths(_rewrite_uses(self, text, here), self.rule_for(here))

Rules.rewrite = _rewrite
