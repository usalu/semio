"""Prepares (default) or applies the retained DWG route edits. `prepare` writes unified diffs under dwg-retained-prepared/ and touches no source; `apply` saves them."""
import difflib, os, sys

ROOT = "/Users/ueli/Documents/semio"
TF = os.path.dirname(os.path.abspath(__file__))
OUT = os.path.join(TF, "dwg-retained-prepared")
NEW = os.path.join(OUT, "new")
DWG = ROOT + "/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖊️dwg/🏅️standards/🔟ac1024/🪆️subsets/✳️any"
BREP = ROOT + "/✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep"
SESSION = ROOT + "/✏️s/🔨️modules/🌐️spatial-kernel/⚙️engine/🧠️semio/🌊️session"
KERNEL = ROOT + "/🧰️framework/🔨️modules/🧊️3d/📐️brep/⚙️engine/📥️mesh-publication/🦀️.rs"
OJ = BREP + "/〰️analytic-curves"
EXPORT = DWG + "/🚪️io/📤️export"

def rel(frm, to):
    return os.path.relpath(to, frm)

def read(path):
    return open(path, encoding="utf-8").read()

class Plan:
    def __init__(self):
        self.edits = {}
        self.files = {}

    def edit(self, path, old, new, count=1):
        text = self.edits.get(path, read(path))
        assert text.count(old) == count, (path, old[:80], text.count(old))
        self.edits[path] = text.replace(old, new)

    def create(self, path, text):
        self.files[path] = text

plan = Plan()

# --- stdio dwg: retire-capable stepped writer ---------------------------------------------------------------
cursor = EXPORT + "/🧵️write-cursor/🦀️.rs"
plan.edit(cursor, "enum Phase {\n    Snapshot,\n", "#[derive(Clone, Copy, PartialEq, Eq)]\nenum Phase {\n")
plan.edit(cursor, "    Taken,\n    Failed(String),\n}\n", "    Taken,\n    Failed,\n}\nsemio_framework_value::artifact_retire_leaf!(Phase);\n")
plan.edit(cursor, "pub struct DwgWriteCursor {\n    phase: Phase,\n    drawing: Option<DwgDrawing>,\n    snapshot", "pub struct DwgWriteCursor {\n    phase: Phase,\n    failure: String,\n    snapshot")
plan.edit(cursor, "    written: usize,\n}\n\nimpl DwgWriteCursor {", "    written: usize,\n}\nsemio_framework_value::artifact_retire_struct!(DwgWriteCursor { phase, failure, snapshot, records, objects, pairs, sorted, handle_blocks, handles, handle_next, slots, slot_next, sections, page_section, page_offset, pages, layout, output, written });\n\nimpl DwgWriteCursor {")
plan.edit(cursor, "            phase, drawing: None, snapshot: None,", "            phase, failure: String::new(), snapshot: None,")
plan.edit(cursor, "    /// 🎟️ Starts a new AC1024 document holding `drawing`, as [`dwg_to_bytes`] does.\n    pub fn new(drawing: DwgDrawing) -> Self {\n        Self { drawing: Some(drawing), ..Self::empty(Phase::Snapshot) }\n    }\n",
"    /// 🎟️ Starts a new AC1024 document holding `drawing`, as [`dwg_to_bytes`] does. Building the logical snapshot is the one unit this call performs.\n    pub fn from_drawing(drawing: &DwgDrawing) -> Result<Self, String> {\n        Ok(Self::begin_objects(DwgSnapshot::from_drawing(drawing)?))\n    }\n\n    fn failed(failure: String) -> Self {\n        Self { failure, ..Self::empty(Phase::Failed) }\n    }\n")
plan.edit(cursor, "        match &self.phase {\n            Phase::Snapshot => 0.0,\n            Phase::Objects", "        match self.phase {\n            Phase::Objects")
plan.edit(cursor, "            Phase::Failed(_) => 0.0,\n", "            Phase::Failed => 0.0,\n")
plan.edit(cursor, '*self = Self::empty(Phase::Failed("DWG write cancelled".into()));', '*self = Self::failed("DWG write cancelled".into());')
plan.edit(cursor, "                *self = Self::empty(Phase::Failed(error.clone()));", "                *self = Self::failed(error.clone());")
plan.edit(cursor, "        match &self.phase {\n            Phase::Snapshot => {\n                let drawing = self.drawing.take().ok_or(\"DWG write source missing\")?;\n                *self = Self::begin_objects(DwgSnapshot::from_drawing(&drawing)?);\n            }\n            Phase::Objects", "        match self.phase {\n            Phase::Objects")
plan.edit(cursor, "            Phase::Failed(error) => return Err(error.clone()),", "            Phase::Failed => return Err(self.failure.clone()),")

