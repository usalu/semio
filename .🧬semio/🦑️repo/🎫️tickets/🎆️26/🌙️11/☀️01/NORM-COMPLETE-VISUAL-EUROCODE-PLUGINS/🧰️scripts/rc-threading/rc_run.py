"""Shared driver steps of the rc-threading v2 tool: schema twins, leaf kind, fixtures and DSL assets of one artifact."""
import sys
sys.path.insert(0, ".")
from rc_lib import *
import rc_lib, rc_leaf, json

def proto_clone_inline(path):
    text = rd(path)
    if "reliability_class" in text:
        return False
    m = re.search(r"(?P<opt>optional )?(?P<ty>\w+) annex = (?P<n>\d+);", text)
    assert m, path[-60:]
    start = text.rfind("message ", 0, m.start())
    end = balanced(text, start, "{", "}")
    nums = [int(x) for x in re.findall(r"= (\d+);", text[start:end])]
    n = max(nums) + 1
    wr(path, text[:m.end()] + f" {m.group('opt') or ''}string reliability_class = {n};" + text[m.end():])
    return True

def graphql_any(path, expected=1):
    text = rd(path)
    if "reliabilityClass:" in text:
        return False
    if re.search(r"^  annex: ", text, re.M):
        return graphql_clone(path, expected)
    new, n = re.subn(r"(\bannex: )AnnexChoice(!?)( @state\(class: ARTIFACT\) )", r"\1AnnexChoice\2\3reliabilityClass: String\2\3", text)
    new = re.sub(r"(\bannex: )String(!?)( @state\(class: ARTIFACT\) )(?!reliabilityClass)", r"\1String\2\3reliabilityClass: String\2\3", text) if n == 0 else new
    assert new != text, path[-60:]
    wr(path, new)
    return True

def twins(S, json_counts=(1, 1, 1), gql=(1, 1, 1), ts_iface=(1, 1, 1), ts_parser=(1, 1, 1)):
    roots = ["🔣️.json", "📸️snapshot/🔣️.json"]
    for rel in roots:
        json_clone_property(f"{S}/{rel}", 1, converter=json_rc_enum)
        json_required_after_annex(f"{S}/{rel}", 1)
    json_clone_property(f"{S}/🔣️.diff.json", 1, converter=json_rc_enum)
    for rel in roots + ["🔣️.diff.json"]:
        json_reorder_last(f"{S}/{rel}")
    for rel in ["🔗️.graphql", "📸️snapshot/🔗️.graphql"]:
        graphql_any(f"{S}/{rel}", 1)
    graphql_any(f"{S}/🔺️diff/🔗️.graphql", 1)
    for rel in ["🛰️.proto", "📸️snapshot/🛰️.proto", "🔺️diff/🛰️.proto"]:
        p = f"{S}/{rel}"
        if re.search(r"^  (optional )?\w+ annex = \d+;$", rd(p), re.M):
            proto_clone(p)
        else:
            proto_clone_inline(p)
    for rel in ["🟦️.ts", "📸️snapshot/🟦️.ts"]:
        ts_clone(f"{S}/{rel}", False)
    ts_clone(f"{S}/🔺️diff/🟦️.ts", True)
    for rel in ["🟦️.ts", "📸️snapshot/🟦️.ts", "🔺️diff/🟦️.ts"]:
        twin_last_ts(f"{S}/{rel}")
    for rel in ["🔗️.graphql", "📸️snapshot/🔗️.graphql", "🔺️diff/🔗️.graphql"]:
        twin_last_graphql(f"{S}/{rel}")
    for rel in ["🛰️.proto", "📸️snapshot/🛰️.proto", "🔺️diff/🛰️.proto"]:
        twin_last_proto(f"{S}/{rel}")

def leaf(R, S, artdir, art, Art):
    M = S + "/🧬️mutations"
    ad = rc_leaf.find_annex_dir(M)
    tag = rc_leaf.leaf_sources(R, art, Art, M, ad)
    rc_leaf.fixtures(R + "/🧪️tests/🧫️fixtures/🧬️mutations", ad)
    fp = rc_leaf.footprint(R, ad)
    rc_leaf.line_register(fp["line"], rc_leaf.lc)
    for p in fp["mount"]:
        rc_leaf.mount_register(p)
    for p in fp["proto"]:
        rc_leaf.proto_register(p)
    for p in fp["graphql"]:
        rc_leaf.graphql_register(p)
    for p in fp["semio"]:
        rc_leaf.protocol_semio_register(p, tag)
    for p in fp["json"]:
        (rc_leaf.oracles_register if "oracles" in p else rc_leaf.mutations_json_register)(p)
    rc_leaf.taxonomy_register(artdir, ad)
    for dp, _, names in os.walk(R + "/🧪️tests"):
        if "🚦️check-outcome" in dp:
            for n in names:
                if n.endswith(".feature"):
                    q = f"{dp}/{n}"
                    wr(q, re.sub(r"^[^\n]*change-reliability-class/✅apply/📸️snapshot/➡️after[^\n]*\n", "", rd(q), flags=re.M))
    return fp

def fixtures(R, S, assets):
    sch = json.load(open(f"{S}/📸️snapshot/🔣️.json"))
    dif = json.load(open(f"{S}/🔣️.diff.json"))
    req = set(sch["required"]) - {"reliabilityClass"}
    n = fixture_sweep(R, req, set(dif["properties"]) - {"reliabilityClass"})
    print("fixtures", n)
    for a in assets:
        if os.path.exists(f"{R}/🖼️assets/{a}/{a}/🗣️.dsl.semio"):
            dsl_asset_insert(f"{R}/🖼️assets/{a}/{a}/🗣️.dsl.semio")

def finish(dry_print=True):
    for p in [x for x in CHANGED if "fixtures/" in x]:
        CHANGED.remove(p)
    if dry_print:
        for p in CHANGED:
            show(p)
    print("changed files", len(CHANGED))
