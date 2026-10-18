//! 📬️ The command text and binary formats: the `.ops` header grammar, command header lines, supersede input lines and
//! the command transaction codec shared by the text format, the history fold and the composition coordinator.
//!
//! Layering: `📓️2026-10-13-wave-e1-os-kernel.md` (ticket BUILD-SEMIO-TECH-PLAY-FOR-CDN-DEPLOYMENT).
use protocol::{MutationId, UndoPolicy, Mutation, MutationDiff, OpBinary, OpText};
use semio_framework_os_vcs::{Author, VcsError};
use semio_framework_diagnostic::{TextError, TextSpan};
use semio_framework_os_pack::{PackDecodeOptions, PackEncodeOptions, PackError, PackRefusal, pack_rt};
#[path="🧬️schema/🦀️.rs"]
mod schema;
pub use schema::{ArtifactCommand, HistoryLane, OpenToolTransaction, SupersedeInput};
#[path="♻️retirement/🦀️.rs"]
mod retirement;

#[path="🛬️receiving/🦀️.rs"]
pub mod receiving;

/// 🖋️ One `by=[...]` list entry on a `commit` line: id then name, both positional
/// (bare-preferred, quoted only when needed — e.g. a name containing a space), then the optional
/// avatar — every field the transition's content-addressed id covers.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord)]
pub struct OpsAuthor {
    #[dsl(positional)]
    pub id: String,
    #[dsl(positional)]
    pub name: String,
    pub avatar: Option<String>,
}

impl From<&Author> for OpsAuthor {
    fn from(author: &Author) -> Self {
        Self { id: author.id.clone(), name: author.name.clone(), avatar: author.avatar.clone() }
    }
}

impl From<OpsAuthor> for Author {
    fn from(author: OpsAuthor) -> Self {
        Self { id: author.id, name: author.name, avatar: author.avatar }
    }
}

/// 📌️ One `pins=[...]` entry on a `repin` line: child artifact uri, then its checkpoint.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord)]
pub struct OpsPin {
    #[dsl(positional)]
    pub child: String,
    #[dsl(positional)]
    pub checkpoint: String,
}

/// ✏️ One `inputs=[...]` entry on a `supersede` line: the superseded operation, then its
/// replacement — `schema` plus the base64 `OpBinary` `payload`, or neither for a withdrawal.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord)]
pub struct OpsSupersededInput {
    #[dsl(positional)]
    pub target: String,
    pub schema: Option<String>,
    pub payload: Option<String>,
}

impl From<protocol::SupersededInput> for OpsSupersededInput {
    fn from(input: protocol::SupersededInput) -> Self {
        match input.replacement {
            protocol::InputReplacement::Input { schema, payload } => Self { target: input.target.0, schema: Some(schema), payload: Some(semio_framework_io_base64::base64_standard_encode(&payload)) },
            protocol::InputReplacement::Withdrawn => Self { target: input.target.0, schema: None, payload: None },
        }
    }
}

/// 🧾️ One `.ops` header/structural line — `doc`/`edit` and one line per history transition
/// (`revert`/`reinstate`/`commit`/`branch`/`checkout`/`repin`/`supersede`) — re-derived directly on the `dsl_schema` grammar engine (`#[derive(DslOps)]` generates
/// `OpText::parse_op`/`print_op` from this declaration; see {@link print_edit_lines}/
/// {@link print_document_text}/{@link parse_document_text}, its only callers). Sigil-free lowercase
/// keywords (bare `doc`, never `@doc` — `@` is reserved for connection points everywhere else in the
/// unified DSL syntax); `id` is always the first positional field on every line; every other field is
/// a plain `key=value` attribute that is simply OMITTED when absent (no more `-` placeholder
/// sentinel); `operations`/`after`/`by`/`pins` are real DSL lists (`by=[ u1 "Ueli Saluz" ]`), not
/// comma-joined, percent-escaped strings.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslEnum)]
pub enum OpsHeaderLine {
    Doc {
        #[dsl(positional)]
        id: String,
        schema: String,
    },
    Edit {
        #[dsl(positional)]
        id: String,
        sequence: i32,
        started: String,
        actor: Option<semio_framework_value::SharedUtf8>,
        finished: Option<String>,
        verb: Option<String>,
        line: Vec<String>,
    },
    /// ⏪️ A `Revert` transition: `clock` is the HLC as `physical.logical.actor`, `after` its
    /// causal dependencies; every transition line carries both so its envelope id re-derives exactly.
    Revert {
        #[dsl(positional)]
        id: String,
        actor: semio_framework_value::SharedUtf8,
        clock: String,
        after: Vec<String>,
        operations: Vec<String>,
    },
    /// ⏩️ A `Reinstate` transition.
    Reinstate {
        #[dsl(positional)]
        id: String,
        actor: semio_framework_value::SharedUtf8,
        clock: String,
        after: Vec<String>,
        operations: Vec<String>,
    },
    /// 🚩️ A `Commit` transition with the change and checkpoint facts it introduces.
    Commit {
        #[dsl(positional)]
        id: String,
        actor: semio_framework_value::SharedUtf8,
        clock: String,
        after: Vec<String>,
        checkpoint: String,
        parent: Option<String>,
        change: String,
        operations: Vec<String>,
        description: Option<String>,
        saved: String,
        by: Vec<OpsAuthor>,
        message: Option<String>,
        at: String,
        line: Vec<String>,
    },
    /// 🌿️ A `Branch` transition.
    Branch {
        #[dsl(positional)]
        id: String,
        actor: semio_framework_value::SharedUtf8,
        clock: String,
        after: Vec<String>,
        alternative: String,
        name: String,
        checkpoint: String,
    },
    /// 🎯️ A `Checkout` transition.
    Checkout {
        #[dsl(positional)]
        id: String,
        actor: semio_framework_value::SharedUtf8,
        clock: String,
        after: Vec<String>,
        checkpoint: String,
        alternative: Option<String>,
    },
    /// 🧩️ A `Repin` transition.
    Repin {
        #[dsl(positional)]
        id: String,
        actor: semio_framework_value::SharedUtf8,
        clock: String,
        after: Vec<String>,
        checkpoint: String,
        pinned: String,
        pins: Vec<OpsPin>,
    },
    /// ✏️ A `Supersede` transition: every input it replaces, document-wide or within `scope`, and the newest foreign
    /// operation its author had `observed` (its grading frontier at the hub).
    Supersede {
        #[dsl(positional)]
        id: String,
        actor: semio_framework_value::SharedUtf8,
        clock: String,
        after: Vec<String>,
        scope: Option<String>,
        inputs: Vec<OpsSupersededInput>,
        observed: Option<String>,
    },
    /// 🔙️ One edit's complete inverse sequence, encoded with the operation's own text grammar.
    Inverse { edit: String, ops: Vec<String> },
    /// 🪪️ One authoritative metadata record for a forward operation.
    Metadata { edit: String, index: u32, data: String },
    /// 📨️ One durable diagnostic ledger entry, explicitly owned by an edit.
    Message { edit: String, data: String },
    /// ⚔️ One first-class conflict, including its content-addressed identity and lifecycle.
    Conflict { data: String },
}