export_root = EXPORT + "/🦀️.rs"
plan.edit(export_root, "const R2004_PAGE_SIZE: usize = 0x7400;\n\nstruct R2004Section {\n    id: u32,\n    name: &'static str,\n    content: Vec<u8>,\n    stored: Option<(u32, u32)>,\n}\n", "const R2004_PAGE_SIZE: usize = 0x7400;\n\nstruct R2004Section {\n    id: u32,\n    name: &'static str,\n    content: Vec<u8>,\n    stored: Option<(u32, u32)>,\n}\nsemio_framework_value::artifact_retire_struct!(R2004Section { id, name, content, stored });\n")
plan.edit(export_root, "    section_map_page_number: u32,\n    last_page_number: u32,\n}\n", "    section_map_page_number: u32,\n    last_page_number: u32,\n}\nsemio_framework_value::artifact_retire_struct!(R2004Layout { prefix, section_info, page_map, section_info_allocation, section_map_allocation, section_map_page_number, last_page_number });\n")

records_container = EXPORT + "/🧱️records/🗜️container/🦀️.rs"
plan.edit(records_container, "    pub address: u64,\n    pub allocation_size: u32,\n}\n", "    pub address: u64,\n    pub allocation_size: u32,\n}\nsemio_framework_value::artifact_retire_struct!(EncodedR2004Page { section_id, page_number, start_offset, payload, address, allocation_size });\n")
objects = EXPORT + "/📦️objects/🦀️.rs"
plan.edit(objects, "pub struct HandleMapBlocks {\n    block: Vec<u8>,\n    base_handle: u64,\n    base_address: i64,\n    previous_handle: u64,\n}\n", "pub struct HandleMapBlocks {\n    block: Vec<u8>,\n    base_handle: u64,\n    base_address: i64,\n    previous_handle: u64,\n}\nsemio_framework_value::artifact_retire_struct!(HandleMapBlocks { block, base_handle, base_address, previous_handle });\n")
plan.edit(objects, "    handles: BTreeSet<u64>,\n    block_names: BTreeMap<u64, String>,\n}\n", "    handles: BTreeSet<u64>,\n    block_names: BTreeMap<u64, String>,\n}\nsemio_framework_value::artifact_retire_struct!(ObjectRecords { address, length, indexed, framed, handles, block_names });\n")
for crate in (EXPORT + "/🧱️records/📦️packages/🦀️rust/Cargo.toml", EXPORT + "/📦️objects/📦️packages/🦀️rust/Cargo.toml"):
    plan.edit(crate, "[dependencies]\n", "[dependencies]\nsemio-framework-value = { workspace = true }\n")

tests = EXPORT + "/🧪️tests/🔬️unit/🦀️.rs"
plan.edit(tests, "drain_cursor(DwgWriteCursor::new(drawing))", "drain_cursor(DwgWriteCursor::from_drawing(&drawing).unwrap())")
plan.edit(tests, "let mut cursor = DwgWriteCursor::new(super::acadrust_oracle_tests::every_entity_kind());", "let mut cursor = DwgWriteCursor::from_drawing(&super::acadrust_oracle_tests::every_entity_kind()).unwrap();")
oracle = EXPORT + "/🧪️tests/🔮️acadrust-oracle/🦀️.rs"
plan.edit(oracle, "DwgWriteCursor::new(drawing.clone())", "DwgWriteCursor::from_drawing(&drawing).unwrap()")
fixture = EXPORT + "/🧪️tests/🧫️fixtures/🧵️write-cursor/🔣️.json"
plan.edit(fixture, '["snapshot", "objects"', '["objects"')

