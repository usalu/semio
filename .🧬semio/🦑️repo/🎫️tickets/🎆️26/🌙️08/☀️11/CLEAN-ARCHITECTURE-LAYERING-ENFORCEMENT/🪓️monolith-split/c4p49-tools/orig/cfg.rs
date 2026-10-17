//! 🎚️ Token-sampling configuration: schedules, sampling methods, processor/constraint/stop specs, the sampling config and its builder, and step results.

use semio_framework_math_sampling_vocabulary::{Accum, DistributionHealth, ErrorMode, FallbackAction, FinishReason, JsonValue, SamplingError, SamplingLimits, SanitizePolicy, StepIndex, StreamKey, TokenId, write_json};

// #region 🔖️Schedules
/// 📅️ What a [`Schedule`] is evaluated against at one sampling step.
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct ScheduleInput {
    pub step: StepIndex,
    pub generated_len: usize,
    pub last_entropy: Option<f64>,
}

/// 📅️ A parameter value that may vary over the course of generation. Every warper/penalty knob
/// that the feature tree calls out as schedulable is `Schedule`-typed in [`ProcessorSpec`].
///
/// `PartialEq` compares `Callback` variants by function-pointer identity — good enough to
/// distinguish "the same fn item" from "a different one" in tests/config diffing, even though two
/// pointers to the same fn aren't guaranteed unique across monomorphizations; hence the allow.
#[allow(unpredictable_function_pointer_comparisons)]
#[derive(Clone, PartialEq, Debug)]
pub enum Schedule {
    Constant(f64),
    Linear {
        from: f64,
        to: f64,
        over_steps: u32,
    },
    Exponential {
        from: f64,
        to: f64,
        over_steps: u32,
    },
    Cosine {
        from: f64,
        to: f64,
        over_steps: u32,
    },
    /// 📅️ Step-indexed breakpoints; the value holds at the most recent breakpoint `<= step`.
    Piecewise(Vec<(StepIndex, f64)>),
    /// 📅️ One value per generated-token position, clamped to the last entry past its length.
    ByPosition(Vec<f64>),
    EntropyScaled {
        base: f64,
        gain: f64,
        min: f64,
        max: f64,
    },
    /// 📅️ Escape hatch for host-defined logic; not text-serializable (see [`Schedule::to_json`]).
    Callback(fn(ScheduleInput) -> f64),
}

