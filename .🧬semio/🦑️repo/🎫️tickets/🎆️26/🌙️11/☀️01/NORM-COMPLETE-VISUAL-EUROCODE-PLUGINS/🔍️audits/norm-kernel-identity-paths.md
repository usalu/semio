# Norm Kernel Identity Repairs

The actual canonical package is semio-os-kernel. EN1990 Cargo failed before Rust on its aggregate's two stale inherited dependency keys. The aggregate now names the canonical workspace key, and current norm viewers/editor tests/physical-law crate references use that same defining Rust crate. No alias, package shim, dependency duplication or compatibility reexport was added.

- ✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚖️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📦️packages/🦀️rust/Cargo.toml (2 references)
- ✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪶️en1999/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs (3 references)
- ✏️s/🔌️plugins/📕️norm/🗿️artifacts/🧱️din4108/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs (2 references)
- ✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚖️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs (3 references)
- ✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚡️din18599/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs (2 references)
- ✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs (3 references)
- ✏️s/🔌️plugins/📕️norm/🗿️artifacts/🌍️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs (3 references)
- ✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏋️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs (3 references)
- ✏️s/🔌️plugins/📕️norm/🗿️artifacts/🌬️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs (2 references)
- ✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪵️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs (3 references)
- ✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚖️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧪️tests/🧮️physical/🦀️.rs (2 references)
- ✏️s/🔌️plugins/📕️norm/🗿️artifacts/🔩️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs (3 references)
- ✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏛️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs (3 references)
- ✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪨️en1996/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs (3 references)
- ✏️s/🔌️plugins/📕️norm/🗿️artifacts/🧩️en1994/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs (3 references)
- ✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏭️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs (2 references)

## Plugin Library Identity

Each affected source owner's Cargo manifest already declares the canonical semio-plugin dependency. The actual target library identity is semio_plugin. Fourteen viewer/editor test sources still used the removed semio_framework_plugin name; their exact identifier references now match their defining dependency, after checking every current source owner before writing. No alias was introduced. Native compilation remains pending the wider source closure.

- ✏️s/🔌️plugins/📕️norm/🗿️artifacts/🔩️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs (39 references)
- ✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚡️din18599/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs (34 references)
- ✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs (39 references)
- ✏️s/🔌️plugins/📕️norm/🗿️artifacts/🌍️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs (39 references)
- ✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏋️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs (39 references)
- ✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪶️en1999/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs (39 references)
- ✏️s/🔌️plugins/📕️norm/🗿️artifacts/🧩️en1994/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs (39 references)
- ✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪨️en1996/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs (39 references)
- ✏️s/🔌️plugins/📕️norm/🗿️artifacts/🧱️din4108/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs (33 references)
- ✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪵️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs (39 references)
- ✏️s/🔌️plugins/📕️norm/🗿️artifacts/🌬️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs (32 references)
- ✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏭️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs (32 references)
- ✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏛️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs (39 references)
- ✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚖️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs (39 references)
