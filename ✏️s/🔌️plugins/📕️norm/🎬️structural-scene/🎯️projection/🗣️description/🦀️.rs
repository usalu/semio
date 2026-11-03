//! 🗣️ Text alternative of a scene in English and German: a summary paragraph and a per-entity listing for screen readers and the table twin.

use crate::{CheckStatus, EntityKind, LocalizedCopy, SceneEntity, StructuralScene};

/// 🏷️ Plural display name and default layer id of an entity kind.
pub fn kind_text(kind: EntityKind) -> (&'static str, &'static str, &'static str) {
    match kind {
        EntityKind::Member => ("members", "Members", "Bauteile"),
        EntityKind::Plate => ("plates", "Plates", "Platten"),
        EntityKind::Shell => ("shells", "Shells", "Schalen"),
        EntityKind::Solid => ("solids", "Solids", "Körper"),
        EntityKind::Fastener => ("fasteners", "Fasteners", "Verbindungsmittel"),
        EntityKind::Support => ("supports", "Supports", "Lager"),
        EntityKind::Load => ("loads", "Loads", "Lasten"),
        EntityKind::Surface => ("surfaces", "Surfaces", "Flächen"),
        EntityKind::Group => ("groups", "Groups", "Gruppen"),
    }
}

const KINDS: [EntityKind; 9] = [EntityKind::Member, EntityKind::Plate, EntityKind::Shell, EntityKind::Solid, EntityKind::Fastener, EntityKind::Support, EntityKind::Load, EntityKind::Surface, EntityKind::Group];

fn status(s: CheckStatus) -> (&'static str, &'static str) {
    match s {
        CheckStatus::Pass => ("pass", "erfüllt"),
        CheckStatus::Warning => ("warning", "Warnung"),
        CheckStatus::Fail => ("fail", "nicht erfüllt"),
        CheckStatus::NotApplicable => ("not applicable", "nicht anwendbar"),
    }
}

fn percent(ratio: f64) -> String {
    format!("{:.0} %", ratio * 100.0)
}

fn utilisation_text(e: &SceneEntity) -> Option<(String, String)> {
    e.utilisation.as_ref().map(|u| {
        let (en, de) = status(u.status);
        (format!("utilisation {}, {en}{}", percent(u.ratio), if u.stale { ", stale" } else { "" }), format!("Ausnutzung {}, {de}{}", percent(u.ratio), if u.stale { ", veraltet" } else { "" }))
    })
}

/// 📝️ One paragraph: element count per kind, the governing utilisation with its element, failing and stale counts.
pub fn describe(scene: &StructuralScene) -> LocalizedCopy {
    let n = scene.entities.len();
    let counts: Vec<(EntityKind, usize)> = KINDS.iter().map(|k| (*k, scene.entities.iter().filter(|e| e.entity.kind == *k).count())).filter(|(_, c)| *c > 0).collect();
    let (mut en, mut de) = (
        if n == 1 { "Scene with 1 element.".to_owned() } else { format!("Scene with {n} elements.") },
        if n == 1 { "Szene mit 1 Element.".to_owned() } else { format!("Szene mit {n} Elementen.") },
    );
    if !counts.is_empty() {
        en.push_str(&format!(" {}.", counts.iter().map(|(k, c)| format!("{}: {c}", kind_text(*k).1)).collect::<Vec<_>>().join("; ")));
        de.push_str(&format!(" {}.", counts.iter().map(|(k, c)| format!("{}: {c}", kind_text(*k).2)).collect::<Vec<_>>().join("; ")));
    }
    let worst = scene.entities.iter().filter_map(|e| e.utilisation.as_ref().map(|u| (e, u))).fold(None, |best: Option<(&SceneEntity, &crate::UtilisationBinding)>, c| match best {
        Some(b) if b.1.ratio >= c.1.ratio => Some(b),
        _ => Some(c),
    });
    if let Some((e, u)) = worst {
        let (se, sd) = status(u.status);
        en.push_str(&format!(" Governing utilisation {} at {} ({se}).", percent(u.ratio), e.accessible_name.en));
        de.push_str(&format!(" Maßgebende Ausnutzung {} bei {} ({sd}).", percent(u.ratio), e.accessible_name.de));
    }
    let failing = scene.entities.iter().filter(|e| e.utilisation.as_ref().is_some_and(|u| u.status == CheckStatus::Fail)).count();
    if failing > 0 {
        en.push_str(&if failing == 1 { " 1 element fails.".to_owned() } else { format!(" {failing} elements fail.") });
        de.push_str(&format!(" {failing} {} nicht erfüllt.", if failing == 1 { "Element" } else { "Elemente" }));
    }
    let stale = scene.entities.iter().filter(|e| e.utilisation.as_ref().is_some_and(|u| u.stale)).count();
    if stale > 0 {
        en.push_str(&if stale == 1 { " 1 element awaits evaluation.".to_owned() } else { format!(" {stale} elements await evaluation.") });
        de.push_str(&if stale == 1 { " 1 Element wartet auf Auswertung.".to_owned() } else { format!(" {stale} Elemente warten auf Auswertung.") });
    }
    LocalizedCopy::new(en, de)
}

