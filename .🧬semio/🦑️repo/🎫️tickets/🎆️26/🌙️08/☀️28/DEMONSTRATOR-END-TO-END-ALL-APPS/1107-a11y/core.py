#!/usr/bin/env python3
"""Staged CORE edits for audit items 17/18 (localized MediaPortSpec.label + terminology_breadcrumbs).
Dry run by default: every old text must occur exactly COUNT times in the current tree (stale = exit 1).
Apply ONLY after C/🗑️generated/1105b/hub-ok-1107 exists:  python3 -I core.py --write  &&  python3 -I localize.py --write
Then: LOCK NEEDED for the two Cargo.toml edits (ui-locale gets pack-json). Stdlib only."""
import sys
import os as _o
R=_o.environ.get("A11Y_ROOT","/Users/ueli/Documents/semio/")
FW=R+"🧰️framework/"
OS=FW+"🛍️products/💻️os/🔨️modules/"
M=FW+"🔨️modules/"
WF=OS+"🔁️workflow/🗿️artifacts/🔁️workflow/"
SH=OS+"📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/"
STG=SH+"🎯️targets/🧊️wgpu/📥️staging/"
LBL=M+"🖱️ui/🌐️locale/🏷️label/"
DOC="""    /// 🗣️ The port's own name, resolved per locale and terminology — see `LocalizedLabel`.
"""
E=[]  # (path, old, new, count)
def e(p,o,n,c=1): E.append((p,o,n,c))

# ---- 1. ui-locale: row newtype so a label can be a canonical JSON object, plus the matrix text codec
L=LBL+"🦀️.rs"
e(L,"    cells: [[Cow<'static, str>; Locale::COUNT]; Terminology::COUNT],","    cells: [LabelRow; Terminology::COUNT],")
e(L,"            std::array::from_fn(|li| Cow::Owned(resolve(terminology, Locale::ALL[li])))","            LabelRow(std::array::from_fn(|li| Cow::Owned(resolve(terminology, Locale::ALL[li]))))")
e(L,"Ok(LocalizedLabel { cells: rows.map(|row| row.expect(\"complete terminology matrix\").0.map(Cow::Owned)) })","Ok(LocalizedLabel { cells: rows.map(|row| LabelRow(row.expect(\"complete terminology matrix\").0.map(Cow::Owned))) })")
e(L,"Ok(Self { cells: cells.map(|row| row.map(|text| Cow::Owned(text.to_string()))) })","Ok(Self { cells: cells.map(|row| LabelRow(row.map(|text| Cow::Owned(text.to_string())))) })")
e(L,"let mut cells = std::array::from_fn(|_| std::array::from_fn(|_| Cow::Borrowed(\"\")));","let mut cells: [LabelRow; Terminology::COUNT] = std::array::from_fn(|_| LabelRow(std::array::from_fn(|_| Cow::Borrowed(\"\"))));")
e(L,"#[path = \"♻️retirement/🦀️.rs\"]\nmod retirement;\n","""#[path = "♻️retirement/🦀️.rs"]
mod retirement;

/// 🧱️ One terminology row of a label: the locale cells, a named type so the row is its own canonical JSON object.
#[derive(Clone, Debug, PartialEq)]
struct LabelRow([Cow<'static, str>; Locale::COUNT]);

impl std::ops::Deref for LabelRow { type Target = [Cow<'static, str>; Locale::COUNT]; fn deref(&self) -> &Self::Target { &self.0 } }
impl std::ops::DerefMut for LabelRow { fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 } }
impl<'a> IntoIterator for &'a LabelRow { type Item = &'a Cow<'static, str>; type IntoIter = std::slice::Iter<'a, Cow<'static, str>>; fn into_iter(self) -> Self::IntoIter { self.0.iter() } }
impl<'a> IntoIterator for &'a mut LabelRow { type Item = &'a mut Cow<'static, str>; type IntoIter = std::slice::IterMut<'a, Cow<'static, str>>; fn into_iter(self) -> Self::IntoIter { self.0.iter_mut() } }

/// 🔤️ Canonical JSON object keys are emitted in ascending key order, whatever order the axes are declared in.
fn ascending<T: Copy, const N: usize>(all: [T; N], key: impl Fn(&T) -> &'static str, ordinal: usize) -> Result<T, ValueError> {
    let mut sorted = all;
    sorted.sort_by_key(|item| key(item));
    sorted.get(ordinal).copied().ok_or_else(|| ValueError::literal(ValueRefusalKind::InvariantViolated, "canonical label ordinal is absent"))
}

impl semio_framework_pack_json::ArtifactCanonicalJsonTree for LabelRow {
    fn canonical_tree_node(&self) -> Result<semio_framework_pack_json::ArtifactCanonicalJsonNode<'_>, ValueError> { Ok(semio_framework_pack_json::ArtifactCanonicalJsonNode::Object(Locale::COUNT)) }
    fn canonical_tree_child(&self, ordinal: usize) -> Result<&dyn semio_framework_pack_json::ArtifactCanonicalJsonTree, ValueError> { Ok(&self.0[ascending(Locale::ALL, |l| l.as_str(), ordinal)?.index()]) }
    fn canonical_tree_key(&self, ordinal: usize) -> Result<semio_framework_pack_json::ArtifactCanonicalJsonText<'_>, ValueError> { Ok(ascending(Locale::ALL, |l| l.as_str(), ordinal)?.as_str().into()) }
}

impl semio_framework_pack_json::ArtifactCanonicalJsonTree for LocalizedLabel {
    fn canonical_tree_node(&self) -> Result<semio_framework_pack_json::ArtifactCanonicalJsonNode<'_>, ValueError> { Ok(semio_framework_pack_json::ArtifactCanonicalJsonNode::Object(Terminology::COUNT)) }
    fn canonical_tree_child(&self, ordinal: usize) -> Result<&dyn semio_framework_pack_json::ArtifactCanonicalJsonTree, ValueError> { Ok(&self.cells[ascending(Terminology::ALL, |t| t.as_str(), ordinal)?.index()]) }
    fn canonical_tree_key(&self, ordinal: usize) -> Result<semio_framework_pack_json::ArtifactCanonicalJsonText<'_>, ValueError> { Ok(ascending(Terminology::ALL, |t| t.as_str(), ordinal)?.as_str().into()) }
}
""")
e(LBL+"🛬️receiving/🦀️.rs","Self{cells:[[Cow::Owned(value.native.en),Cow::Owned(value.native.de)],[Cow::Owned(value.reuse.en),Cow::Owned(value.reuse.de)]]}","Self{cells:[LabelRow([Cow::Owned(value.native.en),Cow::Owned(value.native.de)]),LabelRow([Cow::Owned(value.reuse.en),Cow::Owned(value.reuse.de)])]}")
e(LBL+"♻️retirement/🦀️.rs","cells:ManuallyDrop<[[Cow<'static,str>;Locale::COUNT];Terminology::COUNT]>","cells:ManuallyDrop<[super::LabelRow;Terminology::COUNT]>")
e(M+"🖱️ui/🌐️locale/📦️packages/🦀️rust/Cargo.toml","semio-framework-value = { path = \"../../../../🌱️value/📦️packages/🦀️rust\" }\n","semio-framework-value = { path = \"../../../../🌱️value/📦️packages/🦀️rust\" }\nsemio-framework-pack-json = { path = \"../../../../🎒️pack/🔤️json/📦️packages/🦀️rust\" }\n")
e(M+"🎒️pack/🔤️json/🛫️encode/🧭️tree/🦀️.rs","impl ArtifactCanonicalJsonTree for String { fn canonical_tree_node(&self)->Result<Node<'_>,ValueError>{Ok(Node::String(self))} }\n","impl ArtifactCanonicalJsonTree for String { fn canonical_tree_node(&self)->Result<Node<'_>,ValueError>{Ok(Node::String(self))} }\nimpl ArtifactCanonicalJsonTree for std::borrow::Cow<'static,str> { fn canonical_tree_node(&self)->Result<Node<'_>,ValueError>{Ok(Node::String(self))} }\n")

