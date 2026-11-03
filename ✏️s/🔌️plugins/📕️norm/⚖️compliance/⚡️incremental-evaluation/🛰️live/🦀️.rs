//! 🛰️ Live evaluation: the retained `SharedEvaluation` of one artifact, declared once next to its evaluator and read by editor and viewer alike.

/// 🔦 Declares a unit source that folds the whole document into one unit running `$evaluate`; the fallback for families without per-entity check units.
#[macro_export]
macro_rules! live_fold_source {
    ($source:ident, $document:ty, $evaluate:path) => {
        pub struct $source;

        impl $crate::incremental_evaluation::UnitSource for $source {
            type Document = $document;

            fn units(_document: &$document) -> ::std::vec::Vec<$crate::incremental_evaluation::CheckUnit<$document>> {
                ::std::vec![$crate::incremental_evaluation::CheckUnit::new(
                    $crate::incremental_evaluation::UnitId::new("document", "all"),
                    ::std::vec::Vec::new(),
                    $crate::incremental_evaluation::ReadSet::new(["*"]),
                    $crate::incremental_evaluation::CostTier::T2,
                    |document: &$document| $evaluate(document).checks,
                )]
            }
        }
    };
}

/// 🛰️ Declares `mod $module` with the retained session of one artifact: `report(&Document)` settles exactly the given document incrementally and never through a manual evaluate step and returns the retained report as one shared `Arc` (no copy per paint or tick), `progress()` reads the latest run for the transient lane and `cancel()` supersedes the run in flight.
#[macro_export]
macro_rules! live_report {
    ($vis:vis mod $module:ident($document:ty) = $source:ty) => {
        $vis mod $module {
            use super::*;

            fn session() -> &'static ::std::sync::Arc<$crate::incremental_evaluation::SharedEvaluation<$source>> {
                static SESSION: ::std::sync::OnceLock<::std::sync::Arc<$crate::incremental_evaluation::SharedEvaluation<$source>>> = ::std::sync::OnceLock::new();
                SESSION.get_or_init(|| ::std::sync::Arc::new($crate::incremental_evaluation::SharedEvaluation::new(<$document as ::std::default::Default>::default())))
            }

            pub fn report(document: &$document) -> ::std::sync::Arc<$crate::CheckReport> {
                session().report_for(document)
            }

            /// 🧵️ Non-blocking catch-up: cheap units settle at once, slow (T2) units run on a worker thread; `None` when the session is already settled on `document`.
            pub fn spawn(document: &$document) -> ::std::option::Option<$crate::incremental_evaluation::Worker> {
                session().background(document)
            }

            /// 👀️ The report as it stands (partially settled while a worker runs); read `settled()` to know when it is the full evaluation.
            pub fn report_now() -> ::std::sync::Arc<$crate::CheckReport> {
                session().report_now()
            }

            /// 🖼️ Paint path that never freezes the editor: starts the background catch-up, waits at most `budget` for it (polling every 100 µs, so a unit that settles in a few hundred µs does not wait a whole millisecond) and returns the report as it stands plus whether it is the full evaluation (`false` while slow units are still running; `progress()` then tells how far, `cancel()` or the next edit supersedes them).
            pub fn responsive(document: &$document, budget: ::std::time::Duration) -> (::std::sync::Arc<$crate::CheckReport>, bool) {
                if let ::std::option::Option::Some(worker) = spawn(document) {
                    let deadline = ::std::time::Instant::now() + budget;
                    while !worker.is_finished() && ::std::time::Instant::now() < deadline {
                        ::std::thread::sleep(::std::time::Duration::from_micros(100));
                    }
                }
                (report_now(), settled())
            }

            pub fn settled() -> bool {
                session().settled()
            }

            pub fn progress() -> $crate::incremental_evaluation::EvalProgress {
                session().progress()
            }

            pub fn cancel() {
                session().cancel_handle().cancel()
            }
        }
    };
}
