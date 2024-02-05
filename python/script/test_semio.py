from typing import List
from uuid import uuid4
from semio import (
    Transformation,
    Formation,
    Host,
    Parameter,
    Piece,
    Quality,
    Type,
    Kit,
    TypeId,
    Attraction,
)

# TODO: Refactor so scripts can be defined like this
# class Orientation(Enum):
#     NORTH = "north"
#     SOUTH = "south"
#     EAST = "east"
#     WEST = "west"
#
# @transformation("tower placement", "Place a tower on a base", "🏭")
# def tower_placement(formation: Formation, id: str, storeys: int, orientation: Orientation) -> Formation:
#    ...


class TowerPlacement(Transformation):
    name = "tower placement"
    explanation = "Place a tower on a base"
    icon = "🏭"

    def transform(formation: Formation, parameters: List[Parameter]) -> Formation:
        # Parameters
        id = [p for p in parameters if p.name == "id"]
        if len(id) < 1:
            raise ValueError("No id in parameters")
        elif len(id) > 1:
            raise ValueError("More than one id in parameters")
        id = id[0]
        storeys = [p for p in parameters if p.name == "storeys"]
        if len(storeys) < 1:
            raise ValueError("No storeys in parameters")
        elif len(storeys) > 1:
            raise ValueError("More than one storeys in parameters")
        storeys = storeys[0]
        orientation = [p for p in parameters if p.name == "orientation"]
        if len(orientation) < 1:
            raise ValueError("No orientation in parameters")
        elif len(orientation) > 1:
            raise ValueError("More than one orientation in parameters")
        orientation = orientation[0]
        # LHS
        base = [p for p in formation.pieces if p.name == "base"]
        if len(base) == 0:
            raise ValueError("No base in formation")
        base = base[0]
        # RHS
        shaft = Piece(
            id="shaft" + str(uuid4()),
            type=TypeId(
                "shaft",
                qualities=[
                    Quality(name="id", value=id),
                    Quality(name="storeys", value=storeys),
                    Quality(name="orientation", value=orientation),
                ],
            ),
        )
        # TODO: Add capital piece
        formation.pieces.append(shaft)
        # TODO: Add base shaft attraction
        # TODO: Add shaft head attraction
        return formation


metabolismHost = Host(transformations=[TowerPlacement]).run()
