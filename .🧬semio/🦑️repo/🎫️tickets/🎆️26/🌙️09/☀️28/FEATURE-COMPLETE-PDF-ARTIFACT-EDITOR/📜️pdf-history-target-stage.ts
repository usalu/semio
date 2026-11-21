/** 🎯 Stage paid document conflict paths for every registered PDF operation and editor. */
import assert from "node:assert/strict";
import {createHash} from "node:crypto";
import {existsSync,readdirSync,readFileSync} from "node:fs";
import {dirname,join,resolve} from "node:path";

const base17="✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base";
const base14=base17.replace("7️⃣1.7","4️⃣1.4");
const generated=join(import.meta.dir,"🗑️generated");
const sha=(s:string)=>createHash("sha256").update(s).digest("hex");
const records=new Map<string,[string,string|null,string]>();
function put(file:string,next:string){
    const original=existsSync(file)?readFileSync(file,"utf8"):null;
    if(original!==next)records.set(file,[file,original===null?null:sha(original),next]);
}
function once(s:string,needle:string,next:string){
    assert.equal(s.split(needle).length,2,needle);
    return s.replace(needle,next);
}

const native=`//! 🎯 Exact borrowed Store conflict paths for the PDF mutation vocabulary.

use super::PdfMutation;
use semio_framework_value::{ValueError, ValueRefusalKind};
use std::io::{Cursor, Write};
use store::ArtifactOperationTargets;

#[derive(Clone, Copy)]
enum Segment<'a> { Text(&'a str), Index(usize), Reference(u32, u16) }

impl Segment<'_> {
    fn visit<T>(self, read: impl FnOnce(&[u8]) -> Result<T, ValueError>) -> Result<T, ValueError> {
        let mut scratch = [0; 32];
        let mut output = Cursor::new(&mut scratch[..]);
        match self {
            Self::Text(text) => return read(text.as_bytes()),
            Self::Index(index) => write!(output, "{index}"),
            Self::Reference(object, generation) => write!(output, "{object} {generation}"),
        }.map_err(|_| ValueError::invariant("PDF conflict path exceeds its fixed scratch"))?;
        let end = output.position() as usize;
        read(&scratch[..end])
    }
}

fn absent() -> ValueError {
    ValueError::literal(ValueRefusalKind::InvalidValue, "PDF conflict path segment is absent")
}

impl ArtifactOperationTargets for PdfMutation {
    fn len(&self) -> usize {
        self.conflict_segments().into_iter().flatten().count()
    }

    fn segment_len(&self, index: usize) -> Result<usize, ValueError> {
        self.conflict_segment(index)?.visit(|segment| Ok(segment.len()))
    }

    fn read_segment(&self, index: usize, offset: usize, output: &mut [u8]) -> Result<usize, ValueError> {
        self.conflict_segment(index)?.visit(|segment| {
            let rest = segment.get(offset..).ok_or_else(absent)?;
            let copied = rest.len().min(output.len());
            output[..copied].copy_from_slice(&rest[..copied]);
            Ok(copied)
        })
    }
}

impl PdfMutation {
    /// 🎞️ Lends the canonical target directly from the persisted operation.
    pub fn original_conflict_target(&self) -> Option<&dyn ArtifactOperationTargets> {
        self.conflict_segments()[0].map(|_| self as &dyn ArtifactOperationTargets)
    }

    /// 🏠 Admits the document's semantic replay, retirement and conflict target issuers together.
    pub fn document_store_owners(
        grant: semio_framework_value::RetainedCloneGrant,
    ) -> Result<
        (store::os_store::document_store_owners::DocumentStoreOwners<super::PdfSnapshot, Self>, semio_framework_value::RetainedCloneProgress),
        store::os_store::document_store_owners::DocumentStoreOwnersAdmissionError<super::PdfSnapshot, Self>,
    > {
        semio_plugin::cold_replay_document_store_owners::<super::PdfSnapshot, Self>(grant).map(|(mut owners, progress)| {
            owners.operation_wire = owners.operation_wire.with_original_target(Self::original_conflict_target);
            (owners, progress)
        })
    }

    fn conflict_segment(&self, index: usize) -> Result<Segment<'_>, ValueError> {
        self.conflict_segments().get(index).copied().flatten().ok_or_else(absent)
    }

    fn conflict_segments(&self) -> [Option<Segment<'_>>; 2] {
        use Segment::*;
        match self {
{MATCHES}
        }
    }
}
`;

