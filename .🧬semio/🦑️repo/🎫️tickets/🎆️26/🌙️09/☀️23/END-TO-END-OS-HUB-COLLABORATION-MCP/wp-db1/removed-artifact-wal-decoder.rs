const ARTIFACT_WAL_DEPENDENCIES: usize = 64;
const ARTIFACT_WAL_FIELD_BYTES: usize = (db_storage::DB_IO_OPERATION_PAGES - 1) * db_storage::DB_IO_PAGE_BYTES;

struct ArtifactWalTextField {
    bytes: [u8; db_storage::DbIoText::maximum_capacity()],
    len: usize,
    remaining: Option<usize>,
}

impl ArtifactWalTextField {
    fn new() -> Self {
        Self { bytes: [0; db_storage::DbIoText::maximum_capacity()], len: 0, remaining: None }
    }

    fn poll(&mut self, cursor: &mut db_wal::WalBytesCursor<'_>, control: &mut db_wal::WalCursorControl) -> Result<Option<db_storage::DbIoText>, DbError> {
        if self.remaining.is_none() {
            self.remaining = Some(cursor.begin_field(db_storage::DbIoText::maximum_capacity() as u64, control)?);
            return Ok(None);
        }
        let remaining = self.remaining.as_mut().ok_or_else(|| DbError::Internal("artifact WAL text lost remaining bytes".to_string()))?;
        if *remaining != 0 {
            let copied = cursor.read_field_fragment(remaining, &mut self.bytes[self.len..], control)?;
            self.len += copied;
            return Ok(None);
        }
        let value = std::str::from_utf8(&self.bytes[..self.len]).map_err(|_| DbError::Corrupt("artifact WAL text is not UTF-8".to_string()))?;
        let text = db_storage::DbIoText::try_from_str(value)?;
        *self = Self::new();
        Ok(Some(text))
    }
}

struct ArtifactWalPageField {
    remaining: Option<usize>,
    writer: Option<db_storage::DbIoPageWriter>,
    seal: Option<db_storage::DbIoPageWriterSeal>,
}

impl ArtifactWalPageField {
    fn new() -> Self {
        Self { remaining: None, writer: None, seal: None }
    }

    fn poll(&mut self, cursor: &mut db_wal::WalBytesCursor<'_>, control: &mut db_wal::WalCursorControl, context: &mut std::task::Context<'_>) -> std::task::Poll<Result<db_storage::DbIoPages, DbError>> {
        if self.remaining.is_none() {
            let remaining = match cursor.begin_field(ARTIFACT_WAL_FIELD_BYTES as u64, control) {
                Ok(remaining) => remaining,
                Err(error) => return std::task::Poll::Ready(Err(error)),
            };
            let writer = match db_storage::DbIoPageWriter::try_reserve(remaining.div_ceil(db_storage::DB_IO_PAGE_BYTES)) {
                Ok(writer) => writer,
                Err(rejected) => return std::task::Poll::Ready(Err(rejected.into_error())),
            };
            self.remaining = Some(remaining);
            self.writer = Some(writer);
            context.waker().wake_by_ref();
            return std::task::Poll::Pending;
        }
        let remaining = self.remaining.as_mut().ok_or_else(|| DbError::Internal("artifact WAL page field lost remaining bytes".to_string()));
        let remaining = match remaining {
            Ok(remaining) => remaining,
            Err(error) => return std::task::Poll::Ready(Err(error)),
        };
        if *remaining != 0 {
            let mut fragment = [0u8; db_storage::DB_IO_PAGE_BYTES];
            let copied = match cursor.read_field_fragment(remaining, &mut fragment, control) {
                Ok(copied) => copied,
                Err(error) => return std::task::Poll::Ready(Err(error)),
            };
            let written = match self.writer.as_mut().ok_or_else(|| DbError::Internal("artifact WAL page field lost writer".to_string())).and_then(|writer| {
                let head = writer.write_fragment(&fragment[..copied])?;
                if head == copied {
                    return Ok(head);
                }
                Ok(head + writer.write_fragment(&fragment[head..copied])?)
            }) {
                Ok(written) => written,
                Err(error) => return std::task::Poll::Ready(Err(error)),
            };
            if written != copied {
                return std::task::Poll::Ready(Err(DbError::Internal("artifact WAL page field writer made a partial admitted write".to_string())));
            }
            context.waker().wake_by_ref();
            return std::task::Poll::Pending;
        }
        if self.seal.is_none() {
            let writer = match self.writer.take() {
                Some(writer) => writer,
                None => return std::task::Poll::Ready(Err(DbError::Internal("artifact WAL page field lost seal owner".to_string()))),
            };
            self.seal = Some(writer.seal_retained());
            context.waker().wake_by_ref();
            return std::task::Poll::Pending;
        }
        let seal = match self.seal.as_mut() {
            Some(seal) => seal,
            None => return std::task::Poll::Ready(Err(DbError::Internal("artifact WAL page field lost retained seal".to_string()))),
        };
        match Pin::new(seal).poll(context) {
            std::task::Poll::Pending => std::task::Poll::Pending,
            std::task::Poll::Ready(Ok(pages)) => {
                self.seal = None;
                self.remaining = None;
                std::task::Poll::Ready(Ok(pages))
            }
            std::task::Poll::Ready(Err(rejected)) => std::task::Poll::Ready(Err(rejected.into_error())),
        }
    }
}