# ---- 2. MediaPortSpec schema (Rust owner) and its owned TS projection rows
MP=M+"🛂️manifest/🎞️media/🦀️.rs"
e(MP,"    pub id: String,\n    pub label: String,\n    pub direction: MediaPortDirection,","    pub id: String,\n"+DOC+"    pub label: LocalizedLabel,\n    pub direction: MediaPortDirection,")
TSF="""/**
 * 🗣️ The port's own name, resolved per locale and terminology — see `LocalizedLabel`.
 */
"""
e(M+"🧬️schema/📽️projection/🎟️rows-h-t/🦀️.rs","export type MediaPortSpec = { id: string, label: string, direction:","export type MediaPortSpec = { id: string,\n"+TSF+"label: unknown, direction:")
e(M+"🛂️manifest/🤖️generated/🪪️manifest/🟦️.ts","export type MediaPortSpec = { id: string, label: string, direction:","export type MediaPortSpec = { id: string,\n"+TSF+"label: unknown, direction:")

# ---- 3. AppDefinition.terminology_breadcrumbs + resolver + builders
AD=M+"🛂️manifest/🧱️app-surface/🧬️app-definition/🦀️.rs"
e(AD,"""    /// 🗺️ Terminology id -> full replacement breadcrumb (product + app segments), e.g. "reuse" ->
    /// ["Entwerfen mit Bestand", "Aggregator"]; ids absent here keep the canonical breadcrumb under that terminology.""","""    /// 🗺️ Terminology id -> full replacement breadcrumb (product + app segments), every segment a `LocalizedLabel`, e.g. "reuse" ->
    /// [Designing with Existing Stock | Entwerfen mit Bestand, Aggregator]; ids absent here keep the canonical breadcrumb under that terminology.""")
e(AD,"pub terminology_breadcrumbs: std::collections::HashMap<String, Vec<String>>,","pub terminology_breadcrumbs: std::collections::HashMap<String, Vec<LocalizedLabel>>,")
BC=M+"🛂️manifest/🧱️app-surface/📚️breadcrumbs-examples-and-command-grammar/🦀️.rs"
e(BC,"""/// 🗺️ Resolves the breadcrumb effective under the active terminology; unknown/native ids fall back to the canonical breadcrumb.
pub fn resolve_app_breadcrumb<'a>(app: &'a AppDefinition, terminology: &str) -> &'a [String] {
    app.terminology_breadcrumbs.get(terminology).map_or(&app.breadcrumb, Vec::as_slice)
}""","""/// 🗺️ Resolves the breadcrumb effective under the active terminology and `locale`; unknown/native ids fall back to the canonical breadcrumb.
pub fn resolve_app_breadcrumb(app: &AppDefinition, terminology: &str, locale: Locale) -> Vec<String> {
    let axis = Terminology::parse(terminology).unwrap_or(Terminology::Native);
    app.terminology_breadcrumbs.get(terminology).map_or_else(|| app.breadcrumb.clone(), |segments| segments.iter().map(|segment| segment.resolve(axis, locale).to_string()).collect())
}""")
e(BC,"let mut breadcrumb = resolve_app_breadcrumb(app, terminology).to_vec();","let mut breadcrumb = resolve_app_breadcrumb(app, terminology, locale);")
PL=OS+"🔌️plugin/"
e(PL+"🏗️builder/🧩️app/🦀️.rs","terminology_breadcrumbs: HashMap<String, Vec<String>>,","terminology_breadcrumbs: HashMap<String, Vec<LocalizedLabel>>,")
e(PL+"🏗️builder/🧩️app/🧱️builder-identity-and-modes/🦀️.rs","""pub async fn terminology_document(mut self, id: impl Into<String>, document: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.terminology_breadcrumbs.insert(id.into(), document.into_iter().map(Into::into).collect());""","""pub async fn terminology_document(mut self, id: impl Into<String>, document: impl IntoIterator<Item = LocalizedLabel>) -> Self {
        self.terminology_breadcrumbs.insert(id.into(), document.into_iter().collect());""")
