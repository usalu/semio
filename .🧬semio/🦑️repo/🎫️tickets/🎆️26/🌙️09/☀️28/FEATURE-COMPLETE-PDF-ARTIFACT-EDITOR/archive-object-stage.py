"""📂 Stages Original receiving for simple typed COS insertion and removal."""

from argparse import ArgumentParser
from hashlib import sha256
import json
from pathlib import Path
import re
import runpy

TICKET = Path(__file__).resolve().parent
OUT = TICKET / "🗑️generated" / "archive-object"
METADATA = runpy.run_path(str(TICKET / "set-image-metadata-stage.py"))
BASE = METADATA["OUT"]
DONATION = METADATA["DONATION"]
change = METADATA["change"]


def receiver(text):
    text = change(text, "use metadata::{MetadataReceiving, MetadataStep};", '''use metadata::{MetadataReceiving, MetadataStep};
#[path = "📦️object/🦀️.rs"]
mod object;
use object::{ObjectReceiving, ObjectStep};''')
    text = change(text, "Bind, BindMetadata, Ready", "Bind, BindMetadata, BindObject, Ready")
    text = change(text, "    metadata: MetadataReceiving,", "    metadata: MetadataReceiving,\n    object: ObjectReceiving,")
    text = change(text, "metadata: Default::default(),", "metadata: Default::default(), object: Default::default(),")
    text = change(text, "            self.phase = if self.samples.value.is_some() { ReceivingPhase::BindMetadata } else { ReceivingPhase::Bind };",
        '''            self.phase = if self.samples.value.is_some() {
                ReceivingPhase::BindMetadata
            } else if matches!(self.frame.kind, Some("insertObject" | "removeObject")) {
                ReceivingPhase::BindObject
            } else {
                ReceivingPhase::Bind
            };''')
    text = change(text, "        if self.phase == ReceivingPhase::BindMetadata {\n            let (step, receipt) =",
        '''        if self.phase == ReceivingPhase::BindObject {
            let (step, receipt) = self.object.advance(
                self.frame.kind == Some("insertObject"),
                self.value.value.as_mut().expect("retained PDF archive object"),
                &mut self.output.value, effect)?;
            match step {
                ObjectStep::Pending => {},
                ObjectStep::Ordinary => self.phase = ReceivingPhase::Bind,
                ObjectStep::Complete => self.phase = ReceivingPhase::Ready,
            }
            return Ok(receipt);
        }
        if self.phase == ReceivingPhase::BindMetadata {
            let (step, receipt) =''')
    text = change(text, "            ReceivingPhase::BindMetadata => self.metadata.demand",
        '''            ReceivingPhase::BindObject => self.object.demand(self.value.value.as_ref().expect("retained PDF archive object"))?.0,
            ReceivingPhase::BindMetadata => self.metadata.demand''')
    text = change(text, "        if self.phase == ReceivingPhase::BindMetadata {\n            return self.metadata.demand",
        '''        if self.phase == ReceivingPhase::BindObject {
            return self.object.demand(self.value.value.as_ref().expect("retained PDF archive object")).map(|quote| quote.1);
        }
        if self.phase == ReceivingPhase::BindMetadata {
            return self.metadata.demand''')
    return text


def stage(workspace, verify):
    if not verify:
        OUT.mkdir(parents=True, exist_ok=True)
    base = json.loads((BASE / "receipt.json").read_text(encoding="utf8"))
    target = DONATION["RECEIVER"].parent / "📦️object/🦀️.rs"
    base[str(target)] = {"stage": "9.rs", "before": None, "after": None}
    helpers = ("array", "boolean", "named", "named_mut", "optional", "record", "reserve",
               "sequence", "small_array", "small_record", "string", "unsigned")
    for relative, row in base.items():
        path = workspace / relative
        before = sha256(path.read_bytes()).hexdigest() if path.is_file() else None
        assert before == row["before"], relative
        assert len(str(path.resolve()).encode("utf-16-le")) // 2 <= 256, relative
        name = row["stage"]
        if name == "9.rs":
            text = (TICKET / "archive-object.rs").read_text(encoding="utf8")
        else:
            text = (BASE / name).read_text(encoding="utf8")
            assert sha256(text.encode()).hexdigest() == row["after"], relative
            if name == "5.rs":
                text = receiver(text)
            elif name == "8.rs":
                for helper in helpers:
                    text, count = re.subn(rf"(?m)^fn {helper}(?=[<(])", f"pub(super) fn {helper}", text)
                    assert count == 1, helper
            elif name == "6.rs":
                text += "\n" + (TICKET / "archive-object-test.rs").read_text(encoding="utf8")
                text += "\n" + (TICKET / "archive-object-state-test.rs").read_text(encoding="utf8")
            elif name == "7.json":
                cases = json.loads(text)
                cases["objects"] = json.loads((TICKET / "archive-object-laws.json").read_text(encoding="utf8"))
                cases["archive"] = json.loads((TICKET / "archive-cases.json").read_text(encoding="utf8"))
                text = json.dumps(cases, ensure_ascii=False, indent=2) + "\n"
        data = text.encode()
        row["after"] = sha256(data).hexdigest()
        if verify:
            assert (OUT / name).read_bytes() == data
        else:
            (OUT / name).write_bytes(data)
    receipt = OUT / "receipt.json"
    if verify:
        assert json.loads(receipt.read_bytes()) == base
    else:
        receipt.write_text(json.dumps(base, ensure_ascii=False, indent=2) + "\n", encoding="utf8")
    print(f"[DEBUG] {'verified' if verify else 'staged'} {len(base)} archive object paths")


if __name__ == "__main__":
    parser = ArgumentParser()
    parser.add_argument("workspace", type=Path)
    parser.add_argument("--verify", action="store_true")
    args = parser.parse_args()
    stage(args.workspace.resolve(), args.verify)
