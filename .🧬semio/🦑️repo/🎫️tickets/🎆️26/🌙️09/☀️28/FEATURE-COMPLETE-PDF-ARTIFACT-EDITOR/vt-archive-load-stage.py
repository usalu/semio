"""⏪ Stages the mounted PDF/VT replay hooks and committed archive acceptance."""

from argparse import ArgumentParser
from hashlib import sha256
import json
from pathlib import Path
import runpy

TICKET = Path(__file__).resolve().parent
OUT = TICKET / "🗑️generated" / "vt-load"
PDF = Path("✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf")
VT = PDF / "🏅️standards/7️⃣1.7/🪆️subsets/🧾️vt"
EDITOR = VT / "✏️editor/🦀️.rs"
VIEWER = VT / "👁️viewer/🦀️.rs"
TEST = VT / "👁️viewer/🧪️tests/🔬️unit/🦀️.rs"
EDITOR_OWNER = runpy.run_path(str(TICKET / "📜️vt-replay-owner-stage.py"))

VIEWER_OWNER = '''    fn document_store_owners_source_demands() -> Result<semio_plugin::__value::RetirementDemand, semio_plugin::__value::ValueError> {
        semio_plugin::cold_replay_document_store_owners_source_demands::<Self::Snapshot, Self::Mutation>()
    }

    fn build_document_store_owners(
        grant: semio_plugin::__value::RetainedCloneGrant,
    ) -> Option<Result<(
        protocol::os_store::document_store_owners::DocumentStoreOwners<Self::Snapshot, Self::Mutation>,
        semio_plugin::__value::RetainedCloneProgress,
    ), protocol::os_store::document_store_owners::DocumentStoreOwnersAdmissionError<Self::Snapshot, Self::Mutation>>> {
        Some(semio_plugin::cold_replay_document_store_owners::<Self::Snapshot, Self::Mutation>(grant))
    }

'''


def stage(workspace, verify):
    before_editor, editor = EDITOR_OWNER["candidates"](workspace / PDF)
    before_viewer = (workspace / VIEWER).read_bytes()
    viewer = before_viewer.decode("utf8")
    marker = "    fn reference_birth()"
    assert viewer.count(marker) == 1
    viewer = viewer.replace(marker, VIEWER_OWNER + marker, 1).encode("utf8")
    before_tests = (workspace / TEST).read_bytes()
    tests = before_tests + b"\n" + (TICKET / "vt-committed-archive-test.rs").read_bytes()
    if not verify:
        OUT.mkdir(parents=True, exist_ok=True)
    receipt = {}
    for name, path, before, staged in [
        ("editor.rs", EDITOR, before_editor, editor),
        ("viewer.rs", VIEWER, before_viewer, viewer),
        ("tests.rs", TEST, before_tests, tests),
    ]:
        current = workspace / path
        assert len(str(current.resolve()).encode("utf-16-le")) // 2 <= 256
        assert current.read_bytes() == before
        path = path.as_posix()
        receipt[path] = {"stage": name, "before": sha256(before).hexdigest(), "after": sha256(staged).hexdigest()}
        candidate = OUT / name
        if verify:
            assert candidate.read_bytes() == staged, path
        else:
            candidate.write_bytes(staged)
    proof = OUT / "receipt.json"
    if verify:
        assert json.loads(proof.read_text(encoding="utf8")) == receipt
    else:
        proof.write_text(json.dumps(receipt, ensure_ascii=False, indent=2) + "\n", encoding="utf8")
    print(f"[DEBUG] {'verified' if verify else 'staged'} {len(receipt)} PDF/VT replay paths")


if __name__ == "__main__":
    parser = ArgumentParser()
    parser.add_argument("workspace", type=Path)
    parser.add_argument("--verify", action="store_true")
    args = parser.parse_args()
    stage(args.workspace.resolve(), args.verify)