# --- framework + session ------------------------------------------------------------------------------------
plan.edit(KERNEL, "        let cursor = decoder.into_cursor()?;\n        self.native.unpublished_topology += 1;\n        Ok(MeshPublication { cursor, owner: std::ptr::from_ref(&self.native) as usize, revision: self.native.shape_mutation_revision,\n            key: None, published: false })\n    }\n",
"        let cursor = decoder.into_cursor()?;\n        Ok(self.plan_mesh_publication_from_cursor(cursor))\n    }\n\n    /// 🫴️ Admits a cursor an external format decoder already filled, such as the retained DWG decoder.\n    pub fn plan_mesh_publication_from_cursor(&mut self, cursor: MeshImportCursor) -> MeshPublication {\n        self.native.unpublished_topology += 1;\n        MeshPublication { cursor, owner: std::ptr::from_ref(&self.native) as usize, revision: self.native.shape_mutation_revision,\n            key: None, published: false }\n    }\n")
core = SESSION + "/🧠️session-core/🦀️.rs"
plan.edit(core, "pub fn mesh_preparation_demand(&self, plan: &MeshPublication)", "pub fn plan_mesh_publication_from_cursor(&self, cursor: semio_framework_3d_brep_engine::MeshImportCursor) -> Result<MeshPublication, ValueError> {\n    self.with_open_kernel(|kernel| Ok(kernel.plan_mesh_publication_from_cursor(cursor)))\n}\n\npub fn mesh_preparation_demand(&self, plan: &MeshPublication)")
plan.edit(SESSION + "/🦀️.rs", "pub use mesh_export::NativeMeshExport;", "pub use mesh_export::{MeshExchange, NativeMeshExport};")
mx = SESSION + "/📤️mesh-export/🦀️.rs"
plan.edit(mx, "type Progress = RetainedCloneProgress;\n", "type Progress = RetainedCloneProgress;\n\n#[path = \"🧵️dwg-export/🦀️.rs\"]\nmod dwg_export;\nuse dwg_export::DwgMeshExport;\n\n/// 💾️ The registered exchange outputs of one retained mesh export.\n#[derive(Clone, Copy)]\npub enum MeshExchange { Obj, Stl, Dwg }\nsemio_framework_value::artifact_retire_leaf!(MeshExchange);\n\nimpl MeshExchange {\n    pub fn channel(self) -> &'static str { match self { Self::Obj => \"obj\", Self::Stl => \"stl\", Self::Dwg => \"dwg\" } }\n}\n\n#[derive(semio_framework_value::RetireOwned)]\nenum Writer { Text(MeshTextExport), Dwg(DwgMeshExport) }\n\nimpl Writer {\n    fn new(mesh: &mut Option<semio_framework::MeshData>, format: MeshExchange) -> Result<Self, ValueError> {\n        match format {\n            MeshExchange::Obj => MeshTextExport::new(mesh, MeshTextFormat::Obj).map(Self::Text),\n            MeshExchange::Stl => MeshTextExport::new(mesh, MeshTextFormat::Stl).map(Self::Text),\n            MeshExchange::Dwg => DwgMeshExport::new(mesh).map(Self::Dwg),\n        }\n    }\n    fn ready(&self) -> bool { match self { Self::Text(writer) => writer.ready(), Self::Dwg(writer) => writer.ready() } }\n    fn take_output(&mut self) -> Option<String> { match self { Self::Text(writer) => writer.take_output(), Self::Dwg(writer) => writer.take_output() } }\n    fn demand(&self) -> RetirementDemand { match self { Self::Text(writer) => writer.demand(), Self::Dwg(writer) => writer.demand() } }\n    fn step(&mut self, effect: &mut RetainedAllocationEffect<'_>) -> Result<Progress, ValueError> { match self { Self::Text(writer) => writer.step(effect), Self::Dwg(writer) => writer.step(effect) } }\n}\n")
plan.edit(mx, "    format: MeshTextFormat,\n    preview: Preview,\n    mesh: Option<semio_framework::MeshData>,\n    writer: Option<MeshTextExport>,\n", "    format: MeshExchange,\n    preview: Preview,\n    mesh: Option<semio_framework::MeshData>,\n    writer: Option<Writer>,\n")
plan.edit(mx, "self.writer.as_ref().is_some_and(MeshTextExport::ready)", "self.writer.as_ref().is_some_and(Writer::ready)")
plan.edit(mx, "self.writer = Some(MeshTextExport::new(&mut self.mesh, self.format)", "self.writer = Some(Writer::new(&mut self.mesh, self.format)")
plan.edit(mx, "tolerance: f64, format: MeshTextFormat)", "tolerance: f64, format: MeshExchange)")
session_toml = SESSION + "/📦️packages/🦀️rust/Cargo.toml"
pkg = SESSION + "/📦️packages/🦀️rust"
plan.edit(session_toml, "[dev-dependencies]\n", "semio-s-artifact-stdio-dwg-geometry = { path = \"%s\" }\nsemio-s-artifact-stdio-dwg-write = { path = \"%s\" }\n\n[dev-dependencies]\n" % (rel(pkg, DWG + "/🧬️schema/🧮️geometry/📦️packages/🦀️rust"), rel(pkg, EXPORT + "/📦️packages/🦀️rust")))
plan.create(SESSION + "/📤️mesh-export/🧵️dwg-export/🦀️.rs", read(NEW + "/session-dwg-export.rs"))