function matchRules(root:string, sources: string[]) {
    const aggregate=join(root,"🦀️.rs");
    const project=readFileSync(aggregate,"utf8");
    const variants=[...project.match(/pub enum PdfMutation \{([\s\S]*?)\n\}/)![1].matchAll(/^\s+(\w+)\(\w+\),?$/gm)].map(row=>row[1]);
    const matches=new Map<string,string>();
    const files=Array.from(new Set(sources.flatMap(dir=>readdirSync(dir).map(n=>join(dir,n,"🦀️.rs"))))).filter(existsSync);
    for(const file of files){
        const src=readFileSync(file,"utf8");
        const name=src.match(/pub struct (\w+)/)?.[1];
        if(!name||!variants.includes(name))continue;
        const source=src.match(/fn target\(&self\) -> Vec<String> \{\s*(.*)\s*\}/)?.[1]?.trim();
        if(!source)continue;
        const options=[];
        const expr=source.startsWith("vec![")?source.slice(5,-1):null;
        if(expr!==null){
            const parts=expr.replace('format!("{} {}", self.id.num, self.id.gen)',"REFERENCE").split(", ");
            for(const part of parts){
                if(part==="REFERENCE")options.push("Some(Reference(operation.id.num, operation.id.gen))");
                else if(part.startsWith("self.")&&part.endsWith(".to_string()"))options.push(`Some(Index(${part.replace("self.","operation.").replace(".to_string()","")}))`);
                else if(part.startsWith("self.")&&part.endsWith(".clone()"))options.push(`Some(Text(&${part.replace("self.","operation.").replace(".clone()","")}))`);
                else if(part==='"Info".to_string()')options.push('Some(Text("Info"))');
                else throw new Error(name+" "+part);
            }
        }else assert.equal(source,"Vec::new()",name);
        assert.ok(options.length<=2);
        const item=options.some(v=>v.includes("operation"))?"operation":"_";
        while(options.length<2)options.push("None");
        const row=`            Self::${name}(${item}) => [${options.join(", ")}],`;
        assert.ok(!matches.has(name)||matches.get(name)===row,name);
        matches.set(name,row);
    }
    assert.equal(matches.size,variants.length,JSON.stringify(variants.filter(name=>!matches.has(name))));
    let history = native.replace("{MATCHES}",variants.map(name=>matches.get(name)).join("\n"));
    if ([...matches.values()].every(line => !line.includes("Some(Text(") && !line.includes("Some(Reference("))) {
        history = once(history, "enum Segment<'a> { Text(&'a str), Index(usize), Reference(u32, u16) }", "struct Segment(usize);");
        history = once(history, "impl Segment<'_> {", "impl Segment {");
        const format = history.slice(history.indexOf("        match self {"), history.indexOf("        let end = output.position()", history.indexOf("        match self {")));
        history = once(history, format, '        write!(output, "{}", self.0).map_err(|_| ValueError::invariant("PDF conflict path exceeds its fixed scratch"))?;\n');
        history = history.replaceAll("Segment<'_>", "Segment").replace("        use Segment::*;\n", "").replaceAll("Some(Index(", "Some(Segment(");
    }
    put(aggregate,once(project,"//#endregion 🔖️Aggregate","mod history_target {\n"+history+"}\n\n//#endregion 🔖️Aggregate"));
}

const mut17=join(base17,"🧬️schema","🧬️mutations");
const mut14=join(base14,"🧬️schema","🧬️mutations");
matchRules(mut17,[mut17]);
matchRules(mut14,[mut14]);

