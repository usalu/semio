#!/usr/bin/env python3
"""R8 session 12 prepared patch set (lands in the window after W2's `--packages all` publish; touches the
`semio-framework-ui-contract` lib every UI guest links).

Root cause of the ui-contract retirement reds (24 laws) and ui-runtime `runtime_tree_retirement_preserves_occupied_
sources_and_closes_exact_payloads`: the typed retirement of a `UiFixedList` gated its emptied backing on the BYTE grant
(`release_empty_page(bytes)`) and reported the physical page bytes, while every other typed owner meters payload by bytes
and backing by items (`UiFixedBytes` frees its emptied buffer as one item, zero bytes) and the typed laws account logical
payload bytes. A grant narrower than one page therefore stalled forever, and the document ladder papered over it by raising
the grant to the node table's footprint (puzzle3d B52), which then broke `released_bytes <= grant`.

Fix: the list releases its emptied backing one page per turn as an item (`release_empty_page(usize::MAX)`, zero bytes);
the document ladder drops its raise. Measured in an isolated copy of the crate closure (scratch workspace, no shared
build-dir): ui-contract 197/197, ui-runtime 125/125, replication 292/292 (baseline 173/197 + 124/125).

Usage: python3 ui-retirement-item-metered.py [--apply]   (dry run by default: verifies every anchor exactly once)
"""
import sys

ROOT = "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧬️contract/♻️retirement/🌳️typed/"
apply = "--apply" in sys.argv

EDITS = {
    ROOT + "🧱️component/🦀️.rs": [
        (
            "impl<T: UiTypedRetire, const N: usize> UiTypedRetire for UiFixedList<T, N> {",
            "/// 📋️ Retires the items last-first, then the emptied backing one page per turn. The byte grant meters payload\n"
            "/// (text, bytes, values), never backing: a page release is one item of O(1) work, reported as an item with zero\n"
            "/// bytes exactly like [`UiFixedBytes`]'s emptied buffer, so no grant is too narrow to finish a list.\n"
            "impl<T: UiTypedRetire, const N: usize> UiTypedRetire for UiFixedList<T, N> {",
        ),
        (
            "            let released = self.release_empty_page(bytes)?;\n"
            "            return Ok(UiValueRetirementStep { complete: self.terminal_is_empty(), progressed: released.progressed, released_items: usize::from(released.progressed), released_bytes: released.released_allocation_bytes });",
            "            let released = self.release_empty_page(usize::MAX)?;\n"
            "            return Ok(UiValueRetirementStep { complete: self.terminal_is_empty(), progressed: released.progressed, released_items: usize::from(released.progressed), released_bytes: 0 });",
        ),
    ],
    ROOT + "📃️document/🦀️.rs": [
        ("        let grant = maximum_bytes.max(slot.nodes.entries.allocated_bytes());\n", ""),
        ("            0 => slot.retirement.advance(&mut slot.nodes.entries, 1, grant)?,", "            0 => slot.retirement.advance(&mut slot.nodes.entries, 1, maximum_bytes)?,"),
        ("            1 => slot.retirement.advance(&mut slot.surface, 1, grant)?,", "            1 => slot.retirement.advance(&mut slot.surface, 1, maximum_bytes)?,"),
    ],
}
DOC_START = "    /// 🧮️ Retires one unit of this document against the caller's grant, RAISED"
DOC_END = "    fn retire_exact(&mut self, handle: UiDocumentHandle, maximum_bytes: usize)"
DOC_NEW = (
    "    /// 🧮️ Retires one unit of this document against the caller's grant. Page releases are item-metered in the\n"
    "    /// typed cursor (`UiTypedRetire for UiFixedList`), so the reconciler's 4 096-byte copy grant retires a 6 416-byte\n"
    "    /// `UiNodeRecord` page without being raised (ticket 26/09/02/PUZZLE-3D-END-TO-END wave B52: 67 667 consecutive\n"
    "    /// `progressed: false` steps on one catalogue surface while the page was byte-gated).\n"
)

LAW = "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧬️contract/📋️list/🧪️tests/📋️list/🦀️.rs"
LAW_START = "#[test]\nfn retained_fixed_list_pages_typed_retirement_obeys_actual_backing_grant()"
LAW_END = "//#region 🧪️PagedStorageLaws"
LAW_NEW = open(__file__.replace(".py", "-law.rs.txt"), encoding="utf8").read()

ok = True
out = {}
law = open(LAW, encoding="utf8").read()
start, end = law.find(LAW_START), law.find(LAW_END)
print(f"{'OK ' if 0 <= start < end else 'BAD'} list law block {start}..{end}")
ok &= 0 <= start < end
out[LAW] = law[:start] + LAW_NEW + law[end:]
for path, edits in EDITS.items():
    text = open(path, encoding="utf8").read()
    for old, new in edits:
        count = text.count(old)
        print(f"{'OK ' if count == 1 else 'BAD'} {count}x {path.rsplit('/', 2)[-2]}: {old.strip()[:70]}")
        ok &= count == 1
        text = text.replace(old, new)
    if path.endswith("📃️document/🦀️.rs"):
        start, end = text.find(DOC_START), text.find(DOC_END)
        print(f"{'OK ' if 0 <= start < end else 'BAD'} doc block {start}..{end}")
        ok &= 0 <= start < end
        text = text[:start] + DOC_NEW + text[end:]
    out[path] = text
if not ok:
    sys.exit("dry run failed: an anchor moved; re-derive the hunk")
if apply:
    for path, text in out.items():
        open(path, "w", encoding="utf8").write(text)
    print("applied — next: CARGO_INCREMENTAL=0 cargo check -p semio-framework-ui-contract --all-targets (+ --target wasm32-wasip2 via the wasm mutex), then nextest ui-contract + ui-runtime")
else:
    print("dry run clean")
