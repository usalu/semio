#!/usr/bin/env python3
"""Fix remaining 50 violations by inserting definition summaries and wrapping imports."""
FILE = "/workspaces/semio/semio/js/semio.ts"

with open(FILE, "r") as f:
    lines = f.readlines()

fixes = {
    27: ("// #region 🔖Imports\n// External dependency imports MUST be declared here.\n", None),
    49: (None, "// MUST merge CSS class names using Tailwind merge.\n// Performs the cn operation.\n"),
    185: (None, "// Identifier type for Attribute entities.\n"),
    600: (None, "// Zod schema for Coord validation.\n"),
    656: (None, "// Zod schema for Vec validation.\n"),
    712: (None, "// Zod schema for Point validation.\n"),
    778: (None, "// Zod schema for Vector validation.\n"),
    844: (None, "// Zod schema for Plane validation.\n"),
    981: (None, "// Zod schema for Camera validation.\n"),
    1047: (None, "// Zod schema for Location validation.\n"),
    1116: (None, "// Zod schema for Author validation.\n"),
    1185: (None, "// Zod schema for File validation.\n"),
    1283: (None, "// Zod schema for Folder validation.\n"),
    1377: (None, "// Zod schema for Benchmark validation.\n"),
    1528: (None, "// Zod schema for Quality validation.\n"),
    1673: (None, "// Zod schema for Port validation.\n"),
    1851: (None, "// Zod schema for Prop validation.\n"),
    1988: (None, "// Zod schema for Tag validation.\n"),
    2161: (None, "// Zod schema for Concept validation.\n"),
    2334: (None, "// Zod schema for Model validation.\n"),
    2569: (None, "// Zod schema for Connector validation.\n"),
    2718: (None, "// Zod schema for Type validation.\n"),
    2897: (None, "// Zod schema for Layer validation.\n"),
    2984: (None, "// Zod schema for Piece validation.\n"),
    3185: (None, "// Zod schema for Group validation.\n"),
    3268: (None, "// Zod schema for Side validation.\n"),
    3341: (None, "// Zod schema for Connection validation.\n"),
    3500: (None, "// Zod schema for Stat validation.\n"),
    3578: (None, "// Zod schema for Design validation.\n"),
    4631: (None, "// Zod schema for Kit validation.\n"),
    5126: (None, "// MUST return the requested value.\n// Retrieves the PrimitiveDesign value.\n"),
    5192: (None, "// MUST return the requested value.\n// Retrieves the PrimitiveType value.\n"),
    5523: (None, "// Interface defining FileTreeNode structure.\n"),
    5642: (None, "// Interface defining KitImportResult structure.\n"),
    7874: (None, "// Enumeration of EntityKind values.\n"),
    7912: (None, "// Interface defining ValidationContext structure.\n"),
    7966: (None, "// MUST produce a Fix that regenerates the GUID.\n// Performs the semioMakeFix operation.\n"),
    8012: (None, "// MUST detect and report constraint violations.\n// Constraint validating GuidUniqueness rules.\n"),
    8055: (None, "// MUST detect and report constraint violations.\n// Constraint validating TypeNameUniqueness rules.\n"),
    8099: (None, "// MUST detect and report constraint violations.\n// Constraint validating DesignNameUniqueness rules.\n"),
    8143: (None, "// MUST detect and report constraint violations.\n// Constraint validating PieceNameUniqueness rules.\n"),
    8185: (None, "// MUST detect and report constraint violations.\n// Constraint validating QualityNameUniqueness rules.\n"),
    8221: (None, "// MUST detect and report constraint violations.\n// Constraint validating PortNameUniqueness rules.\n"),
    8257: (None, "// MUST detect and report constraint violations.\n// Constraint validating FileNameUniqueness rules.\n"),
    8293: (None, "// MUST detect and report constraint violations.\n// Constraint validating FolderNameUniqueness rules.\n"),
    8337: (None, "// MUST detect and report constraint violations.\n// Constraint validating ConnectorNameUniqueness rules.\n"),
    8379: (None, "// MUST detect and report constraint violations.\n// Constraint validating ModelNameUniqueness rules.\n"),
    8421: (None, "// MUST detect and report constraint violations.\n// Constraint validating LayerPathUniqueness rules.\n"),
    8462: (None, "// MUST detect and report constraint violations.\n// Constraint validating DesignPieceSameFamily rules.\n"),
    8536: (None, "// Interface defining SerializableValidationFix structure.\n"),
}

import_end_line = None
for i, line in enumerate(lines):
    ln = i + 1
    if ln >= 27 and line.strip().startswith('import '):
        import_end_line = ln
    elif import_end_line and ln > 27 and not line.strip().startswith('import ') and line.strip() != '':
        break

result = []
for i, line in enumerate(lines):
    ln = i + 1
    if ln in fixes:
        prefix, before_def = fixes[ln]
        if prefix:
            result.append(prefix)
        if before_def:
            result.append(before_def)
    if ln == import_end_line:
        result.append(line)
        result.append("\n")
        result.append("// #endregion 🔖Imports\n")
        continue
    result.append(line)

with open(FILE, "w") as f:
    f.writelines(result)

print(f"Done. Lines: {len(lines)} -> {len(result)}")
