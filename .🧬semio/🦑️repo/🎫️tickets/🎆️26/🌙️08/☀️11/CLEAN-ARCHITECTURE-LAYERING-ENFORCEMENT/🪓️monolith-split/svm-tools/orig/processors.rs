//! 🎛️ Token-sampling logits processors and samplers: warpers, penalties, biases, length controls, selection samplers and adaptive controllers.

#![allow(async_fn_in_trait)]

use semio_framework_dispatch_macros::{dyn_enum, dyn_enum_close};
use semio_framework_math_sampling_vocabulary::{RandomSource, RandomSources, SamplingError, TokenBitset, TokenId, cdf_binary_search, entropy_nats};
use semio_framework_math_sampling_config::{Candidate, ForcedSpec, MirostatVersion, MultinomialStrategy, PenaltyScope, ProcessorSpec, SamplingMethod, Schedule, TieBreak};
use semio_framework_math_sampling_workspace::{Distribution, LogitsWorkspace, ProcessorKind, SelectionBuffer, StateMark, StepView, schedule_input};

/// 🔌️ One step of the logits pipeline: transforms `ws` in place (mask, soft penalty, or
/// truncation), and optionally commits per-sequence effects once a token is definitively accepted.
#[dyn_enum]
pub trait LogitsProcessor {
    async fn name(&self) -> &'static str;
    async fn kind(&self) -> ProcessorKind;
    async fn process(&mut self, view: &StepView<'_>, ws: &mut LogitsWorkspace) -> Result<(), SamplingError>;
    async fn commit(&mut self, _view: &StepView<'_>, _token: TokenId) {}
    async fn save(&mut self) -> StateMark {
        StateMark::default()
    }
    async fn rollback_to(&mut self, _mark: StateMark) {}
    async fn reset(&mut self) {}
    async fn fork(&self) -> LogitsProcessors;
}

/// 🔌️ Selects one or more tokens from the final [`Distribution`]; must write at least one
/// candidate into `out` or return an error.
#[dyn_enum]
pub trait TokenSampler {
    async fn name(&self) -> &'static str;
    async fn sample(&mut self, view: &StepView<'_>, dist: &Distribution<'_>, rng: &mut RandomSources, out: &mut SelectionBuffer) -> Result<(), SamplingError>;
    async fn fork(&self) -> TokenSamplers;
}

// #region 🔖️Warpers
/// 🌡️ `logit / temperature`; `temperature <= 0` collapses to greedy via [`LogitsWorkspace::collapse_live_to_argmax`].
pub struct Temperature {
    pub value: Schedule,
}

impl LogitsProcessor for Temperature {
    async fn name(&self) -> &'static str {
        "temperature"
    }
    async fn kind(&self) -> ProcessorKind {
        ProcessorKind::SoftPenalty
    }
    async fn process(&mut self, view: &StepView<'_>, ws: &mut LogitsWorkspace) -> Result<(), SamplingError> {
        let value = self.value.eval(schedule_input(view).await).await;
        if value <= 0.0 {
            ws.collapse_live_to_argmax().await;
        } else {
            ws.scale_processed_over_live(1.0 / value as f32).await;
        }
        Ok(())
    }
    async fn fork(&self) -> LogitsProcessors {
        (Self { value: self.value.clone() }).into()
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
    async fn name(&self) -> &'static str {
        "dynamic_temperature"
    }
    async fn kind(&self) -> ProcessorKind {
        ProcessorKind::SoftPenalty
    }
    async fn process(&mut self, view: &StepView<'_>, ws: &mut LogitsWorkspace) -> Result<(), SamplingError> {
        ws.softmax_over_live().await;
        let entropy = entropy_nats(ws.probs().await).await;
        let temp = (self.base.eval(schedule_input(view).await).await + self.entropy_gain * entropy).clamp(self.min, self.max);
        if temp <= 0.0 {
            ws.collapse_live_to_argmax().await;
        } else {
            ws.scale_processed_over_live(1.0 / temp as f32).await;
        }
        Ok(())
    }
    async fn fork(&self) -> LogitsProcessors {
        (Self { base: self.base.clone(), entropy_gain: self.entropy_gain, min: self.min, max: self.max }).into()
    }
}

/// 🌡️ Keeps exactly the top `k` live tokens by probability.
pub struct TopK {
    pub k: Schedule,
    pub min_keep: usize,
}

impl LogitsProcessor for TopK {
    async fn name(&self) -> &'static str {
        "top_k"
    }
    async fn kind(&self) -> ProcessorKind {
        ProcessorKind::Truncation
    }
    async fn process(&mut self, view: &StepView<'_>, ws: &mut LogitsWorkspace) -> Result<(), SamplingError> {
        let k = self.k.eval(schedule_input(view).await).await.round().max(1.0) as usize;
        ws.sort_live_by_prob_desc().await;
        ws.truncate_live_to(k, self.min_keep).await;
        Ok(())
    }
    async fn fork(&self) -> LogitsProcessors {
        (Self { k: self.k.clone(), min_keep: self.min_keep }).into()
    }
}

/// 🌡️ Nucleus sampling: retains the smallest prefix (by descending probability) whose cumulative
/// mass reaches `p`.
pub struct TopP {
    pub p: Schedule,
    pub min_keep: usize,
}

impl LogitsProcessor for TopP {
    async fn name(&self) -> &'static str {
        "top_p"
    }
    async fn kind(&self) -> ProcessorKind {
        ProcessorKind::Truncation
    }
    async fn process(&mut self, view: &StepView<'_>, ws: &mut LogitsWorkspace) -> Result<(), SamplingError> {
        let p = self.p.eval(schedule_input(view).await).await.clamp(0.0, 1.0);
        ws.sort_live_by_prob_desc().await;
        let probs = ws.probs().await;
        let mut cumulative = 0.0f64;
        let mut keep = probs.len();
        for (i, &pr) in probs.iter().enumerate() {
            cumulative += pr as f64;
            if cumulative >= p {
                keep = i + 1;
                break;
            }
        }
        ws.truncate_live_to(keep, self.min_keep).await;
        Ok(())
    }
    async fn fork(&self) -> LogitsProcessors {
        (Self { p: self.p.clone(), min_keep: self.min_keep }).into()
    }
}

/// 🌡️ Retains tokens whose probability is at least `p` times the live set's maximum probability.
pub struct MinP {
    pub p: Schedule,
    pub min_keep: usize,
}

impl LogitsProcessor for MinP {
    async fn name(&self) -> &'static str {
        "min_p"
    }
    async fn kind(&self) -> ProcessorKind {
        ProcessorKind::Truncation
    }
    async fn process(&mut self, view: &StepView<'_>, ws: &mut LogitsWorkspace) -> Result<(), SamplingError> {
        let p = self.p.eval(schedule_input(view).await).await.clamp(0.0, 1.0) as f32;
        ws.sort_live_by_prob_desc().await;
        let probs = ws.probs().await;
        let threshold = probs.first().copied().unwrap_or(0.0) * p;
        let keep = probs.iter().take_while(|&&pr| pr >= threshold).count().max(1);
        ws.truncate_live_to(keep, self.min_keep).await;
        Ok(())
    }
    async fn fork(&self) -> LogitsProcessors {
        (Self { p: self.p.clone(), min_keep: self.min_keep }).into()
    }
}

/// 🌡️ Shared implementation for [`Typical`] and [`LocallyTypical`]: ranks tokens by absolute
/// deviation of their surprisal (`-ln p`) from the live set's entropy and retains the smallest
/// such prefix whose cumulative mass reaches `mass`.
async fn apply_typical_truncation(ws: &mut LogitsWorkspace, mass: f64, min_keep: usize) {
    ws.softmax_over_live().await;
    let n = ws.live().await.len();
    if n == 0 {
        return;
    }
    let probs = ws.probs().await.to_vec();
    let entropy = entropy_nats(&probs).await;
    let live = ws.live().await.to_vec();
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
    ws.set_live(kept).await;
}

/// 🌡️ Locally typical sampling (global variant): see [`apply_typical_truncation`].
pub struct Typical {
    pub mass: Schedule,
    pub min_keep: usize,
}

impl LogitsProcessor for Typical {
    async fn name(&self) -> &'static str {
        "typical"
    }
    async fn kind(&self) -> ProcessorKind {
        ProcessorKind::Truncation
    }
    async fn process(&mut self, view: &StepView<'_>, ws: &mut LogitsWorkspace) -> Result<(), SamplingError> {
        let mass = self.mass.eval(schedule_input(view).await).await.clamp(0.0, 1.0);
        apply_typical_truncation(ws, mass, self.min_keep).await;
        Ok(())
    }
    async fn fork(&self) -> LogitsProcessors {
        (Self { mass: self.mass.clone(), min_keep: self.min_keep }).into()
    }
}