const editor17s=["🧱️base","🗄️a","📐️e","🏠️h","🔓️ua","🖨️x","🧾️vt"];
const editor14s=["🧱️base","🗄️a","🖨️x"];
for(const standard of [base14,base17]){
    const root=dirname(standard);
    for(const profile of readdirSync(root)){
        if(!(standard===base14?editor14s:editor17s).includes(profile)&&!existsSync(join(root,profile,"✏️editor","🦀️.rs")))continue;
        const file=join(root,profile,"✏️editor","🦀️.rs");
        if(!existsSync(file))continue;
        let text=readFileSync(file,"utf8");
        assert.ok(text.includes("type Mutation = PdfMutation;"),file);
        if(!text.includes("fn document_store_owners_source_demands()")){
            const hook=`    fn document_store_owners_source_demands() -> Result<semio_plugin::__value::RetirementDemand, semio_plugin::__value::ValueError> {
        semio_plugin::cold_replay_document_store_owners_source_demands::<PdfSnapshot, PdfMutation>()
    }

`;
            text=once(text,"    fn natural_file_codec()",hook+"    fn natural_file_codec()");
        }
        const implicit="        bounded_native: true";
        if(!text.includes("        document_store_owners:")){
            assert.equal(text.split(implicit).length,2,file);
            text=text.replace(implicit,"        document_store_owners: PdfMutation::document_store_owners,\n"+implicit);
        }else{
            text=once(text,"        document_store_owners: semio_plugin::cold_replay_document_store_owners::<PdfSnapshot, PdfMutation>,","        document_store_owners: PdfMutation::document_store_owners,");
        }
        put(file,text);
    }
}
const vt=base17.replace("🧱️base","🧾️vt");
const vtIo=join(vt,"🚪️io","🦀️.rs");
const vtIoStart=readFileSync(vtIo,"utf8");
const analysis='    #[cfg(test)]\n    include!("../🧬️schema/🧪️tests/🔬️derived-analysis-unit/🦀️.rs");\n';
put(vtIo,once(vtIoStart,analysis,""));
const vtManifest=join(vt,"🧬️schema","📦️packages","🦀️rust","Cargo.toml");
const vtAnalysis=join(vt,"🧬️schema","🧪️tests","🔬️derived-analysis-unit","🦀️.rs");
const oldAnalysis=readFileSync(vtAnalysis,"utf8");
const exposed=`use semio_s_artifact_stdio_pdf_schema_5::standards;
use standards::v1_7::subsets::base::schema::snapshot::{ObjRef, PdfDictEntry, PdfIndirectObject, PdfObject, PdfSnapshot};
use standards::v1_7::subsets::vt::io::{CODE_DPART_ROOT, CODE_DPM, check_vt_conformance};
use semio_framework_diagnostic::Severity;

`;
put(vtAnalysis,exposed+oldAnalysis);
const oldManifest=readFileSync(vtManifest,"utf8");
assert.ok(!oldManifest.includes('name = "vt_conformance"'));
put(vtManifest,oldManifest+'\n[[test]]\nname = "vt_conformance"\npath = "../../🧪️tests/🔬️derived-analysis-unit/🦀️.rs"\n');

const native14=join(base14,"🚪️io","💾️binary","🧬️mutations","🦀️.rs");
const receiver14=join(dirname(native14),"📥️receiving","🦀️.rs");
const native14Source=readFileSync(native14,"utf8");
const receiver14Source=readFileSync(join(import.meta.dir,"📜️pdf14-wire-receiving.rs"),"utf8");
put(receiver14,receiver14Source);
put(native14,once(native14Source,"//#endregion 🔖️Framing",`//#endregion 🔖️Framing

#[path = "📥️receiving/🦀️.rs"]
mod receiving;
pub use receiving::{Pdf14OperationReceiving, Pdf14ReceivingRetirement};

impl protocol::OpBinaryReceiving for PdfMutation {
    type DomainFailure = semio_framework_value::ValueError;
    type Receiving = Pdf14OperationReceiving;

    fn admit_receiving(
        source: &[u8], range: (usize, usize), grant: semio_framework_value::RetainedCloneGrant,
    ) -> semio_framework_value::RetainedWorkOutcome<Option<(Self::Receiving, semio_framework_value::RetainedCloneProgress)>> {
        Pdf14OperationReceiving::admit(source, range, grant)
    }
}
`));
const schema14=join(mut14,"🦀️.rs");
put(schema14,once(records.get(schema14)![2],"dsl::original_pack_value_operation_receiving!(PdfMutation);\n",""));

const rows=[...records.values()];
for(const [file,before] of rows)assert.equal(existsSync(file)?sha(readFileSync(file,"utf8")):null,before);
await Bun.write(join(generated,"pdf-history-target-overlay.json"),JSON.stringify(rows,null,2)+"\n");
console.log("[DEBUG] Staged registered PDF history paths and semantic issuers",rows.length);
