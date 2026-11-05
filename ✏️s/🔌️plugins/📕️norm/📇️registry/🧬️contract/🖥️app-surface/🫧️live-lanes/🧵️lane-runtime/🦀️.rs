//! 🧵️ Lane runtime: transient reducer, preview overlay, frame coalescer, 30 Hz presence publisher and evaluation debounce. Never awaits the network. See [`super`].

use super::{DragPreview, EvalProgress, FieldWrite, GhostPreview, LaneTransient, NormPresence, NormTransient, PeerDrag, ViewLane};
use semio_framework_plugin::app::InteractionView;
use crate::direct_manipulation::DragEffect;
use semio_framework_value::DslValue;
use semio_s_plugin_norm_structural_scene::projection::{ViewKey, ViewOptions, ViewValue};

pub const PRESENCE_INTERVAL_MS: u64 = 34;
pub const T1_DEBOUNCE_MS: u64 = 60;
pub const T2_QUIESCENCE_MS: u64 = 250;
pub const MAX_LINK_LOSS_MS: u64 = 30_000;

/// 📜️ One event of the ephemeral local lane; the transient state is its pure fold.
#[derive(Clone, Debug, PartialEq)]
pub enum LaneEvent {
    DragPreviewed(DragPreview),
    DragCleared,
    GhostPreviewed(GhostPreview),
    GhostCleared,
    EvalStarted { units: Vec<String> },
    EvalUnitDone { unit: String },
    EvalCancelled,
    EvalFinished,
    PeersSeen(Vec<PeerDrag>),
    ViewField(Option<String>),
    ViewTrack(Option<String>),
    ViewSeek(f64),
    ViewToggle,
    ViewSpeed(f64),
    ViewUnits(Option<String>),
    ViewItem(Option<u32>),
    ViewCheck(Option<u32>),
    ViewGroup(Option<u32>),
    ViewMenu(bool),
    ViewFrame,
    ViewStep { step: ViewStep, forward: bool },
}

/// 🔢️ What a stepping shortcut walks: the checks in list order, the checks from the governing one down, or the combinations (variants) of the scene; the step needs the report and the scene to become [`LaneEvent::ViewCheck`] or [`LaneEvent::ViewItem`] events, see the panels crate.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ViewStep {
    Check,
    Governing,
    Combination,
}

/// 🎞️ Viewport action ids the single editor tool `viewportAct` carries in its `action` argument.
pub const VIEW_ACTION_FIELD: &str = "viewportSetActiveField";
pub const VIEW_ACTION_TRACK: &str = "viewportSelectTrack";
pub const VIEW_ACTION_SEEK: &str = "viewportSeek";
pub const VIEW_ACTION_STEP: &str = "viewportStep";
pub const VIEW_ACTION_TOGGLE: &str = "viewportTogglePlay";
pub const VIEW_ACTION_SPEED: &str = "viewportSetPlaybackSpeed";
pub const VIEW_ACTION_UNITS: &str = "viewportSetUnits";
pub const VIEW_ACTION_STEP_CHECK: &str = "viewportStepCheck";
pub const VIEW_ACTION_STEP_GOVERNING: &str = "viewportStepGoverning";
pub const VIEW_ACTION_STEP_COMBINATION: &str = "viewportStepCombination";
pub const VIEW_ACTION_MENU: &str = "viewportTouchMenu";
pub const VIEW_ACTION_FRAME: &str = "viewportTouchFrame";

/// 🪟️ Prefix of the `setFields` paths that address the transient view lane instead of the document.
pub const VIEW_PATH_PREFIX: &str = "view.";
pub const VIEW_PATH_ACTIVE_COMBINATION: &str = "view.activeCombination";
pub const VIEW_PATH_ACTIVE_TRAFFIC_GROUP: &str = "view.activeTrafficGroup";

/// 🪟️ Whether a `setFields` write addresses the view lane.
pub fn is_view_write(write: &FieldWrite) -> bool {
    write.path.starts_with(VIEW_PATH_PREFIX)
}