/// 🌡️ Locally typical sampling; this single-step engine applies the same rule as [`Typical`]
/// (the "local" distinction only matters across multiple denoising passes, not one logits step).
pub struct LocallyTypical {
    pub mass: Schedule,
    pub min_keep: usize,
}

impl LogitsProcessor for LocallyTypical {
    async fn name(&self) -> &'static str {
        "locally_typical"
    }
    async fn kind(&self) -> ProcessorKind {
        ProcessorKind::Truncation
    }
    async fn process(&mut self, view: &StepView<'_>, ws: &mut LogitsWorkspace) -> Result<(), SamplingError> {
        let mass = self.mass.eval(schedule_input(view).await).await.clamp(0.0, 1.0);
        apply_typical_truncation(ws, mass, self.min_keep).await;
        Ok(())
    }
    async fn fork(&self) -> LogitsProcessors {
        (Self { mass: self.mass.clone(), min_keep: self.min_keep }).into()
    }
}

/// 🌡️ Tail-free sampling: cuts where the cumulative normalized second derivative of the sorted
/// probability curve reaches `z`.
pub struct TailFree {
    pub z: Schedule,
    pub min_keep: usize,
}

impl LogitsProcessor for TailFree {
    async fn name(&self) -> &'static str {
        "tail_free"
    }
    async fn kind(&self) -> ProcessorKind {
        ProcessorKind::Truncation
    }
    async fn process(&mut self, view: &StepView<'_>, ws: &mut LogitsWorkspace) -> Result<(), SamplingError> {
        let z = self.z.eval(schedule_input(view).await).await.clamp(0.0, 1.0);
        ws.sort_live_by_prob_desc().await;
        let probs = ws.probs().await.to_vec();
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
        ws.truncate_live_to(keep, self.min_keep).await;
        Ok(())
    }
    async fn fork(&self) -> LogitsProcessors {
        (Self { z: self.z.clone(), min_keep: self.min_keep }).into()
    }
}

/// 🌡️ Drops any token with probability below an absolute `cutoff`.
pub struct EpsilonCutoff {
    pub cutoff: Schedule,
    pub min_keep: usize,
}

impl LogitsProcessor for EpsilonCutoff {
    async fn name(&self) -> &'static str {
        "epsilon"
    }
    async fn kind(&self) -> ProcessorKind {
        ProcessorKind::Truncation
    }
    async fn process(&mut self, view: &StepView<'_>, ws: &mut LogitsWorkspace) -> Result<(), SamplingError> {
        let cutoff = self.cutoff.eval(schedule_input(view).await).await.max(0.0);
        ws.sort_live_by_prob_desc().await;
        let probs = ws.probs().await;
        let keep = probs.iter().take_while(|&&p| p as f64 >= cutoff).count().max(1);
        ws.truncate_live_to(keep, self.min_keep).await;
        Ok(())
    }
    async fn fork(&self) -> LogitsProcessors {
        (Self { cutoff: self.cutoff.clone(), min_keep: self.min_keep }).into()
    }
}

/// 🌡️ Entropy-adaptive cutoff: `eta = min(epsilon, sqrt(epsilon) * exp(-entropy))`.
pub struct EtaCutoff {
    pub cutoff: Schedule,
    pub min_keep: usize,
}

impl LogitsProcessor for EtaCutoff {
    async fn name(&self) -> &'static str {
        "eta"
    }
    async fn kind(&self) -> ProcessorKind {
        ProcessorKind::Truncation
    }
    async fn process(&mut self, view: &StepView<'_>, ws: &mut LogitsWorkspace) -> Result<(), SamplingError> {
        let epsilon = self.cutoff.eval(schedule_input(view).await).await.max(1e-12);
        ws.sort_live_by_prob_desc().await;
        let probs = ws.probs().await.to_vec();
        let entropy = entropy_nats(&probs).await;
        let eta = epsilon.min(epsilon.sqrt() * (-entropy).exp());
        let keep = probs.iter().take_while(|&&p| p as f64 >= eta).count().max(1);
        ws.truncate_live_to(keep, self.min_keep).await;
        Ok(())
    }
    async fn fork(&self) -> LogitsProcessors {
        (Self { cutoff: self.cutoff.clone(), min_keep: self.min_keep }).into()
    }
}

/// 🌡️ Keeps tokens with `prob >= power * max_prob^2`.
pub struct TopA {
    pub power: Schedule,
    pub min_keep: usize,
}

impl LogitsProcessor for TopA {
    async fn name(&self) -> &'static str {
        "top_a"
    }
    async fn kind(&self) -> ProcessorKind {
        ProcessorKind::Truncation
    }
    async fn process(&mut self, view: &StepView<'_>, ws: &mut LogitsWorkspace) -> Result<(), SamplingError> {
        let a = self.power.eval(schedule_input(view).await).await.max(0.0);
        ws.sort_live_by_prob_desc().await;
        let probs = ws.probs().await;
        let max_p = probs.first().copied().unwrap_or(0.0) as f64;
        let threshold = a * max_p * max_p;
        let keep = probs.iter().take_while(|&&p| p as f64 >= threshold).count().max(1);
        ws.truncate_live_to(keep, self.min_keep).await;
        Ok(())
    }
    async fn fork(&self) -> LogitsProcessors {
        (Self { power: self.power.clone(), min_keep: self.min_keep }).into()
    }
}

/// 🌡️ Keeps only the top `max_rank` tokens by probability (a fixed-count variant of [`TopK`]
/// used when rank alone, not a schedule, determines the cutoff).
pub struct RankTruncation {
    pub max_rank: usize,
}

impl LogitsProcessor for RankTruncation {
    async fn name(&self) -> &'static str {
        "rank_truncation"
    }
    async fn kind(&self) -> ProcessorKind {
        ProcessorKind::Truncation
    }
    async fn process(&mut self, _view: &StepView<'_>, ws: &mut LogitsWorkspace) -> Result<(), SamplingError> {
        ws.sort_live_by_prob_desc().await;
        ws.truncate_live_to(self.max_rank, 1).await;
        Ok(())
    }
    async fn fork(&self) -> LogitsProcessors {
        (Self { max_rank: self.max_rank }).into()
    }
}

/// 🌡️ Grows the kept prefix (by descending probability) until the renormalized subset's entropy
/// drops to `target_entropy`, or its effective candidate count reaches `target_effective_count`.
pub struct AdaptiveTruncation {
    pub target_entropy: Option<f64>,
    pub target_effective_count: Option<f64>,
}

impl LogitsProcessor for AdaptiveTruncation {
    async fn name(&self) -> &'static str {
        "adaptive_truncation"
    }
    async fn kind(&self) -> ProcessorKind {
        ProcessorKind::Truncation
    }
    async fn process(&mut self, _view: &StepView<'_>, ws: &mut LogitsWorkspace) -> Result<(), SamplingError> {
        ws.sort_live_by_prob_desc().await;
        let probs = ws.probs().await.to_vec();
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
        ws.truncate_live_to(keep, 1).await;
        Ok(())
    }
    async fn fork(&self) -> LogitsProcessors {
        (Self { target_entropy: self.target_entropy, target_effective_count: self.target_effective_count }).into()
    }
}