/// 🌿️ A text record explicitly carries zero trunk/unknown or one authored branch identity.
pub fn ops_line_provenance(mut lines: Vec<String>) -> Result<Option<String>, TextError> {
    if lines.len() > 1 {
        return Err(TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,("history line admits zero or one identity").to_string(),TextSpan::at(1,1)));
    }
    Ok(lines.pop())
}

/// 🎞️ Handcrafted OpText (P6).
impl OpText for OpsHeaderLine {
    fn parse_op(line: &str) -> Result<Self, TextError> {
        let variants = <Self as semio_framework_dsl_record::DslVariants>::variants();
        for (keyword, spec_fn) in &variants {
            let probe = format!("{} ", keyword);
            if line == keyword.as_str() || line.starts_with(&probe) {
                let record = semio_framework_dsl_record::parse(line, &(spec_fn.ordinary)(), &semio_framework_dsl_record::ParseOptions { limits: semio_framework_diagnostic::Limits::default(), mode: semio_framework_dsl_record::SourceMode::Inline })?;
                return <Self as semio_framework_dsl_record::DslVariants>::from_named_record(keyword, &record);
            }
        }
        Err(TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,(format!("unknown operation line '{line}'")).to_string(),TextSpan::at(1,1)))
    }
    fn print_op(&self) -> String {
        let (keyword, record) = <Self as semio_framework_dsl_record::DslVariants>::to_named_record(self);
        let variants = <Self as semio_framework_dsl_record::DslVariants>::variants();
        let spec_fn = variants.iter().find(|(k, _)| k == &keyword).map(|(_, s)| *s).expect("variant spec must exist for its own keyword");
        semio_framework_dsl_record::print(&record, &(spec_fn.ordinary)(), semio_framework_dsl_record::JoinMode::Inline)
    }
}

/// 🎯️ Handcrafted OpBinary (P6).
impl OpBinary for OpsHeaderLine {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        const OP_BINARY_FORMAT: u8 = 1;
        let (keyword, record) = <Self as semio_framework_dsl_record::DslVariants>::to_named_record(self);
        let variants = <Self as semio_framework_dsl_record::DslVariants>::variants();
        let ordinal = variants.iter().position(|(k, _)| *k == keyword).ok_or(protocol::ProtocolError::Malformed { what: "op variant", offset: 0, detail: format!("keyword {keyword:?} is not a declared variant") })?;
        let spec = (variants[ordinal].1.ordinary)();
        let body = semio_framework_os_pack::record::encode_record_body(&spec, &record, &PackEncodeOptions::default()).map_err(protocol::ProtocolError::from)?;
        let mut out = Vec::with_capacity(body.len() + 3);
        out.push(OP_BINARY_FORMAT);
        semio_framework_os_pack::write_varint_u64(&mut out, ordinal as u64);
        out.extend_from_slice(&body);
        Ok(out)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        const OP_BINARY_FORMAT: u8 = 1;
        let mut reader = semio_framework_os_pack::ByteReader::new(bytes);
        let format = reader.read_u8()?;
        if format != OP_BINARY_FORMAT {
            return Err(protocol::ProtocolError::Malformed { what: "op format", offset: 0, detail: format!("unsupported op format {format}") });
        }
        let ordinal = reader.read_varint_u64()?;
        let variants = <Self as semio_framework_dsl_record::DslVariants>::variants();
        let (keyword, spec_fn) = variants.get(ordinal as usize).ok_or(protocol::ProtocolError::Malformed { what: "op variant", offset: 1, detail: format!("ordinal {ordinal} out of range for {} declared variants", variants.len()) })?;
        let spec = (spec_fn.ordinary)();
        let body = &bytes[reader.position()..];
        let (record, _report) = semio_framework_os_pack::record::decode_record_body(body, &spec, &PackDecodeOptions::default()).map_err(protocol::ProtocolError::from)?;
        let record_offset = reader.position() as u64;
        <Self as semio_framework_dsl_record::DslVariants>::from_named_record(keyword, &record).map_err(|error| protocol::ProtocolError::Malformed { what: "op record", offset: record_offset, detail: error.to_string() })
    }
}

/// 🕹️ One structural `ArtifactCommand` line — `apply`/`undo`/`redo`/`commit-checkpoint`/
/// `create-alternative`/`switch-alternative`/`checkout` — the command-level twin of
/// `OpsHeaderLine`, re-derived on the same `dsl_schema` grammar engine. `Apply` carries no
/// operations here (those follow as 2-space-indented `Op::print_op` lines, exactly like
/// `print_edit_lines`); `Undo`'s `policy` is `None` for the plain `undo` command and `Some(token)`
/// for `UndoWithPolicy` (token = kebab of the `UndoPolicy` variant name), optionally followed by an
/// indented nested command block for `semantic-undo`/`compensating-action`.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslEnum)]
pub enum CommandHeaderLine {
    /// ▶️ `transaction`/`tool` are the [`protocol::TransactionRef`] halves, present together or not at all.
    Apply {
        transaction: Option<String>,
        tool: Option<String>,
    },
    Undo {
        policy: Option<String>,
    },
    Redo,
    CommitCheckpoint {
        message: Option<String>,
        by: Vec<OpsAuthor>,
    },
    CreateAlternative {
        name: String,
    },
    SwitchAlternative {
        #[dsl(positional)]
        id: String,
    },
    Checkout {
        #[dsl(positional)]
        id: String,
    },
    PruneDrafts,
    /// 🛤️ Text twin of `ArtifactCommand::ApplyInLane` — `lane` is a kebab token
    /// (`history_lane_to_token`/`parse_history_lane_token`), matching `Undo.policy`'s convention.
    ApplyInLane {
        lane: String,
        transaction: Option<String>,
        tool: Option<String>,
    },
    UndoInLane {
        lane: String,
    },
    RedoInLane {
        lane: String,
    },
    /// ⚖️ Text twin of `ArtifactCommand::SetMergePolicy` — `policy` is a kebab token
    /// (`merge_policy_to_token`/`parse_merge_policy_token`), matching `Undo.policy`'s convention.
    SetMergePolicy {
        policy: String,
    },
    /// ⚔️ Text twin of `ArtifactCommand::ResolveConflict` — `resolution` is a kebab token
    /// (`conflict_resolution_to_token`/`parse_conflict_resolution_token`).
    ResolveConflict {
        #[dsl(positional)]
        conflict_id: String,
        resolution: String,
    },
    /// ✏️ Text twin of `ArtifactCommand::Supersede`; its inputs follow as [`SupersedeInputLine`]s.
    Supersede {
        scope: Option<String>,
    },
    /// 🌿️ Text twin of `ArtifactCommand::CreateAlternativeWithSupersede`; its inputs follow as [`SupersedeInputLine`]s.
    CreateAlternativeWithSupersede {
        name: String,
    },
    /// 🎞️ Text twin of `ArtifactCommand::AppendTransaction` — the [`protocol::TransactionRef`] halves; its operations follow
    /// as indented op lines.
    AppendTransaction {
        transaction: String,
        tool: String,
    },
    /// ✅️ Text twin of `ArtifactCommand::CommitTransaction`.
    CommitTransaction {
        #[dsl(positional)]
        id: String,
    },
    /// 🧨️ Text twin of `ArtifactCommand::AbortTransaction`.
    AbortTransaction {
        #[dsl(positional)]
        id: String,
    },
}

