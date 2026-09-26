//! 🧫️ N1 one-off fixture emitter for the fifteen norm families.
//!
//! Reads one JSON request per stdin line and answers one JSON line on stdout. Every answer comes from production code:
//! `from_snapshot` (the editor's own snapshot → mutation derivation), `Mutation::diff` + `MutationDiff::apply`
//! (production dispatch), `Mutation::inverse`, the committed DSL codec and the aggregates' `DESCRIPTORS`.
//!
//!   {"family":"en1990","op":"kinds"}
//!   {"family":"en1990","op":"dsl","text":"…"}
//!   {"family":"en1990","op":"normalize","snapshot":{…}}
//!   {"family":"en1990","op":"derive","base":{…},"target":{…}}
//!   {"family":"en1990","op":"apply","base":{…},"mutation":{…}}

extern crate semio_framework_os_kernel as protocol;

use pack::json::{self, Value};
use protocol::value::{FromValue, ToValue};
use protocol::{Mutation, MutationDiff};
use std::io::{BufRead, Write};

fn field<'a>(request: &'a Value, key: &str) -> Result<&'a Value, String> {
    request.get(key).ok_or_else(|| format!("request carries no {key:?}"))
}

fn decode<T: FromValue>(value: &Value, what: &str) -> Result<T, String> {
    T::from_value(json::to_dsl_value(value)).map_err(|error| format!("{what} does not decode: {error}"))
}

fn encode<T: ToValue>(value: &T) -> Value {
    json::from_dsl_value(&value.to_value())
}

fn object(pairs: Vec<(&str, Value)>) -> Value {
    json::object(pairs.into_iter().map(|(key, value)| (key.to_string(), value)))
}

/// 🎯️ One production application: the raised diagnostics, the diff, the applied snapshot and the restoring inverse.
fn apply_once<S, M>(base: &S, mutation: &M) -> Value
where
    S: ToValue + FromValue + PartialEq + Clone,
    M: Mutation<S>,
    M::Diff: ToValue,
{
    let raised = mutation.diff(base);
    let messages: Vec<Value> = raised.messages().iter().map(|message| Value::String(format!("{:?}:{}", message.level, message.code.0))).collect();
    let diff = encode(raised.diff());
    match raised.diff().apply(base) {
        Err(error) => object(vec![("applied", Value::Bool(false)), ("error", Value::String(format!("{error:?}"))), ("messages", json::array(messages)), ("diff", diff)]),
        Ok(after) => {
            let steps = mutation.inverse(base);
            let mut restored = after.clone();
            let mut inverse_error = Value::Null;
            for step in &steps {
                match step.diff(&restored).diff().apply(&restored) {
                    Ok(next) => restored = next,
                    Err(error) => {
                        inverse_error = Value::String(format!("{error:?}"));
                        break;
                    }
                }
            }
            object(vec![
                ("applied", Value::Bool(true)),
                ("kind", Value::String(mutation.descriptor().semantic_kind.to_string())),
                ("changed", Value::Bool(after != *base)),
                ("messages", json::array(messages)),
                ("diff", diff),
                ("after", encode(&after)),
                ("inverse", json::array(steps.iter().map(encode))),
                ("inverseError", inverse_error),
                ("restores", Value::Bool(restored == *base)),
            ])
        }
    }
}

/// 🧭️ Serves one request for one family's snapshot/mutation pair.
fn serve<S, M>(request: &Value, from_snapshot: fn(&S, &S) -> Vec<M>, from_dsl: fn(&str) -> Result<S, String>) -> Result<Value, String>
where
    S: ToValue + FromValue + PartialEq + Clone,
    M: Mutation<S>,
    M::Diff: ToValue,
{
    match field(request, "op")?.as_str().unwrap_or_default() {
        "kinds" => Ok(json::array(M::DESCRIPTORS.iter().map(encode))),
        "dsl" => Ok(encode(&from_dsl(field(request, "text")?.as_str().ok_or("text must be a string")?)?)),
        "normalize" => Ok(encode(&decode::<S>(field(request, "snapshot")?, "snapshot")?)),
        "derive" => {
            let base = decode::<S>(field(request, "base")?, "base")?;
            let target = decode::<S>(field(request, "target")?, "target")?;
            let mutations = from_snapshot(&base, &target);
            let kinds = json::array(mutations.iter().map(|mutation| Value::String(mutation.descriptor().semantic_kind.to_string())));
            Ok(object(vec![("mutations", json::array(mutations.iter().map(encode))), ("kinds", kinds), ("target", encode(&target))]))
        }
        "apply" => {
            let base = decode::<S>(field(request, "base")?, "base")?;
            let mutation = decode::<M>(field(request, "mutation")?, "mutation")?;
            Ok(apply_once(&base, &mutation))
        }
        other => Err(format!("unknown op {other:?}")),
    }
}