struct ArtifactWalRetainedEnvelope {
    mutation_id: db_storage::DbIoText,
    document_id: db_storage::DbIoText,
    actor: db_storage::DbIoText,
    dependencies: [Option<db_storage::DbIoText>; ARTIFACT_WAL_DEPENDENCIES],
    dependency_count: u8,
    diff_schema: db_storage::DbIoText,
    diff_payload: db_storage::DbIoPages,
    inverse_schema: db_storage::DbIoText,
    inverse_payload: db_storage::DbIoPages,
    timestamp: protocol::HybridLogicalTimestamp,
}

struct ArtifactWalEnvelopeDecode<'bytes, 'control> {
    cursor: db_wal::WalBytesCursor<'bytes>,
    control: &'control mut db_wal::WalCursorControl,
    phase: u8,
    text: ArtifactWalTextField,
    page: ArtifactWalPageField,
    mutation_id: Option<db_storage::DbIoText>,
    document_id: Option<db_storage::DbIoText>,
    actor: Option<db_storage::DbIoText>,
    dependencies: [Option<db_storage::DbIoText>; ARTIFACT_WAL_DEPENDENCIES],
    dependency_count: u8,
    dependency: u8,
    diff_schema: Option<db_storage::DbIoText>,
    diff_payload: Option<db_storage::DbIoPages>,
    inverse_schema: Option<db_storage::DbIoText>,
    inverse_payload: Option<db_storage::DbIoPages>,
    timestamp: Option<protocol::HybridLogicalTimestamp>,
}

fn decode_retained_envelope<'bytes, 'control>(bytes: &'bytes db_wal::WalBytes, control: &'control mut db_wal::WalCursorControl) -> ArtifactWalEnvelopeDecode<'bytes, 'control> {
    ArtifactWalEnvelopeDecode {
        cursor: bytes.cursor(),
        control,
        phase: 0,
        text: ArtifactWalTextField::new(),
        page: ArtifactWalPageField::new(),
        mutation_id: None,
        document_id: None,
        actor: None,
        dependencies: std::array::from_fn(|_| None),
        dependency_count: 0,
        dependency: 0,
        diff_schema: None,
        diff_payload: None,
        inverse_schema: None,
        inverse_payload: None,
        timestamp: None,
    }
}

impl Future for ArtifactWalEnvelopeDecode<'_, '_> {
    type Output = Result<ArtifactWalRetainedEnvelope, DbError>;

