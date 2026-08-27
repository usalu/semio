# -*- coding: utf-8 -*-
"""🧬️ Adds `pub const KINDS`, its enum/catalog conformance test and the sync codec bridges the
`mutate-<slug>-1` case adapters need to the fifteen 📕️norm mutation and snapshot facets."""
import json, os, sys

ROOT = "/Users/ueli/Documents/semio"
TICKET = os.path.join(ROOT, ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️23/END-TO-END-TESTING-REFACTOR/w12-norm")
sys.path.insert(0, TICKET)
from meta import META

ART = os.path.join(ROOT, "✏️s/🔌️plugins/📕️norm/🗿️artifacts")
SUB = "🏅️standards/🔖️1/🪆️subsets/✳️any"
survey = json.load(open(os.path.join(TICKET, "survey.json"), encoding="utf-8"))
WAIVER = "// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9"

for art, info in survey.items():
    m = META[art]
    slug, ty, mod = m["slug"], m["ty"], m["mod"]
    kinds = [k["kind"] for k in info["kinds"]]
    sub_dir = os.path.join(ART, art, SUB)
    mut_file = os.path.join(sub_dir, "🧬️schema/🧬️mutations/🦀️component.rs")
    snap_file = os.path.join(sub_dir, "🧬️schema/📸️snapshot/🦀️component.rs")

    src = open(mut_file, encoding="utf-8").read()
    assert "pub const KINDS" not in src, mut_file
    kind_list = "\n".join(f'    "{k}",' for k in kinds)
    kinds_block = f'''
/// 🏷️ Every declared kind of [`{ty}Mutation`], in `#[derive(dsl::Mutations)]`'s own declaration
/// order and spelling — the list `../../🧪️oracle/🔣️.json` publishes as the `{slug}-1-any`
/// mutation catalog and `../../../../../🧪️tests/mutate-{slug}-1` registers its scenarios from. The
/// test platform never parses Rust, so [`kinds_catalog::kinds_match_the_enum_and_the_catalog`] below
/// is what keeps the enum, this const and the committed manifest from drifting apart.
pub const KINDS: &[&str] = &[
{kind_list}
];
'''
    marker = "//#endregion 🔖️Mutations"
    assert src.count(marker) == 1, mut_file
    src = src.replace(marker, kinds_block.rstrip("\n") + "\n" + marker, 1)

    bridges = f'''

//#region 🌉️ExternalCodecBridge
/// 📥️ Decodes this facet's own internally-tagged (`{{"mutation": "<camelCaseVariant>", …}}`) JSON
/// projection — the exact shape the committed `<kind>/🧪️tests/<fixture>/🦠️mutation/🔣️component.json`
/// specification vectors carry — into a real [`{ty}Mutation`]. The generated test host of
/// `../../../../../🧪️tests/mutate-{slug}-1` links only this crate, so `serde_json` is unreachable
/// from that adapter and the bridge belongs here rather than there.
{WAIVER}
pub fn decode_{slug}_mutation_json(text: &str) -> Result<{ty}Mutation, String> {{
    serde_json::from_str(text).map_err(|error| error.to_string())
}}

/// ▶️ Applies one mutation to `base`, returning the resulting document together with every
/// diagnostic its own diff builder raised, rendered as `<severity>:<code>` so no framework type
/// crosses this boundary. Built on the SYNC `Mutation::diff`/`MutationDiff::apply` pair this
/// facet's own committed fixture tests already call, not on the async `vcs::apply_mutation` wrapper.
{WAIVER}
pub fn apply_{slug}_mutation(base: &{ty}Snapshot, mutation: &{ty}Mutation) -> Result<({ty}Snapshot, Vec<String>), String> {{
    let raised = <{ty}Mutation as protocol::Mutation<{ty}Snapshot>>::diff(mutation, base);
    let messages = raised.messages().iter().map(|message| format!("{{:?}}:{{}}", message.level, message.code.0)).collect();
    let applied = <{ty}Diff as protocol::MutationDiff<{ty}Snapshot>>::apply(raised.diff(), base).map_err(|error| format!("{{error:?}}"))?;
    Ok((applied, messages))
}}

/// ↩️ This mutation's own computed inverse against `base` — the metamorphic property
/// `mutate-{slug}-1`'s `inverse-<kind>` scenarios assert, exposed under a name the test adapter can
/// reach without naming `protocol::Mutation`.
{WAIVER}
pub fn inverse_{slug}_mutation(mutation: &{ty}Mutation, base: &{ty}Snapshot) -> Vec<{ty}Mutation> {{
    <{ty}Mutation as protocol::Mutation<{ty}Snapshot>>::inverse(mutation, base)
}}
//#endregion 🌉️ExternalCodecBridge

//#region 🧪️KindsCatalog
#[cfg(test)]
mod kinds_catalog {{
    use super::*;

    /// 🏷️ [`KINDS`] must name every declared variant, in the exact order and spelling
    /// `#[derive(dsl::Mutations)]` assigns, and every one of those spellings must also appear in the
    /// committed `{slug}-1-any` catalog. The framework never parses Rust, so this is the only thing
    /// standing between a renamed variant and a completeness gate that silently measures the wrong
    /// set.
    #[test]
    fn kinds_match_the_enum_and_the_catalog() {{
        let descriptors = <{ty}Mutation as protocol::SemanticMutation<{ty}Snapshot>>::kinds();
        assert_eq!(KINDS.len(), descriptors.len(), "KINDS must name exactly one entry per declared {ty}Mutation variant");
        for (kind, descriptor) in KINDS.iter().zip(descriptors.iter()) {{
            assert_eq!(*kind, descriptor.kind, "KINDS must match #[derive(dsl::Mutations)]'s own declaration order and spelling");
        }}
        let manifest = include_str!("../../🧪️oracle/🔣️.json");
        for kind in KINDS {{
            assert!(manifest.contains(&format!("\\"{{kind}}\\"")), "KINDS entry {{kind:?}} must also appear in the committed oracle manifest's catalog");
        }}
    }}
}}
//#endregion 🧪️KindsCatalog
'''
    # 🧭️ Appended after the region that mounts the fixture tests, so the file's existing structure
    # is untouched.
    src = src.rstrip("\n") + "\n" + bridges
    open(mut_file, "w", encoding="utf-8").write(src)

    snap = open(snap_file, encoding="utf-8").read()
    assert f"encode_{slug}_snapshot_json" not in snap, snap_file
    snap_bridges = f'''

//#region 🌉️ExternalCodecBridge
/// 📤️ The canonical JSON projection of a [`{ty}Snapshot`] — the surface
/// `../../../../../🧪️tests/mutate-{slug}-1` is compared through under `ordered-json-v1`.
{WAIVER}
pub fn encode_{slug}_snapshot_json(snapshot: &{ty}Snapshot) -> String {{
    serde_json::to_string(snapshot).expect("{ty}Snapshot serialization is infallible")
}}

/// 📥️ The `serde_json` inverse of [`encode_{slug}_snapshot_json`] — decodes the committed
/// `../🧬️mutations/<kind>/🧪️tests/<fixture>/📸️snapshot/{{⬅️before,➡️after}}/🔣️component.json`
/// specification vectors into real [`{ty}Snapshot`] values, so the case adapter reads the committed
/// fixture instead of re-declaring it as a Rust literal beside it. Reaching `serde_json` from that
/// adapter is impossible — the generated test host links only this crate — which is why the bridge
/// belongs here.
{WAIVER}
pub fn decode_{slug}_snapshot_json(text: &str) -> Result<{ty}Snapshot, String> {{
    serde_json::from_str(text).map_err(|error| error.to_string())
}}

/// 📖️ Parses the committed `.dsl.semio` artifact into a [`{ty}Snapshot`]. Calls the `ArtifactDsl`
/// trait method directly rather than the `📝️text` facet's async wrapper, because a test host has no
/// async runtime to drive one.
{WAIVER}
pub fn decode_{slug}_dsl(text: &str) -> Result<{ty}Snapshot, String> {{
    <{ty}Snapshot as store::ArtifactDsl>::parse_dsl(text).map_err(|error| format!("{{error:?}}"))
}}

/// 🖨️ Prints a [`{ty}Snapshot`] back to its canonical `.dsl.semio` body. Canonical is the operative
/// word: the committed example assets ARE this function's own output, which is why the identity
/// scenario asserts byte-exactness rather than the no-byte-pass-through inequality.
{WAIVER}
pub fn encode_{slug}_dsl(snapshot: &{ty}Snapshot) -> String {{
    store::ArtifactDsl::print_dsl(snapshot)
}}

/// 📦️ Decodes a [`{ty}Snapshot`] from the binary `.pack.semio` envelope — an independently written
/// codec from the DSL grammar above, which is what makes their agreement evidence that the document
/// was parsed rather than copied.
{WAIVER}
pub fn decode_{slug}_pack(bytes: &[u8]) -> Result<{ty}Snapshot, String> {{
    <{ty}Snapshot as store::ArtifactPack>::decode_pack(bytes).map_err(|error| format!("{{error:?}}"))
}}

/// 📦️ Encodes a [`{ty}Snapshot`] to its binary `.pack.semio` envelope.
{WAIVER}
pub fn encode_{slug}_pack(snapshot: &{ty}Snapshot) -> Vec<u8> {{
    store::ArtifactPack::encode_pack(snapshot)
}}
//#endregion 🌉️ExternalCodecBridge
'''
    open(snap_file, "w", encoding="utf-8").write(snap.rstrip("\n") + "\n" + snap_bridges)
    print(f"patched {art}: {len(kinds)} kinds")
