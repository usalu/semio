"""📋 Stages exact controlled metadata binding for the SetImage donation slice."""

from argparse import ArgumentParser
from hashlib import sha256
import json
from pathlib import Path
import runpy

TICKET = Path(__file__).resolve().parent
OUT = TICKET / "🗑️generated" / "set-metadata"
DONATION = runpy.run_path(str(TICKET / "set-image-donation-stage.py"))
BASE = DONATION["OUT"]
change = DONATION["change"]


def receiver(text):
    text = change(text, "use super::{BINARY_TAG_REGISTRY, MAX_PAYLOAD_BYTES, PdfMutation};",
        '''use super::{BINARY_TAG_REGISTRY, MAX_PAYLOAD_BYTES, PdfMutation};
#[path = "🧾️metadata/🦀️.rs"]
mod metadata;
use metadata::{MetadataReceiving, MetadataStep};''')
    text = change(text, "SamplesTake, Identity(usize), Bind, Ready", "SamplesTake, Identity(usize), Bind, BindMetadata, Ready")
    text = change(text, "    samples: OwnedSlot<Vec<u32>>,", "    samples: OwnedSlot<Vec<u32>>,\n    metadata: MetadataReceiving,")
    text = change(text, "sample_cursor: None, samples: Default::default(),", "sample_cursor: None, samples: Default::default(), metadata: Default::default(),")
    text = change(text, "            self.phase = ReceivingPhase::Bind;",
        "            self.phase = if self.samples.value.is_some() { ReceivingPhase::BindMetadata } else { ReceivingPhase::Bind };")
    text = change(text, "        let samples_at = if self.phase == ReceivingPhase::Json {",
        '''        if self.phase == ReceivingPhase::BindMetadata {
            let (step, receipt) = self.metadata.advance(
                self.value.value.as_mut().expect("retained PDF image metadata"),
                &mut self.output.value, effect)?;
            match step {
                MetadataStep::Pending => {},
                MetadataStep::Ordinary => self.phase = ReceivingPhase::Bind,
                MetadataStep::Complete => {
                    donate_samples(self.output.value.as_mut().expect("typed PDF image"), &mut self.samples.value)
                        .map_err(|error| error.with_retained_progress(receipt))?;
                    self.phase = ReceivingPhase::Ready;
                }
            }
            return Ok(receipt);
        }
        let samples_at = if self.phase == ReceivingPhase::Json {''')
    text = change(text, "            ReceivingPhase::Bind => receiving_metadata(",
        '''            ReceivingPhase::BindMetadata => self.metadata.demand(self.value.value.as_ref().expect("retained PDF image metadata"))?.0,
            ReceivingPhase::Bind => receiving_metadata(''')
    text = change(text, "    fn step_work_demand(&self, source: &[u8], _: usize) -> Result<u64, ValueError> {\n        self.validate_source(source)?;",
        '''    fn step_work_demand(&self, source: &[u8], _: usize) -> Result<u64, ValueError> {
        self.validate_source(source)?;
        if self.phase == ReceivingPhase::BindMetadata {
            return self.metadata.demand(self.value.value.as_ref().expect("retained PDF image metadata")).map(|quote| quote.1);
        }''')
    return text


def stage(workspace, verify):
    if not verify:
        OUT.mkdir(parents=True, exist_ok=True)
    base = json.loads((BASE / "receipt.json").read_text(encoding="utf8"))
    target = DONATION["RECEIVER"].parent / "🧾️metadata/🦀️.rs"
    base[str(target)] = {"stage": "8.rs", "before": None, "after": None}
    for relative, row in base.items():
        path = workspace / relative
        before = sha256(path.read_bytes()).hexdigest() if path.is_file() else None
        assert before == row["before"], relative
        assert len(str(path.resolve()).encode("utf-16-le")) // 2 <= 256, relative
        name = row["stage"]
        if name == "8.rs":
            text = (TICKET / "set-image-metadata.rs").read_text(encoding="utf8")
        else:
            text = (BASE / name).read_text(encoding="utf8")
            assert sha256(text.encode()).hexdigest() == row["after"], relative
            if name == "5.rs":
                text = receiver(text)
            elif name == "6.rs":
                text += "\n" + (TICKET / "set-image-metadata-test.rs").read_text(encoding="utf8")
            elif name == "7.json":
                source = json.loads(text)
                metadata = json.loads((TICKET / "set-image-metadata-laws.json").read_text(encoding="utf8"))
                source["ordinaryGrammar"] += metadata.pop("ordinaryGrammar")
                source["metadata"] = metadata
                text = json.dumps(source, ensure_ascii=False, indent=2) + "\n"
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
        receipt.write_text(json.dumps(base, ensure_ascii=False) + "\n", encoding="utf8")
    print(f"[DEBUG] {'verified' if verify else 'staged'} {len(base)} metadata projection paths")


if __name__ == "__main__":
    parser = ArgumentParser()
    parser.add_argument("workspace", type=Path)
    parser.add_argument("--verify", action="store_true")
    args = parser.parse_args()
    stage(args.workspace.resolve(), args.verify)
