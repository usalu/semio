//! 🎰 Model-agnostic LLM token-sampling engine: logits in, processor pipeline, constrained
//! distributions, deterministic seeded selection — plus a diffusion/continuous-noise solver module.

// #region 🔖Ids
/// 🧩 Index of one vocabulary entry. `u32` keeps candidate/mask arithmetic cheap even for
/// million-token sharded vocabularies while staying far below any real model's vocab size.
/// `#[repr(transparent)]` lets [`cast_u32_slice_to_token_ids`] hand out a typed view over a raw
/// index buffer without copying.
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct TokenId(pub u32);

impl TokenId {
    /// 🧩 Wraps a raw vocabulary index.
    pub const fn new(id: u32) -> Self {
        Self(id)
    }

    /// 🧩 Raw vocabulary index.
    pub const fn get(self) -> u32 {
        self.0
    }
}

impl core::fmt::Display for TokenId {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// 🧩 Identifies one generation request/sequence across batch reorders — never a slot index, so
/// continuous batching can shuffle rows without breaking RNG-stream or state addressing.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct SequenceId(pub u64);

impl SequenceId {
    /// 🧩 Wraps a raw sequence identifier.
    pub const fn new(id: u64) -> Self {
        Self(id)
    }

    /// 🧩 Raw sequence identifier.
    pub const fn get(self) -> u64 {
        self.0
    }
}

impl core::fmt::Display for SequenceId {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// 🧩 Zero-based count of tokens generated so far for one sequence (excludes the prompt).
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct StepIndex(pub u32);

impl StepIndex {
    /// 🧩 Wraps a raw step count.
    pub const fn new(step: u32) -> Self {
        Self(step)
    }

    /// 🧩 Raw step count.
    pub const fn get(self) -> u32 {
        self.0
    }

    /// 🧩 Next step, or `None` on overflow (caller-observable rather than a silent wraparound).
    pub const fn checked_next(self) -> Option<Self> {
        match self.0.checked_add(1) {
            Some(next) => Some(Self(next)),
            None => None,
        }
    }
}

impl core::fmt::Display for StepIndex {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.0)
    }
}
// #endregion 🔖Ids

// #region 🔖Errors
/// 🚨 Every way a sampling step can fail to produce a token. Kept flat (no nested error types)
/// so callers can match exhaustively without chasing a `source()` chain for common cases.
#[derive(Clone, PartialEq, Debug)]
pub enum SamplingError {
    /// 🚨 A configuration value failed validation (`field` names the offending knob).
    InvalidConfig { field: &'static str, reason: &'static str },
    /// 🚨 Logits length does not match the configured vocabulary size.
    VocabMismatch { expected: usize, actual: usize },
    /// 🚨 Logits contained NaN/Inf and the active [`SanitizePolicy`] is [`SanitizePolicy::Error`].
    NonFiniteLogits { index: usize },
    /// 🚨 Every token was masked or truncated away with no fallback available.
    EmptyDistribution,
    /// 🚨 A constraint reports no valid continuation exists (dead automaton state).
    ConstraintDead { constraint: &'static str },
    /// 🚨 A configured resource cap (§ [`SamplingLimits`]) was exceeded.
    LimitExceeded { limit: &'static str },
    /// 🚨 EBNF grammar text failed to parse at the given byte offset.
    GrammarParse { offset: usize, reason: &'static str },
    /// 🚨 Regex pattern text failed to parse at the given byte offset.
    RegexParse { offset: usize, reason: &'static str },
    /// 🚨 An automaton (DFA/NFA/Earley chart) exceeded its state/size budget mid-construction.
    AutomatonBudget { budget: &'static str },
    /// 🚨 Serialized config/state carries an unsupported or mismatched format version.
    SerializationVersion { expected: u32, actual: u32 },
    /// 🚨 A config/state fingerprint did not match the fingerprint recorded at serialization time.
    FingerprintMismatch,
    /// 🚨 Serialized data is truncated, malformed, or fails an integrity check.
    Corrupted { reason: &'static str },
    /// 🚨 A sharded-vocabulary collective operation failed (timeout, rank mismatch, ...).
    Collective { reason: &'static str },
    /// 🚨 A user-supplied callback (rerank, similarity, denoiser, ...) reported failure.
    Callback { reason: &'static str },
    /// 🚨 Generation was cancelled via an external cancellation signal.
    Cancelled,
}

impl core::fmt::Display for SamplingError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::InvalidConfig { field, reason } => write!(f, "invalid config field `{field}`: {reason}"),
            Self::VocabMismatch { expected, actual } => {
                write!(f, "logits length {actual} does not match vocabulary size {expected}")
            }
            Self::NonFiniteLogits { index } => write!(f, "non-finite logit at index {index}"),
            Self::EmptyDistribution => write!(f, "no valid token remains after masking/truncation"),
            Self::ConstraintDead { constraint } => write!(f, "constraint `{constraint}` has no valid continuation"),
            Self::LimitExceeded { limit } => write!(f, "resource limit exceeded: {limit}"),
            Self::GrammarParse { offset, reason } => write!(f, "grammar parse error at byte {offset}: {reason}"),
            Self::RegexParse { offset, reason } => write!(f, "regex parse error at byte {offset}: {reason}"),
            Self::AutomatonBudget { budget } => write!(f, "automaton exceeded budget: {budget}"),
            Self::SerializationVersion { expected, actual } => {
                write!(f, "serialization version mismatch: expected {expected}, found {actual}")
            }
            Self::FingerprintMismatch => write!(f, "config/state fingerprint mismatch"),
            Self::Corrupted { reason } => write!(f, "corrupted serialized data: {reason}"),
            Self::Collective { reason } => write!(f, "collective operation failed: {reason}"),
            Self::Callback { reason } => write!(f, "callback failed: {reason}"),
            Self::Cancelled => write!(f, "generation cancelled"),
        }
    }
}

impl std::error::Error for SamplingError {}

/// 🚨 Why a sequence stopped generating.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum FinishReason {
    /// 🚨 An end-of-sequence token was selected.
    EosToken,
    /// 🚨 A single stop token (outside the EOS set) was selected.
    StopToken,
    /// 🚨 A configured stop text sequence matched, by index into `StopSpec::sequences`.
    StopSequence { index: usize },
    /// 🚨 The per-sequence maximum generated-token count was reached.
    MaxTokens,
    /// 🚨 The maximum wall-clock duration was reached.
    MaxTimeMs,
    /// 🚨 A constraint (grammar/JSON/schema) reports completion.
    ConstraintComplete,
    /// 🚨 An external cancellation signal was observed.
    Cancelled,
    /// 🚨 No valid token existed and no fallback resolved (only reachable in permissive mode).
    Dead,
    /// 🚨 A user-supplied stop callback returned `true`.
    Callback,
}

/// 🚨 Whether a failed step surfaces an error or resolves through the fallback ladder.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum ErrorMode {
    /// 🚨 Any [`SamplingError`] is returned to the caller.
    Strict,
    /// 🚨 Recoverable failures resolve via [`resolve_fallback`] instead of erroring.
    #[default]
    Permissive,
}

/// 🚨 Which rung of the fallback ladder resolved an otherwise-empty distribution.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum FallbackAction {
    /// 🚨 A configured forced token was substituted.
    ForcedToken,
    /// 🚨 The vocabulary's (first) EOS token was substituted.
    Eos,
    /// 🚨 The pre-mask, pre-truncation argmax of the raw logits was substituted.
    ArgmaxRaw,
    /// 🚨 No rung resolved; the caller's [`ErrorMode`] decides between `Dead` and an error.
    Error,
}

/// 🚨 Walks the fallback ladder `ForcedToken -> Eos -> ArgmaxRaw -> Error` for an empty
/// distribution, in that fixed priority order — forced tokens are an explicit user instruction so
/// they always win, EOS is the next-safest "stop cleanly" choice, and only then does raw argmax
/// (which may violate active masks) get used as a last resort before giving up.
pub fn resolve_fallback(forced: Option<TokenId>, eos: Option<TokenId>, argmax_raw: Option<TokenId>) -> (FallbackAction, Option<TokenId>) {
    if let Some(token) = forced {
        return (FallbackAction::ForcedToken, Some(token));
    }
    if let Some(token) = eos {
        return (FallbackAction::Eos, Some(token));
    }
    if let Some(token) = argmax_raw {
        return (FallbackAction::ArgmaxRaw, Some(token));
    }
    (FallbackAction::Error, None)
}
// #endregion 🔖Errors

// #region 🔖Limits
/// 🛡️ Resource caps applied to untrusted configuration, so a hostile or malformed request cannot
/// force unbounded allocation or CPU time (bounded grammars, bounded automata, bounded beams).
#[derive(Clone, PartialEq, Debug)]
pub struct SamplingLimits {
    pub max_stop_sequences: usize,
    pub max_stop_bytes: usize,
    pub max_grammar_bytes: usize,
    pub max_automaton_states: usize,
    pub max_dfa_cache_entries: usize,
    pub max_beam_width: usize,
    pub max_candidates: usize,
    pub max_ngram_order: usize,
    pub max_forced_tokens: usize,
    pub max_schedule_pieces: usize,
    pub max_json_depth: usize,
}

impl Default for SamplingLimits {
    fn default() -> Self {
        Self {
            max_stop_sequences: 64,
            max_stop_bytes: 4_096,
            max_grammar_bytes: 65_536,
            max_automaton_states: 100_000,
            max_dfa_cache_entries: 16_384,
            max_beam_width: 64,
            max_candidates: 256,
            max_ngram_order: 8,
            max_forced_tokens: 4_096,
            max_schedule_pieces: 256,
            max_json_depth: 64,
        }
    }
}

impl SamplingLimits {
    /// 🛡️ Rejects a manifestly broken limits set (zero caps would make the engine unusable).
    pub fn validate(&self) -> Result<(), SamplingError> {
        let fields: [(&'static str, usize); 11] = [
            ("max_stop_sequences", self.max_stop_sequences),
            ("max_stop_bytes", self.max_stop_bytes),
            ("max_grammar_bytes", self.max_grammar_bytes),
            ("max_automaton_states", self.max_automaton_states),
            ("max_dfa_cache_entries", self.max_dfa_cache_entries),
            ("max_beam_width", self.max_beam_width),
            ("max_candidates", self.max_candidates),
            ("max_ngram_order", self.max_ngram_order),
            ("max_forced_tokens", self.max_forced_tokens),
            ("max_schedule_pieces", self.max_schedule_pieces),
            ("max_json_depth", self.max_json_depth),
        ];
        for (field, value) in fields {
            if value == 0 {
                return Err(SamplingError::InvalidConfig { field, reason: "limit must be non-zero" });
            }
        }
        Ok(())
    }
}
// #endregion 🔖Limits

// #region 🔖Text
// #region 🔖Json
/// 📜 A parsed JSON value. Objects preserve insertion order via `Vec<(String, JsonValue)>` rather
/// than a `HashMap` — configs are small, order-preserving round-trips are nicer to diff, and this
/// keeps the whole module dependency-free (no hashing DoS surface to reason about either).
#[derive(Clone, PartialEq, Debug)]
pub enum JsonValue {
    Null,
    Bool(bool),
    Num(f64),
    Str(String),
    Array(Vec<JsonValue>),
    Object(Vec<(String, JsonValue)>),
}

impl JsonValue {
    /// 📜 Looks up a key in an object value; `None` for any other variant or a missing key.
    pub fn get(&self, key: &str) -> Option<&JsonValue> {
        match self {
            Self::Object(entries) => entries.iter().find(|(k, _)| k == key).map(|(_, v)| v),
            _ => None,
        }
    }

    pub fn as_str(&self) -> Option<&str> {
        match self {
            Self::Str(s) => Some(s),
            _ => None,
        }
    }

    pub fn as_f64(&self) -> Option<f64> {
        match self {
            Self::Num(n) => Some(*n),
            _ => None,
        }
    }

    pub fn as_bool(&self) -> Option<bool> {
        match self {
            Self::Bool(b) => Some(*b),
            _ => None,
        }
    }

    pub fn as_array(&self) -> Option<&[JsonValue]> {
        match self {
            Self::Array(items) => Some(items),
            _ => None,
        }
    }
}

/// 📜 Recursive-descent JSON parser over `&str`, depth-capped by `max_depth` so a maliciously
/// nested document (`[[[[...`) cannot blow the call stack — this is the same nesting bound that
/// protects the JSON-Schema/grammar constraint machinery later in the file.
pub fn parse_json(text: &str, max_depth: usize) -> Result<JsonValue, SamplingError> {
    let bytes = text.as_bytes();
    let mut pos = 0usize;
    let value = parse_json_value(bytes, &mut pos, 0, max_depth)?;
    skip_json_whitespace(bytes, &mut pos);
    if pos != bytes.len() {
        return Err(SamplingError::Corrupted { reason: "trailing data after JSON value" });
    }
    Ok(value)
}

fn skip_json_whitespace(bytes: &[u8], pos: &mut usize) {
    while *pos < bytes.len() && matches!(bytes[*pos], b' ' | b'\t' | b'\n' | b'\r') {
        *pos += 1;
    }
}

fn parse_json_value(bytes: &[u8], pos: &mut usize, depth: usize, max_depth: usize) -> Result<JsonValue, SamplingError> {
    if depth > max_depth {
        return Err(SamplingError::LimitExceeded { limit: "max_json_depth" });
    }
    skip_json_whitespace(bytes, pos);
    let Some(&byte) = bytes.get(*pos) else {
        return Err(SamplingError::Corrupted { reason: "unexpected end of JSON input" });
    };
    match byte {
        b'n' => parse_json_literal(bytes, pos, "null", JsonValue::Null),
        b't' => parse_json_literal(bytes, pos, "true", JsonValue::Bool(true)),
        b'f' => parse_json_literal(bytes, pos, "false", JsonValue::Bool(false)),
        b'"' => parse_json_string(bytes, pos).map(JsonValue::Str),
        b'[' => parse_json_array(bytes, pos, depth, max_depth),
        b'{' => parse_json_object(bytes, pos, depth, max_depth),
        b'-' | b'0'..=b'9' => parse_json_number(bytes, pos),
        _ => Err(SamplingError::Corrupted { reason: "unexpected byte in JSON input" }),
    }
}

fn parse_json_literal(bytes: &[u8], pos: &mut usize, literal: &str, value: JsonValue) -> Result<JsonValue, SamplingError> {
    let lit = literal.as_bytes();
    if bytes[*pos..].starts_with(lit) {
        *pos += lit.len();
        Ok(value)
    } else {
        Err(SamplingError::Corrupted { reason: "invalid JSON literal" })
    }
}

fn parse_json_string(bytes: &[u8], pos: &mut usize) -> Result<String, SamplingError> {
    debug_assert_eq!(bytes[*pos], b'"');
    *pos += 1;
    let mut out = String::new();
    loop {
        let &byte = bytes.get(*pos).ok_or(SamplingError::Corrupted { reason: "unterminated JSON string" })?;
        *pos += 1;
        match byte {
            b'"' => return Ok(out),
            b'\\' => {
                let &esc = bytes.get(*pos).ok_or(SamplingError::Corrupted { reason: "unterminated JSON escape" })?;
                *pos += 1;
                match esc {
                    b'"' => out.push('"'),
                    b'\\' => out.push('\\'),
                    b'/' => out.push('/'),
                    b'n' => out.push('\n'),
                    b't' => out.push('\t'),
                    b'r' => out.push('\r'),
                    b'b' => out.push('\u{8}'),
                    b'f' => out.push('\u{c}'),
                    b'u' => {
                        let code = parse_json_unicode_escape(bytes, pos)?;
                        out.push(char::from_u32(code).unwrap_or('\u{FFFD}'));
                    }
                    _ => return Err(SamplingError::Corrupted { reason: "invalid JSON escape" }),
                }
            }
            _ => {
                // 📜 Re-decode the UTF-8 codepoint starting at this byte instead of pushing raw bytes.
                let start = *pos - 1;
                let ch_len = utf8_sequence_len(byte).ok_or(SamplingError::Corrupted { reason: "invalid UTF-8 in JSON string" })?;
                let end = start + ch_len;
                let slice = bytes.get(start..end).ok_or(SamplingError::Corrupted { reason: "truncated UTF-8 in JSON string" })?;
                let s = core::str::from_utf8(slice).map_err(|_| SamplingError::Corrupted { reason: "invalid UTF-8 in JSON string" })?;
                out.push_str(s);
                *pos = end;
            }
        }
    }
}

fn parse_json_unicode_escape(bytes: &[u8], pos: &mut usize) -> Result<u32, SamplingError> {
    let hex = bytes.get(*pos..*pos + 4).ok_or(SamplingError::Corrupted { reason: "truncated unicode escape" })?;
    let s = core::str::from_utf8(hex).map_err(|_| SamplingError::Corrupted { reason: "invalid unicode escape" })?;
    let code = u32::from_str_radix(s, 16).map_err(|_| SamplingError::Corrupted { reason: "invalid unicode escape" })?;
    *pos += 4;
    Ok(code)
}

fn parse_json_number(bytes: &[u8], pos: &mut usize) -> Result<JsonValue, SamplingError> {
    let start = *pos;
    if bytes.get(*pos) == Some(&b'-') {
        *pos += 1;
    }
    while bytes.get(*pos).is_some_and(u8::is_ascii_digit) {
        *pos += 1;
    }
    if bytes.get(*pos) == Some(&b'.') {
        *pos += 1;
        while bytes.get(*pos).is_some_and(u8::is_ascii_digit) {
            *pos += 1;
        }
    }
    if matches!(bytes.get(*pos), Some(b'e' | b'E')) {
        *pos += 1;
        if matches!(bytes.get(*pos), Some(b'+' | b'-')) {
            *pos += 1;
        }
        while bytes.get(*pos).is_some_and(u8::is_ascii_digit) {
            *pos += 1;
        }
    }
    let text = core::str::from_utf8(&bytes[start..*pos]).expect("ASCII number slice is valid UTF-8");
    text.parse::<f64>().map(JsonValue::Num).map_err(|_| SamplingError::Corrupted { reason: "invalid JSON number" })
}

fn parse_json_array(bytes: &[u8], pos: &mut usize, depth: usize, max_depth: usize) -> Result<JsonValue, SamplingError> {
    debug_assert_eq!(bytes[*pos], b'[');
    *pos += 1;
    let mut items = Vec::new();
    skip_json_whitespace(bytes, pos);
    if bytes.get(*pos) == Some(&b']') {
        *pos += 1;
        return Ok(JsonValue::Array(items));
    }
    loop {
        items.push(parse_json_value(bytes, pos, depth + 1, max_depth)?);
        skip_json_whitespace(bytes, pos);
        match bytes.get(*pos) {
            Some(b',') => {
                *pos += 1;
            }
            Some(b']') => {
                *pos += 1;
                return Ok(JsonValue::Array(items));
            }
            _ => return Err(SamplingError::Corrupted { reason: "expected ',' or ']' in JSON array" }),
        }
    }
}

fn parse_json_object(bytes: &[u8], pos: &mut usize, depth: usize, max_depth: usize) -> Result<JsonValue, SamplingError> {
    debug_assert_eq!(bytes[*pos], b'{');
    *pos += 1;
    let mut entries = Vec::new();
    skip_json_whitespace(bytes, pos);
    if bytes.get(*pos) == Some(&b'}') {
        *pos += 1;
        return Ok(JsonValue::Object(entries));
    }
    loop {
        skip_json_whitespace(bytes, pos);
        if bytes.get(*pos) != Some(&b'"') {
            return Err(SamplingError::Corrupted { reason: "expected string key in JSON object" });
        }
        let key = parse_json_string(bytes, pos)?;
        skip_json_whitespace(bytes, pos);
        if bytes.get(*pos) != Some(&b':') {
            return Err(SamplingError::Corrupted { reason: "expected ':' in JSON object" });
        }
        *pos += 1;
        let value = parse_json_value(bytes, pos, depth + 1, max_depth)?;
        entries.push((key, value));
        skip_json_whitespace(bytes, pos);
        match bytes.get(*pos) {
            Some(b',') => {
                *pos += 1;
            }
            Some(b'}') => {
                *pos += 1;
                return Ok(JsonValue::Object(entries));
            }
            _ => return Err(SamplingError::Corrupted { reason: "expected ',' or '}' in JSON object" }),
        }
    }
}

/// 📜 Writes a [`JsonValue`] in compact form with correct string escaping and round-trippable
/// float formatting (integral floats print without a trailing `.0`-free exponent surprise).
pub fn write_json(value: &JsonValue) -> String {
    let mut out = String::new();
    write_json_into(value, &mut out);
    out
}

fn write_json_into(value: &JsonValue, out: &mut String) {
    match value {
        JsonValue::Null => out.push_str("null"),
        JsonValue::Bool(b) => out.push_str(if *b { "true" } else { "false" }),
        JsonValue::Num(n) => write_json_number(*n, out),
        JsonValue::Str(s) => write_json_string(s, out),
        JsonValue::Array(items) => {
            out.push('[');
            for (i, item) in items.iter().enumerate() {
                if i > 0 {
                    out.push(',');
                }
                write_json_into(item, out);
            }
            out.push(']');
        }
        JsonValue::Object(entries) => {
            out.push('{');
            for (i, (key, item)) in entries.iter().enumerate() {
                if i > 0 {
                    out.push(',');
                }
                write_json_string(key, out);
                out.push(':');
                write_json_into(item, out);
            }
            out.push('}');
        }
    }
}

fn write_json_number(n: f64, out: &mut String) {
    if n.fract() == 0.0 && n.abs() < 1e15 {
        out.push_str(&format!("{}", n as i64));
    } else {
        out.push_str(&format!("{n}"));
    }
}

fn write_json_string(s: &str, out: &mut String) {
    out.push('"');
    for ch in s.chars() {
        match ch {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\t' => out.push_str("\\t"),
            '\r' => out.push_str("\\r"),
            c if (c as u32) < 0x20 => out.push_str(&format!("\\u{:04x}", c as u32)),
            c => out.push(c),
        }
    }
    out.push('"');
}
// #endregion 🔖Json

// #region 🔖Utf8
/// 📜 How much of a byte sequence forms valid UTF-8, for incremental decoding of streamed token
/// bytes that may end mid-codepoint.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Utf8Status {
    /// 📜 The whole slice is valid, complete UTF-8.
    Complete,
    /// 📜 The slice ends with a valid-so-far partial codepoint needing `more` additional bytes.
    Partial { more: usize },
    /// 📜 The slice contains a byte sequence that can never become valid UTF-8.
    Invalid,
}

/// 📜 Length of the UTF-8 sequence a leading byte starts, or `None` if `byte` cannot lead one.
pub fn utf8_sequence_len(byte: u8) -> Option<usize> {
    match byte {
        0x00..=0x7F => Some(1),
        0xC2..=0xDF => Some(2),
        0xE0..=0xEF => Some(3),
        0xF0..=0xF4 => Some(4),
        _ => None,
    }
}

/// 📜 Classifies the tail of `bytes` as complete, valid-partial, or invalid UTF-8. Used to decide
/// how many trailing bytes of a just-emitted token must be held back until more bytes arrive.
pub fn utf8_status(bytes: &[u8]) -> Utf8Status {
    if bytes.is_empty() {
        return Utf8Status::Complete;
    }
    // 📜 Walk backward from the end to find the start of the last (possibly incomplete) codepoint.
    let mut start = bytes.len() - 1;
    let mut lead_len = None;
    for _ in 0..4 {
        if let Some(len) = utf8_sequence_len(bytes[start]) {
            lead_len = Some(len);
            break;
        }
        if start == 0 {
            return Utf8Status::Invalid;
        }
        start -= 1;
    }
    let Some(len) = lead_len else {
        return Utf8Status::Invalid;
    };
    let have = bytes.len() - start;
    if have > len {
        return Utf8Status::Invalid;
    }
    if have == len {
        return match core::str::from_utf8(&bytes[start..]) {
            Ok(_) => Utf8Status::Complete,
            Err(_) => Utf8Status::Invalid,
        };
    }
    Utf8Status::Partial { more: len - have }
}
// #endregion 🔖Utf8
// #endregion 🔖Text

// #region 🔖Numerics
/// 📐 How non-finite input logits are handled before any processor runs.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum SanitizePolicy {
    /// 📐 Any NaN or +Inf logit is a hard [`SamplingError::NonFiniteLogits`].
    Error,
    /// 📐 NaN and -Inf both collapse to `f32::NEG_INFINITY` (effectively masked); +Inf is rejected
    /// (an infinitely-preferred token needs an explicit decision, not a silent uniform pick).
    #[default]
    NegInfNan,
    /// 📐 NaN collapses to `f32::NEG_INFINITY`; +Inf clamps to `f32::MAX`.
    ClampInf,
}

/// 📐 Applies `policy` to `logits` in place. Returns the count of entries that were altered, for
/// diagnostics.
pub fn sanitize_logits(logits: &mut [f32], policy: SanitizePolicy) -> Result<usize, SamplingError> {
    let mut altered = 0usize;
    for (i, l) in logits.iter_mut().enumerate() {
        if l.is_nan() {
            match policy {
                SanitizePolicy::Error => return Err(SamplingError::NonFiniteLogits { index: i }),
                SanitizePolicy::NegInfNan | SanitizePolicy::ClampInf => {
                    *l = f32::NEG_INFINITY;
                    altered += 1;
                }
            }
        } else if *l == f32::INFINITY {
            match policy {
                SanitizePolicy::Error | SanitizePolicy::NegInfNan => return Err(SamplingError::NonFiniteLogits { index: i }),
                SanitizePolicy::ClampInf => {
                    *l = f32::MAX;
                    altered += 1;
                }
            }
        } else if *l == f32::NEG_INFINITY {
            // 📐 Negative infinity is always a valid "hard masked" representation; never altered.
        }
    }
    Ok(altered)
}

/// 📐 Accumulation precision for softmax/entropy computation.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum Accum {
    F32,
    #[default]
    F64,
}

