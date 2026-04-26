//! JSON-RPC 2.0 over NDJSON. Method catalog mirrors the wasm `KitStoreHandle` API.

use std::path::Path;
use std::sync::mpsc::Sender;
use std::sync::OnceLock;

use async_broadcast::RecvError;
use futures_lite::future::block_on;
use serde::Serialize;
use serde_json::{json, Value};

use semio::change_command::ChangeKitCommand;
use semio::error::SemioError;
use semio::geom::Plane;
use semio::id::Id;
use semio::backbone::BackboneConfig;
use semio::kit::KitFullDto;
use semio::kit::KitGraph;
use semio::kit::KitGraphRef;
use semio::kit_conflict_registry::ConflictResolution;
use semio::kit_store::KitStore;
use semio::kit_store_command::KitStoreCommand;
use semio::kit_change::KitChangeKind;
use semio::read::ReadKitCommand;
use semio::kit_read_scope::{self, KitReadScope};

type DispatchRes = std::result::Result<Value, (i32, String)>;

pub fn line_out(out: &Sender<String>, line: String) {
    if out.send(line).is_err() {
        tracing::warn!("stdout writer thread closed");
    }
}

fn err_response(out: &Sender<String>, id: Option<Value>, code: i32, message: impl Into<String>) {
    let body = json!({ "jsonrpc": "2.0", "id": id, "error": { "code": code, "message": message.into() } });
    if let Ok(s) = serde_json::to_string(&body) {
        line_out(out, s);
    }
}

fn ok_response(out: &Sender<String>, id: Option<Value>, result: Value) {
    let body = json!({ "jsonrpc": "2.0", "id": id, "result": result });
    if let Ok(s) = serde_json::to_string(&body) {
        line_out(out, s);
    }
}

pub fn event_line(out: &Sender<String>, ev: &semio::events::KitEvent) {
    let p: Value = match serde_json::to_value(ev) {
        Ok(v) => v,
        Err(e) => {
            tracing::error!("event serialize: {e}");
            return;
        }
    };
    let body = json!({ "jsonrpc": "2.0", "method": "event", "params": p });
    if let Ok(s) = serde_json::to_string(&body) {
        line_out(out, s);
    }
}

fn settle_v(r: semio::error::SetResult) -> Value {
    match r {
        Ok(()) => json!({ "ok": true }),
        Err(e) => json!({ "ok": false, "error": e }),
    }
}

// ----------------------------------------------------------------------------- store + events

static EVENTS_BUILT: std::sync::Once = std::sync::Once::new();

fn install_k(
    store: &OnceLock<KitStore>,
    kit: KitGraphRef,
    out: &Sender<String>,
) -> std::result::Result<(), SemioError> {
    let ks = KitStore::from_graph(kit);
    store.set(ks).map_err(|_| {
        SemioError::InvalidOperation("kit already created (one kit per process)".to_string())
    })?;
    // When `SEMIO_STORE_NO_EVENTS=1` (or `true` / `yes`), do not start the event thread. Integration
    // tests and small stdout pipes can deadlock if the client does not drain `event` lines.
    let skip_events = matches!(
        std::env::var("SEMIO_STORE_NO_EVENTS").as_deref(),
        Ok("1" | "true" | "yes")
    );
    if !skip_events {
        let g = store
            .get()
            .expect("just set")
            .graph();
        EVENTS_BUILT.call_once(|| {
            start_event_thread(g, out.clone());
        });
    }
    Ok(())
}

fn start_event_thread(kit: KitGraphRef, out: Sender<String>) {
    std::thread::spawn(move || {
        let mut rx = match kit.read() {
            Ok(g) => g.subscribe(),
            Err(_) => return,
        };
        drop(kit);
        loop {
            match block_on(async { rx.recv().await }) {
                Ok(ev) => event_line(&out, &ev),
                Err(RecvError::Closed) => break,
                Err(RecvError::Overflowed(_)) => {}
            }
        }
    });
}

fn get_kit_store(store: &OnceLock<KitStore>) -> std::result::Result<&KitStore, SemioError> {
    store.get().ok_or_else(|| {
        SemioError::InvalidOperation("no kit: call kit.create or io.import* first".to_string())
    })
}