/// 🪟️ The lane events of the `view.` writes of a bundle: `view.activeCombination` selects the combination or variant by index and `view.activeTrafficGroup` the traffic group (a non-number or negative value clears either); unknown view paths are no event.
pub fn view_write_events(writes: &[FieldWrite]) -> Vec<LaneEvent> {
    writes
        .iter()
        .filter(|write| write.path == VIEW_PATH_ACTIVE_COMBINATION || write.path == VIEW_PATH_ACTIVE_TRAFFIC_GROUP)
        .map(|write| {
            let index = match &write.value {
                DslValue::Number(number) if number.as_f64().is_finite() && number.as_f64() >= 0.0 => Some(number.as_f64().round() as u32),
                _ => None,
            };
            if write.path == VIEW_PATH_ACTIVE_TRAFFIC_GROUP { LaneEvent::ViewGroup(index) } else { LaneEvent::ViewItem(index) }
        })
        .collect()
}

/// 👁️ The typed view options a live scene projects with: the combination or variant, the traffic group, the selected track and its value, exactly as the ephemeral view lane holds them (an unset choice stays unset and follows the governing one).
pub fn view_options_of(lane: &ViewLane) -> ViewOptions {
    ViewOptions::default()
        .with_some(ViewKey::CombinationItem, lane.item.map(ViewValue::Index))
        .with_some(ViewKey::TrafficGroupItem, lane.group.map(ViewValue::Index))
        .with_some(ViewKey::Track, lane.track.clone().map(ViewValue::Id))
        .with_some(ViewKey::TrackValue, lane.value.map(ViewValue::Number))
}

/// 🧭️ The lane event of a viewport action with its text and number argument; empty text clears a choice, `viewportStep` carries the target value, the three stepping actions carry the direction (negative is backward), unknown actions and missing numbers are no event.
pub fn view_event(action: &str, text: Option<&str>, number: Option<f64>) -> Option<LaneEvent> {
    let choice = || text.filter(|t| !t.is_empty()).map(str::to_string);
    match action {
        VIEW_ACTION_FIELD => Some(LaneEvent::ViewField(choice())),
        VIEW_ACTION_TRACK => Some(LaneEvent::ViewTrack(choice())),
        VIEW_ACTION_SEEK | VIEW_ACTION_STEP => number.filter(|n| n.is_finite()).map(LaneEvent::ViewSeek),
        VIEW_ACTION_TOGGLE => Some(LaneEvent::ViewToggle),
        VIEW_ACTION_UNITS => Some(LaneEvent::ViewUnits(choice().filter(|unit| matches!(unit.as_str(), "si" | "engineering")))),
        VIEW_ACTION_STEP_CHECK => number.filter(|n| n.is_finite()).map(|n| LaneEvent::ViewStep { step: ViewStep::Check, forward: n >= 0.0 }),
        VIEW_ACTION_STEP_GOVERNING => number.filter(|n| n.is_finite()).map(|n| LaneEvent::ViewStep { step: ViewStep::Governing, forward: n >= 0.0 }),
        VIEW_ACTION_STEP_COMBINATION => number.filter(|n| n.is_finite()).map(|n| LaneEvent::ViewStep { step: ViewStep::Combination, forward: n >= 0.0 }),
        VIEW_ACTION_SPEED => number.filter(|n| n.is_finite() && *n > 0.0).map(LaneEvent::ViewSpeed),
        VIEW_ACTION_MENU => number.filter(|n| n.is_finite()).map(|n| LaneEvent::ViewMenu(n >= 0.5)),
        VIEW_ACTION_FRAME => Some(LaneEvent::ViewFrame),
        _ => None,
    }
}

/// 🖐️ Whether a command ends the open touch Edit menu: any command does, except the open and close of the menu itself (`view_event` is the lane event the command stands for); a closed menu has nothing to end.
pub fn menu_closes(menu_open: bool, view_event: Option<&LaneEvent>) -> bool {
    menu_open && !matches!(view_event, Some(LaneEvent::ViewMenu(_)))
}