/// 📐 Compensated (Kahan-Neumaier) running sum — keeps `f64` summation accurate over long live
/// candidate lists where naive summation would otherwise lose low-order bits.
#[derive(Clone, Copy, Default, Debug)]
pub struct KahanSum {
    sum: f64,
    correction: f64,
}

impl KahanSum {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, value: f64) {
        let t = self.sum + value;
        if self.sum.abs() >= value.abs() {
            self.correction += (self.sum - t) + value;
        } else {
            self.correction += (value - t) + self.sum;
        }
        self.sum = t;
    }

    pub fn value(&self) -> f64 {
        self.sum + self.correction
    }
}

/// 📐 Numerically stable softmax over `logits[live]`, writing normalized probabilities into
/// `probs[..live.len()]`. Uses max-subtraction so large logits never overflow `exp`, and
/// [`KahanSum`] under [`Accum::F64`] so the sum stays close to `1.0` even for wide vocabularies.
pub fn softmax_live(logits: &[f32], live: &[u32], probs: &mut [f32], accum: Accum) -> f64 {
    debug_assert_eq!(live.len(), probs.len());
    if live.is_empty() {
        return 0.0;
    }
    let max_logit = live.iter().map(|&i| logits[i as usize]).fold(f32::NEG_INFINITY, f32::max);
    match accum {
        Accum::F32 => {
            let mut sum = 0.0f32;
            for (slot, &i) in live.iter().enumerate() {
                let e = (logits[i as usize] - max_logit).exp();
                probs[slot] = e;
                sum += e;
            }
            let inv = if sum > 0.0 { 1.0 / sum } else { 0.0 };
            for p in probs.iter_mut() {
                *p *= inv;
            }
            sum as f64
        }
        Accum::F64 => {
            let mut sum = KahanSum::new();
            let mut exps = vec![0.0f64; live.len()];
            for (slot, &i) in live.iter().enumerate() {
                let e = ((logits[i as usize] - max_logit) as f64).exp();
                exps[slot] = e;
                sum.add(e);
            }
            let total = sum.value();
            let inv = if total > 0.0 { 1.0 / total } else { 0.0 };
            for (slot, p) in probs.iter_mut().enumerate() {
                *p = (exps[slot] * inv) as f32;
            }
            total
        }
    }
}

/// 📐 `log(sum(exp(values)))` computed via max-subtraction, safe for arbitrarily negative/large
/// inputs (including `-inf` entries, which contribute nothing).
pub fn logsumexp_f64(values: &[f64]) -> f64 {
    let max = values.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    if !max.is_finite() {
        return max;
    }
    let mut sum = KahanSum::new();
    for &v in values {
        sum.add((v - max).exp());
    }
    max + sum.value().ln()
}

/// 📐 Shannon entropy in nats of a probability vector (`0 ln 0 := 0` by convention).
pub fn entropy_nats(probs: &[f32]) -> f64 {
    let mut sum = KahanSum::new();
    for &p in probs {
        let p = p as f64;
        if p > 0.0 {
            sum.add(-p * p.ln());
        }
    }
    sum.value()
}

/// 📐 `exp(entropy)`: the "effective" number of roughly-equally-likely candidates, matching the
/// perplexity of the live distribution.
pub fn effective_candidate_count(probs: &[f32]) -> f64 {
    entropy_nats(probs).exp()
}

/// 📐 Total probability mass removed by comparing a pre-truncation and post-truncation live-prob
/// vector that share the same normalization base (`1.0 - post_sum` when `post` sums the surviving
/// portion of a `pre` distribution that summed to `1.0`).
pub fn truncation_mass(pre_kept_sum: f64) -> f64 {
    (1.0 - pre_kept_sum).max(0.0)
}

/// 📐 Health flags for a live distribution, computed once per step for diagnostics and to decide
/// whether the fallback ladder must engage.
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct DistributionHealth {
    pub live_count: usize,
    pub prob_sum: f64,
    pub is_degenerate: bool,
}

impl DistributionHealth {
    pub fn assess(live_count: usize, prob_sum: f64) -> Self {
        Self { live_count, prob_sum, is_degenerate: live_count == 0 || !(prob_sum > 0.0) }
    }
}

/// 📐 Reorders `live[..]` so the top `k` (by `logits[live[i]]` descending, ties by ascending token
/// id for determinism) occupy `live[..k]`, unordered within that prefix — a Hoare-partition
/// quickselect, `O(n)` average instead of an `O(n log n)` full sort. `scratch` must have length
/// `>= live.len()` and its contents are overwritten.
pub fn partial_select_top_k(logits: &[f32], live: &mut [u32], k: usize, scratch: &mut [f32]) {
    let n = live.len();
    if k >= n {
        return;
    }
    debug_assert!(scratch.len() >= n);
    let key = |token: u32| -> (f32, core::cmp::Reverse<u32>) { (logits[token as usize], core::cmp::Reverse(token)) };
    // 📐 Selection over a byte-key derived from (logit desc, token asc) via index-sort scratch.
    for (slot, &t) in live.iter().enumerate() {
        scratch[slot] = logits[t as usize];
    }
    quickselect_desc(live, scratch, 0, n - 1, k, &key);
}

fn quickselect_desc(live: &mut [u32], scratch: &mut [f32], mut lo: usize, mut hi: usize, k: usize, key: &impl Fn(u32) -> (f32, core::cmp::Reverse<u32>)) {
    loop {
        if lo >= hi {
            return;
        }
        let pivot = key(live[(lo + hi) / 2]);
        let mut i = lo;
        let mut j = hi;
        loop {
            while key(live[i]) > pivot {
                i += 1;
            }
            while key(live[j]) < pivot {
                j -= 1;
            }
            if i >= j {
                break;
            }
            live.swap(i, j);
            scratch.swap(i, j);
            i += 1;
            if j == 0 {
                break;
            }
            j -= 1;
        }
        if k <= j {
            hi = j;
        } else if k > j + 1 {
            lo = j + 1;
        } else {
            return;
        }
    }
}

/// 📐 Smallest index `i` such that `cdf[i] >= u`, via binary search over a nondecreasing CDF.
/// Falls back to the last index when floating-point rounding leaves `u` fractionally above the
/// final (nominally `1.0`) entry.
pub fn cdf_binary_search(cdf: &[f64], u: f64) -> usize {
    if cdf.is_empty() {
        return 0;
    }
    let mut lo = 0usize;
    let mut hi = cdf.len() - 1;
    while lo < hi {
        let mid = lo + (hi - lo) / 2;
        if cdf[mid] < u {
            lo = mid + 1;
        } else {
            hi = mid;
        }
    }
    lo
}
// #endregion 🔖Numerics

// #region 🔖Bitset
/// 🎭 A dense bitset over `0..len` token indices, `u64`-word packed. The single shared
/// representation for hard masks (constraints, allow/forbid lists, forced-token exclusivity) —
/// every mask operation the pipeline needs is `O(vocab / 64)`.
#[derive(Clone, PartialEq, Debug)]
pub struct TokenBitset {
    words: Vec<u64>,
    len: usize,
}

impl TokenBitset {
    /// 🎭 All-zero (empty) bitset over `len` tokens.
    pub fn new_empty(len: usize) -> Self {
        Self { words: vec![0u64; len.div_ceil(64)], len }
    }

    /// 🎭 All-one (full) bitset over `len` tokens.
    pub fn new_full(len: usize) -> Self {
        let mut set = Self::new_empty(len);
        set.fill();
        set
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty_capacity(&self) -> bool {
        self.len == 0
    }

    #[inline]
    pub fn get(&self, token: TokenId) -> bool {
        let idx = token.get() as usize;
        debug_assert!(idx < self.len);
        (self.words[idx / 64] >> (idx % 64)) & 1 != 0
    }

    #[inline]
    pub fn set(&mut self, token: TokenId, value: bool) {
        let idx = token.get() as usize;
        debug_assert!(idx < self.len);
        let mask = 1u64 << (idx % 64);
        if value {
            self.words[idx / 64] |= mask;
        } else {
            self.words[idx / 64] &= !mask;
        }
    }

    /// 🎭 Sets every bit `0..len` (trailing bits beyond `len` in the final word stay zero).
    pub fn fill(&mut self) {
        let full_words = self.len / 64;
        self.words[..full_words].fill(u64::MAX);
        let rem = self.len % 64;
        if rem > 0 {
            self.words[full_words] = (1u64 << rem) - 1;
        }
    }

    pub fn clear_all(&mut self) {
        self.words.fill(0);
    }

    /// 🎭 In-place `self &= other`.
    pub fn and_with(&mut self, other: &TokenBitset) {
        debug_assert_eq!(self.len, other.len);
        for (a, b) in self.words.iter_mut().zip(other.words.iter()) {
            *a &= *b;
        }
    }

    /// 🎭 In-place `self |= other`.
    pub fn or_with(&mut self, other: &TokenBitset) {
        debug_assert_eq!(self.len, other.len);
        for (a, b) in self.words.iter_mut().zip(other.words.iter()) {
            *a |= *b;
        }
    }

    /// 🎭 In-place `self &= !other`.
    pub fn and_not_with(&mut self, other: &TokenBitset) {
        debug_assert_eq!(self.len, other.len);
        for (a, b) in self.words.iter_mut().zip(other.words.iter()) {
            *a &= !*b;
        }
    }

    pub fn count_ones(&self) -> usize {
        self.words.iter().map(|w| w.count_ones() as usize).sum()
    }

    pub fn is_all_zero(&self) -> bool {
        self.words.iter().all(|&w| w == 0)
    }

    /// 🎭 Lowest set bit, word-skipping past all-zero words.
    pub fn first_set(&self) -> Option<TokenId> {
        for (word_idx, &word) in self.words.iter().enumerate() {
            if word != 0 {
                let bit = word.trailing_zeros() as usize;
                return Some(TokenId::new((word_idx * 64 + bit) as u32));
            }
        }
        None
    }

    /// 🎭 Iterates set bits in ascending order, skipping whole zero words at a time.
    pub fn iter_ones(&self) -> impl Iterator<Item = TokenId> + '_ {
        self.words.iter().enumerate().flat_map(|(word_idx, &word)| {
            let mut remaining = word;
            core::iter::from_fn(move || {
                if remaining == 0 {
                    return None;
                }
                let bit = remaining.trailing_zeros();
                remaining &= remaining - 1;
                Some(TokenId::new((word_idx * 64 + bit as usize) as u32))
            })
        })
    }
}
// #endregion 🔖Bitset

// #region 🔖Rng
/// 🎲 Which sub-stream of randomness a draw belongs to, so unrelated concerns (selection noise vs.
/// speculative-decoding acceptance vs. diffusion noise) never share bits even within one sequence.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum StreamPurpose {
    Selection = 0,
    Gumbel = 1,
    Noise = 2,
    Speculative = 3,
    Beam = 4,
    Diffusion = 5,
}

/// 🎲 Identifies one independent random stream by the ids that produced it — never by batch slot,
/// so a continuous-batching reorder never changes which bits a sequence draws.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct StreamKey {
    pub request: u64,
    pub sequence: u64,
    pub beam: u32,
    pub candidate: u32,
    pub purpose: StreamPurpose,
}

fn mix64(x: u64) -> u64 {
    mathematical_random::SplitMix64::new(x).next_u64()
}

fn stream_seed(key: StreamKey) -> u64 {
    let mut acc = mix64(key.request);
    acc = mix64(acc ^ mix64(key.sequence));
    acc = mix64(acc ^ mix64(key.beam as u64));
    acc = mix64(acc ^ mix64(key.candidate as u64));
    acc = mix64(acc ^ mix64(key.purpose as u64));
    acc
}

/// 🎲 Which concrete generator produced a [`RngSnapshot`], so `restore` can reject a snapshot
/// meant for the other kind instead of silently reinterpreting its words.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum RngKind {
    Counter,
    Xoshiro,
}

/// 🎲 Portable capture of a generator's internal state, text-serializable for [`SequenceState`]
/// checkpoints.
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct RngSnapshot {
    pub kind: RngKind,
    pub words: [u64; 4],
}

impl RngSnapshot {
    /// 🎲 Compact `kind:hex:hex:hex:hex` text form.
    pub fn to_text(&self) -> String {
        let kind = match self.kind {
            RngKind::Counter => "counter",
            RngKind::Xoshiro => "xoshiro",
        };
        format!("{kind}:{:016x}:{:016x}:{:016x}:{:016x}", self.words[0], self.words[1], self.words[2], self.words[3])
    }

    /// 🎲 Inverse of [`RngSnapshot::to_text`].
    pub fn from_text(text: &str) -> Result<Self, SamplingError> {
        let mut parts = text.split(':');
        let kind = match parts.next() {
            Some("counter") => RngKind::Counter,
            Some("xoshiro") => RngKind::Xoshiro,
            _ => return Err(SamplingError::Corrupted { reason: "unknown rng snapshot kind" }),
        };
        let mut words = [0u64; 4];
        for w in words.iter_mut() {
            let part = parts.next().ok_or(SamplingError::Corrupted { reason: "truncated rng snapshot" })?;
            *w = u64::from_str_radix(part, 16).map_err(|_| SamplingError::Corrupted { reason: "invalid rng snapshot hex" })?;
        }
        if parts.next().is_some() {
            return Err(SamplingError::Corrupted { reason: "trailing data in rng snapshot" });
        }
        Ok(Self { kind, words })
    }
}

/// 🎲 Object-safe source of randomness handed to samplers/warpers/search algorithms. Every
/// implementation must be splittable into independent child streams keyed by [`StreamKey`] alone
/// (never by call order), which is what keeps continuous-batching reorders and speculative
/// verification bit-reproducible.
pub trait RandomSource {
    fn next_u64(&mut self) -> u64;
    /// 🎲 Derives an independent child stream from `(self, key)` — order-independent across calls.
    fn split(&self, key: StreamKey) -> Box<dyn RandomSource>;
    fn snapshot(&self) -> RngSnapshot;
    fn restore(&mut self, snapshot: &RngSnapshot) -> Result<(), SamplingError>;

    /// 🎲 Uniform `f64` in `[0, 1)`.
    fn next_f64(&mut self) -> f64 {
        (self.next_u64() >> 11) as f64 * (1.0 / (1u64 << 53) as f64)
    }

    /// 🎲 Uniform `f64` in `(0, 1]` — safe as the argument to `ln()`, unlike `next_f64`.
    fn next_f64_open01(&mut self) -> f64 {
        (((self.next_u64() >> 11) + 1) as f64) * (1.0 / (1u64 << 53) as f64)
    }

    /// 🎲 Uniform `u64` in `[lo, hi)` via rejection sampling (no modulo bias).
    fn next_range(&mut self, lo: u64, hi: u64) -> u64 {
        debug_assert!(hi >= lo, "next_range: hi must be >= lo");
        let range = hi - lo;
        if range == 0 {
            return lo;
        }
        let limit = u64::MAX - (u64::MAX % range);
        loop {
            let x = self.next_u64();
            if x < limit {
                return lo + x % range;
            }
        }
    }

    /// 🎲 Standard Gumbel(0, 1) draw, `-ln(-ln(u))` for `u` in `(0, 1]`.
    fn gumbel(&mut self) -> f64 {
        let u = self.next_f64_open01();
        -(-u.ln()).ln()
    }
}

/// 🎲 Default splittable [`RandomSource`]: a counter-based generator (double [`mix64`] of
/// `key ^ mix64(counter)`, Philox-lite) chosen over a stepped generator specifically because
/// splitting never advances or depends on the parent's step count — two sequences split from the
/// same parent at different times still get independent, order-irrelevant streams.
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct CounterRng {
    key: u64,
    ctr: u64,
}

impl CounterRng {
    /// 🎲 A root stream from a plain seed, with no [`StreamKey`] semantics (tests, standalone use).
    pub fn from_seed(seed: u64) -> Self {
        Self { key: mix64(seed), ctr: 0 }
    }

    /// 🎲 A root stream combining a request-level seed with a full [`StreamKey`] in one step.
    pub fn from_root(root_seed: u64, key: StreamKey) -> Self {
        Self { key: mix64(mix64(root_seed) ^ stream_seed(key)), ctr: 0 }
    }
}

impl RandomSource for CounterRng {
    fn next_u64(&mut self) -> u64 {
        let ctr = self.ctr;
        self.ctr = self.ctr.wrapping_add(1);
        mix64(self.key ^ mix64(ctr))
    }

    fn split(&self, key: StreamKey) -> Box<dyn RandomSource> {
        Box::new(Self { key: mix64(self.key ^ stream_seed(key)), ctr: 0 })
    }

    fn snapshot(&self) -> RngSnapshot {
        RngSnapshot { kind: RngKind::Counter, words: [self.key, self.ctr, 0, 0] }
    }

    fn restore(&mut self, snapshot: &RngSnapshot) -> Result<(), SamplingError> {
        if snapshot.kind != RngKind::Counter {
            return Err(SamplingError::Corrupted { reason: "rng snapshot kind mismatch: expected counter" });
        }
        self.key = snapshot.words[0];
        self.ctr = snapshot.words[1];
        Ok(())
    }
}

/// 🎲 [`RandomSource`] adapter over [`mathematical_random::Rng`] (xoshiro256**), for callers who
/// want that generator's statistical profile instead of the default counter-based stream.
pub struct XoshiroSource(mathematical_random::Rng);

impl XoshiroSource {
    pub fn from_seed(seed: u64) -> Self {
        Self(mathematical_random::Rng::from_seed(seed))
    }
}

impl RandomSource for XoshiroSource {
    fn next_u64(&mut self) -> u64 {
        self.0.next_u64()
    }

    fn split(&self, key: StreamKey) -> Box<dyn RandomSource> {
        let state = self.0.state();
        let seed = mix64(state[0] ^ state[1] ^ stream_seed(key));
        Box::new(Self::from_seed(seed))
    }

    fn snapshot(&self) -> RngSnapshot {
        RngSnapshot { kind: RngKind::Xoshiro, words: self.0.state() }
    }

    fn restore(&mut self, snapshot: &RngSnapshot) -> Result<(), SamplingError> {
        if snapshot.kind != RngKind::Xoshiro {
            return Err(SamplingError::Corrupted { reason: "rng snapshot kind mismatch: expected xoshiro" });
        }
        self.0 = mathematical_random::Rng::from_state(snapshot.words);
        Ok(())
    }
}
// #endregion 🔖Rng

// #region 🔖Vocabulary
/// 📖 Static facts about the token space a [`SamplingConfig`] samples over.
#[derive(Clone, PartialEq, Debug)]
pub struct Vocabulary {
    pub size: usize,
    pub eos: Vec<TokenId>,
    pub bos: Option<TokenId>,
    pub pad: Option<TokenId>,
    pub unk: Option<TokenId>,
    pub special: TokenBitset,
}

impl Vocabulary {
    /// 📖 A vocabulary of `size` tokens with no special tokens configured.
    pub fn new(size: usize) -> Self {
        Self { size, eos: Vec::new(), bos: None, pad: None, unk: None, special: TokenBitset::new_empty(size) }
    }

    pub fn with_eos(mut self, eos: Vec<TokenId>) -> Self {
        self.eos = eos;
        self
    }

    pub fn with_bos(mut self, bos: TokenId) -> Self {
        self.bos = Some(bos);
        self
    }

    pub fn with_pad(mut self, pad: TokenId) -> Self {
        self.pad = Some(pad);
        self
    }

    pub fn with_unk(mut self, unk: TokenId) -> Self {
        self.unk = Some(unk);
        self
    }

    /// 📖 Marks `tokens` as special (suppressible via `ProcessorSpec::SuppressSpecial`).
    pub fn with_special(mut self, tokens: &[TokenId]) -> Self {
        for &token in tokens {
            self.special.set(token, true);
        }
        self
    }

    pub fn is_eos(&self, token: TokenId) -> bool {
        self.eos.contains(&token)
    }

    /// 📖 Errors unless `len` matches this vocabulary's declared size.
    pub fn validate_logits_len(&self, len: usize) -> Result<(), SamplingError> {
        if len != self.size {
            Err(SamplingError::VocabMismatch { expected: self.size, actual: len })
        } else {
            Ok(())
        }
    }
}

/// 📖 Maps [`TokenId`]s to their surface-form bytes, for constraints and stop matching that
/// operate on generated text rather than raw ids.
pub trait TokenTextAdapter {
    fn vocab_size(&self) -> usize;
    /// 📖 Raw (possibly partial-UTF-8) bytes of one token; `None` for byte-less special tokens.
    fn token_bytes(&self, token: TokenId) -> Option<&[u8]>;
    /// 📖 Stable hash of the whole token table, used to key automaton-state×token caches so a
    /// swapped tokenizer can never silently reuse another tokenizer's cached transitions.
    fn fingerprint(&self) -> u64;
}

/// 📖 Reference [`TokenTextAdapter`] over a plain `&[&[u8]]` token table.
pub struct SliceTextAdapter<'a> {
    tokens: &'a [&'a [u8]],
}

impl<'a> SliceTextAdapter<'a> {
    pub fn new(tokens: &'a [&'a [u8]]) -> Self {
        Self { tokens }
    }
}

impl TokenTextAdapter for SliceTextAdapter<'_> {
    fn vocab_size(&self) -> usize {
        self.tokens.len()
    }

    fn token_bytes(&self, token: TokenId) -> Option<&[u8]> {
        self.tokens.get(token.get() as usize).copied()
    }

    fn fingerprint(&self) -> u64 {
        // 📖 FNV-1a over every token's bytes with a separator byte between entries so `["ab","c"]`
        // and `["a","bc"]` never collide.
        let mut hash = 0xcbf2_9ce4_8422_2325u64;
        for token in self.tokens {
            for &b in *token {
                hash = (hash ^ b as u64).wrapping_mul(0x0000_0100_0000_01b3);
            }
            hash = (hash ^ 0xFF).wrapping_mul(0x0000_0100_0000_01b3);
        }
        hash
    }
}
// #endregion 🔖Vocabulary

// #region 🔖Schedules
/// 📅 What a [`Schedule`] is evaluated against at one sampling step.
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct ScheduleInput {
    pub step: StepIndex,
    pub generated_len: usize,
    pub last_entropy: Option<f64>,
}

/// 📅 A parameter value that may vary over the course of generation. Every warper/penalty knob
/// that the feature tree calls out as schedulable is `Schedule`-typed in [`ProcessorSpec`].
#[derive(Clone, PartialEq, Debug)]
pub enum Schedule {
    Constant(f64),
    Linear { from: f64, to: f64, over_steps: u32 },
    Exponential { from: f64, to: f64, over_steps: u32 },
    Cosine { from: f64, to: f64, over_steps: u32 },
    /// 📅 Step-indexed breakpoints; the value holds at the most recent breakpoint `<= step`.
    Piecewise(Vec<(StepIndex, f64)>),
    /// 📅 One value per generated-token position, clamped to the last entry past its length.
    ByPosition(Vec<f64>),
    EntropyScaled { base: f64, gain: f64, min: f64, max: f64 },
    /// 📅 Escape hatch for host-defined logic; not text-serializable (see [`Schedule::to_json`]).
    Callback(fn(ScheduleInput) -> f64),
}

