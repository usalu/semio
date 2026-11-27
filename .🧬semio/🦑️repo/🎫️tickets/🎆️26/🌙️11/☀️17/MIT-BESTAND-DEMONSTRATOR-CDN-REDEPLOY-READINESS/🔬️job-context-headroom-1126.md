# job-context headroom (2026-11-26)

Before: closure of semio-framework-job-context = 17 files, 262450 of 262144 bytes (root 90425, source 45022, owned 23904, continuation source 22547, child 15970, ...). StepContext (53 KB impl) and every source module are cyclic with it, so only a split with the context core below and thin consumers above works.

Finding: another agent was already mid-carve at 10:41-10:44 (new crate semio-framework-job-context-step under 🧱️step, root 🦀️.rs moved under it, job-context turned into a facade 🪟️facade that path-includes source admission, close producer, child source and return place). I did not edit anything to avoid last-writer-wins clashes. Remaining: finish that carve, register the step crate in the workspace/lockfiles, then re-measure.
