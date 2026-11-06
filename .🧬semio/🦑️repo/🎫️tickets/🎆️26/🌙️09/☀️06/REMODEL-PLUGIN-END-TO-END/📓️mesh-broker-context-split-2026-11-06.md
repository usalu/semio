# Mesh broker and job-context split review

Mesh broker close_turn synthesizes work from nonzero progress while typed-owner and snapshot lease close branches do not propagate child work; index close receives unlimited fuel. Native agent notified to verify delegated work quote and failure receipt before execution.

The job context retained_work module at framework job work/grant/🦀️.rs is an independent RetainedWorkBudget owner importing retained-clone and value-derive only, with no StepContext or super dependency. It is a candidate for an independently packaged firstparty owner, reexported by context with actual consumers updated; moving only the module path within the same crate would not reduce source closure. No build or measurement performed.

Grant owner depends on retained-clone contract and value-derive only. Context mounts it as retained_work and uses RetainedWorkBudget at line 243; existing job grant tests import semio_framework_job::retained_work::RetainedWorkBudget. Preserve that facade as a reexport of the real new owner. Existing owned rows tests observe real allocations during advance and close, with cancellation cuts; final output retirement is funded but not wrapped in the observer. No compile run.
