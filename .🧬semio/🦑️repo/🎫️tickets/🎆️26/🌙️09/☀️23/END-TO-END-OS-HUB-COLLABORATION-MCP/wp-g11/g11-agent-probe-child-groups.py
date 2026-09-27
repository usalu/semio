"""🧩️ G11 pending set (next guest window; the SDK is guest-linked and frozen during the rebuild): the declared-verb harness'
agent-lane probe counts the owned-child op groups the carrier set made the agent lane carry (`childGroups` in the preview
output), so a verb that writes only composed children is "writes the document" on the agent lane exactly as on the shell lane.
After it lands, re-run the flow law (`⚖️declared-verbs`) and shrink its pinned agent divergences to the lanes an agent
transaction still cannot carry (expected: setActiveExample (document load), evaluate (host-only job), focusSelection
(window camera)); measure first, pin what the law reports.
usage: python3 g11-agent-probe-child-groups.py [--apply]"""
import sys

SDK = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs"
OLD = """                    let (document, config, draft) = (count("documentOps"), count("configOps"), count("draftOps"));
                    let lanes = [("artifact", document), ("config", config), ("draft", draft)].into_iter().filter(|(_, ops)| *ops != 0).map(|(lane, _)| lane).collect::<std::collections::BTreeSet<_>>();"""
NEW = """                    let (document, config, draft, children) = (count("documentOps"), count("configOps"), count("draftOps"), count("childGroups"));
                    let lanes = [("artifact", document), ("config", config), ("draft", draft), ("child", children)].into_iter().filter(|(_, ops)| *ops != 0).map(|(lane, _)| lane).collect::<std::collections::BTreeSet<_>>();"""
OLD2 = """DeclaredVerbOutcome::Settled(DeclaredVerbEffect { lanes, document_changed: document != 0, document_replaced: false, config_changed: config != 0, user_path_written: false, host_effects, fingerprint })"""
NEW2 = """DeclaredVerbOutcome::Settled(DeclaredVerbEffect { lanes, document_changed: document != 0 || children != 0, document_replaced: false, config_changed: config != 0, user_path_written: false, host_effects, fingerprint })"""

text = open(SDK, encoding="utf-8").read()
counts = (text.count(OLD), text.count(OLD2))
print(f"anchors {counts} (expected (1, 1))")
if counts != (1, 1):
    sys.exit(1)
if "--apply" in sys.argv:
    open(SDK, "w", encoding="utf-8").write(text.replace(OLD, NEW).replace(OLD2, NEW2))
    print("applied; next: cargo check -p semio-framework-plugin --features artifact-app-testing --lib --tests, then the flow law")
else:
    print("dry run clean")
