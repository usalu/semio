"""📥 Stages production aggregate donation over the Original format-1 cursor."""

from argparse import ArgumentParser
from hashlib import sha256
import json
from pathlib import Path
import runpy

TICKET = Path(__file__).resolve().parent
OUT = TICKET / "🗑️generated" / "set-donation"
U32 = runpy.run_path(str(TICKET / "set-image-u32-stage.py"))
ARCHIVE = runpy.run_path(str(TICKET / "archive-receiver-stage.py"))
change = U32["change"]
PDF = U32["PDF"]
BINARY = ARCHIVE["BINARY"]
SCHEMA = PDF / "🧬️schema/🧬️mutations/🦀️.rs"
TEST = ARCHIVE["TEST"]
RECEIVER = BINARY / "📩️receiving/🦀️.rs"


def detach(array):
    array = change(array, "pub struct JsonU32ArrayCursor<'a> {\n    source: &'a [u8],",
                   "pub struct JsonU32ArrayCursor {\n    source: (usize, usize),")
    array = change(array, "impl<'a> JsonU32ArrayCursor<'a>", "impl JsonU32ArrayCursor")
    array = change(array, "pub fn admit(source: &'a [u8]", "pub fn admit(source: &[u8]")
    array = change(array, "            source, start, position: start", "            source: (source.as_ptr() as usize, source.len()), start, position: start")
    array = change(array, "    pub fn demand(&self) -> Result<(RetirementDemand, u64), ValueError> {", '''    fn validate_source(&self, source: &[u8]) -> Result<(), ValueError> {
        if self.source != (source.as_ptr() as usize, source.len()) {
            return Err(ValueError::invariant("typed sample source identity changed"));
        }
        Ok(())
    }

    pub fn demand(&self, source: &[u8]) -> Result<(RetirementDemand, u64), ValueError> {
        self.validate_source(source)?;''')
    array = array.replace("self.source.get(", "source.get(")
    array = change(array, "pub fn advance(&mut self, effect:", "pub fn advance(&mut self, source: &[u8], effect:")
    array = change(array, "self.advance_one(effect, checkpoint)", "self.advance_one(source, effect, checkpoint)")
    array = change(array, "fn advance_one(&mut self, effect:", "fn advance_one(&mut self, source: &[u8], effect:")
    array = change(array, "self.demand()?", "self.demand(source)?")
    array = change(array, "self.advance_grammar()", "self.advance_grammar(source)")
    array = change(array, "fn advance_grammar(&mut self)", "fn advance_grammar(&mut self, source: &[u8])")
    array = change(array, "&self.source[start..]", "&source[start..]")
    array = change(array, '''    pub fn finished_span(&self) -> Option<JsonNumberArraySpan<'a>> {
        self.is_ready().then(|| JsonNumberArraySpan::finished(self.source, self.start..self.position))
    }''', '''    pub fn finished_span<'a>(&self, source: &'a [u8]) -> Result<Option<JsonNumberArraySpan<'a>>, ValueError> {
        self.validate_source(source)?;
        Ok(self.is_ready().then(|| JsonNumberArraySpan::finished(source, self.start..self.position)))
    }''')
    return change(array, "impl Drop for JsonU32ArrayCursor<'_>", "impl Drop for JsonU32ArrayCursor")