e(PL+"📜️declaration/🪟️surface/🏗️surface-builders/🦀️.rs","pub fn terminology_document(mut self, id: impl Into<String>, document: impl IntoIterator<Item = impl Into<String>>) -> Self {","pub fn terminology_document(mut self, id: impl Into<String>, document: impl IntoIterator<Item = LocalizedLabel>) -> Self {",2)
e(PL+"🏗️builder/🧩️app/🏗️try-build-definition/🦀️.rs","if !(!document.is_empty() && document.iter().all(|segment| !segment.trim().is_empty())) {","if !(!document.is_empty() && document.iter().all(|segment| !segment.has_blank_cell())) {")
# renderer (wgpu) twin: the title resolves in the session locale
e(STG+"🚶️session/🧱️chrome-paint-and-navbar/🦀️.rs","""pub fn shell_navbar_title(app: &AppDefinition, terminology_id: &str) -> String {
    semio_framework::manifest::app_breadcrumb(semio_framework::manifest::resolve_app_breadcrumb(app, terminology_id))""","""pub fn shell_navbar_title(app: &AppDefinition, terminology_id: &str, locale: semio_ui_locale::Locale) -> String {
    semio_framework::manifest::app_breadcrumb(&semio_framework::manifest::resolve_app_breadcrumb(app, terminology_id, locale))""")
LOC="semio_ui_locale::Locale::parse(&self.prefs.locale_id)"  # CALLERS: confirm the parse helper's name at apply time
e(STG+"🚶️session/⌨️keyboard-and-palette/🦀️.rs","shell_navbar_title(app, &self.prefs.terminology_id)","shell_navbar_title(app, &self.prefs.terminology_id, "+LOC+")")
e(STG+"🍱️view-chrome/🖼️chrome-frame-and-main-window/🦀️.rs","shell_navbar_title(&session.app, &self.prefs.terminology_id)","shell_navbar_title(&session.app, &self.prefs.terminology_id, "+LOC+")")
e(STG+"🍱️view-chrome/🧭️navbar-footer-and-overlays/🦀️.rs","shell_navbar_title(&session.app, &self.prefs.terminology_id)","shell_navbar_title(&session.app, &self.prefs.terminology_id, "+LOC+")")

# ---- 4. tests (Rust)
AL=M+"🛂️manifest/🧪️tests/🔬️app-label/🦀️.rs"
e(AL,"""    app.terminology_breadcrumbs.insert("de".into(), vec!["semio".into(), "a-de".into()]);
    assert_eq!(resolve_app_breadcrumb(&app, "de"), ["semio".to_string(), "a-de".to_string()]);
    assert_eq!(resolve_app_breadcrumb(&app, "native"), app.breadcrumb.as_slice());
    assert_eq!(resolve_app_breadcrumb(&app, "unregistered"), app.breadcrumb.as_slice());""","""    app.terminology_breadcrumbs.insert("reuse".into(), vec![LocalizedLabel::data("semio"), LocalizedLabel::native("a-en", "a-de")]);
    assert_eq!(resolve_app_breadcrumb(&app, "reuse", Locale::De), ["semio".to_string(), "a-de".to_string()]);
    assert_eq!(resolve_app_breadcrumb(&app, "reuse", Locale::En), ["semio".to_string(), "a-en".to_string()]);
    assert_eq!(resolve_app_breadcrumb(&app, "native", Locale::En), app.breadcrumb);
    assert_eq!(resolve_app_breadcrumb(&app, "unregistered", Locale::De), app.breadcrumb);""")
AB=PL+"🧪️tests/🔬️app-app-builder/🎛️descriptor-owner-layouts-terminology-and-history/🦀️.rs"
e(AB,'.terminology_document("reuse", ["Entwerfen mit Bestand", "Bad"])','.terminology_document("reuse", [LocalizedLabel::native("Designing with Existing Stock", "Entwerfen mit Bestand"), LocalizedLabel::data("Bad")])')
e(AB,'.terminology_document("reuse", ["Entwerfen mit Bestand", "Aggregator"])','.terminology_document("reuse", [LocalizedLabel::native("Designing with Existing Stock", "Entwerfen mit Bestand"), LocalizedLabel::data("Aggregator")])')
e(AB,'assert_eq!(definition.terminology_breadcrumbs.get("reuse").map(Vec::as_slice), Some(["Entwerfen mit Bestand".to_string(), "Aggregator".to_string()].as_slice()));','let resolved: Vec<_> = definition.terminology_breadcrumbs["reuse"].iter().map(|segment| (segment.resolve(Terminology::Reuse, Locale::En).to_string(), segment.resolve(Terminology::Reuse, Locale::De).to_string())).collect();\n        assert_eq!(resolved, [("Designing with Existing Stock".to_string(), "Entwerfen mit Bestand".to_string()), ("Aggregator".to_string(), "Aggregator".to_string())]);')
PT=SH+"🧪️tests/🧭️wgpu-navbar-footer-parity/🦀️.rs"
e(PT,'''app.terminology_breadcrumbs = std::collections::HashMap::from([("reuse".to_string(), vec!["Entwerfen mit Bestand".to_string(), "Aggregator".to_string()])]);''','''app.terminology_breadcrumbs = std::collections::HashMap::from([("reuse".to_string(), vec![semio_ui_locale::LocalizedLabel::native("Designing with Existing Stock", "Entwerfen mit Bestand"), semio_ui_locale::LocalizedLabel::data("Aggregator")])]);''')
e(PT,'''assert_eq!(shell_navbar_title(&app, "reuse"), "Entwerfen mit Bestand · Aggregator",''','''assert_eq!(shell_navbar_title(&app, "reuse", semio_ui_locale::Locale::De), "Entwerfen mit Bestand · Aggregator",''')
e(PT,'''assert_eq!(shell_navbar_title(&app, "native"), "workspace · examples · scene")''','''assert_eq!(shell_navbar_title(&app, "native", semio_ui_locale::Locale::En), "workspace · examples · scene")''',2)
e(PT,'''!shell_navbar_title(&app, "native").contains(&app.id)''','''!shell_navbar_title(&app, "native", semio_ui_locale::Locale::En).contains(&app.id)''')
e(PT,'''assert_eq!(shell_navbar_title(&bare, "native"), "",''','''assert_eq!(shell_navbar_title(&bare, "native", semio_ui_locale::Locale::En), "",''')

