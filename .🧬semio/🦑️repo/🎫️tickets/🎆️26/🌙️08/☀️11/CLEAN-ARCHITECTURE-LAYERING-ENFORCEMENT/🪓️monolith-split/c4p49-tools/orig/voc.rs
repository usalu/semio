//! 🔤️ Token-sampling vocabulary: ids, errors, limits, JSON and UTF-8 text, numerics, token bitsets, seeded random streams and the token vocabulary.

#![allow(async_fn_in_trait)]

use semio_framework_dispatch_macros::{dyn_enum, dyn_enum_close};

// #region 🔖️Ids
/// 🧩️ Index of one vocabulary entry. `u32` keeps candidate/mask arithmetic cheap even for
/// million-token sharded vocabularies while staying far below any real model's vocab size.
/// `#[repr(transparent)]` lets [`cast_u32_slice_to_token_ids`] hand out a typed view over a raw
/// index buffer without copying.
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct TokenId(pub u32);

impl TokenId {
    /// 🧩️ Wraps a raw vocabulary index.
    pub const fn new(id: u32) -> Self {
        Self(id)
    }

    /// 🧩️ Raw vocabulary index.
    pub const fn get(self) -> u32 {
        self.0
    }
}

impl core::fmt::Display for TokenId {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// 🧩️ Identifies one generation request/sequence across batch reorders — never a slot index, so
/// continuous batching can shuffle rows without breaking RNG-stream or state addressing.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct SequenceId(pub u64);

impl SequenceId {
    /// 🧩️ Wraps a raw sequence identifier.
    pub const fn new(id: u64) -> Self {
        Self(id)
    }

    /// 🧩️ Raw sequence identifier.
    pub const fn get(self) -> u64 {
        self.0
    }
}

impl core::fmt::Display for SequenceId {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// 🧩️ Zero-based count of tokens generated so far for one sequence (excludes the prompt).
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct StepIndex(pub u32);

impl StepIndex {
    /// 🧩️ Wraps a raw step count.
    pub const fn new(step: u32) -> Self {
        Self(step)
    }

    /// 🧩️ Raw step count.
    pub const fn get(self) -> u32 {
        self.0
    }

