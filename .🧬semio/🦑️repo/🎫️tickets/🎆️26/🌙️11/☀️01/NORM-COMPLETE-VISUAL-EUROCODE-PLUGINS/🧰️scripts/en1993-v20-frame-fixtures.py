"""Rewrites the committed mutation vectors of the frameActions / frameMemberBases kinds of en1993 from the tool's donor copies: every vector starts from the frame-node vector, carries one or two rows and ends in the exact after-snapshot and diff of its kind. Run: python3 -I en1993-v20-frame-fixtures.py"""
import copy, json, os

ROOT = "/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🔩️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations"
BASE = json.load(open(f"{ROOT}/🕸️change-frame-node-load-x/✅apply/🔣️.before.json"))
DIFF = json.load(open(f"{ROOT}/🕸️change-frame-node-load-x/✅apply/🔣️.diff.json"))
ACTION = {"id": "frame-action-1", "loadCaseId": "g-permanent", "targetId": "node-b", "kind": "NodalForce", "fx": 0.0, "fz": -100000.0, "my": 0.0, "startRatio": 0.0, "endRatio": 1.0}
SECOND = {**ACTION, "id": "frame-action-2"}
BASIS = {"id": "frame-member-basis-1", "memberId": "member-b1", "basis": "Explicit"}
BASIS2 = {**BASIS, "id": "frame-member-basis-2"}
ACTION_FIELDS = ["loadCaseId", "targetId", "kind", "fx", "fz", "my", "startRatio", "endRatio"]
BASIS_FIELDS = ["memberId", "basis"]
LANE = lambda **k: {"removed": k.get("removed", []), "inserted": k.get("inserted", []), "moved": [], "modified": k.get("modified", [])}


def write(directory, before, mutation, after, diff):
    base = f"{ROOT}/{directory}/✅apply"
    os.makedirs(base, exist_ok=True)
    for name, value in (("before", before), ("mutation", mutation), ("after", after), ("diff", diff), ("outcome", {"status": "applied"})):
        with open(f"{base}/🔣️.{name}.json", "w") as handle:
            json.dump(value, handle, indent=2, ensure_ascii=False)
            handle.write("\n")


def document(**lanes):
    d = copy.deepcopy(BASE)
    d.update(lanes)
    return d


def diff_of(**lanes):
    d = copy.deepcopy(DIFF)
    d["frameActions"] = None
    d["frameMemberBases"] = None
    d.update(lanes)
    return d


def change(directory, kind, lane, row, fields, field, value, wire, empty_other):
    before_rows, after_row = [copy.deepcopy(row)], {**row, field: value}
    patch = {f: (value if f == field else None) for f in fields}
    other = "frameMemberBases" if lane == "frameActions" else "frameActions"
    mutation = {kind: {"id": row["id"], wire: value}}
    write(directory, document(**{lane: before_rows, other: empty_other}), mutation, document(**{lane: [after_row], other: empty_other}), diff_of(**{lane: LANE(modified=[{"id": row["id"], "patch": patch}])}))


for directory, kind, field, wire, value in [
    ("🧱️change-frame-action-load-case-id", "ChangeFrameActionLoadCaseId", "loadCaseId", "newLoadCaseId", "q-imposed"),
    ("🧱️change-frame-action-target-id", "ChangeFrameActionTargetId", "targetId", "newTargetId", "node-c"),
    ("📏️change-frame-action-kind", "ChangeFrameActionKind", "kind", "newKind", "NodalMoment"),
    ("🧱️change-frame-action-fx", "ChangeFrameActionFx", "fx", "newFx", 2500.0),
    ("🧱️change-frame-action-fz", "ChangeFrameActionFz", "fz", "newFz", -150000.0),
    ("🧱️change-frame-action-my", "ChangeFrameActionMy", "my", "newMy", 5000.0),
    ("🧱️change-frame-action-start-ratio", "ChangeFrameActionStartRatio", "startRatio", "newStartRatio", 0.25),
    ("🧱️change-frame-action-end-ratio", "ChangeFrameActionEndRatio", "endRatio", "newEndRatio", 0.75),
]:
    change(directory, kind, "frameActions", ACTION, ACTION_FIELDS, field, value, wire, [])
for directory, kind, field, wire, value in [
    ("🧱️change-frame-member-basis-member-id", "ChangeFrameMemberBasisMemberId", "memberId", "newMemberId", "member-b2"),
    ("📏️change-frame-member-basis-basis", "ChangeFrameMemberBasisBasis", "basis", "newBasis", "FrameDerived"),
]:
    change(directory, kind, "frameMemberBases", BASIS, BASIS_FIELDS, field, value, wire, [])

for lane, row, second, collection, wrap, remove in [("frameActions", ACTION, SECOND, "frameAction", "FrameAction", "frame-action"), ("frameMemberBases", BASIS, BASIS2, "frameMemberBasis", "FrameMemberBasis", "frame-member-basis")]:
    other = "frameMemberBases" if lane == "frameActions" else "frameActions"
    write(f"➕️insert-{remove}", document(**{lane: [row], other: []}), {f"Insert{wrap}": {"index": 1, collection: second}}, document(**{lane: [row, second], other: []}), diff_of(**{lane: LANE(inserted=[{"index": 1, "row": second}])}))
    write(f"➖️remove-{remove}", document(**{lane: [row, second], other: []}), {f"Remove{wrap}": {"index": 1}}, document(**{lane: [row], other: []}), diff_of(**{lane: LANE(removed=[{"id": second["id"], "index": 1}])}))
