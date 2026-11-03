//! 🧪️ The laws every artifact's incremental units must satisfy, declared once and instantiated per artifact next to its units.

/// 🧪️ Declares the five incremental-evaluation laws for one artifact: fold equals full filed under its owners (`owned`), every unit declares its reads, an edit re-runs exactly the units whose declared reads changed (the global and part units stay clean under an edit of a row field they do not read), ids are unique with the global unit first, and the retained session equals the full evaluation after random edits.
#[macro_export]
macro_rules! incremental_laws {
    (source = $source:ty, full = $full:path, live = $live:path, documents = $documents:expr) => {
        #[test]
        fn the_ordered_fold_of_units_equals_the_full_evaluation_exactly() {
            let owned_full = |document: &<$source as $crate::incremental_evaluation::UnitSource>::Document| $crate::incremental_evaluation::owned::<$source>(document, $full(document));
            for document in $crate::incremental_evaluation::with_every_collection($documents) {
                assert_eq!($crate::incremental_evaluation::evaluate_units::<$source>(&document), owned_full(&document));
            }
        }

        #[test]
        fn every_unit_declares_all_it_reads() {
            use $crate::incremental_evaluation::{audit_reads, UnitSource};
            for document in $crate::incremental_evaluation::with_every_collection($documents) {
                for unit in <$source as UnitSource>::units(&document) {
                    assert_eq!(audit_reads(&unit, &document), Vec::<String>::new(), "undeclared reads of {:?}", unit.id);
                }
            }
        }

        #[test]
        fn an_edit_re_runs_exactly_the_units_whose_declared_reads_changed() {
            use $crate::incremental_evaluation::{leaf_edit, read_hash_of, ChangedSet, EvaluationSession, SessionEvent, UnitSource};
            for base in $crate::incremental_evaluation::with_every_collection($documents) {
                let mut session = EvaluationSession::<$source>::open(base.clone());
                let first = session.plan(&ChangedSet::all(), &[], session.token());
                session.run(&first, &mut |_| {});
                let mut current = base;
                for seed in 0..40u64 {
                    let Some(next) = leaf_edit(&current, seed) else { continue };
                    let known: ::std::collections::HashMap<_, _> = <$source as UnitSource>::units(&current).into_iter().map(|unit| (unit.id.clone(), read_hash_of(&current, &unit.reads))).collect();
                    let expected: ::std::collections::BTreeSet<_> = <$source as UnitSource>::units(&next).into_iter().filter(|unit| known.get(&unit.id) != Some(&read_hash_of(&next, &unit.reads))).map(|unit| unit.id).collect();
                    let plan = session.update(next.clone(), &ChangedSet::all(), &[]);
                    let mut reran = ::std::collections::BTreeSet::new();
                    session.run(&plan, &mut |event| {
                        if let SessionEvent::Done(done) = event {
                            if done.rerun {
                                reran.insert(done.id);
                            }
                        }
                    });
                    assert_eq!(reran, expected, "seed {seed}: a unit re-runs when, and only when, a path it declares changed");
                    current = next;
                }
            }
        }

        #[test]
        fn unit_ids_are_unique_and_the_global_unit_comes_first() {
            use $crate::incremental_evaluation::UnitSource;
            for document in $crate::incremental_evaluation::with_every_collection($documents) {
                let units = <$source as UnitSource>::units(&document);
                let ids: ::std::collections::BTreeSet<_> = units.iter().map(|unit| unit.id.clone()).collect();
                assert_eq!(ids.len(), units.len());
                assert_eq!(units.first().map(|unit| unit.id.0.as_str()), Some("global:document"));
            }
        }

        #[test]
        fn the_retained_session_equals_the_full_evaluation_after_random_edits() {
            let owned_full = |document: &<$source as $crate::incremental_evaluation::UnitSource>::Document| $crate::incremental_evaluation::owned::<$source>(document, $full(document));
            use $crate::incremental_evaluation::{leaf_edit, ChangedSet, SharedEvaluation};
            for base in $crate::incremental_evaluation::with_every_collection($documents) {
                let shared = SharedEvaluation::<$source>::new(base.clone());
                let mut current = base;
                assert_eq!(*shared.report_for(&current), owned_full(&current));
                for seed in 0..40u64 {
                    let Some(next) = leaf_edit(&current, seed) else { continue };
                    let committed = if seed % 2 == 0 { shared.commit(next.clone(), &ChangedSet::all(), &[]) } else { shared.report_for(&next) };
                    assert_eq!(*committed, owned_full(&next), "seed {seed}");
                    assert_eq!(*$live(&next), owned_full(&next), "live seed {seed}");
                    current = next;
                }
            }
        }
    };
}