macro_rules! families {
    ($request:expr, $family:expr, { $($name:literal => $krate:ident, $snapshot:ident, $mutation:ident, $dsl:ident;)* }) => {
        match $family {
            $($name => serve::<$krate::standards::v1::subsets::any::schema::snapshot::$snapshot, $krate::standards::v1::subsets::any::schema::mutations::$mutation>(
                $request,
                $krate::standards::v1::subsets::any::schema::mutations::$mutation::from_snapshot,
                $krate::standards::v1::subsets::any::schema::snapshot::$dsl,
            ),)*
            other => Err(format!("unknown family {other:?}")),
        }
    };
}

fn answer(line: &str) -> Value {
    let request = match json::parse(line) {
        Ok(request) => request,
        Err(error) => return object(vec![("error", Value::String(format!("request is not JSON: {error}")))]),
    };
    let family = request.get("family").and_then(Value::as_str).unwrap_or_default().to_string();
    let result = families!(&request, family.as_str(), {
        "din16798" => semio_s_artifact_norm_din16798, Din16798Snapshot, Din16798Mutation, decode_din16798_dsl;
        "din18599" => semio_s_artifact_norm_din18599, Din18599Snapshot, Din18599Mutation, decode_din18599_dsl;
        "din4108" => semio_s_artifact_norm_din4108, Din4108Snapshot, Din4108Mutation, decode_din4108_dsl;
        "en1990" => semio_s_artifact_norm_en1990, En1990Snapshot, En1990Mutation, decode_en1990_dsl;
        "en1991" => semio_s_artifact_norm_en1991, En1991Snapshot, En1991Mutation, decode_en1991_dsl;
        "en1992" => semio_s_artifact_norm_en1992, En1992Snapshot, En1992Mutation, decode_en1992_dsl;
        "en1993" => semio_s_artifact_norm_en1993, En1993Snapshot, En1993Mutation, decode_en1993_dsl;
        "en1994" => semio_s_artifact_norm_en1994, En1994Snapshot, En1994Mutation, decode_en1994_dsl;
        "en1995" => semio_s_artifact_norm_en1995, En1995Snapshot, En1995Mutation, decode_en1995_dsl;
        "en1996" => semio_s_artifact_norm_en1996, En1996Snapshot, En1996Mutation, decode_en1996_dsl;
        "en1997" => semio_s_artifact_norm_en1997, En1997Snapshot, En1997Mutation, decode_en1997_dsl;
        "en1998" => semio_s_artifact_norm_en1998, En1998Snapshot, En1998Mutation, decode_en1998_dsl;
        "en1999" => semio_s_artifact_norm_en1999, En1999Snapshot, En1999Mutation, decode_en1999_dsl;
        "iso16757" => semio_s_artifact_norm_iso16757, Iso16757Snapshot, Iso16757Mutation, decode_iso16757_dsl;
        "vdi3805" => semio_s_artifact_norm_vdi3805, Vdi3805Snapshot, Vdi3805Mutation, decode_vdi3805_dsl;
    });
    match result {
        Ok(value) => object(vec![("ok", value)]),
        Err(error) => object(vec![("error", Value::String(error))]),
    }
}

fn main() {
    let stdin = std::io::stdin();
    let mut stdout = std::io::stdout().lock();
    for line in stdin.lock().lines() {
        let line = line.expect("stdin line");
        if line.trim().is_empty() {
            continue;
        }
        let reply = std::panic::catch_unwind(|| answer(&line)).unwrap_or_else(|_| object(vec![("error", Value::String("production code panicked".to_string()))]));
        writeln!(stdout, "{}", json::to_string(&reply)).expect("stdout");
        stdout.flush().expect("flush");
    }
}