/// 🌡️ Builds a [`LogitsProcessor`] from a [`ProcessorSpec`]; exhaustive over every variant once
/// the warpers (`🔖️Warpers`), penalties (`🔖️Penalties`), biases (`🔖️Biases`), and adaptive
/// samplers (`🔖️Adaptive`) regions have all landed.
pub async fn build_processor(spec: &ProcessorSpec) -> Result<LogitsProcessors, SamplingError> {
    match spec {
        ProcessorSpec::Temperature { value } => Ok((Temperature { value: value.clone() }).into()),
        ProcessorSpec::DynamicTemperature { base, entropy_gain, min, max } => Ok((DynamicTemperature { base: base.clone(), entropy_gain: *entropy_gain, min: *min, max: *max }).into()),
        ProcessorSpec::TopK { k, min_keep } => Ok((TopK { k: k.clone(), min_keep: *min_keep }).into()),
        ProcessorSpec::TopP { p, min_keep } => Ok((TopP { p: p.clone(), min_keep: *min_keep }).into()),
        ProcessorSpec::MinP { p, min_keep } => Ok((MinP { p: p.clone(), min_keep: *min_keep }).into()),
        ProcessorSpec::Typical { mass, min_keep } => Ok((Typical { mass: mass.clone(), min_keep: *min_keep }).into()),
        ProcessorSpec::LocallyTypical { mass, min_keep } => Ok((LocallyTypical { mass: mass.clone(), min_keep: *min_keep }).into()),
        ProcessorSpec::TailFree { z, min_keep } => Ok((TailFree { z: z.clone(), min_keep: *min_keep }).into()),
        ProcessorSpec::Epsilon { cutoff, min_keep } => Ok((EpsilonCutoff { cutoff: cutoff.clone(), min_keep: *min_keep }).into()),
        ProcessorSpec::Eta { cutoff, min_keep } => Ok((EtaCutoff { cutoff: cutoff.clone(), min_keep: *min_keep }).into()),
        ProcessorSpec::TopA { power, min_keep } => Ok((TopA { power: power.clone(), min_keep: *min_keep }).into()),
        ProcessorSpec::RankTruncation { max_rank } => Ok((RankTruncation { max_rank: *max_rank }).into()),
        ProcessorSpec::AdaptiveTruncation { target_entropy, target_effective_count } => Ok((AdaptiveTruncation { target_entropy: *target_entropy, target_effective_count: *target_effective_count }).into()),
        ProcessorSpec::RepetitionPenalty { penalty, scope } => Ok((RepetitionPenalty::new(*penalty, *scope).await).into()),
        ProcessorSpec::PresencePenalty { penalty, scope } => Ok((PresencePenalty::new(*penalty, *scope).await).into()),
        ProcessorSpec::FrequencyPenalty { penalty, scope } => Ok((FrequencyPenalty::new(*penalty, *scope).await).into()),
        ProcessorSpec::DecayingPenalty { penalty, window, half_life, scope } => Ok((DecayingPenalty::new(*penalty, *window, *half_life, *scope).await).into()),
        ProcessorSpec::TokenClassPenalty { class_tokens, factors } => Ok((TokenClassPenalty::new(class_tokens.clone(), factors.clone()).await).into()),
        ProcessorSpec::NoRepeatNgram { n } => Ok((NoRepeatNgram::new(*n).await).into()),
        ProcessorSpec::PhrasePenalty { phrases, penalty } => Ok((PhrasePenalty { phrases: phrases.clone(), penalty: *penalty }).into()),
        ProcessorSpec::LogitBiasSparse { entries } => Ok((LogitBiasSparse { entries: entries.clone() }).into()),
        ProcessorSpec::LogitBiasDense { values } => Ok((LogitBiasDense { values: values.clone() }).into()),
        ProcessorSpec::AllowTokens { tokens } => Ok((AllowTokens { tokens: tokens.clone() }).into()),
        ProcessorSpec::ForbidTokens { tokens } => Ok((ForbidTokens { tokens: tokens.clone() }).into()),
        ProcessorSpec::SuppressSpecial => Ok((SuppressSpecial).into()),
        ProcessorSpec::BadWords { phrases } => Ok((BadWords { phrases: phrases.clone() }).into()),
        ProcessorSpec::SequenceEncouragement { phrases, bonus } => Ok((SequenceEncouragement { phrases: phrases.clone(), bonus: *bonus }).into()),
        ProcessorSpec::Mirostat { version, target_surprise, learning_rate } => Ok((Mirostat::new(*version, *target_surprise, *learning_rate).await).into()),
        ProcessorSpec::EntropyPid { target, kp, ki, kd } => Ok((EntropyPid::new(*target, *kp, *ki, *kd).await).into()),
        ProcessorSpec::RepetitionController { window, threshold, boost } => Ok((RepetitionController::new(*window, *threshold, *boost).await).into()),
        ProcessorSpec::ConfidenceController { low_entropy, high_entropy, low_temp, high_temp } => Ok((ConfidenceController { low_entropy: *low_entropy, high_entropy: *high_entropy, low_temp: *low_temp, high_temp: *high_temp }).into()),
    }
}
// #endregion 🔖️Warpers

// #region 🔖️Penalties
/// ⚖️ Dense-enough token→count map for repetition-style penalties. Backed by a `HashMap` (not a
/// vocab-sized `Vec`) so processor construction never needs to know the vocabulary size.
#[derive(Clone, Default)]
pub struct FreqTable {
    counts: std::collections::HashMap<TokenId, u32>,
}

impl FreqTable {
    pub async fn new() -> Self {
        Self::default()
    }

    pub async fn count(&self, token: TokenId) -> u32 {
        self.counts.get(&token).copied().unwrap_or(0)
    }

    pub async fn increment(&mut self, token: TokenId) {
        *self.counts.entry(token).or_insert(0) += 1;
    }

    /// ⚖️ Exact inverse of [`FreqTable::increment`] — removes the map entry entirely once its
    /// count reaches zero, so `count()` and `HashMap` iteration agree on "seen at all".
    pub async fn decrement(&mut self, token: TokenId) {
        if let Some(c) = self.counts.get_mut(&token) {
            if *c <= 1 {
                self.counts.remove(&token);
            } else {
                *c -= 1;
            }
        }
    }

    pub async fn reset(&mut self) {
        self.counts.clear();
    }
}

/// ⚖️ Open-addressed (via `HashMap`) rolling-context index: maps a hash of the last `order - 1`
/// tokens to every token observed to follow that context, for [`NoRepeatNgram`]. `undo` records
/// exactly the key touched by each [`NgramIndex::record`] call (or `None` for a no-operation call), so
/// [`NgramIndex::rollback_last_n`] can undo precisely `n` prior commits.
#[derive(Clone, Default)]
pub struct NgramIndex {
    order: usize,
    table: std::collections::HashMap<u64, Vec<TokenId>>,
    undo: Vec<Option<u64>>,
}

impl NgramIndex {
    pub async fn new(order: usize) -> Self {
        Self { order, table: std::collections::HashMap::new(), undo: Vec::new() }
    }

    async fn context_hash(context: &[TokenId]) -> u64 {
        let mut hash = 0xcbf2_9ce4_8422_2325u64;
        for token in context {
            hash = (hash ^ token.get() as u64).wrapping_mul(0x0000_0100_0000_01b3);
        }
        hash
    }

    /// ⚖️ Records that `next` followed the last `order - 1` tokens of `history` (the history as it
    /// existed *before* `next`).
    pub async fn record(&mut self, history: &[TokenId], next: TokenId) {
        if self.order < 2 || history.len() + 1 < self.order {
            self.undo.push(None);
            return;
        }
        let ctx_len = self.order - 1;
        let context = &history[history.len() - ctx_len..];
        let key = Self::context_hash(context).await;
        self.table.entry(key).or_default().push(next);
        self.undo.push(Some(key));
    }

    /// ⚖️ Tokens that would recreate an already-seen `order`-gram if selected next.
    pub async fn forbidden_next(&self, history: &[TokenId]) -> &[TokenId] {
        if self.order < 2 || history.len() + 1 < self.order {
            return &[];
        }
        let ctx_len = self.order - 1;
        let context = &history[history.len() - ctx_len..];
        let key = Self::context_hash(context).await;
        self.table.get(&key).map_or(&[][..], Vec::as_slice)
    }

    pub async fn commit_count(&self) -> u64 {
        self.undo.len() as u64
    }

    pub async fn rollback_last_n(&mut self, n: usize) {
        for _ in 0..n {
            let Some(entry) = self.undo.pop() else { break };
            if let Some(key) = entry {
                if let Some(list) = self.table.get_mut(&key) {
                    list.pop();
                    if list.is_empty() {
                        self.table.remove(&key);
                    }
                }
            }
        }
    }

    pub async fn reset(&mut self) {
        self.table.clear();
        self.undo.clear();
    }
}

/// ⚖️ Sign-aware multiplicative repetition penalty: positive logits are divided by `penalty`,
/// negative logits multiplied by it — so `penalty > 1` always pushes a seen token's logit down
/// regardless of its sign.
pub struct RepetitionPenalty {
    pub penalty: f32,
    pub scope: PenaltyScope,
    counts: FreqTable,
    prompt_included: bool,
    commit_log: Vec<TokenId>,
}

impl RepetitionPenalty {
    pub async fn new(penalty: f32, scope: PenaltyScope) -> Self {
        Self { penalty, scope, counts: FreqTable::new().await, prompt_included: false, commit_log: Vec::new() }
    }

    async fn include_prompt_if_needed(&mut self, prompt: &[TokenId]) {
        if !self.prompt_included && matches!(self.scope, PenaltyScope::PromptAndGenerated | PenaltyScope::PromptOnly) {
            for &t in prompt {
                self.counts.increment(t).await;
            }
            self.prompt_included = true;
        }
    }
}

