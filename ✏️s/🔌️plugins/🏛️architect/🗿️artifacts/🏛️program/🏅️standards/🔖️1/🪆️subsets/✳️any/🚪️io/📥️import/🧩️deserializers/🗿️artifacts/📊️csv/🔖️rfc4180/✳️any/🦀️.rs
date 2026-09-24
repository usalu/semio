//! 🏛️ program ← csv — a register table (`register,id,name,status,priority,tags,source`, any RFC 4180
//! producer) read by stdio's own csv codec into a fresh program through the editor's
//! `import_registers_csv` with `MergeStrategy::Replace`; a table with any other header is refused.
//!
//! 🔖 `IoFidelity::Lossy`: the inverse of the sibling export — register rows only.
use crate::editor::architect::behavior::{import_registers_csv, MergeStrategy};
use crate::schema::snapshot::ProgramSnapshot;

pub fn register() {}

pub fn deserialize_bytes(bytes: &[u8]) -> Result<ProgramSnapshot, store::TextError> {
    let error = |message: String| store::TextError::new(format!("program←csv: {message}"), dsl::TextSpan::at(1, 1));
    let text = std::str::from_utf8(bytes).map_err(|e| error(e.to_string()))?;
    let mut program = ProgramSnapshot::default();
    import_registers_csv(&mut program, text, MergeStrategy::Replace).map_err(|e| error(e.to_string()))?;
    Ok(program)
}
