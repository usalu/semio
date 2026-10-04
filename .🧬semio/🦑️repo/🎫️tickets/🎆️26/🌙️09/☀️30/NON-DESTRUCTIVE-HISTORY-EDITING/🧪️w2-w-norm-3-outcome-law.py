"""🧪️ W2-W-norm-3 follow-up: gives every `🔺️diff` leaf of W2-W-norm-3's scope that `verify mutation-outcome-law` reports
(rule 1, no frozen code) the real state-dependent detection its verb family requires — an `insert-*`/`add-*` refuses
an element whose native `id` the collection already holds (`Fatal mutation.duplicate-id`), `remove-anchor` refuses an
anchor id the document does not hold (`Error mutation.target-missing`) and `update-site` reports re-applying the
site it already has (`Warning mutation.no-op`).

`python3 🧪️w2-w-norm-3-outcome-law.py <breach list>` — the list holds one `🔺️diff/🦀️.rs` path per line.
"""
import re
import sys

INSERT = re.compile(r"(pub fn diff\(payload: &\w+, base: &\w+\) -> protocol::MutationOutcome<\w+> \{\n)(    let mut (\w+) = base\.(\w+)\.clone\(\);\n.*?\3\.insert\(\w+, payload\.(\w+)\.clone\(\)\);)", re.S)

REMOVE_ANCHOR = (
    "pub fn diff(payload: &RemoveAnchor, base: &En1992Snapshot) -> protocol::MutationOutcome<En1992Diff> {\n",
    "pub fn diff(payload: &RemoveAnchor, base: &En1992Snapshot) -> protocol::MutationOutcome<En1992Diff> {\n"
    "    if !base.anchors.iter().any(|anchor| anchor.id == payload.anchor_id) {\n"
    "        return protocol::MutationOutcome::error(\"mutation.target-missing\", format!(\"Anchor {} does not exist.\", payload.anchor_id), [payload.anchor_id.clone()]);\n"
    "    }\n",
)

UPDATE_SITE = (
    "pub fn diff(payload: &UpdateSite, _base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {\n",
    "pub fn diff(payload: &UpdateSite, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {\n"
    "    if base.site == payload.site {\n"
    "        return protocol::MutationOutcome::empty().warning(\"mutation.no-op\", \"The site already has these values.\");\n"
    "    }\n",
)


def guard(match):
    field, element = match.group(4), match.group(5)
    label = element.replace("_", " ").capitalize()
    check = (
        f"    if base.{field}.iter().any(|existing| existing.id == payload.{element}.id) {{\n"
        f"        return protocol::MutationOutcome::fatal(\"mutation.duplicate-id\", format!(\"{label} id {{}} already exists.\", payload.{element}.id), [payload.{element}.id.clone()]);\n"
        f"    }}\n"
    )
    return match.group(1) + check + match.group(2)


def main(listing):
    for path in (line.strip() for line in open(listing, encoding="utf-8") if line.strip()):
        source = open(path, encoding="utf-8").read()
        if "mutation." in source:
            print("already coded", path)
            continue
        if source.count(REMOVE_ANCHOR[0]) == 1:
            patched = source.replace(*REMOVE_ANCHOR)
        elif source.count(UPDATE_SITE[0]) == 1:
            patched = source.replace(*UPDATE_SITE)
        else:
            patched, count = INSERT.subn(guard, source)
            if count != 1:
                print("UNMATCHED", path)
                continue
        open(path, "w", encoding="utf-8").write(patched)
        print("patched", path.split("🧬️mutations/")[1].split("/🔺️diff")[0], "in", path.split("🗿️artifacts/")[1].split("/")[0])


if __name__ == "__main__":
    main(sys.argv[1])