/// ✏️ One 2-space-indented input line under a `supersede` command: `replace <target>` followed by the replacement's
/// 4-space-indented `print_op` line, or `withdraw <target>`.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslEnum)]
pub enum SupersedeInputLine {
    Replace {
        #[dsl(positional)]
        target: String,
    },
    Withdraw {
        #[dsl(positional)]
        target: String,
    },
}

impl OpText for SupersedeInputLine {
    fn parse_op(line: &str) -> Result<Self, TextError> {
        let variants = <Self as semio_framework_dsl_record::DslVariants>::variants();
        for (keyword, spec_fn) in &variants {
            let probe = format!("{} ", keyword);
            if line == keyword.as_str() || line.starts_with(&probe) {
                let record = semio_framework_dsl_record::parse(line, &(spec_fn.ordinary)(), &semio_framework_dsl_record::ParseOptions { limits: semio_framework_diagnostic::Limits::default(), mode: semio_framework_dsl_record::SourceMode::Inline })?;
                return <Self as semio_framework_dsl_record::DslVariants>::from_named_record(keyword, &record);
            }
        }
        Err(TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,(format!("unknown supersede input line '{line}'")).to_string(),TextSpan::at(1,1)))
    }
    fn print_op(&self) -> String {
        let (keyword, record) = <Self as semio_framework_dsl_record::DslVariants>::to_named_record(self);
        let variants = <Self as semio_framework_dsl_record::DslVariants>::variants();
        let spec_fn = variants.iter().find(|(k, _)| k == &keyword).map(|(_, s)| *s).expect("variant spec must exist for its own keyword");
        semio_framework_dsl_record::print(&record, &(spec_fn.ordinary)(), semio_framework_dsl_record::JoinMode::Inline)
    }
}

/// 📤️ Prints supersede inputs as their indented input lines (see [`SupersedeInputLine`]).
pub fn print_supersede_inputs<Op: OpText>(out: &mut String, inputs: &[SupersedeInput<Op>]) -> Result<(), VcsError> {
    for input in inputs {
        let line = match &input.replacement {
            Some(_) => SupersedeInputLine::Replace { target: input.target.0.clone() },
            None => SupersedeInputLine::Withdraw { target: input.target.0.clone() },
        };
        out.push_str("  ");
        out.push_str(&line.print_op());
        out.push('\n');
        if let Some(replacement) = &input.replacement {
            let printed = replacement.print_op();
            if printed.contains('\n') {
                return Err(VcsError::Serialize("op-text print_op must not contain a newline".into()));
            }
            out.push_str("    ");
            out.push_str(&printed);
            out.push('\n');
        }
    }
    Ok(())
}

/// 📥️ Parses the indented input lines [`print_supersede_inputs`] prints; at least one input is required. All-or-nothing:
/// on any failure every replacement parsed so far retires through its technology's cold disposal.
pub fn parse_supersede_inputs<P, Op: OpText + Mutation<P>>(lines: &[&str]) -> Result<Vec<SupersedeInput<Op>>, TextError> {
    let mut inputs: Vec<SupersedeInput<Op>> = Vec::new();
    match parse_supersede_input_lines(lines, &mut inputs) {
        Ok(()) => Ok(inputs),
        Err(error) => {
            retire_supersede_inputs::<P, Op>(inputs);
            Err(error)
        }
    }
}

/// 🧺️ Parses supersede input lines into `inputs`, leaving every parsed input there on failure.
pub fn parse_supersede_input_lines<Op: OpText>(lines: &[&str], inputs: &mut Vec<SupersedeInput<Op>>) -> Result<(), TextError> {
    let mut awaiting_replacement = false;
    for raw in lines {
        if let Some(operation) = raw.strip_prefix("    ") {
            if !awaiting_replacement {
                return Err(TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,(format!("replacement operation without a replace line: {raw:?}")).to_string(),TextSpan::at(1,1)));
            }
            let input = inputs.last_mut().expect("a replace line precedes its replacement");
            input.replacement = Some(Op::parse_op(operation.trim())?);
            awaiting_replacement = false;
            continue;
        }
        if awaiting_replacement {
            return Err(TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,("a replace line needs its 4-space-indented replacement operation").to_string(),TextSpan::at(1,1)));
        }
        let Some(line) = raw.strip_prefix("  ") else {
            return Err(TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,(format!("expected a 2-space-indented supersede input line, got: {raw:?}")).to_string(),TextSpan::at(1,1)));
        };
        match SupersedeInputLine::parse_op(line.trim())? {
            SupersedeInputLine::Replace { target } => {
                inputs.push(SupersedeInput { target: MutationId(target), replacement: None });
                awaiting_replacement = true;
            }
            SupersedeInputLine::Withdraw { target } => inputs.push(SupersedeInput { target: MutationId(target), replacement: None }),
        }
    }
    if awaiting_replacement {
        return Err(TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,("a replace line needs its 4-space-indented replacement operation").to_string(),TextSpan::at(1,1)));
    }
    if inputs.is_empty() {
        return Err(TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,("supersede requires at least one input line").to_string(),TextSpan::at(1,1)));
    }
    Ok(())
}

/// 🎯️ Binary twin of [`print_supersede_inputs`]: `count varint | (target str | 0 u8 | op len varint | op bytes | 1 u8)*`.
pub fn write_supersede_inputs<Op: OpBinary>(out: &mut Vec<u8>, inputs: &[SupersedeInput<Op>]) -> Result<(), protocol::ProtocolError> {
    semio_framework_os_pack::write_varint_u64(out, inputs.len() as u64);
    for input in inputs {
        write_command_str(out, &input.target.0);
        match &input.replacement {
            Some(replacement) => {
                out.push(0);
                let bytes = replacement.encode_op()?;
                semio_framework_os_pack::write_varint_u64(out, bytes.len() as u64);
                out.extend_from_slice(&bytes);
            }
            None => out.push(1),
        }
    }
    Ok(())
}

