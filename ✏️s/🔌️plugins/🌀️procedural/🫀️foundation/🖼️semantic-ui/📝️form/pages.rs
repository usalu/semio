//! 📄️ Record admission for contiguous windows of complete form controls.

use std::ops::Range;

#[derive(Clone, Copy)]
pub(super) struct FieldCost {
    pub nodes: usize,
    pub section: Option<usize>,
}

#[derive(Debug, PartialEq, Eq)]
pub(super) struct FormPage {
    pub index: usize,
    pub total: usize,
    pub fields: Range<usize>,
    pub nodes: usize,
}

/// 📏️ Seats whole fields in order and returns the page at the requested ordinal, clamped to the last page.
pub(super) fn form_page(fields: impl IntoIterator<Item = FieldCost>, budget: usize, requested: usize) -> Result<FormPage, usize> {
    let mut index = 0;
    let mut first = 0;
    let mut last = 0;
    let mut nodes = 0;
    let mut section = None;
    let mut selected = None;
    for (ordinal, field) in fields.into_iter().enumerate() {
        let opening_cost = field.nodes.checked_add(usize::from(field.section.is_some())).ok_or(ordinal)?;
        if field.nodes == 0 || opening_cost > budget {
            return Err(ordinal);
        }
        let cost = field.nodes + usize::from(field.section.is_some() && field.section != section);
        if cost > budget - nodes {
            if index == requested {
                selected = Some((index, first..ordinal, nodes));
            }
            index += 1;
            first = ordinal;
            nodes = opening_cost;
        } else {
            nodes += cost;
        }
        section = field.section;
        last = ordinal + 1;
    }
    let (page, fields, nodes) = selected.unwrap_or((index, first..last, nodes));
    Ok(FormPage { index: page, total: index + 1, fields, nodes })
}

#[cfg(test)]
#[path = "pages.test.rs"]
mod tests;
