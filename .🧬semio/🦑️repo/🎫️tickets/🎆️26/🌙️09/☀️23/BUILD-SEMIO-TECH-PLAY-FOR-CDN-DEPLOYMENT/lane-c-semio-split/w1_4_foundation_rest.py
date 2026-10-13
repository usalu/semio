#!/usr/bin/env python3
"""🧱️ W1 step 4 (split plan 1d remainder): mutation-outcome refusals, editor plumbing, examples, the snapshot retirement
factory with its member-owner macro, `applied` and `semio_written` move into the foundation tree."""
import sys
sys.path.insert(0, __file__.rsplit("/", 1)[0])
from semio_split import *
from use_rewrite import rewrite_uses, rewrite_linear

S = lambda rel: SUBSETS + "/" + rel
FOUNDATION = S("✉️base/🦀️.rs")

mut = S("✉️base/🧬️schema/🧬️mutations/🦀️.rs")
src, texts, first = extract_functions(read(mut), ["semio_mutation_refusals", "semio_mutation_refusal_codes"])
keep = used_names(src, ["semio_mutation_refusals", "semio_mutation_refusal_codes"])
if keep:
    src = src[:first] + "use crate::base_foundation::schema::mutations::outcome::{%s};\n" % ", ".join(keep) + src[first:]
write(mut, src)
write(S("✉️base/🧬️schema/🧬️mutations/🎯️outcome/🦀️.rs"), "//! 🎯️ Reads the refusals out of any semio mutation outcome, for adapters that cannot name the kernel's protocol types.\n\n" + "".join(texts).rstrip("\n") + "\n")

ed = S("✉️base/✏️editor/🦀️.rs")
e = read(ed)
mount = '#[path = "🧭️edit-plumbing/🦀️.rs"]\npub(crate) mod edit_plumbing;\n'
assert e.count(mount) == 1
write(ed, e.replace(mount, ""))

root = read(ROOT_RS)
ex_block = '''#[path = "."]
pub mod examples {
    #[path = "🏅️standards/🔖️v1/🪆️subsets/✉️base/📚️examples/🎬️demo/🦀️.rs"]
    pub mod demo;
    #[path = "🏅️standards/🔖️v1/🪆️subsets/✉️base/📚️examples/📃️note/🦀️.rs"]
    pub mod note;
}
'''
assert root.count(ex_block) == 1
root = root.replace(ex_block, "")
base_examples = []
for m in re.finditer(r'#\[path = "🏅️standards/🔖️v1/🪆️subsets/✉️base/📚️examples/([^/]+)/🦀️.rs"\]\npub mod v1_subsets_base_examples_([a-z_]+);\n', root):
    base_examples.append((m.group(1), m.group(2)))
root = re.sub(r'#\[path = "🏅️standards/🔖️v1/🪆️subsets/✉️base/📚️examples/[^/]+/🦀️.rs"\]\npub mod v1_subsets_base_examples_[a-z_]+;\n', "", root)
base_examples = sorted(set(base_examples) | {("🎬️demo", "demo"), ("📃️note", "note")}, key=lambda x: x[1])

a = root.index("#[derive(semio_framework_value::FactoryPayloadRetirement)]\nstruct SemioSnapshotRetirementFactory")
b = root.index("semio_subset_table!(member_owners);\n") + len("semio_subset_table!(member_owners);\n")
members = []
for name in NAMES:
    cap = name.capitalize()
    snap = "subsets::%s::schema::snapshot::Semio%sSnapshot" % (name, cap)
    mutn = "subsets::%s::schema::mutations::Semio%sMutation" % (name, cap)
    open_ = ", open = subsets::flow::io::binary::snapshot::SemioFlowSnapshotDecode" if name == "flow" else ""
    members.append("crate::semio_member_owner!(%s, %s%s);" % (snap, mutn, open_))
root = root[:a] + "\n".join(members) + "\n" + root[b:]
a = root.index("/// 📦️ Which retained snapshot decoder a subset's member opens through")
b = root.index("}\nuse subsets::{\n", a) + 2
root = root[:a] + root[b:]
root = root.replace("use std::{marker::PhantomData, sync::Arc};\n", "")
ap = root.index("/// 🧪️ Diff then central apply")
ap_end = root.index("}\n", root.index("pub(crate) fn applied", ap)) + 2
root = root[:ap] + root[ap_end:]
sw = root.index("/// 🎹️ The rows of `table` that write a `s.stdio.semio` dialect")
sw_end = root.index("}\n", root.index("pub(crate) fn semio_written", sw)) + 2
semio_written = root[sw:sw_end]
root = root[:sw] + root[sw_end:]
sid = "/// 🧬️ Artifact schema descriptor id.\npub const SEMIO_ARTIFACT_SCHEMA_ID: &str = \"s.stdio.semio\";\n"
assert root.count(sid) == 1
root = root.replace(sid, "")
write(ROOT_RS, root)

