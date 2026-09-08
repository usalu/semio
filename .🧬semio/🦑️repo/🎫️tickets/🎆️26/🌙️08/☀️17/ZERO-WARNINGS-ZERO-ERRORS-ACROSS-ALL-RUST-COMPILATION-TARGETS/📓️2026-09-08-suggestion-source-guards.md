# Compiler Suggestion Source Guards

Pass595 strengthens this ticket's existing compiler-suggestion helper. It retains each source buffer used to plan edits, requires the whole current file to still match before applying that plan, verifies each original byte span, and checks the source snapshot again immediately before writing. Concurrent changes are skipped and reported. This prevents a suggestion planned from one source revision from being written over another contributor's later revision.

Changed file: `📜️script.ts` in this ticket. No compiler suggestions were applied by this change. The helper remains restricted by its existing eligibility checks; only individually reviewed lint categories should be applied. Fresh compiler validation remains pending.
