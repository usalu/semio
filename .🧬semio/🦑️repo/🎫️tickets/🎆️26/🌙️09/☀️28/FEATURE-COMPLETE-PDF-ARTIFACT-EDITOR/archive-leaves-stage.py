"""📂 Stages the three ordinary typed PDF archive leaves over the existing receiver."""

from argparse import ArgumentParser
from hashlib import sha256
import json
from pathlib import Path
import re
import runpy

TICKET = Path(__file__).resolve().parent
OBJECT = runpy.run_path(str(TICKET / "archive-object-stage.py"))
BASE = OBJECT["OUT"]
OUT = TICKET / "🗑️generated" / "archive-leaves"
change = OBJECT["change"]


def receiver(text):
    text = change(text, "use object::{ObjectReceiving, ObjectStep};", '''use object::{ObjectReceiving, ObjectStep};
#[path = "🌿️leaves/🦀️.rs"]
mod leaves;
use leaves::{Leaf, LeafReceiving, LeafStep};''')
    text = change(text, "BindObject, Ready", "BindObject, BindLeaf, Ready")
    text = change(text, "    object: ObjectReceiving,", "    object: ObjectReceiving,\n    leaf: LeafReceiving,")
    text = change(text, "object: Default::default(),", "object: Default::default(), leaf: Default::default(),")
    text = change(text, "                ReceivingPhase::BindObject\n            } else {",
        "                ReceivingPhase::BindObject\n            } else if Leaf::for_kind(self.frame.kind).is_some() {\n                ReceivingPhase::BindLeaf\n            } else {")
    text = change(text, "        if self.phase == ReceivingPhase::BindObject {\n            let (step, receipt) =",
        '''        if self.phase == ReceivingPhase::BindLeaf {
            let (step, receipt) = self.leaf.advance(
                Leaf::for_kind(self.frame.kind).expect("admitted PDF archive leaf"),
                self.value.value.as_mut().expect("retained PDF archive leaf"),
                &mut self.output.value, effect)?;
            match step {
                LeafStep::Pending => {},
                LeafStep::Ordinary => self.phase = ReceivingPhase::Bind,
                LeafStep::Complete => self.phase = ReceivingPhase::Ready,
            }
            return Ok(receipt);
        }
        if self.phase == ReceivingPhase::BindObject {
            let (step, receipt) =''')
    text = change(text, "            ReceivingPhase::BindObject =>",
        '''            ReceivingPhase::BindLeaf => self.leaf.demand(
                Leaf::for_kind(self.frame.kind).expect("admitted PDF archive leaf"),
                self.value.value.as_ref().expect("retained PDF archive leaf")).0,
            ReceivingPhase::BindObject =>''')
    return change(text, "        if self.phase == ReceivingPhase::BindObject {\n            return self.object.demand",
        '''        if self.phase == ReceivingPhase::BindLeaf {
            return Ok(self.leaf.demand(
                Leaf::for_kind(self.frame.kind).expect("admitted PDF archive leaf"),
                self.value.value.as_ref().expect("retained PDF archive leaf")).1);
        }
        if self.phase == ReceivingPhase::BindObject {
            return self.object.demand''')


def stage(workspace, verify):
    if not verify:
        OUT.mkdir(parents=True, exist_ok=True)
    rows = json.loads((BASE / "receipt.json").read_bytes())
    target = OBJECT["DONATION"]["RECEIVER"].parent / "🌿️leaves/🦀️.rs"
    rows[str(target)] = {"stage": "10.rs", "before": None, "after": None}
    for relative, row in rows.items():
        dest = workspace / relative
        before = sha256(dest.read_bytes()).hexdigest() if dest.is_file() else None
        assert before == row["before"], relative
        assert len(str(dest.resolve()).encode("utf-16-le")) // 2 <= 256, relative
        name = row["stage"]
        if name == "10.rs":
            data = (TICKET / "archive-leaves.rs").read_text(encoding="utf8")
        else:
            data = (BASE / name).read_text(encoding="utf8")
            assert sha256(data.encode()).hexdigest() == row["after"], relative
            if name == "5.rs":
                data = receiver(data)
            elif name == "6.rs":
                marker = "// Registered snapshot replay test."
                data = change(data, marker, (TICKET / "archive-leaves-test.rs").read_text(encoding="utf8") + "\n" + marker)
                data += "\n" + (TICKET / "archive-leaves-state-test.rs").read_text(encoding="utf8")
            elif name == "7.json":
                fixtures = json.loads(data)
                fixtures["ordinaryLeaves"] = json.loads((TICKET / "archive-leaves-laws.json").read_bytes())
                data = json.dumps(fixtures, ensure_ascii=False, indent=2) + "\n"
            elif name == "8.rs":
                for helper in ("scope_extra", "extra", "optional_string"):
                    data, count = re.subn(rf"(?m)^fn {helper}(?=[<(])", f"pub(super) fn {helper}", data)
                    assert count == 1, helper
        data = data.encode()
        row["after"] = sha256(data).hexdigest()
        if verify:
            assert (OUT / name).read_bytes() == data, name
        else:
            (OUT / name).write_bytes(data)
    if verify:
        assert json.loads((OUT / "receipt.json").read_bytes()) == rows
    else:
        (OUT / "receipt.json").write_text(json.dumps(rows, ensure_ascii=False, indent=2) + "\n", encoding="utf8")
    print(f"[DEBUG] {'verified' if verify else 'staged'} {len(rows)} ordinary archive paths")


if __name__ == "__main__":
    parser = ArgumentParser()
    parser.add_argument("workspace", type=Path)
    parser.add_argument("--verify", action="store_true")
    args = parser.parse_args()
    stage(args.workspace.resolve(), args.verify)