/// 🗳️ Reads a supersede command's inputs all-or-nothing: on any failure every replacement decoded so far retires through
/// its technology's cold disposal, so a refused command never drops a live operation.
pub fn read_supersede_inputs<P, Op: OpBinary + Mutation<P>>(reader: &mut semio_framework_os_pack::ByteReader<'_>) -> Result<Vec<SupersedeInput<Op>>, CommandDecodeError> {
    let count = reader.read_varint_u64()?;
    let mut inputs = Vec::with_capacity(count.min(reader.remaining() as u64) as usize);
    for index in 0..count as usize {
        match read_supersede_input(reader) {
            Ok(input) => inputs.push(input),
            Err(error) => {
                retire_supersede_inputs::<P, Op>(inputs);
                return Err(match error {
                    CommandDecodeError::Operation { error, .. } => CommandDecodeError::Operation { index, error },
                    layout => layout,
                });
            }
        }
    }
    Ok(inputs)
}

/// 🎫️ One supersede input: its target and its replacement operation or withdrawal tag.
pub fn read_supersede_input<Op: OpBinary>(reader: &mut semio_framework_os_pack::ByteReader<'_>) -> Result<SupersedeInput<Op>, CommandDecodeError> {
    let target = MutationId(read_command_str(reader)?);
    let replacement = match reader.read_u8()? {
        0 => Some(read_command_op(reader).map_err(|error| CommandDecodeError::Operation { index: 0, error })?),
        1 => None,
        tag => return Err(protocol::ProtocolError::Malformed { what: "supersede input tag", offset: reader.position() as u64, detail: format!("unknown supersede input tag {tag}") }.into()),
    };
    Ok(SupersedeInput { target, replacement })
}

/// 🎞️ Handcrafted OpText (P6).
impl OpText for CommandHeaderLine {
    fn parse_op(line: &str) -> Result<Self, TextError> {
        let variants = <Self as semio_framework_dsl_record::DslVariants>::variants();
        for (keyword, spec_fn) in &variants {
            let probe = format!("{} ", keyword);
            if line == keyword.as_str() || line.starts_with(&probe) {
                let record = semio_framework_dsl_record::parse(line, &(spec_fn.ordinary)(), &semio_framework_dsl_record::ParseOptions { limits: semio_framework_diagnostic::Limits::default(), mode: semio_framework_dsl_record::SourceMode::Inline })?;
                return <Self as semio_framework_dsl_record::DslVariants>::from_named_record(keyword, &record);
            }
        }
        Err(TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,(format!("unknown operation line '{line}'")).to_string(),TextSpan::at(1,1)))
    }
    fn print_op(&self) -> String {
        let (keyword, record) = <Self as semio_framework_dsl_record::DslVariants>::to_named_record(self);
        let variants = <Self as semio_framework_dsl_record::DslVariants>::variants();
        let spec_fn = variants.iter().find(|(k, _)| k == &keyword).map(|(_, s)| *s).expect("variant spec must exist for its own keyword");
        semio_framework_dsl_record::print(&record, &(spec_fn.ordinary)(), semio_framework_dsl_record::JoinMode::Inline)
    }
}

/// 🎯️ Handcrafted OpBinary (P6).
impl OpBinary for CommandHeaderLine {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        const OP_BINARY_FORMAT: u8 = 1;
        let (keyword, record) = <Self as semio_framework_dsl_record::DslVariants>::to_named_record(self);
        let variants = <Self as semio_framework_dsl_record::DslVariants>::variants();
        let ordinal = variants.iter().position(|(k, _)| *k == keyword).ok_or(protocol::ProtocolError::Malformed { what: "op variant", offset: 0, detail: format!("keyword {keyword:?} is not a declared variant") })?;
        let spec = (variants[ordinal].1.ordinary)();
        let body = semio_framework_os_pack::record::encode_record_body(&spec, &record, &PackEncodeOptions::default()).map_err(protocol::ProtocolError::from)?;
        let mut out = Vec::with_capacity(body.len() + 3);
        out.push(OP_BINARY_FORMAT);
        semio_framework_os_pack::write_varint_u64(&mut out, ordinal as u64);
        out.extend_from_slice(&body);
        Ok(out)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        const OP_BINARY_FORMAT: u8 = 1;
        let mut reader = semio_framework_os_pack::ByteReader::new(bytes);
        let format = reader.read_u8()?;
        if format != OP_BINARY_FORMAT {
            return Err(protocol::ProtocolError::Malformed { what: "op format", offset: 0, detail: format!("unsupported op format {format}") });
        }
        let ordinal = reader.read_varint_u64()?;
        let variants = <Self as semio_framework_dsl_record::DslVariants>::variants();
        let (keyword, spec_fn) = variants.get(ordinal as usize).ok_or(protocol::ProtocolError::Malformed { what: "op variant", offset: 1, detail: format!("ordinal {ordinal} out of range for {} declared variants", variants.len()) })?;
        let spec = (spec_fn.ordinary)();
        let body = &bytes[reader.position()..];
        let (record, _report) = semio_framework_os_pack::record::decode_record_body(body, &spec, &PackDecodeOptions::default()).map_err(protocol::ProtocolError::from)?;
        let record_offset = reader.position() as u64;
        <Self as semio_framework_dsl_record::DslVariants>::from_named_record(keyword, &record).map_err(|error| protocol::ProtocolError::Malformed { what: "op record", offset: record_offset, detail: error.to_string() })
    }
}

pub async fn undo_policy_to_token(policy: UndoPolicy) -> &'static str {
    match policy {
        UndoPolicy::ExactBaseOnly => "exact-base-only",
        UndoPolicy::TransformAgainstConcurrent => "transform-against-concurrent",
        UndoPolicy::SemanticUndo => "semantic-undo",
        UndoPolicy::CompensatingAction => "compensating-action",
    }
}

pub async fn parse_undo_policy_token(token: &str) -> Result<UndoPolicy, TextError> {
    match token {
        "exact-base-only" => Ok(UndoPolicy::ExactBaseOnly),
        "transform-against-concurrent" => Ok(UndoPolicy::TransformAgainstConcurrent),
        "semantic-undo" => Ok(UndoPolicy::SemanticUndo),
        "compensating-action" => Ok(UndoPolicy::CompensatingAction),
        other => Err(TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,(format!("unknown undo policy token {other:?}")).to_string(),TextSpan::at(1,1))),
    }
}

pub fn undo_policy_ordinal(policy: UndoPolicy) -> u8 {
    match policy {
        UndoPolicy::ExactBaseOnly => 0,
        UndoPolicy::TransformAgainstConcurrent => 1,
        UndoPolicy::SemanticUndo => 2,
        UndoPolicy::CompensatingAction => 3,
    }
}