fn graph_for(store: &OnceLock<KitStore>) -> std::result::Result<KitGraphRef, SemioError> {
    Ok(get_kit_store(store)?.graph())
}

// ----------------------------------------------------------------------------- params

type E = (i32, String);

fn e32602(msg: impl Into<String>) -> E {
    (-32602, msg.into())
}

fn e32000(e: impl std::fmt::Display) -> E {
    (-32000, e.to_string())
}

fn p_obj<'a>(params: &'a Option<Value>) -> std::result::Result<&'a Value, E> {
    let v = params
        .as_ref()
        .ok_or_else(|| e32602("missing params object"))?;
    if v.is_object() {
        return Ok(v);
    }
    Err(e32602("params must be a JSON object"))
}

fn p_any(params: &Option<Value>) -> std::result::Result<&Value, E> {
    params
        .as_ref()
        .ok_or_else(|| e32602("missing params"))
}

fn take_str(obj: &Value, k: &str) -> std::result::Result<String, E> {
    let v = obj
        .get(k)
        .ok_or_else(|| e32602(format!("missing param '{k}'")))?;
    v.as_str()
        .map(String::from)
        .ok_or_else(|| e32602(format!("'{k}' is not a string")))
}

fn take_f64(obj: &Value, k: &str) -> std::result::Result<f64, E> {
    let v = obj
        .get(k)
        .ok_or_else(|| e32602(format!("missing param '{k}'")))?;
    v.as_f64()
        .ok_or_else(|| e32602(format!("'{k}' is not a number")))
}

fn take_i32(obj: &Value, k: &str) -> std::result::Result<i32, E> {
    let v = obj
        .get(k)
        .ok_or_else(|| e32602(format!("missing param '{k}'")))?;
    v.as_i64()
        .and_then(|i| i32::try_from(i).ok())
        .ok_or_else(|| e32602(format!("'{k}' is not an i32")))
}

// ----------------------------------------------------------------------------- public entry

pub fn handle_line(line: &str, store: &OnceLock<KitStore>, out: &Sender<String>) {
    let v: Value = match serde_json::from_str(line) {
        Ok(v) => v,
        Err(e) => {
            err_response(out, None, -32700, e.to_string());
            return;
        }
    };
    if !v
        .get("jsonrpc")
        .and_then(|x| x.as_str())
        .map(|s| s == "2.0")
        .unwrap_or(false)
    {
        err_response(
            out,
            v.get("id").cloned(),
            -32600,
            "not a valid jsonrpc 2.0 object",
        );
        return;
    }
    if v.get("error").is_some() {
        return;
    }
    if v.get("result").is_some() && v.get("method").is_none() {
        return;
    }
    let id = v.get("id").cloned();
    let method = v.get("method").and_then(|m| m.as_str()).map(str::to_string);
    let method = match method {
        Some(m) => m,
        None => {
            err_response(out, id, -32600, "missing 'method' field");
            return;
        }
    };
    let params: Option<Value> = v.get("params").cloned();
    match run_method(&method, &params, store, out) {
        Ok(r) => ok_response(out, id, r),
        Err((code, msg)) => err_response(out, id, code, msg),
    }
}

// ----------------------------------------------------------------------------- dispatch