impl Schedule {
    /// 📅️ Evaluates the schedule at `input`.
    pub async fn eval(&self, input: ScheduleInput) -> f64 {
        match self {
            Self::Constant(v) => *v,
            Self::Linear { from, to, over_steps } => {
                let t = schedule_progress(input.step, *over_steps).await;
                from + (to - from) * t
            }
            Self::Exponential { from, to, over_steps } => {
                let t = schedule_progress(input.step, *over_steps).await;
                if *from > 0.0 && *to > 0.0 {
                    from * (to / from).powf(t)
                } else {
                    from + (to - from) * t
                }
            }
            Self::Cosine { from, to, over_steps } => {
                let t = schedule_progress(input.step, *over_steps).await;
                let cos_t = 0.5 * (1.0 - (core::f64::consts::PI * t).cos());
                from + (to - from) * cos_t
            }
            Self::Piecewise(pieces) => {
                let mut value = pieces.first().map_or(0.0, |(_, v)| *v);
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

    /// 📅️ Structured form for config serialization; `Callback` encodes as a marker that
    /// deliberately fails to round-trip (see [`Schedule::from_json`]).
    pub async fn to_json(&self) -> JsonValue {
        let obj = |pairs: Vec<(&str, JsonValue)>| JsonValue::Object(pairs.into_iter().map(|(k, v)| (k.to_string(), v)).collect());
        match self {
            Self::Constant(v) => obj(vec![("kind", JsonValue::Str("constant".into())), ("value", JsonValue::Num(*v))]),
            Self::Linear { from, to, over_steps } => obj(vec![("kind", JsonValue::Str("linear".into())), ("from", JsonValue::Num(*from)), ("to", JsonValue::Num(*to)), ("over_steps", JsonValue::Num(*over_steps as f64))]),
            Self::Exponential { from, to, over_steps } => obj(vec![("kind", JsonValue::Str("exponential".into())), ("from", JsonValue::Num(*from)), ("to", JsonValue::Num(*to)), ("over_steps", JsonValue::Num(*over_steps as f64))]),
            Self::Cosine { from, to, over_steps } => obj(vec![("kind", JsonValue::Str("cosine".into())), ("from", JsonValue::Num(*from)), ("to", JsonValue::Num(*to)), ("over_steps", JsonValue::Num(*over_steps as f64))]),
            Self::Piecewise(pieces) => obj(vec![("kind", JsonValue::Str("piecewise".into())), ("pieces", JsonValue::Array(pieces.iter().map(|(s, v)| JsonValue::Array(vec![JsonValue::Num(s.get() as f64), JsonValue::Num(*v)])).collect()))]),
            Self::ByPosition(values) => obj(vec![("kind", JsonValue::Str("by_position".into())), ("values", JsonValue::Array(values.iter().map(|v| JsonValue::Num(*v)).collect()))]),
            Self::EntropyScaled { base, gain, min, max } => obj(vec![("kind", JsonValue::Str("entropy_scaled".into())), ("base", JsonValue::Num(*base)), ("gain", JsonValue::Num(*gain)), ("min", JsonValue::Num(*min)), ("max", JsonValue::Num(*max))]),
            Self::Callback(_) => obj(vec![("kind", JsonValue::Str("callback".into()))]),
        }
    }

    /// 📅️ Inverse of [`Schedule::to_json`]; rejects `"callback"` since function pointers are not
    /// recoverable from serialized data.
    pub async fn from_json(value: &JsonValue) -> Result<Self, SamplingError> {
        // 🔀️ `let num = |key| value.get(key).and_then(JsonValue::as_f64)` no longer type-checks
        // once `JsonValue::get`/`as_f64` are `async fn` — `Option::and_then` takes a plain
        // `FnOnce`, not an async one, and a captured-`value` sync closure cannot itself `.await`.
        // A nested `async fn` taking `value` explicitly replaces it (O1 de-dyn, math-dedyn).
        async fn num(value: &JsonValue, key: &'static str) -> Result<f64, SamplingError> {
            let found = match value.get(key).await {
                Some(v) => v.as_f64().await,
                None => None,
            };
            found.ok_or(SamplingError::Corrupted { reason: "schedule missing numeric field" })
        }
        let kind = match value.get("kind").await {
            Some(v) => v.as_str().await,
            None => None,
        }
        .ok_or(SamplingError::Corrupted { reason: "schedule missing kind" })?;
        match kind {
            "constant" => Ok(Self::Constant(num(value, "value").await?)),
            "linear" => Ok(Self::Linear { from: num(value, "from").await?, to: num(value, "to").await?, over_steps: num(value, "over_steps").await? as u32 }),
            "exponential" => Ok(Self::Exponential { from: num(value, "from").await?, to: num(value, "to").await?, over_steps: num(value, "over_steps").await? as u32 }),
            "cosine" => Ok(Self::Cosine { from: num(value, "from").await?, to: num(value, "to").await?, over_steps: num(value, "over_steps").await? as u32 }),
            "piecewise" => {
                let pieces = match value.get("pieces").await {
                    Some(v) => v.as_array().await,
                    None => None,
                }
                .ok_or(SamplingError::Corrupted { reason: "piecewise schedule missing pieces" })?;
                let mut out = Vec::with_capacity(pieces.len());
                for piece in pieces {
                    let pair = piece.as_array().await.ok_or(SamplingError::Corrupted { reason: "piecewise entry must be a pair" })?;
                    let step = match pair.first() {
                        Some(v) => v.as_f64().await,
                        None => None,
                    };
                    let v = match pair.get(1) {
                        Some(v) => v.as_f64().await,
                        None => None,
                    };
                    let (Some(step), Some(v)) = (step, v) else {
                        return Err(SamplingError::Corrupted { reason: "piecewise entry must be [step, value]" });
                    };
                    out.push((StepIndex::new(step as u32), v));
                }
                Ok(Self::Piecewise(out))
            }
            "by_position" => {
                let values = match value.get("values").await {
                    Some(v) => v.as_array().await,
                    None => None,
                }
                .ok_or(SamplingError::Corrupted { reason: "by_position schedule missing values" })?;
                let mut out = Vec::with_capacity(values.len());
                for v in values {
                    out.push(v.as_f64().await.ok_or(SamplingError::Corrupted { reason: "by_position value must be numeric" })?);
                }
                Ok(Self::ByPosition(out))
            }
            "entropy_scaled" => Ok(Self::EntropyScaled { base: num(value, "base").await?, gain: num(value, "gain").await?, min: num(value, "min").await?, max: num(value, "max").await? }),
            "callback" => Err(SamplingError::Corrupted { reason: "callback schedules cannot be deserialized" }),
            _ => Err(SamplingError::Corrupted { reason: "unknown schedule kind" }),
        }
    }
}

async fn schedule_progress(step: StepIndex, over_steps: u32) -> f64 {
    if over_steps == 0 {
        1.0
    } else {
        (step.get() as f64 / over_steps as f64).min(1.0)
    }
}
// #endregion 🔖️Schedules

// #region 🔖️Config
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
    Temperature {
        value: Schedule,
    },
    DynamicTemperature {
        base: Schedule,
        entropy_gain: f64,
        min: f64,
        max: f64,
    },
    TopK {
        k: Schedule,
        min_keep: usize,
    },
    TopP {
        p: Schedule,
        min_keep: usize,
    },
    MinP {
        p: Schedule,
        min_keep: usize,
    },
    Typical {
        mass: Schedule,
        min_keep: usize,
    },
    LocallyTypical {
        mass: Schedule,
        min_keep: usize,
    },
    TailFree {
        z: Schedule,
        min_keep: usize,
    },
    Epsilon {
        cutoff: Schedule,
        min_keep: usize,
    },
    Eta {
        cutoff: Schedule,
        min_keep: usize,
    },
    TopA {
        power: Schedule,
        min_keep: usize,
    },
    RankTruncation {
        max_rank: usize,
    },
    AdaptiveTruncation {
        target_entropy: Option<f64>,
        target_effective_count: Option<f64>,
    },
    RepetitionPenalty {
        penalty: f32,
        scope: PenaltyScope,
    },
    PresencePenalty {
        penalty: f32,
        scope: PenaltyScope,
    },
    FrequencyPenalty {
        penalty: f32,
        scope: PenaltyScope,
    },
    DecayingPenalty {
        penalty: f32,
        window: usize,
        half_life: f64,
        scope: PenaltyScope,
    },
    /// ⚙️ `class_tokens[c]` lists the token ids in class `c`; `factors[c]` is that class's
    /// multiplicative penalty factor (same lengths, index-aligned).
    TokenClassPenalty {
        class_tokens: Vec<Vec<TokenId>>,
        factors: Vec<f32>,
    },
    NoRepeatNgram {
        n: usize,
    },
    PhrasePenalty {
        phrases: Vec<Vec<TokenId>>,
        penalty: f32,
    },
    LogitBiasSparse {
        entries: Vec<(TokenId, f32)>,
    },
    LogitBiasDense {
        values: Vec<f32>,
    },
    AllowTokens {
        tokens: Vec<TokenId>,
    },
    ForbidTokens {
        tokens: Vec<TokenId>,
    },
    SuppressSpecial,
    BadWords {
        phrases: Vec<Vec<TokenId>>,
    },
    SequenceEncouragement {
        phrases: Vec<Vec<TokenId>>,
        bonus: f32,
    },
    Mirostat {
        version: MirostatVersion,
        target_surprise: f64,
        learning_rate: f64,
    },
    EntropyPid {
        target: f64,
        kp: f64,
        ki: f64,
        kd: f64,
    },
    RepetitionController {
        window: usize,
        threshold: f64,
        boost: f64,
    },
    ConfidenceController {
        low_entropy: f64,
        high_entropy: f64,
        low_temp: f64,
        high_temp: f64,
    },
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
    pub async fn validate(&self) -> Result<(), SamplingError> {
        self.limits.validate().await?;
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
            validate_processor_spec(processor, &self.limits).await?;
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
    pub async fn fingerprint(&self) -> u64 {
        let text = write_json(&self.to_json().await).await;
        let mut hash = 0xcbf2_9ce4_8422_2325u64;
        for &b in text.as_bytes() {
            hash = (hash ^ b as u64).wrapping_mul(0x0000_0100_0000_01b3);
        }
        hash
    }

    /// ⚙️ Greedy decoding, no penalties or truncation — the deterministic baseline preset.
    pub async fn precise() -> Self {
        Self { method: SamplingMethod::Greedy { tie_break: TieBreak::LowestTokenId }, ..Self::default() }
    }

    /// ⚙️ Temperature 0.7, top-p 0.9, min-p 0.05, mild repetition penalty — a general-purpose
    /// preset balancing coherence and variety.
    pub async fn balanced() -> Self {
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
    pub async fn creative() -> Self {
        Self {
            method: SamplingMethod::Multinomial { strategy: MultinomialStrategy::CdfBinarySearch },
            processors: vec![ProcessorSpec::Temperature { value: Schedule::Constant(1.0) }, ProcessorSpec::TopK { k: Schedule::Constant(100.0), min_keep: 1 }, ProcessorSpec::TopP { p: Schedule::Constant(0.95), min_keep: 1 }],
            ..Self::default()
        }
    }

    /// ⚙️ Fixed seed, greedy, tiny token budget — for reproducible tests, not production use.
    pub async fn deterministic_test() -> Self {
        Self { method: SamplingMethod::Greedy { tie_break: TieBreak::LowestTokenId }, seed: 42, max_tokens: 64, ..Self::default() }
    }

    /// ⚙️ Structured JSON form (versioned: top-level `"version": 1`).
    pub async fn to_json(&self) -> JsonValue {
        // 🔀️ Sequential `.await` loop, not `.iter().map(processor_spec_to_json).collect()` — a
        // sync iterator closure cannot call an `async fn` (O1 de-dyn, math-dedyn).
        let mut processors = Vec::with_capacity(self.processors.len());
        for p in &self.processors {
            processors.push(processor_spec_to_json(p).await);
        }
        JsonValue::Object(vec![
            ("version".into(), JsonValue::Num(1.0)),
            ("method".into(), sampling_method_to_json(&self.method).await),
            ("processors".into(), JsonValue::Array(processors)),
            (
                "error_mode".into(),
                JsonValue::Str(match self.error_mode {
                    ErrorMode::Strict => "strict".into(),
                    ErrorMode::Permissive => "permissive".into(),
                }),
            ),
            ("seed".into(), JsonValue::Num(self.seed as f64)),
            ("candidate_count".into(), JsonValue::Num(self.candidate_count as f64)),
            ("min_tokens".into(), JsonValue::Num(self.min_tokens as f64)),
            ("max_tokens".into(), JsonValue::Num(self.max_tokens as f64)),
        ])
    }

    /// ⚙️ Parses the `"version": 1` JSON form back into a config, tolerating unknown top-level
    /// fields (forward compatibility) but rejecting a version it does not recognize.
    pub async fn from_json(value: &JsonValue) -> Result<Self, SamplingError> {
        // 🔀️ See `Schedule::from_json`'s note: `and_then`/iterator-adapter closures cannot call
        // `async fn`s, so lookups are `match`ed and array elements loop-and-`.await`ed explicitly
        // (O1 de-dyn, math-dedyn).
        async fn num(value: &JsonValue, key: &'static str, default: f64) -> f64 {
            match value.get(key).await {
                Some(v) => v.as_f64().await,
                None => None,
            }
            .unwrap_or(default)
        }
        let version = match value.get("version").await {
            Some(v) => v.as_f64().await,
            None => None,
        }
        .ok_or(SamplingError::Corrupted { reason: "config missing version" })?;
        if version as u32 != 1 {
            return Err(SamplingError::SerializationVersion { expected: 1, actual: version as u32 });
        }
        let method = match value.get("method").await {
            Some(v) => sampling_method_from_json(v).await,
            None => Err(SamplingError::Corrupted { reason: "config missing method" }),
        }?;
        let processor_specs = match value.get("processors").await {
            Some(v) => v.as_array().await,
            None => None,
        }
        .ok_or(SamplingError::Corrupted { reason: "config missing processors" })?;
        let mut processors = Vec::with_capacity(processor_specs.len());
        for spec in processor_specs {
            processors.push(processor_spec_from_json(spec).await?);
        }
        let error_mode_str = match value.get("error_mode").await {
            Some(v) => v.as_str().await,
            None => None,
        };
        let error_mode = match error_mode_str {
            Some("strict") => ErrorMode::Strict,
            Some("permissive") | None => ErrorMode::Permissive,
            Some(_) => return Err(SamplingError::Corrupted { reason: "unknown error_mode" }),
        };
        Ok(Self {
            method,
            processors,
            error_mode,
            seed: num(value, "seed", 0.0).await as u64,
            candidate_count: num(value, "candidate_count", 1.0).await as usize,
            min_tokens: num(value, "min_tokens", 0.0).await as usize,
            max_tokens: num(value, "max_tokens", 4_096.0).await as usize,
            ..Self::default()
        })
    }
}

async fn validate_processor_spec(spec: &ProcessorSpec, limits: &SamplingLimits) -> Result<(), SamplingError> {
    match spec {
        ProcessorSpec::NoRepeatNgram { n } if *n == 0 || *n > limits.max_ngram_order => {
            return Err(SamplingError::LimitExceeded { limit: "max_ngram_order" });
        }
        ProcessorSpec::TokenClassPenalty { class_tokens, factors } if class_tokens.len() != factors.len() => {
            return Err(SamplingError::InvalidConfig { field: "token_class_penalty", reason: "class_tokens and factors must have equal length" });
        }
        ProcessorSpec::RankTruncation { max_rank } if *max_rank == 0 => {
            return Err(SamplingError::InvalidConfig { field: "rank_truncation.max_rank", reason: "must be >= 1" });
        }
        _ => {}
    }
    Ok(())
}

async fn sampling_method_to_json(method: &SamplingMethod) -> JsonValue {
    let obj = |pairs: Vec<(&str, JsonValue)>| JsonValue::Object(pairs.into_iter().map(|(k, v)| (k.to_string(), v)).collect());
    match method {
        SamplingMethod::Greedy { tie_break } => obj(vec![("kind", JsonValue::Str("greedy".into())), ("tie_break", tie_break_to_json(*tie_break).await)]),
        SamplingMethod::Multinomial { strategy } => obj(vec![("kind", JsonValue::Str("multinomial".into())), ("strategy", multinomial_strategy_to_json(*strategy).await)]),
        SamplingMethod::GumbelMax => obj(vec![("kind", JsonValue::Str("gumbel_max".into()))]),
        SamplingMethod::GumbelTopK { k } => obj(vec![("kind", JsonValue::Str("gumbel_top_k".into())), ("k", JsonValue::Num(*k as f64))]),
    }
}

async fn sampling_method_from_json(value: &JsonValue) -> Result<SamplingMethod, SamplingError> {
    let kind = match value.get("kind").await {
        Some(v) => v.as_str().await,
        None => None,
    }
    .ok_or(SamplingError::Corrupted { reason: "method missing kind" })?;
    match kind {
        "greedy" => Ok(SamplingMethod::Greedy { tie_break: tie_break_from_json(value.get("tie_break").await).await? }),
        "multinomial" => Ok(SamplingMethod::Multinomial { strategy: multinomial_strategy_from_json(value.get("strategy").await).await? }),
        "gumbel_max" => Ok(SamplingMethod::GumbelMax),
        "gumbel_top_k" => {
            let k = match value.get("k").await {
                Some(v) => v.as_f64().await,
                None => None,
            };
            Ok(SamplingMethod::GumbelTopK { k: k.unwrap_or(1.0) as usize })
        }
        _ => Err(SamplingError::Corrupted { reason: "unknown sampling method kind" }),
    }
}

async fn tie_break_to_json(tie_break: TieBreak) -> JsonValue {
    JsonValue::Str(
        match tie_break {
            TieBreak::LowestTokenId => "lowest_token_id",
            TieBreak::HighestTokenId => "highest_token_id",
            TieBreak::FirstSeen => "first_seen",
        }
        .into(),
    )
}

async fn tie_break_from_json(value: Option<&JsonValue>) -> Result<TieBreak, SamplingError> {
    let as_str = match value {
        Some(v) => v.as_str().await,
        None => None,
    };
    match as_str {
        Some("lowest_token_id") | None => Ok(TieBreak::LowestTokenId),
        Some("highest_token_id") => Ok(TieBreak::HighestTokenId),
        Some("first_seen") => Ok(TieBreak::FirstSeen),
        Some(_) => Err(SamplingError::Corrupted { reason: "unknown tie_break" }),
    }
}

async fn multinomial_strategy_to_json(strategy: MultinomialStrategy) -> JsonValue {
    JsonValue::Str(
        match strategy {
            MultinomialStrategy::CdfBinarySearch => "cdf_binary_search",
            MultinomialStrategy::LinearScan => "linear_scan",
            MultinomialStrategy::Alias => "alias",
        }
        .into(),
    )
}

async fn multinomial_strategy_from_json(value: Option<&JsonValue>) -> Result<MultinomialStrategy, SamplingError> {
    let as_str = match value {
        Some(v) => v.as_str().await,
        None => None,
    };
    match as_str {
        Some("cdf_binary_search") | None => Ok(MultinomialStrategy::CdfBinarySearch),
        Some("linear_scan") => Ok(MultinomialStrategy::LinearScan),
        Some("alias") => Ok(MultinomialStrategy::Alias),
        Some(_) => Err(SamplingError::Corrupted { reason: "unknown multinomial strategy" }),
    }
}

async fn penalty_scope_to_json(scope: PenaltyScope) -> JsonValue {
    JsonValue::Str(
        match scope {
            PenaltyScope::PromptAndGenerated => "prompt_and_generated",
            PenaltyScope::GeneratedOnly => "generated_only",
            PenaltyScope::PromptOnly => "prompt_only",
        }
        .into(),
    )
}

async fn penalty_scope_from_json(value: Option<&JsonValue>) -> Result<PenaltyScope, SamplingError> {
    let as_str = match value {
        Some(v) => v.as_str().await,
        None => None,
    };
    match as_str {
        Some("prompt_and_generated") | None => Ok(PenaltyScope::PromptAndGenerated),
        Some("generated_only") => Ok(PenaltyScope::GeneratedOnly),
        Some("prompt_only") => Ok(PenaltyScope::PromptOnly),
        Some(_) => Err(SamplingError::Corrupted { reason: "unknown penalty scope" }),
    }
}

async fn processor_spec_to_json(spec: &ProcessorSpec) -> JsonValue {
    let obj = |pairs: Vec<(&str, JsonValue)>| JsonValue::Object(pairs.into_iter().map(|(k, v)| (k.to_string(), v)).collect());
    let tokens_json = |tokens: &[TokenId]| JsonValue::Array(tokens.iter().map(|t| JsonValue::Num(t.get() as f64)).collect());
    let phrases_json = |phrases: &[Vec<TokenId>]| JsonValue::Array(phrases.iter().map(|p| tokens_json(p)).collect());
    match spec {
        ProcessorSpec::Temperature { value } => obj(vec![("kind", JsonValue::Str("temperature".into())), ("value", value.to_json().await)]),
        ProcessorSpec::DynamicTemperature { base, entropy_gain, min, max } => {
            obj(vec![("kind", JsonValue::Str("dynamic_temperature".into())), ("base", base.to_json().await), ("entropy_gain", JsonValue::Num(*entropy_gain)), ("min", JsonValue::Num(*min)), ("max", JsonValue::Num(*max))])
        }
        ProcessorSpec::TopK { k, min_keep } => obj(vec![("kind", JsonValue::Str("top_k".into())), ("k", k.to_json().await), ("min_keep", JsonValue::Num(*min_keep as f64))]),
        ProcessorSpec::TopP { p, min_keep } => obj(vec![("kind", JsonValue::Str("top_p".into())), ("p", p.to_json().await), ("min_keep", JsonValue::Num(*min_keep as f64))]),
        ProcessorSpec::MinP { p, min_keep } => obj(vec![("kind", JsonValue::Str("min_p".into())), ("p", p.to_json().await), ("min_keep", JsonValue::Num(*min_keep as f64))]),
        ProcessorSpec::Typical { mass, min_keep } => obj(vec![("kind", JsonValue::Str("typical".into())), ("mass", mass.to_json().await), ("min_keep", JsonValue::Num(*min_keep as f64))]),
        ProcessorSpec::LocallyTypical { mass, min_keep } => obj(vec![("kind", JsonValue::Str("locally_typical".into())), ("mass", mass.to_json().await), ("min_keep", JsonValue::Num(*min_keep as f64))]),
        ProcessorSpec::TailFree { z, min_keep } => obj(vec![("kind", JsonValue::Str("tail_free".into())), ("z", z.to_json().await), ("min_keep", JsonValue::Num(*min_keep as f64))]),
        ProcessorSpec::Epsilon { cutoff, min_keep } => obj(vec![("kind", JsonValue::Str("epsilon".into())), ("cutoff", cutoff.to_json().await), ("min_keep", JsonValue::Num(*min_keep as f64))]),
        ProcessorSpec::Eta { cutoff, min_keep } => obj(vec![("kind", JsonValue::Str("eta".into())), ("cutoff", cutoff.to_json().await), ("min_keep", JsonValue::Num(*min_keep as f64))]),
        ProcessorSpec::TopA { power, min_keep } => obj(vec![("kind", JsonValue::Str("top_a".into())), ("power", power.to_json().await), ("min_keep", JsonValue::Num(*min_keep as f64))]),
        ProcessorSpec::RankTruncation { max_rank } => obj(vec![("kind", JsonValue::Str("rank_truncation".into())), ("max_rank", JsonValue::Num(*max_rank as f64))]),
        ProcessorSpec::AdaptiveTruncation { target_entropy, target_effective_count } => {
            obj(vec![("kind", JsonValue::Str("adaptive_truncation".into())), ("target_entropy", target_entropy.map_or(JsonValue::Null, JsonValue::Num)), ("target_effective_count", target_effective_count.map_or(JsonValue::Null, JsonValue::Num))])
        }
        ProcessorSpec::RepetitionPenalty { penalty, scope } => obj(vec![("kind", JsonValue::Str("repetition_penalty".into())), ("penalty", JsonValue::Num(*penalty as f64)), ("scope", penalty_scope_to_json(*scope).await)]),
        ProcessorSpec::PresencePenalty { penalty, scope } => obj(vec![("kind", JsonValue::Str("presence_penalty".into())), ("penalty", JsonValue::Num(*penalty as f64)), ("scope", penalty_scope_to_json(*scope).await)]),
        ProcessorSpec::FrequencyPenalty { penalty, scope } => obj(vec![("kind", JsonValue::Str("frequency_penalty".into())), ("penalty", JsonValue::Num(*penalty as f64)), ("scope", penalty_scope_to_json(*scope).await)]),
        ProcessorSpec::DecayingPenalty { penalty, window, half_life, scope } => obj(vec![
            ("kind", JsonValue::Str("decaying_penalty".into())),
            ("penalty", JsonValue::Num(*penalty as f64)),
            ("window", JsonValue::Num(*window as f64)),
            ("half_life", JsonValue::Num(*half_life)),
            ("scope", penalty_scope_to_json(*scope).await),
        ]),
        ProcessorSpec::TokenClassPenalty { class_tokens, factors } => {
            obj(vec![("kind", JsonValue::Str("token_class_penalty".into())), ("class_tokens", phrases_json(class_tokens)), ("factors", JsonValue::Array(factors.iter().map(|f| JsonValue::Num(*f as f64)).collect()))])
        }
        ProcessorSpec::NoRepeatNgram { n } => obj(vec![("kind", JsonValue::Str("no_repeat_ngram".into())), ("n", JsonValue::Num(*n as f64))]),
        ProcessorSpec::PhrasePenalty { phrases, penalty } => obj(vec![("kind", JsonValue::Str("phrase_penalty".into())), ("phrases", phrases_json(phrases)), ("penalty", JsonValue::Num(*penalty as f64))]),
        ProcessorSpec::LogitBiasSparse { entries } => {
            obj(vec![("kind", JsonValue::Str("logit_bias_sparse".into())), ("entries", JsonValue::Array(entries.iter().map(|(t, b)| JsonValue::Array(vec![JsonValue::Num(t.get() as f64), JsonValue::Num(*b as f64)])).collect()))])
        }
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
        ProcessorSpec::EntropyPid { target, kp, ki, kd } => obj(vec![("kind", JsonValue::Str("entropy_pid".into())), ("target", JsonValue::Num(*target)), ("kp", JsonValue::Num(*kp)), ("ki", JsonValue::Num(*ki)), ("kd", JsonValue::Num(*kd))]),
        ProcessorSpec::RepetitionController { window, threshold, boost } => {
            obj(vec![("kind", JsonValue::Str("repetition_controller".into())), ("window", JsonValue::Num(*window as f64)), ("threshold", JsonValue::Num(*threshold)), ("boost", JsonValue::Num(*boost))])
        }
        ProcessorSpec::ConfidenceController { low_entropy, high_entropy, low_temp, high_temp } => obj(vec![
            ("kind", JsonValue::Str("confidence_controller".into())),
            ("low_entropy", JsonValue::Num(*low_entropy)),
            ("high_entropy", JsonValue::Num(*high_entropy)),
            ("low_temp", JsonValue::Num(*low_temp)),
            ("high_temp", JsonValue::Num(*high_temp)),
        ]),
    }
}

async fn processor_spec_from_json(value: &JsonValue) -> Result<ProcessorSpec, SamplingError> {
    // 🔀️ `and_then`/`filter_map`/`map` closures cannot call `async fn`s (`JsonValue::get`/
    // `as_str`/`as_f64`/`as_array` are all `async fn` under the universal-async decree) — every
    // helper below is a nested `async fn` taking `value` explicitly, and every array walk is an
    // explicit `.await`ing loop rather than an iterator-adapter chain (O1 de-dyn, math-dedyn).
    async fn opt_str<'a>(value: &'a JsonValue, key: &'static str) -> Option<&'a str> {
        match value.get(key).await {
            Some(v) => v.as_str().await,
            None => None,
        }
    }
    async fn opt_num(value: &JsonValue, key: &'static str) -> Option<f64> {
        match value.get(key).await {
            Some(v) => v.as_f64().await,
            None => None,
        }
    }
    async fn num(value: &JsonValue, key: &'static str, default: f64) -> f64 {
        opt_num(value, key).await.unwrap_or(default)
    }
    async fn opt_array<'a>(value: &'a JsonValue, key: &'static str) -> Option<&'a [JsonValue]> {
        match value.get(key).await {
            Some(v) => v.as_array().await,
            None => None,
        }
    }
    async fn schedule(value: &JsonValue, key: &'static str) -> Result<Schedule, SamplingError> {
        let field = value.get(key).await.ok_or(SamplingError::Corrupted { reason: "processor missing schedule field" })?;
        Schedule::from_json(field).await
    }
    async fn f32_array(value: &JsonValue, key: &'static str) -> Vec<f32> {
        let Some(arr) = opt_array(value, key).await else { return Vec::new() };
        let mut out = Vec::with_capacity(arr.len());
        for item in arr {
            if let Some(n) = item.as_f64().await {
                out.push(n as f32);
            }
        }
        out
    }
    async fn tokens(value: &JsonValue, key: &'static str) -> Vec<TokenId> {
        let Some(arr) = opt_array(value, key).await else { return Vec::new() };
        let mut out = Vec::with_capacity(arr.len());
        for item in arr {
            if let Some(n) = item.as_f64().await {
                out.push(TokenId::new(n as u32));
            }
        }
        out
    }
    async fn phrases(value: &JsonValue, key: &'static str) -> Vec<Vec<TokenId>> {
        let Some(arr) = opt_array(value, key).await else { return Vec::new() };
        let mut out = Vec::with_capacity(arr.len());
        for item in arr {
            let Some(inner) = item.as_array().await else { continue };
            let mut phrase = Vec::with_capacity(inner.len());
            for n in inner {
                if let Some(f) = n.as_f64().await {
                    phrase.push(TokenId::new(f as u32));
                }
            }
            out.push(phrase);
        }
        out
    }

    let kind = opt_str(value, "kind").await.ok_or(SamplingError::Corrupted { reason: "processor missing kind" })?;
    match kind {
        "temperature" => Ok(ProcessorSpec::Temperature { value: schedule(value, "value").await? }),
        "dynamic_temperature" => Ok(ProcessorSpec::DynamicTemperature { base: schedule(value, "base").await?, entropy_gain: num(value, "entropy_gain", 0.0).await, min: num(value, "min", 0.0).await, max: num(value, "max", 2.0).await }),
        "top_k" => Ok(ProcessorSpec::TopK { k: schedule(value, "k").await?, min_keep: num(value, "min_keep", 1.0).await as usize }),
        "top_p" => Ok(ProcessorSpec::TopP { p: schedule(value, "p").await?, min_keep: num(value, "min_keep", 1.0).await as usize }),
        "min_p" => Ok(ProcessorSpec::MinP { p: schedule(value, "p").await?, min_keep: num(value, "min_keep", 1.0).await as usize }),
        "typical" => Ok(ProcessorSpec::Typical { mass: schedule(value, "mass").await?, min_keep: num(value, "min_keep", 1.0).await as usize }),
        "locally_typical" => Ok(ProcessorSpec::LocallyTypical { mass: schedule(value, "mass").await?, min_keep: num(value, "min_keep", 1.0).await as usize }),
        "tail_free" => Ok(ProcessorSpec::TailFree { z: schedule(value, "z").await?, min_keep: num(value, "min_keep", 1.0).await as usize }),
        "epsilon" => Ok(ProcessorSpec::Epsilon { cutoff: schedule(value, "cutoff").await?, min_keep: num(value, "min_keep", 1.0).await as usize }),
        "eta" => Ok(ProcessorSpec::Eta { cutoff: schedule(value, "cutoff").await?, min_keep: num(value, "min_keep", 1.0).await as usize }),
        "top_a" => Ok(ProcessorSpec::TopA { power: schedule(value, "power").await?, min_keep: num(value, "min_keep", 1.0).await as usize }),
        "rank_truncation" => Ok(ProcessorSpec::RankTruncation { max_rank: num(value, "max_rank", 1.0).await as usize }),
        "adaptive_truncation" => Ok(ProcessorSpec::AdaptiveTruncation { target_entropy: opt_num(value, "target_entropy").await, target_effective_count: opt_num(value, "target_effective_count").await }),
        "repetition_penalty" => Ok(ProcessorSpec::RepetitionPenalty { penalty: num(value, "penalty", 1.0).await as f32, scope: penalty_scope_from_json(value.get("scope").await).await? }),
        "presence_penalty" => Ok(ProcessorSpec::PresencePenalty { penalty: num(value, "penalty", 0.0).await as f32, scope: penalty_scope_from_json(value.get("scope").await).await? }),
        "frequency_penalty" => Ok(ProcessorSpec::FrequencyPenalty { penalty: num(value, "penalty", 0.0).await as f32, scope: penalty_scope_from_json(value.get("scope").await).await? }),
        "decaying_penalty" => Ok(ProcessorSpec::DecayingPenalty {
            penalty: num(value, "penalty", 0.0).await as f32,
            window: num(value, "window", 16.0).await as usize,
            half_life: num(value, "half_life", 1.0).await,
            scope: penalty_scope_from_json(value.get("scope").await).await?,
        }),
        "token_class_penalty" => Ok(ProcessorSpec::TokenClassPenalty { class_tokens: phrases(value, "class_tokens").await, factors: f32_array(value, "factors").await }),
        "no_repeat_ngram" => Ok(ProcessorSpec::NoRepeatNgram { n: num(value, "n", 3.0).await as usize }),
        "phrase_penalty" => Ok(ProcessorSpec::PhrasePenalty { phrases: phrases(value, "phrases").await, penalty: num(value, "penalty", 0.0).await as f32 }),
        "logit_bias_sparse" => {
            let mut entries = Vec::new();
            if let Some(arr) = opt_array(value, "entries").await {
                for pair_value in arr {
                    let Some(pair) = pair_value.as_array().await else { continue };
                    let (Some(first), Some(second)) = (pair.first(), pair.get(1)) else { continue };
                    let (Some(token), Some(bias)) = (first.as_f64().await, second.as_f64().await) else { continue };
                    entries.push((TokenId::new(token as u32), bias as f32));
                }
            }
            Ok(ProcessorSpec::LogitBiasSparse { entries })
        }
        "logit_bias_dense" => Ok(ProcessorSpec::LogitBiasDense { values: f32_array(value, "values").await }),
        "allow_tokens" => Ok(ProcessorSpec::AllowTokens { tokens: tokens(value, "tokens").await }),
        "forbid_tokens" => Ok(ProcessorSpec::ForbidTokens { tokens: tokens(value, "tokens").await }),
        "suppress_special" => Ok(ProcessorSpec::SuppressSpecial),
        "bad_words" => Ok(ProcessorSpec::BadWords { phrases: phrases(value, "phrases").await }),
        "sequence_encouragement" => Ok(ProcessorSpec::SequenceEncouragement { phrases: phrases(value, "phrases").await, bonus: num(value, "bonus", 0.0).await as f32 }),
        "mirostat" => Ok(ProcessorSpec::Mirostat {
            version: if opt_str(value, "version").await == Some("v1") { MirostatVersion::V1 } else { MirostatVersion::V2 },
            target_surprise: num(value, "target_surprise", 5.0).await,
            learning_rate: num(value, "learning_rate", 0.1).await,
        }),
        "entropy_pid" => Ok(ProcessorSpec::EntropyPid { target: num(value, "target", 2.0).await, kp: num(value, "kp", 0.1).await, ki: num(value, "ki", 0.0).await, kd: num(value, "kd", 0.0).await }),
        "repetition_controller" => Ok(ProcessorSpec::RepetitionController { window: num(value, "window", 16.0).await as usize, threshold: num(value, "threshold", 0.5).await, boost: num(value, "boost", 0.2).await }),
        "confidence_controller" => {
            Ok(ProcessorSpec::ConfidenceController { low_entropy: num(value, "low_entropy", 0.5).await, high_entropy: num(value, "high_entropy", 3.0).await, low_temp: num(value, "low_temp", 0.5).await, high_temp: num(value, "high_temp", 1.2).await })
        }
        _ => Err(SamplingError::Corrupted { reason: "unknown processor kind" }),
    }
}

/// ⚙️ Chainable constructor for [`SamplingConfig`]; `build()` runs full validation once.
#[derive(Clone, Debug)]
#[derive(Default)]
pub struct SamplingConfigBuilder {
    config: SamplingConfig,
}

impl SamplingConfigBuilder {
    pub async fn new() -> Self {
        Self { config: SamplingConfig::default() }
    }

    pub async fn method(mut self, method: SamplingMethod) -> Self {
        self.config.method = method;
        self
    }

    pub async fn processor(mut self, processor: ProcessorSpec) -> Self {
        self.config.processors.push(processor);
        self
    }

    pub async fn error_mode(mut self, mode: ErrorMode) -> Self {
        self.config.error_mode = mode;
        self
    }

    pub async fn sanitize(mut self, policy: SanitizePolicy) -> Self {
        self.config.sanitize = policy;
        self
    }

    pub async fn accum(mut self, accum: Accum) -> Self {
        self.config.accum = accum;
        self
    }

    pub async fn seed(mut self, seed: u64) -> Self {
        self.config.seed = seed;
        self
    }

    pub async fn candidate_count(mut self, count: usize) -> Self {
        self.config.candidate_count = count;
        self
    }

    pub async fn min_tokens(mut self, min_tokens: usize) -> Self {
        self.config.min_tokens = min_tokens;
        self
    }

    pub async fn max_tokens(mut self, max_tokens: usize) -> Self {
        self.config.max_tokens = max_tokens;
        self
    }

    pub async fn forced(mut self, forced: ForcedSpec) -> Self {
        self.config.forced = forced;
        self
    }

    pub async fn stops(mut self, stops: StopSpec) -> Self {
        self.config.stops = stops;
        self
    }

    pub async fn constraint(mut self, constraint: ConstraintSpec) -> Self {
        self.config.constraints.push(constraint);
        self
    }

    pub async fn logprobs(mut self, logprobs: LogprobsSpec) -> Self {
        self.config.logprobs = logprobs;
        self
    }

    pub async fn limits(mut self, limits: SamplingLimits) -> Self {
        self.config.limits = limits;
        self
    }

    pub async fn diagnostics(mut self, diagnostics: DiagnosticsSpec) -> Self {
        self.config.diagnostics = diagnostics;
        self
    }

    /// ⚙️ Validates and returns the finished config.
    pub async fn build(self) -> Result<SamplingConfig, SamplingError> {
        self.config.validate().await?;
        Ok(self.config)
    }
}
// #endregion 🔖️Config

// #region 🔖️Candidates
/// 🏅️ One scored token, at whatever pipeline stage produced it (pre- or post-truncation, or the
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

/// 🏅️ Alternative log-probabilities reported alongside a selection, before and/or after
/// truncation warpers ran (per [`LogprobsSpec`]).
#[derive(Clone, PartialEq, Debug, Default)]
pub struct TopLogprobs {
    pub pre_truncation: Vec<Candidate>,
    pub post_truncation: Vec<Candidate>,
}

/// 🏅️ Optional per-step numerical/pipeline diagnostics (only populated when
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

/// 🏅️ Everything one [`sample_step_stateless`] call (or, later, a stateful engine step) returns.
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
// #endregion 🔖️Candidates

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
