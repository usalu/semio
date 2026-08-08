from pathlib import Path

# 1) pub fn lex
lex = Path("✏️s/🔌️plugins/🔱️trinity/🔤️lexer/🦀️component.rs")
t = lex.read_text()
if "\nfn lex(input: &str)" in t:
    lex.write_text(t.replace("\nfn lex(input: &str)", "\npub fn lex(input: &str)"))
    print("lex made public")
else:
    print("lex pattern missing", "pub fn lex" in t)

# 2) language-service OnceLock for manifest
ls = Path("✏️s/🔌️plugins/� missing", "pub fn lex" in t)

# 2) language-service OnceLock for manifest
ls = Path("✏️s/🔌️plugins/🔱️trinity/🗣️language-service/🦀️component.rs")
t = ls.read_text()
old = """    fn trinity_jack_manifest() -> math::graph::manifest::GraphManifest {
        math::graph::manifest::manifest_by_id("nakagin").expect("nakagin manifest").clone()
    }"""
new = """    fn trinity_jack_manifest() -> &'static math::graph::manifest::GraphManifest {
        use std::sync::OnceLock;
        static MANIFEST: OnceLock<math::graph::manifest::GraphManifest> = OnceLock::new();
        MANIFEST.get_or_init(|| math::graph::manifest::manifest_by_id("nakagin").expect("nakagin manifest").clone())
    }"""
if old in t:
    ls.write_text(t.replace(old, new))
    print("manifest OnceLock fixed")
else:
    print("manifest fn pattern missing")
    for i, line in enumerate(t.splitlines(), 1):
        if "trinity_jack_manifest" in line:
            print(f"{i}:{line}")

# 3) update obsolete JackDiff test
mut = Path("✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🧬️mutations/🦀️component.rs")
t = mut.read_text()
old_test = """    fn trinity_graph_diff_apply_uses_set_fixture_as_base_and_recomputes() {
        let base = mini_fixture();
        let mut replacement = base.clone();
        replacement.name = "swapped".into();
        let diff = JackDiff { set_fixture: Some(replacement), recompute_derived: true, ..Default::default() };
        let applied = diff.apply(&base);
        assert_eq!(applied.name, "swapped");
        assert!(applied.nodes.iter().any(|n| n.properties.contains_key("flatPosition")));
    }"""
new_test = """    fn trinity_graph_diff_apply_uses_artifact_replacement() {
        let base = mini_fixture();
        let mut replacement = base.clone();
        replacement.name = "swapped".into();
        let diff = crate::artifacts::jack::diff::diff_set_snapshot(&replacement);
        let applied = diff.apply(&base);
        assert_eq!(applied.name, "swapped");
    }"""
if old_test in t:
    mut.write_text(t.replace(old_test, new_test))
    print("diff test updated")
else:
    print("diff test pattern missing")
    idx = t.find("trinity_graph_diff_apply")
    print(repr(t[idx:idx+500]))

# 4) verify JackArtifact::from_snapshot
for art in ["🔌️jack", "♻️rewrite"]:
    p = Path(f"✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/{art}/🧬️schema/🦀️component.rs")
    t = p.read_text()
    print(art, "from_snapshot" in t)
    for i, line in enumerate(t.splitlines(), 1):
        if "fn from_" in line or ("struct " in line and ("Artifact" in line or "Snapshot" in line)):
            print(f"  {i}:{line.strip()[:120]}")