# ---- 5. workflow: the port label is a typed record (terminology rows of locale texts) in the record DSL, the borrowed carrier, the sqlite tables and the TS twin
G=WF+"🕸️graph/"
BR=G+"🫳️borrowed-record-specs/🦀️.rs"
MPF=G+"🔌️media-port/🦀️.rs"
LABEL_CODE = r'''
//#region 🏷️LocalizedLabelRecord
const _: () = assert!(semio_ui_locale::Locale::COUNT == 2 && semio_ui_locale::Terminology::COUNT == 2, "🏷️ the borrowed label carrier below spells the axes `native`/`reuse` x `en`/`de`; extend it with the axes");

struct WorkflowLabelRow;
workflow_borrowed_record!(WorkflowLabelRow;
 BorrowedFieldSpec::new(0,"en",BorrowedShape::Text),BorrowedFieldSpec::new(1,"de",BorrowedShape::Text));

struct WorkflowLabelMatrix;
workflow_borrowed_record!(WorkflowLabelMatrix;
 BorrowedFieldSpec::new(0,"native",<WorkflowLabelRow as semio_framework_dsl_record::BorrowedDslField>::SHAPE),BorrowedFieldSpec::new(1,"reuse",<WorkflowLabelRow as semio_framework_dsl_record::BorrowedDslField>::SHAPE));

/// 🏷️ The shape of a port label in the borrowed carrier.
pub(super) const WORKFLOW_LABEL_BORROWED_SHAPE:semio_framework_dsl_record::BorrowedShape=<WorkflowLabelMatrix as semio_framework_dsl_record::BorrowedDslField>::SHAPE;

fn workflow_label_row_spec()->semio_framework_dsl_record::RecordSpec{
    semio_framework_dsl_record::RecordSpec::new(None,semio_framework_dsl_record::RecordLayout::Inline,semio_ui_locale::Locale::ALL.iter().map(|locale|semio_framework_dsl_record::FieldSpec::new(locale.index() as u16,locale.as_str(),semio_framework_dsl_record::Shape::Text)).collect())
}

fn workflow_label_row_spec_controlled<C:semio_framework_dsl_record::NativeSchemaControl>(control:&mut C)->Result<semio_framework_dsl_record::RecordSpec,semio_framework_value::ValueError>{
    control.scoped_stage(|control|{control.begin_stage(semio_ui_locale::Locale::COUNT)?;let mut fields=control.allocate_vec::<semio_framework_dsl_record::FieldSpec>(semio_ui_locale::Locale::COUNT)?;
        for locale in semio_ui_locale::Locale::ALL{let shape=semio_framework_dsl_record::Shape::Text;fields.push(semio_framework_dsl_record::producer::field(locale.index() as u16,locale.as_str(),shape,control)?);control.step()?;}
        semio_framework_dsl_record::producer::record(None,semio_framework_dsl_record::RecordLayout::Inline,fields,control)
    })
}

fn workflow_label_row_producer()->semio_framework_dsl_record::RecordSpecProducer{semio_framework_dsl_record::RecordSpecProducer{ordinary:workflow_label_row_spec,decoding:|control|workflow_label_row_spec_controlled(control),encoding:|control|workflow_label_row_spec_controlled(control)}}

fn workflow_label_matrix_spec()->semio_framework_dsl_record::RecordSpec{
    semio_framework_dsl_record::RecordSpec::new(None,semio_framework_dsl_record::RecordLayout::Inline,semio_ui_locale::Terminology::ALL.iter().map(|terminology|semio_framework_dsl_record::FieldSpec::new(terminology.index() as u16,terminology.as_str(),semio_framework_dsl_record::Shape::Record(workflow_label_row_producer()))).collect())
}

fn workflow_label_matrix_spec_controlled<C:semio_framework_dsl_record::NativeSchemaControl>(control:&mut C)->Result<semio_framework_dsl_record::RecordSpec,semio_framework_value::ValueError>{
    control.scoped_stage(|control|{control.begin_stage(semio_ui_locale::Terminology::COUNT)?;let mut fields=control.allocate_vec::<semio_framework_dsl_record::FieldSpec>(semio_ui_locale::Terminology::COUNT)?;
        for terminology in semio_ui_locale::Terminology::ALL{let shape=semio_framework_dsl_record::Shape::Record(workflow_label_row_producer());fields.push(semio_framework_dsl_record::producer::field(terminology.index() as u16,terminology.as_str(),shape,control)?);control.step()?;}
        semio_framework_dsl_record::producer::record(None,semio_framework_dsl_record::RecordLayout::Inline,fields,control)
    })
}

fn workflow_label_matrix_producer()->semio_framework_dsl_record::RecordSpecProducer{semio_framework_dsl_record::RecordSpecProducer{ordinary:workflow_label_matrix_spec,decoding:|control|workflow_label_matrix_spec_controlled(control),encoding:|control|workflow_label_matrix_spec_controlled(control)}}

/// 🏷️ The shape of a port label in the record DSL.
pub(super) fn workflow_label_shape()->semio_framework_dsl_record::Shape{semio_framework_dsl_record::Shape::Record(workflow_label_matrix_producer())}

/// 🏷️ The shape of a port label in the controlled schema lane.
pub(super) fn workflow_label_shape_controlled<C:semio_framework_dsl_record::NativeSchemaControl>(control:&mut C)->Result<semio_framework_dsl_record::Shape,semio_framework_value::ValueError>{control.checkpoint()?;Ok(semio_framework_dsl_record::Shape::Record(workflow_label_matrix_producer()))}

/// 🏷️ A port label as its record value.
pub(super) fn workflow_label_to_value(label:&semio_ui_locale::LocalizedLabel)->semio_framework_dsl_record::FieldValue{
    let mut matrix=semio_framework_dsl_record::RecordValue::default();
    for terminology in semio_ui_locale::Terminology::ALL{
        let mut row=semio_framework_dsl_record::RecordValue::default();
        for locale in semio_ui_locale::Locale::ALL{row.fields.insert(locale.index() as u16,semio_framework_dsl_record::FieldValue::Text(label.resolve(terminology,locale).to_string()));}
        matrix.fields.insert(terminology.index() as u16,semio_framework_dsl_record::FieldValue::Record(row));
    }
    semio_framework_dsl_record::FieldValue::Record(matrix)
}

/// 🏷️ A port label from its record value; refuses a missing terminology row or locale cell.
pub(super) fn workflow_label_from_value(value:&semio_framework_dsl_record::FieldValue)->Result<semio_ui_locale::LocalizedLabel,String>{
    let semio_framework_dsl_record::FieldValue::Record(matrix)=value else{return Err("expected label record".to_string())};
    let mut refusal=None;
    let label=semio_ui_locale::LocalizedLabel::from_fn(|terminology,locale|match matrix.get(terminology.index() as u16){
        Some(semio_framework_dsl_record::FieldValue::Record(row))=>match row.get(locale.index() as u16){Some(semio_framework_dsl_record::FieldValue::Text(text))=>text.clone(),_=>{refusal.get_or_insert(format!("expected label text for {}.{}",terminology.as_str(),locale.as_str()));String::new()}},
        _=>{refusal.get_or_insert(format!("expected label row for {}",terminology.as_str()));String::new()}
    });
    refusal.map_or(Ok(label),Err)
}

fn workflow_native_project_label(record:&mut semio_framework_dsl_record::native_encoding::EncodedRecord,id:u16,label:&semio_ui_locale::LocalizedLabel,control:&mut semio_framework_value::NativeEncodeControl<'_>)->Result<(),semio_framework_value::ValueError>{
    let matrix=control.scoped_stage(|control|{control.begin_stage(semio_ui_locale::Terminology::COUNT)?;let mut matrix=semio_framework_dsl_record::native_encoding::EncodedRecord::new(semio_ui_locale::Terminology::COUNT,control)?;
        for terminology in semio_ui_locale::Terminology::ALL{
            let row=control.scoped_stage(|control|{control.begin_stage(semio_ui_locale::Locale::COUNT)?;let mut row=semio_framework_dsl_record::native_encoding::EncodedRecord::new(semio_ui_locale::Locale::COUNT,control)?;
                for locale in semio_ui_locale::Locale::ALL{row.insert(locale.index() as u16,semio_framework_dsl_record::FieldValue::Text(control.copy_text(label.resolve(terminology,locale))?))?;control.step()?;}
                Ok::<_,semio_framework_value::ValueError>(row.take())
            })?;
            matrix.insert(terminology.index() as u16,semio_framework_dsl_record::FieldValue::Record(row))?;control.step()?;
        }
        Ok::<_,semio_framework_value::ValueError>(matrix.take())
    })?;
    record.insert(id,semio_framework_dsl_record::FieldValue::Record(matrix))?;control.step()
}

fn workflow_native_field_label(record:&semio_framework_dsl_record::RecordValue,id:u16,control:&mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>)->Result<semio_ui_locale::LocalizedLabel,semio_framework_value::ValueError>{
    let Some(semio_framework_dsl_record::FieldValue::Record(matrix))=record.get(id) else{return Err(workflow_invalid("expected declared label record"))};
    let mut cells:[[String;semio_ui_locale::Locale::COUNT];semio_ui_locale::Terminology::COUNT]=Default::default();
    control.scoped_stage(|control|{control.begin_stage(semio_ui_locale::Terminology::COUNT*semio_ui_locale::Locale::COUNT)?;
        for terminology in semio_ui_locale::Terminology::ALL{
            let Some(semio_framework_dsl_record::FieldValue::Record(row))=matrix.get(terminology.index() as u16) else{return Err(workflow_invalid("expected declared label row"))};
            for locale in semio_ui_locale::Locale::ALL{
                let Some(semio_framework_dsl_record::FieldValue::Text(text))=row.get(locale.index() as u16) else{return Err(workflow_invalid("expected declared label text"))};
                cells[terminology.index()][locale.index()]=control.copy_text(text)?;control.step()?;
            }
        }
        Ok::<(),semio_framework_value::ValueError>(())
    })?;
    Ok(semio_ui_locale::LocalizedLabel::from_fn(|terminology,locale|std::mem::take(&mut cells[terminology.index()][locale.index()])))
}
//#endregion 🏷️LocalizedLabelRecord
'''
e(BR,"//#endregion 🔖️MediaContractDsl",LABEL_CODE+"//#endregion 🔖️MediaContractDsl")
e(BR,'BorrowedFieldSpec::new(2,"label",BorrowedShape::Text),BorrowedFieldSpec::new(3,"direction"','BorrowedFieldSpec::new(2,"label",WORKFLOW_LABEL_BORROWED_SHAPE),BorrowedFieldSpec::new(3,"direction"')
e(BR,"workflow_native_project(&mut record,2,&value.spec.label,control)?;","workflow_native_project_label(&mut record,2,&value.spec.label,control)?;")
e(BR,"let label=workflow_native_field(record,2,control)?;","let label=workflow_native_field_label(record,2,control)?;")
e(MPF,'let shape=semio_framework_dsl_record::Shape::Text;fields.push(semio_framework_dsl_record::producer::field(2,"label",shape,control)?);','let shape=workflow_label_shape_controlled(control)?;fields.push(semio_framework_dsl_record::producer::field(2,"label",shape,control)?);')
e(MPF,'semio_framework_dsl_record::FieldSpec::new(2, "label", semio_framework_dsl_record::Shape::Text),','semio_framework_dsl_record::FieldSpec::new(2, "label", workflow_label_shape()),')
e(MPF,"semio_framework_dsl_record::FieldValue::Text(port.spec.label.clone())","workflow_label_to_value(&port.spec.label)")
e(MPF,"""    let label = match record.get(2) {
        Some(semio_framework_dsl_record::FieldValue::Text(s)) => s.clone(),
        other => return Err(semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,(format!("expected label, found {other:?}")).to_string(),semio_framework_diagnostic::TextSpan::at(1,1))),
    };""","""    let label = workflow_label_from_value(record.get(2).unwrap_or(&semio_framework_dsl_record::FieldValue::Absent)).map_err(|message| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,message,semio_framework_diagnostic::TextSpan::at(1,1)))?;""")