pub fn undo_policy_from_ordinal(ordinal: u8) -> Result<UndoPolicy, protocol::ProtocolError> {
    match ordinal {
        0 => Ok(UndoPolicy::ExactBaseOnly),
        1 => Ok(UndoPolicy::TransformAgainstConcurrent),
        2 => Ok(UndoPolicy::SemanticUndo),
        3 => Ok(UndoPolicy::CompensatingAction),
        other => Err(protocol::ProtocolError::Malformed { what: "undo policy ordinal", offset: 0, detail: format!("unknown undo policy ordinal {other}") }),
    }
}

/// 🛤️ `HistoryLane`'s text token — the `ApplyInLane`/`UndoInLane`/
/// `RedoInLane` command lines' `lane=...` value, mirroring `undo_policy_to_token`'s convention.
pub async fn history_lane_to_token(lane: HistoryLane) -> &'static str {
    match lane {
        HistoryLane::Document => "document",
        HistoryLane::Interaction => "interaction",
    }
}

pub async fn parse_history_lane_token(token: &str) -> Result<HistoryLane, TextError> {
    match token {
        "document" => Ok(HistoryLane::Document),
        "interaction" => Ok(HistoryLane::Interaction),
        other => Err(TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,(format!("unknown history lane token {other:?}")).to_string(),TextSpan::at(1,1))),
    }
}

/// 🛤️ `HistoryLane`'s binary ordinal, mirroring `undo_policy_ordinal`'s convention.
pub fn history_lane_ordinal(lane: HistoryLane) -> u8 {
    match lane {
        HistoryLane::Document => 0,
        HistoryLane::Interaction => 1,
    }
}

pub fn history_lane_from_ordinal(ordinal: u8) -> Result<HistoryLane, protocol::ProtocolError> {
    match ordinal {
        0 => Ok(HistoryLane::Document),
        1 => Ok(HistoryLane::Interaction),
        other => Err(protocol::ProtocolError::Malformed { what: "history lane ordinal", offset: 0, detail: format!("unknown history lane ordinal {other}") }),
    }
}

/// ⚖️ `protocol::MergePolicy`'s text token — the `SetMergePolicy` command line's
/// `policy=...` value, mirroring `undo_policy_to_token`'s convention.
pub async fn merge_policy_to_token(policy: protocol::MergePolicy) -> &'static str {
    match policy {
        protocol::MergePolicy::LaissezFaire => "laissez-faire",
        protocol::MergePolicy::Normal => "normal",
        protocol::MergePolicy::Vigilant => "vigilant",
    }
}

pub async fn parse_merge_policy_token(token: &str) -> Result<protocol::MergePolicy, TextError> {
    match token {
        "laissez-faire" => Ok(protocol::MergePolicy::LaissezFaire),
        "normal" => Ok(protocol::MergePolicy::Normal),
        "vigilant" => Ok(protocol::MergePolicy::Vigilant),
        other => Err(TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,(format!("unknown merge policy token {other:?}")).to_string(),TextSpan::at(1,1))),
    }
}

/// ⚔️ `protocol::ConflictResolution`'s text token — the `ResolveConflict` command line's
/// `resolution=...` value, mirroring `undo_policy_to_token`'s convention.
pub async fn conflict_resolution_to_token(resolution: protocol::ConflictResolution) -> &'static str {
    match resolution {
        protocol::ConflictResolution::Accept => "accept",
        protocol::ConflictResolution::Discard => "discard",
    }
}

pub async fn parse_conflict_resolution_token(token: &str) -> Result<protocol::ConflictResolution, TextError> {
    match token {
        "accept" => Ok(protocol::ConflictResolution::Accept),
        "discard" => Ok(protocol::ConflictResolution::Discard),
        other => Err(TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,(format!("unknown conflict resolution token {other:?}")).to_string(),TextSpan::at(1,1))),
    }
}

/// ⚔️ `protocol::ConflictResolution`'s binary ordinal, mirroring
/// `undo_policy_ordinal`'s convention.
pub fn conflict_resolution_ordinal(resolution: protocol::ConflictResolution) -> u8 {
    match resolution {
        protocol::ConflictResolution::Accept => 0,
        protocol::ConflictResolution::Discard => 1,
    }
}

pub fn conflict_resolution_from_ordinal(ordinal: u8) -> Result<protocol::ConflictResolution, protocol::ProtocolError> {
    match ordinal {
        0 => Ok(protocol::ConflictResolution::Accept),
        1 => Ok(protocol::ConflictResolution::Discard),
        other => Err(protocol::ProtocolError::Malformed { what: "conflict resolution ordinal", offset: 0, detail: format!("unknown conflict resolution ordinal {other}") }),
    }
}

/// 📤️ Prints every 2-space-indented `Op::print_op` line for one `apply`/`amend` body,
/// erroring exactly like `print_edit_lines` if any op prints a line containing a newline.
pub async fn print_indented_ops<Op: OpText>(out: &mut String, mutations: &[Op]) -> Result<(), VcsError> {
    for mutation in mutations {
        let printed = mutation.print_op();
        if printed.contains('\n') {
            return Err(VcsError::Serialize("op-text print_op must not contain a newline".into()));
        }
        out.push_str("  ");
        out.push_str(&printed);
        out.push('\n');
    }
    Ok(())
}

/// 📥️ Parses every already-2-space-indented body line of an `apply`/`amend` command as one
/// operation each — the command-level twin of `replay_ops`'s indented-op-line branch.
/// All-or-nothing: on any failure every operation parsed so far retires through its technology's cold disposal.
pub async fn parse_indented_ops<P, Op: OpText + Mutation<P>>(body_lines: &[&str]) -> Result<Vec<Op>, TextError> {
    let mut mutations = Vec::with_capacity(body_lines.len());
    for raw in body_lines {
        let parsed = match raw.starts_with("  ") {
            true => Op::parse_op(raw.trim()),
            false => Err(TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,(format!("expected a 2-space-indented op line, got: {raw:?}")).to_string(),TextSpan::at(1,1))),
        };
        match parsed {
            Ok(operation) => mutations.push(operation),
            Err(error) => {
                retire_operations::<P, Op>(mutations);
                return Err(error);
            }
        }
    }
    Ok(mutations)
}

/// 📥️ Strips exactly one 2-space indent level from every line, joining them back into a
/// standalone command text — used to recurse `parse_command` into a `semantic-undo`/
/// `compensating-action` nested command block.
pub async fn dedent_command_lines(lines: &[&str]) -> Result<String, TextError> {
    let mut out = String::new();
    for raw in lines {
        if !raw.starts_with("  ") {
            return Err(TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,(format!("expected a 2-space-indented nested command line, got: {raw:?}")).to_string(),TextSpan::at(1,1)));
        }
        out.push_str(&raw[2..]);
        out.push('\n');
    }
    Ok(out)
}