impl Schedule {
    /// 📅 Evaluates the schedule at `input`.
    pub fn eval(&self, input: ScheduleInput) -> f64 {
        match self {
            Self::Constant(v) => *v,
            Self::Linear { from, to, over_steps } => {
                let t = schedule_progress(input.step, *over_steps);
                from + (to - from) * t
            }
            Self::Exponential { from, to, over_steps } => {
                let t = schedule_progress(input.step, *over_steps);
                if *from > 0.0 && *to > 0.0 {
                    from * (to / from).powf(t)
                } else {
                    from + (to - from) * t
                }
            }
            Self::Cosine { from, to, over_steps } => {
                let t = schedule_progress(input.step, *over_steps);
                let cos_t = 0.5 * (1.0 - (core::f64::consts::PI * t).cos());
                from + (to - from) * cos_t
            }
            Self::Piecewise(pieces) => {
                let mut value = pieces.first().map(|(_, v)| *v).unwrap_or(0.0);
                for (step, v) in pieces {
                    if step.get() <= input.step.get() {
                        value = *v;
                    } else {
                        break;
                    }
                }
                value
            }
            Self::ByPosition(values) => {
                if values.is_empty() {
                    0.0
                } else {
                    values[input.generated_len.min(values.len() - 1)]
                }
            }
            Self::EntropyScaled { base, gain, min, max } => {
                let entropy = input.last_entropy.unwrap_or(0.0);
                (base + gain * entropy).clamp(*min, *max)
            }
            Self::Callback(f) => f(input),
        }
    }

    /// 📅 Structured form for config serialization; `Callback` encodes as a marker that
    /// deliberately fails to round-trip (see [`Schedule::from_json`]).
    pub fn to_json(&self) -> JsonValue {
        let obj = |pairs: Vec<(&str, JsonValue)>| JsonValue::Object(pairs.into_iter().map(|(k, v)| (k.to_string(), v)).collect());
        match self {
            Self::Constant(v) => obj(vec![("kind", JsonValue::Str("constant".into())), ("value", JsonValue::Num(*v))]),
            Self::Linear { from, to, over_steps } => obj(vec![
                ("kind", JsonValue::Str("linear".into())),
                ("from", JsonValue::Num(*from)),
                ("to", JsonValue::Num(*to)),
                ("over_steps", JsonValue::Num(*over_steps as f64)),
            ]),
            Self::Exponential { from, to, over_steps } => obj(vec![
                ("kind", JsonValue::Str("exponential".into())),
                ("from", JsonValue::Num(*from)),
                ("to", JsonValue::Num(*to)),
                ("over_steps", JsonValue::Num(*over_steps as f64)),
            ]),
            Self::Cosine { from, to, over_steps } => obj(vec![
                ("kind", JsonValue::Str("cosine".into())),
                ("from", JsonValue::Num(*from)),
                ("to", JsonValue::Num(*to)),
                ("over_steps", JsonValue::Num(*over_steps as f64)),
            ]),
            Self::Piecewise(pieces) => obj(vec![
                ("kind", JsonValue::Str("piecewise".into())),
                (
                    "pieces",
                    JsonValue::Array(pieces.iter().map(|(s, v)| JsonValue::Array(vec![JsonValue::Num(s.get() as f64), JsonValue::Num(*v)])).collect()),
                ),
            ]),
            Self::ByPosition(values) => obj(vec![
                ("kind", JsonValue::Str("by_position".into())),
                ("values", JsonValue::Array(values.iter().map(|v| JsonValue::Num(*v)).collect())),
            ]),
            Self::EntropyScaled { base, gain, min, max } => obj(vec![
                ("kind", JsonValue::Str("entropy_scaled".into())),
                ("base", JsonValue::Num(*base)),
                ("gain", JsonValue::Num(*gain)),
                ("min", JsonValue::Num(*min)),
                ("max", JsonValue::Num(*max)),
            ]),
            Self::Callback(_) => obj(vec![("kind", JsonValue::Str("callback".into()))]),
        }
    }

    /// 📅 Inverse of [`Schedule::to_json`]; rejects `"callback"` since function pointers are not
    /// recoverable from serialized data.
    pub fn from_json(value: &JsonValue) -> Result<Self, SamplingError> {
        let kind = value.get("kind").and_then(JsonValue::as_str).ok_or(SamplingError::Corrupted { reason: "schedule missing kind" })?;
        let num = |key: &'static str| value.get(key).and_then(JsonValue::as_f64).ok_or(SamplingError::Corrupted { reason: "schedule missing numeric field" });
        match kind {
            "constant" => Ok(Self::Constant(num("value")?)),
            "linear" => Ok(Self::Linear { from: num("from")?, to: num("to")?, over_steps: num("over_steps")? as u32 }),
            "exponential" => Ok(Self::Exponential { from: num("from")?, to: num("to")?, over_steps: num("over_steps")? as u32 }),
            "cosine" => Ok(Self::Cosine { from: num("from")?, to: num("to")?, over_steps: num("over_steps")? as u32 }),
            "piecewise" => {
                let pieces = value.get("pieces").and_then(JsonValue::as_array).ok_or(SamplingError::Corrupted { reason: "piecewise schedule missing pieces" })?;
                let mut out = Vec::with_capacity(pieces.len());
                for piece in pieces {
                    let pair = piece.as_array().ok_or(SamplingError::Corrupted { reason: "piecewise entry must be a pair" })?;
                    let (Some(step), Some(v)) = (pair.first().and_then(JsonValue::as_f64), pair.get(1).and_then(JsonValue::as_f64)) else {
                        return Err(SamplingError::Corrupted { reason: "piecewise entry must be [step, value]" });
                    };
                    out.push((StepIndex::new(step as u32), v));
                }
                Ok(Self::Piecewise(out))
            }
            "by_position" => {
                let values = value.get("values").and_then(JsonValue::as_array).ok_or(SamplingError::Corrupted { reason: "by_position schedule missing values" })?;
                let out = values.iter().map(|v| v.as_f64().ok_or(SamplingError::Corrupted { reason: "by_position value must be numeric" })).collect::<Result<Vec<_>, _>>()?;
                Ok(Self::ByPosition(out))
            }
            "entropy_scaled" => Ok(Self::EntropyScaled { base: num("base")?, gain: num("gain")?, min: num("min")?, max: num("max")? }),
            "callback" => Err(SamplingError::Corrupted { reason: "callback schedules cannot be deserialized" }),
            _ => Err(SamplingError::Corrupted { reason: "unknown schedule kind" }),
        }
    }
}

fn schedule_progress(step: StepIndex, over_steps: u32) -> f64 {
    if over_steps == 0 {
        1.0
    } else {
        (step.get() as f64 / over_steps as f64).min(1.0)
    }
}
// #endregion 🔖Schedules

// #region 🔖Config
/// ⚙️ Deterministic tie-breaking policy for greedy selection and every truncation warper's
/// boundary decision.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum TieBreak {
    #[default]
    LowestTokenId,
    HighestTokenId,
    FirstSeen,
}

/// ⚙️ Which concrete algorithm backs [`SamplingMethod::Multinomial`].
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum MultinomialStrategy {
    #[default]
    CdfBinarySearch,
    LinearScan,
    Alias,
}

/// ⚙️ How a token is finally chosen from the (already-warped) live distribution.
#[derive(Clone, PartialEq, Debug)]
pub enum SamplingMethod {
    Greedy { tie_break: TieBreak },
    Multinomial { strategy: MultinomialStrategy },
    GumbelMax,
    GumbelTopK { k: usize },
}

impl Default for SamplingMethod {
    fn default() -> Self {
        Self::Greedy { tie_break: TieBreak::default() }
    }
}

/// ⚙️ Whether a penalty/bias considers the prompt, the generated continuation, or both, when
/// scanning a sequence's history.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum PenaltyScope {
    #[default]
    PromptAndGenerated,
    GeneratedOnly,
    PromptOnly,
}

/// ⚙️ Which Mirostat control law an adaptive processor runs.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum MirostatVersion {
    V1,
    V2,
}

/// ⚙️ One entry in the ordered processor pipeline. Every schedulable numeric parameter is
/// [`Schedule`]-typed so config presets and per-step dynamics share one representation.
#[derive(Clone, PartialEq, Debug)]
pub enum ProcessorSpec {
    Temperature { value: Schedule },
    DynamicTemperature { base: Schedule, entropy_gain: f64, min: f64, max: f64 },
    TopK { k: Schedule, min_keep: usize },
    TopP { p: Schedule, min_keep: usize },
    MinP { p: Schedule, min_keep: usize },
    Typical { mass: Schedule, min_keep: usize },
    LocallyTypical { mass: Schedule, min_keep: usize },
    TailFree { z: Schedule, min_keep: usize },
    Epsilon { cutoff: Schedule, min_keep: usize },
    Eta { cutoff: Schedule, min_keep: usize },
    TopA { power: Schedule, min_keep: usize },
    RankTruncation { max_rank: usize },
    AdaptiveTruncation { target_entropy: Option<f64>, target_effective_count: Option<f64> },
    RepetitionPenalty { penalty: f32, scope: PenaltyScope },
    PresencePenalty { penalty: f32, scope: PenaltyScope },
    FrequencyPenalty { penalty: f32, scope: PenaltyScope },
    DecayingPenalty { penalty: f32, window: usize, half_life: f64, scope: PenaltyScope },
    TokenClassPenalty { classes: Vec<u16>, factors: Vec<f32> },
    NoRepeatNgram { n: usize },
    PhrasePenalty { phrases: Vec<Vec<TokenId>>, penalty: f32 },
    LogitBiasSparse { entries: Vec<(TokenId, f32)> },
    LogitBiasDense { values: Vec<f32> },
    AllowTokens { tokens: Vec<TokenId> },
    ForbidTokens { tokens: Vec<TokenId> },
    SuppressSpecial,
    BadWords { phrases: Vec<Vec<TokenId>> },
    SequenceEncouragement { phrases: Vec<Vec<TokenId>>, bonus: f32 },
    Mirostat { version: MirostatVersion, target_surprise: f64, learning_rate: f64 },
    EntropyPid { target: f64, kp: f64, ki: f64, kd: f64 },
    RepetitionController { window: usize, threshold: f64, boost: f64 },
    ConfidenceController { low_entropy: f64, high_entropy: f64, low_temp: f64, high_temp: f64 },
}

/// ⚙️ One hand-rolled-constraint source. Compiled into a [`Constraint`] impl by the engine.
#[derive(Clone, PartialEq, Debug)]
pub enum ConstraintSpec {
    Regex(String),
    Trie(Vec<Vec<TokenId>>),
    MustInclude(Vec<Vec<TokenId>>),
    JsonMode,
    Ebnf(String),
    JsonSchema(JsonValue),
}

/// ⚙️ How matched stop text is reflected in the returned generation.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum StopTextMode {
    #[default]
    Include,
    Exclude,
    Separate,
}

/// ⚙️ Stop-condition configuration: token-level, text-sequence, and time-based stops.
#[derive(Clone, PartialEq, Debug, Default)]
pub struct StopSpec {
    pub tokens: Vec<TokenId>,
    pub sequences: Vec<Vec<u8>>,
    pub mode: StopTextMode,
    pub max_time_ms: Option<u64>,
}

/// ⚙️ Forced-token configuration: an initial BOS/prefix and/or exact tokens at fixed positions.
#[derive(Clone, PartialEq, Debug, Default)]
pub struct ForcedSpec {
    pub bos: Option<TokenId>,
    pub prefix: Vec<TokenId>,
    pub at_position: Vec<(StepIndex, TokenId)>,
}

/// ⚙️ How many alternative log-probabilities to report alongside the selected token.
#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub struct LogprobsSpec {
    pub top_n: usize,
    pub include_pre_truncation: bool,
}

/// ⚙️ Which optional per-step diagnostics to collect (all zero-cost when disabled).
#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub struct DiagnosticsSpec {
    pub enabled: bool,
    pub timing: bool,
}

/// ⚙️ Everything needed to sample one sequence, independent of any particular model or backend.
#[derive(Clone, PartialEq, Debug)]
pub struct SamplingConfig {
    pub method: SamplingMethod,
    pub processors: Vec<ProcessorSpec>,
    pub error_mode: ErrorMode,
    pub sanitize: SanitizePolicy,
    pub accum: Accum,
    pub seed: u64,
    pub candidate_count: usize,
    pub min_tokens: usize,
    pub max_tokens: usize,
    pub forced: ForcedSpec,
    pub stops: StopSpec,
    pub constraints: Vec<ConstraintSpec>,
    pub logprobs: LogprobsSpec,
    pub limits: SamplingLimits,
    pub diagnostics: DiagnosticsSpec,
}

impl Default for SamplingConfig {
    fn default() -> Self {
        Self {
            method: SamplingMethod::default(),
            processors: Vec::new(),
            error_mode: ErrorMode::default(),
            sanitize: SanitizePolicy::default(),
            accum: Accum::default(),
            seed: 0,
            candidate_count: 1,
            min_tokens: 0,
            max_tokens: 4_096,
            forced: ForcedSpec::default(),
            stops: StopSpec::default(),
            constraints: Vec::new(),
            logprobs: LogprobsSpec::default(),
            limits: SamplingLimits::default(),
            diagnostics: DiagnosticsSpec::default(),
        }
    }
}

impl SamplingConfig {
    /// ⚙️ Full structural validation: resource limits, cross-field consistency, and per-processor
    /// sanity — run before generation starts and again whenever an override merges into a base.
    pub fn validate(&self) -> Result<(), SamplingError> {
        self.limits.validate()?;
        if self.candidate_count == 0 {
            return Err(SamplingError::InvalidConfig { field: "candidate_count", reason: "must be >= 1" });
        }
        if self.candidate_count > self.limits.max_candidates {
            return Err(SamplingError::LimitExceeded { limit: "max_candidates" });
        }
        if self.min_tokens > self.max_tokens {
            return Err(SamplingError::InvalidConfig { field: "min_tokens", reason: "must not exceed max_tokens" });
        }
        if self.stops.sequences.len() > self.limits.max_stop_sequences {
            return Err(SamplingError::LimitExceeded { limit: "max_stop_sequences" });
        }
        let stop_bytes: usize = self.stops.sequences.iter().map(Vec::len).sum();
        if stop_bytes > self.limits.max_stop_bytes {
            return Err(SamplingError::LimitExceeded { limit: "max_stop_bytes" });
        }
        if self.forced.at_position.len() > self.limits.max_forced_tokens {
            return Err(SamplingError::LimitExceeded { limit: "max_forced_tokens" });
        }
        if let SamplingMethod::GumbelTopK { k } = self.method {
            if k == 0 {
                return Err(SamplingError::InvalidConfig { field: "method.k", reason: "gumbel top-k requires k >= 1" });
            }
        }
        for processor in &self.processors {
            validate_processor_spec(processor, &self.limits)?;
        }
        for constraint in &self.constraints {
            if let ConstraintSpec::Ebnf(text) | ConstraintSpec::Regex(text) = constraint {
                if text.len() > self.limits.max_grammar_bytes {
                    return Err(SamplingError::LimitExceeded { limit: "max_grammar_bytes" });
                }
            }
        }
        Ok(())
    }

    /// ⚙️ FNV-1a fingerprint of the canonical JSON form, stamped into serialized sequence state so
    /// a resumed sequence can detect it was checkpointed under a different configuration.
    pub fn fingerprint(&self) -> u64 {
        let text = write_json(&self.to_json());
        let mut hash = 0xcbf2_9ce4_8422_2325u64;
        for &b in text.as_bytes() {
            hash = (hash ^ b as u64).wrapping_mul(0x0000_0100_0000_01b3);
        }
        hash
    }

    /// ⚙️ Greedy decoding, no penalties or truncation — the deterministic baseline preset.
    pub fn precise() -> Self {
        Self { method: SamplingMethod::Greedy { tie_break: TieBreak::LowestTokenId }, ..Self::default() }
    }

    /// ⚙️ Temperature 0.7, top-p 0.9, min-p 0.05, mild repetition penalty — a general-purpose
    /// preset balancing coherence and variety.
    pub fn balanced() -> Self {
        Self {
            method: SamplingMethod::Multinomial { strategy: MultinomialStrategy::CdfBinarySearch },
            processors: vec![
                ProcessorSpec::Temperature { value: Schedule::Constant(0.7) },
                ProcessorSpec::TopP { p: Schedule::Constant(0.9), min_keep: 1 },
                ProcessorSpec::MinP { p: Schedule::Constant(0.05), min_keep: 1 },
                ProcessorSpec::RepetitionPenalty { penalty: 1.1, scope: PenaltyScope::GeneratedOnly },
            ],
            ..Self::default()
        }
    }

    /// ⚙️ Higher temperature and wider top-p/top-k for more diverse output.
    pub fn creative() -> Self {
        Self {
            method: SamplingMethod::Multinomial { strategy: MultinomialStrategy::CdfBinarySearch },
            processors: vec![
                ProcessorSpec::Temperature { value: Schedule::Constant(1.0) },
                ProcessorSpec::TopK { k: Schedule::Constant(100.0), min_keep: 1 },
                ProcessorSpec::TopP { p: Schedule::Constant(0.95), min_keep: 1 },
            ],
            ..Self::default()
        }
    }

    /// ⚙️ Fixed seed, greedy, tiny token budget — for reproducible tests, not production use.
    pub fn deterministic_test() -> Self {
        Self { method: SamplingMethod::Greedy { tie_break: TieBreak::LowestTokenId }, seed: 42, max_tokens: 64, ..Self::default() }
    }

    /// ⚙️ Structured JSON form (versioned: top-level `"version": 1`).
    pub fn to_json(&self) -> JsonValue {
        JsonValue::Object(vec![
            ("version".into(), JsonValue::Num(1.0)),
            ("method".into(), sampling_method_to_json(&self.method)),
            ("processors".into(), JsonValue::Array(self.processors.iter().map(processor_spec_to_json).collect())),
            ("error_mode".into(), JsonValue::Str(match self.error_mode { ErrorMode::Strict => "strict".into(), ErrorMode::Permissive => "permissive".into() })),
            ("seed".into(), JsonValue::Num(self.seed as f64)),
            ("candidate_count".into(), JsonValue::Num(self.candidate_count as f64)),
            ("min_tokens".into(), JsonValue::Num(self.min_tokens as f64)),
            ("max_tokens".into(), JsonValue::Num(self.max_tokens as f64)),
        ])
    }

    /// ⚙️ Parses the `"version": 1` JSON form back into a config, tolerating unknown top-level
    /// fields (forward compatibility) but rejecting a version it does not recognize.
    pub fn from_json(value: &JsonValue) -> Result<Self, SamplingError> {
        let version = value.get("version").and_then(JsonValue::as_f64).ok_or(SamplingError::Corrupted { reason: "config missing version" })?;
        if version as u32 != 1 {
            return Err(SamplingError::SerializationVersion { expected: 1, actual: version as u32 });
        }
        let method = value.get("method").ok_or(SamplingError::Corrupted { reason: "config missing method" }).and_then(sampling_method_from_json)?;
        let processors = value
            .get("processors")
            .and_then(JsonValue::as_array)
            .ok_or(SamplingError::Corrupted { reason: "config missing processors" })?
            .iter()
            .map(processor_spec_from_json)
            .collect::<Result<Vec<_>, _>>()?;
        let error_mode = match value.get("error_mode").and_then(JsonValue::as_str) {
            Some("strict") => ErrorMode::Strict,
            Some("permissive") | None => ErrorMode::Permissive,
            Some(_) => return Err(SamplingError::Corrupted { reason: "unknown error_mode" }),
        };
        let num = |key: &'static str, default: f64| value.get(key).and_then(JsonValue::as_f64).unwrap_or(default);
        Ok(Self {
            method,
            processors,
            error_mode,
            seed: num("seed", 0.0) as u64,
            candidate_count: num("candidate_count", 1.0) as usize,
            min_tokens: num("min_tokens", 0.0) as usize,
            max_tokens: num("max_tokens", 4_096.0) as usize,
            ..Self::default()
        })
    }
}

fn validate_processor_spec(spec: &ProcessorSpec, limits: &SamplingLimits) -> Result<(), SamplingError> {
    match spec {
        ProcessorSpec::NoRepeatNgram { n } => {
            if *n == 0 || *n > limits.max_ngram_order {
                return Err(SamplingError::LimitExceeded { limit: "max_ngram_order" });
            }
        }
        ProcessorSpec::TokenClassPenalty { classes, factors } => {
            if classes.len() != factors.len() {
                return Err(SamplingError::InvalidConfig { field: "token_class_penalty", reason: "classes and factors must have equal length" });
            }
        }
        ProcessorSpec::RankTruncation { max_rank } => {
            if *max_rank == 0 {
                return Err(SamplingError::InvalidConfig { field: "rank_truncation.max_rank", reason: "must be >= 1" });
            }
        }
        _ => {}
    }
    Ok(())
}

fn sampling_method_to_json(method: &SamplingMethod) -> JsonValue {
    let obj = |pairs: Vec<(&str, JsonValue)>| JsonValue::Object(pairs.into_iter().map(|(k, v)| (k.to_string(), v)).collect());
    match method {
        SamplingMethod::Greedy { tie_break } => obj(vec![("kind", JsonValue::Str("greedy".into())), ("tie_break", tie_break_to_json(*tie_break))]),
        SamplingMethod::Multinomial { strategy } => obj(vec![("kind", JsonValue::Str("multinomial".into())), ("strategy", multinomial_strategy_to_json(*strategy))]),
        SamplingMethod::GumbelMax => obj(vec![("kind", JsonValue::Str("gumbel_max".into()))]),
        SamplingMethod::GumbelTopK { k } => obj(vec![("kind", JsonValue::Str("gumbel_top_k".into())), ("k", JsonValue::Num(*k as f64))]),
    }
}

fn sampling_method_from_json(value: &JsonValue) -> Result<SamplingMethod, SamplingError> {
    let kind = value.get("kind").and_then(JsonValue::as_str).ok_or(SamplingError::Corrupted { reason: "method missing kind" })?;
    match kind {
        "greedy" => Ok(SamplingMethod::Greedy { tie_break: tie_break_from_json(value.get("tie_break"))? }),
        "multinomial" => Ok(SamplingMethod::Multinomial { strategy: multinomial_strategy_from_json(value.get("strategy"))? }),
        "gumbel_max" => Ok(SamplingMethod::GumbelMax),
        "gumbel_top_k" => Ok(SamplingMethod::GumbelTopK { k: value.get("k").and_then(JsonValue::as_f64).unwrap_or(1.0) as usize }),
        _ => Err(SamplingError::Corrupted { reason: "unknown sampling method kind" }),
    }
}

fn tie_break_to_json(tie_break: TieBreak) -> JsonValue {
    JsonValue::Str(
        match tie_break {
            TieBreak::LowestTokenId => "lowest_token_id",
            TieBreak::HighestTokenId => "highest_token_id",
            TieBreak::FirstSeen => "first_seen",
        }
        .into(),
    )
}

fn tie_break_from_json(value: Option<&JsonValue>) -> Result<TieBreak, SamplingError> {
    match value.and_then(JsonValue::as_str) {
        Some("lowest_token_id") | None => Ok(TieBreak::LowestTokenId),
        Some("highest_token_id") => Ok(TieBreak::HighestTokenId),
        Some("first_seen") => Ok(TieBreak::FirstSeen),
        Some(_) => Err(SamplingError::Corrupted { reason: "unknown tie_break" }),
    }
}

fn multinomial_strategy_to_json(strategy: MultinomialStrategy) -> JsonValue {
    JsonValue::Str(
        match strategy {
            MultinomialStrategy::CdfBinarySearch => "cdf_binary_search",
            MultinomialStrategy::LinearScan => "linear_scan",
            MultinomialStrategy::Alias => "alias",
        }
        .into(),
    )
}

fn multinomial_strategy_from_json(value: Option<&JsonValue>) -> Result<MultinomialStrategy, SamplingError> {
    match value.and_then(JsonValue::as_str) {
        Some("cdf_binary_search") | None => Ok(MultinomialStrategy::CdfBinarySearch),
        Some("linear_scan") => Ok(MultinomialStrategy::LinearScan),
        Some("alias") => Ok(MultinomialStrategy::Alias),
        Some(_) => Err(SamplingError::Corrupted { reason: "unknown multinomial strategy" }),
    }
}

