// Included by lib.rs (kit_diff_validation). Validates kit diffs; heal scrubs invalid entries.

use super::*;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct KitDiffValidationNote {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct KitDiffValidationResult {
    pub ok: bool,
    pub errors: Vec<KitDiffValidationNote>,
    pub warnings: Vec<KitDiffValidationNote>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub diff: Option<KitDiff>,
}

struct KitDiffValidateCtx {
    errors: Vec<KitDiffValidationNote>,
    warnings: Vec<KitDiffValidationNote>,
    heal: bool,
}

fn push_note(ctx: &mut KitDiffValidateCtx, kind: &str, code: &str, msg: String) {
    let n = KitDiffValidationNote {
        code: if code.is_empty() {
            None
        } else {
            Some(code.to_string())
        },
        message: msg,
    };
    if kind == "errors" {
        ctx.errors.push(n);
    } else {
        ctx.warnings.push(n);
    }
}

fn kitdiff_deep_equal(a: &Value, b: &Value) -> bool {
    a == b
}

fn to_map_slice(v: &Value) -> Vec<&Map<String, Value>> {
    let Some(arr) = v.as_array() else {
        return vec![];
    };
    arr.iter().filter_map(|x| x.as_object()).collect()
}

fn guid_set_from_entities(v: &Value) -> HashSet<String> {
    to_map_slice(v)
        .into_iter()
        .filter_map(|m| m.get("guid").and_then(|g| g.as_str()).map(String::from))
        .collect()
}

fn filter_updates_by_guid(updates: Vec<Value>, id_key: &str, gid: &str) -> Vec<Value> {
    updates
        .into_iter()
        .filter(|u| {
            let Some(m) = u.as_object() else {
                return true;
            };
            let Some(id_obj) = m.get(id_key).and_then(|x| x.as_object()) else {
                return true;
            };
            let Some(g) = id_obj.get("guid").and_then(|x| x.as_str()) else {
                return true;
            };
            g != gid
        })
        .collect()
}

fn validate_guid_collection_diff<F>(
    ctx: &mut KitDiffValidateCtx,
    path: &str,
    id_key: &str,
    base: &[Map<String, Value>],
    raw: &Value,
    mut on_updated: F,
) -> Option<Value>
where
    F: FnMut(&mut KitDiffValidateCtx, &Map<String, Value>, Option<&Map<String, Value>>, &str),
{
    let Some(raw_map) = raw.as_object() else {
        return None;
    };
    let mut base_by: HashMap<String, &Map<String, Value>> = HashMap::new();
    for it in base {
        if let Some(g) = it.get("guid").and_then(|x| x.as_str()) {
            base_by.insert(g.to_string(), it);
        }
    }
    let mut removed_set: HashSet<String> = HashSet::new();
    if let Some(arr) = raw_map.get("removed").and_then(|x| x.as_array()) {
        for r in arr {
            let Some(rm) = r.as_object() else { continue };
            if let Some(g) = rm.get("guid").and_then(|x| x.as_str()) {
                removed_set.insert(g.to_string());
            }
        }
    }
    let mut after_remove: HashSet<String> = HashSet::new();
    for g in base_by.keys() {
        if !removed_set.contains(g) {
            after_remove.insert(g.clone());
        }
    }
    let mut h_rem: Option<Vec<Value>> = None;
    let mut h_upd: Option<Vec<Value>> = None;
    let mut h_add: Option<Vec<Value>> = None;
    if ctx.heal {
        h_rem = raw_map
            .get("removed")
            .and_then(|x| x.as_array())
            .cloned()
            .map(|a| a.to_vec());
        h_upd = raw_map
            .get("updated")
            .and_then(|x| x.as_array())
            .cloned()
            .map(|a| a.to_vec());
        h_add = raw_map
            .get("added")
            .and_then(|x| x.as_array())
            .cloned()
            .map(|a| a.to_vec());
    }
    if let Some(arr) = raw_map.get("removed").and_then(|x| x.as_array()) {
        for r in arr {
            let Some(rm) = r.as_object() else { continue };
            let rg = rm
                .get("guid")
                .and_then(|x| x.as_str())
                .unwrap_or("")
                .to_string();
            if !base_by.contains_key(&rg) {
                push_note(
                    ctx,
                    "warnings",
                    "kitdiff.remove.missing-target",
                    format!("{path}: remove references missing {id_key} {rg}"),
                );
                if let Some(ref mut hr) = h_rem {
                    hr.retain(|x| {
                        x.as_object()
                            .and_then(|m| m.get("guid").and_then(|g| g.as_str()))
                            != Some(rg.as_str())
                    });
                }
            }
        }
    }
    let mut add_by: HashMap<String, Map<String, Value>> = HashMap::new();
    if let Some(arr) = raw_map.get("added").and_then(|x| x.as_array()) {
        for a in arr {
            let Some(am) = a.as_object() else { continue };
            if let Some(g) = am.get("guid").and_then(|x| x.as_str()) {
                add_by.insert(g.to_string(), am.clone());
            }
        }
    }
    if let Some(arr) = raw_map.get("removed").and_then(|x| x.as_array()) {
        for r in arr {
            let Some(rm) = r.as_object() else { continue };
            let rg = rm
                .get("guid")
                .and_then(|x| x.as_str())
                .unwrap_or("")
                .to_string();
            let orig = base_by.get(&rg).copied();
            let add = add_by.get(&rg);
            if let (Some(orig), Some(add)) = (orig, add) {
                let orig_v = Value::Object((*orig).clone());
                let add_v = Value::Object(add.clone());
                if kitdiff_deep_equal(&orig_v, &add_v) {
                    push_note(
                        ctx,
                        "warnings",
                        "kitdiff.cycle.noop-restore",
                        format!(
                            "{path}: removed and re-added {id_key} {rg} are deeply equal (no effective change)"
                        ),
                    );
                    if ctx.heal {
                        if let Some(ref mut hr) = h_rem {
                            hr.retain(|x| {
                                x.as_object()
                                    .and_then(|m| m.get("guid").and_then(|g| g.as_str()))
                                    != Some(rg.as_str())
                            });
                        }
                        if let Some(ref mut ha) = h_add {
                            ha.retain(|x| {
                                x.as_object()
                                    .and_then(|m| m.get("guid").and_then(|g| g.as_str()))
                                    != Some(rg.as_str())
                            });
                        }
                    }
                }
            }
        }
    }
    let mut seen_add: HashSet<String> = HashSet::new();
    if let Some(arr) = raw_map.get("added").and_then(|x| x.as_array()) {
        for a in arr {
            let Some(am) = a.as_object() else { continue };
            let ag = am
                .get("guid")
                .and_then(|x| x.as_str())
                .unwrap_or("")
                .to_string();
            if seen_add.contains(&ag) {
                push_note(
                    ctx,
                    "errors",
                    "kitdiff.add.duplicate-in-diff",
                    format!("{path}: duplicate added {id_key} guid {ag}"),
                );
                if let Some(ref mut ha) = h_add {
                    let mut first = true;
                    let mut na: Vec<Value> = vec![];
                    for x in ha.drain(..) {
                        let skip = x
                            .as_object()
                            .and_then(|m| m.get("guid").and_then(|g| g.as_str()))
                            == Some(ag.as_str());
                        if skip {
                            if first {
                                na.push(x);
                                first = false;
                            }
                            continue;
                        }
                        na.push(x);
                    }
                    *ha = na;
                }
            }
            seen_add.insert(ag.clone());
            if after_remove.contains(&ag) {
                push_note(
                    ctx,
                    "errors",
                    "kitdiff.add.duplicate-guid",
                    format!("{path}: cannot add {id_key} {ag} that still exists after removes"),
                );
                if let Some(ref mut ha) = h_add {
                    ha.retain(|x| {
                        x.as_object()
                            .and_then(|m| m.get("guid").and_then(|g| g.as_str()))
                            != Some(ag.as_str())
                    });
                }
            }
        }
    }
    if let Some(arr) = raw_map.get("updated").and_then(|x| x.as_array()) {
        for u in arr {
            let Some(um) = u.as_object() else { continue };
            let Some(id_obj) = um.get(id_key).and_then(|x| x.as_object()) else {
                continue;
            };
            let gid = id_obj
                .get("guid")
                .and_then(|x| x.as_str())
                .unwrap_or("")
                .to_string();
            let p = format!("{path}.{id_key}[{gid}]");
            if gid.is_empty() {
                push_note(
                    ctx,
                    "errors",
                    "kitdiff.update.bad-id",
                    format!("{p}: missing {id_key} id"),
                );
                if let Some(ref mut hu) = h_upd {
                    *hu = filter_updates_by_guid(std::mem::take(hu), id_key, &gid);
                }
                continue;
            }
            if !after_remove.contains(&gid) {
                push_note(
                    ctx,
                    "errors",
                    "kitdiff.update.missing-target",
                    format!("{p}: update targets {id_key} not present after removes"),
                );
                if let Some(ref mut hu) = h_upd {
                    *hu = filter_updates_by_guid(std::mem::take(hu), id_key, &gid);
                }
                continue;
            }
            let Some(item) = base_by.get(&gid).copied() else {
                push_note(
                    ctx,
                    "errors",
                    "kitdiff.update.missing-base",
                    format!("{p}: {id_key} not found in base kit"),
                );
                if let Some(ref mut hu) = h_upd {
                    *hu = filter_updates_by_guid(std::mem::take(hu), id_key, &gid);
                }
                continue;
            };
            let dm = um.get("diff").and_then(|d| d.as_object());
            on_updated(ctx, item, dm, &p);
        }
    }
    if !ctx.heal {
        return None;
    }
    let mut out = Map::new();
    if let Some(ref hr) = h_rem {
        if !hr.is_empty() {
            out.insert("removed".into(), Value::Array(hr.clone()));
        }
    }
    if let Some(ref hu) = h_upd {
        if !hu.is_empty() {
            out.insert("updated".into(), Value::Array(hu.clone()));
        }
    }
    if let Some(ref ha) = h_add {
        if !ha.is_empty() {
            out.insert("added".into(), Value::Array(ha.clone()));
        }
    }
    if out.is_empty() {
        None
    } else {
        Some(Value::Object(out))
    }
}

struct RefSets {
    type_guids: HashSet<String>,
    design_guids: HashSet<String>,
    author_guids: HashSet<String>,
}

fn validate_design_diff_nested(
    ctx: &mut KitDiffValidateCtx,
    kit_map: &Map<String, Value>,
    design: &Map<String, Value>,
    diff: Option<&Map<String, Value>>,
    path: &str,
    refs: &RefSets,
) {
    let Some(diff) = diff else {
        return;
    };
    if let Some(p) = diff.get("parent").and_then(|x| x.as_object()) {
        if let Some(pg) = p.get("guid").and_then(|x| x.as_str()) {
            if !pg.is_empty() && !refs.design_guids.contains(pg) {
                push_note(
                    ctx,
                    "errors",
                    "kitdiff.ref.design-parent-missing",
                    format!("{path}: parent design {pg} not in kit"),
                );
            }
            if let Some(dg) = design.get("guid").and_then(|x| x.as_str()) {
                if pg == dg {
                    push_note(
                        ctx,
                        "errors",
                        "kitdiff.ref.design-parent-self",
                        format!("{path}: design cannot be its own parent"),
                    );
                }
            }
        }
    }
    if let Some(da) = diff.get("authors") {
        if let Some(arr) = da.as_array() {
            for a in arr {
                let Some(am) = a.as_object() else { continue };
                if let Some(g) = am.get("guid").and_then(|x| x.as_str()) {
                    if !g.is_empty() && !refs.author_guids.contains(g) {
                        push_note(
                            ctx,
                            "errors",
                            "kitdiff.ref.author-missing",
                            format!("{path}: author {g} not in kit"),
                        );
                    }
                }
            }
        } else if let Some(dm) = da.as_object() {
            let auth_base: Vec<Map<String, Value>> = kit_map
                .get("authors")
                .map(|v| {
                    to_map_slice(v)
                        .into_iter()
                        .map(|m| (*m).clone())
                        .collect()
                })
                .unwrap_or_default();
            let nested = Value::Object(dm.clone());
            let _ = validate_guid_collection_diff(
                ctx,
                &format!("{path}.authors"),
                "author",
                &auth_base,
                &nested,
                |_, _, _, _| {},
            );
        }
    }
    if let Some(pd) = diff.get("pieces").and_then(|x| x.as_object()) {
        let pieces_base: Vec<Map<String, Value>> = design
            .get("pieces")
            .map(|v| {
                to_map_slice(v)
                    .into_iter()
                    .map(|m| (*m).clone())
                    .collect()
            })
            .unwrap_or_default();
        let pv = Value::Object(pd.clone());
        let _ = validate_guid_collection_diff(
            ctx,
            &format!("{path}.pieces"),
            "piece",
            &pieces_base,
            &pv,
            |_, _, _, _| {},
        );
        if let Some(arr) = pd.get("added").and_then(|x| x.as_array()) {
            for a in arr {
                let Some(am) = a.as_object() else { continue };
                let tg = am
                    .get("type")
                    .and_then(|x| x.as_object())
                    .and_then(|t| t.get("guid").and_then(|x| x.as_str()))
                    .unwrap_or("")
                    .to_string();
                if !tg.is_empty() && !refs.type_guids.contains(&tg) {
                    push_note(
                        ctx,
                        "errors",
                        "kitdiff.ref.piece-type-missing",
                        format!("{path}.pieces.added: type {tg} not in kit"),
                    );
                }
                let dg = am
                    .get("design")
                    .and_then(|x| x.as_object())
                    .and_then(|d| d.get("guid").and_then(|x| x.as_str()))
                    .unwrap_or("")
                    .to_string();
                if !dg.is_empty() && !refs.design_guids.contains(&dg) {
                    push_note(
                        ctx,
                        "errors",
                        "kitdiff.ref.piece-design-missing",
                        format!("{path}.pieces.added: subdesign {dg} not in kit"),
                    );
                }
            }
        }
    }
}

fn merge_top_level_guid_coll<F>(
    ctx: &mut KitDiffValidateCtx,
    kit_map: &Map<String, Value>,
    diff_map: &Map<String, Value>,
    out_diff: &mut Option<Map<String, Value>>,
    heal: bool,
    key: &str,
    id_key: &str,
    arr_key: &str,
    mut on_updated: F,
) where
    F: FnMut(&mut KitDiffValidateCtx, &Map<String, Value>, Option<&Map<String, Value>>, &str),
{
    let Some(part) = diff_map.get(key) else {
        return;
    };
    let base_slice: Vec<Map<String, Value>> = kit_map
        .get(arr_key)
        .map(|v| {
            to_map_slice(v)
                .into_iter()
                .map(|m| (*m).clone())
                .collect()
        })
        .unwrap_or_default();
    let fixed = validate_guid_collection_diff(ctx, key, id_key, &base_slice, part, |c, item, dm, p| {
        on_updated(c, item, dm, p);
    });
    if heal {
        if let Some(od) = out_diff {
            match fixed {
                Some(v) => {
                    od.insert(key.to_string(), v);
                }
                None => {
                    od.remove(key);
                }
            }
        }
    }
}

pub fn validate_kit_diff(kit: &Kit, diff: &KitDiff, heal: bool) -> KitDiffValidationResult {
    let mut ctx = KitDiffValidateCtx {
        errors: vec![],
        warnings: vec![],
        heal,
    };
    let km = match serde_json::to_value(kit) {
        Ok(v) => v,
        Err(_) => {
            return KitDiffValidationResult {
                ok: false,
                errors: vec![KitDiffValidationNote {
                    code: Some("kitdiff.internal".into()),
                    message: "failed to serialize kit".into(),
                }],
                warnings: vec![],
                diff: None,
            };
        }
    };
    let dm = match serde_json::to_value(diff) {
        Ok(v) => v,
        Err(_) => {
            return KitDiffValidationResult {
                ok: false,
                errors: vec![KitDiffValidationNote {
                    code: Some("kitdiff.internal".into()),
                    message: "failed to serialize diff".into(),
                }],
                warnings: vec![],
                diff: None,
            };
        }
    };
    let Some(kit_map) = km.as_object() else {
        return KitDiffValidationResult {
            ok: false,
            errors: vec![KitDiffValidationNote {
                code: Some("kitdiff.internal".into()),
                message: "kit json not an object".into(),
            }],
            warnings: vec![],
            diff: None,
        };
    };
    let Some(diff_map) = dm.as_object() else {
        return KitDiffValidationResult {
            ok: false,
            errors: vec![KitDiffValidationNote {
                code: Some("kitdiff.internal".into()),
                message: "diff json not an object".into(),
            }],
            warnings: vec![],
            diff: None,
        };
    };
    let mut out_diff: Option<Map<String, Value>> = if heal {
        Some(diff_map.clone())
    } else {
        None
    };
    let refs = RefSets {
        type_guids: guid_set_from_entities(kit_map.get("types").unwrap_or(&Value::Null)),
        design_guids: guid_set_from_entities(kit_map.get("designs").unwrap_or(&Value::Null)),
        author_guids: guid_set_from_entities(kit_map.get("authors").unwrap_or(&Value::Null)),
    };
    merge_top_level_guid_coll(
        &mut ctx,
        kit_map,
        diff_map,
        &mut out_diff,
        heal,
        "types",
        "type",
        "types",
        |_, _, _, _| {},
    );
    let refs_clone = RefSets {
        type_guids: refs.type_guids.clone(),
        design_guids: refs.design_guids.clone(),
        author_guids: refs.author_guids.clone(),
    };
    merge_top_level_guid_coll(
        &mut ctx,
        kit_map,
        diff_map,
        &mut out_diff,
        heal,
        "designs",
        "design",
        "designs",
        |c, item, dm, p| {
            validate_design_diff_nested(c, kit_map, item, dm, p, &refs_clone);
        },
    );
    merge_top_level_guid_coll(
        &mut ctx,
        kit_map,
        diff_map,
        &mut out_diff,
        heal,
        "tags",
        "tag",
        "tags",
        |_, _, _, _| {},
    );
    merge_top_level_guid_coll(
        &mut ctx,
        kit_map,
        diff_map,
        &mut out_diff,
        heal,
        "concepts",
        "concept",
        "concepts",
        |_, _, _, _| {},
    );
    merge_top_level_guid_coll(
        &mut ctx,
        kit_map,
        diff_map,
        &mut out_diff,
        heal,
        "ports",
        "port",
        "ports",
        |_, _, _, _| {},
    );
    merge_top_level_guid_coll(
        &mut ctx,
        kit_map,
        diff_map,
        &mut out_diff,
        heal,
        "qualities",
        "quality",
        "qualities",
        |_, _, _, _| {},
    );
    merge_top_level_guid_coll(
        &mut ctx,
        kit_map,
        diff_map,
        &mut out_diff,
        heal,
        "files",
        "file",
        "files",
        |_, _, _, _| {},
    );
    merge_top_level_guid_coll(
        &mut ctx,
        kit_map,
        diff_map,
        &mut out_diff,
        heal,
        "folders",
        "folder",
        "folders",
        |_, _, _, _| {},
    );
    merge_top_level_guid_coll(
        &mut ctx,
        kit_map,
        diff_map,
        &mut out_diff,
        heal,
        "authors",
        "author",
        "authors",
        |_, _, _, _| {},
    );
    if let Some(a) = diff_map.get("attributes") {
        let attr_base: Vec<Map<String, Value>> = kit_map
            .get("attributes")
            .map(|v| {
                to_map_slice(v)
                    .into_iter()
                    .map(|m| (*m).clone())
                    .collect()
            })
            .unwrap_or_default();
        let _ = validate_guid_collection_diff(
            &mut ctx,
            "kit.attributes",
            "attribute",
            &attr_base,
            a,
            |_, _, _, _| {},
        );
    }
    let ok = ctx.errors.is_empty();
    let diff_out = if heal {
        out_diff.and_then(|m| {
            if m.is_empty() {
                None
            } else {
                serde_json::from_value(Value::Object(m)).ok()
            }
        })
    } else {
        None
    };
    KitDiffValidationResult {
        ok,
        errors: ctx.errors,
        warnings: ctx.warnings,
        diff: diff_out,
    }
}
