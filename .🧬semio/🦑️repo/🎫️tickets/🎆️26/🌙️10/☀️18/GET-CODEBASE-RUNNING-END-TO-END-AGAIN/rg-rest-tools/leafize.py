"""Leafize helpers: turn leaf impls of `MutationKind<S, Agg>` into impls generic over the dispatching aggregate (`Op`), split into `MutationKind` + `ColdMutationKind`."""
import sys, re
sys.path.insert(0, "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/rg-rest-tools")
from lib import *

def find_impl(text, trait_re):
    """locate `impl <trait_re> for X {` ; return (start, header_end_idx_of_open_brace, close_idx_after, X)"""
    m = re.search(r'(?m)^impl\s+' + trait_re + r'\s+for\s+([A-Za-z0-9_]+)\s*\{', text)
    if not m:
        return None
    o = m.end() - 1
    c = block_bounds(text, o)
    return m.start(), o, c, m.group(1)

def split_methods(body):
    """body text between braces -> dict name -> text (fn blocks), 'const' -> const text, plus order list"""
    out = {}
    order = []
    i = 0
    n = len(body)
    items = top_items(body)
    for it in items:
        s = it.strip("\n")
        if not s.strip():
            continue
        m = re.search(r'(?m)^\s*(?:pub\s+)?fn\s+([a-z_0-9]+)', s)
        if m and not s.lstrip().startswith("const"):
            out[m.group(1)] = it
            order.append(m.group(1))
        else:
            mc = re.search(r'(?m)^\s*const\s+([A-Z_]+)', s)
            key = mc.group(1) if mc else "other%d" % len(order)
            out[key] = it
            order.append(key)
    return out, order

def dedent_fn(text, indent="    "):
    return text

def generic_impls(text, snap, diff, leaf_trait_agg, extra_from=(), snap_alias=None):
    """Rewrite the `impl protocol::MutationKind<snap, Agg> for X` block in text. Returns new text, X."""
    f = find_impl(text, r'protocol::MutationKind<' + re.escape(snap) + r',\s*' + re.escape(leaf_trait_agg) + r'>')
    assert f, "no MutationKind impl"
    start, o, c, x = f
    body = text[o + 1:c - 1]
    methods, order = split_methods(body)
    hot = [k for k in order if k not in ("diff", "inverse")]
    cold = [k for k in order if k in ("diff", "inverse")]
    froms = "".join(" + From<%s>" % e for e in extra_from)
    new = "impl<Op> protocol::MutationKind<%s, Op> for %s\nwhere\n    Op: protocol::Mutation<%s, Diff = %s> + From<%s>,\n{%s}\n" % (snap, x, snap, diff, x, "".join(methods[k] if methods[k].startswith("\n") else "\n" + methods[k] for k in hot).rstrip() + "\n")
    if cold:
        cold_body = "".join(methods[k] if methods[k].startswith("\n") else "\n" + methods[k] for k in cold)
        cold_body = cold_body.replace("-> protocol::MutationOutcome<<%s as Mutation<%s>>::Diff>" % (leaf_trait_agg, snap), "-> protocol::MutationOutcome<%s>" % diff)
        cold_body = cold_body.replace("Result<Vec<%s>, semio_framework_value::ValueError>" % leaf_trait_agg, "Result<Vec<Op>, semio_framework_value::ValueError>")
        cold_body = cold_body.replace("Result<Vec<%s>, ValueError>" % leaf_trait_agg, "Result<Vec<Op>, semio_framework_value::ValueError>")
        new += "\nimpl<Op> protocol::ColdMutationKind<%s, Op> for %s\nwhere\n    Op: protocol::ColdMutation<%s, ReferenceDiff = %s> + From<%s>%s,\n{%s}\n" % (snap, x, snap, diff, x, froms, cold_body.rstrip() + "\n")
    return text[:start] + new + text[c:], x
