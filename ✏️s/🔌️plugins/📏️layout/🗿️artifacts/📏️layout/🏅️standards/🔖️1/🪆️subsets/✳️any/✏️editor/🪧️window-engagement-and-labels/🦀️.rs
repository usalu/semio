//! 🪧️ Window engagement and labels of the layout play app: the window engagement record and the entity label with its excerpt helper. See [`super`].

use super::*;

//#region 🔖️WindowEngagement
pub(super) fn layout_window_engagement(config: &LayoutWindowConfig, transient: &LayoutWindowTransient, label: &str, labels: &LayoutLabels) -> WindowEngagement {
    WindowEngagement {
        session_active: Some(false),
        options: None,
        input: Some(WindowEngagementInput {
            id: Some(format!("layout-engagement-{label}")),
            value: Some(transient.engagement_input.clone()),
            placeholder: Some("undo, redo, export png".into()),
            disabled: None,
            on_change: Some(ActionDescriptor { controller_id: LAYOUT_PLAY_APP_ID.into(), action: "engagementInput".into(), args: None }),
            on_submit: Some(ActionDescriptor { controller_id: LAYOUT_PLAY_APP_ID.into(), action: "engagementSubmit".into(), args: None }),
            on_repeat_last: None,
            on_abort: None,
        }),
        control: None,
        controls: None,
        status: Some(vec![WindowEngagementStatus { id: format!("layout-status-{label}"), text: format!("{} {}", labels.page.as_str(), config.active_page_id) }]),
        possible_engagements: Some(vec![
            WindowEngagementPossible { id: "layout.eng.undo".into(), label: labels.undo.into(), detail: None, action: Some(ActionDescriptor { controller_id: LAYOUT_PLAY_APP_ID.into(), action: "undo".into(), args: None }) },
            WindowEngagementPossible { id: "layout.eng.redo".into(), label: labels.redo.into(), detail: None, action: Some(ActionDescriptor { controller_id: LAYOUT_PLAY_APP_ID.into(), action: "redo".into(), args: None }) },
        ]),
    }
}
//#endregion 🔖️WindowEngagement

//#region 🪧️EntityLabels
/// 🪧️ What a history-edit reference chip reads for the layout entity `id` (gap N3): a frame (`kinds` names `frame`, or
/// none is declared) by its kind word — a text frame quoting the start of its story, an image frame its linked file, any
/// other frame its ordinal among the frames of its kind on its page when there are several — then its page name. Pages,
/// layers and styles name themselves through the framework's generic walk (their `name`); `None` for every other id.
pub fn layout_entity_label(document: &LayoutSnapshot, kinds: &[String], id: &str) -> Option<LocalizedLabel> {
    if !(kinds.is_empty() || kinds.iter().any(|kind| kind == "frame")) {
        return None;
    }
    let pages = document.pages.iter().map(|page| (page.name.as_str(), page.frames.as_slice())).chain(document.parent_pages.iter().map(|page| (page.name.as_str(), page.frames.as_slice())));
    let (page, frames) = pages.into_iter().find(|(_, frames)| frames.iter().any(|frame| frame.id() == id))?;
    let frame = frames.iter().find(|frame| frame.id() == id)?;
    let quoted = match frame {
        Frame::Text { story_id, .. } => document.stories.iter().find(|story| &story.id == story_id).and_then(|story| layout_label_excerpt(&story.content)),
        Frame::Image { link_id, .. } => document.links.iter().find(|link| &link.id == link_id).and_then(|link| link.path.rsplit(['/', '\\']).next().and_then(layout_label_excerpt)),
        Frame::Rect { .. } => None,
    };
    let siblings: Vec<&Frame> = frames.iter().filter(|other| other.kind_str() == frame.kind_str()).collect();
    let ordinal = (siblings.len() > 1).then(|| siblings.iter().position(|other| other.id() == id).map_or(1, |index| index + 1));
    let (kind, page) = (frame.kind_str().to_string(), page.to_string());
    Some(LocalizedLabel::from_fn(move |terminology, locale| {
        let labels = <LayoutLabels as semio_framework_ui_locale::AppLabels>::labels(locale, terminology);
        let word = match kind.as_str() {
            "text" => labels.kind_text,
            "image" => labels.kind_image,
            _ => labels.kind_rect,
        }
        .as_str();
        let (open, close) = match locale {
            semio_framework_ui_locale::Locale::De => ("\u{201e}", "\u{201c}"),
            semio_framework_ui_locale::Locale::En => ("\u{201c}", "\u{201d}"),
        };
        match (&quoted, ordinal) {
            (Some(quoted), _) => format!("{word} {open}{quoted}{close} \u{b7} {page}"),
            (None, Some(ordinal)) => format!("{word} {ordinal} \u{b7} {page}"),
            (None, None) => format!("{word} \u{b7} {page}"),
        }
    }))
}

/// ✂️ The first non-blank line of `content`, trimmed and cut to [`LAYOUT_LABEL_EXCERPT_CHARS`] characters with an ellipsis.
fn layout_label_excerpt(content: &str) -> Option<String> {
    let line = content.lines().map(str::trim).find(|line| !line.is_empty())?;
    Some(if line.chars().count() > LAYOUT_LABEL_EXCERPT_CHARS { format!("{}\u{2026}", line.chars().take(LAYOUT_LABEL_EXCERPT_CHARS).collect::<String>()) } else { line.to_string() })
}

/// 📏️ The longest story or file excerpt a frame chip quotes.
const LAYOUT_LABEL_EXCERPT_CHARS: usize = 24;
//#endregion 🪧️EntityLabels