impl LogitsProcessor for RepetitionPenalty {
    async fn name(&self) -> &'static str {
        "repetition_penalty"
    }
    async fn kind(&self) -> ProcessorKind {
        ProcessorKind::SoftPenalty
    }
    async fn process(&mut self, view: &StepView<'_>, ws: &mut LogitsWorkspace) -> Result<(), SamplingError> {
        self.include_prompt_if_needed(view.prompt).await;
        let penalty = self.penalty;
        // 🔀️ `transform_processed_over_live`'s closure is a sync `FnMut` — `FreqTable::count` is
        // now `async fn`, so counts are snapshotted into a plain sync-lookup map first (O1
        // de-dyn, math-dedyn).
        let mut seen: std::collections::HashSet<TokenId> = std::collections::HashSet::new();
        for &idx in ws.live().await {
            let token = TokenId::new(idx);
            if self.counts.count(token).await > 0 {
                seen.insert(token);
            }
        }
        ws.transform_processed_over_live(|token, logit| {
            if seen.contains(&token) {
                if logit > 0.0 {
                    logit / penalty
                } else {
                    logit * penalty
                }
            } else {
                logit
            }
        })
        .await;
        Ok(())
    }
    async fn commit(&mut self, _view: &StepView<'_>, token: TokenId) {
        if matches!(self.scope, PenaltyScope::PromptAndGenerated | PenaltyScope::GeneratedOnly) {
            self.counts.increment(token).await;
        }
        self.commit_log.push(token);
    }
    async fn save(&mut self) -> StateMark {
        StateMark(self.commit_log.len() as u64)
    }
    async fn rollback_to(&mut self, mark: StateMark) {
        while self.commit_log.len() as u64 > mark.0 {
            if let Some(token) = self.commit_log.pop() {
                if matches!(self.scope, PenaltyScope::PromptAndGenerated | PenaltyScope::GeneratedOnly) {
                    self.counts.decrement(token).await;
                }
            }
        }
    }
    async fn reset(&mut self) {
        self.counts.reset().await;
        self.prompt_included = false;
        self.commit_log.clear();
    }
    async fn fork(&self) -> LogitsProcessors {
        (Self { penalty: self.penalty, scope: self.scope, counts: self.counts.clone(), prompt_included: self.prompt_included, commit_log: self.commit_log.clone() }).into()
    }
}

/// ⚖️ Fixed additive penalty applied once per distinct token that has appeared.
pub struct PresencePenalty {
    pub penalty: f32,
    pub scope: PenaltyScope,
    counts: FreqTable,
    prompt_included: bool,
    commit_log: Vec<TokenId>,
}

impl PresencePenalty {
    pub async fn new(penalty: f32, scope: PenaltyScope) -> Self {
        Self { penalty, scope, counts: FreqTable::new().await, prompt_included: false, commit_log: Vec::new() }
    }

    async fn include_prompt_if_needed(&mut self, prompt: &[TokenId]) {
        if !self.prompt_included && matches!(self.scope, PenaltyScope::PromptAndGenerated | PenaltyScope::PromptOnly) {
            for &t in prompt {
                self.counts.increment(t).await;
            }
            self.prompt_included = true;
        }
    }
}

impl LogitsProcessor for PresencePenalty {
    async fn name(&self) -> &'static str {
        "presence_penalty"
    }
    async fn kind(&self) -> ProcessorKind {
        ProcessorKind::SoftPenalty
    }
    async fn process(&mut self, view: &StepView<'_>, ws: &mut LogitsWorkspace) -> Result<(), SamplingError> {
        self.include_prompt_if_needed(view.prompt).await;
        let penalty = self.penalty;
        // 🔀️ See `RepetitionPenalty::process`'s note: `add_bias_over_live`'s closure is a sync
        // `FnMut`, so counts are snapshotted first (O1 de-dyn, math-dedyn).
        let mut seen: std::collections::HashSet<TokenId> = std::collections::HashSet::new();
        for &idx in ws.live().await {
            let token = TokenId::new(idx);
            if self.counts.count(token).await > 0 {
                seen.insert(token);
            }
        }
        ws.add_bias_over_live(|token| if seen.contains(&token) { -penalty } else { 0.0 }).await;
        Ok(())
    }
    async fn commit(&mut self, _view: &StepView<'_>, token: TokenId) {
        if matches!(self.scope, PenaltyScope::PromptAndGenerated | PenaltyScope::GeneratedOnly) {
            self.counts.increment(token).await;
        }
        self.commit_log.push(token);
    }
    async fn save(&mut self) -> StateMark {
        StateMark(self.commit_log.len() as u64)
    }
    async fn rollback_to(&mut self, mark: StateMark) {
        while self.commit_log.len() as u64 > mark.0 {
            if let Some(token) = self.commit_log.pop() {
                if matches!(self.scope, PenaltyScope::PromptAndGenerated | PenaltyScope::GeneratedOnly) {
                    self.counts.decrement(token).await;
                }
            }
        }
    }
    async fn reset(&mut self) {
        self.counts.reset().await;
        self.prompt_included = false;
        self.commit_log.clear();
    }
    async fn fork(&self) -> LogitsProcessors {
        (Self { penalty: self.penalty, scope: self.scope, counts: self.counts.clone(), prompt_included: self.prompt_included, commit_log: self.commit_log.clone() }).into()
    }
}

/// ⚖️ Additive penalty proportional to a token's occurrence count.
pub struct FrequencyPenalty {
    pub penalty: f32,
    pub scope: PenaltyScope,
    counts: FreqTable,
    prompt_included: bool,
    commit_log: Vec<TokenId>,
}

impl FrequencyPenalty {
    pub async fn new(penalty: f32, scope: PenaltyScope) -> Self {
        Self { penalty, scope, counts: FreqTable::new().await, prompt_included: false, commit_log: Vec::new() }
    }

    async fn include_prompt_if_needed(&mut self, prompt: &[TokenId]) {
        if !self.prompt_included && matches!(self.scope, PenaltyScope::PromptAndGenerated | PenaltyScope::PromptOnly) {
            for &t in prompt {
                self.counts.increment(t).await;
            }
            self.prompt_included = true;
        }
    }
}

impl LogitsProcessor for FrequencyPenalty {
    async fn name(&self) -> &'static str {
        "frequency_penalty"
    }
    async fn kind(&self) -> ProcessorKind {
        ProcessorKind::SoftPenalty
    }
    async fn process(&mut self, view: &StepView<'_>, ws: &mut LogitsWorkspace) -> Result<(), SamplingError> {
        self.include_prompt_if_needed(view.prompt).await;
        let penalty = self.penalty;
        // 🔀️ See `RepetitionPenalty::process`'s note: counts are snapshotted before the sync
        // closure (O1 de-dyn, math-dedyn).
        let mut counts_by_token: std::collections::HashMap<TokenId, u32> = std::collections::HashMap::new();
        for &idx in ws.live().await {
            let token = TokenId::new(idx);
            counts_by_token.insert(token, self.counts.count(token).await);
        }
        ws.add_bias_over_live(|token| -penalty * counts_by_token.get(&token).copied().unwrap_or(0) as f32).await;
        Ok(())
    }
    async fn commit(&mut self, _view: &StepView<'_>, token: TokenId) {
        if matches!(self.scope, PenaltyScope::PromptAndGenerated | PenaltyScope::GeneratedOnly) {
            self.counts.increment(token).await;
        }
        self.commit_log.push(token);
    }
    async fn save(&mut self) -> StateMark {
        StateMark(self.commit_log.len() as u64)
    }
    async fn rollback_to(&mut self, mark: StateMark) {
        while self.commit_log.len() as u64 > mark.0 {
            if let Some(token) = self.commit_log.pop() {
                if matches!(self.scope, PenaltyScope::PromptAndGenerated | PenaltyScope::GeneratedOnly) {
                    self.counts.decrement(token).await;
                }
            }
        }
    }
    async fn reset(&mut self) {
        self.counts.reset().await;
        self.prompt_included = false;
        self.commit_log.clear();
    }
    async fn fork(&self) -> LogitsProcessors {
        (Self { penalty: self.penalty, scope: self.scope, counts: self.counts.clone(), prompt_included: self.prompt_included, commit_log: self.commit_log.clone() }).into()
    }
}

/// ⚖️ Repetition penalty that decays exponentially with distance from the most recent occurrence,
/// over a bounded trailing `window` of generated tokens.
pub struct DecayingPenalty {
    pub penalty: f32,
    pub window: usize,
    pub half_life: f64,
    pub scope: PenaltyScope,
    history: std::collections::VecDeque<TokenId>,
    snapshots: Vec<std::collections::VecDeque<TokenId>>,
}