fn penalty_scope_to_json(scope: PenaltyScope) -> JsonValue {
    JsonValue::Str(
        match scope {
            PenaltyScope::PromptAndGenerated => "prompt_and_generated",
            PenaltyScope::GeneratedOnly => "generated_only",
            PenaltyScope::PromptOnly => "prompt_only",
        }
        .into(),
    )
}

fn penalty_scope_from_json(value: Option<&JsonValue>) -> Result<PenaltyScope, SamplingError> {
    match value.and_then(JsonValue::as_str) {
        Some("prompt_and_generated") | None => Ok(PenaltyScope::PromptAndGenerated),
        Some("generated_only") => Ok(PenaltyScope::GeneratedOnly),
        Some("prompt_only") => Ok(PenaltyScope::PromptOnly),
        Some(_) => Err(SamplingError::Corrupted { reason: "unknown penalty scope" }),
    }
}

fn processor_spec_to_json(spec: &ProcessorSpec) -> JsonValue {
    let obj = |pairs: Vec<(&str, JsonValue)>| JsonValue::Object(pairs.into_iter().map(|(k, v)| (k.to_string(), v)).collect());
    let tokens_json = |tokens: &[TokenId]| JsonValue::Array(tokens.iter().map(|t| JsonValue::Num(t.get() as f64)).collect());
    let phrases_json = |phrases: &[Vec<TokenId>]| JsonValue::Array(phrases.iter().map(|p| tokens_json(p)).collect());
    match spec {
        ProcessorSpec::Temperature { value } => obj(vec![("kind", JsonValue::Str("temperature".into())), ("value", value.to_json())]),
        ProcessorSpec::DynamicTemperature { base, entropy_gain, min, max } => obj(vec![
            ("kind", JsonValue::Str("dynamic_temperature".into())),
            ("base", base.to_json()),
            ("entropy_gain", JsonValue::Num(*entropy_gain)),
            ("min", JsonValue::Num(*min)),
            ("max", JsonValue::Num(*max)),
        ]),
        ProcessorSpec::TopK { k, min_keep } => obj(vec![("kind", JsonValue::Str("top_k".into())), ("k", k.to_json()), ("min_keep", JsonValue::Num(*min_keep as f64))]),
        ProcessorSpec::TopP { p, min_keep } => obj(vec![("kind", JsonValue::Str("top_p".into())), ("p", p.to_json()), ("min_keep", JsonValue::Num(*min_keep as f64))]),
        ProcessorSpec::MinP { p, min_keep } => obj(vec![("kind", JsonValue::Str("min_p".into())), ("p", p.to_json()), ("min_keep", JsonValue::Num(*min_keep as f64))]),
        ProcessorSpec::Typical { mass, min_keep } => obj(vec![("kind", JsonValue::Str("typical".into())), ("mass", mass.to_json()), ("min_keep", JsonValue::Num(*min_keep as f64))]),
        ProcessorSpec::LocallyTypical { mass, min_keep } => obj(vec![("kind", JsonValue::Str("locally_typical".into())), ("mass", mass.to_json()), ("min_keep", JsonValue::Num(*min_keep as f64))]),
        ProcessorSpec::TailFree { z, min_keep } => obj(vec![("kind", JsonValue::Str("tail_free".into())), ("z", z.to_json()), ("min_keep", JsonValue::Num(*min_keep as f64))]),
        ProcessorSpec::Epsilon { cutoff, min_keep } => obj(vec![("kind", JsonValue::Str("epsilon".into())), ("cutoff", cutoff.to_json()), ("min_keep", JsonValue::Num(*min_keep as f64))]),
        ProcessorSpec::Eta { cutoff, min_keep } => obj(vec![("kind", JsonValue::Str("eta".into())), ("cutoff", cutoff.to_json()), ("min_keep", JsonValue::Num(*min_keep as f64))]),
        ProcessorSpec::TopA { power, min_keep } => obj(vec![("kind", JsonValue::Str("top_a".into())), ("power", power.to_json()), ("min_keep", JsonValue::Num(*min_keep as f64))]),
        ProcessorSpec::RankTruncation { max_rank } => obj(vec![("kind", JsonValue::Str("rank_truncation".into())), ("max_rank", JsonValue::Num(*max_rank as f64))]),
        ProcessorSpec::AdaptiveTruncation { target_entropy, target_effective_count } => obj(vec![
            ("kind", JsonValue::Str("adaptive_truncation".into())),
            ("target_entropy", target_entropy.map_or(JsonValue::Null, JsonValue::Num)),
            ("target_effective_count", target_effective_count.map_or(JsonValue::Null, JsonValue::Num)),
        ]),
        ProcessorSpec::RepetitionPenalty { penalty, scope } => obj(vec![("kind", JsonValue::Str("repetition_penalty".into())), ("penalty", JsonValue::Num(*penalty as f64)), ("scope", penalty_scope_to_json(*scope))]),
        ProcessorSpec::PresencePenalty { penalty, scope } => obj(vec![("kind", JsonValue::Str("presence_penalty".into())), ("penalty", JsonValue::Num(*penalty as f64)), ("scope", penalty_scope_to_json(*scope))]),
        ProcessorSpec::FrequencyPenalty { penalty, scope } => obj(vec![("kind", JsonValue::Str("frequency_penalty".into())), ("penalty", JsonValue::Num(*penalty as f64)), ("scope", penalty_scope_to_json(*scope))]),
        ProcessorSpec::DecayingPenalty { penalty, window, half_life, scope } => obj(vec![
            ("kind", JsonValue::Str("decaying_penalty".into())),
            ("penalty", JsonValue::Num(*penalty as f64)),
            ("window", JsonValue::Num(*window as f64)),
            ("half_life", JsonValue::Num(*half_life)),
            ("scope", penalty_scope_to_json(*scope)),
        ]),
        ProcessorSpec::TokenClassPenalty { classes, factors } => obj(vec![
            ("kind", JsonValue::Str("token_class_penalty".into())),
            ("classes", JsonValue::Array(classes.iter().map(|c| JsonValue::Num(*c as f64)).collect())),
            ("factors", JsonValue::Array(factors.iter().map(|f| JsonValue::Num(*f as f64)).collect())),
        ]),
        ProcessorSpec::NoRepeatNgram { n } => obj(vec![("kind", JsonValue::Str("no_repeat_ngram".into())), ("n", JsonValue::Num(*n as f64))]),
        ProcessorSpec::PhrasePenalty { phrases, penalty } => obj(vec![("kind", JsonValue::Str("phrase_penalty".into())), ("phrases", phrases_json(phrases)), ("penalty", JsonValue::Num(*penalty as f64))]),
        ProcessorSpec::LogitBiasSparse { entries } => obj(vec![
            ("kind", JsonValue::Str("logit_bias_sparse".into())),
            ("entries", JsonValue::Array(entries.iter().map(|(t, b)| JsonValue::Array(vec![JsonValue::Num(t.get() as f64), JsonValue::Num(*b as f64)])).collect())),
        ]),
        ProcessorSpec::LogitBiasDense { values } => obj(vec![("kind", JsonValue::Str("logit_bias_dense".into())), ("values", JsonValue::Array(values.iter().map(|v| JsonValue::Num(*v as f64)).collect()))]),
        ProcessorSpec::AllowTokens { tokens } => obj(vec![("kind", JsonValue::Str("allow_tokens".into())), ("tokens", tokens_json(tokens))]),
        ProcessorSpec::ForbidTokens { tokens } => obj(vec![("kind", JsonValue::Str("forbid_tokens".into())), ("tokens", tokens_json(tokens))]),
        ProcessorSpec::SuppressSpecial => obj(vec![("kind", JsonValue::Str("suppress_special".into()))]),
        ProcessorSpec::BadWords { phrases } => obj(vec![("kind", JsonValue::Str("bad_words".into())), ("phrases", phrases_json(phrases))]),
        ProcessorSpec::SequenceEncouragement { phrases, bonus } => obj(vec![("kind", JsonValue::Str("sequence_encouragement".into())), ("phrases", phrases_json(phrases)), ("bonus", JsonValue::Num(*bonus as f64))]),
        ProcessorSpec::Mirostat { version, target_surprise, learning_rate } => obj(vec![
            ("kind", JsonValue::Str("mirostat".into())),
            ("version", JsonValue::Str(if *version == MirostatVersion::V1 { "v1".into() } else { "v2".into() })),
            ("target_surprise", JsonValue::Num(*target_surprise)),
            ("learning_rate", JsonValue::Num(*learning_rate)),
        ]),
        ProcessorSpec::EntropyPid { target, kp, ki, kd } => obj(vec![
            ("kind", JsonValue::Str("entropy_pid".into())),
            ("target", JsonValue::Num(*target)),
            ("kp", JsonValue::Num(*kp)),
            ("ki", JsonValue::Num(*ki)),
            ("kd", JsonValue::Num(*kd)),
        ]),
        ProcessorSpec::RepetitionController { window, threshold, boost } => obj(vec![
            ("kind", JsonValue::Str("repetition_controller".into())),
            ("window", JsonValue::Num(*window as f64)),
            ("threshold", JsonValue::Num(*threshold)),
            ("boost", JsonValue::Num(*boost)),
        ]),
        ProcessorSpec::ConfidenceController { low_entropy, high_entropy, low_temp, high_temp } => obj(vec![
            ("kind", JsonValue::Str("confidence_controller".into())),
            ("low_entropy", JsonValue::Num(*low_entropy)),
            ("high_entropy", JsonValue::Num(*high_entropy)),
            ("low_temp", JsonValue::Num(*low_temp)),
            ("high_temp", JsonValue::Num(*high_temp)),
        ]),
    }
}

fn processor_spec_from_json(value: &JsonValue) -> Result<ProcessorSpec, SamplingError> {
    let kind = value.get("kind").and_then(JsonValue::as_str).ok_or(SamplingError::Corrupted { reason: "processor missing kind" })?;
    let schedule = |key: &'static str| -> Result<Schedule, SamplingError> { value.get(key).ok_or(SamplingError::Corrupted { reason: "processor missing schedule field" }).and_then(Schedule::from_json) };
    let num = |key: &'static str, default: f64| value.get(key).and_then(JsonValue::as_f64).unwrap_or(default);
    let tokens = |key: &'static str| -> Vec<TokenId> {
        value.get(key).and_then(JsonValue::as_array).map(|a| a.iter().filter_map(JsonValue::as_f64).map(|n| TokenId::new(n as u32)).collect()).unwrap_or_default()
    };
    let phrases = |key: &'static str| -> Vec<Vec<TokenId>> {
        value
            .get(key)
            .and_then(JsonValue::as_array)
            .map(|a| a.iter().filter_map(JsonValue::as_array).map(|p| p.iter().filter_map(JsonValue::as_f64).map(|n| TokenId::new(n as u32)).collect()).collect())
            .unwrap_or_default()
    };
    match kind {
        "temperature" => Ok(ProcessorSpec::Temperature { value: schedule("value")? }),
        "dynamic_temperature" => Ok(ProcessorSpec::DynamicTemperature { base: schedule("base")?, entropy_gain: num("entropy_gain", 0.0), min: num("min", 0.0), max: num("max", 2.0) }),
        "top_k" => Ok(ProcessorSpec::TopK { k: schedule("k")?, min_keep: num("min_keep", 1.0) as usize }),
        "top_p" => Ok(ProcessorSpec::TopP { p: schedule("p")?, min_keep: num("min_keep", 1.0) as usize }),
        "min_p" => Ok(ProcessorSpec::MinP { p: schedule("p")?, min_keep: num("min_keep", 1.0) as usize }),
        "typical" => Ok(ProcessorSpec::Typical { mass: schedule("mass")?, min_keep: num("min_keep", 1.0) as usize }),
        "locally_typical" => Ok(ProcessorSpec::LocallyTypical { mass: schedule("mass")?, min_keep: num("min_keep", 1.0) as usize }),
        "tail_free" => Ok(ProcessorSpec::TailFree { z: schedule("z")?, min_keep: num("min_keep", 1.0) as usize }),
        "epsilon" => Ok(ProcessorSpec::Epsilon { cutoff: schedule("cutoff")?, min_keep: num("min_keep", 1.0) as usize }),
        "eta" => Ok(ProcessorSpec::Eta { cutoff: schedule("cutoff")?, min_keep: num("min_keep", 1.0) as usize }),
        "top_a" => Ok(ProcessorSpec::TopA { power: schedule("power")?, min_keep: num("min_keep", 1.0) as usize }),
        "rank_truncation" => Ok(ProcessorSpec::RankTruncation { max_rank: num("max_rank", 1.0) as usize }),
        "adaptive_truncation" => Ok(ProcessorSpec::AdaptiveTruncation {
            target_entropy: value.get("target_entropy").and_then(JsonValue::as_f64),
            target_effective_count: value.get("target_effective_count").and_then(JsonValue::as_f64),
        }),
        "repetition_penalty" => Ok(ProcessorSpec::RepetitionPenalty { penalty: num("penalty", 1.0) as f32, scope: penalty_scope_from_json(value.get("scope"))? }),
        "presence_penalty" => Ok(ProcessorSpec::PresencePenalty { penalty: num("penalty", 0.0) as f32, scope: penalty_scope_from_json(value.get("scope"))? }),
        "frequency_penalty" => Ok(ProcessorSpec::FrequencyPenalty { penalty: num("penalty", 0.0) as f32, scope: penalty_scope_from_json(value.get("scope"))? }),
        "decaying_penalty" => Ok(ProcessorSpec::DecayingPenalty {
            penalty: num("penalty", 0.0) as f32,
            window: num("window", 16.0) as usize,
            half_life: num("half_life", 1.0),
            scope: penalty_scope_from_json(value.get("scope"))?,
        }),
        "token_class_penalty" => Ok(ProcessorSpec::TokenClassPenalty {
            classes: value.get("classes").and_then(JsonValue::as_array).map(|a| a.iter().filter_map(JsonValue::as_f64).map(|n| n as u16).collect()).unwrap_or_default(),
            factors: value.get("factors").and_then(JsonValue::as_array).map(|a| a.iter().filter_map(JsonValue::as_f64).map(|n| n as f32).collect()).unwrap_or_default(),
        }),
        "no_repeat_ngram" => Ok(ProcessorSpec::NoRepeatNgram { n: num("n", 3.0) as usize }),
        "phrase_penalty" => Ok(ProcessorSpec::PhrasePenalty { phrases: phrases("phrases"), penalty: num("penalty", 0.0) as f32 }),
        "logit_bias_sparse" => Ok(ProcessorSpec::LogitBiasSparse {
            entries: value
                .get("entries")
                .and_then(JsonValue::as_array)
                .map(|a| {
                    a.iter()
                        .filter_map(JsonValue::as_array)
                        .filter_map(|pair| Some((TokenId::new(pair.first()?.as_f64()? as u32), pair.get(1)?.as_f64()? as f32)))
                        .collect()
                })
                .unwrap_or_default(),
        }),
        "logit_bias_dense" => Ok(ProcessorSpec::LogitBiasDense { values: value.get("values").and_then(JsonValue::as_array).map(|a| a.iter().filter_map(JsonValue::as_f64).map(|n| n as f32).collect()).unwrap_or_default() }),
        "allow_tokens" => Ok(ProcessorSpec::AllowTokens { tokens: tokens("tokens") }),
        "forbid_tokens" => Ok(ProcessorSpec::ForbidTokens { tokens: tokens("tokens") }),
        "suppress_special" => Ok(ProcessorSpec::SuppressSpecial),
        "bad_words" => Ok(ProcessorSpec::BadWords { phrases: phrases("phrases") }),
        "sequence_encouragement" => Ok(ProcessorSpec::SequenceEncouragement { phrases: phrases("phrases"), bonus: num("bonus", 0.0) as f32 }),
        "mirostat" => Ok(ProcessorSpec::Mirostat {
            version: if value.get("version").and_then(JsonValue::as_str) == Some("v1") { MirostatVersion::V1 } else { MirostatVersion::V2 },
            target_surprise: num("target_surprise", 5.0),
            learning_rate: num("learning_rate", 0.1),
        }),
        "entropy_pid" => Ok(ProcessorSpec::EntropyPid { target: num("target", 2.0), kp: num("kp", 0.1), ki: num("ki", 0.0), kd: num("kd", 0.0) }),
        "repetition_controller" => Ok(ProcessorSpec::RepetitionController { window: num("window", 16.0) as usize, threshold: num("threshold", 0.5), boost: num("boost", 0.2) }),
        "confidence_controller" => Ok(ProcessorSpec::ConfidenceController {
            low_entropy: num("low_entropy", 0.5),
            high_entropy: num("high_entropy", 3.0),
            low_temp: num("low_temp", 0.5),
            high_temp: num("high_temp", 1.2),
        }),
        _ => Err(SamplingError::Corrupted { reason: "unknown processor kind" }),
    }
}

/// ⚙️ Chainable constructor for [`SamplingConfig`]; `build()` runs full validation once.
#[derive(Clone, Debug)]
pub struct SamplingConfigBuilder {
    config: SamplingConfig,
}

impl SamplingConfigBuilder {
    pub fn new() -> Self {
        Self { config: SamplingConfig::default() }
    }

    pub fn method(mut self, method: SamplingMethod) -> Self {
        self.config.method = method;
        self
    }

    pub fn processor(mut self, processor: ProcessorSpec) -> Self {
        self.config.processors.push(processor);
        self
    }

    pub fn error_mode(mut self, mode: ErrorMode) -> Self {
        self.config.error_mode = mode;
        self
    }

    pub fn sanitize(mut self, policy: SanitizePolicy) -> Self {
        self.config.sanitize = policy;
        self
    }

    pub fn accum(mut self, accum: Accum) -> Self {
        self.config.accum = accum;
        self
    }

    pub fn seed(mut self, seed: u64) -> Self {
        self.config.seed = seed;
        self
    }

    pub fn candidate_count(mut self, count: usize) -> Self {
        self.config.candidate_count = count;
        self
    }

    pub fn min_tokens(mut self, min_tokens: usize) -> Self {
        self.config.min_tokens = min_tokens;
        self
    }

    pub fn max_tokens(mut self, max_tokens: usize) -> Self {
        self.config.max_tokens = max_tokens;
        self
    }

    pub fn forced(mut self, forced: ForcedSpec) -> Self {
        self.config.forced = forced;
        self
    }

    pub fn stops(mut self, stops: StopSpec) -> Self {
        self.config.stops = stops;
        self
    }

    pub fn constraint(mut self, constraint: ConstraintSpec) -> Self {
        self.config.constraints.push(constraint);
        self
    }

    pub fn logprobs(mut self, logprobs: LogprobsSpec) -> Self {
        self.config.logprobs = logprobs;
        self
    }

    pub fn limits(mut self, limits: SamplingLimits) -> Self {
        self.config.limits = limits;
        self
    }

    pub fn diagnostics(mut self, diagnostics: DiagnosticsSpec) -> Self {
        self.config.diagnostics = diagnostics;
        self
    }

    /// ⚙️ Validates and returns the finished config.
    pub fn build(self) -> Result<SamplingConfig, SamplingError> {
        self.config.validate()?;
        Ok(self.config)
    }
}

impl Default for SamplingConfigBuilder {
    fn default() -> Self {
        Self::new()
    }
}
// #endregion 🔖Config

// #region 🔖Candidates
/// 🏅 One scored token, at whatever pipeline stage produced it (pre- or post-truncation, or the
/// final selected token).
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Candidate {
    pub token: TokenId,
    pub raw_logit: f32,
    pub processed_logit: f32,
    pub prob: f32,
    pub logprob: f32,
    pub rank: u32,
}

/// 🏅 Alternative log-probabilities reported alongside a selection, before and/or after
/// truncation warpers ran (per [`LogprobsSpec`]).
#[derive(Clone, PartialEq, Debug, Default)]
pub struct TopLogprobs {
    pub pre_truncation: Vec<Candidate>,
    pub post_truncation: Vec<Candidate>,
}

