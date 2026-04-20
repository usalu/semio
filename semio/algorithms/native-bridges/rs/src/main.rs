mod header { // 🧲Header
// 💻 semio/algorithms/native-bridges/rs/src/main.rs
// Specs: Read JSON op+payload from stdin; write JSON {ok,result,error} to stdout.
// Summary: Rust native bridge for algorithms Storybook proxy using semio/rs library only.
// 2026 Ueli Saluz <ueli@semio-tech.com>
} // 🧲Header

use semio::{
    Design, DesignChange, DesignDiff, DesignDto, Kit, KitDto, OperationNote, SemioReport, Type,
};
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;
use std::io::Read;
use std::sync::Arc;

#[derive(Serialize)]
struct DesignChangeOut<'a> {
    forward: &'a DesignDiff,
    backward: &'a DesignDiff,
    #[serde(skip_serializing_if = "Option::is_none")]
    author: &'a Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    time: &'a Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    before: Option<DesignDto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    after: Option<DesignDto>,
}

fn design_change_to_value(c: &DesignChange) -> Result<serde_json::Value, serde_json::Error> {
    serde_json::to_value(DesignChangeOut {
        forward: &c.forward,
        backward: &c.backward,
        author: &c.author,
        time: &c.time,
        before: c.before.as_ref().map(|d| d.to_dto()),
        after: c.after.as_ref().map(|d| d.to_dto()),
    })
}

#[derive(Serialize)]
struct FlattenReportJson<'a> {
    ok: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    diff: Option<serde_json::Value>,
    warnings: &'a [OperationNote],
    infos: &'a [OperationNote],
    errors: &'a [OperationNote],
}

fn semio_report_design_change_to_value(
    rep: &SemioReport<DesignChange>,
) -> Result<serde_json::Value, serde_json::Error> {
    let diff = match &rep.diff {
        Some(c) => Some(design_change_to_value(c)?),
        None => None,
    };
    serde_json::to_value(FlattenReportJson {
        ok: rep.ok,
        diff,
        warnings: &rep.warnings,
        infos: &rep.infos,
        errors: &rep.errors,
    })
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct BridgeRequest {
    op: String,
    kit: KitDto,
    #[serde(default)]
    design: Option<DesignDto>,
    design_guid: String,
    #[serde(default)]
    piece_guids: Vec<String>,
    #[serde(default)]
    connection_guids: Vec<String>,
}

#[derive(Debug, Serialize)]
struct BridgeResponse {
    ok: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    result: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<String>,
}

fn types_map_from_kit(kit: &Kit) -> HashMap<String, Arc<Type>> {
    kit.types
        .as_ref()
        .map(|v| v.iter().map(|t| (t.guid.clone(), t.clone())).collect())
        .unwrap_or_default()
}

fn main() {
    let mut buf = String::new();
    if let Err(e) = std::io::stdin().read_to_string(&mut buf) {
        write_err(format!("read stdin: {e}"));
        return;
    }
    let req: BridgeRequest = match serde_json::from_str(&buf) {
        Ok(v) => v,
        Err(e) => {
            write_err(format!("parse request: {e}"));
            return;
        }
    };
    let kit = match Kit::from_dto(req.kit) {
        Ok(k) => k,
        Err(e) => {
            write_err(format!("hydrate kit: {e}"));
            return;
        }
    };
    let design_from_wire = match req.design {
        Some(ddto) => {
            let tm = types_map_from_kit(&kit);
            match Design::from_dto(ddto, &tm) {
                Ok(d) => Some(d),
                Err(e) => {
                    write_err(format!("hydrate design: {e}"));
                    return;
                }
            }
        }
        None => None,
    };
    match req.op.as_str() {
        "flatten" => {
            let dc = match kit.design(&req.design_guid) {
                Some(d) => d.flatten(&kit),
                None => {
                    write_err(format!("design {} not found", req.design_guid));
                    return;
                }
            };
            match semio_report_design_change_to_value(&dc) {
                Ok(v) => write_ok(v),
                Err(e) => write_err(format!("marshal flatten: {e}")),
            }
        }
        "delete" => {
            let design = match design_from_wire {
                Some(d) => d,
                None => {
                    write_err("missing design".to_string());
                    return;
                }
            };
            let pieces: Vec<_> = req
                .piece_guids
                .iter()
                .filter_map(|g| {
                    design
                        .pieces
                        .as_ref()
                        .and_then(|ps| ps.iter().find(|p| p.guid == *g).cloned())
                })
                .collect();
            let connections: Vec<_> = req
                .connection_guids
                .iter()
                .filter_map(|g| design.connection_by_guid(g).cloned())
                .collect();
            let rep = design.delete_pieces_and_connections(&kit, &pieces, &connections);
            match serde_json::to_value(&rep) {
                Ok(v) => write_ok(v),
                Err(e) => write_err(format!("marshal delete: {e}")),
            }
        }
        _ => write_err(format!("unknown op: {}", req.op)),
    }
}

fn write_ok(result: serde_json::Value) {
    let resp = BridgeResponse {
        ok: true,
        result: Some(result),
        error: None,
    };
    println!("{}", serde_json::to_string(&resp).unwrap());
}

fn write_err(msg: String) {
    let resp = BridgeResponse {
        ok: false,
        result: None,
        error: Some(msg),
    };
    println!("{}", serde_json::to_string(&resp).unwrap());
}
