# Artifact Extern Scope

Pass558 gates the Norm artifacts' vcs aliases to test builds. A namespace scan found references only in test modules and documentation; native532 reported them unused in the libraries. The mutation tests retain their alias.

Removed four unused self aliases from DAG, Playground, Sequence and Wires. The remaining artifact-name references occur in separately compiled language-neutral fixture adapters; none of those adapter paths are mounted as unit modules by the artifact roots. The Wires attribute that had attached clippy::result_large_err to the unused extern declaration was removed with its obsolete explanation; that attribute did not govern handlers.

- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🧱️din4108/🦀️.rs`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🌬️din16798/🦀️.rs`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📇️iso16757/🦀️.rs`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪵️en1995/🦀️.rs`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🔩️en1993/🦀️.rs`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪶️en1999/🦀️.rs`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏭️vdi3805/🦀️.rs`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🌍️en1997/🦀️.rs`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚡️din18599/🦀️.rs`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏛️en1992/🦀️.rs`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚖️en1990/🦀️.rs`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🧩️en1994/🦀️.rs`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪨️en1996/🦀️.rs`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏋️en1991/🦀️.rs`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998/🦀️.rs`
- `✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/🦀️.rs`
- `✏️s/🔌️plugins/🎪️demonstrator/🗿️artifacts/🎪️playground/🦀️.rs`
- `✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🦀️.rs`
- `✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/🦀️.rs`

Fresh syntax, compiler and runtime verification remain required.


Pass562 syntax validation of passes558,559 and561: 22/22 files parsed with unchanged input hashes. The fresh native560 compiler check remains authoritative for compilation.