/// 🏅 Optional per-step numerical/pipeline diagnostics (only populated when
/// [`DiagnosticsSpec::enabled`]).
#[derive(Clone, PartialEq, Debug, Default)]
pub struct StepDiagnostics {
    pub entropy: f64,
    pub effective_count: f64,
    pub truncation_mass: f64,
    pub masked_by: Vec<(&'static str, u32)>,
    pub timings_ns: Vec<(&'static str, u64)>,
    pub fallback: Option<FallbackAction>,
    pub health: Option<DistributionHealth>,
}

/// 🏅 Everything one [`sample_step_stateless`] call (or, later, a stateful engine step) returns.
#[derive(Clone, PartialEq, Debug)]
pub struct SamplingResult {
    pub token: TokenId,
    pub logprob: f32,
    pub finish: Option<FinishReason>,
    pub alternatives: Vec<Candidate>,
    pub top_logprobs: Option<TopLogprobs>,
    pub rng_stream: StreamKey,
    pub diagnostics: Option<StepDiagnostics>,
}
// #endregion 🔖Candidates

// #region 🔖Workspace
/// 🧰 Reinterprets a `&[u32]` as `&[TokenId]` without copying.
///
/// SAFETY: `TokenId` is `#[repr(transparent)]` over `u32` (see its definition in `🔖Ids`), so the
/// two types share identical size, alignment, and bit-pattern validity — every `u32` is a valid
/// `TokenId`.
fn cast_u32_slice_to_token_ids(ids: &[u32]) -> &[TokenId] {
    unsafe { core::slice::from_raw_parts(ids.as_ptr().cast::<TokenId>(), ids.len()) }
}

fn argmax_token(logits: &[f32], tie_break: TieBreak) -> TokenId {
    let mut best_idx = 0u32;
    let mut best_val = f32::NEG_INFINITY;
    for (i, &v) in logits.iter().enumerate() {
        let better = match tie_break {
            TieBreak::HighestTokenId => v >= best_val,
            TieBreak::LowestTokenId | TieBreak::FirstSeen => v > best_val,
        };
        if better {
            best_val = v;
            best_idx = i as u32;
        }
    }
    TokenId::new(best_idx)
}

fn argmax_index_in_slice(logits: &[f32], indices: &[u32]) -> u32 {
    let mut best = indices[0];
    let mut best_val = logits[best as usize];
    for &idx in &indices[1..] {
        let v = logits[idx as usize];
        if v > best_val {
            best_val = v;
            best = idx;
        }
    }
    best
}

/// 🧰 Per-step scratch state for one sequence's logits: raw/processed vocab-sized arrays, the
/// hard-mask bitset, the shrinking `live` candidate-index list every warper operates on, and the
/// buffers the shared prob-sort (`sort_live_by_prob_desc`) reuses so steady-state stepping never
/// allocates once capacities have grown to the vocabulary size.
pub struct LogitsWorkspace {
    vocab_size: usize,
    accum: Accum,
    raw: Vec<f32>,
    processed: Vec<f32>,
    mask: TokenBitset,
    live: Vec<u32>,
    probs: Vec<f32>,
    sort_order: Vec<u32>,
    sorted_live_buf: Vec<u32>,
    sorted_probs_buf: Vec<f32>,
    saved_argmax: TokenId,
}

impl LogitsWorkspace {
    pub fn new(vocab_size: usize) -> Self {
        Self {
            vocab_size,
            accum: Accum::default(),
            raw: vec![0.0; vocab_size],
            processed: vec![0.0; vocab_size],
            mask: TokenBitset::new_full(vocab_size),
            live: (0..vocab_size as u32).collect(),
            probs: Vec::with_capacity(vocab_size),
            sort_order: Vec::with_capacity(vocab_size),
            sorted_live_buf: Vec::with_capacity(vocab_size),
            sorted_probs_buf: Vec::with_capacity(vocab_size),
            saved_argmax: TokenId::new(0),
        }
    }

    /// 🧰 Grows every buffer to `vocab_size` if it is larger than the workspace's current
    /// capacity; a no-op (never shrinks) otherwise — the basis of pool reuse across batch slots.
    pub fn ensure_capacity(&mut self, vocab_size: usize) {
        if vocab_size <= self.vocab_size {
            return;
        }
        self.vocab_size = vocab_size;
        self.raw.resize(vocab_size, 0.0);
        self.processed.resize(vocab_size, 0.0);
        self.mask = TokenBitset::new_full(vocab_size);
    }

    pub fn set_accum(&mut self, accum: Accum) {
        self.accum = accum;
    }

    pub fn vocab_size(&self) -> usize {
        self.vocab_size
    }

    pub fn live(&self) -> &[u32] {
        &self.live
    }

    pub fn set_live(&mut self, live: Vec<u32>) {
        self.live = live;
    }

    pub fn processed(&self) -> &[f32] {
        &self.processed
    }

    pub fn raw(&self) -> &[f32] {
        &self.raw
    }

    pub fn probs(&self) -> &[f32] {
        &self.probs
    }

    pub fn mask_mut(&mut self) -> &mut TokenBitset {
        &mut self.mask
    }

    pub fn mask(&self) -> &TokenBitset {
        &self.mask
    }

    pub fn saved_argmax(&self) -> TokenId {
        self.saved_argmax
    }

    /// 🧰 Resets the workspace for a fresh step: copies `raw_logits` into `raw`, sanitizes it in
    /// place per `policy`, mirrors the sanitized values into `processed`, fills the mask, resets
    /// `live` to every vocabulary index, and records the (post-sanitize) argmax — the anchor the
    /// fallback ladder uses once every other option is exhausted. Returns the count of altered
    /// (NaN/Inf-resolved) entries.
    pub fn reset_for_step(&mut self, raw_logits: &[f32], policy: SanitizePolicy) -> Result<usize, SamplingError> {
        debug_assert_eq!(raw_logits.len(), self.vocab_size);
        self.raw.copy_from_slice(raw_logits);
        let altered = sanitize_logits(&mut self.raw, policy)?;
        self.processed.copy_from_slice(&self.raw);
        self.mask.fill();
        self.live.clear();
        self.live.extend(0..self.vocab_size as u32);
        self.saved_argmax = argmax_token(&self.raw, TieBreak::LowestTokenId);
        self.probs.clear();
        Ok(altered)
    }

    /// 🧰 Removes every `live` entry whose mask bit is unset — call once after all hard-mask
    /// processors have run, before any soft-penalty or truncation processor.
    pub fn sync_live_with_mask(&mut self) {
        let mask = &self.mask;
        self.live.retain(|&idx| mask.get(TokenId::new(idx)));
    }

    pub fn scale_processed_over_live(&mut self, factor: f32) {
        for &idx in &self.live {
            self.processed[idx as usize] *= factor;
        }
    }

    pub fn add_bias_over_live(&mut self, mut bias: impl FnMut(TokenId) -> f32) {
        for &idx in &self.live {
            self.processed[idx as usize] += bias(TokenId::new(idx));
        }
    }

    /// 🧰 Collapses `live` to a single entry: the current argmax of `processed` restricted to
    /// `live` — how `Temperature`/`DynamicTemperature` implement "temperature 0 == greedy".
    pub fn collapse_live_to_argmax(&mut self) {
        if self.live.is_empty() {
            return;
        }
        let best = argmax_index_in_slice(&self.processed, &self.live);
        self.live.clear();
        self.live.push(best);
    }

    /// 🧰 Softmax over the *current* `live` order (does not sort); used by processors that need
    /// this step's entropy without disturbing candidate order (e.g. [`DynamicTemperature`]).
    pub fn softmax_over_live(&mut self) -> f64 {
        let n = self.live.len();
        self.probs.resize(n, 0.0);
        softmax_live(&self.processed, &self.live, &mut self.probs, self.accum)
    }

    /// 🧰 Softmax over `live`, then reorders `live`/`probs` in lockstep by probability descending
    /// (ties ascending token id) — the shared, allocation-amortized sort every truncation warper
    /// (and the final distribution build) starts from.
    pub fn sort_live_by_prob_desc(&mut self) {
        self.softmax_over_live();
        let n = self.live.len();
        let Self { live, probs, sort_order, sorted_live_buf, sorted_probs_buf, .. } = self;
        if sort_order.len() < n {
            sort_order.resize(n, 0);
        }
        for (i, slot) in sort_order[..n].iter_mut().enumerate() {
            *slot = i as u32;
        }
        sort_order[..n].sort_unstable_by(|&a, &b| {
            probs[b as usize].partial_cmp(&probs[a as usize]).unwrap_or(core::cmp::Ordering::Equal).then_with(|| live[a as usize].cmp(&live[b as usize]))
        });
        if sorted_live_buf.len() < n {
            sorted_live_buf.resize(n, 0);
        }
        if sorted_probs_buf.len() < n {
            sorted_probs_buf.resize(n, 0.0);
        }
        for (slot, &idx) in sort_order[..n].iter().enumerate() {
            sorted_live_buf[slot] = live[idx as usize];
            sorted_probs_buf[slot] = probs[idx as usize];
        }
        core::mem::swap(live, sorted_live_buf);
        core::mem::swap(probs, sorted_probs_buf);
        live.truncate(n);
        probs.truncate(n);
    }

    /// 🧰 Keeps the top `keep.max(min_keep)` entries of an already-[`LogitsWorkspace::sort_live_by_prob_desc`]-sorted
    /// `live`/`probs` pair — every truncation warper's shared "never drop below `min_keep`" guarantee.
    pub fn truncate_live_to(&mut self, keep: usize, min_keep: usize) {
        let keep = keep.max(min_keep.min(self.live.len())).min(self.live.len());
        self.live.truncate(keep);
        self.probs.truncate(keep);
    }
}

/// 🧰 Output of one [`TokenSampler::sample`] call; cleared (not deallocated) between steps.
#[derive(Clone, Debug, Default)]
pub struct SelectionBuffer {
    pub chosen: Vec<Candidate>,
}

impl SelectionBuffer {
    pub fn clear(&mut self) {
        self.chosen.clear();
    }
}

/// 🧰 Reusable pool of [`LogitsWorkspace`]s for batch/continuous-batching use, so per-slot state
/// survives across steps instead of being reallocated.
#[derive(Default)]
pub struct WorkspacePool {
    slots: Vec<LogitsWorkspace>,
}

impl WorkspacePool {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn acquire(&mut self, vocab_size: usize) -> LogitsWorkspace {
        match self.slots.pop() {
            Some(mut workspace) => {
                workspace.ensure_capacity(vocab_size);
                workspace
            }
            None => LogitsWorkspace::new(vocab_size),
        }
    }

    pub fn release(&mut self, workspace: LogitsWorkspace) {
        self.slots.push(workspace);
    }
}
// #endregion 🔖Workspace

// #region 🔖Traits
/// 🔌 Read-only view of one sequence at one step, borrowed for the duration of a pipeline phase.
pub struct StepView<'a> {
    pub sequence: SequenceId,
    pub step: StepIndex,
    pub prompt: &'a [TokenId],
    pub generated: &'a [TokenId],
    pub vocab: &'a Vocabulary,
    pub adapter: Option<&'a dyn TokenTextAdapter>,
    pub last_entropy: Option<f64>,
}

fn schedule_input(view: &StepView<'_>) -> ScheduleInput {
    ScheduleInput { step: view.step, generated_len: view.generated.len(), last_entropy: view.last_entropy }
}

/// 🔌 The renormalized live distribution handed to samplers: every slice shares indexing, sorted
/// by probability descending with ties broken by ascending token id.
pub struct Distribution<'a> {
    pub tokens: &'a [TokenId],
    pub probs: &'a [f32],
    pub logprobs: &'a [f32],
    pub cdf: &'a [f64],
    pub entropy: f64,
}

/// 🔌 Which pipeline phase a [`LogitsProcessor`] belongs to; the (forthcoming, wave-4) engine
/// dispatches hard masks before soft penalties before truncation.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ProcessorKind {
    HardMask,
    SoftPenalty,
    Truncation,
}

/// 🔌 Opaque undo-log position returned by `LogitsProcessor::save`/`Constraint::save`/`StopCondition::save`.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct StateMark(pub u64);

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct ConstraintMark(pub u64);

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct StopMark(pub u64);

/// 🔌 One step of the logits pipeline: transforms `ws` in place (mask, soft penalty, or
/// truncation), and optionally commits per-sequence effects once a token is definitively accepted.
pub trait LogitsProcessor {
    fn name(&self) -> &'static str;
    fn kind(&self) -> ProcessorKind;
    fn process(&mut self, view: &StepView<'_>, ws: &mut LogitsWorkspace) -> Result<(), SamplingError>;
    fn commit(&mut self, _view: &StepView<'_>, _token: TokenId) {}
    fn save(&mut self) -> StateMark {
        StateMark::default()
    }
    fn rollback_to(&mut self, _mark: StateMark) {}
    fn reset(&mut self) {}
    fn fork(&self) -> Box<dyn LogitsProcessor>;
}

/// 🔌 Selects one or more tokens from the final [`Distribution`]; must write at least one
/// candidate into `out` or return an error.
pub trait TokenSampler {
    fn name(&self) -> &'static str;
    fn sample(&mut self, view: &StepView<'_>, dist: &Distribution<'_>, rng: &mut dyn RandomSource, out: &mut SelectionBuffer) -> Result<(), SamplingError>;
    fn fork(&self) -> Box<dyn TokenSampler>;
}

/// 🔌 A structural/lexical constraint on the next token (regex, grammar, JSON mode, ...).
pub trait Constraint {
    fn name(&self) -> &'static str;
    /// 🧱 ANDs the set of currently-valid tokens into `mask` (starts all-ones for the first
    /// constraint in a composition).
    fn fill_mask(&mut self, view: &StepView<'_>, mask: &mut TokenBitset) -> Result<(), SamplingError>;
    fn accept(&mut self, view: &StepView<'_>, token: TokenId) -> Result<(), SamplingError>;
    fn is_satisfied(&self) -> bool;
    fn is_finished(&self) -> bool;
    fn is_dead(&self) -> bool;
    fn save(&mut self) -> ConstraintMark;
    fn rollback_to(&mut self, mark: ConstraintMark);
    fn reset(&mut self);
    fn fork(&self) -> Box<dyn Constraint>;
}

/// 🛑 Result of feeding one token's bytes to a [`StopCondition`].
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum StopPoll {
    Continue,
    Hold { ambiguous_bytes: usize },
    Finished { reason: FinishReason, matched_bytes: usize },
}

pub trait StopCondition {
    fn name(&self) -> &'static str;
    fn on_token(&mut self, view: &StepView<'_>, token: TokenId) -> StopPoll;
    fn save(&mut self) -> StopMark;
    fn rollback_to(&mut self, mark: StopMark);
    fn reset(&mut self);
    fn fork(&self) -> Box<dyn StopCondition>;
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct ProcessorStats {
    pub masked: u32,
    pub timing_ns: u64,
}

/// 🔭 Hook points into the engine's per-step lifecycle; every method is a no-op default so
/// observers only implement what they need.
pub trait SamplingObserver {
    fn on_step_start(&mut self, _sequence: SequenceId, _step: StepIndex) {}
    fn on_processor(&mut self, _sequence: SequenceId, _name: &'static str, _stats: &ProcessorStats) {}
    fn on_fallback(&mut self, _sequence: SequenceId, _error: &SamplingError, _action: FallbackAction) {}
    fn on_token(&mut self, _sequence: SequenceId, _result: &SamplingResult) {}
    fn on_finish(&mut self, _sequence: SequenceId, _reason: FinishReason) {}
}

/// 🗂️ One rank's local top candidate, for sharded-vocabulary all-gather.
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct ShardCandidate {
    pub token: TokenId,
    pub logit: f32,
}

/// 🗂️ Deterministic collective-communication primitives sharded-vocabulary sampling needs.
pub trait Collective {
    fn rank(&self) -> usize;
    fn world_size(&self) -> usize;
    fn all_reduce_max_f32(&mut self, values: &mut [f32]) -> Result<(), SamplingError>;
    fn all_reduce_sum_f64(&mut self, values: &mut [f64]) -> Result<(), SamplingError>;
    /// 🗂️ Gathers every rank's local candidates into `out`, in rank order (deterministic).
    fn all_gather_candidates(&mut self, local: &[ShardCandidate], out: &mut Vec<ShardCandidate>) -> Result<(), SamplingError>;
}
// #endregion 🔖Traits

// #region 🔖Warpers
/// 🌡️ `logit / temperature`; `temperature <= 0` collapses to greedy via [`LogitsWorkspace::collapse_live_to_argmax`].
pub struct Temperature {
    pub value: Schedule,
}

impl LogitsProcessor for Temperature {
    fn name(&self) -> &'static str {
        "temperature"
    }
    fn kind(&self) -> ProcessorKind {
        ProcessorKind::SoftPenalty
    }
    fn process(&mut self, view: &StepView<'_>, ws: &mut LogitsWorkspace) -> Result<(), SamplingError> {
        let value = self.value.eval(schedule_input(view));
        if value <= 0.0 {
            ws.collapse_live_to_argmax();
        } else {
            ws.scale_processed_over_live(1.0 / value as f32);
        }
        Ok(())
    }
    fn fork(&self) -> Box<dyn LogitsProcessor> {
        Box::new(Self { value: self.value.clone() })
    }
}

/// 🌡️ Temperature whose value tracks the live set's current entropy: `clamp(base + gain *
/// entropy, min, max)`.
pub struct DynamicTemperature {
    pub base: Schedule,
    pub entropy_gain: f64,
    pub min: f64,
    pub max: f64,
}

impl LogitsProcessor for DynamicTemperature {
    fn name(&self) -> &'static str {
        "dynamic_temperature"
    }
    fn kind(&self) -> ProcessorKind {
        ProcessorKind::SoftPenalty
    }
    fn process(&mut self, view: &StepView<'_>, ws: &mut LogitsWorkspace) -> Result<(), SamplingError> {
        ws.softmax_over_live();
        let entropy = entropy_nats(ws.probs());
        let temp = (self.base.eval(schedule_input(view)) + self.entropy_gain * entropy).clamp(self.min, self.max);
        if temp <= 0.0 {
            ws.collapse_live_to_argmax();
        } else {
            ws.scale_processed_over_live(1.0 / temp as f32);
        }
        Ok(())
    }
    fn fork(&self) -> Box<dyn LogitsProcessor> {
        Box::new(Self { base: self.base.clone(), entropy_gain: self.entropy_gain, min: self.min, max: self.max })
    }
}

/// 🌡️ Keeps exactly the top `k` live tokens by probability.
pub struct TopK {
    pub k: Schedule,
    pub min_keep: usize,
}

impl LogitsProcessor for TopK {
    fn name(&self) -> &'static str {
        "top_k"
    }
    fn kind(&self) -> ProcessorKind {
        ProcessorKind::Truncation
    }
    fn process(&mut self, view: &StepView<'_>, ws: &mut LogitsWorkspace) -> Result<(), SamplingError> {
        let k = self.k.eval(schedule_input(view)).round().max(1.0) as usize;
        ws.sort_live_by_prob_desc();
        ws.truncate_live_to(k, self.min_keep);
        Ok(())
    }
    fn fork(&self) -> Box<dyn LogitsProcessor> {
        Box::new(Self { k: self.k.clone(), min_keep: self.min_keep })
    }
}

/// 🌡️ Nucleus sampling: retains the smallest prefix (by descending probability) whose cumulative
/// mass reaches `p`.
pub struct TopP {
    pub p: Schedule,
    pub min_keep: usize,
}

impl LogitsProcessor for TopP {
    fn name(&self) -> &'static str {
        "top_p"
    }
    fn kind(&self) -> ProcessorKind {
        ProcessorKind::Truncation
    }
    fn process(&mut self, view: &StepView<'_>, ws: &mut LogitsWorkspace) -> Result<(), SamplingError> {
        let p = self.p.eval(schedule_input(view)).clamp(0.0, 1.0);
        ws.sort_live_by_prob_desc();
        let probs = ws.probs();
        let mut cumulative = 0.0f64;
        let mut keep = probs.len();
        for (i, &pr) in probs.iter().enumerate() {
            cumulative += pr as f64;
            if cumulative >= p {
                keep = i + 1;
                break;
            }
        }
        ws.truncate_live_to(keep, self.min_keep);
        Ok(())
    }
    fn fork(&self) -> Box<dyn LogitsProcessor> {
        Box::new(Self { p: self.p.clone(), min_keep: self.min_keep })
    }
}

/// 🌡️ Retains tokens whose probability is at least `p` times the live set's maximum probability.
pub struct MinP {
    pub p: Schedule,
    pub min_keep: usize,
}

impl LogitsProcessor for MinP {
    fn name(&self) -> &'static str {
        "min_p"
    }
    fn kind(&self) -> ProcessorKind {
        ProcessorKind::Truncation
    }
    fn process(&mut self, view: &StepView<'_>, ws: &mut LogitsWorkspace) -> Result<(), SamplingError> {
        let p = self.p.eval(schedule_input(view)).clamp(0.0, 1.0) as f32;
        ws.sort_live_by_prob_desc();
        let probs = ws.probs();
        let threshold = probs.first().copied().unwrap_or(0.0) * p;
        let keep = probs.iter().take_while(|&&pr| pr >= threshold).count().max(1);
        ws.truncate_live_to(keep, self.min_keep);
        Ok(())
    }
    fn fork(&self) -> Box<dyn LogitsProcessor> {
        Box::new(Self { p: self.p.clone(), min_keep: self.min_keep })
    }
}

/// 🌡️ Shared implementation for [`Typical`] and [`LocallyTypical`]: ranks tokens by absolute
/// deviation of their surprisal (`-ln p`) from the live set's entropy and retains the smallest
/// such prefix whose cumulative mass reaches `mass`.
fn apply_typical_truncation(ws: &mut LogitsWorkspace, mass: f64, min_keep: usize) {
    ws.softmax_over_live();
    let n = ws.live().len();
    if n == 0 {
        return;
    }
    let probs = ws.probs().to_vec();
    let entropy = entropy_nats(&probs);
    let live = ws.live().to_vec();
    let mut order: Vec<usize> = (0..n).collect();
    order.sort_by(|&a, &b| {
        let da = (-(probs[a] as f64).ln() - entropy).abs();
        let db = (-(probs[b] as f64).ln() - entropy).abs();
        da.partial_cmp(&db).unwrap_or(core::cmp::Ordering::Equal).then_with(|| live[a].cmp(&live[b]))
    });
    let mut cumulative = 0.0f64;
    let mut keep = 0usize;
    for &i in &order {
        cumulative += probs[i] as f64;
        keep += 1;
        if cumulative >= mass {
            break;
        }
    }
    keep = keep.max(min_keep.min(n));
    let kept: Vec<u32> = order[..keep].iter().map(|&i| live[i]).collect();
    ws.set_live(kept);
}

/// 🌡️ Locally typical sampling (global variant): see [`apply_typical_truncation`].
pub struct Typical {
    pub mass: Schedule,
    pub min_keep: usize,
}

impl LogitsProcessor for Typical {
    fn name(&self) -> &'static str {
        "typical"
    }
    fn kind(&self) -> ProcessorKind {
        ProcessorKind::Truncation
    }
    fn process(&mut self, view: &StepView<'_>, ws: &mut LogitsWorkspace) -> Result<(), SamplingError> {
        let mass = self.mass.eval(schedule_input(view)).clamp(0.0, 1.0);
        apply_typical_truncation(ws, mass, self.min_keep);
        Ok(())
    }
    fn fork(&self) -> Box<dyn LogitsProcessor> {
        Box::new(Self { mass: self.mass.clone(), min_keep: self.min_keep })
    }
}

/// 🌡️ Locally typical sampling; this single-step engine applies the same rule as [`Typical`]
/// (the "local" distinction only matters across multiple denoising passes, not one logits step).
pub struct LocallyTypical {
    pub mass: Schedule,
    pub min_keep: usize,
}

impl LogitsProcessor for LocallyTypical {
    fn name(&self) -> &'static str {
        "locally_typical"
    }
    fn kind(&self) -> ProcessorKind {
        ProcessorKind::Truncation
    }
    fn process(&mut self, view: &StepView<'_>, ws: &mut LogitsWorkspace) -> Result<(), SamplingError> {
        let mass = self.mass.eval(schedule_input(view)).clamp(0.0, 1.0);
        apply_typical_truncation(ws, mass, self.min_keep);
        Ok(())
    }
    fn fork(&self) -> Box<dyn LogitsProcessor> {
        Box::new(Self { mass: self.mass.clone(), min_keep: self.min_keep })
    }
}

/// 🌡️ Tail-free sampling: cuts where the cumulative normalized second derivative of the sorted
/// probability curve reaches `z`.
pub struct TailFree {
    pub z: Schedule,
    pub min_keep: usize,
}

impl LogitsProcessor for TailFree {
    fn name(&self) -> &'static str {
        "tail_free"
    }
    fn kind(&self) -> ProcessorKind {
        ProcessorKind::Truncation
    }
    fn process(&mut self, view: &StepView<'_>, ws: &mut LogitsWorkspace) -> Result<(), SamplingError> {
        let z = self.z.eval(schedule_input(view)).clamp(0.0, 1.0);
        ws.sort_live_by_prob_desc();
        let probs = ws.probs().to_vec();
        let n = probs.len();
        if n < 3 {
            return Ok(());
        }
        let first_deriv: Vec<f32> = probs.windows(2).map(|w| (w[0] - w[1]).abs()).collect();
        let second_deriv: Vec<f32> = first_deriv.windows(2).map(|w| (w[0] - w[1]).abs()).collect();
        let total: f32 = second_deriv.iter().sum();
        let mut keep = n;
        if total > 0.0 {
            let mut cumulative = 0.0f32;
            for (i, &d) in second_deriv.iter().enumerate() {
                cumulative += d / total;
                if cumulative as f64 >= z {
                    keep = i + 2;
                    break;
                }
            }
        }
        ws.truncate_live_to(keep, self.min_keep);
        Ok(())
    }
    fn fork(&self) -> Box<dyn LogitsProcessor> {
        Box::new(Self { z: self.z.clone(), min_keep: self.min_keep })
    }
}

/// 🌡️ Drops any token with probability below an absolute `cutoff`.
pub struct EpsilonCutoff {
    pub cutoff: Schedule,
    pub min_keep: usize,
}

impl LogitsProcessor for EpsilonCutoff {
    fn name(&self) -> &'static str {
        "epsilon"
    }
    fn kind(&self) -> ProcessorKind {
        ProcessorKind::Truncation
    }
    fn process(&mut self, view: &StepView<'_>, ws: &mut LogitsWorkspace) -> Result<(), SamplingError> {
        let cutoff = self.cutoff.eval(schedule_input(view)).max(0.0);
        ws.sort_live_by_prob_desc();
        let probs = ws.probs();
        let keep = probs.iter().take_while(|&&p| p as f64 >= cutoff).count().max(1);
        ws.truncate_live_to(keep, self.min_keep);
        Ok(())
    }
    fn fork(&self) -> Box<dyn LogitsProcessor> {
        Box::new(Self { cutoff: self.cutoff.clone(), min_keep: self.min_keep })
    }
}

/// 🌡️ Entropy-adaptive cutoff: `eta = min(epsilon, sqrt(epsilon) * exp(-entropy))`.
pub struct EtaCutoff {
    pub cutoff: Schedule,
    pub min_keep: usize,
}

impl LogitsProcessor for EtaCutoff {
    fn name(&self) -> &'static str {
        "eta"
    }
    fn kind(&self) -> ProcessorKind {
        ProcessorKind::Truncation
    }
    fn process(&mut self, view: &StepView<'_>, ws: &mut LogitsWorkspace) -> Result<(), SamplingError> {
        let epsilon = self.cutoff.eval(schedule_input(view)).max(1e-12);
        ws.sort_live_by_prob_desc();
        let probs = ws.probs().to_vec();
        let entropy = entropy_nats(&probs);
        let eta = epsilon.min(epsilon.sqrt() * (-entropy).exp());
        let keep = probs.iter().take_while(|&&p| p as f64 >= eta).count().max(1);
        ws.truncate_live_to(keep, self.min_keep);
        Ok(())
    }
    fn fork(&self) -> Box<dyn LogitsProcessor> {
        Box::new(Self { cutoff: self.cutoff.clone(), min_keep: self.min_keep })
    }
}

/// 🌡️ Keeps tokens with `prob >= power * max_prob^2`.
pub struct TopA {
    pub power: Schedule,
    pub min_keep: usize,
}