    fn poll(mut self: Pin<&mut Self>, context: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
        let owner = self.as_mut().get_mut();
        let result = match owner.phase {
            0 | 1 | 2 | 4 | 5 | 7 => owner.text.poll(&mut owner.cursor, &mut *owner.control),
            _ => Ok(None),
        };
        match result {
            Err(error) => return std::task::Poll::Ready(Err(error)),
            Ok(Some(text)) => match owner.phase {
                0 => owner.mutation_id = Some(text),
                1 => owner.document_id = Some(text),
                2 => owner.actor = Some(text),
                4 => {
                    owner.dependencies[owner.dependency as usize] = Some(text);
                    owner.dependency += 1;
                    if owner.dependency < owner.dependency_count {
                        context.waker().wake_by_ref();
                        return std::task::Poll::Pending;
                    }
                }
                5 => owner.diff_schema = Some(text),
                7 => owner.inverse_schema = Some(text),
                _ => return std::task::Poll::Ready(Err(DbError::Internal("artifact WAL text completed outside its phase".to_string()))),
            },
            Ok(None) if matches!(owner.phase, 0 | 1 | 2 | 4 | 5 | 7) => {
                context.waker().wake_by_ref();
                return std::task::Poll::Pending;
            }
            Ok(None) => {}
        }
        match owner.phase {
            0 | 1 | 2 | 4 | 5 | 7 => owner.phase += 1,
            3 => {
                let count = match owner.cursor.varint(&mut *owner.control) {
                    Ok(count) => count,
                    Err(error) => return std::task::Poll::Ready(Err(error)),
                };
                if let Err(error) = check_len(count, ARTIFACT_WAL_DEPENDENCIES as u64, "artifact WAL envelope dependencies") {
                    return std::task::Poll::Ready(Err(error));
                }
                owner.dependency_count = count as u8;
                owner.phase = if count == 0 { 5 } else { 4 };
            }
            6 => match owner.page.poll(&mut owner.cursor, owner.control, context) {
                std::task::Poll::Pending => return std::task::Poll::Pending,
                std::task::Poll::Ready(Ok(pages)) => {
                    owner.diff_payload = Some(pages);
                    owner.phase = 7;
                }
                std::task::Poll::Ready(Err(error)) => return std::task::Poll::Ready(Err(error)),
            },
            8 => match owner.page.poll(&mut owner.cursor, owner.control, context) {
                std::task::Poll::Pending => return std::task::Poll::Pending,
                std::task::Poll::Ready(Ok(pages)) => {
                    owner.inverse_payload = Some(pages);
                    owner.phase = 9;
                }
                std::task::Poll::Ready(Err(error)) => return std::task::Poll::Ready(Err(error)),
            },
            9 => {
                let actor = match owner.cursor.varint(&mut *owner.control) {
                    Ok(value) => value,
                    Err(error) => return std::task::Poll::Ready(Err(error)),
                };
                let physical_ms = match owner.cursor.varint(&mut *owner.control) {
                    Ok(value) => value,
                    Err(error) => return std::task::Poll::Ready(Err(error)),
                };
                let logical = match owner.cursor.varint(&mut *owner.control) {
                    Ok(value) => value,
                    Err(error) => return std::task::Poll::Ready(Err(error)),
                };
                owner.timestamp = Some(protocol::HybridLogicalTimestamp { actor, physical_ms, logical });
                owner.phase = 10;
            }
            10 => {
                if owner.cursor.remaining() != 0 {
                    return std::task::Poll::Ready(Err(DbError::Corrupt("WAL command envelope has trailing bytes".to_string())));
                }
                return std::task::Poll::Ready(Ok(ArtifactWalRetainedEnvelope {
                    mutation_id: owner.mutation_id.take().ok_or_else(|| DbError::Internal("artifact WAL decode lost mutation identity".to_string()))?,
                    document_id: owner.document_id.take().ok_or_else(|| DbError::Internal("artifact WAL decode lost document identity".to_string()))?,
                    actor: owner.actor.take().ok_or_else(|| DbError::Internal("artifact WAL decode lost actor identity".to_string()))?,
                    dependencies: std::mem::replace(&mut owner.dependencies, std::array::from_fn(|_| None)),
                    dependency_count: owner.dependency_count,
                    diff_schema: owner.diff_schema.take().ok_or_else(|| DbError::Internal("artifact WAL decode lost diff schema".to_string()))?,
                    diff_payload: owner.diff_payload.take().ok_or_else(|| DbError::Internal("artifact WAL decode lost diff pages".to_string()))?,
                    inverse_schema: owner.inverse_schema.take().ok_or_else(|| DbError::Internal("artifact WAL decode lost inverse schema".to_string()))?,
                    inverse_payload: owner.inverse_payload.take().ok_or_else(|| DbError::Internal("artifact WAL decode lost inverse pages".to_string()))?,
                    timestamp: owner.timestamp.take().ok_or_else(|| DbError::Internal("artifact WAL decode lost timestamp".to_string()))?,
                }));
            }
            _ => return std::task::Poll::Ready(Err(DbError::Internal("artifact WAL decoder reached a stale phase".to_string()))),
        }
        context.waker().wake_by_ref();
        std::task::Poll::Pending
    }
}

struct ArtifactWalEnvelopeAdapter<'control> {
    retained: Option<ArtifactWalRetainedEnvelope>,
    control: &'control mut db_wal::WalCursorControl,
    phase: u8,
    dependency: u8,
    page: u8,
    mutation_id: Option<protocol::MutationId>,
    document_id: Option<protocol::ArtifactId>,
    actor: Option<protocol::ActorId>,
    dependencies: Vec<protocol::MutationId>,
    diff_schema: Option<protocol::SchemaId>,
    diff_payload: Vec<u8>,
    inverse_schema: Option<protocol::SchemaId>,
    inverse_payload: Vec<u8>,
}

impl ArtifactWalEnvelopeAdapter<'_> {
    fn take_text(text: &mut db_storage::DbIoText) -> String {
        let value = text.as_str().to_string();
        text.close_step();
        value
    }
}