# sqlite: four label columns instead of one, in the DDL, the Rust codec and the TS twin
SQ=WF+"🚪️io/🪶️sqlite/📸️snapshot/"
e(SQ+"🗄️.sql","port_id TEXT NOT NULL, label TEXT NOT NULL, direction INTEGER","port_id TEXT NOT NULL, label_native_en TEXT NOT NULL, label_native_de TEXT NOT NULL, label_reuse_en TEXT NOT NULL, label_reuse_de TEXT NOT NULL, direction INTEGER")
e(SQ+"🦀️.rs","const NODE_GEOMETRY:","""fn port_label(row:&SqliteRow,first:usize,native:&mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>)->Result<semio_ui_locale::LocalizedLabel,ValueError>{
 let mut cells:[[String;semio_ui_locale::Locale::COUNT];semio_ui_locale::Terminology::COUNT]=Default::default();
 for terminology in semio_ui_locale::Terminology::ALL{for locale in semio_ui_locale::Locale::ALL{cells[terminology.index()][locale.index()]=native.copy_text(row.text(first+terminology.index()*semio_ui_locale::Locale::COUNT+locale.index())?)?;}}
 Ok(semio_ui_locale::LocalizedLabel::from_fn(|terminology,locale|std::mem::take(&mut cells[terminology.index()][locale.index()])))
}
const PORT_COLUMNS:usize=3+semio_ui_locale::Terminology::COUNT*semio_ui_locale::Locale::COUNT+10;
const NODE_GEOMETRY:""")
e(SQ+"🦀️.rs","Cell::Text(&value.spec.label),","Cell::Text(value.spec.label.resolve(semio_ui_locale::Terminology::Native,semio_ui_locale::Locale::En)),Cell::Text(value.spec.label.resolve(semio_ui_locale::Terminology::Native,semio_ui_locale::Locale::De)),Cell::Text(value.spec.label.resolve(semio_ui_locale::Terminology::Reuse,semio_ui_locale::Locale::En)),Cell::Text(value.spec.label.resolve(semio_ui_locale::Terminology::Reuse,semio_ui_locale::Locale::De)),")
e(SQ+"🦀️.rs",'grouped(&database.table("workflow_port")?.rows,13,&mut native)','grouped(&database.table("workflow_port")?.rows,PORT_COLUMNS,&mut native)')
e(SQ+"🦀️.rs","ordered(rows,13,3,native)","ordered(rows,PORT_COLUMNS,3,native)")
e(SQ+"🦀️.rs","label:native.copy_text(row.text(6)?)?,direction:crate::media_port_direction_from_ordinal(enum32(row.integer(7)?)?)","label:port_label(row,6,native)?,direction:crate::media_port_direction_from_ordinal(enum32(row.integer(10)?)?)")
e(SQ+"🦀️.rs","class:crate::media_class_from_ordinal(enum32(row.integer(8)?)?)","class:crate::media_class_from_ordinal(enum32(row.integer(11)?)?)")
e(SQ+"🦀️.rs","form:crate::media_form_from_ordinal(enum32(row.integer(9)?)?)","form:crate::media_form_from_ordinal(enum32(row.integer(12)?)?)")
e(SQ+"🦀️.rs","kind_id:nullable(row,10,native)?,required:boolean(row.integer(11)?)?,multiplicity:crate::port_multiplicity_from_ordinal(enum32(row.integer(12)?)?)","kind_id:nullable(row,13,native)?,required:boolean(row.integer(14)?)?,multiplicity:crate::port_multiplicity_from_ordinal(enum32(row.integer(15)?)?)")
e(SQ+"🟦️.ts","function label<T extends string>(","""const LABEL_AXES:readonly(readonly["native"|"reuse","en"|"de"])[]=[["native","en"],["native","de"],["reuse","en"],["reuse","de"]];
function labelCells(value:WorkflowPort["spec"]["label"]):string[]{record(value,["native","reuse"]);return LABEL_AXES.map(([terminology,locale])=>{record(value[terminology],["en","de"]);return text(value[terminology][locale])})}
function labelFrom(row:SqliteRow,first:number):WorkflowPort["spec"]["label"]{const cell=(index:number)=>artifactSqliteText(row,first+index);return{native:{en:cell(0),de:cell(1)},reuse:{en:cell(2),de:cell(3)}}}
function label<T extends string>(""")
e(SQ+"🟦️.ts","text(spec.id),text(spec.label),ordinal(spec.direction","text(spec.id),...labelCells(spec.label),ordinal(spec.direction")
e(SQ+"🟦️.ts",'label:artifactSqliteText(row,6),direction:label(row,7,["in","out"]),mediaType:{class:label(row,8,CLASSES),form:label(row,9,FORMS)},required:flag(row,11),multiplicity:label(row,12,["one","many"])};if(row.values[10]!==null)spec.kindId=artifactSqliteText(row,10);','label:labelFrom(row,6),direction:label(row,10,["in","out"]),mediaType:{class:label(row,11,CLASSES),form:label(row,12,FORMS)},required:flag(row,14),multiplicity:label(row,15,["one","many"])};if(row.values[13]!==null)spec.kindId=artifactSqliteText(row,13);')
# owned snapshot schema (JSON Schema + TS twin)
SS=WF+"🧬️schema/📸️snapshot/"
e(SS+"🟦️.ts",'export interface WorkflowPort{id:string;spec:{id:string;label:string;direction:','export interface WorkflowLocalizedLabel{native:{en:string;de:string};reuse:{en:string;de:string}}\nexport interface WorkflowPort{id:string;spec:{id:string;label:WorkflowLocalizedLabel;direction:')
LL='''    "LocalizedLabel": {
      "type": "object",
      "additionalProperties": false,
      "required": ["native", "reuse"],
      "properties": {
        "native": { "$ref": "#/$defs/LocalizedLabelRow" },
        "reuse": { "$ref": "#/$defs/LocalizedLabelRow" }
      }
    },
    "LocalizedLabelRow": {
      "type": "object",
      "additionalProperties": false,
      "required": ["en", "de"],
      "properties": {
        "en": { "type": "string" },
        "de": { "type": "string" }
      }
    },
    "Port": {'''