impl LogitsProcessor for TopA {
    fn name(&self) -> &'static str {
        "top_a"
    }
    fn kind(&self) -> ProcessorKind {
        ProcessorKind::Truncation
    }
    fn process(&mut self, view: &StepView<'_>, ws: &mut LogitsWorkspace) -> Result<(), SamplingError> {
        let a = self.power.eval(schedule_input(view)).max(0.0);
        ws.sort_live_by_prob_desc();
        let probs = ws.probs();
        let max_p = probs.first().copied().unwrap_or(0.0) as f64;
        let threshold = a * max_p * max_p;
        let keep = probs.iter().take_while(|&&p| p as f64 >= threshold).count().max(1);
        ws.truncate_live_to(keep, self.min_keep);
        Ok(())
    }
    fn fork(&self) -> Box<dyn LogitsProcessor> {
        Box::new(Self { power: self.power.clone(), min_keep: self.min_keep })
    }
}

/// 🌡️ Keeps only the top `max_rank` tokens by probability (a fixed-count variant of [`TopK`]
/// used when rank alone, not a schedule, determines the cutoff).
pub struct RankTruncation {
    pub max_rank: usize,
}

impl LogitsProcessor for RankTruncation {
    fn name(&self) -> &'static str {
        "rank_truncation"
    }
    fn kind(&self) -> ProcessorKind {
        ProcessorKind::Truncation
    }
    fn process(&mut self, _view: &StepView<'_>, ws: &mut LogitsWorkspace) -> Result<(), SamplingError> {
        ws.sort_live_by_prob_desc();
        ws.truncate_live_to(self.max_rank, 1);
        Ok(())
    }
    fn fork(&self) -> Box<dyn LogitsProcessor> {
        Box::new(Self { max_rank: self.max_rank })
    }
}

/// 🌡️ Grows the kept prefix (by descending probability) until the renormalized subset's entropy
/// drops to `target_entropy`, or its effective candidate count reaches `target_effective_count`.
pub struct AdaptiveTruncation {
    pub target_entropy: Option<f64>,
    pub target_effective_count: Option<f64>,
}

impl LogitsProcessor for AdaptiveTruncation {
    fn name(&self) -> &'static str {
        "adaptive_truncation"
    }
    fn kind(&self) -> ProcessorKind {
        ProcessorKind::Truncation
    }
    fn process(&mut self, _view: &StepView<'_>, ws: &mut LogitsWorkspace) -> Result<(), SamplingError> {
        ws.sort_live_by_prob_desc();
        let probs = ws.probs().to_vec();
        let n = probs.len();
        let mut keep = n;
        let mut cumulative = 0.0f64;
        'search: for (i, &p) in probs.iter().enumerate() {
            cumulative += p as f64;
            let mut renorm_entropy = 0.0f64;
            for &q in &probs[..=i] {
                let r = q as f64 / cumulative;
                if r > 0.0 {
                    renorm_entropy -= r * r.ln();
                }
            }
            if let Some(target) = self.target_effective_count {
                if renorm_entropy.exp() >= target {
                    keep = i + 1;
                    break 'search;
                }
            }
            if let Some(target) = self.target_entropy {
                if renorm_entropy <= target {
                    keep = i + 1;
                    break 'search;
                }
            }
        }
        ws.truncate_live_to(keep, 1);
        Ok(())
    }
    fn fork(&self) -> Box<dyn LogitsProcessor> {
        Box::new(Self { target_entropy: self.target_entropy, target_effective_count: self.target_effective_count })
    }
}

/// 🌡️ Builds a [`LogitsProcessor`] from a [`ProcessorSpec`]. Grows across implementation waves;
/// variants without a struct yet (penalties, biases, adaptive controllers — see later regions)
/// fall through to an error until their region lands.
pub fn build_processor(spec: &ProcessorSpec) -> Result<Box<dyn LogitsProcessor>, SamplingError> {
    match spec {
        ProcessorSpec::Temperature { value } => Ok(Box::new(Temperature { value: value.clone() })),
        ProcessorSpec::DynamicTemperature { base, entropy_gain, min, max } => Ok(Box::new(DynamicTemperature { base: base.clone(), entropy_gain: *entropy_gain, min: *min, max: *max })),
        ProcessorSpec::TopK { k, min_keep } => Ok(Box::new(TopK { k: k.clone(), min_keep: *min_keep })),
        ProcessorSpec::TopP { p, min_keep } => Ok(Box::new(TopP { p: p.clone(), min_keep: *min_keep })),
        ProcessorSpec::MinP { p, min_keep } => Ok(Box::new(MinP { p: p.clone(), min_keep: *min_keep })),
        ProcessorSpec::Typical { mass, min_keep } => Ok(Box::new(Typical { mass: mass.clone(), min_keep: *min_keep })),
        ProcessorSpec::LocallyTypical { mass, min_keep } => Ok(Box::new(LocallyTypical { mass: mass.clone(), min_keep: *min_keep })),
        ProcessorSpec::TailFree { z, min_keep } => Ok(Box::new(TailFree { z: z.clone(), min_keep: *min_keep })),
        ProcessorSpec::Epsilon { cutoff, min_keep } => Ok(Box::new(EpsilonCutoff { cutoff: cutoff.clone(), min_keep: *min_keep })),
        ProcessorSpec::Eta { cutoff, min_keep } => Ok(Box::new(EtaCutoff { cutoff: cutoff.clone(), min_keep: *min_keep })),
        ProcessorSpec::TopA { power, min_keep } => Ok(Box::new(TopA { power: power.clone(), min_keep: *min_keep })),
        ProcessorSpec::RankTruncation { max_rank } => Ok(Box::new(RankTruncation { max_rank: *max_rank })),
        ProcessorSpec::AdaptiveTruncation { target_entropy, target_effective_count } => Ok(Box::new(AdaptiveTruncation { target_entropy: *target_entropy, target_effective_count: *target_effective_count })),
        _ => Err(SamplingError::InvalidConfig { field: "processors", reason: "processor kind not yet implemented" }),
    }
}
// #endregion 🔖Warpers

// #region 🔖Selection
fn candidate_from(dist: &Distribution<'_>, index: usize) -> Candidate {
    Candidate { token: dist.tokens[index], raw_logit: 0.0, processed_logit: 0.0, prob: dist.probs[index], logprob: dist.logprobs[index], rank: index as u32 }
}

/// 🎯 Walker's alias method, reimplemented locally (rather than reusing
/// [`mathematical_random::AliasTable`]) because that type's `sample` is hard-wired to the
/// concrete `mathematical_random::Rng` and cannot accept our `dyn RandomSource` trait object.
struct AliasTable {
    prob: Vec<f32>,
    alias: Vec<u32>,
}

impl AliasTable {
    fn new(weights: &[f32]) -> Self {
        let n = weights.len();
        if n == 0 {
            return Self { prob: vec![1.0], alias: vec![0] };
        }
        let sum: f32 = weights.iter().sum();
        if sum <= 0.0 {
            let mut prob = vec![0.0; n];
            prob[0] = 1.0;
            return Self { prob, alias: vec![0; n] };
        }
        let mut scaled: Vec<f32> = weights.iter().map(|w| w / sum * n as f32).collect();
        let mut small: Vec<usize> = Vec::new();
        let mut large: Vec<usize> = Vec::new();
        for (i, &p) in scaled.iter().enumerate() {
            if p < 1.0 {
                small.push(i);
            } else {
                large.push(i);
            }
        }
        let mut prob = vec![0.0f32; n];
        let mut alias = vec![0u32; n];
        while let (Some(s), Some(l)) = (small.pop(), large.pop()) {
            prob[s] = scaled[s];
            alias[s] = l as u32;
            scaled[l] = scaled[l] + scaled[s] - 1.0;
            if scaled[l] < 1.0 {
                small.push(l);
            } else {
                large.push(l);
            }
        }
        for l in large {
            prob[l] = 1.0;
        }
        for s in small {
            prob[s] = 1.0;
        }
        Self { prob, alias }
    }

    fn sample(&self, rng: &mut dyn RandomSource) -> usize {
        let n = self.prob.len();
        let i = rng.next_range(0, n as u64) as usize;
        if rng.next_f64() < self.prob[i] as f64 {
            i
        } else {
            self.alias[i] as usize
        }
    }
}

/// 🎯 Deterministic argmax selection over the (already prob-sorted) [`Distribution`].
pub struct GreedySampler {
    pub tie_break: TieBreak,
}

impl TokenSampler for GreedySampler {
    fn name(&self) -> &'static str {
        "greedy"
    }
    fn sample(&mut self, _view: &StepView<'_>, dist: &Distribution<'_>, _rng: &mut dyn RandomSource, out: &mut SelectionBuffer) -> Result<(), SamplingError> {
        if dist.tokens.is_empty() {
            return Err(SamplingError::EmptyDistribution);
        }
        let max_prob = dist.probs[0];
        let tie_epsilon = f32::EPSILON.max(max_prob.abs() * 1e-6);
        let mut end = 0usize;
        while end + 1 < dist.probs.len() && (dist.probs[end + 1] - max_prob).abs() <= tie_epsilon {
            end += 1;
        }
        let idx = match self.tie_break {
            TieBreak::HighestTokenId => end,
            TieBreak::LowestTokenId | TieBreak::FirstSeen => 0,
        };
        out.chosen.push(candidate_from(dist, idx));
        Ok(())
    }
    fn fork(&self) -> Box<dyn TokenSampler> {
        Box::new(Self { tie_break: self.tie_break })
    }
}

/// 🎯 Samples one token proportional to the live distribution via the configured strategy.
pub struct MultinomialSampler {
    pub strategy: MultinomialStrategy,
}

impl TokenSampler for MultinomialSampler {
    fn name(&self) -> &'static str {
        "multinomial"
    }
    fn sample(&mut self, _view: &StepView<'_>, dist: &Distribution<'_>, rng: &mut dyn RandomSource, out: &mut SelectionBuffer) -> Result<(), SamplingError> {
        if dist.tokens.is_empty() {
            return Err(SamplingError::EmptyDistribution);
        }
        let idx = match self.strategy {
            MultinomialStrategy::CdfBinarySearch => cdf_binary_search(dist.cdf, rng.next_f64()),
            MultinomialStrategy::LinearScan => {
                let u = rng.next_f64();
                let mut cumulative = 0.0f64;
                let mut idx = dist.probs.len() - 1;
                for (i, &p) in dist.probs.iter().enumerate() {
                    cumulative += p as f64;
                    if cumulative >= u {
                        idx = i;
                        break;
                    }
                }
                idx
            }
            MultinomialStrategy::Alias => {
                let table = AliasTable::new(dist.probs);
                table.sample(rng)
            }
        };
        out.chosen.push(candidate_from(dist, idx));
        Ok(())
    }
    fn fork(&self) -> Box<dyn TokenSampler> {
        Box::new(Self { strategy: self.strategy })
    }
}

/// 🎯 Gumbel-max trick: `argmax(logprob_i + Gumbel_i)`, statistically equivalent to multinomial
/// sampling but usable when only a perturb-then-argmax primitive is available.
pub struct GumbelMaxSampler;

impl TokenSampler for GumbelMaxSampler {
    fn name(&self) -> &'static str {
        "gumbel_max"
    }
    fn sample(&mut self, _view: &StepView<'_>, dist: &Distribution<'_>, rng: &mut dyn RandomSource, out: &mut SelectionBuffer) -> Result<(), SamplingError> {
        if dist.tokens.is_empty() {
            return Err(SamplingError::EmptyDistribution);
        }
        let mut best = 0usize;
        let mut best_score = f64::NEG_INFINITY;
        for i in 0..dist.tokens.len() {
            let score = dist.logprobs[i] as f64 + rng.gumbel();
            if score > best_score {
                best_score = score;
                best = i;
            }
        }
        out.chosen.push(candidate_from(dist, best));
        Ok(())
    }
    fn fork(&self) -> Box<dyn TokenSampler> {
        Box::new(Self)
    }
}

/// 🎯 Gumbel-top-k: `k` tokens without replacement, drawn by taking the top `k` of `logprob_i +
/// Gumbel_i`.
pub struct GumbelTopKSampler {
    pub k: usize,
}

impl TokenSampler for GumbelTopKSampler {
    fn name(&self) -> &'static str {
        "gumbel_top_k"
    }
    fn sample(&mut self, _view: &StepView<'_>, dist: &Distribution<'_>, rng: &mut dyn RandomSource, out: &mut SelectionBuffer) -> Result<(), SamplingError> {
        if dist.tokens.is_empty() {
            return Err(SamplingError::EmptyDistribution);
        }
        let k = self.k.min(dist.tokens.len());
        let mut scored: Vec<(f64, usize)> = (0..dist.tokens.len()).map(|i| (dist.logprobs[i] as f64 + rng.gumbel(), i)).collect();
        scored.sort_unstable_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(core::cmp::Ordering::Equal).then_with(|| dist.tokens[a.1].cmp(&dist.tokens[b.1])));
        for &(_, i) in scored.iter().take(k) {
            out.chosen.push(candidate_from(dist, i));
        }
        Ok(())
    }
    fn fork(&self) -> Box<dyn TokenSampler> {
        Box::new(Self { k: self.k })
    }
}

/// 🎯 Builds a [`TokenSampler`] from a [`SamplingMethod`].
pub fn build_sampler(method: &SamplingMethod) -> Box<dyn TokenSampler> {
    match method {
        SamplingMethod::Greedy { tie_break } => Box::new(GreedySampler { tie_break: *tie_break }),
        SamplingMethod::Multinomial { strategy } => Box::new(MultinomialSampler { strategy: *strategy }),
        SamplingMethod::GumbelMax => Box::new(GumbelMaxSampler),
        SamplingMethod::GumbelTopK { k } => Box::new(GumbelTopKSampler { k: *k }),
    }
}
// #endregion 🔖Selection

// #region 🔖Engine
fn cumulative_from_probs(probs: &[f32]) -> Vec<f64> {
    let mut out = Vec::with_capacity(probs.len());
    let mut sum = KahanSum::new();
    for &p in probs {
        sum.add(p as f64);
        out.push(sum.value());
    }
    if let Some(last) = out.last_mut() {
        *last = 1.0;
    }
    out
}

/// 🚂 Everything [`sample_step_stateless`] needs about the sequence beyond the raw logits.
pub struct StatelessStepInput<'a> {
    pub sequence: SequenceId,
    pub step: StepIndex,
    pub prompt: &'a [TokenId],
    pub generated: &'a [TokenId],
    pub vocab: &'a Vocabulary,
    pub adapter: Option<&'a dyn TokenTextAdapter>,
    pub last_entropy: Option<f64>,
}

/// 🚂 Runs one sampling step with no persistent per-sequence state (the "Stateless one-step
/// sampling" operating mode from § 1): applies every configured warper in order, falls back
/// through [`resolve_fallback`] if truncation empties the live set, builds the final distribution,
/// then samples via `config.method`. Penalties, biases, constraints, and stop conditions are not
/// yet applied here — they require [`SequenceState`] (added by the stateful engine in a later
/// wave); see [`build_processor`] for which [`ProcessorSpec`] variants are wired up so far.
pub fn sample_step_stateless(config: &SamplingConfig, ws: &mut LogitsWorkspace, rng: &mut dyn RandomSource, raw_logits: &[f32], input: StatelessStepInput<'_>) -> Result<SamplingResult, SamplingError> {
    input.vocab.validate_logits_len(raw_logits.len())?;
    ws.set_accum(config.accum);
    ws.reset_for_step(raw_logits, config.sanitize)?;

    let view = StepView {
        sequence: input.sequence,
        step: input.step,
        prompt: input.prompt,
        generated: input.generated,
        vocab: input.vocab,
        adapter: input.adapter,
        last_entropy: input.last_entropy,
    };

    let mut processors = Vec::with_capacity(config.processors.len());
    for spec in &config.processors {
        processors.push(build_processor(spec)?);
    }
    for processor in processors.iter_mut() {
        if ws.live().is_empty() {
            break;
        }
        processor.process(&view, ws)?;
    }

    let fallback = if ws.live().is_empty() {
        let (action, token) = resolve_fallback(None, input.vocab.eos.first().copied(), Some(ws.saved_argmax()));
        let token = token.ok_or(SamplingError::EmptyDistribution)?;
        ws.set_live(vec![token.get()]);
        Some(action)
    } else {
        None
    };

    ws.sort_live_by_prob_desc();
    let cdf = cumulative_from_probs(ws.probs());
    let logprobs: Vec<f32> = ws.probs().iter().map(|&p| (p as f64).ln() as f32).collect();
    let entropy = entropy_nats(ws.probs());
    let tokens = cast_u32_slice_to_token_ids(ws.live());
    let dist = Distribution { tokens, probs: ws.probs(), logprobs: &logprobs, cdf: &cdf, entropy };

    let mut sampler = build_sampler(&config.method);
    let mut selection = SelectionBuffer::default();
    sampler.sample(&view, &dist, rng, &mut selection)?;
    let chosen = *selection.chosen.first().ok_or(SamplingError::EmptyDistribution)?;

    let next_len = input.generated.len() + 1;
    let finish = if input.vocab.is_eos(chosen.token) {
        Some(FinishReason::EosToken)
    } else if next_len >= config.max_tokens {
        Some(FinishReason::MaxTokens)
    } else {
        None
    };

    let diagnostics = config.diagnostics.enabled.then(|| StepDiagnostics {
        entropy,
        effective_count: entropy.exp(),
        truncation_mass: 0.0,
        masked_by: Vec::new(),
        timings_ns: Vec::new(),
        fallback,
        health: Some(DistributionHealth::assess(ws.live().len(), ws.probs().iter().map(|&p| p as f64).sum())),
    });

    Ok(SamplingResult {
        token: chosen.token,
        logprob: chosen.logprob,
        finish,
        alternatives: selection.chosen,
        top_logprobs: None,
        rng_stream: StreamKey { request: 0, sequence: input.sequence.get(), beam: 0, candidate: 0, purpose: StreamPurpose::Selection },
        diagnostics,
    })
}
// #endregion 🔖Engine

// #region 🔖Tests
#[cfg(test)]
mod tests {
    use super::*;

    // #region 🔖IdsTests
    #[test]
    fn step_index_checked_next_overflows_to_none() {
        assert_eq!(StepIndex::new(u32::MAX).checked_next(), None);
        assert_eq!(StepIndex::new(0).checked_next(), Some(StepIndex::new(1)));
    }

    #[test]
    fn ids_display_show_raw_value() {
        assert_eq!(format!("{}", TokenId::new(42)), "42");
        assert_eq!(format!("{}", SequenceId::new(7)), "7");
        assert_eq!(format!("{}", StepIndex::new(3)), "3");
    }
    // #endregion 🔖IdsTests

    // #region 🔖ErrorsTests
    #[test]
    fn every_error_variant_has_nonempty_display() {
        let errors = [
            SamplingError::InvalidConfig { field: "temperature", reason: "must be >= 0" },
            SamplingError::VocabMismatch { expected: 10, actual: 5 },
            SamplingError::NonFiniteLogits { index: 3 },
            SamplingError::EmptyDistribution,
            SamplingError::ConstraintDead { constraint: "regex" },
            SamplingError::LimitExceeded { limit: "max_beam_width" },
            SamplingError::GrammarParse { offset: 0, reason: "unexpected token" },
            SamplingError::RegexParse { offset: 0, reason: "unbalanced paren" },
            SamplingError::AutomatonBudget { budget: "max_automaton_states" },
            SamplingError::SerializationVersion { expected: 1, actual: 2 },
            SamplingError::FingerprintMismatch,
            SamplingError::Corrupted { reason: "bad header" },
            SamplingError::Collective { reason: "timeout" },
            SamplingError::Callback { reason: "reranker panicked" },
            SamplingError::Cancelled,
        ];
        for error in &errors {
            assert!(!format!("{error}").is_empty());
            let _: &dyn std::error::Error = error;
        }
    }

    #[test]
    fn fallback_ladder_prefers_forced_over_eos_over_argmax_over_error() {
        let forced = Some(TokenId::new(1));
        let eos = Some(TokenId::new(2));
        let argmax = Some(TokenId::new(3));
        assert_eq!(resolve_fallback(forced, eos, argmax), (FallbackAction::ForcedToken, forced));
        assert_eq!(resolve_fallback(None, eos, argmax), (FallbackAction::Eos, eos));
        assert_eq!(resolve_fallback(None, None, argmax), (FallbackAction::ArgmaxRaw, argmax));
        assert_eq!(resolve_fallback(None, None, None), (FallbackAction::Error, None));
    }
    // #endregion 🔖ErrorsTests

    // #region 🔖LimitsTests
    #[test]
    fn default_limits_validate() {
        assert!(SamplingLimits::default().validate().is_ok());
    }

    #[test]
    fn zero_limit_fails_validation() {
        let mut limits = SamplingLimits::default();
        limits.max_beam_width = 0;
        assert!(limits.validate().is_err());
    }
    // #endregion 🔖LimitsTests

    // #region 🔖JsonTests
    #[test]
    fn json_round_trips_basic_values() {
        let text = r#"{"a":1,"b":[true,false,null,"x\ny"],"c":{"d":-2.5}}"#;
        let value = parse_json(text, 64).expect("valid json");
        assert_eq!(value.get("a").and_then(JsonValue::as_f64), Some(1.0));
        let b = value.get("b").and_then(JsonValue::as_array).expect("array");
        assert_eq!(b[0], JsonValue::Bool(true));
        assert_eq!(b[3], JsonValue::Str("x\ny".to_string()));
        assert_eq!(value.get("c").and_then(|c| c.get("d")).and_then(JsonValue::as_f64), Some(-2.5));

        let written = write_json(&value);
        let reparsed = parse_json(&written, 64).expect("valid round-trip json");
        assert_eq!(value, reparsed);
    }

    #[test]
    fn json_depth_cap_is_enforced() {
        let nested = "[".repeat(10) + &"]".repeat(10);
        assert!(parse_json(&nested, 5).is_err());
        assert!(parse_json(&nested, 20).is_ok());
    }

    #[test]
    fn json_rejects_trailing_garbage() {
        assert!(parse_json("1 2", 8).is_err());
    }
    // #endregion 🔖JsonTests

    // #region 🔖Utf8Tests
    #[test]
    fn utf8_status_classifies_complete_partial_invalid() {
        assert_eq!(utf8_status(b"hello"), Utf8Status::Complete);
        assert_eq!(utf8_status("héllo".as_bytes()), Utf8Status::Complete);
        let full = "é".as_bytes();
        assert_eq!(utf8_status(&full[..1]), Utf8Status::Partial { more: 1 });
        assert_eq!(utf8_status(&[0xFF]), Utf8Status::Invalid);
        assert_eq!(utf8_status(b""), Utf8Status::Complete);
    }
    // #endregion 🔖Utf8Tests

    // #region 🔖NumericsTests
    #[test]
    fn softmax_live_sums_to_one_and_matches_hand_computed() {
        let logits = [1.0f32, 2.0, 3.0];
        let live = [0u32, 1, 2];
        let mut probs = [0.0f32; 3];
        softmax_live(&logits, &live, &mut probs, Accum::F64);
        let sum: f32 = probs.iter().sum();
        assert!((sum - 1.0).abs() < 1e-6);
        // 📐 Hand-computed via exp(x - 3): [exp(-2), exp(-1), exp(0)] / sum
        let expected = [0.09003057f32, 0.24472847, 0.66524096];
        for (p, e) in probs.iter().zip(expected.iter()) {
            assert!((p - e).abs() < 1e-5, "{p} vs {e}");
        }
    }

    #[test]
    fn softmax_live_f32_and_f64_accum_agree_closely() {
        let logits: Vec<f32> = (0..50).map(|i| (i as f32) * 0.37 - 5.0).collect();
        let live: Vec<u32> = (0..50).collect();
        let mut probs_32 = vec![0.0f32; 50];
        let mut probs_64 = vec![0.0f32; 50];
        softmax_live(&logits, &live, &mut probs_32, Accum::F32);
        softmax_live(&logits, &live, &mut probs_64, Accum::F64);
        for (a, b) in probs_32.iter().zip(probs_64.iter()) {
            assert!((a - b).abs() < 1e-3, "{a} vs {b}");
        }
    }

    #[test]
    fn softmax_live_is_invariant_to_constant_shift() {
        let base = [1.0f32, 2.0, 3.0];
        let shifted = [1001.0f32, 1002.0, 1003.0];
        let live = [0u32, 1, 2];
        let mut probs_base = [0.0f32; 3];
        let mut probs_shifted = [0.0f32; 3];
        softmax_live(&base, &live, &mut probs_base, Accum::F64);
        softmax_live(&shifted, &live, &mut probs_shifted, Accum::F64);
        for (a, b) in probs_base.iter().zip(probs_shifted.iter()) {
            assert!((a - b).abs() < 1e-5);
        }
    }