# --- brep extension -----------------------------------------------------------------------------------------
ojs = OJ + "/🦀️.rs"
plan.edit(ojs, "use semio_framework_3d_brep_engine::{MeshDecode, MeshPublication, MeshTextFormat, ObjDecode, StlDecode};", "use semio_framework_3d_brep_engine::{MeshDecode, MeshPublication, ObjDecode, StlDecode};")
plan.edit(ojs, "use semio_s_spatial_kernel_semio_session::{NativeMeshExport,", "use semio_s_spatial_kernel_semio_session::{MeshExchange,NativeMeshExport,")
plan.edit(ojs, "use compound_cut::CompoundCutDraft;\n", "use compound_cut::CompoundCutDraft;\n#[path = \"🖊️dwg-decode/🦀️.rs\"]\nmod dwg_decode;\nuse dwg_decode::DwgDecode;\n")
plan.edit(ojs, "    ImportStep,\n    ExportStep,\n    ExportObj,\n    ExportStl,\n", "    ImportStep,\n    ImportDwg,\n    ExportStep,\n    ExportObj,\n    ExportStl,\n    ExportDwg,\n")
plan.edit(ojs, "            Self::ImportStep => \"step-source\",\n", "            Self::ImportStep => \"step-source\",\n            Self::ImportDwg => \"dwg-source\",\n")
plan.edit(ojs, "            Self::ExportObj | Self::ExportStl => \"mesh-export-source\",", "            Self::ExportObj | Self::ExportStl | Self::ExportDwg => \"mesh-export-source\",")
plan.edit(ojs, "            Self::ImportObj | Self::ImportStl => size_of::<(usize, f64)>(),\n            Self::ExportObj | Self::ExportStl => size_of::<SolidPrimitive>() + size_of::<f64>(),", "            Self::ImportObj | Self::ImportStl | Self::ImportDwg => size_of::<(usize, f64)>(),\n            Self::ExportObj | Self::ExportStl | Self::ExportDwg => size_of::<SolidPrimitive>() + size_of::<f64>(),")
plan.edit(ojs, "            Self::ExportObj | Self::ExportStl => size_of::<NativeMeshExport>(),", "            Self::ExportObj | Self::ExportStl | Self::ExportDwg => size_of::<NativeMeshExport>(),")
plan.edit(ojs, "            Self::ImportObj | Self::ImportStl => {\n                let source = exchange_text(input)?;", "            Self::ImportDwg => return Ok(GeometryOutput::DwgDecode(Box::new(DwgDecode::new(exchange_text(input)?, number(input, \"tolerance\", \"value\")?)?))),\n            Self::ImportObj | Self::ImportStl => {\n                let source = exchange_text(input)?;")
plan.edit(ojs, "            Self::ExportObj | Self::ExportStl => {\n                let format = if matches!(self, Self::ExportObj) { MeshTextFormat::Obj } else { MeshTextFormat::Stl };", "            Self::ExportObj | Self::ExportStl | Self::ExportDwg => {\n                let format = match self { Self::ExportObj => MeshExchange::Obj, Self::ExportStl => MeshExchange::Stl, _ => MeshExchange::Dwg };")
plan.edit(ojs, "if matches!(self, Self::ExportObj | Self::ExportStl | Self::ExportStep) {", "if matches!(self, Self::ExportObj | Self::ExportStl | Self::ExportDwg | Self::ExportStep) {")
plan.edit(ojs, "    MeshDecode(MeshDecode),\n    StepDecode", "    MeshDecode(MeshDecode),\n    DwgDecode(Box<DwgDecode>),\n    StepDecode")
plan.edit(ojs, "Self::MeshDecode(_) | Self::MeshExport(_) | Self::StepDecode(_)", "Self::MeshDecode(_) | Self::DwgDecode(_) | Self::MeshExport(_) | Self::StepDecode(_)")
plan.edit(ojs, "GeometryOutput::MeshDecode(_) | GeometryOutput::MeshExport(_) | GeometryOutput::StepDecode(_)", "GeometryOutput::MeshDecode(_) | GeometryOutput::DwgDecode(_) | GeometryOutput::MeshExport(_) | GeometryOutput::StepDecode(_)", count=2)
plan.edit(ojs, "                GeometryOutput::MeshDecode(_) => self.progress.phase = \"mesh-decoding\",\n", "                GeometryOutput::MeshDecode(_) => self.progress.phase = \"mesh-decoding\",\n                GeometryOutput::DwgDecode(decoder) => self.progress.phase = decoder.phase(),\n")
plan.edit(ojs, "        if let GeometryOutput::MeshDecode(decoder) = result {\n            if !decoder.ready() { return decoder.step(", "        if let GeometryOutput::DwgDecode(decoder) = result {\n            if !decoder.ready() { return decoder.step(exchange_text(&data.input)?, effect).map(|receipt| (None, receipt)); }\n            let demand = decoder.demand();\n            if !retained_clone_grant_admits(grant, demand) || !effect.performed(1)? { return Ok((None, empty)); }\n            self.result = Some(GeometryOutput::MeshPublication(session.plan_mesh_publication_from_cursor(decoder.into_cursor()?)?));\n            self.progress.phase = \"mesh-topology\";\n            return Ok((None, Progress { copied_bytes: demand.copy_bytes, ..item() }));\n        }\n        if let GeometryOutput::MeshDecode(decoder) = result {\n            if !decoder.ready() { return decoder.step(")
plan.edit(ojs, "        if let GeometryOutput::MeshDecode(decoder) = result { return Ok(decoder.demand()); }\n", "        if let GeometryOutput::MeshDecode(decoder) = result { return Ok(decoder.demand()); }\n        if let GeometryOutput::DwgDecode(decoder) = result { return Ok(decoder.demand()); }\n")
plan.edit(ojs, "        if let Some(GeometryOutput::MeshDecode(decoder)) = &self.result {\n", "        if let Some(GeometryOutput::DwgDecode(decoder)) = &self.result {\n            if !decoder.terminal_is_empty() { return Ok(decoder.close_demand()); }\n        }\n        if let Some(GeometryOutput::MeshDecode(decoder)) = &self.result {\n")
plan.edit(ojs, "        if let Some(GeometryOutput::MeshDecode(decoder)) = &mut self.result {\n", "        if let Some(GeometryOutput::DwgDecode(decoder)) = &mut self.result {\n            if !decoder.terminal_is_empty() { return decoder.close(effect); }\n        }\n        if let Some(GeometryOutput::MeshDecode(decoder)) = &mut self.result {\n")
plan.create(OJ + "/🖊️dwg-decode/🦀️.rs", read(NEW + "/original-jobs-dwg-decode.rs"))
oj_toml = OJ + "/📦️packages/🦀️rust/Cargo.toml"
oj_pkg = OJ + "/📦️packages/🦀️rust"
plan.edit(oj_toml, "semio-s-artifact-stdio-step-geometry.workspace = true\n", "semio-s-artifact-stdio-step-geometry.workspace = true\nsemio-s-artifact-stdio-dwg-geometry = { path = \"%s\" }\nsemio-s-artifact-stdio-dwg-read = { path = \"%s\" }\n" % (rel(oj_pkg, DWG + "/🧬️schema/🧮️geometry/📦️packages/🦀️rust"), rel(oj_pkg, DWG + "/🚪️io/📥️import/📦️packages/🦀️rust")))