e(SS+"🔣️.json",'    "Port": {',LL)
e(SS+"🔣️.json",'''            "id": {
              "type": "string"
            },
            "label": {
              "type": "string"
            },
            "direction": {''','''            "id": {
              "type": "string"
            },
            "label": {
              "$ref": "#/$defs/LocalizedLabel"
            },
            "direction": {''')
# fixtures, by hand: sqlite round-trip graph (distinct cells per axis so a swapped column cannot pass), mutation witnesses, binding
FX=SQ+"🧪️tests/🧫️fixtures/"
def lab(n_en,n_de,r_en,r_de,ind):
    p=" "*ind
    return ('{\n'+p+'  "native": { "de": "'+n_de+'", "en": "'+n_en+'" },\n'+p+'  "reuse": { "de": "'+r_de+'", "en": "'+r_en+'" }\n'+p+'}')
e(FX+"🔣️.json",'"label": "\\u0000世界",\n                "direction": "out",','"label": '+lab("\\u0000世界","世界\\u0000","😀","",16)+',\n                "direction": "out",')
e(FX+"🔣️.json",'"label": "",\n                "direction": "in",\n                "mediaType": {\n                  "class": "twoD",','"label": '+lab("a","ä","","b",16)+',\n                "direction": "in",\n                "mediaType": {\n                  "class": "twoD",')
e(FX+"🔣️.json",'"label": "",\n                "direction": "in",\n                "mediaType": {\n                  "class": "presentation",','"label": '+lab("","","","",16)+',\n                "direction": "in",\n                "mediaType": {\n                  "class": "presentation",')
WT=WF+"🧪️tests/🧫️fixtures/"
e(WT+"🧬️mutations/➕️add-node/🧾️wire-witness/🦠️mutation/🔣️.json",'"label": "Out",','"label": { "native": { "de": "Aus", "en": "Out" }, "reuse": { "de": "Aus", "en": "Out" } },')
e(WT+"🧬️mutations/🔧set-node-ports/🧾️wire-witness/🦠️mutation/🔣️.json",'"label": "In",','"label": { "native": { "de": "Ein", "en": "In" }, "reuse": { "de": "Ein", "en": "In" } },')
e(WT+"🪆️binding/🔣️.json",'"label": "😀",','"label": { "native": { "de": "😀", "en": "😀" }, "reuse": { "de": "😀", "en": "😀" } },')
e(WF+"🧪️tests/🔁️workflow/🦀️.rs","MediaPortSpec { id: id.into(), label: id.into(), direction,","MediaPortSpec { id: id.into(), label: semio_ui_locale::LocalizedLabel::data(id), direction,")
# norm: the packed `Model / Modell` string and its tests go away
NO=R+"✏️s/🔌️plugins/📕️norm/📇️registry/🧬️contract/🖥️app-surface/"
e(NO+"🏷️ids-chrome-and-labels/🦀️.rs","""/// 🏷️ `MediaPortSpec.label` is a plain `String` — pack both native terms from a [`LocalizedLabel`].
pub fn media_port_label(label: LocalizedLabel) -> String {
    format!(
        "{} / {}",
        label.resolve(Terminology::Native, Locale::En),
        label.resolve(Terminology::Native, Locale::De),
    )
}

""","")
e(NO+"🪟️manifest-and-media/🦀️.rs",'label: media_port_label(LocalizedLabel::native("Model", "Modell")),','label: LocalizedLabel::native("Model", "Modell"),')
e(NO+"🪟️manifest-and-media/🦀️.rs",'label: media_port_label(LocalizedLabel::native("Report", "Bericht")),','label: LocalizedLabel::native("Report", "Bericht"),')
e(NO+"🧪️tests/🔬️unit/🦀️.rs",'port.id == "model:in" && port.label.contains("Modell")','port.id == "model:in" && port.label.resolve(semio_ui_locale::Terminology::Native, semio_ui_locale::Locale::De) == "Modell"')
e(NO+"🧪️tests/🔬️unit/🦀️.rs",'port.id == "report:out" && port.label.contains("Bericht")','port.id == "report:out" && port.label.resolve(semio_ui_locale::Terminology::Native, semio_ui_locale::Locale::De) == "Bericht"')
e(NO+"🧪️tests/🔬️unit/🦀️.rs",'assert!(!io.ports.iter().any(|port| port.label == "Model" || port.label == "Report"), "{io:?}");','assert!(io.ports.iter().all(|port| port.label.resolve(semio_ui_locale::Terminology::Native, semio_ui_locale::Locale::De) != port.label.resolve(semio_ui_locale::Terminology::Native, semio_ui_locale::Locale::En)), "{io:?}");')

