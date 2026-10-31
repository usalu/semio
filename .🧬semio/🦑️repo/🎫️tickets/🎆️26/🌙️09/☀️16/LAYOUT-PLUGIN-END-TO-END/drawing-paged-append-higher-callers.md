# Higher Paged Append Callers

The explicit append API requires the original byte body independently of its paid grant, and returns actual work on success and failure. The replication identity cursor now forwards normal append and close outcomes directly, and reads ControlledRetirement work before interpreting a close result. This is source-only and remains partial: constructor/take ports and original owning tests still require the same outcome contract and genuine finite test stock. The fixed three empty output headers now use explicit initializers rather than an opaque from_fn callback.

Its reached receiver is Store `canonical-edit/source.rs`. That defining owner still has Result-only normal/close ports, obsolete Source constructor/projection ports, and two alias outputs temporarily held across subsequent fallible work. These need actual staged alias slots and direct child work, including the semantic and hash cursors. Dropping work through a result-only projection is not an acceptable intermediate implementation.

Store `one-item-owners/lifted/publication.rs` also calls append in `take_staged_prepared`, with a single DurableGroup receiver in `staged-root-retirement-and-commit-phases`. Its current return port lacks CPU, precharges capacity, and its stage close discards lower work. Both normal and close paths need full outcomes and retained output before receipt folding. The owning receiver must forward that outcome through its genuine control boundary; no default work unit or grant derived from demand is permitted.

These are higher reverse consumers and are outside the Context production gate. Their paths and current hashes are in `drawing-paged-append-higher-callers.json`. No compiler or test was run on the partial identity migration.
