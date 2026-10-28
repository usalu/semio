# Strict Unit Role Refusal And Derived Schema Oracle

Read-only released catalog review and authored unit-role-refusal-vectors.json; no production edits, install or runtime dependency. Executed unit-role-schema-oracle.py with existing IfcOpenShell0.9.0.post1 validator express_rules=True. Six independently constructed ADD2 unit populations matched expected validity: density and modulus valid with zero diagnostics; four invalid populations refused. Generated/unit-traversal/schema-oracle.json and physical source graphs retain exact subject IDs/diagnostics. This supplements earlier calculate_unit_scale four named-chain comparisons; it does not turn that helper into a derived-unit evaluator.

## Exact released roles

IFC4X3_ADD2.exp11950 UnitAssignment.Units is SET[1:?]OF IfcUnit and WR01 calls IfcCorrectUnitAssignment. It is not valid to borrow arbitrary entity.args[0] as assignment. Duplicate named nonUSERDEFINED physical types are rejected by released WR01 (observed duplicate metre/mm assignment refusal).

NamedUnit8105 has Dimensions and UnitType with IfcCorrectDimensions WR1. DerivedUnit5689 has Elements SET[1:?]OF DerivedUnitElement, DerivedUnitEnum, optional UserDefinedType/Name and derived Dimensions. WR1 forbids a sole element with exponent1; WR2 requires UserDefinedType for USERDEFINED. DerivedUnitElement5703.Unit is **IfcNamedUnit**, not IfcUnit and not another DerivedUnit; Exponent is INTEGER. Derived dimensions may contain conversion-based named elements, but arbitrary nested derived children violate the declared role even if arithmetic is evaluable.

ConversionBasedUnit5342 inherits NamedUnit and ConversionFactor is IfcMeasureWithUnit. MeasureWithUnit7950.ValueComponent is broad IfcValue SELECT and UnitComponent is IfcUnit; schema alone does not constrain its wrapper to a matching physical measure. Current runtime matching measure name/dimensions/finitepositive factor is an explicit supported physical-projection admission rule, not a newly invented EXPRESS declaration. ConversionBasedUnitWithOffset has extra affine offset and must stay unsupported by multiplicative scale evaluator.

## Actual independent schema results

Derived density kg/mm³ MASSDENSITYUNIT and modulus kg/m/s² MODULUSOFELASTICITYUNIT both validate with zero EXPRESS diagnostics. This proves their object/role construction is valid, not numerical derived conversion by external unit helper. One-element exponent1 fails WR1; USERDEFINED without UserDefinedType fails WR2; metre+mm project assignment fails WR01; named LENGTHUNIT with dimensionless exponents fails NamedUnit.WR1. Full diagnostics are generated evidence, not abbreviated success badges.

## Refusal policy split

Declaration failures: wrong assignment owner/member; derived child Unit not NamedUnit; exponent not integral; missing required refs. Released rule failures: listed WR1/WR2/WR01/dimensions. Source-resolution failures: duplicate/missing IDs, cycles and depth exhaustion. Supported projection refusals: unrelated conversion measure wrapper, mismatched expected physical dimensions, nonfinite/nonpositive factor, offset conversion and runtime exponent magnitude bound. INTEGER exponents have no schema magnitude64 cap; keep such implementation limit explicitly resource/support policy. A valid USERDEFINED unit can still be unsupported, and successful arithmetic does not prove valid IFC.

Do not widen FEM SI-only endpoint behavior when adding common generator support: FEM still intentionally demands project/metred physical scales1 and explicit typed SI engineering fields. BIM per-property length-dimension checks remain distinct from project geometry scale. Cancellation/original callback authority must be tested on project/assignment/conversion/derived-element/placement traversals separately, using the earlier cooperative plan.