/// 📤️ Prints a `ArtifactCommand` as its one-line-per-structural-field header, plus any
/// 2-space-indented operation lines (`Apply`) or a further-indented nested command
/// block (`UndoWithPolicy`'s `semantic_command`) — the maximum-token-efficient textual twin of
/// `encode_command`. `Author::avatar` is never printed, mirroring `OpsAuthor`'s `by=[...]` law.
pub async fn print_command<Op: OpText>(command: &ArtifactCommand<Op>) -> Result<String, VcsError> {
    let mut out = String::new();
    match command {
        ArtifactCommand::Apply { mutations, transaction } => {
            let (transaction, tool) = transaction_text(transaction.as_ref());
            out.push_str(&CommandHeaderLine::Apply { transaction, tool }.print_op());
            out.push('\n');
            print_indented_ops(&mut out, mutations).await?;
        }
        ArtifactCommand::Undo => {
            out.push_str(&CommandHeaderLine::Undo { policy: None }.print_op());
            out.push('\n');
        }
        ArtifactCommand::Redo => {
            out.push_str(&CommandHeaderLine::Redo.print_op());
            out.push('\n');
        }
        ArtifactCommand::UndoWithPolicy { policy, semantic_command } => {
            out.push_str(&CommandHeaderLine::Undo { policy: Some(undo_policy_to_token(*policy).await.to_string()) }.print_op());
            out.push('\n');
            if let Some(nested) = semantic_command {
                let nested_text = Box::pin(print_command(nested)).await?;
                for line in nested_text.lines() {
                    out.push_str("  ");
                    out.push_str(line);
                    out.push('\n');
                }
            }
        }
        ArtifactCommand::CommitCheckpoint { message, authors } => {
            let header = CommandHeaderLine::CommitCheckpoint { message: message.clone(), by: authors.iter().map(OpsAuthor::from).collect() };
            out.push_str(&header.print_op());
            out.push('\n');
        }
        ArtifactCommand::CreateAlternative { name } => {
            out.push_str(&CommandHeaderLine::CreateAlternative { name: name.clone() }.print_op());
            out.push('\n');
        }
        ArtifactCommand::SwitchAlternative { alternative_id } => {
            out.push_str(&CommandHeaderLine::SwitchAlternative { id: alternative_id.clone() }.print_op());
            out.push('\n');
        }
        ArtifactCommand::CheckoutCheckpoint { checkpoint_id } => {
            out.push_str(&CommandHeaderLine::Checkout { id: checkpoint_id.clone() }.print_op());
            out.push('\n');
        }
        ArtifactCommand::ApplyInLane { mutations, lane, transaction } => {
            let (transaction, tool) = transaction_text(transaction.as_ref());
            out.push_str(&CommandHeaderLine::ApplyInLane { lane: history_lane_to_token(*lane).await.to_string(), transaction, tool }.print_op());
            out.push('\n');
            print_indented_ops(&mut out, mutations).await?;
        }
        ArtifactCommand::UndoInLane { lane } => {
            out.push_str(&CommandHeaderLine::UndoInLane { lane: history_lane_to_token(*lane).await.to_string() }.print_op());
            out.push('\n');
        }
        ArtifactCommand::RedoInLane { lane } => {
            out.push_str(&CommandHeaderLine::RedoInLane { lane: history_lane_to_token(*lane).await.to_string() }.print_op());
            out.push('\n');
        }
        ArtifactCommand::IngestRemote { .. } => {
            return Err(VcsError::Serialize("IngestRemote has no text command form".into()));
        }
        ArtifactCommand::PruneDrafts => {
            out.push_str(&CommandHeaderLine::PruneDrafts.print_op());
            out.push('\n');
        }
        ArtifactCommand::SetMergePolicy { policy } => {
            out.push_str(&CommandHeaderLine::SetMergePolicy { policy: merge_policy_to_token(*policy).await.to_string() }.print_op());
            out.push('\n');
        }
        ArtifactCommand::ResolveConflict { conflict_id, resolution } => {
            out.push_str(&CommandHeaderLine::ResolveConflict { conflict_id: conflict_id.clone(), resolution: conflict_resolution_to_token(*resolution).await.to_string() }.print_op());
            out.push('\n');
        }
        ArtifactCommand::Supersede { scope, inputs } => {
            out.push_str(&CommandHeaderLine::Supersede { scope: scope.clone() }.print_op());
            out.push('\n');
            print_supersede_inputs(&mut out, inputs)?;
        }
        ArtifactCommand::CreateAlternativeWithSupersede { name, inputs } => {
            out.push_str(&CommandHeaderLine::CreateAlternativeWithSupersede { name: name.clone() }.print_op());
            out.push('\n');
            print_supersede_inputs(&mut out, inputs)?;
        }
        ArtifactCommand::AppendTransaction { mutations, transaction } => {
            out.push_str(&CommandHeaderLine::AppendTransaction { transaction: transaction.id.clone(), tool: transaction.tool.clone() }.print_op());
            out.push('\n');
            print_indented_ops(&mut out, mutations).await?;
        }
        ArtifactCommand::CommitTransaction { transaction_id } => {
            out.push_str(&CommandHeaderLine::CommitTransaction { id: transaction_id.clone() }.print_op());
            out.push('\n');
        }
        ArtifactCommand::AbortTransaction { transaction_id } => {
            out.push_str(&CommandHeaderLine::AbortTransaction { id: transaction_id.clone() }.print_op());
            out.push('\n');
        }
    }
    Ok(out)
}