measure = BREP + "/📏️measure-utilities-and-io/🦀️.rs"
immediate = "    fn step_plan(&self,input:Dictionary,grant:semio_framework_value::RetainedCloneGrant)->Result<(neural_engine::OperatorPlanAdmission,semio_framework_value::RetainedCloneProgress),(EvalError,Dictionary)>{neural_engine::OperatorPlanAdmission::immediate(input,grant)}\n    fn next_plan_copy_byte_demand(&self,_input:&Dictionary)->Result<usize,semio_framework_value::ValueError>{Ok(0)}\n    fn next_plan_capacity_byte_demand(&self,_input:&Dictionary,_maximum_copy_bytes:usize)->Result<usize,semio_framework_value::ValueError>{Ok(0)}\n    fn next_plan_release_byte_demand(&self,_input:&Dictionary)->Result<usize,semio_framework_value::ValueError>{Ok(0)}\n    fn next_plan_depth_demand(&self,_input:&Dictionary)->Result<usize,semio_framework_value::ValueError>{Ok(1)}\n            retire_geometry_capture!(0);\n"
for name in ("ExportDwg", "ImportDwg"):
    plan.edit(measure, "impl Operator for %s {\n%s" % (name, immediate), "impl Operator for %s {\n    retained_geometry_job!(%s);\n    retire_geometry_capture!(0);\n" % (name, name))

