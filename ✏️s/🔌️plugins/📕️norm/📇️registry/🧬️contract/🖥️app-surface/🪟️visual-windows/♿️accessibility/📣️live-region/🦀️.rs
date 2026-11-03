//! 📣️ Live region: bilingual, coalesced announcements of evaluation results and utilisation changes for assistive technology.

use crate::viewport3d::config::Lang;
use crate::viewport3d::description::{pick, status_word};
use crate::viewport3d::overlays::number;
use super::screen_reader::{handle_node, BoundUtilisation};
use semio_s_plugin_norm_structural_scene::{CheckStatus, Handle, LocalizedCopy, StructuralScene};

/// 🔔️ How urgently assistive technology interrupts.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Politeness {
    Polite,
    Assertive,
}

impl Politeness {
    pub fn aria(self) -> &'static str {
        match self {
            Self::Polite => "polite",
            Self::Assertive => "assertive",
        }
    }
}

/// 📢️ One announcement; `key` coalesces repeated announcements about the same subject.
#[derive(Clone, Debug, PartialEq)]
pub struct Announcement {
    pub key: String,
    pub text: LocalizedCopy,
    pub politeness: Politeness,
}

/// 📸️ Utilisation of one entity at one evaluation.
#[derive(Clone, Debug, PartialEq)]
pub struct Utilisation {
    pub id: String,
    pub name: LocalizedCopy,
    pub ratio: f64,
    pub status: CheckStatus,
}

pub const RATIO_STEP: f64 = 0.05;

/// 📸️ Utilisations of every entity of a scene that has a binding.
pub fn utilisations(scene: &StructuralScene) -> Vec<Utilisation> {
    scene.entities.iter().filter_map(|e| e.utilisation.as_ref().map(|u| Utilisation { id: e.entity.id.clone(), name: e.accessible_name.clone(), ratio: u.ratio, status: u.status })).collect()
}

fn percent(ratio: f64, de: bool) -> String {
    format!("{} %", number(ratio * 100.0, de, 0))
}

fn words(status: CheckStatus) -> LocalizedCopy {
    LocalizedCopy::new(status_word(status, Lang::En), status_word(status, Lang::De))
}

/// 🏁️ "Evaluation finished: 12 checks, 1 failing. Highest utilisation: B2 at 112 %, fails."
pub fn announce_evaluation(results: &[Utilisation]) -> Announcement {
    let failing = results.iter().filter(|u| u.status == CheckStatus::Fail).count();
    let warning = results.iter().filter(|u| u.status == CheckStatus::Warning).count();
    let worst = results.iter().filter(|u| u.status != CheckStatus::NotApplicable).max_by(|a, b| a.ratio.total_cmp(&b.ratio));
    let (en_tail, de_tail) = match worst {
        Some(w) => (format!(" Highest utilisation: {} at {}, {}.", w.name.en, percent(w.ratio, false), status_word(w.status, Lang::En)), format!(" Höchste Ausnutzung: {} mit {}, {}.", w.name.de, percent(w.ratio, true), status_word(w.status, Lang::De))),
        None => (String::new(), String::new()),
    };
    let text = LocalizedCopy::new(
        format!("Evaluation finished: {} checks, {failing} failing, {warning} near the limit.{en_tail}", results.len()),
        format!("Auswertung abgeschlossen: {} Nachweise, {failing} nicht erfüllt, {warning} nahe am Grenzwert.{de_tail}", results.len()),
    );
    Announcement { key: "evaluation".into(), text, politeness: if failing > 0 { Politeness::Assertive } else { Politeness::Polite } }
}

/// 🔄️ One announcement per entity whose status changed or whose ratio moved by at least [`RATIO_STEP`]; new failures are assertive.
pub fn announce_changes(before: &[Utilisation], after: &[Utilisation]) -> Vec<Announcement> {
    after.iter().filter_map(|now| {
        let was = before.iter().find(|b| b.id == now.id);
        let changed = was.is_none_or(|b| b.status != now.status || (b.ratio - now.ratio).abs() >= RATIO_STEP);
        if !changed {
            return None;
        }
        let new_failure = now.status == CheckStatus::Fail && was.is_none_or(|b| b.status != CheckStatus::Fail);
        let (en_from, de_from) = was.map_or((String::new(), String::new()), |b| (format!("from {} ", percent(b.ratio, false)), format!("von {} ", percent(b.ratio, true))));
        let w = words(now.status);
        Some(Announcement {
            key: format!("utilisation:{}", now.id),
            text: LocalizedCopy::new(format!("{} utilisation {en_from}to {}, {}", now.name.en, percent(now.ratio, false), w.en), format!("{} Ausnutzung {de_from}auf {}, {}", now.name.de, percent(now.ratio, true), w.de)),
            politeness: if new_failure { Politeness::Assertive } else { Politeness::Polite },
        })
    }).collect()
}

/// 🕹️ The polite announcement of a grip or marker at drag parameter `s`, on focus and after every nudge or typed value: its name and the field value with unit, then the live utilisation of the bound check. The key is the handle, so a run of nudges speaks only the latest value.
pub fn announce_handle(handle: &Handle, s: f64, utilisation: Option<BoundUtilisation>) -> Announcement {
    let say = |lang| {
        let node = handle_node(handle, Some(s), utilisation, lang);
        format!("{}: {}", node.name, node.value_text.unwrap_or_default())
    };
    Announcement { key: format!("handle:{}", handle.id), text: LocalizedCopy::new(say(Lang::En), say(Lang::De)), politeness: Politeness::Polite }
}

/// 📬️ Pending announcements: later ones replace earlier ones with the same key, assertive ones are spoken first.
#[derive(Clone, Debug, Default)]
pub struct LiveRegion {
    pending: Vec<Announcement>,
}

impl LiveRegion {
    pub fn push(&mut self, announcement: Announcement) {
        self.pending.retain(|a| a.key != announcement.key);
        self.pending.push(announcement);
    }

    pub fn extend(&mut self, announcements: impl IntoIterator<Item = Announcement>) {
        announcements.into_iter().for_each(|a| self.push(a));
    }

    pub fn len(&self) -> usize {
        self.pending.len()
    }

    pub fn is_empty(&self) -> bool {
        self.pending.is_empty()
    }

    /// 🗣️ Up to `max` announcements, assertive first then in arrival order; the rest collapse into one polite "n more changes" line.
    pub fn drain(&mut self, max: usize) -> Vec<Announcement> {
        let mut all = std::mem::take(&mut self.pending);
        all.sort_by_key(|a| std::cmp::Reverse(a.politeness));
        let rest = all.split_off(max.min(all.len()));
        if !rest.is_empty() {
            let n = rest.len();
            all.push(Announcement { key: "overflow".into(), text: LocalizedCopy::new(format!("{n} more changes, see the checks list"), format!("{n} weitere Änderungen, siehe Nachweisliste")), politeness: Politeness::Polite });
        }
        all
    }
}

/// 🌐️ The announcement text in a language.
pub fn text(a: &Announcement, lang: Lang) -> &str {
    pick(&a.text, lang)
}
