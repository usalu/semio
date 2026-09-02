//! 🔤️ Source-borrowed byte copy and equality frontiers; callers retain immutable owners.

use super::{Owner, Retirement};

//#region 📑️Copy
pub(super) fn edit_id_length(sequence: u64, metadata: bool) -> usize { b"flow-retained-".len() + if sequence == 0 { 1 } else { sequence.ilog10() as usize + 1 } + if metadata { 2 } else { 0 } }

pub(super) fn edit_id_byte(sequence: u64, index: usize, metadata: bool) -> u8 {
    const PREFIX: &[u8] = b"flow-retained-";
    let digits = if sequence == 0 { 1 } else { sequence.ilog10() as usize + 1 };
    if index < PREFIX.len() { return PREFIX[index]; }
    let index = index - PREFIX.len();
    if index < digits { return b'0' + ((sequence / 10u64.pow((digits - index - 1) as u32)) % 10) as u8; }
    if metadata { b"#0"[index - digits] } else { unreachable!("Flow edit id byte escaped extent") }
}

#[derive(Default)]
pub(super) struct TextCopy { bytes: Vec<u8>, reserved: bool, complete: bool }

impl TextCopy {
    pub(super) fn advance(&mut self, source: &str, maximum_bytes: usize) -> Result<Option<usize>, String> {
        if maximum_bytes == 0 { return Ok(None); }
        if !self.reserved {
            self.bytes.try_reserve_exact(source.len()).map_err(|_| "Flow text allocation admission failed")?;
            self.reserved = true;
            return Ok(Some(0));
        }
        if self.complete { return Ok(Some(0)); }
        let start = self.bytes.len();
        let count = maximum_bytes.min(source.len().checked_sub(start).ok_or("Flow text source changed during copy")?);
        self.bytes.extend_from_slice(&source.as_bytes()[start..start + count]);
        self.complete = self.bytes.len() == source.len();
        Ok(Some(count))
    }

    pub(super) fn complete(&self) -> bool { self.complete }

    pub(super) fn advance_ascii(&mut self, length: usize, byte_at: impl Fn(usize) -> u8, maximum_bytes: usize) -> Result<Option<usize>, String> {
        if maximum_bytes == 0 { return Ok(None); }
        if !self.reserved { self.bytes.try_reserve_exact(length).map_err(|_| "Flow generated text allocation failed")?; self.reserved = true; return Ok(Some(0)); }
        if self.complete { return Ok(Some(0)); }
        let count = maximum_bytes.min(length.checked_sub(self.bytes.len()).ok_or("Flow generated text length changed")?);
        for _ in 0..count { let byte = byte_at(self.bytes.len()); if !byte.is_ascii() { return Err("Flow generated identifier is not ASCII".into()); } self.bytes.push(byte); }
        self.complete = self.bytes.len() == length;
        Ok(Some(count))
    }

    pub(super) fn take(&mut self) -> Option<String> {
        if !self.complete { return None; }
        self.complete = false; self.reserved = false;
        Some(unsafe { String::from_utf8_unchecked(std::mem::take(&mut self.bytes)) })
    }

    pub(super) fn retire(mut self, retirement: &mut Retirement) { retirement.push(Owner::Bytes(std::mem::take(&mut self.bytes))); }
}
//#endregion 📑️Copy

//#region ⚖️Equality
#[derive(Default)]
pub(super) struct Equality { index: usize, left: Option<u8>, result: Option<bool> }

impl Equality {
    pub(super) fn advance(&mut self, left: &str, right: &str, maximum_bytes: usize) -> (Option<bool>, usize) {
        if maximum_bytes == 0 { return (None, 0); }
        if let Some(result) = self.result { return (Some(result), 0); }
        if left.len() != right.len() { self.result = Some(false); return (self.result, 0); }
        let mut bytes = 0;
        while bytes < maximum_bytes && self.index < left.len() {
            if let Some(value) = self.left.take() {
                bytes += 1;
                if value != right.as_bytes()[self.index] { self.result = Some(false); return (self.result, bytes); }
                self.index += 1;
            } else { self.left = Some(left.as_bytes()[self.index]); bytes += 1; }
        }
        if self.index == left.len() { self.result = Some(true); }
        (self.result, bytes)
    }
}
//#endregion ⚖️Equality

//#region 🧪️ByteLaws
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn semantic_text_copy_and_equality_obey_one_byte_and_production_grants() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧪️fixtures/🧫️grant-frontier/🔣️.json")).unwrap();
        for row in fixture["cases"].as_array().unwrap() {
            let source = row["unit"].as_str().unwrap().repeat(row["repetitions"].as_u64().unwrap() as usize);
            let grant = row["grantBytes"].as_u64().unwrap() as usize;
            let mut copy = TextCopy::default(); let mut copied = 0;
            while !copy.complete() { let bytes = copy.advance(&source, grant).unwrap().unwrap(); assert!(bytes <= grant); copied += bytes; }
            let target = copy.take().unwrap(); assert_eq!(target, source); assert_eq!(copied, source.len());
            let mut equality = Equality::default(); let mut compared = 0;
            loop { let (result, bytes) = equality.advance(&source, &target, grant); assert!(bytes <= grant); compared += bytes; if let Some(result) = result { assert!(result); break; } }
            assert_eq!(compared, source.len() * 2);
            assert_eq!(Equality::default().advance(&source, "", grant), (Some(false), 0));
        }
    }
}
//#endregion 🧪️ByteLaws
