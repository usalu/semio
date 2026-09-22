#!/usr/bin/env python3
"""🩹️ Anchored, idempotent: the space INDEX opens with its hub SCOPE but without a hub document
socket (ticket 26/09/18, slice S12 §2.2)."""
import pathlib
ROOT = pathlib.Path("/Users/ueli/Documents/semio")
SH = ROOT / "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx"
text = SH.read_text()

ANCHOR = """      const scope: DocumentScope | undefined = hubBinding === undefined ? undefined : { spaceId: hubBinding.spaceId, documentId: ref.documentId };
"""
INSERT = """      // 📇️ The space INDEX is the shell's projection of a space's DIRECTORY, not an artifact the hub
      // stores: no hub data root in this repository has ever held a document descriptor for it, so
      // `POST /spaces/{id}/documents/index/open-plan` answers 404 (`NotFound` from
      // `get_document_descriptor`), `runDocumentOpeningAttemptV1`'s socket phase rejects before
      // `attach` ever runs, and with it the scoped directory stream AND the artifact-creation catalog
      // never open — which is why the artifact table renders its header and zero rows on every hub and
      // why `createArtifact`'s kind chooser is empty (measured 2026-09-22, ticket 26/09/18 S12 §2.2).
      // The hub BINDING stays: `scope`, the hub runtime key and the catalog/presence lanes are all
      // keyed on it. Only the worker's own document open drops it.
      const spaceIndexDocument = ref.documentId === S_SPACE_INDEX_DOCUMENT_ID;
      const workerBindings = spaceIndexDocument ? resolvedBindings.filter((binding) => binding.kind !== "hub") : resolvedBindings;
"""
OLD_REQUEST = """        bindings: resolvedBindings,
        watchExternal: true,"""
NEW_REQUEST = """        bindings: workerBindings,
        watchExternal: true,"""
OLD_EXPECTS = """      const expectsSocketActor = resolvedBindings.some((binding) => binding.kind === "hub");"""
NEW_EXPECTS = """      const expectsSocketActor = workerBindings.some((binding) => binding.kind === "hub");"""

changed = 0
if "const spaceIndexDocument =" not in text:
    assert text.count(ANCHOR) == 1, "scope anchor not unique"
    text = text.replace(ANCHOR, ANCHOR + INSERT)
    changed += 1
if NEW_REQUEST not in text:
    assert text.count(OLD_REQUEST) == 1, "request anchor not unique"
    text = text.replace(OLD_REQUEST, NEW_REQUEST)
    changed += 1
if NEW_EXPECTS not in text:
    assert text.count(OLD_EXPECTS) == 1, "expects anchor not unique"
    text = text.replace(OLD_EXPECTS, NEW_EXPECTS)
    changed += 1
SH.write_text(text)
print(f"edits applied: {changed}")
