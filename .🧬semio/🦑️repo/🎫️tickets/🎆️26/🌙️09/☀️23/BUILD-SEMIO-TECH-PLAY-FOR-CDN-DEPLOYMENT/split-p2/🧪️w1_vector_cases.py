#!/usr/bin/env python3
"""🧪️ Restores the 136 canonical implementation cases (`🧬️mutations/<leaf>/🧪️tests/<case>/🦀️.rs`, the half the taxonomy's
mutation case-pair law pairs with each fixture bundle) as thin cases over the runtime law library
`🧬️mutations/🧪️tests/🔬️vectors/🦀️.rs`, and mounts them in integration test targets of the mutation group package that
owns their leaf — at most 30 cases per target so every unit stays inside the 32-file ceiling (lane P2).

Each case keeps its original documentation (scenario id, narrative) from the committed generated test it replaces, and
keeps the exact test set that test had: the status law set, plus the inverse-sum law where it had one.

usage: 🧪️w1_vector_cases.py
"""
import os, re, subprocess
REPO = os.path.abspath(os.path.join(os.path.dirname(os.path.abspath(__file__)), *[".."] * 8))
A_REL = "✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any"
A = os.path.join(REPO, A_REL)
M = os.path.join(A, "🧬️schema/🧬️mutations")
FIX = os.path.join(A, "🧪️tests/🧫️fixtures/🧬️mutations")
TARGETS = {
    "🧵️results": {"vectors": ["replace-sparse", "replace-dense", "replace-mesh-result", "replace-trajectory", "replace-tracks", "replace-geo-products", "replace-qc", "commit-reconstruction"]},
    "⚙️params": {"vectors": ["update-ingest-params", "update-feature-params", "update-match-params", "update-sfm-params", "update-dense-params", "update-mesh-params", "update-motion", "update-geo-params"]},
    "🎞️media": {"streams": ["create-stream", "delete-stream", "change-stream-sync", "add-stream-frame", "remove-stream-frame", "replace-stream"], "assets": ["create-asset", "delete-asset", "append-content", "remove-content"]},
    "🎯️calibration": {"cameras": ["create-camera", "update-camera", "delete-camera", "create-rig", "delete-rig-extrinsic", "update-rig-extrinsic"], "gcps": ["create-gcp", "delete-gcp", "add-gcp-observation", "remove-gcp-observation"]},
}
def slug(name):
    return re.sub(r"^[^a-z0-9]+", "", name)
def ident(text):
    return re.sub(r"[^a-z0-9]+", "_", text.lower()).strip("_")
leaf_dirs = {slug(d): d for d in os.listdir(M) if os.path.isfile(os.path.join(M, d, "🔣️.json"))}
HEAD_ROOT = subprocess.run(["git", "show", "HEAD:✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🦀️.rs"], cwd=REPO, capture_output=True, text=True, check=True).stdout
ORIGINAL_NAME = {(m.group(1), m.group(2)): m.group(3) for m in re.finditer(r'#\[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/([^/"]+)/🧪️tests/([^/"]+)/🦀️\.rs"\]\n\s*mod (tests_[a-z0-9_]+);', HEAD_ROOT)}
written = 0
targets_out = []
for group, targets in TARGETS.items():
    package_dir = os.path.join(M, group, "📦️packages", "🦀️rust")
    for target, leaves in targets.items():
        mounts = []
        for leaf_slug in leaves:
            leaf = leaf_dirs[leaf_slug]
            for case in sorted(os.listdir(os.path.join(FIX, leaf))):
                original = subprocess.run(["git", "show", f"HEAD:{A_REL}/🧬️schema/🧬️mutations/{leaf}/🧪️tests/{case}/🦀️.rs"], cwd=REPO, capture_output=True, text=True, check=True).stdout
                doc = original[:original.index("\n\n", original.index("🏞️")) + 1] if "🏞️" in original else original.split("\n\nuse ")[0] + "\n"
                tests = [f"/// ⚖️ The vector holds the law set its declared outcome selects, its inverse restores the before-document and its committed JSON is canonical.\n#[test]\nfn holds_its_laws() {{\n    crate::vectors::assert_case(\"{leaf}\", \"{case}\");\n}}\n"]
                if "inverse_diffs_sum_to_the_negative_diff" in original:
                    tests.append(f"/// ➕️ The inverse rows of this applied vector sum to the negative of its diff.\n#[semio_framework_async_macros::async_test]\nasync fn inverse_diffs_sum_to_the_negative_diff() {{\n    crate::vectors::assert_inverse_sum(\"{leaf}\", \"{case}\").await;\n}}\n")
                if "an_applied_commit_inverts_to_the_negative_diff" in original:
                    tests.append(f"/// ⚖️ An APPLIED commit over this scenario's base (every result lane cleared) inverts to the negative diff: the inverse rows sum to it and replaying them restores the base.\n#[semio_framework_async_macros::async_test]\nasync fn an_applied_commit_inverts_to_the_negative_diff() {{\n    let commit = semio_s_artifact_remodel_remodeling_mutations::standards::v1::subsets::any::schema::mutations::RemodelingMutation::from(semio_s_artifact_remodel_remodeling_mut_results::standards::v1::subsets::any::schema::mutations::commit_reconstruction::CommitReconstruction {{ sparse: None, trajectory: None, mesh: None, geo: None, qc: None, assets: Vec::new() }});\n    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&commit, &crate::vectors::Vector::load(\"{leaf}\", \"{case}\").before()).await;\n}}\n")
                doc = doc.replace("Source of truth is the committed JSON quintet beside this file", "Source of truth is the committed JSON quintet of the fixture mirror, read at test runtime")
                path = os.path.join(M, leaf, "🧪️tests", case, "🦀️.rs")
                os.makedirs(os.path.dirname(path), exist_ok=True)
                open(path, "w", encoding="utf-8").write(doc.rstrip("\n") + "\n\n" + "\n".join(tests))
                written += 1
                mounts.append((f"{ident(slug(leaf))}_{ORIGINAL_NAME[(leaf, case)]}", os.path.relpath(path, os.path.join(package_dir, "🧪️tests", f"🔬️{target}"))))
        assert len(mounts) <= 30, (group, target, len(mounts))
        root = os.path.join(package_dir, "🧪️tests", f"🔬️{target}", "🦀️.rs")
        os.makedirs(os.path.dirname(root), exist_ok=True)
        law = os.path.relpath(os.path.join(M, "🧪️tests", "🔬️vectors", "🦀️.rs"), os.path.dirname(root))
        body = (f"//! 🧪️ The committed {group[2:] if group[0] != '⚙' else group[2:]} mutation vectors `{target}`: one implementation case per fixture bundle, each checked against its vector by the shared law library.\n\n"
                "extern crate semio_framework_os_kernel as protocol;\n\n"
                f"#[path = \"{law}\"]\nmod vectors;\n\n" + "".join(f"#[path = \"{rel}\"]\nmod {name};\n" for name, rel in mounts))
        open(root, "w", encoding="utf-8").write(body)
        targets_out.append((group, target, len(mounts)))
print("cases", written)
for row in targets_out:
    print(row)
