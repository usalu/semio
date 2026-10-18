//! 🧳️ Mounted pack session: the retained typed pack owner implementation of the snapshot owner, its drop and the mounted pack session constructor. See [`super`].

use super::*;

//#region 🔖️MountedCanonicalPackSession
impl mounted::RetainedTypedPackOwner for Generation3dMountedTypedSnapshotOwner {
    type Value = Generation3dSnapshot;

    fn grant_symbol(&mut self, catalog: &mounted::RetainedPackCatalogCursor) -> Result<bool, &'static str> {
        let Some(owner) = self.string.as_mut() else { return Ok(false) };
        let Some((symbol, index, chars)) = owner.symbol else { return Ok(false) };
        owner.value.push(catalog.symbol_char(symbol, index).map_err(|_| "generation3d-mounted.symref-char")?.ok_or("generation3d-mounted.symref-short")?);
        if index + 1 == chars {
            self.finish_string()?;
        } else {
            self.string.as_mut().expect("P3 mounted symbol retained").symbol = Some((symbol, index + 1, chars));
        }
        Ok(true)
    }

    fn accept(&mut self, token: mounted::RetainedValueToken, catalog: &mounted::RetainedPackCatalogCursor) -> Result<(), &'static str> {
        use mounted::{RetainedValueContainer as Container, RetainedValueRole as Role, RetainedValueToken as Token};
        match token {
            Token::Begin { kind: Container::Record, count } => self.begin_record(count)?,
            Token::Begin { kind, count } => {
                if self.dsl_destination.is_some() {
                    match kind {
                        Container::List => {
                            let mut values = Vec::new();
                            values.try_reserve_exact(usize::try_from(count).map_err(|_| "generation3d-mounted.dsl-count")?).map_err(|_| "generation3d-mounted.dsl-preflight")?;
                            self.dsl_stack.push(Generation3dMountedDslFrame::Array(values));
                        }
                        Container::Map => {
                            let mut values = Vec::new();
                            values.try_reserve_exact(usize::try_from(count).map_err(|_| "generation3d-mounted.dsl-count")?).map_err(|_| "generation3d-mounted.dsl-preflight")?;
                            self.dsl_stack.push(Generation3dMountedDslFrame::Object { values, key: None });
                        }
                        _ => return Err("generation3d-mounted.dsl-container"),
                    }
                    return Ok(());
                }
                if self.json_destination.is_some() {
                    match kind {
                        Container::List => {
                            let mut values = Vec::new();
                            values.try_reserve_exact(usize::try_from(count).map_err(|_| "generation3d-mounted.json-count")?).map_err(|_| "generation3d-mounted.json-preflight")?;
                            self.json_stack.push(Generation3dMountedJsonFrame::Array(values));
                        }
                        Container::Map => self.json_stack.push(Generation3dMountedJsonFrame::Object { values: Vec::new(), key: None }),
                        _ => return Err("generation3d-mounted.json-container"),
                    }
                    return Ok(());
                }
                if kind == Container::Table && self.pending_table_rows.take() != Some(count) {
                    return Err("generation3d-mounted.table-row-count");
                }
                self.begin_container(kind, count)?;
            }
            Token::Unsigned { role: Role::FieldId, value } if value < 16 => match self.stack.last_mut() {
                Some(Generation3dMountedContainerOwner::Record { field, seen, .. }) if field.is_none() => {
                    *field = Some(value as u16);
                    *seen |= 1 << value;
                }
                Some(Generation3dMountedContainerOwner::DictionaryEntry { field, .. }) if field.is_none() => *field = Some(value as u16),
                _ => return Err("generation3d-mounted.field-owner"),
            },
            Token::Unsigned { role: Role::TableRows, value } => self.pending_table_rows = Some(value),
            Token::Unsigned { role: Role::TableField, value } => if let Some(
                    Generation3dMountedContainerOwner::Synapses { field, present, next, .. } | Generation3dMountedContainerOwner::Generations { field, present, next, .. } | Generation3dMountedContainerOwner::Dictionary { field, present, next, .. },
                ) = self.stack.last_mut() {
                *field = Some(u16::try_from(value).map_err(|_| "generation3d-mounted.table-field")?);
                present.fill(false);
                *next = 0;
            },
            Token::Tag { value: 0x06 | 0x07, .. } => self.begin_string()?,
            Token::Unsigned { role: Role::StringLength, value } => {
                let owner = self.string.as_mut().ok_or("generation3d-mounted.string-length-owner")?;
                owner.value.try_reserve_exact(value as usize).map_err(|_| "generation3d-mounted.string-preflight")?;
                owner.remaining = Some(value);
                if value == 0 {
                    self.finish_string()?;
                }
            }
            Token::StringChar(value) => {
                let owner = self.string.as_mut().ok_or("generation3d-mounted.string-char-owner")?;
                owner.value.push(value);
                let remaining = owner.remaining.as_mut().ok_or("generation3d-mounted.string-char-length")?;
                *remaining = remaining.checked_sub(value.len_utf8() as u64).ok_or("generation3d-mounted.string-width")?;
                if *remaining == 0 {
                    self.finish_string()?;
                }
            }
            Token::Unsigned { role: Role::Symbol, value } => self.begin_symbol(value, catalog)?,
            Token::Tag { value: 0x11, .. } if self.json_destination.is_none() => {
                self.begin_dsl();
            }
            Token::F64(value) if self.dsl_destination.is_some() => self.assign_dsl(semio_framework_value::DslValue::float(f64::from_bits(value)))?,
            Token::F64(value) if self.json_destination.is_some() => self.assign_json(semio_framework_value::DslValue::float(f64::from_bits(value)))?,
            Token::F64(value) => self.assign_f64(f64::from_bits(value))?,
            Token::Signed(value) if self.dsl_destination.is_some() => self.assign_dsl(semio_framework_value::DslValue::int(value))?,
            Token::Signed(value) if self.json_destination.is_some() => self.assign_json(semio_framework_value::DslValue::int(value))?,
            Token::Signed(value) => match self.stack.last_mut() {
                Some(Generation3dMountedContainerOwner::Record { owner: Generation3dMountedRecordOwner::NeuralValue { value: target, .. }, field, .. }) if *field == Some(2) && target.is_none() => {
                    *target = Some(semio_framework_artifact_flow_flow::neural::Value::Atom(semio_framework_artifact_flow_flow::neural::Atom::Integer(value)));
                    *field = None;
                }
                _ => return Err("generation3d-mounted.integer-owner"),
            },
            Token::Unsigned { role: Role::Integer | Role::Unsigned | Role::Enum, value } if self.json_destination.is_some() => self.assign_json(semio_framework_value::DslValue::uint(value))?,
            Token::Unsigned { role: Role::Integer | Role::Unsigned | Role::Enum, value } if self.dsl_destination.is_some() => self.assign_dsl(semio_framework_value::DslValue::uint(value))?,
            Token::Tag { value: 0x01, .. } if self.dsl_destination.is_some() => self.assign_dsl(semio_framework_value::DslValue::Bool(false))?,
            Token::Tag { value: 0x02, .. } if self.dsl_destination.is_some() => self.assign_dsl(semio_framework_value::DslValue::Bool(true))?,
            Token::Tag { value: 0x12, .. } if self.dsl_destination.is_some() => self.assign_dsl(semio_framework_value::DslValue::Null)?,
            Token::Tag { value: 0x01, .. } if self.json_destination.is_some() => self.assign_json(semio_framework_value::DslValue::Bool(false))?,
            Token::Tag { value: 0x02, .. } if self.json_destination.is_some() => self.assign_json(semio_framework_value::DslValue::Bool(true))?,
            Token::Tag { value: 0x12, .. } if self.json_destination.is_some() => self.assign_json(semio_framework_value::DslValue::Null)?,
            Token::Tag { value: 0x11, .. } if self.json_destination.is_some() => {}
            Token::Tag { value: 0x01, .. } => self.assign_bool(false),
            Token::Tag { value: 0x02, .. } => self.assign_bool(true),
            Token::Tag { value: 0x00, .. } => {
                if let Some(Generation3dMountedContainerOwner::Record { root_field: None, field, .. }) = self.stack.last_mut() {
                    if matches!(*field, Some(5 | 6)) {
                        *field = None;
                    }
                }
            }
            Token::WirePresence(_) => {}
            Token::WireNodePresence(presence) => match self.stack.last_mut() {
                Some(Generation3dMountedContainerOwner::Wire { roles, roles_len, nodes, .. }) => {
                    let base = if *nodes == 0 { 0 } else { 3 };
                    roles[*roles_len] = base;
                    *roles_len += 1;
                    if presence & 1 != 0 {
                        roles[*roles_len] = base + 1;
                        *roles_len += 1;
                    }
                    if presence & 2 != 0 {
                        roles[*roles_len] = base + 2;
                        *roles_len += 1;
                    }
                    *nodes += 1;
                }
                _ => return Err("generation3d-mounted.wire-node-owner"),
            },
            Token::TablePresence { rows, value } => match self.stack.last_mut() {
                Some(Generation3dMountedContainerOwner::Synapses { present, .. } | Generation3dMountedContainerOwner::Generations { present, .. } | Generation3dMountedContainerOwner::Dictionary { present, .. }) if rows as usize == present.len() && value == 0 => {
                    present.fill(true);
                }
                _ => {}
            },
            Token::TableBitmap { first_row, value } => if let Some(Generation3dMountedContainerOwner::Synapses { present, .. } | Generation3dMountedContainerOwner::Generations { present, .. } | Generation3dMountedContainerOwner::Dictionary { present, .. }) = self.stack.last_mut() {
                for bit in 0..8 {
                    let row = first_row as usize + bit;
                    if row < present.len() {
                        present[row] = value & (1 << bit) != 0;
                    }
                }
            },
            Token::End(kind) => {
                if !self.end_dsl(kind)? && !self.end_json(kind)? {
                    self.end(kind)?;
                }
            }
            Token::Complete { .. } => {
                if !self.stack.is_empty() || self.string.is_some() || !self.json_stack.is_empty() || self.json_destination.is_some() || !self.dsl_stack.is_empty() || self.dsl_destination.is_some() {
                    return Err("generation3d-mounted.typed-terminal-populated");
                }
                self.complete = true;
            }
            Token::Tag { .. } | Token::Unsigned { .. } | Token::Byte(_) | Token::WireLabelPresence(_) => {}
        }
        Ok(())
    }

    fn take(&mut self) -> Option<Generation3dSnapshot> {
        if !self.complete || self.handed_back {
            return None;
        }
        self.handed_back = true;
        self.candidate.take()
    }

    fn close_step(&mut self) -> bool {
        self.string = None;
        self.json_destination = None;
        self.dsl_destination = None;
        if self.dsl_stack.pop().is_some() {
            return false;
        }
        if self.json_stack.pop().is_some() {
            return false;
        }
        if self.stack.pop().is_some() {
            return false;
        }
        if let Some(candidate) = self.candidate.take() {
            candidate.retire_cold();
            return false;
        }
        self.handed_back = true;
        true
    }

    fn terminal_is_empty(&self) -> bool {
        self.handed_back
            && self.candidate.is_none()
            && self.stack.is_empty()
            && self.string.is_none()
            && self.json_stack.is_empty()
            && self.json_destination.is_none()
            && self.dsl_stack.is_empty()
            && self.dsl_destination.is_none()
    }
}

impl Drop for Generation3dMountedTypedSnapshotOwner {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || mounted::RetainedTypedPackOwner::terminal_is_empty(self), "Generation3d mounted typed snapshot owner reached Drop before handoff or terminal-empty close");
    }
}

/// 🧵️ The mounted canonical session of this artifact: its own `P3D3` discriminator, then the unchanged
/// canonical `.spk` stream [`encode_mounted`] writes. `P2D2` and every other header is refused before
/// the typed owner or any retained cursor is allocated.
pub type Generation3dMountedPackSession = mounted::RetainedTypedPackSession<Generation3dMountedTypedSnapshotOwner>;

/// 🚪️ Opens a mounted session over exactly `expected_bytes` stream bytes and `maximum_items` items.
pub fn generation3d_mounted_pack_session(expected_bytes: usize, maximum_items: usize) -> Result<Generation3dMountedPackSession, &'static str> {
    Generation3dMountedPackSession::new(GENERATION3D_MOUNTED_PREFIX.to_vec(), expected_bytes, maximum_items, GENERATION3D_MOUNTED_TYPED_DEPTH as u16, Generation3dMountedTypedSnapshotOwner::new)
}
//#endregion 🔖️MountedCanonicalPackSession
