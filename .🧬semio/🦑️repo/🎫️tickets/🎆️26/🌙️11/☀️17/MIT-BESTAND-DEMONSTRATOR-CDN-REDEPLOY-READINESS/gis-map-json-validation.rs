/// 🗺️ Checks whether a ranked original Map projection can retain its exact intrinsic tags in JSON.
#[derive(Clone, Copy)]
pub struct MapJsonValidation {
    path: [usize; 64],
    depth: usize,
    member: usize,
    key_offset: usize,
    complete: bool,
}

impl Default for MapJsonValidation {
    fn default() -> Self {
        Self { path: [0; 64], depth: 0, member: 0, key_offset: 0, complete: false }
    }
}

semio_framework_value::artifact_retire_leaf!(MapJsonValidation);

impl MapJsonValidation {
    pub fn step(
        &mut self,
        source: &impl semio_framework_pack_json::JsonWriteSource,
    ) -> Result<bool, semio_framework_value::ValueError> {
        use semio_framework_pack_json::JsonWriteNode;
        use semio_framework_value::Number;

        if self.complete {
            return Ok(true);
        }
        let path = &self.path[..self.depth];
        let value = source.node_at_path(path)?;
        let length = match value {
            JsonWriteNode::Object(length) | JsonWriteNode::Array(length) => Some(length),
            JsonWriteNode::Number(Number::Float(value)) if !value.is_finite() => return Err(inexact()),
            JsonWriteNode::Number(Number::Int(value)) if value >= 0 => return Err(inexact()),
            _ => None,
        };
        let child = self.path[self.depth];
        if length.is_some_and(|length| child < length) {
            if matches!(value, JsonWriteNode::Object(_)) && self.member < child {
                let previous = source.object_key_at_path(path, self.member)?.as_bytes();
                let current = source.object_key_at_path(path, child)?.as_bytes();
                let end = self.key_offset.saturating_add(64).min(current.len());
                if previous.len() != current.len()
                    || previous[self.key_offset..end] != current[self.key_offset..end]
                {
                    self.member += 1;
                    self.key_offset = 0;
                } else if end == current.len() {
                    return Err(inexact());
                } else {
                    self.key_offset = end;
                }
                return Ok(false);
            }
            if self.depth + 1 == self.path.len() {
                return Err(inexact());
            }
            self.depth += 1;
            self.path[self.depth] = 0;
        } else if self.depth == 0 {
            self.complete = true;
        } else {
            self.depth -= 1;
            self.path[self.depth] += 1;
        }
        self.member = 0;
        self.key_offset = 0;
        Ok(self.complete)
    }
}

fn inexact() -> semio_framework_value::ValueError {
    use semio_framework_value::{ValueError, ValueRefusalKind};
    ValueError::literal(ValueRefusalKind::InvalidValue, "map data cannot be represented exactly as JSON")
}
