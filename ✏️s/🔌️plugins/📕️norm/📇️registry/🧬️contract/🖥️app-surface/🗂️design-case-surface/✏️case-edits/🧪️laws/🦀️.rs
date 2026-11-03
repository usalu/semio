//! ⚖️ Case edit laws: one scripted editing session over a real artifact, the same for every code with design cases. An artifact test calls [`assert_case_edit_paths`] with its family, catalogue, edit resolver and `cases` accessor; every action path (add, set number, set text, typed row edit, clear, duplicate, remove) is resolved to the artifact's own mutations and applied, and after each step the live report equals the full evaluation. See [`super`].
use super::*;

fn matches_full<F: NormFamily>(host: &NormHost<F>, step: &str) {
    let full = F::evaluate(host.document());
    let shape = |report: &CheckReport| report.checks.iter().map(|check| (check.id.clone(), check.status, (check.utilization * 1e6).round() as i64)).collect::<Vec<_>>();
    assert_eq!(shape(host.report()), shape(&full), "{step}: the live report equals the full evaluation");
}

fn apply<F: NormFamily>(host: &mut NormHost<F>, resolve: fn(&F::Document, &NormEdit) -> Result<Vec<F::Mutation>, String>, edit: &NormEdit, step: &str)
where
    F::Mutation: protocol::ColdMutation<F::Document>,
{
    for mutation in resolve(host.document(), edit).unwrap_or_else(|error| panic!("{step}: {edit:?} does not resolve: {error}")) {
        host.apply(&mutation).unwrap_or_else(|error| panic!("{step}: the mutation does not apply: {error:?}"));
    }
}

fn answers<F: NormFamily>(host: &NormHost<F>, case_id: &str) -> usize {
    let prefix = format!("cases[id={case_id}]");
    host.report().checks.iter().filter(|check| check.subject.path.starts_with(&prefix)).count()
}

fn row_of(rows: &[CaseRow], id: &str) -> CaseRow {
    rows.iter().find(|row| row.id == id).cloned().unwrap_or_else(|| panic!("case {id} is in the document"))
}

/// ⚖️ Drives every case editing path of one artifact through its own mutations; `cases` reads the document's case rows, `resolve` is the artifact's edit-rule resolver (`EDIT_RULES.resolve`).
pub fn assert_case_edit_paths<F: NormFamily, C: CaseCatalogue>(document: F::Document, cases: fn(&F::Document) -> Vec<CaseRow>, resolve: fn(&F::Document, &NormEdit) -> Result<Vec<F::Mutation>, String>)
where
    F::Mutation: protocol::ColdMutation<F::Document>,
{
    let entry = C::entries().into_iter().find(|entry| design_case::declared::<C>(entry).iter().any(|spec| spec.kind == InputKind::Number && spec.example.is_some())).expect("a catalogue entry with a seeded number input");
    let spec = *design_case::declared::<C>(&entry).iter().find(|spec| spec.kind == InputKind::Number && spec.example.is_some()).expect("seeded number input");
    let mut host = NormHost::<F>::live(document);
    let before = cases(host.document());

    let add = insert_case_edit::<C>(&before, entry.id, before.len()).expect("add case");
    apply(&mut host, resolve, &add, "add");
    let rows = cases(host.document());
    assert_eq!(rows.len(), before.len() + 1, "add: one case more");
    let added = rows.last().cloned().expect("added case");
    assert_eq!(added.entry_id, entry.id, "add: the case verifies the picked entry");
    assert_eq!(added.numbers.iter().find(|n| n.id == spec.key).map(|n| n.value), spec.example, "add: inputs are seeded with the examples");
    assert!(answers(&host, &added.id) > 0, "add: the checks window lists the case");
    matches_full(&host, "add");

    let target = spec.range.map_or(spec.example.unwrap_or(1.0) * 1.25, |(min, max)| (min + (max - min) * 0.5).max(min));
    apply(&mut host, resolve, &set_number_edit(&added.id, spec.key, target), "set number");
    assert_eq!(row_of(&cases(host.document()), &added.id).numbers.iter().find(|n| n.id == spec.key).map(|n| n.value), Some(target), "set number: stored in the vocabulary unit");
    matches_full(&host, "set number");

    let input = case_input_rows::<C>(&DesignCase::from(&row_of(&cases(host.document()), &added.id))).into_iter().find(|row| row.spec.key == spec.key && row.spec.is_number()).expect("input row");
    let example = spec.example.unwrap_or(1.0);
    let text = if spec.unit == UnitId::One { format!("{example}") } else { format!("{example} {}", spec.unit.symbol()) };
    let typed = input_row_edit(&input, &text, spec.unit).expect("typed row edit");
    apply(&mut host, resolve, &typed, "typed row edit");
    let stored = row_of(&cases(host.document()), &added.id).numbers.iter().find(|n| n.id == spec.key).map(|n| n.value).expect("typed value");
    assert!((stored - example).abs() <= 1e-9 * example.abs().max(1.0), "typed row edit: the unit text lands in the stored unit");
    matches_full(&host, "typed row edit");

    if let Some(choice) = design_case::declared::<C>(&entry).into_iter().find(|spec| spec.kind == InputKind::Choice && !spec.choices.is_empty()) {
        let value = choice.choices[choice.choices.len() - 1].value;
        apply(&mut host, resolve, &set_text_edit(&added.id, choice.key, value), "set text");
        assert_eq!(row_of(&cases(host.document()), &added.id).texts.iter().find(|t| t.id == choice.key).map(|t| t.value.as_str()), Some(value), "set text: the choice is stored");
        matches_full(&host, "set text");
        apply(&mut host, resolve, &clear_input_edit(&added.id, choice.key, false), "clear text");
        assert!(row_of(&cases(host.document()), &added.id).texts.iter().all(|t| t.id != choice.key), "clear text: the null set removes the text row");
        matches_full(&host, "clear text");
    }

    apply(&mut host, resolve, &clear_input_edit(&added.id, spec.key, true), "clear number");
    assert!(row_of(&cases(host.document()), &added.id).numbers.iter().all(|n| n.id != spec.key), "clear number: the null set removes the number row");
    assert!(answers(&host, &added.id) > 0, "clear number: the case still answers (input issue or result)");
    matches_full(&host, "clear number");
    apply(&mut host, resolve, &set_number_edit(&added.id, spec.key, target), "restore number");

    let index = cases(host.document()).iter().position(|row| row.id == added.id).expect("index");
    let duplicate = duplicate_case_edit(&cases(host.document()), index).expect("duplicate");
    apply(&mut host, resolve, &duplicate, "duplicate");
    let rows = cases(host.document());
    assert_eq!(rows.len(), before.len() + 2, "duplicate: one case more");
    let copy = rows[index + 1].clone();
    assert_ne!(copy.id, added.id, "duplicate: fresh id");
    assert_eq!((copy.entry_id.as_str(), &copy.numbers, &copy.texts), (added.entry_id.as_str(), &row_of(&rows, &added.id).numbers, &row_of(&rows, &added.id).texts), "duplicate: same entry and inputs");
    assert!(answers(&host, &copy.id) > 0, "duplicate: the copy is checked");
    matches_full(&host, "duplicate");

    apply(&mut host, resolve, &remove_case_edit(&rows, index + 1).expect("remove"), "remove copy");
    let removal = remove_case_edit(&cases(host.document()), index).expect("remove");
    apply(&mut host, resolve, &removal, "remove");
    assert_eq!(cases(host.document()), before, "remove: back to the starting cases");
    assert_eq!(answers(&host, &added.id), 0, "remove: the case leaves the checks window");
    matches_full(&host, "remove");
}