semio_written = semio_written.replace("pub(crate) fn semio_written", "pub fn semio_written")
f = read(FOUNDATION).rstrip("\n")
f = f.replace('''    #[path = "🧬️schema/🧰️triples/🦀️.rs"]
    pub mod triples;
}''', '''    #[path = "🧬️schema/🧰️triples/🦀️.rs"]
    pub mod triples;
    #[path = "."]
    pub mod mutations {
        #[path = "🧬️schema/🧬️mutations/🎯️outcome/🦀️.rs"]
        pub mod outcome;
    }
}''')
f += '''

#[cfg(feature = "component-app-assembly")]
#[path = "."]
pub mod editor {
    #[path = "✏️editor/🧭️edit-plumbing/🦀️.rs"]
    pub mod edit_plumbing;
}

#[path = "."]
pub mod examples {
''' + "".join('    #[path = "📚️examples/%s/🦀️.rs"]\n    pub mod %s;\n' % (d, n) for d, n in base_examples) + '''}

/// 🧬️ Artifact schema descriptor id of the semio artifact.
pub const SEMIO_ARTIFACT_SCHEMA_ID: &str = "s.stdio.semio";

''' + semio_written + '''
/// 🧪️ Diff then central apply: the next snapshot (the base when the diff is refused) and the mutation's own outcome.
pub fn applied<P: Clone, M: protocol::Mutation<P>>(base: &P, mutation: &M) -> (P, protocol::MutationOutcome<M::Diff>) {
    let outcome = mutation.diff(base);
    let next = protocol::apply_diff(outcome.diff(), base).unwrap_or_else(|_| base.clone());
    (next, outcome)
}

/// 🧹️ Retires a subset's shared snapshot through the bounded shared-retirement admission; `semio_member_owner!` gives
/// each subset snapshot its implementation.
#[derive(semio_framework_value::FactoryPayloadRetirement)]
pub struct SemioSnapshotRetirementFactory<P>(pub std::marker::PhantomData<fn() -> P>);

/// 🧸️ Implements one subset's snapshot retirement factory and member store owner: retained snapshots open through
/// the framework's bounded pack decoder unless `open = <decoder>` names the subset's own streaming decoder.
#[macro_export]
macro_rules! semio_member_owner {
    ($snapshot:ty, $mutation:ty) => {
        $crate::semio_member_owner!($snapshot, $mutation, open = ::semio_framework_os_kernel::PackMemberSnapshotOpen<$snapshot>);
    };
    ($snapshot:ty, $mutation:ty, open = $open:ty) => {
        impl ::semio_framework_os_kernel::SnapshotRetirementFactory<$snapshot> for $crate::base_foundation::SemioSnapshotRetirementFactory<$snapshot> {
            fn retirement_birth_bytes(&self, _snapshot: &::std::sync::Arc<$snapshot>) -> usize {
                ::semio_framework_value::retirement::shared::shared_retirement_birth_bytes::<$snapshot>()
            }

            fn retire(
                &self,
                snapshot: ::std::sync::Arc<$snapshot>,
                grant: ::semio_framework_value::retained_clone::RetainedCloneGrant,
            ) -> Result<(Box<dyn ::semio_framework_os_kernel::ErasedSnapshotRetirement>, ::semio_framework_value::retained_clone::RetainedCloneProgress), (::semio_framework_value::ValueError, ::std::sync::Arc<$snapshot>)> {
                ::semio_framework_value::retirement::shared::admit_shared_retirement(snapshot, grant, true)
            }
        }

        impl ::semio_framework_os_kernel::MemberStoreOwner<$mutation> for $snapshot {
            type SnapshotOpen = $open;

            fn member_store_owners_birth_demand() -> Result<::semio_framework_value::retained_clone::RetainedCloneBirthDemand, ::semio_framework_value::ValueError> {
                ::semio_framework_os_kernel::bounded_artifact_store_owners_birth_demand::<Self, $mutation>()
            }

            fn member_store_owners(grant: ::semio_framework_value::retained_clone::RetainedCloneGrant) -> Result<(::semio_framework_os_kernel::DocumentStoreOwners<Self, $mutation>, ::semio_framework_value::retained_clone::RetainedCloneProgress), ::semio_framework_os_kernel::DocumentStoreOwnersAdmissionError<Self, $mutation>> {
                ::semio_framework_os_kernel::bounded_artifact_store_owners::<Self, $mutation>(grant)
            }
        }
    };
}
'''
write(FOUNDATION, f)

rules = [(r"\bcrate::semio_written\b", "crate::base_foundation::semio_written"),
         (r"\bcrate::applied\b", "crate::base_foundation::applied"),
         (r"\b(crate|semio_s_artifact_stdio_semio)::examples::", r"\1::base_foundation::examples::"),
         (r"\b(crate|semio_s_artifact_stdio_semio)::editor::semio_base::edit_plumbing\b", r"\1::base_foundation::editor::edit_plumbing"),
         (r"\b(crate|semio_s_artifact_stdio_semio)::standards::v1::subsets::base::schema::mutations::(semio_mutation_refusals|semio_mutation_refusal_codes)\b", r"\1::base_foundation::schema::mutations::outcome::\2"),
         (r"\b(crate|semio_s_artifact_stdio_semio)::SEMIO_ARTIFACT_SCHEMA_ID\b", r"\1::base_foundation::SEMIO_ARTIFACT_SCHEMA_ID")]


def mapper(seg):
    head = seg[:1]
    if head not in (["crate"], ["semio_s_artifact_stdio_semio"]):
        return None
    if seg[1:8] == ["standards", "v1", "subsets", "base", "schema", "mutations"] + seg[7:8] and len(seg) == 8 and seg[7] in ("semio_mutation_refusals", "semio_mutation_refusal_codes"):
        return head + ["base_foundation", "schema", "mutations", "outcome", seg[7]]
    if seg[1:4] == ["editor", "semio_base", "edit_plumbing"]:
        return head + ["base_foundation", "editor", "edit_plumbing"] + seg[4:]
    return None


changed = 0
for p in list(semio_rust_files()) + consumer_rust_files():
    s = read(p)
    s2, a = rewrite_linear(s, rules)
    s3, b = rewrite_uses(s2, mapper)
    if s3 != s:
        write(p, s3)
        changed += 1
squeeze_blank_runs([ROOT_RS, mut, ed])
print("rewrote", changed, "files; base examples:", [n for _, n in base_examples])
