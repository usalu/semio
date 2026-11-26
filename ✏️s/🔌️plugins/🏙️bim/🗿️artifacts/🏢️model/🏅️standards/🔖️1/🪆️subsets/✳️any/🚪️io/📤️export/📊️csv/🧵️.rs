//! 🧵️ Bounded borrowed schedule projection into RFC 4180 bytes.
use super::{RowKind, Schedule, ScheduleCell, ScheduleTable, trim_number};
use semio_s_artifact_bim_model_schema::standards::v1::subsets::any::schema::snapshot::values::ScheduleKey;
use std::fmt::{self, Write};

#[derive(Clone, Copy, Default)]
enum Position {
    #[default]
    Scan,
    Prefix,
    Field,
    EscapedQuote,
    End,
    LineFeed,
}

/// 🧭️ The scan and write frontiers of one borrowed schedule.
#[derive(Clone, Copy, Default)]
pub struct ScheduleCsvCursor {
    record: usize,
    column: usize,
    offset: usize,
    position: Position,
    quote: bool,
    total_label: bool,
    complete: bool,
}

struct NumberText {
    bytes: [u8; 320],
    length: usize,
}

impl NumberText {
    fn new() -> Self {
        Self { bytes: [0; 320], length: 0 }
    }
    fn text(&self) -> &str {
        trim_number(std::str::from_utf8(&self.bytes[..self.length]).expect("numeric format is ASCII"))
    }
}

impl Write for NumberText {
    fn write_str(&mut self, text: &str) -> fmt::Result {
        let end = self.length + text.len();
        self.bytes.get_mut(self.length..end).ok_or(fmt::Error)?.copy_from_slice(text.as_bytes());
        self.length = end;
        Ok(())
    }
}

impl ScheduleCsvCursor {
    pub fn is_complete(&self) -> bool {
        self.complete
    }

    /// 🖊️ Writes a bounded part of the table, returning bytes produced and work performed.
    pub fn write(&mut self, schedule: &Schedule, table: Option<&ScheduleTable>, bytes: &mut [u8], mut fuel: usize) -> (usize, usize) {
        let initial = fuel;
        let mut count = 0;
        let mut number = NumberText::new();
        let mut numeric_cell = None;
        while !self.complete && fuel > 0 && count < bytes.len() {
            fuel -= 1;
            let row = self.record.checked_sub(1).and_then(|row| table?.rows.get(row));
            if self.record > 0 && row.is_none() {
                self.complete = true;
                break;
            }
            let columns = row.map_or(schedule.columns.len(), |row| row.cells.len());
            if self.column == columns {
                match self.position {
                    Position::LineFeed => {
                        bytes[count] = b'\n';
                        self.record += 1;
                        self.column = 0;
                        self.total_label = false;
                        self.position = Position::Scan;
                    }
                    _ => {
                        bytes[count] = b'\r';
                        self.position = Position::LineFeed;
                    }
                }
                count += 1;
                continue;
            }

            let mut parts = match row {
                Some(row) => match &row.cells[self.column] {
                    ScheduleCell::Empty => [""; 3],
                    ScheduleCell::Text { value } => [value.as_str(), "", ""],
                    ScheduleCell::Number { value } => {
                        let cell = (self.record, self.column);
                        if numeric_cell != Some(cell) {
                            number.length = 0;
                            write!(number, "{value:.6}").expect("binary64 fits its numeric buffer");
                            numeric_cell = Some(cell);
                        }
                        [number.text(), "", ""]
                    }
                },
                None => {
                    let column = &schedule.columns[self.column];
                    match (column.heading.as_deref(), &column.key) {
                        (Some(heading), _) => [heading, "", ""],
                        (None, ScheduleKey::Field { field }) => [field.token(), "", ""],
                        (None, ScheduleKey::Property { set, name }) => [set, ".", name],
                    }
                }
            };
            let label = row.is_some_and(|row| row.kind == RowKind::Total) && !self.total_label && parts.iter().all(|part| part.is_empty());
            if label {
                parts[0] = "total";
            }
            let byte = part_byte(parts, self.offset);
            match self.position {
                Position::Scan => match byte {
                    Some(b',' | b'"' | b'\n' | b'\r') => {
                        self.quote = true;
                        self.offset = 0;
                        self.position = Position::Prefix;
                    }
                    Some(_) => self.offset += 1,
                    None => {
                        self.offset = 0;
                        self.position = Position::Prefix;
                    }
                },
                Position::Prefix => {
                    if self.quote {
                        bytes[count] = b'"';
                        count += 1;
                    }
                    self.position = Position::Field;
                }
                Position::Field => {
                    if let Some(byte) = byte {
                        bytes[count] = byte;
                        count += 1;
                        if byte == b'"' {
                            self.position = Position::EscapedQuote;
                        }
                        self.offset += 1;
                    } else {
                        if self.quote {
                            bytes[count] = b'"';
                            count += 1;
                        }
                        self.position = Position::End;
                    }
                }
                Position::EscapedQuote => {
                    bytes[count] = b'"';
                    count += 1;
                    self.position = Position::Field;
                }
                Position::End => {
                    self.column += 1;
                    self.offset = 0;
                    self.quote = false;
                    self.total_label |= label;
                    self.position = Position::Scan;
                    if self.column < columns {
                        bytes[count] = b',';
                        count += 1;
                    }
                }
                Position::LineFeed => unreachable!(),
            }
        }
        (count, initial - fuel)
    }
}

fn part_byte(parts: [&str; 3], mut offset: usize) -> Option<u8> {
    for part in parts {
        if offset < part.len() {
            return Some(part.as_bytes()[offset]);
        }
        offset -= part.len();
    }
    None
}