fn run_method(
    method: &str,
    params: &Option<Value>,
    store: &OnceLock<KitStore>,
    out: &Sender<String>,
) -> DispatchRes {
    if method == "server.shutdown" {
        std::process::exit(0);
    }
    if method == "semio.generateId" {
        return Ok(json!(Id::new_v7().into_string()));
    }
    if method == "semio.round" {
        let o = p_obj(&params)?;
        let v = take_f64(o, "value")?;
        let d = take_i32(o, "decimals")?;
        let m = 10f64.powi(d);
        return Ok(serde_json::to_value((v * m).round() / m).unwrap());
    }
    if method == "semio.normalizeName" {
        let o = p_obj(&params)?;
        let s = take_str(o, "s")?;
        return Ok(json!(s
            .trim()
            .to_ascii_lowercase()
            .replace(|c: char| c.is_whitespace(), "-")));
    }
    if method == "kit.fromJson" {
        let o = p_obj(&params)?;
        let s = take_str(o, "json")?;
        let k = KitGraph::from_json_str(&s).map_err(e32000)?;
        let g = k.read().map_err(|_| e32000("lock poisoned".to_string()))?;
        return Ok(serde_json::to_value(&g.to_full_dto()).map_err(e32000)?);
    }
    if method == "kit.toJson" {
        let o = p_obj(&params)?;
        let dto: KitFullDto = serde_json::from_value(
            o.get("dto")
                .ok_or_else(|| e32602("missing dto"))?
                .clone(),
        )
        .map_err(e32000)?;
        let k = KitGraph::from_full_dto(dto);
        let json = k
            .read()
            .map_err(|_| e32000("lock poisoned"))?
            .to_json_pretty()
            .map_err(e32000)?;
        return Ok(json!(json));
    }
    if method == "kit.validate" {
        let o = p_obj(&params)?;
        let dto: KitFullDto = serde_json::from_value(
            o.get("dto")
                .ok_or_else(|| e32602("missing dto"))?
                .clone(),
        )
        .map_err(e32000)?;
        let k = KitGraph::from_full_dto(dto);
        let g = k.read().map_err(|_| e32000("lock poisoned"))?;
        return Ok(serde_json::to_value(&g.validate()).map_err(e32000)?);
    }
    if method == "kit.equals" {
        let o = p_obj(&params)?;
        let a: KitFullDto = serde_json::from_value(
            o.get("a")
                .ok_or_else(|| e32602("missing a"))?
                .clone(),
        )
        .map_err(e32000)?;
        let b: KitFullDto = serde_json::from_value(
            o.get("b")
                .ok_or_else(|| e32602("missing b"))?
                .clone(),
        )
        .map_err(e32000)?;
        let ka = KitGraph::from_full_dto(a);
        let kb = KitGraph::from_full_dto(b);
        let ga = ka.read().map_err(|_| e32000("lock a".to_string()))?;
        let gb = kb.read().map_err(|_| e32000("lock b".to_string()))?;
        return Ok(json!(ga.are_equal(&gb)));
    }
    if method == "design.flatten" {
        let o = p_obj(&params)?;
        let design_id = take_str(o, "designId")?;
        let kit_d: KitFullDto = serde_json::from_value(
            o.get("kit")
                .ok_or_else(|| e32602("missing kit"))?
                .clone(),
        )
        .map_err(e32000)?;
        let k = KitGraph::from_full_dto(kit_d);
        let g = k.read().map_err(|_| e32000("lock poisoned"))?;
        let rep = g
            .flatten_design(&design_id)
            .map_err(e32000)?;
        return Ok(serde_json::to_value(&rep).map_err(e32000)?);
    }

    if method == "kit.create" {
        let o = p_obj(&params)?;
        let dto: KitFullDto = serde_json::from_value(
            o.get("dto")
                .ok_or_else(|| e32602("missing dto"))?
                .clone(),
        )
        .map_err(e32000)?;
        let k = KitGraph::from_full_dto(dto);
        install_k(store, k, out).map_err(e32000)?;
        return Ok(json!(null));
    }
    if method == "io.importFromFile" {
        let o = p_obj(&params)?;
        let p = take_str(o, "path")?;
        let k = KitGraph::load_json_file(Path::new(&p)).map_err(e32000)?;
        install_k(store, k, out).map_err(e32000)?;
        return Ok(json!(null));
    }
    if method == "io.exportToFile" {
        let o = p_obj(&params)?;
        let p = take_str(o, "path")?;
        let k = graph_for(store).map_err(e32000)?;
        k.read()
            .map_err(|_| e32000("lock"))?
            .save_json_file(Path::new(&p))
            .map_err(e32000)?;
        return Ok(json!(null));
    }
    if method == "io.importFromFolder" {
        let o = p_obj(&params)?;
        let p = take_str(o, "path")?;
        let k = KitGraph::load_local_folder(Path::new(&p)).map_err(e32000)?;
        install_k(store, k, out).map_err(e32000)?;
        return Ok(json!(null));
    }
    if method == "io.exportToFolder" {
        let o = p_obj(&params)?;
        let p = take_str(o, "path")?;
        let k = graph_for(store).map_err(e32000)?;
        k.read()
            .map_err(|_| e32000("lock"))?
            .save_local_folder(Path::new(&p))
            .map_err(e32000)?;
        return Ok(json!(null));
    }
    if method == "io.importFromZip" {
        let o = p_obj(&params)?;
        let p = take_str(o, "path")?;
        let k = KitGraph::load_zip(Path::new(&p)).map_err(e32000)?;
        install_k(store, k, out).map_err(e32000)?;
        return Ok(json!(null));
    }
    if method == "io.exportToZip" {
        let o = p_obj(&params)?;
        let p = take_str(o, "path")?;
        let k = graph_for(store).map_err(e32000)?;
        k.read()
            .map_err(|_| e32000("lock"))?
            .save_zip(Path::new(&p))
            .map_err(e32000)?;
        return Ok(json!(null));
    }
    if method == "io.importFromRemote" {
        let o = p_obj(&params)?;
        let hub_url = take_str(o, "hubUrl")?;
        let session_id = take_str(o, "sessionId")?;
        let k = KitGraph::load_remote_session(&hub_url, &session_id).map_err(e32000)?;
        install_k(store, k, out).map_err(e32000)?;
        return Ok(json!(null));
    }

    if method == "backbone.attach" {
        let o = p_obj(&params)?;
        let config: BackboneConfig = serde_json::from_value(
            o.get("config")
                .ok_or_else(|| e32602("missing config"))?
                .clone(),
        )
        .map_err(e32000)?;
        let ks = get_kit_store(store).map_err(e32000)?;
        let r = ks
            .execute(KitStoreCommand::AttachBackbone { config })
            .map_err(e32000)?;
        return Ok(serde_json::to_value(&r).map_err(e32000)?);
    }
    if method == "backbone.detach" {
        let ks = get_kit_store(store).map_err(e32000)?;
        let r = ks.execute(KitStoreCommand::DetachBackbone).map_err(e32000)?;
        return Ok(serde_json::to_value(&r).map_err(e32000)?);
    }
    if method == "backbone.status" {
        let ks = get_kit_store(store).map_err(e32000)?;
        let r = ks.execute(KitStoreCommand::BackboneStatus).map_err(e32000)?;
        return Ok(serde_json::to_value(&r).map_err(e32000)?);
    }
    if method == "backbone.setActiveCheckpoint" {
        let o = p_obj(&params)?;
        let id = match o.get("id") {
            None | Some(Value::Null) => None,
            Some(v) => {
                let s = v
                    .as_str()
                    .ok_or_else(|| e32602("id must be string or null"))?;
                if s.is_empty() {
                    None
                } else {
                    Some(Id::from(s))
                }
            }
        };
        let ks = get_kit_store(store).map_err(e32000)?;
        let r = ks
            .execute(KitStoreCommand::SetActiveCheckpoint { id })
            .map_err(e32000)?;
        return Ok(serde_json::to_value(&r).map_err(e32000)?);
    }
    if method == "conflicts.list" {
        let ks = get_kit_store(store).map_err(e32000)?;
        let r = ks.execute(KitStoreCommand::ListConflicts).map_err(e32000)?;
        return Ok(serde_json::to_value(&r).map_err(e32000)?);
    }
    if method == "conflicts.resolve" {
        let o = p_obj(&params)?;
        let id_s = take_str(o, "id")?;
        let id = Id::from(id_s.as_str());
        let strategy: ConflictResolution = serde_json::from_value(
            o.get("strategy")
                .ok_or_else(|| e32602("missing strategy"))?
                .clone(),
        )
        .map_err(e32000)?;
        let ks = get_kit_store(store).map_err(e32000)?;
        let r = ks
            .execute(KitStoreCommand::ResolveConflict { id, strategy })
            .map_err(e32000)?;
        return Ok(serde_json::to_value(&r).map_err(e32000)?);
    }
    if method == "coordinator.syncNow" {
        let ks = get_kit_store(store).map_err(e32000)?;
        let r = ks.execute(KitStoreCommand::SyncNow).map_err(e32000)?;
        return Ok(serde_json::to_value(&r).map_err(e32000)?);
    }

    let k = graph_for(store).map_err(e32000)?;

    if method == "kit.snapshot" {
        let g = k.read().map_err(|_| e32000("lock"))?;
        return Ok(serde_json::to_value(&g.to_full_dto()).map_err(e32000)?);
    }
    if method == "kit.theKitDto" {
        let g = k.read().map_err(|_| e32000("lock"))?;
        return Ok(serde_json::to_value(&g.the_kit_dto()).map_err(e32000)?);
    }
    if method == "kit.execute" {
        let c = p_any(&params)?;
        let cmd: KitStoreCommand = if c.is_array() {
            let v: Vec<KitStoreCommand> = serde_json::from_value(c.clone()).map_err(e32000)?;
            KitStoreCommand::Batch { commands: v }
        } else {
            serde_json::from_value(c.clone()).map_err(e32000)?
        };
        let ks = get_kit_store(store).map_err(e32000)?;
        let r = ks.execute(cmd).map_err(e32000)?;
        return Ok(serde_json::to_value(&r).map_err(e32000)?);
    }
    if method == "kit.executeChangeKitCommands" {
        let o = p_obj(&params)?;
        let cmds: Vec<ChangeKitCommand> = serde_json::from_value(
            o.get("cmds")
                .ok_or_else(|| e32602("missing cmds"))?
                .clone(),
        )
        .map_err(e32000)?;
        let kind = ChangeKitCommand::batch_kind(&cmds);
        let mut inverse_out: Vec<ChangeKitCommand> = Vec::new();
        KitGraph::with_undo(&k, || {
            inverse_out = ChangeKitCommand::apply_many(&k, &cmds).map_err(KitGraph::map_semio_err)?;
            Ok(())
        })
        .map_err(|e| e32000(e.to_string()))?;
        #[derive(Serialize)]
        struct Out {
            kind: KitChangeKind,
            inverse: Vec<ChangeKitCommand>,
        }
        return Ok(serde_json::to_value(&Out { kind, inverse: inverse_out }).map_err(e32000)?);
    }
    if method == "kit.executeReadKitCommands" {
        let o = p_obj(&params)?;
        let scope: KitReadScope = serde_json::from_value(
            o.get("scope")
                .ok_or_else(|| e32602("missing scope"))?
                .clone(),
        )
        .map_err(e32000)?;
        let commands: Vec<ReadKitCommand> = serde_json::from_value(
            o.get("cmds")
                .ok_or_else(|| e32602("missing cmds"))?
                .clone(),
        )
        .map_err(e32000)?;
        let view = kit_read_scope::resolve_read_graph(k, &scope).map_err(e32000)?;
        let g = view.read().map_err(|_| e32000("lock"))?;
        let results = ReadKitCommand::execute_many(&*g, &commands).map_err(e32000)?;
        return Ok(serde_json::to_value(&results).map_err(e32000)?);
    }
    if method == "kit.materializeAt" {
        let p = p_any(&params)?;
        let at: Option<Id> = if p.is_null() {
            None
        } else if let Some(s) = p.as_str() {
            if s.is_empty() {
                None
            } else {
                Some(Id::from(s))
            }
        } else {
            let o = p
                .as_object()
                .ok_or_else(|| e32602("params must be null, string, or { at: string }"))?;
            match o.get("at") {
                None | Some(Value::Null) => None,
                Some(v) => {
                    let s = v
                        .as_str()
                        .ok_or_else(|| e32602("at must be a string or null"))?;
                    if s.is_empty() {
                        None
                    } else {
                        Some(Id::from(s))
                    }
                }
            }
        };
        let g = k.read().map_err(|_| e32000("lock"))?;
        let dto = g.materialize_at(at.as_ref());
        return Ok(serde_json::to_value(&dto).map_err(e32000)?);
    }
    if method == "kit.vcsState" {
        let g = k.read().map_err(|_| e32000("lock"))?;
        let mut checkpoint_ids: Vec<String> = g.checkpoints.keys().map(|i| i.to_string()).collect();
        checkpoint_ids.sort();
        let mut alt_ids: Vec<String> = g.alternatives.keys().map(|i| i.to_string()).collect();
        alt_ids.sort();
        let mut session_ids: Vec<String> = g.sessions.keys().map(|i| i.to_string()).collect();
        session_ids.sort();
        #[derive(Serialize)]
        struct VcsState {
            the_kit_head: Option<String>,
            checkpoint_ids: Vec<String>,
            alternative_ids: Vec<String>,
            session_ids: Vec<String>,
        }
        let out = VcsState {
            the_kit_head: g.the_kit_head.as_ref().map(|i| i.to_string()),
            checkpoint_ids,
            alternative_ids: alt_ids,
            session_ids,
        };
        return Ok(serde_json::to_value(&out).map_err(e32000)?);
    }
    if method == "kit.getField" {
        let o = p_obj(&params)?;
        let kind = take_str(o, "kind")?;
        let id = take_str(o, "id")?;
        let field = take_str(o, "field")?;
        let ek = KitGraph::parse_entity_kind(&kind).map_err(e32000)?;
        let v = KitGraph::get_field_rpc(&k, ek, &id, &field).map_err(e32000)?;
        return Ok(serde_json::to_value(&v).map_err(e32000)?);
    }
    if method == "kit.changeKitCommandsForFieldPatch" {
        let o = p_obj(&params)?;
        let kind = take_str(o, "kind")?;
        let id = take_str(o, "id")?;
        let field = take_str(o, "field")?;
        let val: Value = o
            .get("value")
            .ok_or_else(|| e32602("missing value"))?
            .clone();
        let ek = KitGraph::parse_entity_kind(&kind).map_err(e32000)?;
        let cmds = KitGraph::change_kit_commands_for_field_patch(&k, ek, &id, &field, val).map_err(|e| e32000(e.to_string()))?;
        return Ok(serde_json::to_value(&cmds).map_err(e32000)?);
    }
    if method == "kit.changeKitCommandsForAddChild" {
        let o = p_obj(&params)?;
        let parent_kind = take_str(o, "parentKind")?;
        let parent_id = take_str(o, "parentId")?;
        let child_kind = take_str(o, "childKind")?;
        let dto: Value = o
            .get("dto")
            .ok_or_else(|| e32602("missing dto"))?
            .clone();
        let pk = KitGraph::parse_entity_kind(&parent_kind).map_err(e32000)?;
        let ck = KitGraph::parse_entity_kind(&child_kind).map_err(e32000)?;
        let cmds = KitGraph::change_kit_commands_for_add_child(&k, pk, &parent_id, ck, dto).map_err(|e| e32000(e.to_string()))?;
        return Ok(serde_json::to_value(&cmds).map_err(e32000)?);
    }
    if method == "kit.changeKitCommandsForRemoveChild" {
        let o = p_obj(&params)?;
        let parent_kind = take_str(o, "parentKind")?;
        let parent_id = take_str(o, "parentId")?;
        let child_kind = take_str(o, "childKind")?;
        let child_id = take_str(o, "childId")?;
        let pk = KitGraph::parse_entity_kind(&parent_kind).map_err(e32000)?;
        let ck = KitGraph::parse_entity_kind(&child_kind).map_err(e32000)?;
        let cmds = KitGraph::change_kit_commands_for_remove_child(&k, pk, &parent_id, ck, &child_id).map_err(|e| e32000(e.to_string()))?;
        return Ok(serde_json::to_value(&cmds).map_err(e32000)?);
    }

    if method == "design.clusterPieces" {
        let o = p_obj(&params)?;
        let design_id = take_str(o, "designId")?;
        let ids: Vec<String> = serde_json::from_value(
            o.get("pieceIds")
                .or_else(|| o.get("piece_ids"))
                .ok_or_else(|| e32602("pieceIds"))?
                .clone(),
        )
        .map_err(e32000)?;
        let cluster_name = o
            .get("clusterName")
            .or_else(|| o.get("cluster_name"))
            .and_then(|v| v.as_str())
            .ok_or_else(|| e32602("clusterName"))?
            .to_string();
        let r = KitGraph::cluster_pieces(&k, &design_id, ids, cluster_name);
        return Ok(settle_v(r));
    }
    if method == "design.dragPieces" {
        let o = p_obj(&params)?;
        let design_id = take_str(o, "designId")?;
        let ids: Vec<String> = serde_json::from_value(
            o.get("pieceIds")
                .or_else(|| o.get("piece_ids"))
                .ok_or_else(|| e32602("pieceIds"))?
                .clone(),
        )
        .map_err(e32000)?;
        let du = take_f64(o, "du")?;
        let dv = take_f64(o, "dv")?;
        let r = KitGraph::drag_pieces(&k, &design_id, ids, du, dv);
        return Ok(settle_v(r));
    }
    if method == "design.movePieces" {
        let o = p_obj(&params)?;
        let design_id = take_str(o, "designId")?;
        let ids: Vec<String> = serde_json::from_value(
            o.get("pieceIds")
                .or_else(|| o.get("piece_ids"))
                .ok_or_else(|| e32602("pieceIds"))?
                .clone(),
        )
        .map_err(e32000)?;
        let gap = take_f64(o, "gap")?;
        let shift = take_f64(o, "shift")?;
        let rise = take_f64(o, "rise")?;
        let r = KitGraph::move_pieces(&k, &design_id, ids, gap, shift, rise);
        return Ok(settle_v(r));
    }
    if method == "design.fixPieces" {
        let o = p_obj(&params)?;
        let design_id = take_str(o, "designId")?;
        let ids: Vec<String> = serde_json::from_value(
            o.get("pieceIds")
                .or_else(|| o.get("piece_ids"))
                .ok_or_else(|| e32602("pieceIds"))?
                .clone(),
        )
        .map_err(e32000)?;
        let r = KitGraph::fix_pieces(&k, &design_id, ids);
        return Ok(settle_v(r));
    }
    if method == "design.flattenDesign" {
        let o = p_obj(&params)?;
        let design_id = take_str(o, "designId")?;
        let r = KitGraph::flatten_design_apply(&k, &design_id);
        return Ok(settle_v(r));
    }
    if method == "design.expandDesign" {
        let o = p_obj(&params)?;
        let p = take_str(o, "parentDesignId")?;
        let n = take_str(o, "nestedDesignId")?;
        let r = KitGraph::expand_nested_design(&k, &p, &n);
        return Ok(settle_v(r));
    }
    if method == "design.deleteConnection" {
        let o = p_obj(&params)?;
        let d = take_str(o, "designId")?;
        let c = take_str(o, "connectionId")?;
        let r = KitGraph::delete_connection_in_design(&k, &d, &c);
        return Ok(settle_v(r));
    }
    if method == "design.changePieceType" {
        let o = p_obj(&params)?;
        let d = take_str(o, "designId")?;
        let p = take_str(o, "pieceId")?;
        let t = take_str(o, "newTypeId")?;
        let r = KitGraph::change_piece_type(&k, &d, &p, &t);
        return Ok(settle_v(r));
    }
    if method == "design.pasteDesignSelection" {
        let o = p_obj(&params)?;
        let d = take_str(o, "designId")?;
        let sel: Value = o
            .get("selection")
            .ok_or_else(|| e32602("selection"))?
            .clone();
        let pl: Option<Plane> = if o
            .get("plane")
            .map_or(true, |p| p.is_null())
        {
            None
        } else {
            Some(serde_json::from_value(
                o.get("plane")
                    .ok_or_else(|| e32602("plane key"))?
                    .clone(),
            )
            .map_err(e32000)?)
        };
        let r = KitGraph::paste_design_selection(&k, &d, sel, pl);
        return Ok(settle_v(r));
    }
    if method == "design.createHangingPieces" {
        let o = p_obj(&params)?;
        let d = take_str(o, "designId")?;
        let tgs: Vec<String> = serde_json::from_value(
            o.get("typeIds")
                .or_else(|| o.get("type_ids"))
                .ok_or_else(|| e32602("typeIds"))?
                .clone(),
        )
        .map_err(e32000)?;
        let pl: Plane = serde_json::from_value(
            o.get("plane")
                .ok_or_else(|| e32602("plane"))?
                .clone(),
        )
        .map_err(e32000)?;
        let r = KitGraph::create_hanging_pieces(&k, &d, tgs, pl);
        return Ok(settle_v(r));
    }
    if method == "design.createConnectedPiece" {
        let o = p_obj(&params)?;
        let d = take_str(o, "designId")?;
        let pp = take_str(o, "parentPiece")?;
        let pport = take_str(o, "parentPort")?;
        let ct = take_str(o, "childType")?;
        let cport = take_str(o, "childPort")?;
        let r = KitGraph::create_connected_piece(&k, &d, &pp, &pport, &ct, &cport);
        return Ok(settle_v(r));
    }
    if method == "design.createFixedPiece" {
        let o = p_obj(&params)?;
        let d = take_str(o, "designId")?;
        let t = take_str(o, "typeId")?;
        let pl: Plane = serde_json::from_value(
            o.get("plane")
                .ok_or_else(|| e32602("plane"))?
                .clone(),
        )
        .map_err(e32000)?;
        let r = KitGraph::create_fixed_piece(&k, &d, &t, pl);
        return Ok(settle_v(r));
    }

    if method == "vcs.undo" {
        return Ok(settle_v(KitGraph::undo(&k)));
    }
    if method == "vcs.redo" {
        return Ok(settle_v(KitGraph::redo(&k)));
    }
    if method == "vcs.canUndo" {
        return Ok(json!(KitGraph::can_undo(&k)));
    }
    if method == "vcs.canRedo" {
        return Ok(json!(KitGraph::can_redo(&k)));
    }
    if method == "query.piecesMetadata" {
        let o = p_obj(&params)?;
        let d = take_str(o, "designId")?;
        let g = k.read().map_err(|_| e32000("lock"))?;
        let v = g
            .get_pieces_metadata_json(&d)
            .map_err(|e: semio::error::SetError| e32000(e.to_string()))?;
        return Ok(v);
    }
    if method == "query.pieces" {
        let o = p_obj(&params)?;
        let d = take_str(o, "designId")?;
        let g = k.read().map_err(|_| e32000("lock"))?;
        let v = g
            .get_pieces_json(&d)
            .map_err(|e: semio::error::SetError| e32000(e.to_string()))?;
        return Ok(v);
    }
    if method == "query.connections" {
        let o = p_obj(&params)?;
        let d = take_str(o, "designId")?;
        let g = k.read().map_err(|_| e32000("lock"))?;
        let v = g
            .get_connections_json(&d)
            .map_err(|e: semio::error::SetError| e32000(e.to_string()))?;
        return Ok(v);
    }
    if method == "query.designs" {
        let g = k.read().map_err(|_| e32000("lock"))?;
        let v = g
            .get_designs_json()
            .map_err(|e: semio::error::SetError| e32000(e.to_string()))?;
        return Ok(v);
    }
    if method == "query.types" {
        let g = k.read().map_err(|_| e32000("lock"))?;
        let v = g
            .get_types_json()
            .map_err(|e: semio::error::SetError| e32000(e.to_string()))?;
        return Ok(v);
    }
    if method == "query.authors" {
        let g = k.read().map_err(|_| e32000("lock"))?;
        let v = g
            .get_authors_json()
            .map_err(|e: semio::error::SetError| e32000(e.to_string()))?;
        return Ok(v);
    }
    if method == "query.kitMetadata" {
        let g = k.read().map_err(|_| e32000("lock"))?;
        let v = g
            .get_kit_json()
            .map_err(|e: semio::error::SetError| e32000(e.to_string()))?;
        return Ok(v);
    }
    if method == "events.subscribe" {
        return Ok(json!(null));
    }
    if method == "events.unsubscribe" {
        return Ok(json!(null));
    }

    Err((-32601, format!("method not found: {method}")))
}