    /// 🧩️ Next step, or `None` on overflow (caller-observable rather than a silent wraparound).
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
// #endregion 🔖️Ids

// #region 🔖️Errors
/// 🚨️ Every way a sampling step can fail to produce a token. Kept flat (no nested error types)
/// so callers can match exhaustively without chasing a `source()` chain for common cases.
#[derive(Clone, PartialEq, Debug)]
pub enum SamplingError {
    /// 🚨️ A configuration value failed validation (`field` names the offending knob).
    InvalidConfig { field: &'static str, reason: &'static str },
    /// 🚨️ Logits length does not match the configured vocabulary size.
    VocabMismatch { expected: usize, actual: usize },
    /// 🚨️ Logits contained NaN/Inf and the active [`SanitizePolicy`] is [`SanitizePolicy::Error`].
    NonFiniteLogits { index: usize },
    /// 🚨️ Every token was masked or truncated away with no fallback available.
    EmptyDistribution,
    /// 🚨️ A constraint reports no valid continuation exists (dead automaton state).
    ConstraintDead { constraint: &'static str },
    /// 🚨️ A configured resource cap (§ [`SamplingLimits`]) was exceeded.
    LimitExceeded { limit: &'static str },
    /// 🚨️ EBNF grammar text failed to parse at the given byte offset.
    GrammarParse { offset: usize, reason: &'static str },
    /// 🚨️ Regex pattern text failed to parse at the given byte offset.
    RegexParse { offset: usize, reason: &'static str },
    /// 🚨️ An automaton (DFA/NFA/Earley chart) exceeded its state/size budget mid-construction.
    AutomatonBudget { budget: &'static str },
    /// 🚨️ Serialized config/state carries an unsupported or mismatched format version.
    SerializationVersion { expected: u32, actual: u32 },
    /// 🚨️ A config/state fingerprint did not match the fingerprint recorded at serialization time.
    FingerprintMismatch,
    /// 🚨️ Serialized data is truncated, malformed, or fails an integrity check.
    Corrupted { reason: &'static str },
    /// 🚨️ A sharded-vocabulary collective operation failed (timeout, rank mismatch, ...).
    Collective { reason: &'static str },
    /// 🚨️ A user-supplied callback (rerank, similarity, denoiser, ...) reported failure.
    Callback { reason: &'static str },
    /// 🚨️ Generation was cancelled via an external cancellation signal.
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

/// 🚨️ Why a sequence stopped generating.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum FinishReason {
    /// 🚨️ An end-of-sequence token was selected.
    EosToken,
    /// 🚨️ A single stop token (outside the EOS set) was selected.
    StopToken,
    /// 🚨️ A configured stop text sequence matched, by index into `StopSpec::sequences`.
    StopSequence { index: usize },
    /// 🚨️ The per-sequence maximum generated-token count was reached.
    MaxTokens,
    /// 🚨️ The maximum wall-clock duration was reached.
    MaxTimeMs,
    /// 🚨️ A constraint (grammar/JSON/schema) reports completion.
    ConstraintComplete,
    /// 🚨️ An external cancellation signal was observed.
    Cancelled,
    /// 🚨️ No valid token existed and no fallback resolved (only reachable in permissive mode).
    Dead,
    /// 🚨️ A user-supplied stop callback returned `true`.
    Callback,
}

/// 🚨️ Whether a failed step surfaces an error or resolves through the fallback ladder.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum ErrorMode {
    /// 🚨️ Any [`SamplingError`] is returned to the caller.
    Strict,
    /// 🚨️ Recoverable failures resolve via [`resolve_fallback`] instead of erroring.
    #[default]
    Permissive,
}

/// 🚨️ Which rung of the fallback ladder resolved an otherwise-empty distribution.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum FallbackAction {
    /// 🚨️ A configured forced token was substituted.
    ForcedToken,
    /// 🚨️ The vocabulary's (first) EOS token was substituted.
    Eos,
    /// 🚨️ The pre-mask, pre-truncation argmax of the raw logits was substituted.
    ArgmaxRaw,
    /// 🚨️ No rung resolved; the caller's [`ErrorMode`] decides between `Dead` and an error.
    Error,
}

/// 🚨️ Walks the fallback ladder `ForcedToken -> Eos -> ArgmaxRaw -> Error` for an empty
/// distribution, in that fixed priority order — forced tokens are an explicit user instruction so
/// they always win, EOS is the next-safest "stop cleanly" choice, and only then does raw argmax
/// (which may violate active masks) get used as a last resort before giving up.
pub async fn resolve_fallback(forced: Option<TokenId>, eos: Option<TokenId>, argmax_raw: Option<TokenId>) -> (FallbackAction, Option<TokenId>) {
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
// #endregion 🔖️Errors

// #region 🔖️Limits
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
    pub async fn validate(&self) -> Result<(), SamplingError> {
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
// #endregion 🔖️Limits

// #region 🔖️Text
// #region 🔖️Json
/// 📜️ A parsed JSON value. Objects preserve insertion order via `Vec<(String, JsonValue)>` rather
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
    /// 📜️ Looks up a key in an object value; `None` for any other variant or a missing key.
    pub async fn get(&self, key: &str) -> Option<&JsonValue> {
        match self {
            Self::Object(entries) => entries.iter().find(|(k, _)| k == key).map(|(_, v)| v),
            _ => None,
        }
    }

    pub async fn as_str(&self) -> Option<&str> {
        match self {
            Self::Str(s) => Some(s),
            _ => None,
        }
    }

    pub async fn as_f64(&self) -> Option<f64> {
        match self {
            Self::Num(n) => Some(*n),
            _ => None,
        }
    }

    pub async fn as_bool(&self) -> Option<bool> {
        match self {
            Self::Bool(b) => Some(*b),
            _ => None,
        }
    }

    pub async fn as_array(&self) -> Option<&[JsonValue]> {
        match self {
            Self::Array(items) => Some(items),
            _ => None,
        }
    }
}

/// 📜️ Recursive-descent JSON parser over `&str`, depth-capped by `max_depth` so a maliciously
/// nested document (`[[[[...`) cannot blow the call stack — this is the same nesting bound that
/// protects the JSON-Schema/grammar constraint machinery later in the file.
pub async fn parse_json(text: &str, max_depth: usize) -> Result<JsonValue, SamplingError> {
    let bytes = text.as_bytes();
    let mut pos = 0usize;
    let value = parse_json_value(bytes, &mut pos, 0, max_depth).await?;
    skip_json_whitespace(bytes, &mut pos).await;
    if pos != bytes.len() {
        return Err(SamplingError::Corrupted { reason: "trailing data after JSON value" });
    }
    Ok(value)
}

async fn skip_json_whitespace(bytes: &[u8], pos: &mut usize) {
    while *pos < bytes.len() && matches!(bytes[*pos], b' ' | b'\t' | b'\n' | b'\r') {
        *pos += 1;
    }
}

async fn parse_json_value(bytes: &[u8], pos: &mut usize, depth: usize, max_depth: usize) -> Result<JsonValue, SamplingError> {
    if depth > max_depth {
        return Err(SamplingError::LimitExceeded { limit: "max_json_depth" });
    }
    skip_json_whitespace(bytes, pos).await;
    let Some(&byte) = bytes.get(*pos) else {
        return Err(SamplingError::Corrupted { reason: "unexpected end of JSON input" });
    };
    match byte {
        b'n' => parse_json_literal(bytes, pos, "null", JsonValue::Null).await,
        b't' => parse_json_literal(bytes, pos, "true", JsonValue::Bool(true)).await,
        b'f' => parse_json_literal(bytes, pos, "false", JsonValue::Bool(false)).await,
        b'"' => parse_json_string(bytes, pos).await.map(JsonValue::Str),
        b'[' => parse_json_array(bytes, pos, depth, max_depth).await,
        b'{' => parse_json_object(bytes, pos, depth, max_depth).await,
        b'-' | b'0'..=b'9' => parse_json_number(bytes, pos).await,
        _ => Err(SamplingError::Corrupted { reason: "unexpected byte in JSON input" }),
    }
}

async fn parse_json_literal(bytes: &[u8], pos: &mut usize, literal: &str, value: JsonValue) -> Result<JsonValue, SamplingError> {
    let lit = literal.as_bytes();
    if bytes[*pos..].starts_with(lit) {
        *pos += lit.len();
        Ok(value)
    } else {
        Err(SamplingError::Corrupted { reason: "invalid JSON literal" })
    }
}

async fn parse_json_string(bytes: &[u8], pos: &mut usize) -> Result<String, SamplingError> {
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
                        let code = parse_json_unicode_escape(bytes, pos).await?;
                        out.push(char::from_u32(code).unwrap_or('\u{FFFD}'));
                    }
                    _ => return Err(SamplingError::Corrupted { reason: "invalid JSON escape" }),
                }
            }
            _ => {
                // 📜️ Re-decode the UTF-8 codepoint starting at this byte instead of pushing raw bytes.
                let start = *pos - 1;
                let ch_len = utf8_sequence_len(byte).await.ok_or(SamplingError::Corrupted { reason: "invalid UTF-8 in JSON string" })?;
                let end = start + ch_len;
                let slice = bytes.get(start..end).ok_or(SamplingError::Corrupted { reason: "truncated UTF-8 in JSON string" })?;
                let s = core::str::from_utf8(slice).map_err(|_| SamplingError::Corrupted { reason: "invalid UTF-8 in JSON string" })?;
                out.push_str(s);
                *pos = end;
            }
        }
    }
}

async fn parse_json_unicode_escape(bytes: &[u8], pos: &mut usize) -> Result<u32, SamplingError> {
    let hex = bytes.get(*pos..*pos + 4).ok_or(SamplingError::Corrupted { reason: "truncated unicode escape" })?;
    let s = core::str::from_utf8(hex).map_err(|_| SamplingError::Corrupted { reason: "invalid unicode escape" })?;
    let code = u32::from_str_radix(s, 16).map_err(|_| SamplingError::Corrupted { reason: "invalid unicode escape" })?;
    *pos += 4;
    Ok(code)
}

async fn parse_json_number(bytes: &[u8], pos: &mut usize) -> Result<JsonValue, SamplingError> {
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

async fn parse_json_array(bytes: &[u8], pos: &mut usize, depth: usize, max_depth: usize) -> Result<JsonValue, SamplingError> {
    debug_assert_eq!(bytes[*pos], b'[');
    *pos += 1;
    let mut items = Vec::new();
    skip_json_whitespace(bytes, pos).await;
    if bytes.get(*pos) == Some(&b']') {
        *pos += 1;
        return Ok(JsonValue::Array(items));
    }
    loop {
        // 🔀️ `Box::pin(..)` breaks the `parse_json_value` ↔ `parse_json_array`/`parse_json_object`
        // mutual-recursion cycle — an `async fn` calling itself (even indirectly) has an
        // infinite-size future type otherwise (E0733). Not `dyn Future` (R1 only bans that in
        // trait-method return position): this is `Pin<Box<impl Future>>`, a concrete opaque type.
        items.push(Box::pin(parse_json_value(bytes, pos, depth + 1, max_depth)).await?);
        skip_json_whitespace(bytes, pos).await;
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

async fn parse_json_object(bytes: &[u8], pos: &mut usize, depth: usize, max_depth: usize) -> Result<JsonValue, SamplingError> {
    debug_assert_eq!(bytes[*pos], b'{');
    *pos += 1;
    let mut entries = Vec::new();
    skip_json_whitespace(bytes, pos).await;
    if bytes.get(*pos) == Some(&b'}') {
        *pos += 1;
        return Ok(JsonValue::Object(entries));
    }
    loop {
        skip_json_whitespace(bytes, pos).await;
        if bytes.get(*pos) != Some(&b'"') {
            return Err(SamplingError::Corrupted { reason: "expected string key in JSON object" });
        }
        let key = parse_json_string(bytes, pos).await?;
        skip_json_whitespace(bytes, pos).await;
        if bytes.get(*pos) != Some(&b':') {
            return Err(SamplingError::Corrupted { reason: "expected ':' in JSON object" });
        }
        *pos += 1;
        let value = Box::pin(parse_json_value(bytes, pos, depth + 1, max_depth)).await?;
        entries.push((key, value));
        skip_json_whitespace(bytes, pos).await;
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

/// 📜️ Writes a [`JsonValue`] in compact form with correct string escaping and round-trippable
/// float formatting (integral floats print without a trailing `.0`-free exponent surprise).
pub async fn write_json(value: &JsonValue) -> String {
    let mut out = String::new();
    write_json_into(value, &mut out).await;
    out
}

async fn write_json_into(value: &JsonValue, out: &mut String) {
    match value {
        JsonValue::Null => out.push_str("null"),
        JsonValue::Bool(b) => out.push_str(if *b { "true" } else { "false" }),
        JsonValue::Num(n) => write_json_number(*n, out).await,
        JsonValue::Str(s) => write_json_string(s, out).await,
        JsonValue::Array(items) => {
            out.push('[');
            for (i, item) in items.iter().enumerate() {
                if i > 0 {
                    out.push(',');
                }
                // 🔀️ `Box::pin(..)` breaks `write_json_into`'s self-recursion (E0733) — see the
                // matching note in `parse_json_array` (O1 de-dyn, math-dedyn).
                Box::pin(write_json_into(item, out)).await;
            }
            out.push(']');
        }
        JsonValue::Object(entries) => {
            out.push('{');
            for (i, (key, item)) in entries.iter().enumerate() {
                if i > 0 {
                    out.push(',');
                }
                write_json_string(key, out).await;
                out.push(':');
                Box::pin(write_json_into(item, out)).await;
            }
            out.push('}');
        }
    }
}

async fn write_json_number(n: f64, out: &mut String) {
    if n.fract() == 0.0 && n.abs() < 1e15 {
        out.push_str(&format!("{}", n as i64));
    } else {
        out.push_str(&format!("{n}"));
    }
}

async fn write_json_string(s: &str, out: &mut String) {
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
// #endregion 🔖️Json

// #region 🔖️Utf8
/// 📜️ How much of a byte sequence forms valid UTF-8, for incremental decoding of streamed token
/// bytes that may end mid-codepoint.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Utf8Status {
    /// 📜️ The whole slice is valid, complete UTF-8.
    Complete,
    /// 📜️ The slice ends with a valid-so-far partial codepoint needing `more` additional bytes.
    Partial { more: usize },
    /// 📜️ The slice contains a byte sequence that can never become valid UTF-8.
    Invalid,
}

/// 📜️ Length of the UTF-8 sequence a leading byte starts, or `None` if `byte` cannot lead one.
pub async fn utf8_sequence_len(byte: u8) -> Option<usize> {
    match byte {
        0x00..=0x7F => Some(1),
        0xC2..=0xDF => Some(2),
        0xE0..=0xEF => Some(3),
        0xF0..=0xF4 => Some(4),
        _ => None,
    }
}

/// 📜️ Classifies the tail of `bytes` as complete, valid-partial, or invalid UTF-8. Used to decide
/// how many trailing bytes of a just-emitted token must be held back until more bytes arrive.
pub async fn utf8_status(bytes: &[u8]) -> Utf8Status {
    if bytes.is_empty() {
        return Utf8Status::Complete;
    }
    // 📜️ Walk backward from the end to find the start of the last (possibly incomplete) codepoint.
    let mut start = bytes.len() - 1;
    let mut lead_len = None;
    for _ in 0..4 {
        if let Some(len) = utf8_sequence_len(bytes[start]).await {
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
// #endregion 🔖️Utf8
// #endregion 🔖️Text

// #region 🔖️Numerics
/// 📐️ How non-finite input logits are handled before any processor runs.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum SanitizePolicy {
    /// 📐️ Any NaN or +Inf logit is a hard [`SamplingError::NonFiniteLogits`].
    Error,
    /// 📐️ NaN and -Inf both collapse to `f32::NEG_INFINITY` (effectively masked); +Inf is rejected
    /// (an infinitely-preferred token needs an explicit decision, not a silent uniform pick).
    #[default]
    NegInfNan,
    /// 📐️ NaN collapses to `f32::NEG_INFINITY`; +Inf clamps to `f32::MAX`.
    ClampInf,
}

/// 📐️ Applies `policy` to `logits` in place. Returns the count of entries that were altered, for
/// diagnostics.
pub async fn sanitize_logits(logits: &mut [f32], policy: SanitizePolicy) -> Result<usize, SamplingError> {
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
            // 📐️ Negative infinity is always a valid "hard masked" representation; never altered.
        }
    }
    Ok(altered)
}

/// 📐️ Accumulation precision for softmax/entropy computation.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum Accum {
    F32,
    #[default]
    F64,
}

/// 📐️ Compensated (Kahan-Neumaier) running sum — keeps `f64` summation accurate over long live
/// candidate lists where naive summation would otherwise lose low-order bits.
#[derive(Clone, Copy, Default, Debug)]
pub struct KahanSum {
    sum: f64,
    correction: f64,
}

impl KahanSum {
    pub async fn new() -> Self {
        Self::default()
    }

    pub async fn add(&mut self, value: f64) {
        let t = self.sum + value;
        if self.sum.abs() >= value.abs() {
            self.correction += (self.sum - t) + value;
        } else {
            self.correction += (value - t) + self.sum;
        }
        self.sum = t;
    }

    pub async fn value(&self) -> f64 {
        self.sum + self.correction
    }
}

/// 📐️ Numerically stable softmax over `logits[live]`, writing normalized probabilities into
/// `probs[..live.len()]`. Uses max-subtraction so large logits never overflow `exp`, and
/// [`KahanSum`] under [`Accum::F64`] so the sum stays close to `1.0` even for wide vocabularies.
pub async fn softmax_live(logits: &[f32], live: &[u32], probs: &mut [f32], accum: Accum) -> f64 {
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
            let mut sum = KahanSum::new().await;
            let mut exps = vec![0.0f64; live.len()];
            for (slot, &i) in live.iter().enumerate() {
                let e = ((logits[i as usize] - max_logit) as f64).exp();
                exps[slot] = e;
                sum.add(e).await;
            }
            let total = sum.value().await;
            let inv = if total > 0.0 { 1.0 / total } else { 0.0 };
            for (slot, p) in probs.iter_mut().enumerate() {
                *p = (exps[slot] * inv) as f32;
            }
            total
        }
    }
}

/// 📐️ `log(sum(exp(values)))` computed via max-subtraction, safe for arbitrarily negative/large
/// inputs (including `-inf` entries, which contribute nothing).
pub async fn logsumexp_f64(values: &[f64]) -> f64 {
    let max = values.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    if !max.is_finite() {
        return max;
    }
    let mut sum = KahanSum::new().await;
    for &v in values {
        sum.add((v - max).exp()).await;
    }
    max + sum.value().await.ln()
}

/// 📐️ Shannon entropy in nats of a probability vector (`0 ln 0 := 0` by convention).
pub async fn entropy_nats(probs: &[f32]) -> f64 {
    let mut sum = KahanSum::new().await;
    for &p in probs {
        let p = p as f64;
        if p > 0.0 {
            sum.add(-p * p.ln()).await;
        }
    }
    sum.value().await
}

/// 📐️ `exp(entropy)`: the "effective" number of roughly-equally-likely candidates, matching the
/// perplexity of the live distribution.
pub async fn effective_candidate_count(probs: &[f32]) -> f64 {
    entropy_nats(probs).await.exp()
}

/// 📐️ Total probability mass removed by comparing a pre-truncation and post-truncation live-prob
/// vector that share the same normalization base (`1.0 - post_sum` when `post` sums the surviving
/// portion of a `pre` distribution that summed to `1.0`).
pub async fn truncation_mass(pre_kept_sum: f64) -> f64 {
    (1.0 - pre_kept_sum).max(0.0)
}

/// 📐️ Health flags for a live distribution, computed once per step for diagnostics and to decide
/// whether the fallback ladder must engage.
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct DistributionHealth {
    pub live_count: usize,
    pub prob_sum: f64,
    pub is_degenerate: bool,
}

impl DistributionHealth {
    pub async fn assess(live_count: usize, prob_sum: f64) -> Self {
        // 📐️ `is_nan() || <= 0.0` (not `!(prob_sum > 0.0)`) so NaN is explicitly, intentionally
        // flagged degenerate rather than relying on `>` being false for NaN — same result, clearer.
        Self { live_count, prob_sum, is_degenerate: live_count == 0 || prob_sum.is_nan() || prob_sum <= 0.0 }
    }
}

/// 📐️ Reorders `live[..]` so the top `k` (by `logits[live[i]]` descending, ties by ascending token
/// id for determinism) occupy `live[..k]`, unordered within that prefix — a Hoare-partition
/// quickselect, `O(n)` average instead of an `O(n log n)` full sort. `scratch` must have length
/// `>= live.len()` and its contents are overwritten.
pub async fn partial_select_top_k(logits: &[f32], live: &mut [u32], k: usize, scratch: &mut [f32]) {
    let n = live.len();
    if k >= n {
        return;
    }
    debug_assert!(scratch.len() >= n);
    let key = |token: u32| -> (f32, core::cmp::Reverse<u32>) { (logits[token as usize], core::cmp::Reverse(token)) };
    // 📐️ Selection over a byte-key derived from (logit desc, token asc) via index-sort scratch.
    for (slot, &t) in live.iter().enumerate() {
        scratch[slot] = logits[t as usize];
    }
    quickselect_desc(live, scratch, 0, n - 1, k, &key).await;
}

async fn quickselect_desc(live: &mut [u32], scratch: &mut [f32], mut lo: usize, mut hi: usize, k: usize, key: &impl Fn(u32) -> (f32, core::cmp::Reverse<u32>)) {
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

/// 📐️ Smallest index `i` such that `cdf[i] >= u`, via binary search over a nondecreasing CDF.
/// Falls back to the last index when floating-point rounding leaves `u` fractionally above the
/// final (nominally `1.0`) entry.
pub async fn cdf_binary_search(cdf: &[f64], u: f64) -> usize {
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
// #endregion 🔖️Numerics

// #region 🔖️Bitset
/// 🎭️ A dense bitset over `0..len` token indices, `u64`-word packed. The single shared
/// representation for hard masks (constraints, allow/forbid lists, forced-token exclusivity) —
/// every mask operation the pipeline needs is `O(vocab / 64)`.
#[derive(Clone, PartialEq, Debug)]
pub struct TokenBitset {
    words: Vec<u64>,
    len: usize,
}

impl TokenBitset {
    /// 🎭️ All-zero (empty) bitset over `len` tokens.
    pub async fn new_empty(len: usize) -> Self {
        Self { words: vec![0u64; len.div_ceil(64)], len }
    }

    /// 🎭️ All-one (full) bitset over `len` tokens.
    pub async fn new_full(len: usize) -> Self {
        let mut set = Self::new_empty(len).await;
        set.fill().await;
        set
    }

    pub async fn len(&self) -> usize {
        self.len
    }

    pub async fn is_empty(&self) -> bool {
        self.len == 0
    }

    #[inline]
    pub async fn get(&self, token: TokenId) -> bool {
        let idx = token.get() as usize;
        debug_assert!(idx < self.len);
        (self.words[idx / 64] >> (idx % 64)) & 1 != 0
    }

    #[inline]
    pub async fn set(&mut self, token: TokenId, value: bool) {
        let idx = token.get() as usize;
        debug_assert!(idx < self.len);
        let mask = 1u64 << (idx % 64);
        if value {
            self.words[idx / 64] |= mask;
        } else {
            self.words[idx / 64] &= !mask;
        }
    }

    /// 🎭️ Sets every bit `0..len` (trailing bits beyond `len` in the final word stay zero).
    pub async fn fill(&mut self) {
        let full_words = self.len / 64;
        self.words[..full_words].fill(u64::MAX);
        let rem = self.len % 64;
        if rem > 0 {
            self.words[full_words] = (1u64 << rem) - 1;
        }
    }

    pub async fn clear_all(&mut self) {
        self.words.fill(0);
    }

    /// 🎭️ In-place `self &= other`.
    pub async fn and_with(&mut self, other: &TokenBitset) {
        debug_assert_eq!(self.len, other.len);
        for (a, b) in self.words.iter_mut().zip(other.words.iter()) {
            *a &= *b;
        }
    }

    /// 🎭️ In-place `self |= other`.
    pub async fn or_with(&mut self, other: &TokenBitset) {
        debug_assert_eq!(self.len, other.len);
        for (a, b) in self.words.iter_mut().zip(other.words.iter()) {
            *a |= *b;
        }
    }

    /// 🎭️ In-place `self &= !other`.
    pub async fn and_not_with(&mut self, other: &TokenBitset) {
        debug_assert_eq!(self.len, other.len);
        for (a, b) in self.words.iter_mut().zip(other.words.iter()) {
            *a &= !*b;
        }
    }

    pub async fn count_ones(&self) -> usize {
        self.words.iter().map(|w| w.count_ones() as usize).sum()
    }

    pub async fn is_all_zero(&self) -> bool {
        self.words.iter().all(|&w| w == 0)
    }

    /// 🎭️ Lowest set bit, word-skipping past all-zero words.
    pub async fn first_set(&self) -> Option<TokenId> {
        for (word_idx, &word) in self.words.iter().enumerate() {
            if word != 0 {
                let bit = word.trailing_zeros() as usize;
                return Some(TokenId::new((word_idx * 64 + bit) as u32));
            }
        }
        None
    }

    /// 🎭️ Iterates set bits in ascending order, skipping whole zero words at a time.
    pub async fn iter_ones(&self) -> impl Iterator<Item = TokenId> + '_ {
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
// #endregion 🔖️Bitset

// #region 🔖️Rng
/// 🎲️ Which sub-stream of randomness a draw belongs to, so unrelated concerns (selection noise vs.
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

/// 🎲️ Identifies one independent random stream by the ids that produced it — never by batch slot,
/// so a continuous-batching reorder never changes which bits a sequence draws.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct StreamKey {
    pub request: u64,
    pub sequence: u64,
    pub beam: u32,
    pub candidate: u32,
    pub purpose: StreamPurpose,
}

async fn mix64(x: u64) -> u64 {
    semio_framework_geometry::random::SplitMix64::new(x).next_u64()
}

async fn stream_seed(key: StreamKey) -> u64 {
    let mut acc = mix64(key.request).await;
    acc = mix64(acc ^ mix64(key.sequence).await).await;
    acc = mix64(acc ^ mix64(key.beam as u64).await).await;
    acc = mix64(acc ^ mix64(key.candidate as u64).await).await;
    acc = mix64(acc ^ mix64(key.purpose as u64).await).await;
    acc
}

/// 🎲️ Which concrete generator produced a [`RngSnapshot`], so `restore` can reject a snapshot
/// meant for the other kind instead of silently reinterpreting its words.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum RngKind {
    Counter,
    Xoshiro,
}

/// 🎲️ Portable capture of a generator's internal state, text-serializable for [`SequenceState`]
/// checkpoints.
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct RngSnapshot {
    pub kind: RngKind,
    pub words: [u64; 4],
}

impl RngSnapshot {
    /// 🎲️ Compact `kind:hex:hex:hex:hex` text form.
    pub async fn to_text(&self) -> String {
        let kind = match self.kind {
            RngKind::Counter => "counter",
            RngKind::Xoshiro => "xoshiro",
        };
        format!("{kind}:{:016x}:{:016x}:{:016x}:{:016x}", self.words[0], self.words[1], self.words[2], self.words[3])
    }

    /// 🎲️ Inverse of [`RngSnapshot::to_text`].
    pub async fn from_text(text: &str) -> Result<Self, SamplingError> {
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

/// 🎲️ Object-safe source of randomness handed to samplers/warpers/search algorithms. Every
/// implementation must be splittable into independent child streams keyed by [`StreamKey`] alone
/// (never by call order), which is what keeps continuous-batching reorders and speculative
/// verification bit-reproducible.
#[dyn_enum]
pub trait RandomSource {
    async fn next_u64(&mut self) -> u64;
    /// 🎲️ Derives an independent child stream from `(self, key)` — order-independent across calls.
    async fn split(&self, key: StreamKey) -> RandomSources;
    async fn snapshot(&self) -> RngSnapshot;
    async fn restore(&mut self, snapshot: &RngSnapshot) -> Result<(), SamplingError>;

    /// 🎲️ Uniform `f64` in `[0, 1)`.
    async fn next_f64(&mut self) -> f64 {
        (self.next_u64().await >> 11) as f64 * (1.0 / (1u64 << 53) as f64)
    }

    /// 🎲️ Uniform `f64` in `(0, 1]` — safe as the argument to `ln()`, unlike `next_f64`.
    async fn next_f64_open01(&mut self) -> f64 {
        (((self.next_u64().await >> 11) + 1) as f64) * (1.0 / (1u64 << 53) as f64)
    }

    /// 🎲️ Uniform `u64` in `[lo, hi)` via rejection sampling (no modulo bias).
    async fn next_range(&mut self, lo: u64, hi: u64) -> u64 {
        debug_assert!(hi >= lo, "next_range: hi must be >= lo");
        let range = hi - lo;
        if range == 0 {
            return lo;
        }
        let limit = u64::MAX - (u64::MAX % range);
        loop {
            let x = self.next_u64().await;
            if x < limit {
                return lo + x % range;
            }
        }
    }

    /// 🎲️ Standard Gumbel(0, 1) draw, `-ln(-ln(u))` for `u` in `(0, 1]`.
    async fn gumbel(&mut self) -> f64 {
        let u = self.next_f64_open01().await;
        -(-u.ln()).ln()
    }
}

/// 🎲️ Default splittable [`RandomSource`]: a counter-based generator (double [`mix64`] of
/// `key ^ mix64(counter)`, Philox-lite) chosen over a stepped generator specifically because
/// splitting never advances or depends on the parent's step count — two sequences split from the
/// same parent at different times still get independent, order-irrelevant streams.
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct CounterRng {
    key: u64,
    ctr: u64,
}

impl CounterRng {
    /// 🎲️ A root stream from a plain seed, with no [`StreamKey`] semantics (tests, standalone use).
    pub async fn from_seed(seed: u64) -> Self {
        Self { key: mix64(seed).await, ctr: 0 }
    }

    /// 🎲️ A root stream combining a request-level seed with a full [`StreamKey`] in one step.
    pub async fn from_root(root_seed: u64, key: StreamKey) -> Self {
        Self { key: mix64(mix64(root_seed).await ^ stream_seed(key).await).await, ctr: 0 }
    }
}

impl RandomSource for CounterRng {
    async fn next_u64(&mut self) -> u64 {
        let ctr = self.ctr;
        self.ctr = self.ctr.wrapping_add(1);
        mix64(self.key ^ mix64(ctr).await).await
    }

    async fn split(&self, key: StreamKey) -> RandomSources {
        Self { key: mix64(self.key ^ stream_seed(key).await).await, ctr: 0 }.into()
    }

    async fn snapshot(&self) -> RngSnapshot {
        RngSnapshot { kind: RngKind::Counter, words: [self.key, self.ctr, 0, 0] }
    }

    async fn restore(&mut self, snapshot: &RngSnapshot) -> Result<(), SamplingError> {
        if snapshot.kind != RngKind::Counter {
            return Err(SamplingError::Corrupted { reason: "rng snapshot kind mismatch: expected counter" });
        }
        self.key = snapshot.words[0];
        self.ctr = snapshot.words[1];
        Ok(())
    }
}

/// 🎲️ [`RandomSource`] adapter over [`semio_framework_geometry::random::Rng`] (xoshiro256**), for callers who
/// want that generator's statistical profile instead of the default counter-based stream.
pub struct XoshiroSource(semio_framework_geometry::random::Rng);

impl XoshiroSource {
    pub async fn from_seed(seed: u64) -> Self {
        Self(semio_framework_geometry::random::Rng::from_seed(seed))
    }
}

impl RandomSource for XoshiroSource {
    async fn next_u64(&mut self) -> u64 {
        self.0.next_u64()
    }

    async fn split(&self, key: StreamKey) -> RandomSources {
        let state = self.0.state();
        let seed = mix64(state[0] ^ state[1] ^ stream_seed(key).await).await;
        Self::from_seed(seed).await.into()
    }

    async fn snapshot(&self) -> RngSnapshot {
        RngSnapshot { kind: RngKind::Xoshiro, words: self.0.state() }
    }

    async fn restore(&mut self, snapshot: &RngSnapshot) -> Result<(), SamplingError> {
        if snapshot.kind != RngKind::Xoshiro {
            return Err(SamplingError::Corrupted { reason: "rng snapshot kind mismatch: expected xoshiro" });
        }
        self.0 = semio_framework_geometry::random::Rng::from_state(snapshot.words);
        Ok(())
    }
}

dyn_enum_close! {
    pub enum RandomSources: RandomSource {
        Counter(CounterRng),
        Xoshiro(XoshiroSource),
    }
}
// #endregion 🔖️Rng

// #region 🔖️Vocabulary
/// 📖️ Static facts about the token space a [`SamplingConfig`] samples over.
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
    /// 📖️ A vocabulary of `size` tokens with no special tokens configured.
    pub async fn new(size: usize) -> Self {
        Self { size, eos: Vec::new(), bos: None, pad: None, unk: None, special: TokenBitset::new_empty(size).await }
    }

    pub async fn with_eos(mut self, eos: Vec<TokenId>) -> Self {
        self.eos = eos;
        self
    }

    pub async fn with_bos(mut self, bos: TokenId) -> Self {
        self.bos = Some(bos);
        self
    }

    pub async fn with_pad(mut self, pad: TokenId) -> Self {
        self.pad = Some(pad);
        self
    }

    pub async fn with_unk(mut self, unk: TokenId) -> Self {
        self.unk = Some(unk);
        self
    }

    /// 📖️ Marks `tokens` as special (suppressible via `ProcessorSpec::SuppressSpecial`).
    pub async fn with_special(mut self, tokens: &[TokenId]) -> Self {
        for &token in tokens {
            self.special.set(token, true).await;
        }
        self
    }

    pub async fn is_eos(&self, token: TokenId) -> bool {
        self.eos.contains(&token)
    }

    /// 📖️ Errors unless `len` matches this vocabulary's declared size.
    pub async fn validate_logits_len(&self, len: usize) -> Result<(), SamplingError> {
        if len != self.size {
            Err(SamplingError::VocabMismatch { expected: self.size, actual: len })
        } else {
            Ok(())
        }
    }
}

/// 📖️ Maps [`TokenId`]s to their surface-form bytes, for constraints and stop matching that
/// operate on generated text rather than raw ids.
#[dyn_enum]
pub trait TokenTextAdapter {
    async fn vocab_size(&self) -> usize;
    /// 📖️ Raw (possibly partial-UTF-8) bytes of one token; `None` for byte-less special tokens.
    async fn token_bytes(&self, token: TokenId) -> Option<&[u8]>;
    /// 📖️ Stable hash of the whole token table, used to key automaton-state×token caches so a
    /// swapped tokenizer can never silently reuse another tokenizer's cached transitions.
    async fn fingerprint(&self) -> u64;
}

/// 📖️ Owning [`TokenTextAdapter`] over a copied `Vec<Vec<u8>>` token table — owned rather than
/// borrowed (unlike the `&[&[u8]]` this was built from) so it can sit in [`TokenTextAdapters`]
/// without a struct-level lifetime: `dyn_enum_close!`'s enum-name grammar has no generic/lifetime
/// parameter slot (verified against `🔀️dispatch/🦀️.rs`'s `DynEnumInput::parse`, which
/// reads a bare `Ident` for the enum name). This type is test-only in this crate today (never
/// constructed outside `#[cfg(test)]`), so the one-time copy in [`SliceTextAdapter::new`] costs
/// nothing on any real path.
pub struct SliceTextAdapter {
    tokens: Vec<Vec<u8>>,
}

impl SliceTextAdapter {
    pub async fn new(tokens: &[&[u8]]) -> Self {
        Self { tokens: tokens.iter().map(|t| t.to_vec()).collect() }
    }
}

impl TokenTextAdapter for SliceTextAdapter {
    async fn vocab_size(&self) -> usize {
        self.tokens.len()
    }

    async fn token_bytes(&self, token: TokenId) -> Option<&[u8]> {
        self.tokens.get(token.get() as usize).map(Vec::as_slice)
    }

    async fn fingerprint(&self) -> u64 {
        // 📖️ FNV-1a over every token's bytes with a separator byte between entries so `["ab","c"]`
        // and `["a","bc"]` never collide.
        let mut hash = 0xcbf2_9ce4_8422_2325u64;
        for token in &self.tokens {
            for &b in token {
                hash = (hash ^ b as u64).wrapping_mul(0x0000_0100_0000_01b3);
            }
            hash = (hash ^ 0xFF).wrapping_mul(0x0000_0100_0000_01b3);
        }
        hash
    }
}

dyn_enum_close! {
    pub enum TokenTextAdapters: TokenTextAdapter {
        Slice(SliceTextAdapter),
    }
}
// #endregion 🔖️Vocabulary

pub async fn cumulative_from_probs(probs: &[f32]) -> Vec<f64> {
    let mut out = Vec::with_capacity(probs.len());
    let mut sum = KahanSum::new().await;
    for &p in probs {
        sum.add(p as f64).await;
        out.push(sum.value().await);
    }
    if let Some(last) = out.last_mut() {
        *last = 1.0;
    }
    out
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