fn adapt_retained_envelope<'control>(retained: ArtifactWalRetainedEnvelope, control: &'control mut db_wal::WalCursorControl) -> ArtifactWalEnvelopeAdapter<'control> {
    ArtifactWalEnvelopeAdapter {
        retained: Some(retained),
        control,
        phase: 0,
        dependency: 0,
        page: 0,
        mutation_id: None,
        document_id: None,
        actor: None,
        dependencies: Vec::new(),
        diff_schema: None,
        diff_payload: Vec::new(),
        inverse_schema: None,
        inverse_payload: Vec::new(),
    }
}

impl Future for ArtifactWalEnvelopeAdapter<'_> {
    type Output = Result<protocol::MutationEnvelope, DbError>;

    fn poll(mut self: Pin<&mut Self>, context: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
        let owner = self.as_mut().get_mut();
        if let Err(error) = owner.control.grant() {
            return std::task::Poll::Ready(Err(error));
        }
        let retained = match owner.retained.as_mut() {
            Some(retained) => retained,
            None => return std::task::Poll::Ready(Err(DbError::Internal("artifact WAL adapter was polled after terminal handoff".to_string()))),
        };
        match owner.phase {
            0 => owner.mutation_id = Some(protocol::MutationId(Self::take_text(&mut retained.mutation_id))),
            1 => owner.document_id = Some(protocol::ArtifactId(Self::take_text(&mut retained.document_id))),
            2 => owner.actor = Some(protocol::ActorId(Self::take_text(&mut retained.actor))),
            3 if owner.dependency < retained.dependency_count => {
                let text = retained.dependencies[owner.dependency as usize].as_mut().ok_or_else(|| DbError::Internal("artifact WAL adapter lost dependency owner".to_string()))?;
                owner.dependencies.push(protocol::MutationId(Self::take_text(text)));
                owner.dependency += 1;
                context.waker().wake_by_ref();
                return std::task::Poll::Pending;
            }
            3 => {}
            4 => owner.diff_schema = Some(protocol::SchemaId(Self::take_text(&mut retained.diff_schema))),
            5 if owner.page < retained.diff_payload.page_count() => {
                let fragment = retained.diff_payload.page(owner.page).ok_or_else(|| DbError::Internal("artifact WAL adapter lost diff page".to_string()))?;
                owner.diff_payload.extend_from_slice(fragment);
                owner.page += 1;
                context.waker().wake_by_ref();
                return std::task::Poll::Pending;
            }
            5 => owner.page = 0,
            6 => match retained.diff_payload.close_step()? {
                Some(_) => {
                    context.waker().wake_by_ref();
                    return std::task::Poll::Pending;
                }
                None => {}
            },
            7 => owner.inverse_schema = Some(protocol::SchemaId(Self::take_text(&mut retained.inverse_schema))),
            8 if owner.page < retained.inverse_payload.page_count() => {
                let fragment = retained.inverse_payload.page(owner.page).ok_or_else(|| DbError::Internal("artifact WAL adapter lost inverse page".to_string()))?;
                owner.inverse_payload.extend_from_slice(fragment);
                owner.page += 1;
                context.waker().wake_by_ref();
                return std::task::Poll::Pending;
            }
            8 => owner.page = 0,
            9 => match retained.inverse_payload.close_step()? {
                Some(_) => {
                    context.waker().wake_by_ref();
                    return std::task::Poll::Pending;
                }
                None => {}
            },
            10 => {
                let retained = owner.retained.take().ok_or_else(|| DbError::Internal("artifact WAL adapter lost terminal owner".to_string()))?;
                return std::task::Poll::Ready(Ok(protocol::MutationEnvelope {
                    mutation_id: owner.mutation_id.take().ok_or_else(|| DbError::Internal("artifact WAL adapter lost mutation identity".to_string()))?,
                    document_id: owner.document_id.take().ok_or_else(|| DbError::Internal("artifact WAL adapter lost document identity".to_string()))?,
                    actor: owner.actor.take().ok_or_else(|| DbError::Internal("artifact WAL adapter lost actor identity".to_string()))?,
                    dependencies: std::mem::take(&mut owner.dependencies),
                    diff: protocol::ArtifactDiff { schema: owner.diff_schema.take().ok_or_else(|| DbError::Internal("artifact WAL adapter lost diff schema".to_string()))?, payload: std::mem::take(&mut owner.diff_payload) },
                    inverse: protocol::InverseMutation { schema: owner.inverse_schema.take().ok_or_else(|| DbError::Internal("artifact WAL adapter lost inverse schema".to_string()))?, payload: std::mem::take(&mut owner.inverse_payload) },
                    timestamp: retained.timestamp,
                }));
            }
            _ => return std::task::Poll::Ready(Err(DbError::Internal("artifact WAL adapter reached a stale phase".to_string()))),
        }
        owner.phase += 1;
        context.waker().wake_by_ref();
        std::task::Poll::Pending
    }
}

