//! 🧩️ Typed catalogue creation with deterministic automatic placement.

use crate::editor::generation3d::config::{Generation3dConfig, Generation3dConfigMutation};
use semio_s_artifact_procedural_3d_mutations::Generation3dMutation;
use semio_s_artifact_procedural_3d_model::with_host;
use semio_s_artifact_procedural_3d_mutations::GraphEditor;
use semio_s_artifact_procedural_3d_model::snapshot::Generation3dSnapshot;
use semio_framework_os_flow::FlowEvalSession;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, FaultCode, FaultOrigin};
use semio_framework_value_derive::{FromValue, ToValue};

pub const AUTOMATIC_GAP: f64 = 48.0;

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "add-widget")]
pub struct AddWidget {
    pub kind: String,
    pub neuron_kind: Option<String>,
    pub format: Option<String>,
    pub action: Option<String>,
    pub x: Option<f64>,
    pub y: Option<f64>,
    pub source_widget: Option<String>,
    pub source_port: Option<String>,
    pub target_port: Option<String>,
    pub source_revision: Option<String>,
}

fn creation_fault(message: impl Into<String>) -> Fault {
    Fault::new(FaultOrigin::App, FaultCode::new(semio_s_artifact_procedural_3d_model::GENERATION3D_WIDGET_ADD), message.into())
}

/// 🧯️ The source diagnostic keeps its exact reason while the UI uses a translated notice.
pub(crate) fn source_fault(reason:semio_framework_os_flow::FlowSourceReadFailure)->Fault{Fault::new(FaultOrigin::App,FaultCode::new("generation3d.analysis.source-unavailable"),format!("original analysis source refused: {reason:?}"))}
/// 📢️ Refreshing the selected source is an explicit action in either supported language.
pub(crate) fn source_fault_notices()->&'static[(&'static str,semio_framework_ui_locale::LocalizedLabel)]{
 static NOTICES:std::sync::LazyLock<[(&str,semio_framework_ui_locale::LocalizedLabel);1]>=std::sync::LazyLock::new(||[("generation3d.analysis.source-unavailable",semio_framework_ui_locale::LocalizedLabel::native("The selected output changed or is unavailable. Wait for evaluation and select it again.","Die ausgewählte Ausgabe wurde geändert oder ist nicht verfügbar. Die Auswertung abwarten und erneut auswählen."))]);
 &*NOTICES
}

impl AddWidget {
    /// 🛍️ One descriptor source for catalogue activation and canvas drag.
    pub(crate) fn from_catalogue(item: &semio_framework_os_flow::CatalogueItem) -> Self {
        Self { kind: item.kind.clone(), neuron_kind: item.neuron_kind.clone(), format: item.format.clone(), action: item.action.clone(), x: None, y: None, source_widget:None,source_port:None,target_port:None,source_revision:None }
    }

    pub(crate) fn source_request(&self)->Result<Option<semio_framework_os_flow::FlowSourceReadRequest<'_>>,Fault>{
        let fields=[self.source_widget.as_deref(),self.source_port.as_deref(),self.target_port.as_deref(),self.source_revision.as_deref()];
        if fields.iter().all(Option::is_none){return Ok(None)}
        if self.kind!="neuron"||fields.iter().any(|field|field.is_none_or(str::is_empty)){return Err(creation_fault("connected neuron creation requires the complete original source endpoint and revision"))}
        self.source_revision_bytes()?;
        Ok(Some(semio_framework_os_flow::FlowSourceReadRequest{source_widget:fields[0].unwrap(),source_port:fields[1].unwrap(),target:semio_framework_os_flow::FlowSourceReadTarget::Port{port:fields[2].unwrap(),kind:self.neuron_kind.as_deref().ok_or_else(||creation_fault("connected creation requires a registered neuron kind"))?}}))
    }

    pub(crate) fn source_revision_bytes(&self)->Result<[u8;32],Fault>{
        let source=self.source_revision.as_deref().ok_or_else(||creation_fault("connected creation requires its displayed source revision"))?;
        if source.len()!=64||!source.bytes().all(|byte|byte.is_ascii_digit()||(b'a'..=b'f').contains(&byte)){return Err(creation_fault("connected creation requires the exact canonical source revision"))}
        let mut revision=[0;32];for(index,pair)in source.as_bytes().chunks_exact(2).enumerate(){let digit=|byte:u8|if byte.is_ascii_digit(){byte-b'0'}else{byte-b'a'+10};revision[index]=digit(pair[0])*16+digit(pair[1]);}Ok(revision)
    }

    pub(crate) fn descriptor_fields(&self) -> Vec<(&'static str, &str)> {
        let mut fields = vec![("kind", self.kind.as_str())];
        for (name, value) in [("neuronKind", &self.neuron_kind), ("format", &self.format), ("action", &self.action)] {
            if let Some(value) = value { fields.push((name, value.as_str())); }
        }
        if self.kind == "outputExport" && self.format.is_none() { fields.push(("format", "gltf")); }
        fields
    }

    /// 🧬️ Validates the schema's discriminated fields before building the host descriptor.
    pub(crate) fn descriptor_json(&self) -> Result<String, Fault> {
        self.source_request()?;
        if !matches!(self.kind.as_str(), "neuron" | "inputSlider" | "inputNote" | "inputImage" | "outputPreview" | "outputExport" | "outputAction" | "variable") {
            return Err(creation_fault(format!("unknown widget kind: {}", self.kind)));
        }
        for (kind, name, value) in [("neuron", "neuronKind", &self.neuron_kind), ("outputExport", "format", &self.format), ("outputAction", "action", &self.action)] {
            if kind == "outputExport" && self.kind == kind && value.is_none() { continue; }
            if (self.kind == kind) != value.is_some() || value.as_ref().is_some_and(|value| value.is_empty()) {
                return Err(creation_fault(format!("{} requires only its own descriptor fields; invalid {name}", self.kind)));
            }
        }
        if self.kind == "outputExport" && self.format.as_deref().is_some_and(|format| !semio_s_artifact_procedural_3d_io::document_io::EXPORT_FORMATS.iter().any(|row| row.id == format)) { return Err(creation_fault("choose an artifact-owned 3D export format")); }
        if [self.x, self.y].into_iter().flatten().any(|number| !number.is_finite()) {
            return Err(creation_fault("widget coordinates must be finite"));
        }
        Ok(semio_framework_pack_json::to_json_string(&semio_framework_value::DslValue::object(self.descriptor_fields().into_iter().map(|(name, value)| (name.into(), semio_framework_value::DslValue::String(value.into()))))))
    }
}

