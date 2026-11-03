//! ♿️ Text alternative of the 3D scene and the live-region announcement of a selection, in English and German.

use super::config::Lang;
use super::lowering::LoweringStats;
use super::overlays::number;
use super::picking::{resolve, PickTarget};
use super::scene::{CheckStatus, LocalizedCopy, SceneEntity, StructuralScene};

pub fn pick(copy: &LocalizedCopy, lang: Lang) -> &str {
    match lang {
        Lang::En => &copy.en,
        Lang::De => &copy.de,
    }
}

pub fn status_word(status: CheckStatus, lang: Lang) -> &'static str {
    match (status, lang) {
        (CheckStatus::Pass, Lang::En) => "passes",
        (CheckStatus::Pass, Lang::De) => "besteht",
        (CheckStatus::Warning, Lang::En) => "near limit",
        (CheckStatus::Warning, Lang::De) => "nahe der Grenze",
        (CheckStatus::Fail, Lang::En) => "fails",
        (CheckStatus::Fail, Lang::De) => "versagt",
        (CheckStatus::NotApplicable, Lang::En) => "not applicable",
        (CheckStatus::NotApplicable, Lang::De) => "nicht anwendbar",
    }
}

fn percent(ratio: f64, lang: Lang) -> String {
    format!("{} %", number(ratio * 100.0, matches!(lang, Lang::De), 0))
}

/// 🗣️ Utilisation as a screen reader says it: "utilisation 64 %, passes", or only the status when there is no utilisation to speak.
pub fn spoken_utilisation(ratio: f64, status: CheckStatus, lang: Lang) -> String {
    if status == CheckStatus::NotApplicable || !ratio.is_finite() {
        return status_word(status, lang).to_owned();
    }
    format!("{} {}, {}", if matches!(lang, Lang::De) { "Ausnutzung" } else { "utilisation" }, percent(ratio, lang), status_word(status, lang))
}

/// 🗣️ The accessible name of a viewport entity, the one source of the screen-reader option and the viewport instance label: its name, then the spoken utilisation when it has one.
pub fn entity_spoken_name(entity: &SceneEntity, lang: Lang) -> String {
    let name = pick(&entity.accessible_name, lang);
    entity.utilisation.as_ref().map_or_else(|| name.to_owned(), |u| format!("{name}, {}", spoken_utilisation(u.ratio, u.status, lang)))
}

/// ♿️ "Member B1 selected, utilisation 64 %, passes" for an interaction address; `None` when it resolves to no scene entity.
pub fn announce_selection(scene: &StructuralScene, lang: Lang, address: &str) -> Option<String> {
    let Some(PickTarget::Entity { entity, .. }) = resolve(scene, address) else { return None };
    let e = scene.entities.iter().find(|e| e.entity.id == entity.id)?;
    let name = pick(&e.accessible_name, lang);
    Some(match (&e.utilisation, lang) {
        (Some(u), Lang::En) => format!("{name} selected, utilisation {}, {}", percent(u.ratio, lang), status_word(u.status, lang)),
        (Some(u), Lang::De) => format!("{name} ausgewählt, Ausnutzung {}, {}", percent(u.ratio, lang), status_word(u.status, lang)),
        (None, Lang::En) => format!("{name} selected"),
        (None, Lang::De) => format!("{name} ausgewählt"),
    })
}

/// ♿️ One-paragraph description of the whole lowered viewport: scene text, object count, budget truncation, worst check and selection.
pub fn describe(scene: &StructuralScene, lang: Lang, stats: &LoweringStats, selection: &[String]) -> String {
    let mut text = pick(&scene.description, lang).to_string();
    let de = matches!(lang, Lang::De);
    text.push_str(&if de { format!(" {} Objekte dargestellt.", stats.entities) } else { format!(" {} objects shown.", stats.entities) });
    if stats.truncated > 0 {
        text.push_str(&if de { format!(" {} Objekte wegen des Geräte-Budgets ausgeblendet.", stats.truncated) } else { format!(" {} objects hidden to stay within the device budget.", stats.truncated) });
    }
    if let Some((e, u)) = scene.entities.iter().filter_map(|e| e.utilisation.as_ref().map(|u| (e, u))).max_by(|a, b| a.1.ratio.total_cmp(&b.1.ratio)) {
        let name = pick(&e.accessible_name, lang);
        text.push_str(&if de { format!(" Höchste Ausnutzung: {name} mit {}, {}.", percent(u.ratio, lang), status_word(u.status, lang)) } else { format!(" Highest utilisation: {name} at {}, {}.", percent(u.ratio, lang), status_word(u.status, lang)) });
    }
    let names: Vec<String> = selection.iter().filter_map(|a| match resolve(scene, a) { Some(PickTarget::Entity { entity, .. }) => scene.entities.iter().find(|e| e.entity.id == entity.id).map(|e| pick(&e.accessible_name, lang).to_string()), _ => None }).collect();
    if !names.is_empty() {
        text.push_str(&if de { format!(" Ausgewählt: {}.", names.join(", ")) } else { format!(" Selected: {}.", names.join(", ")) });
    }
    text
}