law = BREP + "/🧪️tests/🔬️unit/📂️original-exchange.rs"
plan.edit(law, "fn original_mesh_import(operator: &str,", read(NEW + "/brep-dwg-law.rs") + "\nfn original_mesh_import(operator: &str,")

mode = sys.argv[1] if len(sys.argv) > 1 else "prepare"
os.makedirs(OUT, exist_ok=True)
if mode == "prepare":
    for old in os.listdir(OUT):
        if old.endswith(".diff"): os.remove(os.path.join(OUT, old))
    for path, text in list(plan.edits.items()) + list(plan.files.items()):
        before = read(path).splitlines(keepends=True) if os.path.exists(path) else []
        diff = difflib.unified_diff(before, text.splitlines(keepends=True), "a/" + os.path.relpath(path, ROOT), "b/" + os.path.relpath(path, ROOT))
        name = os.path.relpath(path, ROOT).replace("/", "__")[-120:]
        open(os.path.join(OUT, name + ".diff"), "w", encoding="utf-8").writelines(diff)
    print(len(plan.edits), "edited,", len(plan.files), "created")
elif mode == "apply":
    for path, text in list(plan.edits.items()) + list(plan.files.items()):
        os.makedirs(os.path.dirname(path), exist_ok=True)
        open(path, "w", encoding="utf-8").write(text)
    print("applied", len(plan.edits) + len(plan.files))