impl NormTransient {
    /// 📜️ Folds one lane event; the same events in the same order always give the same transient.
    pub fn apply(self, event: LaneEvent) -> Self {
        match event {
            LaneEvent::DragPreviewed(drag) => self.with_drag(Some(drag)),
            LaneEvent::DragCleared => self.with_drag(None),
            LaneEvent::GhostPreviewed(ghost) => self.with_ghost(Some(ghost)),
            LaneEvent::GhostCleared => self.with_ghost(None),
            LaneEvent::EvalStarted { units } => {
                let total = units.len() as u32;
                self.with_eval(EvalProgress { running: units, done: 0, total, cancelled: false })
            }
            LaneEvent::EvalUnitDone { unit } => {
                let mut eval = self.eval.clone();
                if let Some(at) = eval.running.iter().position(|u| *u == unit) {
                    eval.running.remove(at);
                    eval.done += 1;
                }
                self.with_eval(eval)
            }
            LaneEvent::EvalCancelled => {
                let eval = EvalProgress { running: Vec::new(), cancelled: true, ..self.eval.clone() };
                self.with_eval(eval)
            }
            LaneEvent::EvalFinished => self.with_eval(EvalProgress::default()),
            LaneEvent::PeersSeen(peers) => self.with_peers(peers),
            LaneEvent::ViewField(active_field) => {
                let view = ViewLane { active_field, ..self.view.clone() };
                self.with_view(view)
            }
            LaneEvent::ViewTrack(track) => {
                let view = ViewLane { track, value: None, playing: false, clock: 0.0, ..self.view.clone() };
                self.with_view(view)
            }
            LaneEvent::ViewSeek(value) => {
                let view = ViewLane { value: Some(value), ..self.view.clone() };
                self.with_view(view)
            }
            LaneEvent::ViewToggle => {
                let view = ViewLane { playing: !self.view.playing, ..self.view.clone() };
                self.with_view(view)
            }
            LaneEvent::ViewSpeed(speed) => {
                let view = ViewLane { speed, ..self.view.clone() };
                self.with_view(view)
            }
            LaneEvent::ViewUnits(units) => {
                let view = ViewLane { units, ..self.view.clone() };
                self.with_view(view)
            }
            LaneEvent::ViewItem(item) => {
                let view = ViewLane { item, ..self.view.clone() };
                self.with_view(view)
            }
            LaneEvent::ViewCheck(check) => {
                let view = ViewLane { check, ..self.view.clone() };
                self.with_view(view)
            }
            LaneEvent::ViewGroup(group) => {
                let view = ViewLane { group, ..self.view.clone() };
                self.with_view(view)
            }
            LaneEvent::ViewMenu(menu) => {
                let view = ViewLane { menu, ..self.view.clone() };
                self.with_view(view)
            }
            LaneEvent::ViewFrame => {
                let view = ViewLane { frame: !self.view.frame, ..self.view.clone() };
                self.with_view(view)
            }
            LaneEvent::ViewStep { .. } => self,
        }
    }
}

/// 👁️ The preview document: the committed `document` with every provisional write of `transient` laid over it, drag before ghost; the placement leaves of an entity without a row insert its row, exactly like the release of the drag does. The input is untouched.
pub fn overlay_document(document: &DslValue, transient: &NormTransient) -> Result<DslValue, String> {
    let writes: Vec<FieldWrite> = transient.overlay().cloned().collect();
    let mut preview = document.clone();
    crate::direct_manipulation::overlay_writes(&mut preview, &writes)?;
    Ok(preview)
}

/// 🗜️ Keeps only the latest value per key, in first-seen key order; intermediate drag ticks are dropped.
#[derive(Clone, Debug)]
pub struct Coalescer<K, V> {
    pending: Vec<(K, V)>,
    dropped: u64,
}

impl<K: PartialEq, V> Default for Coalescer<K, V> {
    fn default() -> Self {
        Self { pending: Vec::new(), dropped: 0 }
    }
}