/// 📍️ Sweeps along the unspecified axis using centered rendered rectangles and a visible gap.
fn automatic_position(x: Option<f64>, y: Option<f64>, size: [f64; 2], occupied: &[[f64; 4]]) -> [f64; 2] {
    let mut position = [x.unwrap_or(120.0), y.unwrap_or(120.0)];
    if x.is_some() && y.is_some() { return position; }
    let axis = usize::from(y.is_none());
    for _ in 0..=occupied.len() {
        let next = occupied.iter().filter(|rect| (position[0] - rect[0]).abs() < (size[0] + rect[2]) / 2.0 + AUTOMATIC_GAP && (position[1] - rect[1]).abs() < (size[1] + rect[3]) / 2.0 + AUTOMATIC_GAP)
            .map(|rect| rect[axis] + (size[axis] + rect[axis + 2]) / 2.0 + AUTOMATIC_GAP).fold(position[axis], f64::max);
        if next == position[axis] { break; }
        position[axis] = next;
    }
    position
}

/// 🕹️ Creates one event-sourced widget as ONE edit — the `create-widget` and the `move-widget` that places it; rejected
/// descriptors produce a fault and no mutation.
pub fn handle(payload: &AddWidget, doc: &ArtifactView<'_, Generation3dSnapshot>, _cfg: &ConfigView<'_, Generation3dConfig>, _session: &mut FlowEvalSession) -> Result<Emit<Generation3dMutation, Generation3dConfigMutation>, Fault> {
    create(payload,doc,None)
}

/// 🔗️ The original retained command lends its admitted read result into one create/connect edit.
pub(crate) fn create_connected(payload:&AddWidget,doc:&ArtifactView<'_,Generation3dSnapshot>,source:&semio_framework_os_flow::FlowCurrentOutput<'_>)->Result<Emit<Generation3dMutation,Generation3dConfigMutation>,Fault>{create(payload,doc,Some(source))}

fn create(payload:&AddWidget,doc:&ArtifactView<'_,Generation3dSnapshot>,source:Option<&semio_framework_os_flow::FlowCurrentOutput<'_>>)->Result<Emit<Generation3dMutation,Generation3dConfigMutation>,Fault>{
    let host_snapshot = &doc.snapshot.host_snapshot;
    let descriptor = payload.descriptor_json()?;
    let request=payload.source_request()?;
    if let Some(request)=request.as_ref(){let revision=payload.source_revision_bytes()?;let operation=doc.operation()?;if operation.canonical_base_revision!=revision||source.is_none_or(|source|!source.admits(request,revision,operation.operation_id,operation.generation)){return Err(creation_fault("connected creation requires the original current-source read admission"))}}
    if let Some(kind) = &payload.neuron_kind {
        if !semio_framework_os_flow::flow_neuron_kind_info_map().contains_key(kind) && !matches!(kind.as_str(), semio_framework_artifact_flow_flow::neural::INPUT_KIND | semio_framework_artifact_flow_flow::neural::OUTPUT_KIND) {
            return Err(creation_fault(format!("unknown neuron kind: {kind}")));
        }
    }
    with_host(host_snapshot, |host| {
        let mut editor = GraphEditor::new(host);
        let id = editor.add_widget_json(&descriptor, payload.x.unwrap_or(120.0), payload.y.unwrap_or(120.0)).map_err(creation_fault)?;
        let size = editor.size_of(&id).ok_or_else(|| creation_fault("created widget has no rendered node"))?;
        let occupied = editor.occupied(&id);
        let [x, y] = automatic_position(payload.x, payload.y, size, &occupied);
        editor.move_widget(&id, x, y).map_err(creation_fault)?;
        if let Some(request)=request.as_ref(){editor.connect_ports(request.source_widget,request.source_port,&id,request.target_port().ok_or_else(||creation_fault("connected creation requires a registered input port"))?).map_err(creation_fault)?;}
        Ok(Emit { artifact_mutations: editor.finish(), ..Default::default() })
    })
}


#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
