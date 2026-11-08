@capability-structural-scene-schema
@oracle-serde-json-canonical
@comparison-canonical-json-v1
Feature: Structural scene schema parity
  The Rust declarations are the source; the proto, GraphQL and TypeScript twins must declare the same types
  and the canonical JSON of the golden scene must be byte-stable and parseable by an independent JSON library.

  Further specification assertions, asserted by the Rust unit tests of the crate and not by the platform adapters:
  - Every Rust scene type is declared in every twin: Given the structural scene schema modules When the declared struct and enum names are collected Then each name is declared in the proto, GraphQL and TypeScript twins And the diagram2d and handles part files are imported by each twin
  - Golden <fixture> is byte-identical to the canonical writer output: Given the committed golden "<fixture>" When the reference scene is written as canonical JSON Then the bytes equal the golden And an independent JSON parser re-serialises the golden to the same bytes Examples (1 rows: scene-json)
  - Section <profile> area equals the closed form: Given a parametric section "<profile>" with parameters "<params>" When its canonical outline area is computed Then it equals <area> within 1e-12 Examples (3 rows: rect-h-0-5-b-0-3-0-15, rhs-h-0-2-b-0-1-t-0-01-0-0056, ishape-h-0-24-b-0-24-tw-0-01-tf-0-017-0-01022)
  - An unchanged report keeps the evaluation sequence and every fragment: Given a projected document and its compliance report When the same document and report are projected again Then no fragment is rebuilt and the canonical scenes are identical
  - A drag preview rebuilds the edited entity and its dependents only: Given four members with one load each When the length of the third member changes and only that entity is marked changed Then the delta touches that member and its load and no other member
  - A moved utilisation repaints only the entity whose checks changed: Given a projected document When the report changes the utilisation of one member Then the delta names that member and no other entity
  - Quantity kind <kind> is declared in every twin with its SI unit: Given the quantity-kind fixture row "<kind>" When the TypeScript, GraphQL and proto twins and the units table are read by the Python oracle Then the wire name, the twin names and the factor-one SI unit of the family "<family>" are found Examples (4 rows: velocity-m-s, ventilation-rate-m3-s, heat-transfer-coefficient-w-m2-k, power-w)