/// 📥️ Parses a `print_command`-produced (or hand-authored) command text back into a
/// `ArtifactCommand`. LAW: `parse_command(&print_command(c)?) == Ok(c)` for every `c`.
pub async fn parse_command<P, Op: OpText + Mutation<P>>(text: &str) -> Result<ArtifactCommand<Op>, TextError> {
    let all_lines: Vec<&str> = text.lines().collect();
    let mut header: Option<(u32, &str)> = None;
    let mut body_start = all_lines.len();
    for (index, raw) in all_lines.iter().enumerate() {
        let trimmed = raw.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        header = Some((index as u32 + 1, trimmed));
        body_start = index + 1;
        break;
    }
    let (header_line_no, header_text) = header.ok_or_else(|| TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,("empty command text").to_string(),TextSpan::at(1,1)))?;
    let header_line = CommandHeaderLine::parse_op(header_text).map_err(|error| TextError { span: TextSpan::at(header_line_no, error.span.column), ..error })?;
    let body_lines: Vec<&str> = all_lines[body_start..].iter().filter(|line| !line.trim().is_empty() && !line.trim().starts_with('#')).copied().collect();

    match header_line {
        CommandHeaderLine::Apply { transaction, tool } => {
            let transaction = transaction_from_text(transaction, tool)?;
            let mutations = parse_indented_ops::<P, Op>(&body_lines).await?;
            if mutations.is_empty() {
                return Err(TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,("apply requires at least one operation line").to_string(),TextSpan::at(1,1)));
            }
            Ok(ArtifactCommand::Apply { mutations, transaction })
        }
        CommandHeaderLine::Undo { policy: None } => Ok(ArtifactCommand::Undo),
        CommandHeaderLine::Undo { policy: Some(token) } => {
            let policy = parse_undo_policy_token(&token).await?;
            let semantic_command = if body_lines.is_empty() {
                None
            } else {
                let dedented = dedent_command_lines(&body_lines).await?;
                Some(Box::new(Box::pin(parse_command::<P, Op>(&dedented)).await?))
            };
            Ok(ArtifactCommand::UndoWithPolicy { policy, semantic_command })
        }
        CommandHeaderLine::Redo => Ok(ArtifactCommand::Redo),
        CommandHeaderLine::CommitCheckpoint { message, by } => Ok(ArtifactCommand::CommitCheckpoint { message, authors: by.into_iter().map(Author::from).collect() }),
        CommandHeaderLine::CreateAlternative { name } => Ok(ArtifactCommand::CreateAlternative { name }),
        CommandHeaderLine::SwitchAlternative { id } => Ok(ArtifactCommand::SwitchAlternative { alternative_id: id }),
        CommandHeaderLine::Checkout { id } => Ok(ArtifactCommand::CheckoutCheckpoint { checkpoint_id: id }),
        CommandHeaderLine::PruneDrafts => Ok(ArtifactCommand::PruneDrafts),
        CommandHeaderLine::ApplyInLane { lane, transaction, tool } => {
            let transaction = transaction_from_text(transaction, tool)?;
            let lane = parse_history_lane_token(&lane).await?;
            let mutations = parse_indented_ops::<P, Op>(&body_lines).await?;
            if mutations.is_empty() {
                return Err(TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,("apply-in-lane requires at least one operation line").to_string(),TextSpan::at(1,1)));
            }
            Ok(ArtifactCommand::ApplyInLane { mutations, lane, transaction })
        }
        CommandHeaderLine::UndoInLane { lane } => Ok(ArtifactCommand::UndoInLane { lane: parse_history_lane_token(&lane).await? }),
        CommandHeaderLine::RedoInLane { lane } => Ok(ArtifactCommand::RedoInLane { lane: parse_history_lane_token(&lane).await? }),
        CommandHeaderLine::SetMergePolicy { policy } => Ok(ArtifactCommand::SetMergePolicy { policy: parse_merge_policy_token(&policy).await? }),
        CommandHeaderLine::ResolveConflict { conflict_id, resolution } => Ok(ArtifactCommand::ResolveConflict { conflict_id, resolution: parse_conflict_resolution_token(&resolution).await? }),
        CommandHeaderLine::Supersede { scope } => Ok(ArtifactCommand::Supersede { scope, inputs: parse_supersede_inputs::<P, Op>(&body_lines)? }),
        CommandHeaderLine::CreateAlternativeWithSupersede { name } => Ok(ArtifactCommand::CreateAlternativeWithSupersede { name, inputs: parse_supersede_inputs::<P, Op>(&body_lines)? }),
        CommandHeaderLine::AppendTransaction { transaction, tool } => {
            let mutations = parse_indented_ops::<P, Op>(&body_lines).await?;
            if mutations.is_empty() {
                return Err(TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,("append-transaction requires at least one operation line").to_string(),TextSpan::at(1,1)));
            }
            Ok(ArtifactCommand::AppendTransaction { mutations, transaction: protocol::TransactionRef { id: transaction, tool } })
        }
        CommandHeaderLine::CommitTransaction { id } => Ok(ArtifactCommand::CommitTransaction { transaction_id: id }),
        CommandHeaderLine::AbortTransaction { id } => Ok(ArtifactCommand::AbortTransaction { transaction_id: id }),
    }
}

/// 🧾️ A transaction reference as its two text attributes.
pub fn transaction_text(transaction: Option<&protocol::TransactionRef>) -> (Option<String>, Option<String>) {
    match transaction {
        Some(transaction) => (Some(transaction.id.clone()), Some(transaction.tool.clone())),
        None => (None, None),
    }
}

/// 🧾️ Inverse of [`transaction_text`]: both attributes or neither.
pub fn transaction_from_text(transaction: Option<String>, tool: Option<String>) -> Result<Option<protocol::TransactionRef>, TextError> {
    match (transaction, tool) {
        (Some(id), Some(tool)) => Ok(Some(protocol::TransactionRef { id, tool })),
        (None, None) => Ok(None),
        _ => Err(TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,("a transaction needs both its id and its tool").to_string(),TextSpan::at(1,1))),
    }
}

/// 🧾️ Binary twin of [`transaction_text`]: the id and tool strings after a `0b10` presence bit; every other presence bit
/// is unassigned and refused.
pub fn write_command_transaction(out: &mut Vec<u8>, transaction: Option<&protocol::TransactionRef>) {
    if let Some(transaction) = transaction {
        write_command_str(out, &transaction.id);
        write_command_str(out, &transaction.tool);
    }
}

pub fn read_command_transaction(reader: &mut semio_framework_os_pack::ByteReader<'_>, presence: u8) -> Result<Option<protocol::TransactionRef>, protocol::ProtocolError> {
    if presence & !0b10 != 0 {
        return Err(protocol::ProtocolError::Malformed { what: "command presence", offset: 0, detail: format!("presence bits {presence:#04b} are unassigned beyond the 0b10 transaction bit") });
    }
    if presence & 0b10 == 0 {
        return Ok(None);
    }
    let id = read_command_str(reader)?;
    let tool = read_command_str(reader)?;
    Ok(Some(protocol::TransactionRef { id, tool }))
}

/// 🎯️ Format byte every encoded command starts with — matches `crate::os_dsl::op_rt::OP_BINARY_FORMAT`
/// (B-R6 "one wire convention": `format u8 | ordinal varint | record body`).
pub const COMMAND_BINARY_FORMAT: u8 = 1;

pub fn write_command_str(out: &mut Vec<u8>, s: &str) {
    semio_framework_os_pack::write_varint_u64(out, s.len() as u64);
    out.extend_from_slice(s.as_bytes());
}

pub fn read_command_str(reader: &mut semio_framework_os_pack::ByteReader<'_>) -> Result<String, protocol::ProtocolError> {
    let len = reader.read_varint_u64()?;
    let bytes = reader.read_bytes(len as usize)?;
    std::str::from_utf8(bytes).map(str::to_string).map_err(|error| protocol::ProtocolError::Malformed { what: "command string", offset: 0, detail: error.to_string() })
}

pub fn write_command_ops<Op: OpBinary>(out: &mut Vec<u8>, mutations: &[Op]) -> Result<(), protocol::ProtocolError> {
    semio_framework_os_pack::write_varint_u64(out, mutations.len() as u64);
    for mutation in mutations {
        let bytes = mutation.encode_op()?;
        semio_framework_os_pack::write_varint_u64(out, bytes.len() as u64);
        out.extend_from_slice(&bytes);
    }
    Ok(())
}

