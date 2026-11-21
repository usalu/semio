"""⏪️ Stages the PDF/VT history replay owner without changing live sources."""

from pathlib import Path
import hashlib
import json
import sys

TICKET=Path(__file__).resolve().parent
OUT=TICKET/"🗑️generated"/"vt-replay"
VT=Path("🏅️standards/7️⃣1.7/🪆️subsets/🧾️vt/✏️editor/🦀️.rs")
QUOTE="""    fn document_store_owners_source_demands() -> Result<semio_plugin::__value::RetirementDemand, semio_plugin::__value::ValueError> {
        semio_plugin::cold_replay_document_store_owners_source_demands::<Self::Snapshot, Self::Mutation>()
    }

"""
CATALOG="        document_store_owners: semio_plugin::cold_replay_document_store_owners::<PdfSnapshot, PdfMutation>,\n"


def candidates(base):
    original=(base/VT).read_bytes()
    stable=original.decode("utf8")
    assert stable.count("    const SINGLE_TURN_NATURAL_DECODE") == 1
    assert stable.count("        bounded_native: true") == 1
    staged=stable.replace("    const SINGLE_TURN_NATURAL_DECODE", QUOTE+"    const SINGLE_TURN_NATURAL_DECODE", 1)
    staged=staged.replace("        bounded_native: true", CATALOG+"        bounded_native: true", 1)
    return original, staged.encode("utf8")


def stage(base, action):
    original, candidate = candidates(base)
    expected={"relative":VT.as_posix(),"before":hashlib.sha256(original).hexdigest(),"after":hashlib.sha256(candidate).hexdigest()}
    output=OUT/"vt-editor.rs"
    receipt=OUT/"receipt.json"
    if action=="--prepare":
        OUT.mkdir(parents=True, exist_ok=True)
        output.write_bytes(candidate)
        receipt.write_text(json.dumps(expected,ensure_ascii=False)+"\n", encoding="utf8")
    else:
        assert output.read_bytes()==candidate and json.loads(receipt.read_bytes())==expected
        assert len(str((base/VT).resolve()).encode("utf-16-le"))//2<=256
        assert (base/VT).read_bytes()==original
        if action=="--apply-reviewed":
            (base/VT).write_bytes(candidate)
    print(f"[DEBUG] {action} checked PDF/VT replay owner")


if __name__=="__main__":
    assert sys.argv[2] in ("--prepare","--verify","--apply-reviewed")
    stage(Path(sys.argv[1]),sys.argv[2])
