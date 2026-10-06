//! 📝️ Grid inference request and commit encodings.
use crate::host::inferences::Grid2dInferenceJob;

impl Grid2dInferenceJob {
    pub(crate) fn encode_chunk(&self) -> Option<Vec<u8>> {
        match self.encode_phase {
            0 => Some(br#"{"assignments":["#.to_vec()),
            1 => Some(match self.assignments.get(self.cursor) {
                Some((x, y, tile)) => {
                    let separator = if self.cursor == 0 { "" } else { "," };
                    format!("{separator}[{x},{y},{}]", semio_framework_pack_json::to_json_string(tile)).into_bytes()
                }
                None => format!(r#"],"contradiction":{},"entropy":["#, self.contradiction).into_bytes(),
            }),
            2 => Some(if self.cursor < self.cell_count() {
                let width = self.snapshot.width as usize;
                let x = (self.cursor % width) as u32;
                let y = (self.cursor / width) as u32;
                let separator = if self.cursor == 0 { "" } else { "," };
                format!("{separator}[{x},{y},{}]", semio_framework_pack_json::to_json_string(&self.cell_entropy(x, y))).into_bytes()
            } else { b"]}".to_vec() }),
            _ => None,
        }
    }
}

pub(crate) fn decode_inference_value<T:semio_framework_value::FromValue>(text:&str)->Result<T,String> {
    semio_framework_pack_json::from_json_str(text,semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error|error.to_string())
}

pub(crate) fn encode_inference_value<T:semio_framework_value::ToValue>(value:&T)->String {
    semio_framework_pack_json::to_json_string(value)
}
