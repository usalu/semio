# Drawing Identity Current Design And Caller Frontier

Current implementation remains unfinished. The schema helper and constructor frontier is recorded by source references rather than source copies.

The existing schema root owns DefaultHasher, hash-to-hex formatting, name-derived layer/default constructors and recursive clone identity hashing. Seven distinctive direct free-function consumers include add-layer, canvas-pointer-down, edit-selection, SVG import and tests; 414 constructor/default/clone call expressions make the actual frontier larger.

The chosen schema contract separates authored identity keys from commitment facts. DrawingIdentity is a first-party owned key fact. Pure constructors require it explicitly and never derive it from names. The empty/default document carries an explicitly authored initial-layer identity. DrawingIdentityCommitment owns a typed kind and 32 digest octets. Binary IO defines counted unambiguous versioned SHA-256 preimages and controlled hashing; text IO defines lowercase prefixed spelling. Neither operation is callable indirectly from semantic constructors.

Clone and DuplicateLayer must carry a complete source-to-target identity assignment set. Semantic validation checks source census, nonempty distinct targets, collisions and Boolean references; inverse uses admitted target facts. Editor creation and SVG import call the concrete IO owner with controls.

The proof plan preserves the original Draw laws, adds neutral preimage/digest/spelling and malformed input vectors, independently computes them with Node crypto, and imports production leaf sources in an isolated native harness. Whole Draw native remains held at the previously observed shared Kernel floor. No identity completion claim is made.