def receiver():
    rust = ARCHIVE["candidate"]()
    rust = change(rust, "JsonError, JsonGrammarCursor, JsonMemberPolicy", "JsonError, JsonGrammarCursor, JsonMemberPolicy, JsonU32ArrayCursor")
    rust = change(rust, "StartJson, Json, Identity", "StartJson, Json, Samples, SamplesMarker, SamplesTake, Identity")
    rust = change(rust, "    retained_json: usize,", '''    retained_json: usize,
    sample_cursor: Option<JsonU32ArrayCursor>,
    samples: OwnedSlot<Vec<u32>>,''')
    rust = change(rust, "retained_json: 0, failed: false,", '''retained_json: 0, failed: false,
            sample_cursor: None, samples: Default::default(),''')
    rust = change(rust, "if self.phase == ReceivingPhase::Json { return self.advance_json(source, native, effect); }", '''if matches!(self.phase, ReceivingPhase::Samples | ReceivingPhase::SamplesMarker | ReceivingPhase::SamplesTake) {
            return self.advance_samples(source, effect).map_err(Into::into);
        }
        let samples_at = if self.phase == ReceivingPhase::Json {
            let start = self.samples_start(source)?;
            if start.is_none() { return self.advance_json(source, native, effect); }
            start
        } else { None };''')
    rust = change(rust, "            ReceivingPhase::Identity(index) => self.check_identity(index),", '''            ReceivingPhase::Json => {
                self.sample_cursor = JsonU32ArrayCursor::admit(self.frame.payload(source),
                    samples_at.expect("selected PDF Samples start"), SAMPLE_BACKING_BYTES, SAMPLE_BACKING_BYTES, effect.grant())?;
                self.phase = ReceivingPhase::Samples;
                Ok(())
            }
            ReceivingPhase::Identity(index) => self.check_identity(index),''')
    rust = change(rust, '''            ReceivingPhase::Bind => PdfMutation::from_value(self.value.value.take().expect("admitted PDF mutation JSON value")).map(|mutation| {
                *self.output.value = Some(mutation);
                self.phase = ReceivingPhase::Ready;
            }),''', "            ReceivingPhase::Bind => self.bind_mutation(),")
    rust = change(rust, "            ReceivingPhase::Json => {\n                let parser =",
                  '''            ReceivingPhase::Json if self.samples_start(source)?.is_some() => JsonU32ArrayCursor::admission_demand(),
            ReceivingPhase::Samples => self.sample_cursor.as_ref().expect("admitted PDF Samples cursor").demand(self.frame.payload(source))?.0,
            ReceivingPhase::SamplesMarker => JsonGrammarCursor::<DslValue>::number_array_marker_demand(),
            ReceivingPhase::SamplesTake => JsonU32ArrayCursor::take_demand(),
            ReceivingPhase::Json => {
                let parser =''')
    rust = change(rust, "self.retained_json.saturating_add(OUTPUT_HANDOFF + size_of::<DslValue>())",
                  "self.retained_json.saturating_add(OUTPUT_HANDOFF + size_of::<DslValue>() + 2 * size_of::<Option<Vec<u32>>>())")
    rust = change(rust, "    fn step_in(&mut self,", '''    fn step_work_demand(&self, source: &[u8], _: usize) -> Result<u64, ValueError> {
        self.validate_source(source)?;
        if self.phase == ReceivingPhase::Samples {
            return self.sample_cursor.as_ref().expect("admitted PDF Samples cursor").demand(self.frame.payload(source)).map(|quote| quote.1);
        }
        Ok(1)
    }

    fn step_in(&mut self,''')
    rust = change(rust, "let demand = if !self.output.terminal_is_empty() { self.output.close_demands(body) }",
                  '''let demand = if let Some(samples) = self.sample_cursor.as_ref() { Ok(samples.close_demand()) }
        else if !self.samples.terminal_is_empty() { self.samples.close_demands(body) }
        else if !self.output.terminal_is_empty() { self.output.close_demands(body) }''')
    rust = change(rust, "        if !self.output.terminal_is_empty() { return self.output.close_work_demand(body); }",
                  '''        if self.sample_cursor.is_some() { return Ok(1); }
        if !self.samples.terminal_is_empty() { return self.samples.close_work_demand(body); }
        if !self.output.terminal_is_empty() { return self.output.close_work_demand(body); }''')
    rust = change(rust, "            let child = if !self.output.terminal_is_empty() { Some(self.output.close_step(effect)) }", '''            if let Some(samples) = self.sample_cursor.as_mut() {
                let turn = samples.close(effect);
                if samples.terminal_is_empty() { self.sample_cursor = None; }
                return match turn {
                    RetainedWorkPoll::Pending { work } => RetainedWorkPoll::Pending { work },
                    RetainedWorkPoll::Complete(turn) => RetainedWorkPoll::Complete(RetainedWorkOutcome {
                        result: turn.result.map(|step| RetainedCloneStep::Progress(step.progress())), work: turn.work,
                    }),
                };
            }
            let child = if !self.samples.terminal_is_empty() { Some(self.samples.close_step(effect)) }
            else if !self.output.terminal_is_empty() { Some(self.output.close_step(effect)) }''')
    rust = change(rust, "self.phase == ReceivingPhase::Closed && self.output.terminal_is_empty()",
                  "self.phase == ReceivingPhase::Closed && self.sample_cursor.is_none() && self.samples.terminal_is_empty() && self.output.terminal_is_empty()")
    return rust + "\n" + (TICKET / "set-image-donation.rs").read_text(encoding="utf8")


def stage(workspace, verify):
    root, grammar, array = ((U32["OUT"] / f"{index}.rs").read_text(encoding="utf8") for index in range(3))
    grammar += (TICKET / "set-image-tagged-grammar.rs").read_text(encoding="utf8")
    binary = (workspace / BINARY / "🦀️.rs").read_text(encoding="utf8")
    binary = change(binary, "//#region 🧱️Framing", '''#[path = "📩️receiving/🦀️.rs"]
mod receiving;
pub use receiving::{PdfMutationReceiving, PdfMutationReceivingRetirement};

//#region 🧱️Framing''')
    schema = change((workspace / SCHEMA).read_text(encoding="utf8"), "dsl::original_pack_value_operation_receiving!(PdfMutation);\n", "")
    test = (workspace / TEST / "🦀️.rs").read_text(encoding="utf8") + "\n" + (TICKET / "set-image-donation-test.rs").read_text(encoding="utf8")
    changes = {
        U32["ROOT"]: root, U32["GRAMMAR"]: grammar, U32["ARRAY"]: detach(array),
        BINARY / "🦀️.rs": binary, SCHEMA: schema, RECEIVER: receiver(),
        TEST / "🦀️.rs": test,
        TEST / "set-image-donation.json": (TICKET / "set-image-donation.json").read_text(encoding="utf8"),
    }
    if not verify:
        OUT.mkdir(parents=True, exist_ok=True)
    receipt = {}
    for index, (path, source) in enumerate(changes.items()):
        target = workspace / path
        assert len(str(target.resolve()).encode("utf-16-le")) // 2 <= 256
        before = target.read_bytes() if target.exists() else None
        draft = OUT / f"{index}{path.suffix}"
        after = source.encode()
        receipt[str(path)] = {"stage": draft.name, "before": sha256(before).hexdigest() if before else None, "after": sha256(after).hexdigest()}
        if verify:
            assert draft.read_bytes() == after
        else:
            draft.write_bytes(after)
    witness = OUT / "receipt.json"
    if verify:
        assert json.loads(witness.read_bytes()) == receipt
    else:
        witness.write_text(json.dumps(receipt, ensure_ascii=False) + "\n", encoding="utf8")
    print(f"[DEBUG] {'verified' if verify else 'staged'} {len(changes)} typed donation paths")


if __name__ == "__main__":
    parser = ArgumentParser()
    parser.add_argument("workspace", type=Path)
    parser.add_argument("--verify", action="store_true")
    args = parser.parse_args()
    stage(args.workspace.resolve(), args.verify)
