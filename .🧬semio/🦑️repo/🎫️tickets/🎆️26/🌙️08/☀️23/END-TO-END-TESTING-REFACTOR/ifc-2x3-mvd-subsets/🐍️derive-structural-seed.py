"""🏗️ Derives the ✳️sav case's committed input from the real committed IFC2X3 export.

No real IFC2X3 StructuralAnalysisView document exists in this repository — grepping the FULL 21 MB
source (temp/wellness-center-sama.ifc) for IFCSTRUCTURAL* returns zero matches — so the structural
half of this fixture is seeded, not real. Everything else is the real 3464-entity export, byte for
byte: only the FILE_DESCRIPTION view-definition string is re-stamped and exactly three structural
entities are appended before the DATA section's ENDSEC.
"""

from pathlib import Path

ROOT = Path(__file__).resolve().parents[8]
FIXTURES = ROOT / "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🧫️fixtures"
SOURCE = FIXTURES / "🏗️wellness-center-sama-street-level.ifc"
TARGET = FIXTURES / "🏗️wellness-center-sama-structural-seed.ifc"

SEED = (
    "/* Seeded structural scaffolding (ticket 26/08/23/END-TO-END-TESTING-REFACTOR, 🏗️ifc 2x3 ✳️sav oracle):"
    " the three entities below are NOT real -- no real IFC2X3 StructuralAnalysisView document exists in this"
    " repository (the full 21 MB temp/wellness-center-sama.ifc contains zero IFCSTRUCTURAL* instances). They are"
    " appended so the Structural Analysis View vocabulary has a target; #9200003 relates the two REAL"
    " IFCWALLSTANDARDCASE instances #270549 and #523123 of the real model, and #41 is the real IFCOWNERHISTORY."
    " Every other entity in this file is the real EDM export, unmodified. */\n"
    "#9200001= IFCSTRUCTURALANALYSISMODEL('2SavAnalysisModelSeed001',#41,'Street level analysis model',$,$,.NOTDEFINED.,$,$,$);\n"
    "#9200002= IFCSTRUCTURALLOADGROUP('2SavLoadGroupSeed00000001',#41,'Self weight',$,$,.LOAD_GROUP.,.VARIABLE_Q.,.LIVE_LOAD_Q.,$,$);\n"
    "#9200003= IFCRELASSIGNSTOGROUP('2SavGroupAssignmentSeed01',#41,$,$,(#270549,#523123),$,#9200001);\n"
)

text = SOURCE.read_text(encoding="utf-8")
before = "FILE_DESCRIPTION(('ViewDefinition [CoordinationView_V2.0]'),'2;1');"
after = "FILE_DESCRIPTION(('ViewDefinition [StructuralAnalysisView]'),'2;1');"
assert text.count(before) == 1, "the real export's view-definition line was not found exactly once"
text = text.replace(before, after)

marker = "ENDSEC;\n\nEND-ISO-10303-21;"
assert text.count(marker) == 1, "the DATA section's terminator was not found exactly once"
text = text.replace(marker, SEED + marker)

TARGET.write_text(text, encoding="utf-8")
print(f"wrote {TARGET} ({TARGET.stat().st_size} bytes)")