/// 🪣️ Reads a command's operation list all-or-nothing: on any failure every operation decoded so far retires through its
/// technology's cold disposal, so a refused list never drops a live operation.
pub fn read_command_ops<P, Op: OpBinary + Mutation<P>>(reader: &mut semio_framework_os_pack::ByteReader<'_>) -> Result<Vec<Op>, CommandDecodeError> {
    let count = reader.read_varint_u64()?;
    let mut mutations = Vec::with_capacity(count.min(reader.remaining() as u64) as usize);
    for index in 0..count as usize {
        match read_command_op(reader) {
            Ok(operation) => mutations.push(operation),
            Err(error) => {
                retire_operations::<P, Op>(mutations);
                return Err(CommandDecodeError::Operation { index, error });
            }
        }
    }
    Ok(mutations)
}

/// 🔹️ One length-prefixed operation of a command.
pub fn read_command_op<Op: OpBinary>(reader: &mut semio_framework_os_pack::ByteReader<'_>) -> Result<Op, protocol::ProtocolError> {
    let len = reader.read_varint_u64()?;
    Op::decode_op(reader.read_bytes(len as usize)?)
}

/// 🧯️ Why a binary [`ArtifactCommand`] did not decode ([`ArtifactCommand::decode_command`]). Decoding is all-or-nothing:
/// every operation decoded before the failure retired through its technology's cold disposal.
#[derive(Clone, Debug, PartialEq)]
pub enum CommandDecodeError {
    /// 📐️ The command's own layout — its format, ordinal, a field, the framing of a list, or bytes past its end.
    Layout(protocol::ProtocolError),
    /// 🧩️ Entry `index` of the command's operation list or supersede inputs does not decode as one operation.
    Operation { index: usize, error: protocol::ProtocolError },
}

impl std::fmt::Display for CommandDecodeError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Layout(error) => write!(formatter, "command layout: {error}"),
            Self::Operation { index, error } => write!(formatter, "command operation {index}: {error}"),
        }
    }
}

impl std::error::Error for CommandDecodeError {
    fn source(&self)->Option<&(dyn std::error::Error+'static)>{match self{Self::Layout(error)|Self::Operation{error,..}=>Some(error)}}
}

impl From<protocol::ProtocolError> for CommandDecodeError {
    fn from(error: protocol::ProtocolError) -> Self {
        Self::Layout(error)
    }
}

impl From<PackError> for CommandDecodeError {
    fn from(error: PackError) -> Self {
        Self::Layout(error.into())
    }
}

impl From<PackRefusal> for CommandDecodeError {
    fn from(error:PackRefusal)->Self{Self::Layout(error.into())}
}

/// 🧊️ Retires operations nobody will apply through their technology's cold disposal.
pub fn retire_operations<P, Op: Mutation<P>>(operations: impl IntoIterator<Item = Op>) {
    for operation in operations {
        Op::retire_cold(operation);
    }
}

/// 🪦️ Retires every operation a command owns — its operation list, its supersede replacements, its nested command.
pub fn retire_command<P, Op: Mutation<P>>(command: ArtifactCommand<Op>) {
    match command {
        ArtifactCommand::Apply { mutations, .. } | ArtifactCommand::ApplyInLane { mutations, .. } | ArtifactCommand::AppendTransaction { mutations, .. } => retire_operations::<P, Op>(mutations),
        ArtifactCommand::Supersede { inputs, .. } | ArtifactCommand::CreateAlternativeWithSupersede { inputs, .. } => retire_supersede_inputs::<P, Op>(inputs),
        ArtifactCommand::UndoWithPolicy { semantic_command: Some(nested), .. } => retire_command::<P, Op>(*nested),
        ArtifactCommand::UndoWithPolicy { semantic_command: None, .. }
        | ArtifactCommand::Undo
        | ArtifactCommand::Redo
        | ArtifactCommand::UndoInLane { .. }
        | ArtifactCommand::RedoInLane { .. }
        | ArtifactCommand::CommitCheckpoint { .. }
        | ArtifactCommand::CreateAlternative { .. }
        | ArtifactCommand::SwitchAlternative { .. }
        | ArtifactCommand::CheckoutCheckpoint { .. }
        | ArtifactCommand::IngestRemote { .. }
        | ArtifactCommand::PruneDrafts
        | ArtifactCommand::SetMergePolicy { .. }
        | ArtifactCommand::ResolveConflict { .. }
        | ArtifactCommand::CommitTransaction { .. }
        | ArtifactCommand::AbortTransaction { .. } => {}
    }
}

mod operation_envelope_serde {
    /// 🧵️ `ToValue` twin of the former `serialize` — identical byte framing, wire shape is
    /// `DslValue::Array` of `DslValue::Number` (one per byte), matching `Vec<u8>`'s own
    /// `ToValue` codec exactly (so this is equivalent to `ToValue::to_value(&bytes)`).
    pub fn to_value(envelope: &protocol::MutationEnvelope) -> semio_framework_value::DslValue {
        let mut bytes = Vec::new();
        protocol::encode_envelope(envelope, &mut bytes);
        semio_framework_value::ToValue::to_value(&bytes)
    }

    /// 🧵️ `FromValue` twin of `to_value` above.
    pub fn from_value(value: semio_framework_value::DslValue) -> Result<protocol::MutationEnvelope, semio_framework_value::ValueError> {
        let bytes: Vec<u8> = semio_framework_value::FromValue::from_value(value)?;
        let mut pos = 0;
        protocol::decode_envelope(&bytes, &mut pos).map_err(|error| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, error.to_string()))
    }
}

/// 🫗️ Retires every typed replacement of a refused supersession's `inputs`, none of which was encoded.
pub fn retire_supersede_inputs<P, Mutation>(inputs: Vec<SupersedeInput<Mutation>>)
where
    Mutation: protocol::Mutation<P>,
{
    for replacement in inputs.into_iter().filter_map(|input| input.replacement) {
        protocol::Mutation::<P>::retire_cold(replacement);
    }
}


#[cfg(test)]
#[global_allocator]
static TEST_ALLOCATION_OBSERVER: semio_framework_trace::HeapWitness = semio_framework_trace::HeapWitness;

/// 🧊️ Cold-retires scratch operations (a rebased or discarded inverse): an operation may own a
/// fail-closed root, so a displaced copy never reaches `Drop` owning it.
pub fn retire_scratch_operations<P, Mutation>(operations: impl IntoIterator<Item = Mutation>)
where
    Mutation: protocol::Mutation<P>,
{
    for operation in operations {
        protocol::Mutation::<P>::retire_cold(operation);
    }
}

pub fn retire_scratch_edits<P, Mutation>(edits: impl IntoIterator<Item = protocol::Edit<Mutation>>)
where
    Mutation: protocol::Mutation<P>,
{
    for edit in edits {
        retire_scratch_operations::<P, Mutation>(edit.forwards);
        retire_scratch_operations::<P, Mutation>(edit.inverse);
    }
}