e(OS+"🏃️run/🧪️tests/🔬️unit/🦀️.rs","label: spec_id.into()","label: semio_ui_locale::LocalizedLabel::data(spec_id)")

# ---- 5b. no default language in the workflow: a node's name is data chosen by the caller (the UI knows its locale), never the app label's English cell
GM=G+"🧱️graph-model/🦀️.rs"
e(GM,"pub async fn workflow_node_for_app(app: &AppDefinition, plugin_id: &str, node_id: &str, position: &WorkflowPosition) -> WorkflowNode {","pub async fn workflow_node_for_app(app: &AppDefinition, plugin_id: &str, node_id: &str, position: &WorkflowPosition, label: &str) -> WorkflowNode {")
e(GM,"        label: app.label.resolve(Terminology::Native, Locale::En).to_string(),","        label: label.to_string(),")
WS=FW+"🛍️products/💻️os/🖥️host/🔌️plugin-host/🏪️workflow-store/🦀️.rs"
e(WS,"app_id: &str, label: Option<&str>, x: f64, y: f64, space_store","app_id: &str, label: &str, x: f64, y: f64, space_store")
e(WS,"""        let mut node = resolve_kernel_future(workflow::workflow_node_for_app(&app, plugin_id, &node_id, &position));
        if let Some(label) = label {
            node.label = label.into();
        }
""","""        let node = resolve_kernel_future(workflow::workflow_node_for_app(&app, plugin_id, &node_id, &position, label));
""")
HU=FW+"🛍️products/💻️os/🖥️host/🧪️tests/🔬️host-unit/🌱️instances-parameters-and-catalog/🦀️.rs"
e(HU,'add_workflow_node("draw", "draw", None,','add_workflow_node("draw", "draw", "Draw",',2)
e(HU,'add_workflow_node("sink", "sink", None,','add_workflow_node("sink", "sink", "Sink",')