    #[test]
    fn logsumexp_matches_naive_for_moderate_values() {
        let values = [0.1, 0.2, 0.3, -0.5];
        let naive = values.iter().map(|v| v.exp()).sum::<f64>().ln();
        assert!((logsumexp_f64(&values) - naive).abs() < 1e-9);
    }

    #[test]
    fn logsumexp_handles_all_neg_infinity() {
        assert_eq!(logsumexp_f64(&[f64::NEG_INFINITY, f64::NEG_INFINITY]), f64::NEG_INFINITY);
    }

    #[test]
    fn entropy_of_uniform_distribution_is_ln_n() {
        let probs = [0.25f32; 4];
        assert!((entropy_nats(&probs) - (4.0f64).ln()).abs() < 1e-6);
    }

    #[test]
    fn entropy_of_deterministic_distribution_is_zero() {
        let probs = [1.0f32, 0.0, 0.0];
        assert!(entropy_nats(&probs).abs() < 1e-9);
    }

    #[test]
    fn effective_candidate_count_matches_perplexity_of_uniform() {
        let probs = [0.125f32; 8];
        assert!((effective_candidate_count(&probs) - 8.0).abs() < 1e-6);
    }

    #[test]
    fn kahan_sum_matches_naive_sum_for_short_sequences() {
        let mut sum = KahanSum::new();
        let values = [1.0, 2.0, 3.0, 4.5];
        for v in values {
            sum.add(v);
        }
        assert!((sum.value() - 10.5).abs() < 1e-12);
    }

    #[test]
    fn partial_select_top_k_selects_highest_k_by_logit_with_tie_break() {
        let logits = [5.0f32, 1.0, 5.0, 3.0, 2.0];
        let mut live: Vec<u32> = (0..5).collect();
        let mut scratch = vec![0.0f32; 5];
        partial_select_top_k(&logits, &mut live, 3, &mut scratch);
        let mut top3 = live[..3].to_vec();
        top3.sort_unstable();
        assert_eq!(top3, vec![0, 2, 3]);
    }

    #[test]
    fn partial_select_top_k_is_noop_when_k_covers_everything() {
        let logits = [1.0f32, 2.0, 3.0];
        let mut live: Vec<u32> = (0..3).collect();
        let mut scratch = vec![0.0f32; 3];
        let before = live.clone();
        partial_select_top_k(&logits, &mut live, 3, &mut scratch);
        assert_eq!(live, before);
    }

    #[test]
    fn cdf_binary_search_finds_first_index_at_or_above_u() {
        let cdf = [0.2, 0.5, 0.5, 0.9, 1.0];
        assert_eq!(cdf_binary_search(&cdf, 0.0), 0);
        assert_eq!(cdf_binary_search(&cdf, 0.2), 0);
        assert_eq!(cdf_binary_search(&cdf, 0.21), 1);
        assert_eq!(cdf_binary_search(&cdf, 0.5), 1);
        assert_eq!(cdf_binary_search(&cdf, 0.95), 4);
        assert_eq!(cdf_binary_search(&cdf, 1.0), 4);
    }

    #[test]
    fn sanitize_logits_neg_inf_nan_policy_masks_nan_and_rejects_pos_inf() {
        let mut logits = [1.0f32, f32::NAN, f32::NEG_INFINITY];
        let altered = sanitize_logits(&mut logits, SanitizePolicy::NegInfNan).expect("no +inf present");
        assert_eq!(altered, 1);
        assert_eq!(logits[1], f32::NEG_INFINITY);

        let mut with_pos_inf = [1.0f32, f32::INFINITY];
        assert!(sanitize_logits(&mut with_pos_inf, SanitizePolicy::NegInfNan).is_err());
    }

    #[test]
    fn sanitize_logits_error_policy_rejects_any_non_finite() {
        let mut logits = [1.0f32, f32::NAN];
        assert!(sanitize_logits(&mut logits, SanitizePolicy::Error).is_err());
    }

    #[test]
    fn sanitize_logits_clamp_inf_policy_clamps_positive_infinity() {
        let mut logits = [1.0f32, f32::INFINITY];
        let altered = sanitize_logits(&mut logits, SanitizePolicy::ClampInf).expect("clamp policy never errors on inf");
        assert_eq!(altered, 1);
        assert_eq!(logits[1], f32::MAX);
    }
    // #endregion 🔖NumericsTests

    // #region 🔖BitsetTests
    #[test]
    fn bitset_new_full_has_exactly_len_bits_set() {
        let set = TokenBitset::new_full(70);
        assert_eq!(set.count_ones(), 70);
        for i in 0..70 {
            assert!(set.get(TokenId::new(i)));
        }
    }

    #[test]
    fn bitset_set_get_round_trip() {
        let mut set = TokenBitset::new_empty(10);
        set.set(TokenId::new(3), true);
        set.set(TokenId::new(7), true);
        assert!(set.get(TokenId::new(3)));
        assert!(set.get(TokenId::new(7)));
        assert!(!set.get(TokenId::new(4)));
        assert_eq!(set.count_ones(), 2);
        set.set(TokenId::new(3), false);
        assert!(!set.get(TokenId::new(3)));
    }

    #[test]
    fn bitset_and_or_and_not_operations() {
        let mut a = TokenBitset::new_empty(8);
        let mut b = TokenBitset::new_empty(8);
        a.set(TokenId::new(0), true);
        a.set(TokenId::new(1), true);
        b.set(TokenId::new(1), true);
        b.set(TokenId::new(2), true);

        let mut and = a.clone();
        and.and_with(&b);
        assert_eq!(and.count_ones(), 1);
        assert!(and.get(TokenId::new(1)));

        let mut or = a.clone();
        or.or_with(&b);
        assert_eq!(or.count_ones(), 3);

        let mut and_not = a.clone();
        and_not.and_not_with(&b);
        assert_eq!(and_not.count_ones(), 1);
        assert!(and_not.get(TokenId::new(0)));
    }

    #[test]
    fn bitset_iter_ones_skips_zero_words_across_boundaries() {
        let mut set = TokenBitset::new_empty(200);
        set.set(TokenId::new(5), true);
        set.set(TokenId::new(130), true);
        set.set(TokenId::new(199), true);
        let ones: Vec<u32> = set.iter_ones().map(TokenId::get).collect();
        assert_eq!(ones, vec![5, 130, 199]);
    }

    #[test]
    fn bitset_first_set_and_is_all_zero() {
        let mut set = TokenBitset::new_empty(64);
        assert!(set.is_all_zero());
        assert_eq!(set.first_set(), None);
        set.set(TokenId::new(40), true);
        assert!(!set.is_all_zero());
        assert_eq!(set.first_set(), Some(TokenId::new(40)));
    }
    // #endregion 🔖BitsetTests

    // #region 🔖RngTests
    #[test]
    fn counter_rng_is_deterministic_for_same_seed() {
        let mut a = CounterRng::from_seed(123);
        let mut b = CounterRng::from_seed(123);
        let seq_a: Vec<u64> = (0..32).map(|_| a.next_u64()).collect();
        let seq_b: Vec<u64> = (0..32).map(|_| b.next_u64()).collect();
        assert_eq!(seq_a, seq_b);
    }

    #[test]
    fn counter_rng_different_seeds_diverge() {
        let mut a = CounterRng::from_seed(1);
        let mut b = CounterRng::from_seed(2);
        let seq_a: Vec<u64> = (0..8).map(|_| a.next_u64()).collect();
        let seq_b: Vec<u64> = (0..8).map(|_| b.next_u64()).collect();
        assert_ne!(seq_a, seq_b);
    }

    #[test]
    fn counter_rng_split_is_independent_of_call_order() {
        let parent = CounterRng::from_seed(999);
        let key_a = StreamKey { request: 1, sequence: 2, beam: 0, candidate: 0, purpose: StreamPurpose::Selection };
        let key_b = StreamKey { request: 1, sequence: 3, beam: 0, candidate: 0, purpose: StreamPurpose::Selection };

        // 🎲 Splitting in either order from the same parent must produce identical child streams.
        let mut a_first = parent.split(key_a);
        let mut b_first = parent.split(key_b);
        let a_vals_1: Vec<u64> = (0..4).map(|_| a_first.next_u64()).collect();
        let b_vals_1: Vec<u64> = (0..4).map(|_| b_first.next_u64()).collect();

        let mut b_second = parent.split(key_b);
        let mut a_second = parent.split(key_a);
        let b_vals_2: Vec<u64> = (0..4).map(|_| b_second.next_u64()).collect();
        let a_vals_2: Vec<u64> = (0..4).map(|_| a_second.next_u64()).collect();

        assert_eq!(a_vals_1, a_vals_2);
        assert_eq!(b_vals_1, b_vals_2);
        assert_ne!(a_vals_1, b_vals_1);
    }

    #[test]
    fn counter_rng_split_differs_by_purpose() {
        let parent = CounterRng::from_seed(42);
        let base = StreamKey { request: 1, sequence: 1, beam: 0, candidate: 0, purpose: StreamPurpose::Selection };
        let gumbel = StreamKey { purpose: StreamPurpose::Gumbel, ..base };
        let mut a = parent.split(base);
        let mut b = parent.split(gumbel);
        assert_ne!(a.next_u64(), b.next_u64());
    }

    #[test]
    fn counter_rng_snapshot_restore_resumes_identically() {
        let mut original = CounterRng::from_seed(77);
        for _ in 0..9 {
            original.next_u64();
        }
        let snapshot = original.snapshot();
        let mut resumed = CounterRng::from_seed(0);
        resumed.restore(&snapshot).expect("matching kind restores cleanly");
        let expected: Vec<u64> = (0..16).map(|_| original.next_u64()).collect();
        let actual: Vec<u64> = (0..16).map(|_| resumed.next_u64()).collect();
        assert_eq!(expected, actual);
    }

    #[test]
    fn rng_snapshot_rejects_kind_mismatch_on_restore() {
        let snapshot = CounterRng::from_seed(1).snapshot();
        let mut xoshiro = XoshiroSource::from_seed(1);
        assert!(xoshiro.restore(&snapshot).is_err());
    }

    #[test]
    fn rng_snapshot_text_round_trips() {
        let snapshot = RngSnapshot { kind: RngKind::Counter, words: [1, 2, 3, 4] };
        let text = snapshot.to_text();
        let parsed = RngSnapshot::from_text(&text).expect("valid snapshot text");
        assert_eq!(snapshot, parsed);
    }

    #[test]
    fn xoshiro_source_matches_underlying_rng_sequence() {
        let mut source = XoshiroSource::from_seed(4242);
        let mut reference = mathematical_random::Rng::from_seed(4242);
        for _ in 0..16 {
            assert_eq!(source.next_u64(), reference.next_u64());
        }
    }

    #[test]
    fn next_f64_open01_is_never_zero() {
        let mut rng = CounterRng::from_seed(0);
        for _ in 0..1000 {
            let u = rng.next_f64_open01();
            assert!(u > 0.0 && u <= 1.0, "u = {u} out of (0, 1]");
        }
    }

    #[test]
    fn next_range_stays_within_bounds() {
        let mut rng = CounterRng::from_seed(5);
        for _ in 0..1000 {
            let x = rng.next_range(3, 9);
            assert!((3..9).contains(&x));
        }
        assert_eq!(rng.next_range(4, 4), 4);
    }
    // #endregion 🔖RngTests

    // #region 🔖VocabularyTests
    #[test]
    fn vocabulary_validates_logits_length() {
        let vocab = Vocabulary::new(10);
        assert!(vocab.validate_logits_len(10).is_ok());
        assert!(vocab.validate_logits_len(9).is_err());
    }

    #[test]
    fn vocabulary_is_eos_reflects_configured_set() {
        let vocab = Vocabulary::new(10).with_eos(vec![TokenId::new(0), TokenId::new(1)]);
        assert!(vocab.is_eos(TokenId::new(0)));
        assert!(!vocab.is_eos(TokenId::new(2)));
    }

    #[test]
    fn slice_text_adapter_returns_bytes_and_stable_fingerprint() {
        let tokens: Vec<&[u8]> = vec![b"ab", b"c"];
        let adapter = SliceTextAdapter::new(&tokens);
        assert_eq!(adapter.vocab_size(), 2);
        assert_eq!(adapter.token_bytes(TokenId::new(0)), Some(b"ab".as_slice()));
        assert_eq!(adapter.token_bytes(TokenId::new(5)), None);
        let fp1 = adapter.fingerprint();
        let fp2 = SliceTextAdapter::new(&tokens).fingerprint();
        assert_eq!(fp1, fp2);

        let different: Vec<&[u8]> = vec![b"a", b"bc"];
        let fp3 = SliceTextAdapter::new(&different).fingerprint();
        assert_ne!(fp1, fp3, "separator byte must prevent boundary-shift collisions");
    }
    // #endregion 🔖VocabularyTests

    // #region 🔖ScheduleTests
    #[test]
    fn constant_schedule_ignores_input() {
        let schedule = Schedule::Constant(0.7);
        let input = ScheduleInput { step: StepIndex::new(50), generated_len: 50, last_entropy: None };
        assert_eq!(schedule.eval(input), 0.7);
    }

    #[test]
    fn linear_schedule_interpolates_and_clamps_at_bound() {
        let schedule = Schedule::Linear { from: 0.0, to: 1.0, over_steps: 10 };
        let at = |step: u32| schedule.eval(ScheduleInput { step: StepIndex::new(step), generated_len: 0, last_entropy: None });
        assert!((at(0) - 0.0).abs() < 1e-9);
        assert!((at(5) - 0.5).abs() < 1e-9);
        assert!((at(10) - 1.0).abs() < 1e-9);
        assert!((at(20) - 1.0).abs() < 1e-9, "must clamp past over_steps");
    }

    #[test]
    fn cosine_schedule_starts_and_ends_at_bounds() {
        let schedule = Schedule::Cosine { from: 1.0, to: 0.0, over_steps: 8 };
        let at = |step: u32| schedule.eval(ScheduleInput { step: StepIndex::new(step), generated_len: 0, last_entropy: None });
        assert!((at(0) - 1.0).abs() < 1e-9);
        assert!((at(8) - 0.0).abs() < 1e-9);
    }

    #[test]
    fn piecewise_schedule_holds_at_last_breakpoint() {
        let schedule = Schedule::Piecewise(vec![(StepIndex::new(0), 1.0), (StepIndex::new(5), 2.0), (StepIndex::new(10), 3.0)]);
        let at = |step: u32| schedule.eval(ScheduleInput { step: StepIndex::new(step), generated_len: 0, last_entropy: None });
        assert_eq!(at(0), 1.0);
        assert_eq!(at(3), 1.0);
        assert_eq!(at(5), 2.0);
        assert_eq!(at(7), 2.0);
        assert_eq!(at(100), 3.0);
    }

    #[test]
    fn by_position_schedule_clamps_past_its_length() {
        let schedule = Schedule::ByPosition(vec![0.1, 0.2, 0.3]);
        let at = |len: usize| schedule.eval(ScheduleInput { step: StepIndex::new(0), generated_len: len, last_entropy: None });
        assert_eq!(at(0), 0.1);
        assert_eq!(at(2), 0.3);
        assert_eq!(at(50), 0.3);
    }

    #[test]
    fn entropy_scaled_schedule_clamps_to_range() {
        let schedule = Schedule::EntropyScaled { base: 0.5, gain: 1.0, min: 0.0, max: 1.0 };
        let at = |entropy: f64| schedule.eval(ScheduleInput { step: StepIndex::new(0), generated_len: 0, last_entropy: Some(entropy) });
        assert_eq!(at(-10.0), 0.0);
        assert_eq!(at(10.0), 1.0);
        assert!((at(0.0) - 0.5).abs() < 1e-9);
    }

    #[test]
    fn schedule_json_round_trips_every_non_callback_variant() {
        let schedules = vec![
            Schedule::Constant(0.5),
            Schedule::Linear { from: 0.0, to: 1.0, over_steps: 10 },
            Schedule::Exponential { from: 0.1, to: 1.0, over_steps: 5 },
            Schedule::Cosine { from: 1.0, to: 0.0, over_steps: 8 },
            Schedule::Piecewise(vec![(StepIndex::new(0), 1.0), (StepIndex::new(4), 2.0)]),
            Schedule::ByPosition(vec![0.1, 0.2]),
            Schedule::EntropyScaled { base: 0.5, gain: 1.0, min: 0.0, max: 2.0 },
        ];
        for schedule in schedules {
            let json = schedule.to_json();
            let parsed = Schedule::from_json(&json).expect("round trip should succeed");
            assert_eq!(schedule, parsed);
        }
    }

    #[test]
    fn callback_schedule_serializes_but_refuses_to_deserialize() {
        fn double(input: ScheduleInput) -> f64 {
            input.step.get() as f64 * 2.0
        }
        let schedule = Schedule::Callback(double);
        let json = schedule.to_json();
        assert!(Schedule::from_json(&json).is_err());
    }
    // #endregion 🔖ScheduleTests

    // #region 🔖ConfigTests
    #[test]
    fn default_config_validates() {
        assert!(SamplingConfig::default().validate().is_ok());
    }

    #[test]
    fn all_presets_validate() {
        assert!(SamplingConfig::precise().validate().is_ok());
        assert!(SamplingConfig::balanced().validate().is_ok());
        assert!(SamplingConfig::creative().validate().is_ok());
        assert!(SamplingConfig::deterministic_test().validate().is_ok());
    }

    #[test]
    fn builder_produces_a_validated_config() {
        let config = SamplingConfigBuilder::new()
            .method(SamplingMethod::Multinomial { strategy: MultinomialStrategy::CdfBinarySearch })
            .processor(ProcessorSpec::Temperature { value: Schedule::Constant(0.8) })
            .seed(7)
            .max_tokens(128)
            .build()
            .expect("valid config");
        assert_eq!(config.seed, 7);
        assert_eq!(config.max_tokens, 128);
        assert_eq!(config.processors.len(), 1);
    }

    #[test]
    fn validate_rejects_min_tokens_above_max_tokens() {
        let config = SamplingConfig { min_tokens: 10, max_tokens: 5, ..SamplingConfig::default() };
        assert!(config.validate().is_err());
    }

    #[test]
    fn validate_rejects_zero_candidate_count() {
        let config = SamplingConfig { candidate_count: 0, ..SamplingConfig::default() };
        assert!(config.validate().is_err());
    }

    #[test]
    fn validate_rejects_candidate_count_above_limit() {
        let mut limits = SamplingLimits::default();
        limits.max_candidates = 4;
        let config = SamplingConfig { candidate_count: 5, limits, ..SamplingConfig::default() };
        assert!(config.validate().is_err());
    }

    #[test]
    fn validate_rejects_too_many_stop_sequences() {
        let mut limits = SamplingLimits::default();
        limits.max_stop_sequences = 1;
        let config = SamplingConfig {
            limits,
            stops: StopSpec { sequences: vec![b"a".to_vec(), b"b".to_vec()], ..StopSpec::default() },
            ..SamplingConfig::default()
        };
        assert!(config.validate().is_err());
    }

    #[test]
    fn validate_rejects_zero_no_repeat_ngram_order() {
        let config = SamplingConfig { processors: vec![ProcessorSpec::NoRepeatNgram { n: 0 }], ..SamplingConfig::default() };
        assert!(config.validate().is_err());
    }

    #[test]
    fn validate_rejects_mismatched_token_class_penalty_lengths() {
        let config = SamplingConfig {
            processors: vec![ProcessorSpec::TokenClassPenalty { classes: vec![0, 1], factors: vec![0.5] }],
            ..SamplingConfig::default()
        };
        assert!(config.validate().is_err());
    }

    #[test]
    fn config_fingerprint_is_stable_and_sensitive_to_changes() {
        let a = SamplingConfig::balanced();
        let b = SamplingConfig::balanced();
        assert_eq!(a.fingerprint(), b.fingerprint());
        let c = SamplingConfig { seed: a.seed + 1, ..a.clone() };
        assert_ne!(a.fingerprint(), c.fingerprint());
    }

    #[test]
    fn config_json_round_trips_core_fields() {
        let config = SamplingConfigBuilder::new()
            .method(SamplingMethod::GumbelTopK { k: 3 })
            .processor(ProcessorSpec::TopK { k: Schedule::Constant(40.0), min_keep: 1 })
            .processor(ProcessorSpec::RepetitionPenalty { penalty: 1.2, scope: PenaltyScope::GeneratedOnly })
            .seed(99)
            .candidate_count(2)
            .min_tokens(1)
            .max_tokens(200)
            .build()
            .expect("valid config");
        let json = config.to_json();
        let parsed = SamplingConfig::from_json(&json).expect("valid round trip");
        assert_eq!(parsed.method, config.method);
        assert_eq!(parsed.processors, config.processors);
        assert_eq!(parsed.seed, config.seed);
        assert_eq!(parsed.candidate_count, config.candidate_count);
        assert_eq!(parsed.min_tokens, config.min_tokens);
        assert_eq!(parsed.max_tokens, config.max_tokens);
    }

    #[test]
    fn config_json_rejects_unknown_version() {
        let json = JsonValue::Object(vec![("version".into(), JsonValue::Num(2.0))]);
        assert!(matches!(SamplingConfig::from_json(&json), Err(SamplingError::SerializationVersion { expected: 1, actual: 2 })));
    }

    #[test]
    fn all_processor_spec_variants_round_trip_through_json() {
        let specs = vec![
            ProcessorSpec::Temperature { value: Schedule::Constant(0.7) },
            ProcessorSpec::DynamicTemperature { base: Schedule::Constant(0.5), entropy_gain: 0.1, min: 0.0, max: 2.0 },
            ProcessorSpec::TopK { k: Schedule::Constant(40.0), min_keep: 1 },
            ProcessorSpec::TopP { p: Schedule::Constant(0.9), min_keep: 1 },
            ProcessorSpec::MinP { p: Schedule::Constant(0.05), min_keep: 1 },
            ProcessorSpec::Typical { mass: Schedule::Constant(0.9), min_keep: 1 },
            ProcessorSpec::LocallyTypical { mass: Schedule::Constant(0.9), min_keep: 1 },
            ProcessorSpec::TailFree { z: Schedule::Constant(0.95), min_keep: 1 },
            ProcessorSpec::Epsilon { cutoff: Schedule::Constant(0.001), min_keep: 1 },
            ProcessorSpec::Eta { cutoff: Schedule::Constant(0.001), min_keep: 1 },
            ProcessorSpec::TopA { power: Schedule::Constant(2.0), min_keep: 1 },
            ProcessorSpec::RankTruncation { max_rank: 50 },
            ProcessorSpec::AdaptiveTruncation { target_entropy: Some(2.0), target_effective_count: None },
            ProcessorSpec::RepetitionPenalty { penalty: 1.1, scope: PenaltyScope::GeneratedOnly },
            ProcessorSpec::PresencePenalty { penalty: 0.5, scope: PenaltyScope::PromptAndGenerated },
            ProcessorSpec::FrequencyPenalty { penalty: 0.3, scope: PenaltyScope::PromptOnly },
            ProcessorSpec::DecayingPenalty { penalty: 0.5, window: 32, half_life: 4.0, scope: PenaltyScope::GeneratedOnly },
            ProcessorSpec::TokenClassPenalty { classes: vec![0, 1], factors: vec![0.5, 0.9] },
            ProcessorSpec::NoRepeatNgram { n: 3 },
            ProcessorSpec::PhrasePenalty { phrases: vec![vec![TokenId::new(1), TokenId::new(2)]], penalty: 0.4 },
            ProcessorSpec::LogitBiasSparse { entries: vec![(TokenId::new(5), 2.0)] },
            ProcessorSpec::LogitBiasDense { values: vec![0.0, 1.0, -1.0] },
            ProcessorSpec::AllowTokens { tokens: vec![TokenId::new(1)] },
            ProcessorSpec::ForbidTokens { tokens: vec![TokenId::new(2)] },
            ProcessorSpec::SuppressSpecial,
            ProcessorSpec::BadWords { phrases: vec![vec![TokenId::new(3)]] },
            ProcessorSpec::SequenceEncouragement { phrases: vec![vec![TokenId::new(4)]], bonus: 1.5 },
            ProcessorSpec::Mirostat { version: MirostatVersion::V2, target_surprise: 5.0, learning_rate: 0.1 },
            ProcessorSpec::EntropyPid { target: 2.0, kp: 0.1, ki: 0.01, kd: 0.0 },
            ProcessorSpec::RepetitionController { window: 16, threshold: 0.5, boost: 0.2 },
            ProcessorSpec::ConfidenceController { low_entropy: 0.5, high_entropy: 3.0, low_temp: 0.5, high_temp: 1.2 },
        ];
        for spec in specs {
            let json = processor_spec_to_json(&spec);
            let parsed = processor_spec_from_json(&json).expect("round trip should succeed");
            assert_eq!(spec, parsed);
        }
    }
    // #endregion 🔖ConfigTests

