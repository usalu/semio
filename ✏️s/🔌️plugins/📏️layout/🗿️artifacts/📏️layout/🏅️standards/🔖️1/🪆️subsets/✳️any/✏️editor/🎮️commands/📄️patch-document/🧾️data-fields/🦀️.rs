//! 🧾️ Typed dictionary member edits use the native data-fields mutation.

use super::*;
use semio_framework_value::{DslValue, Number};
use semio_framework_value_dictionary::{QuestionDictionary, QuestionDictionaryEntry};

fn at<'a>(dictionary: &'a mut QuestionDictionary, path: &[usize]) -> Option<&'a mut DslValue> {
    let mut value = &mut dictionary.entries.get_mut(*path.first()?)?.value;
    for index in &path[1..] {
        value = match value { DslValue::Array(values) => values.get_mut(*index)?, DslValue::Object(values) => &mut values.get_mut(*index)?.1, _ => return None };
    }
    Some(value)
}

fn next_key(exists: impl Fn(&str) -> bool) -> String {
    let mut index = 1;
    loop { let key = format!("field-{index}"); if !exists(&key) { return key } index += 1; }
}

pub(super) fn mutation(document: &LayoutSnapshot, field: &str, value: &str) -> Option<LayoutMutation> {
    if field == "clearDataFields" { return Some(LayoutMutation::ChangeDataFields(ChangeDataFields { new_fields: None })) }
    let mut dictionary = document.data_fields.clone().unwrap_or_default();
    if field == "addDataField" {
        let question_id = next_key(|key| dictionary.entries.iter().any(|entry| entry.question_id == key));
        dictionary.entries.push(QuestionDictionaryEntry { question_id, value: DslValue::String(String::new()) });
    } else {
        let rest = field.strip_prefix("data.")?;
        let (indices, operation) = rest.rsplit_once('.')?;
        let path: Vec<usize> = indices.split('.').map(str::parse).collect::<Result<_, _>>().ok()?;
        let last = *path.last()?;
        match operation {
            "key" if path.len() == 1 => dictionary.entries.get_mut(last)?.question_id = value.into(),
            "key" => {
                let DslValue::Object(values) = at(&mut dictionary, &path[..path.len() - 1])? else { return None };
                if values.iter().enumerate().any(|(index, (key, _))| index != last && key == value) { return None }
                values.get_mut(last)?.0 = value.into();
            }
            "delete" if path.len() == 1 => { if last >= dictionary.entries.len() { return None } dictionary.entries.remove(last); }
            "delete" => match at(&mut dictionary, &path[..path.len() - 1])? {
                DslValue::Array(values) if last < values.len() => { values.remove(last); }
                DslValue::Object(values) if last < values.len() => { values.remove(last); }
                _ => return None,
            },
            "add" => match at(&mut dictionary, &path)? {
                DslValue::Array(values) => values.push(DslValue::String(String::new())),
                DslValue::Object(values) => { let key = next_key(|key| values.iter().any(|(name, _)| name == key)); values.push((key, DslValue::String(String::new()))); }
                _ => return None,
            },
            "kind" => *at(&mut dictionary, &path)? = match value {
                "text" => DslValue::String(String::new()),
                "number" => DslValue::Number(Number::Int(0)),
                "boolean" => DslValue::Bool(false),
                "null" => DslValue::Null,
                "list" => DslValue::Array(Vec::new()),
                "object" => DslValue::Object(Vec::new()),
                "bytes" => DslValue::Bytes(Vec::new()),
                _ => return None,
            },
            "value" => {
                let target = at(&mut dictionary, &path)?;
                *target = match target {
                    DslValue::String(_) => DslValue::String(value.into()),
                    DslValue::Bool(_) => DslValue::Bool(matches!(value, "true" | "1")),
                    DslValue::Number(_) => {
                        let number = semio_framework_pack_json::from_json_str::<DslValue>(value, semio_framework_pack_json::JsonMemberPolicy::Reject).ok()?;
                        if !matches!(number, DslValue::Number(_)) { return None }
                        number
                    }
                    DslValue::Bytes(_) => {
                        let digits: String = value.chars().filter(|ch| !ch.is_whitespace()).collect();
                        if digits.len() % 2 != 0 || !digits.is_ascii() { return None }
                        let bytes = (0..digits.len()).step_by(2).map(|index| u8::from_str_radix(&digits[index..index + 2], 16)).collect::<Result<Vec<_>, _>>().ok()?;
                        DslValue::Bytes(bytes)
                    }
                    _ => return None,
                };
            }
            _ => return None,
        }
    }
    dictionary.validate().ok()?;
    Some(LayoutMutation::ChangeDataFields(ChangeDataFields { new_fields: Some(dictionary) }))
}

/// 🔐️ Ordered dictionary fingerprints protect both chunk contents and indexed member identity.
pub(super) fn guarded(document: &LayoutSnapshot, field: &str, value: &str) -> Result<LayoutMutation, Fault> {
    let missing = || super::text_chunks::stale();
    if let Some((field, expected)) = field.rsplit_once(".at.") {
        if expected != super::text_chunks::dictionary_stamp(document) { return Err(missing()) }
        return mutation(document, field, value).ok_or_else(|| Fault::new(semio_framework_diagnostic::FaultOrigin::App, "layout.dictionary.invalid", "The dictionary field does not accept this value."));
    }
    let (address, kind, suffix) = if let Some((address, suffix)) = field.rsplit_once(".value.chunk.") { (address, "chunk", suffix) } else if let Some((address, suffix)) = field.rsplit_once(".value.bytes.") { (address, "bytes", suffix) } else { return Err(missing()) };
    let stamp = super::text_chunks::dictionary_stamp(document);
    let range_field = format!("{kind}.{suffix}");
    super::text_chunks::guarded_range(&range_field, kind, &stamp, usize::MAX)?;
    let path = address.strip_prefix("data.").ok_or_else(missing)?.split('.').map(str::parse).collect::<Result<Vec<usize>, _>>().map_err(|_| missing())?;
    let mut dictionary = document.data_fields.clone().ok_or_else(missing)?;
    let target = at(&mut dictionary, &path).ok_or_else(missing)?;
    match (kind, target) {
        ("chunk", DslValue::String(content)) => {
            if let Some((offset, delete, insert)) = super::text_chunks::splice(content, &range_field, value, &stamp)? {
                *content = content.chars().take(offset).chain(insert.chars()).chain(content.chars().skip(offset + delete)).collect();
            }
        }
        ("bytes", DslValue::Bytes(bytes)) => {
            let span = super::text_chunks::guarded_range(&range_field, kind, &stamp, bytes.len())?;
            let digits: String = value.chars().filter(|character| !character.is_whitespace()).collect();
            let invalid = || Fault::new(semio_framework_diagnostic::FaultOrigin::App, "layout.bytes.invalid", "A byte draft contains complete hexadecimal byte pairs.");
            if !digits.is_ascii() || digits.len() % 2 != 0 { return Err(invalid()) }
            let inserted = (0..digits.len()).step_by(2).map(|index| u8::from_str_radix(&digits[index..index + 2], 16)).collect::<Result<Vec<_>, _>>().map_err(|_| invalid())?;
            drop(bytes.splice(span, inserted));
        }
        _ => return Err(missing()),
    }
    Ok(LayoutMutation::ChangeDataFields(ChangeDataFields { new_fields: Some(dictionary) }))
}
