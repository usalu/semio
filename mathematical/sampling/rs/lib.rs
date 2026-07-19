//! 🎰 Model-agnostic LLM token-sampling engine: logits in, processor pipeline, constrained
//! distributions, deterministic seeded selection — plus a diffusion/continuous-noise solver module.

// #region 🔖Ids
/// 🧩 Index of one vocabulary entry. `u32` keeps candidate/mask arithmetic cheap even for
/// million-token sharded vocabularies while staying far below any real model's vocab size.
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
}
// #endregion 🔖Tests
