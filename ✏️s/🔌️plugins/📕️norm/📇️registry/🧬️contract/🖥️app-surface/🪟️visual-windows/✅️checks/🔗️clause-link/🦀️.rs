//! 🔗️ Clause link of the checks window: the localized clause reference and its optional open-clause action. See [`super`].

use super::*;

/// 🔗️ Localized clause reference, `EN 1993 1-1 §6.2.5` / `EN 1993 1-1 Abschn. 6.2.5`.
pub fn clause_text(clause: &ClauseId, locale: Locale) -> String {
    clause.text(&locale)
}

/// 🔗️ Natural ordering of clause sections: `6.2.10` after `6.2.9`.
pub fn clause_order(left: &ClauseId, right: &ClauseId) -> std::cmp::Ordering {
    let key = |clause: &ClauseId| -> Vec<u64> { clause.section.split(|c: char| !c.is_ascii_digit()).filter(|part| !part.is_empty()).map(|part| part.parse().unwrap_or(0)).collect() };
    (&left.family, &left.part, key(left), &left.section).cmp(&(&right.family, &right.part, key(right), &right.section))
}

/// 🔗️ Keyboard-activatable row opening the clause panel; the action is only bound once the clause panel declares `openClause`.
pub fn clause_link_row(id: String, clause: &ClauseId, locale: Locale, controller_id: Option<&'static str>) -> UiAssemblyResult<BuiltNode> {
    let label = norm_ui_label(chrome("Clause", "Klausel", locale))?;
    let text = clause_text(clause, locale);
    match controller_id {
        Some(controller_id) => {
            let args = action_args_map(vec![("family", ui_value_text(&clause.family)?), ("part", ui_value_text(&clause.part)?), ("section", ui_value_text(&clause.section)?)])?;
            tree_item_with_action(id, label, Some(text), norm_action(controller_id, "openClause", Some(args))?)
        }
        None => tree_item_desc(id, label, Some(text)),
    }
}