impl<K: PartialEq, V> Coalescer<K, V> {
    pub fn push(&mut self, key: K, value: V) {
        match self.pending.iter_mut().find(|(k, _)| *k == key) {
            Some(slot) => {
                slot.1 = value;
                self.dropped += 1;
            }
            None => self.pending.push((key, value)),
        }
    }

    pub fn drain(&mut self) -> Vec<(K, V)> {
        std::mem::take(&mut self.pending)
    }

    pub fn len(&self) -> usize {
        self.pending.len()
    }

    pub fn is_empty(&self) -> bool {
        self.pending.is_empty()
    }

    pub fn dropped(&self) -> u64 {
        self.dropped
    }
}

/// 🔗️ Connection state of the shared lanes; the local store never depends on it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Link {
    Online,
    Lost { since_ms: u64 },
}

/// 📡️ Publishes the latest presence at most every [`PRESENCE_INTERVAL_MS`] (30 Hz), only when it changed, never queues while the link is lost
/// and publishes the freshest value at once after reconnect.
#[derive(Clone, Debug)]
pub struct PresencePublisher {
    latest: Option<NormPresence>,
    sent: Option<NormPresence>,
    last_sent_ms: Option<u64>,
    link: Link,
    suppressed: u64,
}

impl Default for PresencePublisher {
    fn default() -> Self {
        Self { latest: None, sent: None, last_sent_ms: None, link: Link::Online, suppressed: 0 }
    }
}

impl PresencePublisher {
    pub fn offer(&mut self, presence: NormPresence) {
        self.latest = Some(presence);
    }

    pub fn set_link(&mut self, link: Link) {
        if matches!((self.link, link), (Link::Lost { .. }, Link::Online)) {
            self.last_sent_ms = None;
            self.sent = None;
        }
        self.link = link;
    }

    /// 📡️ The presence to send now, if any: online, changed since the last send, and the 30 Hz gate is open.
    pub fn poll(&mut self, now_ms: u64) -> Option<NormPresence> {
        if matches!(self.link, Link::Lost { .. }) {
            self.suppressed += u64::from(self.latest.is_some());
            return None;
        }
        let latest = self.latest.as_ref()?;
        if self.sent.as_ref() == Some(latest) || self.last_sent_ms.is_some_and(|last| now_ms.saturating_sub(last) < PRESENCE_INTERVAL_MS) {
            return None;
        }
        self.last_sent_ms = Some(now_ms);
        self.sent = Some(latest.clone());
        self.sent.clone()
    }

    /// 🚧️ True once the link has been lost longer than [`MAX_LINK_LOSS_MS`]: the UI shows a notice, nothing blocks.
    pub fn offline_too_long(&self, now_ms: u64) -> bool {
        matches!(self.link, Link::Lost { since_ms } if now_ms.saturating_sub(since_ms) > MAX_LINK_LOSS_MS)
    }

    pub fn suppressed(&self) -> u64 {
        self.suppressed
    }
}

/// ⏱️ Which evaluation tier became due.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EvalTier {
    T1,
    T2,
}

/// ⏱️ Debounces re-evaluation: T1 60 ms after the last tick, T2 250 ms after quiescence or at once on commit.
#[derive(Clone, Copy, Debug, Default)]
pub struct EvalScheduler {
    last_tick_ms: Option<u64>,
    t1_pending: bool,
    t2_pending: bool,
}

impl EvalScheduler {
    pub fn on_tick(&mut self, now_ms: u64) {
        self.last_tick_ms = Some(now_ms);
        self.t1_pending = true;
        self.t2_pending = true;
    }

    pub fn on_commit(&mut self) {
        self.last_tick_ms = None;
        self.t1_pending = true;
        self.t2_pending = true;
    }

