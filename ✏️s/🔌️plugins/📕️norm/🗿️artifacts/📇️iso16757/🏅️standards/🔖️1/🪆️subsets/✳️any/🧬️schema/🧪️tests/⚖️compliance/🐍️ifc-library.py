#!/usr/bin/env python3
"""Validate the generated catalogue library against IFC4 and its language-neutral fixture."""
import json
from pathlib import Path
import re
import sys

import ifcopenshell
from ifcopenshell import validate
from ifcopenshell.guid import expand


def verify(source: Path, fixture: dict) -> None:
    """Read the emitted IFC4 file independently and compare its data to the original catalogue."""
    document = ifcopenshell.open(str(source))
    assert document.schema == "IFC4"
    logger = validate.json_logger()
    validate.validate(str(source), logger, express_rules=True)
    assert not logger.statements, logger.statements
    library, = document.by_type("IfcProjectLibrary")
    assert (library.Name, library.Description) == (fixture["catalogueName"], fixture["catalogueId"])
    dictionary, = document.by_type("IfcLibraryInformation")
    assert (dictionary.Name, dictionary.Version) == (fixture["dictionaryId"], fixture["dictionaryVersion"])
    association, = document.by_type("IfcRelAssociatesLibrary")
    assert association.RelatingLibrary == dictionary
    assert list(association.RelatedObjects) == [library]
    products = document.by_type("IfcTypeProduct")
    assert {product.Tag: product.Name for product in products} == {
        row["id"]: row["name"] for row in fixture["products"]
    }
    group, = document.by_type("IfcGroup")
    assert (group.Name, group.Description) == (fixture["className"], fixture["classId"])
    membership, = document.by_type("IfcRelAssignsToGroup")
    assert membership.RelatingGroup == group
    assert set(membership.RelatedObjects) == set(products)
    declaration, = document.by_type("IfcRelDeclares")
    assert declaration.RelatingContext == library
    assert set(declaration.RelatedDefinitions) == {group, *products}
    identifiers = [obj.GlobalId for obj in document.by_type("IfcRoot")]
    assert len(set(identifiers)) == len(identifiers)
    for identifier in identifiers:
        assert re.fullmatch(r"[0-3][0-9A-Za-z_$]{21}", identifier)
        assert expand(identifier)[12] == "8"


if __name__ == "__main__":
    source, fixture_path = map(Path, sys.argv[1:])
    verify(source, json.loads(fixture_path.read_text(encoding="utf-8")))
    print("IFC4 schema, EXPRESS rules, references, IDs and text match the source fixture")