    // #region 🔖WorkspaceTests
    fn small_vocab() -> Vocabulary {
        Vocabulary::new(8).with_eos(vec![TokenId::new(7)])
    }

    fn step_view<'a>(vocab: &'a Vocabulary, prompt: &'a [TokenId], generated: &'a [TokenId]) -> StepView<'a> {
        StepView { sequence: SequenceId::new(1), step: StepIndex::new(generated.len() as u32), prompt, generated, vocab, adapter: None, last_entropy: None }
    }

    #[test]
    fn workspace_reset_for_step_initializes_live_to_full_vocab_and_argmax() {
        let mut ws = LogitsWorkspace::new(5);
        let logits = [1.0f32, 3.0, 2.0, 3.0, 0.0];
        ws.reset_for_step(&logits, SanitizePolicy::NegInfNan).expect("finite logits never error");
        assert_eq!(ws.live(), &[0, 1, 2, 3, 4]);
        // 📐 Ties between indices 1 and 3 (both 3.0) break toward the lowest token id.
        assert_eq!(ws.saved_argmax(), TokenId::new(1));
    }

    #[test]
    fn workspace_sync_live_with_mask_removes_masked_entries() {
        let mut ws = LogitsWorkspace::new(4);
        ws.reset_for_step(&[0.0; 4], SanitizePolicy::NegInfNan).unwrap();
        ws.mask_mut().set(TokenId::new(2), false);
        ws.sync_live_with_mask();
        assert_eq!(ws.live(), &[0, 1, 3]);
    }

    #[test]
    fn workspace_sort_live_by_prob_desc_orders_by_probability_then_token_id() {
        let mut ws = LogitsWorkspace::new(4);
        ws.reset_for_step(&[1.0, 3.0, 3.0, 0.5], SanitizePolicy::NegInfNan).unwrap();
        ws.sort_live_by_prob_desc();
        assert_eq!(ws.live(), &[1, 2, 0, 3]);
        assert!(ws.probs()[0] >= ws.probs()[1]);
        assert!(ws.probs()[1] >= ws.probs()[2]);
    }

    #[test]
    fn workspace_truncate_live_to_respects_min_keep() {
        let mut ws = LogitsWorkspace::new(5);
        ws.reset_for_step(&[5.0, 4.0, 3.0, 2.0, 1.0], SanitizePolicy::NegInfNan).unwrap();
        ws.sort_live_by_prob_desc();
        ws.truncate_live_to(1, 3);
        assert_eq!(ws.live().len(), 3);
    }

    #[test]
    fn workspace_collapse_live_to_argmax_leaves_single_best_entry() {
        let mut ws = LogitsWorkspace::new(4);
        ws.reset_for_step(&[1.0, 5.0, 2.0, 5.0], SanitizePolicy::NegInfNan).unwrap();
        ws.collapse_live_to_argmax();
        assert_eq!(ws.live(), &[1]);
    }

    #[test]
    fn workspace_pool_reuses_released_workspace() {
        let mut pool = WorkspacePool::new();
        let ws = pool.acquire(16);
        assert_eq!(ws.vocab_size(), 16);
        pool.release(ws);
        let reused = pool.acquire(16);
        assert_eq!(reused.vocab_size(), 16);
    }
    // #endregion 🔖WorkspaceTests

    // #region 🔖WarperTests
    #[test]
    fn temperature_zero_collapses_to_greedy() {
        let mut ws = LogitsWorkspace::new(4);
        ws.reset_for_step(&[1.0, 5.0, 2.0, 0.0], SanitizePolicy::NegInfNan).unwrap();
        let vocab = small_vocab();
        let view = step_view(&vocab, &[], &[]);
        let mut temp = Temperature { value: Schedule::Constant(0.0) };
        temp.process(&view, &mut ws).unwrap();
        assert_eq!(ws.live(), &[1]);
    }

    #[test]
    fn temperature_scales_processed_logits() {
        let mut ws = LogitsWorkspace::new(3);
        ws.reset_for_step(&[2.0, 4.0, 6.0], SanitizePolicy::NegInfNan).unwrap();
        let vocab = Vocabulary::new(3);
        let view = step_view(&vocab, &[], &[]);
        let mut temp = Temperature { value: Schedule::Constant(2.0) };
        temp.process(&view, &mut ws).unwrap();
        assert_eq!(ws.processed(), &[1.0, 2.0, 3.0]);
    }

    #[test]
    fn top_k_keeps_exactly_k_highest_probability_tokens() {
        let mut ws = LogitsWorkspace::new(5);
        ws.reset_for_step(&[5.0, 4.0, 3.0, 2.0, 1.0], SanitizePolicy::NegInfNan).unwrap();
        let vocab = Vocabulary::new(5);
        let view = step_view(&vocab, &[], &[]);
        let mut top_k = TopK { k: Schedule::Constant(2.0), min_keep: 1 };
        top_k.process(&view, &mut ws).unwrap();
        let mut kept = ws.live().to_vec();
        kept.sort_unstable();
        assert_eq!(kept, vec![0, 1]);
    }

    #[test]
    fn top_p_retains_smallest_prefix_covering_cumulative_mass() {
        let mut ws = LogitsWorkspace::new(4);
        // 🌡️ Logits chosen so softmax gives one dominant token (~0.87) plus a long tail.
        ws.reset_for_step(&[10.0, 0.0, 0.0, 0.0], SanitizePolicy::NegInfNan).unwrap();
        let vocab = Vocabulary::new(4);
        let view = step_view(&vocab, &[], &[]);
        let mut top_p = TopP { p: Schedule::Constant(0.5), min_keep: 1 };
        top_p.process(&view, &mut ws).unwrap();
        assert_eq!(ws.live(), &[0]);
    }

    #[test]
    fn top_p_min_keep_overrides_a_too_small_cutoff() {
        let mut ws = LogitsWorkspace::new(4);
        ws.reset_for_step(&[1.0, 1.0, 1.0, 1.0], SanitizePolicy::NegInfNan).unwrap();
        let vocab = Vocabulary::new(4);
        let view = step_view(&vocab, &[], &[]);
        let mut top_p = TopP { p: Schedule::Constant(0.01), min_keep: 3 };
        top_p.process(&view, &mut ws).unwrap();
        assert_eq!(ws.live().len(), 3);
    }

    #[test]
    fn min_p_drops_tokens_far_below_the_maximum() {
        let mut ws = LogitsWorkspace::new(3);
        ws.reset_for_step(&[10.0, 0.0, -10.0], SanitizePolicy::NegInfNan).unwrap();
        let vocab = Vocabulary::new(3);
        let view = step_view(&vocab, &[], &[]);
        let mut min_p = MinP { p: Schedule::Constant(0.1), min_keep: 1 };
        min_p.process(&view, &mut ws).unwrap();
        assert_eq!(ws.live(), &[0]);
    }

    #[test]
    fn typical_and_locally_typical_agree_on_the_same_input() {
        let logits = [3.0f32, 1.0, 0.5, 0.1];
        let vocab = Vocabulary::new(4);
        let view = step_view(&vocab, &[], &[]);

        let mut ws_a = LogitsWorkspace::new(4);
        ws_a.reset_for_step(&logits, SanitizePolicy::NegInfNan).unwrap();
        let mut typical = Typical { mass: Schedule::Constant(0.8), min_keep: 1 };
        typical.process(&view, &mut ws_a).unwrap();

        let mut ws_b = LogitsWorkspace::new(4);
        ws_b.reset_for_step(&logits, SanitizePolicy::NegInfNan).unwrap();
        let mut locally = LocallyTypical { mass: Schedule::Constant(0.8), min_keep: 1 };
        locally.process(&view, &mut ws_b).unwrap();

        let mut a = ws_a.live().to_vec();
        let mut b = ws_b.live().to_vec();
        a.sort_unstable();
        b.sort_unstable();
        assert_eq!(a, b);
        assert!(!a.is_empty());
    }

    #[test]
    fn tail_free_keeps_at_least_min_keep_on_short_live_sets() {
        let mut ws = LogitsWorkspace::new(2);
        ws.reset_for_step(&[1.0, 2.0], SanitizePolicy::NegInfNan).unwrap();
        let vocab = Vocabulary::new(2);
        let view = step_view(&vocab, &[], &[]);
        let mut tail_free = TailFree { z: Schedule::Constant(0.9), min_keep: 1 };
        tail_free.process(&view, &mut ws).unwrap();
        assert!(!ws.live().is_empty());
    }

    #[test]
    fn epsilon_cutoff_drops_near_zero_probability_tokens() {
        let mut ws = LogitsWorkspace::new(3);
        ws.reset_for_step(&[20.0, -20.0, -20.0], SanitizePolicy::NegInfNan).unwrap();
        let vocab = Vocabulary::new(3);
        let view = step_view(&vocab, &[], &[]);
        let mut epsilon = EpsilonCutoff { cutoff: Schedule::Constant(0.01), min_keep: 1 };
        epsilon.process(&view, &mut ws).unwrap();
        assert_eq!(ws.live(), &[0]);
    }

    #[test]
    fn eta_cutoff_never_empties_live_set() {
        let mut ws = LogitsWorkspace::new(4);
        ws.reset_for_step(&[1.0, 2.0, 3.0, 4.0], SanitizePolicy::NegInfNan).unwrap();
        let vocab = Vocabulary::new(4);
        let view = step_view(&vocab, &[], &[]);
        let mut eta = EtaCutoff { cutoff: Schedule::Constant(0.1), min_keep: 1 };
        eta.process(&view, &mut ws).unwrap();
        assert!(!ws.live().is_empty());
    }

    #[test]
    fn top_a_drops_low_probability_tokens_relative_to_max() {
        let mut ws = LogitsWorkspace::new(3);
        ws.reset_for_step(&[10.0, -10.0, -10.0], SanitizePolicy::NegInfNan).unwrap();
        let vocab = Vocabulary::new(3);
        let view = step_view(&vocab, &[], &[]);
        let mut top_a = TopA { power: Schedule::Constant(0.5), min_keep: 1 };
        top_a.process(&view, &mut ws).unwrap();
        assert_eq!(ws.live(), &[0]);
    }

    #[test]
    fn rank_truncation_keeps_exactly_max_rank_entries() {
        let mut ws = LogitsWorkspace::new(5);
        ws.reset_for_step(&[5.0, 4.0, 3.0, 2.0, 1.0], SanitizePolicy::NegInfNan).unwrap();
        let vocab = Vocabulary::new(5);
        let view = step_view(&vocab, &[], &[]);
        let mut rank = RankTruncation { max_rank: 2 };
        rank.process(&view, &mut ws).unwrap();
        assert_eq!(ws.live().len(), 2);
    }

    #[test]
    fn adaptive_truncation_targeting_effective_count_shrinks_a_peaked_distribution() {
        let mut ws = LogitsWorkspace::new(6);
        ws.reset_for_step(&[10.0, 0.0, 0.0, 0.0, 0.0, 0.0], SanitizePolicy::NegInfNan).unwrap();
        let vocab = Vocabulary::new(6);
        let view = step_view(&vocab, &[], &[]);
        let mut adaptive = AdaptiveTruncation { target_entropy: None, target_effective_count: Some(1.5) };
        adaptive.process(&view, &mut ws).unwrap();
        assert!(ws.live().len() < 6);
    }

    #[test]
    fn min_keep_guarantee_is_honored_by_every_truncation_warper() {
        let vocab = Vocabulary::new(6);
        let view = step_view(&vocab, &[], &[]);
        let logits = [1.0f32, 1.0, 1.0, 1.0, 1.0, 1.0];
        let min_keep = 4;

        let mut ws = LogitsWorkspace::new(6);
        ws.reset_for_step(&logits, SanitizePolicy::NegInfNan).unwrap();
        TopK { k: Schedule::Constant(1.0), min_keep }.process(&view, &mut ws).unwrap();
        assert!(ws.live().len() >= min_keep);

        let mut ws = LogitsWorkspace::new(6);
        ws.reset_for_step(&logits, SanitizePolicy::NegInfNan).unwrap();
        TopP { p: Schedule::Constant(0.001), min_keep }.process(&view, &mut ws).unwrap();
        assert!(ws.live().len() >= min_keep);

        let mut ws = LogitsWorkspace::new(6);
        ws.reset_for_step(&logits, SanitizePolicy::NegInfNan).unwrap();
        MinP { p: Schedule::Constant(0.999), min_keep }.process(&view, &mut ws).unwrap();
        assert!(ws.live().len() >= min_keep);
    }
    // #endregion 🔖WarperTests

    // #region 🔖SelectionTests
    fn distribution_fixture<'a>(tokens: &'a [TokenId], probs: &'a [f32], logprobs: &'a [f32], cdf: &'a [f64]) -> Distribution<'a> {
        Distribution { tokens, probs, logprobs, cdf, entropy: entropy_nats(probs) }
    }

    #[test]
    fn greedy_sampler_lowest_token_id_picks_first_of_a_tie() {
        let tokens = [TokenId::new(0), TokenId::new(1), TokenId::new(2)];
        let probs = [0.5f32, 0.5, 0.0];
        let logprobs = [probs[0].ln(), probs[1].ln(), f32::NEG_INFINITY];
        let cdf = [0.5, 1.0, 1.0];
        let dist = distribution_fixture(&tokens, &probs, &logprobs, &cdf);
        let mut sampler = GreedySampler { tie_break: TieBreak::LowestTokenId };
        let mut out = SelectionBuffer::default();
        let mut rng = CounterRng::from_seed(1);
        sampler.sample(&step_view(&small_vocab(), &[], &[]), &dist, &mut rng, &mut out).unwrap();
        assert_eq!(out.chosen[0].token, TokenId::new(0));
    }

    #[test]
    fn greedy_sampler_highest_token_id_picks_last_of_a_tie() {
        let tokens = [TokenId::new(0), TokenId::new(1), TokenId::new(2)];
        let probs = [0.5f32, 0.5, 0.0];
        let logprobs = [probs[0].ln(), probs[1].ln(), f32::NEG_INFINITY];
        let cdf = [0.5, 1.0, 1.0];
        let dist = distribution_fixture(&tokens, &probs, &logprobs, &cdf);
        let mut sampler = GreedySampler { tie_break: TieBreak::HighestTokenId };
        let mut out = SelectionBuffer::default();
        let mut rng = CounterRng::from_seed(1);
        sampler.sample(&step_view(&small_vocab(), &[], &[]), &dist, &mut rng, &mut out).unwrap();
        assert_eq!(out.chosen[0].token, TokenId::new(1));
    }

    #[test]
    fn greedy_sampler_errors_on_empty_distribution() {
        let dist = distribution_fixture(&[], &[], &[], &[]);
        let mut sampler = GreedySampler { tie_break: TieBreak::LowestTokenId };
        let mut out = SelectionBuffer::default();
        let mut rng = CounterRng::from_seed(1);
        assert!(sampler.sample(&step_view(&small_vocab(), &[], &[]), &dist, &mut rng, &mut out).is_err());
    }

    #[test]
    fn multinomial_cdf_binary_search_matches_expected_frequencies() {
        let tokens = [TokenId::new(0), TokenId::new(1)];
        let probs = [0.2f32, 0.8];
        let logprobs = [probs[0].ln(), probs[1].ln()];
        let cdf = [0.2, 1.0];
        let dist = distribution_fixture(&tokens, &probs, &logprobs, &cdf);
        let mut sampler = MultinomialSampler { strategy: MultinomialStrategy::CdfBinarySearch };
        let mut rng = CounterRng::from_seed(2024);
        let vocab = small_vocab();
        let view = step_view(&vocab, &[], &[]);
        let draws = 20_000;
        let mut count_1 = 0u32;
        for _ in 0..draws {
            let mut out = SelectionBuffer::default();
            sampler.sample(&view, &dist, &mut rng, &mut out).unwrap();
            if out.chosen[0].token == TokenId::new(1) {
                count_1 += 1;
            }
        }
        let ratio = count_1 as f64 / draws as f64;
        assert!((ratio - 0.8).abs() < 0.02, "ratio {ratio} too far from 0.8");
    }

    #[test]
    fn multinomial_strategies_agree_statistically() {
        let tokens = [TokenId::new(0), TokenId::new(1), TokenId::new(2)];
        let probs = [0.1f32, 0.3, 0.6];
        let logprobs = [probs[0].ln(), probs[1].ln(), probs[2].ln()];
        let cdf = cumulative_from_probs(&probs);
        let dist = distribution_fixture(&tokens, &probs, &logprobs, &cdf);
        let vocab = small_vocab();
        let view = step_view(&vocab, &[], &[]);
        let draws = 20_000;

        for strategy in [MultinomialStrategy::CdfBinarySearch, MultinomialStrategy::LinearScan, MultinomialStrategy::Alias] {
            let mut sampler = MultinomialSampler { strategy };
            let mut rng = CounterRng::from_seed(555);
            let mut counts = [0u32; 3];
            for _ in 0..draws {
                let mut out = SelectionBuffer::default();
                sampler.sample(&view, &dist, &mut rng, &mut out).unwrap();
                counts[out.chosen[0].token.get() as usize] += 1;
            }
            for (i, &expected) in probs.iter().enumerate() {
                let ratio = counts[i] as f64 / draws as f64;
                assert!((ratio - expected as f64).abs() < 0.03, "strategy {strategy:?} index {i}: ratio {ratio} vs expected {expected}");
            }
        }
    }

    #[test]
    fn gumbel_max_sampler_matches_multinomial_marginals() {
        let tokens = [TokenId::new(0), TokenId::new(1), TokenId::new(2)];
        let probs = [0.2f32, 0.3, 0.5];
        let logprobs = [probs[0].ln(), probs[1].ln(), probs[2].ln()];
        let cdf = cumulative_from_probs(&probs);
        let dist = distribution_fixture(&tokens, &probs, &logprobs, &cdf);
        let vocab = small_vocab();
        let view = step_view(&vocab, &[], &[]);
        let mut sampler = GumbelMaxSampler;
        let mut rng = CounterRng::from_seed(321);
        let draws = 20_000;
        let mut counts = [0u32; 3];
        for _ in 0..draws {
            let mut out = SelectionBuffer::default();
            sampler.sample(&view, &dist, &mut rng, &mut out).unwrap();
            counts[out.chosen[0].token.get() as usize] += 1;
        }
        for (i, &expected) in probs.iter().enumerate() {
            let ratio = counts[i] as f64 / draws as f64;
            assert!((ratio - expected as f64).abs() < 0.03, "index {i}: ratio {ratio} vs expected {expected}");
        }
    }

    #[test]
    fn gumbel_top_k_returns_k_distinct_tokens() {
        let tokens = [TokenId::new(0), TokenId::new(1), TokenId::new(2), TokenId::new(3)];
        let probs = [0.4f32, 0.3, 0.2, 0.1];
        let logprobs = [probs[0].ln(), probs[1].ln(), probs[2].ln(), probs[3].ln()];
        let cdf = cumulative_from_probs(&probs);
        let dist = distribution_fixture(&tokens, &probs, &logprobs, &cdf);
        let vocab = small_vocab();
        let view = step_view(&vocab, &[], &[]);
        let mut sampler = GumbelTopKSampler { k: 2 };
        let mut rng = CounterRng::from_seed(7);
        let mut out = SelectionBuffer::default();
        sampler.sample(&view, &dist, &mut rng, &mut out).unwrap();
        assert_eq!(out.chosen.len(), 2);
        assert_ne!(out.chosen[0].token, out.chosen[1].token);
    }
    // #endregion 🔖SelectionTests

    // #region 🔖EngineTests
    #[test]
    fn stateless_step_is_deterministic_for_same_seed_and_config() {
        let vocab = small_vocab();
        let config = SamplingConfig::balanced();
        let logits = [1.0f32, 2.0, 0.5, 3.0, 1.5, 0.2, 0.1, -5.0];

        let run = || {
            let mut ws = LogitsWorkspace::new(8);
            let mut rng = CounterRng::from_seed(config.seed);
            let input = StatelessStepInput { sequence: SequenceId::new(1), step: StepIndex::new(0), prompt: &[], generated: &[], vocab: &vocab, adapter: None, last_entropy: None };
            sample_step_stateless(&config, &mut ws, &mut rng, &logits, input).unwrap()
        };
        let a = run();
        let b = run();
        assert_eq!(a.token, b.token);
        assert_eq!(a.logprob, b.logprob);
    }

    #[test]
    fn stateless_step_greedy_precise_always_picks_the_argmax() {
        let vocab = small_vocab();
        let config = SamplingConfig::precise();
        let logits = [1.0f32, 2.0, 9.0, 3.0, 1.5, 0.2, 0.1, -5.0];
        let mut ws = LogitsWorkspace::new(8);
        let mut rng = CounterRng::from_seed(0);
        let input = StatelessStepInput { sequence: SequenceId::new(1), step: StepIndex::new(0), prompt: &[], generated: &[], vocab: &vocab, adapter: None, last_entropy: None };
        let result = sample_step_stateless(&config, &mut ws, &mut rng, &logits, input).unwrap();
        assert_eq!(result.token, TokenId::new(2));
    }

    #[test]
    fn stateless_step_reports_eos_finish_reason() {
        let vocab = small_vocab();
        let config = SamplingConfig::precise();
        let mut logits = [0.0f32; 8];
        logits[7] = 100.0; // 📖 token 7 is the configured EOS token.
        let mut ws = LogitsWorkspace::new(8);
        let mut rng = CounterRng::from_seed(0);
        let input = StatelessStepInput { sequence: SequenceId::new(1), step: StepIndex::new(0), prompt: &[], generated: &[], vocab: &vocab, adapter: None, last_entropy: None };
        let result = sample_step_stateless(&config, &mut ws, &mut rng, &logits, input).unwrap();
        assert_eq!(result.token, TokenId::new(7));
        assert_eq!(result.finish, Some(FinishReason::EosToken));
    }

    #[test]
    fn stateless_step_reports_max_tokens_finish_reason() {
        let vocab = small_vocab();
        let config = SamplingConfig { max_tokens: 1, ..SamplingConfig::precise() };
        let logits = [1.0f32, 5.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        let mut ws = LogitsWorkspace::new(8);
        let mut rng = CounterRng::from_seed(0);
        let input = StatelessStepInput { sequence: SequenceId::new(1), step: StepIndex::new(0), prompt: &[], generated: &[], vocab: &vocab, adapter: None, last_entropy: None };
        let result = sample_step_stateless(&config, &mut ws, &mut rng, &logits, input).unwrap();
        assert_eq!(result.finish, Some(FinishReason::MaxTokens));
    }

    #[test]
    fn stateless_step_rejects_mismatched_logits_length() {
        let vocab = small_vocab();
        let config = SamplingConfig::precise();
        let logits = [0.0f32; 4];
        let mut ws = LogitsWorkspace::new(8);
        let mut rng = CounterRng::from_seed(0);
        let input = StatelessStepInput { sequence: SequenceId::new(1), step: StepIndex::new(0), prompt: &[], generated: &[], vocab: &vocab, adapter: None, last_entropy: None };
        assert!(sample_step_stateless(&config, &mut ws, &mut rng, &logits, input).is_err());
    }

    #[test]
    fn stateless_step_probabilities_sum_to_approximately_one() {
        let vocab = small_vocab();
        let config = SamplingConfig::balanced();
        let logits = [1.0f32, 2.0, 0.5, 3.0, 1.5, 0.2, 0.1, -5.0];
        let mut ws = LogitsWorkspace::new(8);
        let mut rng = CounterRng::from_seed(1);
        let input = StatelessStepInput { sequence: SequenceId::new(1), step: StepIndex::new(0), prompt: &[], generated: &[], vocab: &vocab, adapter: None, last_entropy: None };
        sample_step_stateless(&config, &mut ws, &mut rng, &logits, input).unwrap();
        let sum: f32 = ws.probs().iter().sum();
        assert!((sum - 1.0).abs() < 1e-4, "sum = {sum}");
    }
    // #endregion 🔖EngineTests
}
// #endregion 🔖Tests