impl DecayingPenalty {
    pub async fn new(penalty: f32, window: usize, half_life: f64, scope: PenaltyScope) -> Self {
        Self { penalty, window, half_life, scope, history: std::collections::VecDeque::new(), snapshots: Vec::new() }
    }
}

impl LogitsProcessor for DecayingPenalty {
    async fn name(&self) -> &'static str {
        "decaying_penalty"
    }
    async fn kind(&self) -> ProcessorKind {
        ProcessorKind::SoftPenalty
    }
    async fn process(&mut self, _view: &StepView<'_>, ws: &mut LogitsWorkspace) -> Result<(), SamplingError> {
        let half_life = self.half_life.max(1e-6);
        let penalty = self.penalty as f64;
        let mut bias: std::collections::HashMap<TokenId, f32> = std::collections::HashMap::new();
        for (i, &token) in self.history.iter().rev().enumerate() {
            let distance = (i + 1) as f64;
            let decay = 0.5f64.powf(distance / half_life);
            *bias.entry(token).or_insert(0.0) += (penalty * decay) as f32;
        }
        ws.add_bias_over_live(|token| -bias.get(&token).copied().unwrap_or(0.0)).await;
        Ok(())
    }
    async fn commit(&mut self, _view: &StepView<'_>, token: TokenId) {
        self.history.push_back(token);
        if self.history.len() > self.window {
            self.history.pop_front();
        }
    }
    async fn save(&mut self) -> StateMark {
        self.snapshots.push(self.history.clone());
        StateMark((self.snapshots.len() - 1) as u64)
    }
    async fn rollback_to(&mut self, mark: StateMark) {
        if let Some(history) = self.snapshots.get(mark.0 as usize) {
            self.history = history.clone();
        }
        self.snapshots.truncate(mark.0 as usize + 1);
    }
    async fn reset(&mut self) {
        self.history.clear();
        self.snapshots.clear();
    }
    async fn fork(&self) -> LogitsProcessors {
        (Self { penalty: self.penalty, window: self.window, half_life: self.half_life, scope: self.scope, history: self.history.clone(), snapshots: self.snapshots.clone() }).into()
    }
}

/// ⚖️ Static multiplicative factor applied per token class (`class_tokens[c]` lists the tokens in
/// class `c`, `factors[c]` its factor). No per-sequence state, so `commit`/`save`/`rollback_to`
/// use their trait defaults.
pub struct TokenClassPenalty {
    class_of: std::collections::HashMap<TokenId, u16>,
    factors: Vec<f32>,
}

impl TokenClassPenalty {
    pub async fn new(class_tokens: Vec<Vec<TokenId>>, factors: Vec<f32>) -> Self {
        let mut class_of = std::collections::HashMap::new();
        for (class, tokens) in class_tokens.into_iter().enumerate() {
            for token in tokens {
                class_of.insert(token, class as u16);
            }
        }
        Self { class_of, factors }
    }
}

impl LogitsProcessor for TokenClassPenalty {
    async fn name(&self) -> &'static str {
        "token_class_penalty"
    }
    async fn kind(&self) -> ProcessorKind {
        ProcessorKind::SoftPenalty
    }
    async fn process(&mut self, _view: &StepView<'_>, ws: &mut LogitsWorkspace) -> Result<(), SamplingError> {
        let class_of = &self.class_of;
        let factors = &self.factors;
        ws.transform_processed_over_live(|token, logit| match class_of.get(&token).and_then(|&c| factors.get(c as usize)) {
            Some(&factor) => logit * factor,
            None => logit,
        })
        .await;
        Ok(())
    }
    async fn fork(&self) -> LogitsProcessors {
        (Self { class_of: self.class_of.clone(), factors: self.factors.clone() }).into()
    }
}

/// ⚖️ Hard-masks any token that would recreate an `n`-gram already present in the sequence's
/// history (prompt + generated).
pub struct NoRepeatNgram {
    pub n: usize,
    index: NgramIndex,
}

impl NoRepeatNgram {
    pub async fn new(n: usize) -> Self {
        Self { n, index: NgramIndex::new(n).await }
    }

    async fn full_history(view: &StepView<'_>) -> Vec<TokenId> {
        let mut history = Vec::with_capacity(view.prompt.len() + view.generated.len());
        history.extend_from_slice(view.prompt);
        history.extend_from_slice(view.generated);
        history
    }
}

impl LogitsProcessor for NoRepeatNgram {
    async fn name(&self) -> &'static str {
        "no_repeat_ngram"
    }
    async fn kind(&self) -> ProcessorKind {
        ProcessorKind::HardMask
    }
    async fn process(&mut self, view: &StepView<'_>, ws: &mut LogitsWorkspace) -> Result<(), SamplingError> {
        let history = Self::full_history(view).await;
        for &token in self.index.forbidden_next(&history).await {
            ws.mask_mut().await.set(token, false).await;
        }
        Ok(())
    }
    async fn commit(&mut self, view: &StepView<'_>, token: TokenId) {
        let history = Self::full_history(view).await;
        self.index.record(&history, token).await;
    }
    async fn save(&mut self) -> StateMark {
        StateMark(self.index.commit_count().await)
    }
    async fn rollback_to(&mut self, mark: StateMark) {
        let current = self.index.commit_count().await;
        if current > mark.0 {
            self.index.rollback_last_n((current - mark.0) as usize).await;
        }
    }
    async fn reset(&mut self) {
        self.index.reset().await;
    }
    async fn fork(&self) -> LogitsProcessors {
        (Self { n: self.n, index: self.index.clone() }).into()
    }
}

/// ⚖️ Penalizes the final token of any phrase whose proper prefix the generated text currently
/// ends with (discourages completing that exact phrase). Stateless: reads `view.generated`
/// directly each step.
pub struct PhrasePenalty {
    pub phrases: Vec<Vec<TokenId>>,
    pub penalty: f32,
}

impl LogitsProcessor for PhrasePenalty {
    async fn name(&self) -> &'static str {
        "phrase_penalty"
    }
    async fn kind(&self) -> ProcessorKind {
        ProcessorKind::SoftPenalty
    }
    async fn process(&mut self, view: &StepView<'_>, ws: &mut LogitsWorkspace) -> Result<(), SamplingError> {
        let generated = view.generated;
        let penalty = self.penalty;
        let mut biased: std::collections::HashMap<TokenId, f32> = std::collections::HashMap::new();
        for phrase in &self.phrases {
            let Some((&last, prefix)) = phrase.split_last() else { continue };
            if generated.len() >= prefix.len() && &generated[generated.len() - prefix.len()..] == prefix {
                *biased.entry(last).or_insert(0.0) += penalty;
            }
        }
        ws.add_bias_over_live(|token| -biased.get(&token).copied().unwrap_or(0.0)).await;
        Ok(())
    }
    async fn fork(&self) -> LogitsProcessors {
        (Self { phrases: self.phrases.clone(), penalty: self.penalty }).into()
    }
}
// #endregion 🔖️Penalties

// #region 🔖️Biases
/// 🧲️ Sparse per-token additive logit bias.
pub struct LogitBiasSparse {
    pub entries: Vec<(TokenId, f32)>,
}

impl LogitsProcessor for LogitBiasSparse {
    async fn name(&self) -> &'static str {
        "logit_bias_sparse"
    }
    async fn kind(&self) -> ProcessorKind {
        ProcessorKind::SoftPenalty
    }
    async fn process(&mut self, _view: &StepView<'_>, ws: &mut LogitsWorkspace) -> Result<(), SamplingError> {
        for &(token, bias) in &self.entries {
            ws.bias_processed(token, bias).await;
        }
        Ok(())
    }
    async fn fork(&self) -> LogitsProcessors {
        (Self { entries: self.entries.clone() }).into()
    }
}

/// 🧲️ Dense per-token additive logit bias (`values[i]` biases token `i`).
pub struct LogitBiasDense {
    pub values: Vec<f32>,
}

impl LogitsProcessor for LogitBiasDense {
    async fn name(&self) -> &'static str {
        "logit_bias_dense"
    }
    async fn kind(&self) -> ProcessorKind {
        ProcessorKind::SoftPenalty
    }
    async fn process(&mut self, _view: &StepView<'_>, ws: &mut LogitsWorkspace) -> Result<(), SamplingError> {
        for (i, &delta) in self.values.iter().enumerate() {
            ws.bias_processed(TokenId::new(i as u32), delta).await;
        }
        Ok(())
    }
    async fn fork(&self) -> LogitsProcessors {
        (Self { values: self.values.clone() }).into()
    }
}

