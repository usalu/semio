"""🧱 Base documents of mutation fixtures: the vectors of a new collection start from the artifact's own valid sample snapshot with the rows of the new collection inserted, never from the document of the donor kind's vectors."""
import copy
from core import Unsupported


def sample(ctx):
    """📗 The committed sample snapshot of the artifact (same document the SQLite contract measures), a complete snapshot with every lane of the schema."""
    import l_extent
    doc = l_extent.sample_document(ctx)
    props = set(ctx.snapshot["properties"])
    own = {ctx.spec.collection.names.plural_camel} if ctx.spec.collection else set()
    if not isinstance(doc, dict) or not set(doc) >= props - own:
        raise Unsupported("fixtures: the artifact has no complete sample snapshot to build the mutation vectors on")
    return doc


def base(ctx, lane):
    """📘 A deep copy of the sample snapshot where `lane` (the new collection) is empty."""
    doc = copy.deepcopy(sample(ctx))
    doc.setdefault(lane, [])
    return doc


def with_rows(ctx, lane, rows):
    """📙 The base snapshot with the rows of the new collection."""
    doc = base(ctx, lane)
    doc[lane] = copy.deepcopy(rows)
    return doc


def empty_lane(value):
    """🪹 The untouched form of a diff lane: an empty delta stays an empty delta, a list an empty list, anything else (`null` of optional lanes) `null`."""
    if isinstance(value, dict) and value and all(isinstance(v, list) for v in value.values()):
        return {k: [] for k in value}
    return [] if isinstance(value, list) else None


def diff_of(donor_diff, lane, rows):
    """📕 A diff document where every lane is untouched except `lane` (the new collection), which carries `rows` in the form the donor lane has; the donor's lane order is kept and the new lane follows the last lane."""
    out = {k: empty_lane(v) for k, v in donor_diff.items()}
    out[lane] = rows
    return out