/// 📜️ One line per entity: accessible name, detail and utilisation, joined by newlines.
pub fn describe_entities(scene: &StructuralScene) -> LocalizedCopy {
    let line = |e: &SceneEntity| {
        let u = utilisation_text(e);
        (
            format!("{}: {}{}", e.accessible_name.en, e.accessible_detail.en, u.as_ref().map(|t| format!(", {}", t.0)).unwrap_or_default()),
            format!("{}: {}{}", e.accessible_name.de, e.accessible_detail.de, u.as_ref().map(|t| format!(", {}", t.1)).unwrap_or_default()),
        )
    };
    let lines: Vec<(String, String)> = scene.entities.iter().map(line).collect();
    LocalizedCopy::new(lines.iter().map(|l| l.0.as_str()).collect::<Vec<_>>().join("\n"), lines.iter().map(|l| l.1.as_str()).collect::<Vec<_>>().join("\n"))
}

/// 🇩🇪️ Every German text of a scene (description, entity names, details and labels, layer names, legend titles) that still reads as English, as `where: text`; see [`semio_s_plugin_norm_compliance::looks_english`].
pub fn german_scene_leaks(scene: &StructuralScene) -> Vec<String> {
    let english = semio_s_plugin_norm_compliance::looks_english;
    let mut texts: Vec<(String, &str)> = vec![("description".to_owned(), scene.description.de.as_str())];
    for e in &scene.entities {
        texts.push((format!("{} name", e.entity.id), &e.accessible_name.de));
        texts.push((format!("{} detail", e.entity.id), &e.accessible_detail.de));
        if let Some(label) = &e.label {
            texts.push((format!("{} label", e.entity.id), &label.de));
        }
    }
    texts.extend(scene.layers.iter().map(|l| (format!("layer {}", l.id), l.name.de.as_str())));
    texts.extend(scene.legends.iter().map(|l| (format!("legend {}", l.id), l.title.de.as_str())));
    texts.into_iter().filter(|(_, text)| english(text)).map(|(at, text)| format!("{at}: {text}")).collect()
}

/// 🇩🇪️ Every German text of a diagram (title, description, axis labels, marker labels, notes, scatter labels, table headers) that still reads as English.
pub fn german_diagram_leaks(diagram: &crate::Diagram2d) -> Vec<String> {
    let english = semio_s_plugin_norm_compliance::looks_english;
    let mut texts: Vec<&str> = vec![&diagram.title.de, &diagram.description.de];
    texts.extend(diagram.axes.iter().map(|a| a.label.de.as_str()));
    texts.extend(diagram.markers.iter().map(|m| m.label.de.as_str()));
    texts.extend(diagram.annotations.iter().map(|n| n.text.de.as_str()));
    texts.extend(diagram.table_twin.columns.iter().map(|c| c.header.de.as_str()));
    texts.into_iter().filter(|text| english(text)).map(|text| format!("{}: {text}", diagram.id)).collect()
}

/// 🇩🇪️ Every German accessible name of `handles` that still reads as English.
pub fn german_handle_leaks(handles: &[crate::Handle]) -> Vec<String> {
    handles.iter().filter(|h| semio_s_plugin_norm_compliance::looks_english(&h.accessible_name.de)).map(|h| format!("{}: {}", h.id, h.accessible_name.de)).collect()
}
