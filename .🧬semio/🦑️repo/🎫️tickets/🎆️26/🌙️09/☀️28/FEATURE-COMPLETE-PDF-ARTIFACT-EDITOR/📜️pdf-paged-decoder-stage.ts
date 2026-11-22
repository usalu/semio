/** 📥️ Stages decoder admission and paid close for the original paged PDF source. */
import assert from "node:assert/strict";
import { join } from "node:path";

const base = "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base";
const root = join(base,"🚪️io/📥️import");
const moving = join(root,"🔁️controlled/🦀️.rs");
const api = join(root,"🦀️.rs");
const test = join(base,"✏️editor/🧪️tests/🔬️unit/🦀️.rs");
const rows: [string,string,string][]=[];
const edit=(text:string,from:string,to:string)=>{
  assert.equal(text.split(from).length,2,from.slice(0,115));
  return text.replace(from,to);
};
const guard=async(path:string,transform:(text:string)=>string)=>{
  assert(Buffer.byteLength(path)<=256,path);
  const source=await Bun.file(path).text();
  rows.push([path,new Bun.CryptoHasher("sha256").update(source).digest("hex"),transform(source)]);
};
const ownedSource = [
  "enum PdfNaturalSource {",
  "    Contiguous(Vec<u8>),",
  "    Paged(NaturalFilePagedSource),",
  "}",
  "",
  "impl PdfNaturalSource {",
  "    fn reader(&self) -> Option<&dyn ByteReadSource> {",
  "        match self {",
  "            Self::Contiguous(source) => Some(source),",
  "            Self::Paged(source) => source.original().map(|bytes| bytes as &dyn ByteReadSource),",
  "        }",
  "    }",
  "",
  "    fn contiguous_capacity(&self) -> usize {",
  "        match self { Self::Contiguous(bytes) => bytes.capacity(), Self::Paged(_) => 0 }",
  "    }",
  "",
  "    fn close_paged(&mut self, effect: &mut RetainedAllocationEffect<'_>) -> Result<Option<semio_framework_value::RetainedCloneStep>, ValueError> {",
  "        let Self::Paged(source) = self else { return Ok(None); };",
  "        let progress = settle_close_poll(source.step(effect.grant().maximum_copy_bytes, effect))?;",
  "        if source.terminal_is_empty() { *self = Self::Contiguous(Vec::new()); }",
  "        Ok(Some(progress))",
  "    }",
  "",
  "    fn terminal_is_empty(&self) -> bool {",
  "        match self { Self::Contiguous(bytes) => bytes.capacity() == 0, Self::Paged(source) => source.terminal_is_empty() }",
  "    }",
  "}",
  "",
].join("\n");
const pagedAdmission = [
  "/// 📏️ Bounds the independently admitted ordinary PDF source extent.",
  "pub const PDF_PAGED_NATURAL_SOURCE_MAXIMUM_BYTES: u32 = 16 * 1024 * 1024;",
  "",
  "/// 📄️ Donates the exact sealed page owner to a paid PDF decoder frame.",
  "pub fn admit_pdf_paged_natural_decode(",
  "    source: &mut Option<NaturalFilePagedSource>, contract: ToolExecutionContract,",
  "    receiving: &mut Option<NaturalFileDecodeOwner<PdfSnapshot>>, context: &mut StepContext<'_>,",
  ") -> Result<bool, semio_plugin::__job::OriginalTurnFailure> {",
  "    if receiving.is_some() { return Ok(true); }",
  "    let original = source.as_ref().filter(|owner| owner.original_is_untouched()).and_then(|owner| owner.original())",
  '        .ok_or_else(|| ValueError::invariant("missing sealed original PDF pages"))?;',
  "    if original.byte_len() == 0 || original.byte_len() > PDF_PAGED_NATURAL_SOURCE_MAXIMUM_BYTES as usize {",
  '        return Err(ValueError::literal(ValueRefusalKind::InvalidValue, "original PDF extent exceeds its declared source ceiling").into());',
  "    }",
  "    let capacity = size_of::<NaturalPdfDecoder>();",
  "    let copy = capacity + size_of::<NaturalFileDecodeOwner<PdfSnapshot>>();",
  "    let grant = context.retained_grant();",
  "    if context.should_yield() || context.is_cancelled() || grant.maximum_items == 0 || grant.maximum_depth == 0",
  "        || copy > grant.maximum_copy_bytes || capacity > grant.maximum_capacity_bytes",
  "    {",
  "        return Ok(false);",
  "    }",
  "    let mut decoder = admit_box(NaturalPdfDecoder::new(contract)).ok_or_else(||",
  '        ValueError::literal(ValueRefusalKind::AllocationFailed, "unable to allocate the controlled PDF file decoder"))?;',
  '    decoder.source = PdfNaturalSource::Paged(source.take().expect("sealed original PDF pages"));',
  "    *receiving = Some(NaturalFileDecodeOwner::Controlled(decoder));",
  "    context.consume_fuel(1);",
  "    context.consume_retained(RetainedCloneProgress {",
  "        copied_items: 1, copied_bytes: copy, retained_capacity_bytes: capacity, ..Default::default()",
  "    })?;",
  "    Ok(true)",
  "}",
  "",
].join("\n");
await guard(moving,source=>{
  let next=edit(source,"use semio_framework_value::{ErasedSnapshotRetirement","use semio_framework_value::{ByteReadSource, ErasedSnapshotRetirement");
  next=edit(next,"NaturalFileDecodeOwner, NaturalFileDecodeProgress","NaturalFileDecodeOwner, NaturalFilePagedSource, NaturalFileDecodeProgress");
  next=edit(next,"struct NaturalPdfDecoder {",pagedAdmission+"\n"+ownedSource+"\nstruct NaturalPdfDecoder {");
  next=edit(next,"    source: Vec<u8>,","    source: PdfNaturalSource,");
  next=edit(next,"decoder.source = bytes.take().expect(\"original PDF source is present\");","decoder.source = PdfNaturalSource::Contiguous(bytes.take().expect(\"original PDF source is present\"));");
  next=edit(next,"decoder.source = bytes;","decoder.source = PdfNaturalSource::Contiguous(bytes);");
  next=edit(next,"source: Vec::new(), preparing: None","source: PdfNaturalSource::Contiguous(Vec::new()), preparing: None");
  next=edit(next,"                let mut step = self.preparing.as_mut().expect(\"PDF file preparation\").advance(&self.source, grant, false);",[
    '                let source = self.source.reader().ok_or_else(|| failure("original PDF source was retired before preparation"))?;',
    '                let mut step = self.preparing.as_mut().expect("PDF file preparation").advance(source, grant, false);',
  ].join("\n"));
  next=edit(next,"self.progress.consumed_input_bytes = self.source.len();",
    'self.progress.consumed_input_bytes = self.source.reader().ok_or_else(|| failure("original PDF source was retired before transfer"))?.byte_len();');
  next=edit(next,"        let bytes = self.source.capacity();",[
    "        let PdfNaturalSource::Contiguous(source) = &mut self.source else { return turn(Event::Closed, 0); };",
    "        let bytes = source.capacity();",
  ].join("\n"));
  next=edit(next,"        self.source = Vec::new();","        *source = Vec::new();");
  next=edit(next,"            Ok(PdfByteGraphEvent::Closed) => Ok(PluginLifecycleStep::Complete(progress)),",[
    "            Ok(PdfByteGraphEvent::Closed) => {",
    "                if let Some(settled) = self.source.close_paged(effect)? {",
    "                    return Ok(if self.source.terminal_is_empty() { PluginLifecycleStep::Complete(settled.progress()) }",
    "                        else { PluginLifecycleStep::Progress(settled.progress()) });",
    "                }",
    "                Ok(PluginLifecycleStep::Complete(progress))",
    "            }",
  ].join("\n"));
  next=edit(next,"        let raw_release = if self.preparing.is_none() && self.file.is_none() && self.lifting.is_none() && self.trailer_retirement.is_none() { self.source.capacity() } else { 0 };",[
    "        let files_retired = self.preparing.is_none() && self.file.is_none() && self.lifting.is_none() && self.trailer_retirement.is_none();",
    "        if files_retired {",
    "            if let PdfNaturalSource::Paged(source) = &self.source { return source.next_demand(body); }",
    "        }",
    "        let raw_release = if files_retired { self.source.contiguous_capacity() } else { 0 };",
  ].join("\n"));
  return edit(next,"matches!(self.phase, Phase::Closing) && self.source.capacity() == 0","matches!(self.phase, Phase::Closing) && self.source.terminal_is_empty()");
});
await guard(api,source=>edit(source,"admit_pdf_natural_decode, build_pdf_natural_decode","admit_pdf_natural_decode, admit_pdf_paged_natural_decode, build_pdf_natural_decode, PDF_PAGED_NATURAL_SOURCE_MAXIMUM_BYTES"));
await guard(test,source=>{
  let next=edit(source,"    let run = |constrained_at: Option<usize>| {","    let run = |constrained_at: Option<usize>, paged: bool| {");
  next=edit(next,`        let mut source = Some(input.to_vec());
        let mut claimed = None;
        assert!(scope.turn(stock, |cx| Pdf17Editor::admit_natural_file_decode_owner(&mut source, contract, &mut claimed, cx)).unwrap());
        assert!(source.is_none(), "the exact original PDF bytes transferred");`,`        let mut claimed = None;
        if paged {
            use crate::standards::v1_7::subsets::base::io::import::admit_pdf_paged_natural_decode;
            use semio_framework_value::paged::PagedBytes;
            use semio_plugin::NaturalFilePagedSource;
            let mut source = Some(NaturalFilePagedSource::retain(PagedBytes::try_from_slice(input).unwrap()));
            let smaller_wire_contract = ToolExecutionContract { max_raw_wire_bytes: 1, ..contract };
            let denied = RetainedCloneGrant { maximum_items: 0, ..stock };
            assert!(!scope.turn(denied, |cx| admit_pdf_paged_natural_decode(&mut source, smaller_wire_contract, &mut claimed, cx)).unwrap());
            assert!(source.as_ref().is_some_and(|owner| owner.original_is_untouched()));
            assert!(claimed.is_none());
            assert!(scope.turn(stock, |cx| admit_pdf_paged_natural_decode(&mut source, smaller_wire_contract, &mut claimed, cx)).unwrap());
            assert!(source.is_none(), "the original paged PDF owner transferred");
        } else {
            let mut source = Some(input.to_vec());
            assert!(scope.turn(stock, |cx| Pdf17Editor::admit_natural_file_decode_owner(&mut source, contract, &mut claimed, cx)).unwrap());
            assert!(source.is_none(), "the exact original PDF bytes transferred");
        }`);
  next=edit(next,"run(None);\n    let (published, progress, _, mut scope) = run(Some(terminal_turn));",
    "run(None, false);\n    let (published, progress, _, mut scope) = run(Some(terminal_turn), false);\n    let (paged, paged_progress, _, mut paged_scope) = run(None, true);");
  next=edit(next,"    assert_eq!(published, reference);\n    assert_eq!(progress, reference_progress);",[
    "    assert_eq!(published, reference);",
    "    assert_eq!(paged, reference);",
    "    assert_eq!(progress, reference_progress);",
    "    assert_eq!(paged_progress.consumed_input_bytes, input.len());",
    "    paged_scope.retire_owned(paged, stock);",
    "    paged_scope.finish();",
  ].join("\n"));
  return next;
});
await guard(join(base,"✏️editor/🦀️.rs"),source=>{
  const declared = "    fn natural_file_codec() -> Option<semio_plugin::NaturalFileCodec> {";
  let next = edit(source,declared,"    const PAGED_NATURAL_SOURCE_MAXIMUM_BYTES: u32 = crate::standards::v1_7::subsets::base::io::import::PDF_PAGED_NATURAL_SOURCE_MAXIMUM_BYTES;\n\n"+declared);
  const importMedia = "    fn import_media(port: &str, media: &semio_plugin::app::Media,";
  const implementation = [
    "    fn admit_paged_natural_file_decode_owner(",
    "        source: &mut Option<semio_plugin::NaturalFilePagedSource>, contract: semio_plugin::ToolExecutionContract,",
    "        owner: &mut Option<semio_plugin::NaturalFileDecodeOwner<Self::Snapshot>>, context: &mut semio_plugin::StepContext<'_>,",
    "    ) -> Result<bool, semio_plugin::__job::OriginalTurnFailure> {",
    "        crate::standards::v1_7::subsets::base::io::import::admit_pdf_paged_natural_decode(source, contract, owner, context)",
    "    }",
    "",
    importMedia,
  ].join("\n");
  return edit(next,importMedia,implementation);
});
await Bun.write(join(import.meta.dir,"🗑️generated","pdf-paged-decoder-overlay.json"),JSON.stringify(rows));
console.log("[DEBUG] Staged original paged PDF decoder admission, reads, close and conformance law");
