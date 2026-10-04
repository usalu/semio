//! 🌍 MCP projection of the same closed GIS client contract used by native and browser hosts.
use crate::inference_schema::*;
use semio_framework_value::{DslValue,ToValue,FromValue};
use semio_framework_os_mcp::inference::*;

/// 📦 Actual installed owner codecs for the outward MCP executable.
pub fn gis_map_mcp_protocol_v1()->RemoteInferenceProtocolV1 {
    RemoteInferenceProtocolV1 {owner:"gis",service_id:GIS_MAP_INFERENCE_SERVICE_ID,route:"inference/gis-map",declaration:crate::inference_client::declaration,encode,receipt,page,approval,undo,error}
}

fn json(value:&DslValue)->Result<serde_json::Value,InferenceRouteErrorV1> {serde_json::from_str(&semio_framework_pack_json::to_json_string(value)).map_err(|_|InferenceRouteErrorV1::Invalid)}
fn dsl(value:&serde_json::Value)->Result<DslValue,InferenceRouteErrorV1> {semio_framework_pack_json::from_json_str(&value.to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|_|InferenceRouteErrorV1::Invalid)}
fn projection(value:DslValue)->Result<DslValue,InferenceRouteErrorV1> {
    let mut value=json(&value)?;
    value.as_object_mut().ok_or(InferenceRouteErrorV1::Invalid)?.remove("schema");
    dsl(&value)
}

fn encode(action:&str,payload:&DslValue)->Result<DslValue,InferenceRouteErrorV1> {
    let payload=json(payload)?;
    match action {
        "submit"=> {
            let args:HubInferenceSubmitRequestV1=serde_json::from_value(payload.clone()).map_err(|_|InferenceRouteErrorV1::Invalid)?;
            args.validate()?;
            if args.service_id!=GIS_MAP_INFERENCE_SERVICE_ID || args.lifetime_ms>GIS_MAP_INFERENCE_JOB_MAX_LIFETIME_MS {return Err(InferenceRouteErrorV1::Invalid);}
            Ok(GisMapInferenceJobRequestV1 {schema:"semio.hub.inference-request/v1".into(),version:1,request_id:args.request_id,service_id:args.service_id,policy_version:1,lifetime_ms:args.lifetime_ms}.to_value())
        }
        "approve"=> {
            let args:HubInferenceApprovalRequestV1=serde_json::from_value(payload.clone()).map_err(|_|InferenceRouteErrorV1::Invalid)?;
            args.validate()?;
            Ok(GisMapInferenceApprovalRequestV1 {schema:"semio.hub.inference-approval/v1".into(),version:1,job_id:args.job_id,proposal_hash:args.proposal_hash}.to_value())
        }
        "events"|"cancel"=>dsl(&payload),
        "undo"=> {
            let mut value=payload.clone();
            let fields=value.as_object_mut().ok_or(InferenceRouteErrorV1::Invalid)?;
            fields.insert("schema".into(),serde_json::json!("semio.hub.gis-map-approval-undo/v1"));
            fields.insert("version".into(),serde_json::json!(1));
            let request=GisMapApprovalUndoRequestV1::from_value(dsl(&value)?).map_err(|_|InferenceRouteErrorV1::Invalid)?;
            if !request.validate() {return Err(InferenceRouteErrorV1::Invalid);}
            Ok(request.to_value())
        }
        _=>Err(InferenceRouteErrorV1::Invalid),
    }
}

fn receipt(value:DslValue)->Result<DslValue,InferenceRouteErrorV1> {
    let receipt=GisMapInferenceJobReceiptV1::from_value(value).map_err(|_|InferenceRouteErrorV1::Invalid)?;
    if !receipt.validate() {return Err(InferenceRouteErrorV1::Invalid);}
    projection(receipt.to_value())
}
fn page(value:DslValue,job:&str)->Result<DslValue,InferenceRouteErrorV1> {
    let page=GisMapInferenceEventPageV1::from_value(value).map_err(|_|InferenceRouteErrorV1::Invalid)?;
    if page.job_id!=job {return Err(InferenceRouteErrorV1::Conflict);}
    if page.preview.as_ref().is_some_and(|preview|preview.job_id!=job || Some(preview.proposal_hash.as_str())!=page.proposal_hash.as_deref()) {return Err(InferenceRouteErrorV1::Conflict);}
    if !page.validate(job) {return Err(InferenceRouteErrorV1::Invalid);}
    projection(page.to_value())
}
fn approval(value:DslValue,scope:&semio_framework_os_kernel::os_directory::DocumentScope,job:&str,hash:&str)->Result<DslValue,InferenceRouteErrorV1> {
    let receipt=GisMapInferenceApprovalReceiptV1::from_value(value).map_err(|_|InferenceRouteErrorV1::Invalid)?;
    if !receipt.validate(job,hash) || receipt.undo.expected_current.document_id!=scope.document_id {return Err(InferenceRouteErrorV1::Conflict);}
    projection(receipt.to_value())
}
fn undo(value:DslValue,request:&DslValue)->Result<DslValue,InferenceRouteErrorV1> {
    let receipt=GisMapApprovalUndoReceiptV1::from_value(value).map_err(|_|InferenceRouteErrorV1::Invalid)?;
    let request=GisMapApprovalUndoRequestV1::from_value(encode("undo",request)?).map_err(|_|InferenceRouteErrorV1::Invalid)?;
    if receipt.schema!="semio.hub.gis-map-approval-undo-receipt/v1" || !valid_hex(&receipt.original_job_id,32) || !valid_hex(&receipt.mutation_id,32) || !valid_hex(&receipt.command_hash,64) || receipt.target_id!=request.target_id || !receipt.applied || !receipt.frontier.validate() || receipt.frontier.document_id!=request.expected_current.document_id || receipt.frontier.head_edit_ordinal!=request.expected_current.head_edit_ordinal.saturating_add(1) {return Err(InferenceRouteErrorV1::Conflict);}
    Ok(receipt.to_value())
}
fn error(status:u16,body:&[u8])->InferenceRouteErrorV1 {
    serde_json::from_slice::<serde_json::Value>(body).ok().filter(|value|value.as_object().is_some_and(|fields|fields.len()==2) && value.get("schema").and_then(serde_json::Value::as_str)==Some("semio.hub.inference-error/v1")).and_then(|value|value.get("code").and_then(serde_json::Value::as_str).and_then(InferenceRouteErrorV1::from_code)).unwrap_or_else(||InferenceRouteErrorV1::from_status(status))
}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;

fn valid_hex(value:&str,length:usize)->bool {value.len()==length && value.bytes().all(|byte|byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))}