# ---- 6. TS shell
RC=OS+"📺️renderer/🧑‍🎨engine/🧱️elements/🛠️ShellHelpers/🛣️route-and-chrome/🟦️.tsx"
e(RC,"""export function resolveAppBreadcrumb(app: Pick<AppDefinition, "breadcrumb" | "terminologyBreadcrumbs">, terminology: string): readonly string[] {
  return app.terminologyBreadcrumbs?.[terminology] ?? app.breadcrumb ?? [];""","""export function resolveAppBreadcrumb(app: Pick<AppDefinition, "breadcrumb" | "terminologyBreadcrumbs">, terminology: string, locale: string): readonly string[] {
  const segments = app.terminologyBreadcrumbs?.[terminology];
  return segments === undefined ? app.breadcrumb ?? [] : segments.map((segment) => resolveManifestLabel(segment, terminology, locale));""")
e(RC,"breadcrumb: readonly string[], terminology: string): readonly string[] {\n  for","breadcrumb: readonly string[], terminology: string, locale: string): readonly string[] {\n  for")
e(RC,"if (app) return resolveAppBreadcrumb(app, terminology);","if (app) return resolveAppBreadcrumb(app, terminology, locale);")
e(RC,"""  const override = app.terminologyBreadcrumbs?.[terminology];
  return override?.[override.length - 1]?.trim() || resolveManifestLabel(app.label, terminology, locale).trim();""","""  const override = app.terminologyBreadcrumbs?.[terminology];
  const last = override?.[override.length - 1];
  return (last === undefined ? "" : resolveManifestLabel(last, terminology, locale).trim()) || resolveManifestLabel(app.label, terminology, locale).trim();""")
SHH=OS+"📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/"
for p,o,n in [
 ("🎨️theme-driver-settings/🛍️settings-and-marketplace-hosts/🟦️.tsx","resolveAppBreadcrumb(session.app, uiTerminology)","resolveAppBreadcrumb(session.app, uiTerminology, uiLocale)"),
 ("🎮️focus-and-input/🎯️focused-program-and-introduction/🟦️.tsx","activeSpawnedEntry.breadcrumb, uiTerminology) : session ? resolveAppBreadcrumb(session.app, uiTerminology)","activeSpawnedEntry.breadcrumb, uiTerminology, uiLocale) : session ? resolveAppBreadcrumb(session.app, uiTerminology, uiLocale)"),
 ("🖼️canvas-footer/🔸️canvas/🟦️.tsx","label: appBreadcrumb(resolveAppBreadcrumb(session.app, uiTerminology))","label: appBreadcrumb(resolveAppBreadcrumb(session.app, uiTerminology, uiLocale))"),
 ("🖼️canvas-footer/🔸️canvas/🟦️.tsx","focusedSpawned.breadcrumb, uiTerminology)","focusedSpawned.breadcrumb, uiTerminology, uiLocale)"),
 ("📋️panel-hosting/🔸️search-items/🟦️.tsx","program.breadcrumb, uiTerminology)","program.breadcrumb, uiTerminology, uiLocale)"),
 ("📋️panel-hosting/🔶️brand/🟦️.tsx","resolveAppBreadcrumb(session.app, uiTerminology)","resolveAppBreadcrumb(session.app, uiTerminology, uiLocale)")]:
    e(SHH+p,o,n)

import re
def _cells(t):
    t,n1=re.subn(r'("table": "workflow_port",\n\s*"rows": 3,\n\s*"bytes": )234\b',lambda m:m.group(1)+"249",t)
    t,n2=re.subn(r'(      \],\n      "bytes": )(\d+)',lambda m:m.group(1)+str(int(m.group(2))+15),t)
    assert (n1,n2)==(10,10),(n1,n2)
    return t
F=[(SQ+"🧪️tests/🧫️fixtures/📏️semantic-cells/🔣️.json",_cells)]  # label cells 18+4+0 bytes replace 7: +15 per case (UTF-8 TEXT bytes, INTEGER 8, unchanged)

if __name__=="__main__":
    W="--write" in sys.argv; bad=0; by={}; out={}
    for p,o,n,c in E:
        by.setdefault(p,[]).append((o,n,c))
    for p,items in by.items():
        try: s=open(p).read()
        except OSError: print("MISSING",p.replace(R,"")); bad+=1; continue
        for o,n,c in items:
            k=s.count(o)
            if k!=c: print(f"STALE x{k}!={c}",p.replace(R,"")[-70:],"|",o[:60].replace("\n"," ")); bad+=1
            else: s=s.replace(o,n)
        out[p]=s
    for p,f in F:
        try: out[p]=f(out.get(p) or open(p).read())
        except (AssertionError,OSError) as x: print("STALE fn",p.replace(R,"")[-60:],x); bad+=1
    if W and not bad:
        for p,s in out.items(): open(p,"w").write(s)
    print("edits",len(E)+len(F),"files",len(set(by)|{p for p,_ in F}),"stale",bad); sys.exit(1 if bad else 0)
