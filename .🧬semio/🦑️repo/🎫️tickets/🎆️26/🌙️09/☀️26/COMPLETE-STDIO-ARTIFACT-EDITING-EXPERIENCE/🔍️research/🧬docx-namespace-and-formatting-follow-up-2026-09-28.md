# DOCX Namespace and Formatting Follow-Up

## Source Findings Requiring the Next Execution Cut

The canonical XML address helpers resolve element namespace URIs, but the semantic read projection in base `🚪️io/📥️import/🧩️deserializers/🦀️.rs` still matches literal `w:document`, `w:body`, `w:r`, `w:pPr`, and style attribute names. `project_document` calls these helpers directly. Thus valid alias/default-prefix documents can be addressed by the new primary run helper while the semantic viewer/projection rejects or omits them. Namespace resolution must be shared and distinguish attribute namespaces from element default namespaces.

In `🧬️schema/🧬️mutations/🧭️xml-address/🦀️.rs`, `paragraph_style_exists` uses the element expanded-name helper for attributes and tests only the local-name suffix. Unprefixed attributes are thereby assigned the element default namespace incorrectly, and a foreign-prefixed `styleId` or `type` can satisfy the lookup. Match exact WordprocessingML expanded attribute names under the actual style element namespace.

`run_with_formatting` and `paragraph_with_style` build value attributes using the parent element prefix. For a default-namespaced WordprocessingML parent, this creates unprefixed `val`, which has no namespace. A qualified attribute prefix must be reused or allocated with a matching declaration while preserving unrelated namespace bindings. Existing rPr/pPr elements may also bind a different prefix locally; generated child names must use the actual property-container scope.

Formatting flags are also reduced to property presence by `run_from_xml`: `w:b w:val="0"`, `w:i w:val="false"`, and `w:u w:val="none"` currently project as enabled. The new formatting mutation removes properties for false flags, which can leave formatting inherited from styles enabled. The next cut must explicitly define direct formatting versus computed formatting and preserve disabled/cleared distinctions in canonical XML, with natural controls showing the effective state.

## Required Evidence

Author neutral strict/transitional, aliased/default, local-prefix-shadowing, foreign-attribute, explicit-false, and style-inherited fixtures before implementation. Use an independent namespace-aware XML reader to validate produced expanded attribute names, exact inverse/save/reopen, and refusal of foreign style identities. Integrate these with the new table projection fixture and visible EN/DE format controls. No runtime result has been claimed for these source findings.

The namespace rule is supported by the W3C Namespaces in XML specification: a default namespace applies to unprefixed element names, not unprefixed attribute names. [W3C Namespaces in XML §6.2](https://www.w3.org/TR/REC-xml-names/#defaulting).

## Implemented Source Cut and Resumed Validation — 3 October 2026

The shared first-party `🚪️io/🏷️namespaces/🦀️.rs` now resolves element names and WordprocessingML attribute names with distinct default-namespace rules. Both semantic import projection and canonical mutation addressing use that scope implementation. Document/body/paragraph/run/table/style reads accept strict/transitional alias/default namespaces, and explicit false direct flags (`0`, `false`, `off`, `none`) no longer project as true. Foreign/unqualified style identities are ignored. DOCX sniffing follows the selected main part and WordprocessingML content type instead of a `word/` path heuristic.

Formatting mutations write explicit false values rather than delete properties, preserving unrelated attributes, comments and property children. Existing property-local prefixes are resolved before edits, and missing attribute prefixes are allocated without overriding existing bindings. Paragraph style edits preserve unrelated fields and refuse foreign style identities. Six neutral namespace/formatting cases plus independent Quick-XML native projection/edit/inverse/save laws were authored. Python Expat independently matched the six fixture expectations on28 September. This is fixture evidence; the current Rust laws have not yet passed.

DOCX native8 on3 October compiled current source but stopped at unrelated-to-namespace integration errors: two retirement trait results now require first-party ValueError, and one SQLite fixture still referenced removed Store JSON facade. Root corrected those exact seams; native9 is running. XML native8 similarly found two missing typed error-kind arguments and a stale SQLite test result type; root corrected these and launched XML9. No stale build lock or peer job was changed.

Remaining scope: computed style inheritance and clear-versus-disabled natural controls, proper page/layout/table presentation, user-facing formatting/style/structure actions, and actual bounded large-document preparation are not provided by the scoped namespace repair. These remain required for the full editor objective.

## October 3 Runtime Evidence and CDATA Follow-Up

Native current 12 ran the namespace projection and namespace-preserving formatting/style edit laws successfully, alongside the table primary projection and canonical pack laws. Its complete result was 140/142, so the suite was not green. The mixed Text/CDATA neutral fixture reproduced a separate text-loss bug: the primary run projection exposed AB from A<![CDATA[<文字>]]>B plus a second CDATA C. The repair now reads and replaces both contribution kinds while leaving comments and tabs intact; identical edits keep the original canonical nodes. Exact inverse and independent QuickXML output are covered by the same law. Current 14 is pending after current 13 encountered the concurrent Store ownership transition.