/// 🧲️ Hard-restricts the live set to exactly `tokens` (intersected with whatever survives earlier
/// hard-mask processors).
pub struct AllowTokens {
    pub tokens: Vec<TokenId>,
}

impl LogitsProcessor for AllowTokens {
    async fn name(&self) -> &'static str {
        "allow_tokens"
    }
    async fn kind(&self) -> ProcessorKind {
        ProcessorKind::HardMask
    }
    async fn process(&mut self, _view: &StepView<'_>, ws: &mut LogitsWorkspace) -> Result<(), SamplingError> {
        let mut allow = TokenBitset::new_empty(ws.vocab_size().await).await;
        for &token in &self.tokens {
            allow.set(token, true).await;
        }
        ws.mask_mut().await.and_with(&allow).await;
        Ok(())
    }
    async fn fork(&self) -> LogitsProcessors {
        (Self { tokens: self.tokens.clone() }).into()
    }
}

/// 🧲️ Hard-excludes `tokens` from the live set.
pub struct ForbidTokens {
    pub tokens: Vec<TokenId>,
}

impl LogitsProcessor for ForbidTokens {
    async fn name(&self) -> &'static str {
        "forbid_tokens"
    }
    async fn kind(&self) -> ProcessorKind {
        ProcessorKind::HardMask
    }
    async fn process(&mut self, _view: &StepView<'_>, ws: &mut LogitsWorkspace) -> Result<(), SamplingError> {
        for &token in &self.tokens {
            ws.mask_mut().await.set(token, false).await;
        }
        Ok(())
    }
    async fn fork(&self) -> LogitsProcessors {
        (Self { tokens: self.tokens.clone() }).into()
    }
}

/// 🧲️ Hard-excludes every token in the vocabulary's [`Vocabulary::special`] set.
pub struct SuppressSpecial;

impl LogitsProcessor for SuppressSpecial {
    async fn name(&self) -> &'static str {
        "suppress_special"
    }
    async fn kind(&self) -> ProcessorKind {
        ProcessorKind::HardMask
    }
    async fn process(&mut self, view: &StepView<'_>, ws: &mut LogitsWorkspace) -> Result<(), SamplingError> {
        ws.mask_mut().await.and_not_with(&view.vocab.special).await;
        Ok(())
    }
    async fn fork(&self) -> LogitsProcessors {
        (Self).into()
    }
}

/// 🧲️ Hard-excludes the final token of any bad-word phrase whose proper prefix the generated text
/// currently ends with (prefix-sensitive multi-token bad-word suppression).
pub struct BadWords {
    pub phrases: Vec<Vec<TokenId>>,
}

impl LogitsProcessor for BadWords {
    async fn name(&self) -> &'static str {
        "bad_words"
    }
    async fn kind(&self) -> ProcessorKind {
        ProcessorKind::HardMask
    }
    async fn process(&mut self, view: &StepView<'_>, ws: &mut LogitsWorkspace) -> Result<(), SamplingError> {
        let generated = view.generated;
        for phrase in &self.phrases {
            let Some((&last, prefix)) = phrase.split_last() else { continue };
            if generated.len() >= prefix.len() && &generated[generated.len() - prefix.len()..] == prefix {
                ws.mask_mut().await.set(last, false).await;
            }
        }
        Ok(())
    }
    async fn fork(&self) -> LogitsProcessors {
        (Self { phrases: self.phrases.clone() }).into()
    }
}

/// 🧲️ Positive bias toward completing a configured phrase once the generated text ends with its
/// proper prefix (the encouragement counterpart to [`BadWords`]/[`PhrasePenalty`]).
pub struct SequenceEncouragement {
    pub phrases: Vec<Vec<TokenId>>,
    pub bonus: f32,
}

impl LogitsProcessor for SequenceEncouragement {
    async fn name(&self) -> &'static str {
        "sequence_encouragement"
    }
    async fn kind(&self) -> ProcessorKind {
        ProcessorKind::SoftPenalty
    }
    async fn process(&mut self, view: &StepView<'_>, ws: &mut LogitsWorkspace) -> Result<(), SamplingError> {
        let generated = view.generated;
        let bonus = self.bonus;
        for phrase in &self.phrases {
            let Some((&last, prefix)) = phrase.split_last() else { continue };
            if generated.len() >= prefix.len() && &generated[generated.len() - prefix.len()..] == prefix {
                ws.bias_processed(last, bonus).await;
            }
        }
        Ok(())
    }
    async fn fork(&self) -> LogitsProcessors {
        (Self { phrases: self.phrases.clone(), bonus: self.bonus }).into()
    }
}
// #endregion 🔖️Biases

// #region 🔖️Length
/// 📏️ Hard-suppresses every EOS token until `min_tokens` generated tokens have been produced.
pub struct MinLengthEosSuppression {
    pub min_tokens: usize,
}

impl LogitsProcessor for MinLengthEosSuppression {
    async fn name(&self) -> &'static str {
        "min_length_eos_suppression"
    }
    async fn kind(&self) -> ProcessorKind {
        ProcessorKind::HardMask
    }
    async fn process(&mut self, view: &StepView<'_>, ws: &mut LogitsWorkspace) -> Result<(), SamplingError> {
        if view.generated.len() < self.min_tokens {
            for &eos in &view.vocab.eos {
                ws.mask_mut().await.set(eos, false).await;
            }
        }
        Ok(())
    }
    async fn fork(&self) -> LogitsProcessors {
        (Self { min_tokens: self.min_tokens }).into()
    }
}

/// 📏️ Hard-restricts the live set to (the first configured) EOS token once the *next* token would
/// reach `max_tokens`, guaranteeing generation stops cleanly at the length cap.
pub struct MaxLengthForceEos {
    pub max_tokens: usize,
}

impl LogitsProcessor for MaxLengthForceEos {
    async fn name(&self) -> &'static str {
        "max_length_force_eos"
    }
    async fn kind(&self) -> ProcessorKind {
        ProcessorKind::HardMask
    }
    async fn process(&mut self, view: &StepView<'_>, ws: &mut LogitsWorkspace) -> Result<(), SamplingError> {
        if view.generated.len() + 1 >= self.max_tokens {
            if let Some(&eos) = view.vocab.eos.first() {
                let mut only_eos = TokenBitset::new_empty(ws.vocab_size().await).await;
                only_eos.set(eos, true).await;
                ws.mask_mut().await.and_with(&only_eos).await;
            }
        }
        Ok(())
    }
    async fn fork(&self) -> LogitsProcessors {
        (Self { max_tokens: self.max_tokens }).into()
    }
}

/// 📏️ Hard-forces an exact token at step 0 (`bos`), steps `1..=prefix.len()` (`prefix`), or any
/// explicitly listed `(step, token)` pair thereafter.
pub struct ForcedTokens {
    pub spec: ForcedSpec,
}

impl ForcedTokens {
    async fn forced_token_at(&self, step: u32) -> Option<TokenId> {
        if step == 0 {
            if let Some(bos) = self.spec.bos {
                return Some(bos);
            }
        }
        if step >= 1 && (step as usize) <= self.spec.prefix.len() {
            return self.spec.prefix.get(step as usize - 1).copied();
        }
        self.spec.at_position.iter().find(|(s, _)| s.get() == step).map(|(_, t)| *t)
    }
}

impl LogitsProcessor for ForcedTokens {
    async fn name(&self) -> &'static str {
        "forced_tokens"
    }
    async fn kind(&self) -> ProcessorKind {
        ProcessorKind::HardMask
    }
    async fn process(&mut self, view: &StepView<'_>, ws: &mut LogitsWorkspace) -> Result<(), SamplingError> {
        if let Some(token) = self.forced_token_at(view.step.get()).await {
            let mut only = TokenBitset::new_empty(ws.vocab_size().await).await;
            only.set(token, true).await;
            ws.mask_mut().await.and_with(&only).await;
        }
        Ok(())
    }
    async fn fork(&self) -> LogitsProcessors {
        (Self { spec: self.spec.clone() }).into()
    }
}
// #endregion 🔖️Length

// #region 🔖️Selection
async fn candidate_from(dist: &Distribution<'_>, index: usize) -> Candidate {
    Candidate { token: dist.tokens[index], raw_logit: 0.0, processed_logit: 0.0, prob: dist.probs[index], logprob: dist.logprobs[index], rank: index as u32 }
}

/// 🎯️ Walker's alias method, reimplemented locally (rather than reusing
/// [`semio_framework_geometry::random::AliasTable`]) because that type's `sample` is hard-wired to the
/// concrete `semio_framework_geometry::random::Rng` and cannot accept our `RandomSources` enum.
struct AliasTable {
    prob: Vec<f32>,
    alias: Vec<u32>,
}

