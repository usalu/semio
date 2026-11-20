# Retained BRep Explode

The registered brep.topology.explode operator accepts a compound and publishes the existing solids geometry list. It shares the bounded list assembler and alias publisher with Shells. Original member order and repeated members are preserved. A live member alias is reused, including its original native primitive provenance. When retention has removed a member alias while keeping the compound, the original member's persistent label produces the same handle and restores its live alias. Empty compounds from cold callers produce an empty list.

The registered native fixture builds two retained boxes, creates an authored compound with one member twice, prunes that member's standalone alias, and cancels the first staged restoration. It then repeats Explode and checks exact handles, labels, member order, duplicate identity, original provenance on the continuously retained member, and both box volumes. The linked fixture awaits the in-progress native Flow build.

The portable Three.js Explode oracle and Ajv Shells contract passed three tests and eleven assertions. The isolated Rust topology-list owner passed shell, duplicate solid, empty list, and staged-claim cancellation cases with fixture adapters. All nine changed Rust sources parsed and the scoped whitespace check passed.
