//! 🧾️ Bounded dictionary payload grammar; emitted ranges are scalar facts, never read authority.
//! The input owner must adopt each event before feeding another byte and publish only after EOF.

#[derive(Debug, PartialEq, Eq)]
pub(super) enum DictionaryDeltaEvent {
    Begin { base: u64, count: u64 },
    Entry { offset: u64, length: u64 },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum DictionaryDeltaError {
    Malformed,
    Capacity,
    State,
    Cancelled,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Stage {
    Format,
    Base,
    Count,
    Length,
    Text,
    Done,
}

/// 🔡️ Constant-space UTF-8/LEB128 cursor with one payload byte per fuel and retained events.
pub(super) struct RetainedDictionaryDelta {
    position: u64,
    end: u64,
    stage: Stage,
    number: u64,
    digits: u8,
    base: u64,
    entries: u64,
    remaining: u64,
    entry_start: u64,
    entry_length: u64,
    utf8: [u8; 4],
    utf8_used: usize,
    utf8_expected: usize,
    event: Option<DictionaryDeltaEvent>,
    diagnostic: Option<DictionaryDeltaError>,
    closed: bool,
}

impl RetainedDictionaryDelta {
    pub(super) fn new(start: u64, end: u64) -> Result<Self, DictionaryDeltaError> {
        if start >= end {
            return Err(DictionaryDeltaError::Malformed);
        }
        Ok(Self { position: start, end, stage: Stage::Format, number: 0, digits: 0, base: 0, entries: 0, remaining: 0, entry_start: 0, entry_length: 0, utf8: [0; 4], utf8_used: 0, utf8_expected: 0, event: None, diagnostic: None, closed: false })
    }

    fn reject<T>(&mut self, diagnostic: DictionaryDeltaError) -> Result<T, DictionaryDeltaError> {
        Err(*self.diagnostic.get_or_insert(diagnostic))
    }
    fn check(&self) -> Result<(), DictionaryDeltaError> {
        if self.closed {
            return Err(DictionaryDeltaError::State);
        }
        self.diagnostic.map_or(Ok(()), Err)
    }
    pub(super) fn next_offset(&self) -> u64 {
        self.position
    }
    pub(super) fn has_event(&self) -> bool {
        self.event.is_some()
    }
    pub(super) fn take_event(&mut self) -> Result<Option<DictionaryDeltaEvent>, DictionaryDeltaError> {
        self.check()?;
        Ok(self.event.take())
    }
    pub(super) fn cancel(&mut self) {
        self.diagnostic.get_or_insert(DictionaryDeltaError::Cancelled);
    }

    pub(super) fn push(&mut self, byte: u8, fuel: &mut usize) -> Result<bool, DictionaryDeltaError> {
        self.check()?;
        if *fuel == 0 {
            return Ok(false);
        }
        if self.event.is_some() {
            return self.reject(DictionaryDeltaError::State);
        }
        if self.position >= self.end || self.stage == Stage::Done {
            return self.reject(DictionaryDeltaError::Malformed);
        }
        *fuel -= 1;
        self.position += 1;
        if let Err(diagnostic) = self.byte(byte) {
            return self.reject(diagnostic);
        }
        Ok(true)
    }

    fn byte(&mut self, byte: u8) -> Result<(), DictionaryDeltaError> {
        match self.stage {
            Stage::Format => {
                if byte != 1 {
                    return Err(DictionaryDeltaError::Malformed);
                }
                self.stage = Stage::Base;
            }
            Stage::Base | Stage::Count | Stage::Length => {
                if self.digits == 9 && byte > 1 {
                    return Err(DictionaryDeltaError::Malformed);
                }
                self.number |= u64::from(byte & 127) << (u32::from(self.digits) * 7);
                self.digits += 1;
                if byte < 128 {
                    if self.digits > 1 && byte == 0 {
                        return Err(DictionaryDeltaError::Malformed);
                    }
                    let value = self.number;
                    self.number = 0;
                    self.digits = 0;
                    match self.stage {
                        Stage::Base => {
                            self.base = value;
                            self.stage = Stage::Count;
                        }
                        Stage::Count => {
                            if value > 8192 {
                                return Err(DictionaryDeltaError::Capacity);
                            }
                            self.entries = value;
                            self.stage = if value == 0 { Stage::Done } else { Stage::Length };
                            self.event = Some(DictionaryDeltaEvent::Begin { base: self.base, count: value });
                        }
                        Stage::Length => {
                            if value > 1_048_576 {
                                return Err(DictionaryDeltaError::Capacity);
                            }
                            if value > self.end - self.position {
                                return Err(DictionaryDeltaError::Malformed);
                            }
                            self.entry_start = self.position;
                            self.entry_length = value;
                            self.remaining = value;
                            if value == 0 {
                                self.complete_entry();
                            } else {
                                self.stage = Stage::Text;
                            }
                        }
                        _ => unreachable!("number stage was matched"),
                    }
                }
            }
            Stage::Text => {
                self.text_byte(byte)?;
                self.remaining -= 1;
                if self.remaining == 0 {
                    if self.utf8_used != 0 {
                        return Err(DictionaryDeltaError::Malformed);
                    }
                    self.complete_entry();
                }
            }
            Stage::Done => return Err(DictionaryDeltaError::Malformed),
        }
        Ok(())
    }

    fn text_byte(&mut self, byte: u8) -> Result<(), DictionaryDeltaError> {
        if self.utf8_used == 0 {
            if byte < 128 {
                return Ok(());
            }
            self.utf8_expected = match byte {
                194..=223 => 2,
                224..=239 => 3,
                240..=244 => 4,
                _ => return Err(DictionaryDeltaError::Malformed),
            };
        } else if byte & 192 != 128 {
            return Err(DictionaryDeltaError::Malformed);
        }
        self.utf8[self.utf8_used] = byte;
        self.utf8_used += 1;
        if self.utf8_used == self.utf8_expected {
            std::str::from_utf8(&self.utf8[..self.utf8_used]).map_err(|_| DictionaryDeltaError::Malformed)?;
            self.utf8.fill(0);
            self.utf8_used = 0;
            self.utf8_expected = 0;
        }
        Ok(())
    }

    fn complete_entry(&mut self) {
        self.entries -= 1;
        self.event = Some(DictionaryDeltaEvent::Entry { offset: self.entry_start, length: self.entry_length });
        self.stage = if self.entries == 0 { Stage::Done } else { Stage::Length };
    }

    pub(super) fn finish(&mut self) -> Result<(), DictionaryDeltaError> {
        self.check()?;
        if self.event.is_some() {
            return self.reject(DictionaryDeltaError::State);
        }
        if self.position != self.end || self.stage != Stage::Done {
            return self.reject(DictionaryDeltaError::Malformed);
        }
        Ok(())
    }

    pub(super) fn close_bytes(&mut self, maximum: usize) -> usize {
        self.closed = true;
        self.event = None;
        let count = maximum.min(self.utf8_used);
        self.utf8[self.utf8_used - count..self.utf8_used].fill(0);
        self.utf8_used -= count;
        if self.utf8_used == 0 {
            self.utf8_expected = 0;
            self.number = 0;
        }
        count
    }
    pub(super) fn retained_scratch_bytes(&self) -> usize {
        self.utf8_used
    }
    pub(super) fn terminal_is_empty(&self) -> bool {
        self.closed && self.utf8_used == 0 && self.event.is_none()
    }
}

impl Drop for RetainedDictionaryDelta {
    fn drop(&mut self) {
        assert!(self.terminal_is_empty(), "dictionary payload scratch requires bounded retirement");
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
