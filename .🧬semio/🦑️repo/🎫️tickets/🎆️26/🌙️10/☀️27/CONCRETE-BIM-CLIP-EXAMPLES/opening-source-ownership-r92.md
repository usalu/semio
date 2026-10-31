# Opening Source Ownership R92

The native export correctly omitted direct opening containment, but the authored source still listed native IfcOpeningElement features in storey containment relationships. IFC4.3 requires opening ContainedInStructure to be absent: its mandatory void relationship identifies the contained host. See [buildingSMART definition](https://standards.buildingsmart.org/IFC/DEV/IFC4_3/HTML/lexical/IfcOpeningElement.html).

A new registered language-neutral negative ownership contract first failed on OP-S. Source correction removes61direct feature memberships across all storeys, preserving every physical product membership, each opening's utility storey metadata, GUID and exactly one original host void relationship. Main relation contracts now distinguish products from subtraction features and retain exact physical-product containment/role assertions. No geometry changed and the correct native Reference export protocol remains. Focused GREEN pending.