impl AliasTable {
    async fn new(weights: &[f32]) -> Self {
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
        // ⚖️ `while let (Some(s), Some(l)) = (small.pop(), large.pop())` looks equivalent but isn't:
        // both `.pop()` calls run unconditionally as call arguments before the pattern is tested, so
        // the moment either vec empties, the other's last element is silently discarded (popped and
        // dropped) instead of falling through to the `prob[x] = 1.0` cleanup loops below — leaving
        // that bucket's `prob`/`alias` at their zeroed defaults. Explicit `is_empty()` checks avoid it.
        while !small.is_empty() && !large.is_empty() {
            let s = small.pop().expect("checked non-empty above");
            let l = large.pop().expect("checked non-empty above");
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

    async fn sample(&self, rng: &mut RandomSources) -> usize {
        let n = self.prob.len();
        let i = rng.next_range(0, n as u64).await as usize;
        if rng.next_f64().await < self.prob[i] as f64 {
            i
        } else {
            self.alias[i] as usize
        }
    }
}

/// 🎯️ Deterministic argmax selection over the (already prob-sorted) [`Distribution`].
pub struct GreedySampler {
    pub tie_break: TieBreak,
}

impl TokenSampler for GreedySampler {
    async fn name(&self) -> &'static str {
        "greedy"
    }
    async fn sample(&mut self, _view: &StepView<'_>, dist: &Distribution<'_>, _rng: &mut RandomSources, out: &mut SelectionBuffer) -> Result<(), SamplingError> {
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
        out.chosen.push(candidate_from(dist, idx).await);
        Ok(())
    }
    async fn fork(&self) -> TokenSamplers {
        (Self { tie_break: self.tie_break }).into()
    }
}

/// 🎯️ Samples one token proportional to the live distribution via the configured strategy.
pub struct MultinomialSampler {
    pub strategy: MultinomialStrategy,
}

impl TokenSampler for MultinomialSampler {
    async fn name(&self) -> &'static str {
        "multinomial"
    }
    async fn sample(&mut self, _view: &StepView<'_>, dist: &Distribution<'_>, rng: &mut RandomSources, out: &mut SelectionBuffer) -> Result<(), SamplingError> {
        if dist.tokens.is_empty() {
            return Err(SamplingError::EmptyDistribution);
        }
        let idx = match self.strategy {
            MultinomialStrategy::CdfBinarySearch => cdf_binary_search(dist.cdf, rng.next_f64().await).await,
            MultinomialStrategy::LinearScan => {
                let u = rng.next_f64().await;
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
                table.await.sample(rng).await
            }
        };
        out.chosen.push(candidate_from(dist, idx).await);
        Ok(())
    }
    async fn fork(&self) -> TokenSamplers {
        (Self { strategy: self.strategy }).into()
    }
}

/// 🎯️ Gumbel-max trick: `argmax(logprob_i + Gumbel_i)`, statistically equivalent to multinomial
/// sampling but usable when only a perturb-then-argmax primitive is available.
pub struct GumbelMaxSampler;

impl TokenSampler for GumbelMaxSampler {
    async fn name(&self) -> &'static str {
        "gumbel_max"
    }
    async fn sample(&mut self, _view: &StepView<'_>, dist: &Distribution<'_>, rng: &mut RandomSources, out: &mut SelectionBuffer) -> Result<(), SamplingError> {
        if dist.tokens.is_empty() {
            return Err(SamplingError::EmptyDistribution);
        }
        let mut best = 0usize;
        let mut best_score = f64::NEG_INFINITY;
        for i in 0..dist.tokens.len() {
            let score = dist.logprobs[i] as f64 + rng.gumbel().await;
            if score > best_score {
                best_score = score;
                best = i;
            }
        }
        out.chosen.push(candidate_from(dist, best).await);
        Ok(())
    }
    async fn fork(&self) -> TokenSamplers {
        (Self).into()
    }
}

/// 🎯️ Gumbel-top-k: `k` tokens without replacement, drawn by taking the top `k` of `logprob_i +
/// Gumbel_i`.
pub struct GumbelTopKSampler {
    pub k: usize,
}

impl TokenSampler for GumbelTopKSampler {
    async fn name(&self) -> &'static str {
        "gumbel_top_k"
    }
    async fn sample(&mut self, _view: &StepView<'_>, dist: &Distribution<'_>, rng: &mut RandomSources, out: &mut SelectionBuffer) -> Result<(), SamplingError> {
        if dist.tokens.is_empty() {
            return Err(SamplingError::EmptyDistribution);
        }
        let k = self.k.min(dist.tokens.len());
        // 🔀️ `.map(async closure)` doesn't type-check — an explicit loop replaces it, since
        // `rng.gumbel()` must be sequentially `.await`ed (O1 de-dyn, math-dedyn).
        let mut scored: Vec<(f64, usize)> = Vec::with_capacity(dist.tokens.len());
        for i in 0..dist.tokens.len() {
            scored.push((dist.logprobs[i] as f64 + rng.gumbel().await, i));
        }
        scored.sort_unstable_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(core::cmp::Ordering::Equal).then_with(|| dist.tokens[a.1].cmp(&dist.tokens[b.1])));
        for &(_, i) in scored.iter().take(k) {
            out.chosen.push(candidate_from(dist, i).await);
        }
        Ok(())
    }
    async fn fork(&self) -> TokenSamplers {
        (Self { k: self.k }).into()
    }
}

/// 🎯️ Builds a [`TokenSampler`] from a [`SamplingMethod`].
pub async fn build_sampler(method: &SamplingMethod) -> TokenSamplers {
    match method {
        SamplingMethod::Greedy { tie_break } => (GreedySampler { tie_break: *tie_break }).into(),
        SamplingMethod::Multinomial { strategy } => (MultinomialSampler { strategy: *strategy }).into(),
        SamplingMethod::GumbelMax => (GumbelMaxSampler).into(),
        SamplingMethod::GumbelTopK { k } => (GumbelTopKSampler { k: *k }).into(),
    }
}

dyn_enum_close! {
    pub enum TokenSamplers: TokenSampler {
        Greedy(GreedySampler),
        Multinomial(MultinomialSampler),
        GumbelMax(GumbelMaxSampler),
        GumbelTopK(GumbelTopKSampler),
    }
}
// #endregion 🔖️Selection

// #region 🔖️Adaptive
/// 🌀️ Mirostat v1/v2: adapts the truncation cutoff step by step so the *observed* surprise of
/// selected tokens tracks `target_surprise`. `last_probs` caches the just-processed live
/// distribution so [`Mirostat::commit`] can look up the selected token's probability without the
/// engine needing to pass it explicitly (the [`LogitsProcessor::commit`] signature only carries
/// the token id).
pub struct Mirostat {
    pub version: MirostatVersion,
    pub target_surprise: f64,
    pub learning_rate: f64,
    mu: f64,
    last_probs: std::collections::HashMap<TokenId, f32>,
    mu_snapshots: Vec<f64>,
}

impl Mirostat {
    pub async fn new(version: MirostatVersion, target_surprise: f64, learning_rate: f64) -> Self {
        Self { version, target_surprise, learning_rate, mu: target_surprise * 2.0, last_probs: std::collections::HashMap::new(), mu_snapshots: Vec::new() }
    }
}

impl LogitsProcessor for Mirostat {
    async fn name(&self) -> &'static str {
        "mirostat"
    }
    async fn kind(&self) -> ProcessorKind {
        ProcessorKind::Truncation
    }
    async fn process(&mut self, _view: &StepView<'_>, ws: &mut LogitsWorkspace) -> Result<(), SamplingError> {
        ws.sort_live_by_prob_desc().await;
        let keep = match self.version {
            // 🌀️ v1: candidate-set size grows/shrinks as `2^mu` (a power-law-fit proxy).
            MirostatVersion::V1 => (2.0f64).powf(self.mu).round().max(1.0) as usize,
            // 🌀️ v2: keep every token whose surprise (`-log2 p`) stays within `mu` of the target.
            MirostatVersion::V2 => {
                let probs = ws.probs().await;
                probs.iter().take_while(|&&p| -(p as f64).log2() <= self.mu).count().max(1)
            }
        };
        ws.truncate_live_to(keep, 1).await;
        self.last_probs.clear();
        for (&token_idx, &p) in ws.live().await.iter().zip(ws.probs().await.iter()) {
            self.last_probs.insert(TokenId::new(token_idx), p);
        }
        Ok(())
    }
    async fn commit(&mut self, _view: &StepView<'_>, token: TokenId) {
        self.mu_snapshots.push(self.mu);
        if let Some(&p) = self.last_probs.get(&token) {
            let surprise = -(p as f64).log2();
            self.mu -= self.learning_rate * (surprise - self.target_surprise);
        }
    }
    async fn save(&mut self) -> StateMark {
        StateMark(self.mu_snapshots.len() as u64)
    }
    async fn rollback_to(&mut self, mark: StateMark) {
        self.mu_snapshots.truncate(mark.0 as usize);
        self.mu = self.mu_snapshots.last().copied().unwrap_or(self.target_surprise * 2.0);
    }
    async fn reset(&mut self) {
        self.mu = self.target_surprise * 2.0;
        self.mu_snapshots.clear();
        self.last_probs.clear();
    }
    async fn fork(&self) -> LogitsProcessors {
        (Self { version: self.version, target_surprise: self.target_surprise, learning_rate: self.learning_rate, mu: self.mu, last_probs: self.last_probs.clone(), mu_snapshots: self.mu_snapshots.clone() }).into()
    }
}

