# DOCX Namespace and Formatting Follow-Up

## Source Findings Requiring the Next Execution Cut

The canonical XML address helpers resolve element namespace URIs, but the semantic read projection in base `🚪️io/📥️import/🧩️deserializers/🦀️.rs` still matches literal `w:document`, `w:body`, `w:r`, `w:pPr`, and style attribute names. `project_document` calls these helpers directly. Thus valid alias/default-prefix documents can be addressed by the new primary run helper while the semantic viewer/projection rejects or omits them. Namespace resolution must be shared and distinguish attribute namespaces from element default namespaces.

In `🧬️schema/🧬️mutations/🧭️xml-address/🦀️.rs`, `paragraph_style_exists` uses the element expanded-name helper for attributes and tests only the local-name suffix. Unprefixed attributes are thereby assigned the element default namespace incorrectly, and a foreign-prefixed `styleId` or `type` can satisfy the lookup. Match exact WordprocessingML expanded attribute names under the actual style element namespace.

`run_with_formatting` and `paragraph_with_style` build value attributes using the parent element prefix. For a default-namespaced WordprocessingML parent, this creates unprefixed `val`, which has no namespace. A qualified attribute prefix must be reused or allocated with a matching declaration while preserving unrelated namespace bindings. Existing rPr/pPr elements may also bind a different prefix locally; generated child names must use the actual property-container scope.

Formatting flags are also reduced to property presence by `run_from_xml`: `w:b w:val="0"`, `w:i w:val="false"`, and `w:u w:val="none"` currently project as enabled. The new formatting mutation removes properties for false flags, which can leave formatting inherited from styles enabled. The next cut must explicitly define direct formatting versus computed formatting and preserve disabled/cleared distinctions in canonical XML, with natural controls showing the effective state.

## Required Evidence

Author neutral strict/transitional, aliased/default, local-prefix-shadowing, foreign-attribute, explicit-false, and style-inherited fixtures before implementation. Use an independent namespace-aware XML reader to validate produced expanded attribute names, exact inverse/save/reopen, and refusal of foreign style identities. Integrate these with the new table projection fixture and visible EN/DE format controls. No runtime result has been claimed for these source findings.

The namespace rule is supported by the W3C Namespaces in XML specification: a default namespace applies to unprefixed element names, not unprefixed attribute names. [W3C Namespaces in XML §6.2](https://www.w3.org/TR/REC-xml-names/#defaulting).
