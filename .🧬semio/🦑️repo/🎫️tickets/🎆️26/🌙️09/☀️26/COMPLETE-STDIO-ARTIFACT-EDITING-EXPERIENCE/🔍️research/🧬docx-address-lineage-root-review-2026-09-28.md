# DOCX Address Lineage Review

Root reviewed the in-flight canonical XML address implementation after the independent DOCX audit. This is a source review, not a native result.

An address formed only from part/path/name and target-subtree hash still accepts structural displacement when an identical XML run is inserted before the addressed run. The semantic execution owner has added ancestor structure to the revision and is adding an explicit identical-run regression. This is necessary to prevent edits from silently targeting a newly occupying identical node.

The first lineage implementation hashes each ancestor and all of its immediate children. Its inverse revision path substitutes the replacement only for the final subtree hash, while the last ancestor’s child loop still hashes the old target’s shallow attributes and child count. A replacement that changes that shallow shape can therefore prepare an inverse that immediately rejects its own post-state. Root assigned the owner to substitute the new target in that loop and test no-text-node insertion or a changed attribute count.

The ancestor child loops also traverse unbounded sibling counts and attribute/text lengths. The retained preparation owner has been notified that depth-limited addressing alone does not establish bounded work; the retained route must account for and admit or incrementally traverse the actual address footprint. No boundedness or undo claim is established by the address helper’s existence.

## Empty Namespace Address Review

The root review found that `expanded_name` rejected an unprefixed element when no default namespace was bound. Such elements are valid and have no namespace name; canonical custom XML parts must therefore remain addressable. The text execution owner was assigned an empty-namespace case while retaining refusal for an unbound explicit prefix. This interpretation was checked against the primary [W3C Namespaces in XML recommendation, sections2.1 and6.2](https://www.w3.org/TR/REC-xml-names/#defaulting). Source repair and native proof remain the owner’s follow-up; this review does not report a passing law.