    pub fn poll(&mut self, now_ms: u64) -> Vec<EvalTier> {
        let quiet = self.last_tick_ms.map_or(u64::MAX, |last| now_ms.saturating_sub(last));
        let mut due = Vec::new();
        if self.t1_pending && quiet >= T1_DEBOUNCE_MS {
            self.t1_pending = false;
            due.push(EvalTier::T1);
        }
        if self.t2_pending && quiet >= T2_QUIESCENCE_MS {
            self.t2_pending = false;
            due.push(EvalTier::T2);
        }
        due
    }
}

/// 🧵️ The runtime of one open editor: the transient fold, the per-frame tick coalescer, the presence publisher and the evaluation scheduler.
#[derive(Clone, Debug)]
pub struct LaneRuntime {
    pub transient: NormTransient,
    pub presence: PresencePublisher,
    pub scheduler: EvalScheduler,
    ticks: Coalescer<String, DragPreview>,
    local: NormPresence,
}

impl Default for LaneRuntime {
    fn default() -> Self {
        Self::new("")
    }
}

impl LaneRuntime {
    pub fn new(user: impl Into<String>) -> Self {
        Self { transient: NormTransient::default(), presence: PresencePublisher::default(), scheduler: EvalScheduler::default(), ticks: Coalescer::default(), local: NormPresence::new(user) }
    }

    /// 🎬️ Takes the effect of a drag step: previews are coalesced to the frame, commit and clear drop the preview at once.
    pub fn accept(&mut self, effect: DragEffect, now_ms: u64) -> Option<Vec<FieldWrite>> {
        match effect {
            DragEffect::None => None,
            DragEffect::Preview(preview) => {
                self.ticks.push(preview.handle.clone(), preview);
                self.scheduler.on_tick(now_ms);
                None
            }
            DragEffect::Commit(writes) => {
                self.clear_drag();
                self.scheduler.on_commit();
                Some(writes)
            }
            DragEffect::Cleared => {
                self.clear_drag();
                None
            }
        }
    }

    fn clear_drag(&mut self) {
        self.ticks.drain();
        self.transient = std::mem::take(&mut self.transient).apply(LaneEvent::DragCleared);
        self.local.drag = None;
        self.presence.offer(self.local.clone());
    }

    /// 🖼️ Once per frame: the latest coalesced preview becomes the transient drag and the shared presence drag.
    pub fn frame(&mut self) -> bool {
        let Some((_, latest)) = self.ticks.drain().pop() else { return false };
        self.local.drag = Some(latest.clone());
        self.transient = std::mem::take(&mut self.transient).apply(LaneEvent::DragPreviewed(latest));
        self.presence.offer(self.local.clone());
        true
    }

    pub fn set_selection(&mut self, selection: Vec<semio_s_plugin_norm_structural_scene::EntityRef>, hover: Option<semio_s_plugin_norm_structural_scene::EntityRef>) {
        self.local.selection = selection;
        self.local.hover = hover;
        self.presence.offer(self.local.clone());
    }

    pub fn dropped_ticks(&self) -> u64 {
        self.ticks.dropped()
    }
}

//#region 🔖️Frame
/// 👥️ What one other user shows in the viewport: their selected and hovered scene addresses.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PeerLane {
    pub actor: String,
    pub selected: Vec<String>,
    pub hovered: Vec<String>,
}

/// 🖼️ Everything the lanes add to one paint: the open drag and ghost of this user and the selection of every peer.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct LaneFrame {
    pub transient: NormTransient,
    pub peers: Vec<PeerLane>,
}

impl LaneFrame {
    /// 🖼️ Reads the transient store view and the peers of `domain` out of the interaction view.
    pub fn of<T: LaneTransient>(transient: &T, interaction: &InteractionView<'_>, domain: &str) -> Self {
        let peers = interaction
            .peers
            .iter()
            .filter_map(|(actor, peer)| {
                let slice = peer.interaction.as_ref()?.domains.iter().find(|candidate| candidate.domain == domain)?;
                Some(PeerLane { actor: actor.to_string(), selected: slice.selected.clone(), hovered: slice.hovered.clone() })
            })
            .collect();
        Self { transient: transient.lane(), peers }
    }
}
//#endregion 🔖️Frame

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
