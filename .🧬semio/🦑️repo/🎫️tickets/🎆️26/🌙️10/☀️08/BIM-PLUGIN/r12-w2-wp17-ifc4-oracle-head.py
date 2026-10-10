#!/usr/bin/env python3
"""🏗️ Third-party ORACLE (IfcOpenShell) for the IFC4 export of the BIM model.

IfcOpenShell 0.8.4.post1 has never seen this repository's writer. It opens the committed IFC4 (ADD2 TC1) files of the fixtures `🧩️ifc4/<case>/<case>.ifc`, and:

* parses each, requires the schema `IFC4`, no `IfcOwnerHistory`, and validates every attribute type and every EXPRESS WHERE rule (`ifcopenshell.validate`);
* counts the entities of every class the export writes (the IFC4 classes: window, door and roof types, triangulated face sets, the project library and its templates);
* reads the spatial containment of every product, tessellates every straight wall, level slab, column, beam and covering with its C++ kernel and requires the volume to equal the written base quantity;
* requires the occurrence and the type of every door to carry the operation of the door type of the snapshot (leaves and swing) and every window type the partitioning of its panes;
* reads the project library (property set templates with their simple property templates, enumerations and `key=value;` descriptions, classification systems chained by `ReferencedSource` in `Sort` order, the
  declarations and the associations), the `IfcRelDefinesByTemplate` of every property set named like a template, and the classifications attached to every element and type;
* reads the user property sets of every type object and element and requires them to equal the snapshot.

Standalone use (no test host needed):

    python 🐍️.py check <path to 🧫️fixtures> [case...]    # exit 1 on any disagreement
    python 🐍️.py write <path to 🧫️fixtures> [case...]    # rewrite the measured table from the committed file

@see ../../🔮️oracles/🔣️.json — the registration of the oracle this file answers for
@see ../../🚪️io/📤️export/🏗️ifc/🦀️.rs — the writer under test
"""

# region 🔖️Imports
import json
import math
import sys
from pathlib import Path

import ifcopenshell
import ifcopenshell.geom
import ifcopenshell.util.classification
import ifcopenshell.util.element
import ifcopenshell.util.shape
import ifcopenshell.validate

# endregion 🔖️Imports


# region 🔖️Measurement
PSETS = "psets"