/// 🌀️ PID controller driving temperature toward a target entropy.
pub struct EntropyPid {
    pub target: f64,
    pub kp: f64,
    pub ki: f64,
    pub kd: f64,
    integral: f64,
    last_error: f64,
    pending_error: f64,
    history: Vec<(f64, f64)>,
}

impl EntropyPid {
    pub async fn new(target: f64, kp: f64, ki: f64, kd: f64) -> Self {
        Self { target, kp, ki, kd, integral: 0.0, last_error: 0.0, pending_error: 0.0, history: Vec::new() }
    }
}

impl LogitsProcessor for EntropyPid {
    async fn name(&self) -> &'static str {
        "entropy_pid"
    }
    async fn kind(&self) -> ProcessorKind {
        ProcessorKind::SoftPenalty
    }
    async fn process(&mut self, _view: &StepView<'_>, ws: &mut LogitsWorkspace) -> Result<(), SamplingError> {
        ws.softmax_over_live().await;
        let entropy = entropy_nats(ws.probs().await).await;
        self.pending_error = self.target - entropy;
        let temp = (1.0 + self.kp * self.pending_error + self.ki * self.integral + self.kd * (self.pending_error - self.last_error)).max(0.05);
        ws.scale_processed_over_live(1.0 / temp as f32).await;
        Ok(())
    }
    async fn commit(&mut self, _view: &StepView<'_>, _token: TokenId) {
        self.history.push((self.integral, self.last_error));
        self.integral += self.pending_error;
        self.last_error = self.pending_error;
    }
    async fn save(&mut self) -> StateMark {
        StateMark(self.history.len() as u64)
    }
    async fn rollback_to(&mut self, mark: StateMark) {
        self.history.truncate(mark.0 as usize);
        let (integral, last_error) = self.history.last().copied().unwrap_or((0.0, 0.0));
        self.integral = integral;
        self.last_error = last_error;
    }
    async fn reset(&mut self) {
        self.integral = 0.0;
        self.last_error = 0.0;
        self.pending_error = 0.0;
        self.history.clear();
    }
    async fn fork(&self) -> LogitsProcessors {
        (Self { target: self.target, kp: self.kp, ki: self.ki, kd: self.kd, integral: self.integral, last_error: self.last_error, pending_error: self.pending_error, history: self.history.clone() }).into()
    }
}

/// 🌀️ Flattens the distribution (raises effective temperature) once the fraction of repeated
/// tokens in the trailing `window` exceeds `threshold`.
pub struct RepetitionController {
    pub window: usize,
    pub threshold: f64,
    pub boost: f64,
    recent: std::collections::VecDeque<TokenId>,
    snapshots: Vec<std::collections::VecDeque<TokenId>>,
}

impl RepetitionController {
    pub async fn new(window: usize, threshold: f64, boost: f64) -> Self {
        Self { window, threshold, boost, recent: std::collections::VecDeque::new(), snapshots: Vec::new() }
    }

    async fn repetition_ratio(&self) -> f64 {
        if self.recent.is_empty() {
            return 0.0;
        }
        let unique: std::collections::HashSet<&TokenId> = self.recent.iter().collect();
        1.0 - (unique.len() as f64 / self.recent.len() as f64)
    }
}

impl LogitsProcessor for RepetitionController {
    async fn name(&self) -> &'static str {
        "repetition_controller"
    }
    async fn kind(&self) -> ProcessorKind {
        ProcessorKind::SoftPenalty
    }
    async fn process(&mut self, _view: &StepView<'_>, ws: &mut LogitsWorkspace) -> Result<(), SamplingError> {
        if self.repetition_ratio().await > self.threshold {
            ws.scale_processed_over_live(1.0 / (1.0 + self.boost as f32)).await;
        }
        Ok(())
    }
    async fn commit(&mut self, _view: &StepView<'_>, token: TokenId) {
        self.recent.push_back(token);
        if self.recent.len() > self.window {
            self.recent.pop_front();
        }
    }
    async fn save(&mut self) -> StateMark {
        self.snapshots.push(self.recent.clone());
        StateMark((self.snapshots.len() - 1) as u64)
    }
    async fn rollback_to(&mut self, mark: StateMark) {
        if let Some(recent) = self.snapshots.get(mark.0 as usize) {
            self.recent = recent.clone();
        }
        self.snapshots.truncate(mark.0 as usize + 1);
    }
    async fn reset(&mut self) {
        self.recent.clear();
        self.snapshots.clear();
    }
    async fn fork(&self) -> LogitsProcessors {
        (Self { window: self.window, threshold: self.threshold, boost: self.boost, recent: self.recent.clone(), snapshots: self.snapshots.clone() }).into()
    }
}

/// 🌀️ Interpolates temperature between `low_temp` (near-greedy, at or below `low_entropy`) and
/// `high_temp` (broad, at or above `high_entropy`) based on the live set's current entropy.
pub struct ConfidenceController {
    pub low_entropy: f64,
    pub high_entropy: f64,
    pub low_temp: f64,
    pub high_temp: f64,
}

impl LogitsProcessor for ConfidenceController {
    async fn name(&self) -> &'static str {
        "confidence_controller"
    }
    async fn kind(&self) -> ProcessorKind {
        ProcessorKind::SoftPenalty
    }
    async fn process(&mut self, _view: &StepView<'_>, ws: &mut LogitsWorkspace) -> Result<(), SamplingError> {
        ws.softmax_over_live().await;
        let entropy = entropy_nats(ws.probs().await).await;
        let span = (self.high_entropy - self.low_entropy).max(1e-9);
        let t = ((entropy - self.low_entropy) / span).clamp(0.0, 1.0);
        let temp = self.low_temp + (self.high_temp - self.low_temp) * t;
        if temp <= 0.0 {
            ws.collapse_live_to_argmax().await;
        } else {
            ws.scale_processed_over_live(1.0 / temp as f32).await;
        }
        Ok(())
    }
    async fn fork(&self) -> LogitsProcessors {
        (Self { low_entropy: self.low_entropy, high_entropy: self.high_entropy, low_temp: self.low_temp, high_temp: self.high_temp }).into()
    }
}

dyn_enum_close! {
    pub enum LogitsProcessors: LogitsProcessor {
        Temperature(Temperature),
        DynamicTemperature(DynamicTemperature),
        TopK(TopK),
        TopP(TopP),
        MinP(MinP),
        Typical(Typical),
        LocallyTypical(LocallyTypical),
        TailFree(TailFree),
        EpsilonCutoff(EpsilonCutoff),
        EtaCutoff(EtaCutoff),
        TopA(TopA),
        RankTruncation(RankTruncation),
        AdaptiveTruncation(AdaptiveTruncation),
        RepetitionPenalty(RepetitionPenalty),
        PresencePenalty(PresencePenalty),
        FrequencyPenalty(FrequencyPenalty),
        DecayingPenalty(DecayingPenalty),
        TokenClassPenalty(TokenClassPenalty),
        NoRepeatNgram(NoRepeatNgram),
        PhrasePenalty(PhrasePenalty),
        LogitBiasSparse(LogitBiasSparse),
        LogitBiasDense(LogitBiasDense),
        AllowTokens(AllowTokens),
        ForbidTokens(ForbidTokens),
        SuppressSpecial(SuppressSpecial),
        BadWords(BadWords),
        SequenceEncouragement(SequenceEncouragement),
        MinLengthEosSuppression(MinLengthEosSuppression),
        MaxLengthForceEos(MaxLengthForceEos),
        ForcedTokens(ForcedTokens),
        Mirostat(Mirostat),
        EntropyPid(EntropyPid),
        RepetitionController(RepetitionController),
        ConfidenceController(ConfidenceController),
    }
}
// #endregion 🔖️Adaptive

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
