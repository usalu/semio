from graphene.test import Client

from semio import schema

schema = open("../graphql/schema.graphql", "r").read()
createLocalKit = open("../graphql/createLocalKit.graphql", "r").read()
updateLocalKitMetdata = open("../graphql/updateLocalKitMetdata.graphql", "r").read()
deleteLocalKit = open("../graphql/deleteLocalKit.graphql", "r").read()
addTypeToLocalKit = open("../graphql/addTypeToLocalKit.graphql", "r").read()
removeTypeFromLocalKit = open("../graphql/removeTypeFromLocalKit.graphql", "r").read()
addFormationToLocalKit = open("../graphql/addFormationToLocalKit.graphql", "r").read()
removeFormationFromLocalKit = open(
    "../graphql/removeFormationFromLocalKit.graphql", "r"
).read()


def integration_test_graphql_kit_crud(tmp_path):
    client = Client(schema)
    name = "metabolism"
    explanation = "For metabolistic architecture."
    icon = "🫀"
    url = "https://github.com/usalu/semio/tree/main/examples/metabolism"
    base = {
        "name": "base",
        "explanation": "A base with a public entrance and two towers that are on top of it.",
        "icon": "🏫",
        "representations": [
            {"url": "base/geometry.3dm", "lod": "1to100", "tags": ["simple"]}
        ],
        "ports": [
            {
                "plane": {
                    "origin": {"x": 0, "y": 0, "z": 0},
                    "xAxis": {"x": 1, "y": 0, "z": 0},
                    "yAxis": {"x": 0, "y": 1, "z": 0},
                },
                "specifiers": [
                    {"context": "tower", "group": "left"},
                ],
            },
            {
                "plane": {
                    "origin": {"x": 0, "y": 0, "z": 0},
                    "xAxis": {"x": 1, "y": 0, "z": 0},
                    "yAxis": {"x": 0, "y": 1, "z": 0},
                },
                "specifiers": [
                    {"context": "tower", "group": "right"},
                ],
            },
        ],
        "qualities": [
            {"name": "storeys", "value": "2", "unit": None},
        ],
    }
    baseId = {
        "name": "base",
        "qualities": [
            {"name": "storeys", "value": "2", "unit": None},
        ],
    }
    shaft = {
        "name": "shaft",
        "explanation": "A cuboid building core shaft with 10 storeys and a floor height of 3 meters.",
        "icon": "🛗",
        "representations": [
            {"url": "shaft/geometry.3dm", "lod": "1to100", "tags": ["simple"]}
        ],
        "ports": [
            {
                "plane": {
                    "origin": {"x": 0, "y": 0, "z": 0},
                    "xAxis": {"x": 1, "y": 0, "z": 0},
                    "yAxis": {"x": 0, "y": 1, "z": 0},
                },
                "specifiers": [
                    {"context": "facade", "group": "north"},
                    {"context": "floor", "group": "1"},
                    {"context": "door", "group": "0"},
                ],
            }
        ],
        "qualities": [
            {"name": "floor height", "value": "3", "unit": "m"},
            {"name": "storeys", "value": "10", "unit": None},
        ],
    }
    shaftId = {
        "name": "shaft",
        "qualities": [
            {"name": "floor height", "value": "3", "unit": "m"},
            {"name": "storeys", "value": "10", "unit": None},
        ],
    }
    capsule = {
        "name": "capsule",
        "explanation": "A capsule with the door in the back and window in the front.",
        "icon": "📦",
        "representations": [
            {
                "url": "capsules/standard/geometry.3dm",
                "lod": "1to100",
                "tags": ["simple"],
            }
        ],
        "ports": [
            {
                "plane": {
                    "origin": {"x": 0, "y": 0, "z": 0},
                    "xAxis": {"x": 1, "y": 0, "z": 0},
                    "yAxis": {"x": 0, "y": 1, "z": 0},
                },
                "specifiers": [{"context": "doors", "group": "front"}],
            }
        ],
        "qualities": [
            {"name": "door", "value": "behind", "unit": None},
            {"name": "window", "value": "front", "unit": None},
        ],
    }
    capsuleId = {
        "name": "capsule",
        "qualities": [
            {"name": "door", "value": "behind", "unit": None},
            {"name": "window", "value": "front", "unit": None},
        ],
    }
    mirroredCapsule = {
        "name": "capsule",
        "explanation": "A mirrored capsule with the door in the back and window in the front.",
        "icon": "📦^🪞",
        "representations": [
            {
                "url": "capsules/mirrored/geometry.3dm",
                "lod": "1to100",
                "tags": ["simple"],
            }
        ],
        "ports": [
            {
                "plane": {
                    "origin": {"x": 0, "y": 0, "z": 0},
                    "xAxis": {"x": 1, "y": 0, "z": 0},
                    "yAxis": {"x": 0, "y": 1, "z": 0},
                },
                "specifiers": [{"context": "doors", "group": "front"}],
            }
        ],
        "qualities": [
            {"name": "door", "value": "behind", "unit": None},
            {"name": "window", "value": "front", "unit": None},
            {"name": "mirrored", "value": "true", "unit": None},
        ],
    }
    mirroredCapsuleId = {
        "name": "capsule",
        "qualities": [
            {"name": "door", "value": "behind", "unit": None},
            {"name": "window", "value": "front", "unit": None},
            {"name": "mirrored", "value": "true", "unit": None},
        ],
    }
    types = [shaft, capsule, mirroredCapsule]
    nakaginCapsuleTower = {
        "name": "nakagin capsule tower",
        "explanation": "The Nakagin Capsule Tower.",
        "icon": "🏯",
        "pieces": [
            {
                "transient": {"id": "s"},
                "type": {
                    "name": "shaft",
                    "qualities": [
                        {"name": "floor height", "value": "3", "unit": "m"},
                        {"name": "storeys", "value": "10"},
                    ],
                },
            },
            {
                "transient": {"id": "c1"},
                "type": {
                    "name": "capsule",
                    "qualities": [
                        {"name": "door", "value": "behind"},
                        {"name": "window", "value": "front"},
                    ],
                },
            },
            {
                "transient": {"id": "c2"},
                "type": {
                    "name": "capsule",
                    "qualities": [
                        {"name": "door", "value": "behind"},
                        {"name": "window", "value": "front"},
                        {"name": "mirrored", "value": "true"},
                    ],
                },
            },
        ],
        "attractions": [
            {
                "attracting": {
                    "piece": {
                        "transient": {"id": "s"},
                        "type": {
                            "port": {
                                "specifiers": [
                                    {"context": "facade", "group": "north"},
                                    {"context": "floor", "group": "1"},
                                    {"context": "door", "group": "0"},
                                ]
                            }
                        },
                    }
                },
                "attracted": {
                    "piece": {
                        "transient": {"id": "c1"},
                        "type": {
                            "port": {
                                "specifiers": [{"context": "doors", "group": "front"}]
                            }
                        },
                    }
                },
            }
        ],
        "qualities": [{"name": "storeys", "value": "12"}],
    }
    kit = {
        "name": name,
        "explanation": explanation,
        "icon": icon,
        "types": types,
        "formations": [nakaginCapsuleTower],
    }
    createResponse = client.execute(
        createLocalKit, variables={"directory": str(tmp_path), "kit": kit}
    )
    assert createResponse == {
        "data": {
            "createLocalKit": {
                "kit": kit,
                "error": None,
            },
            "errors": None,
        }
    }
    removeShaftResponse = client.execute(
        removeTypeFromLocalKit,
        variables={"directory": str(tmp_path), "type": shaftId},
    )
    assert removeShaftResponse == {
        "data": {
            "removeTypeFromLocalKit": {
                "kit": None,
                "error": {"code": "FORMATION_DEPENDS_ON_TYPE", "message": None},
            }
        }
    }
    addBaseResponse = client.execute(
        addTypeToLocalKit,
        variables={"directory": str(tmp_path), "type": base},
    )
    assert addBaseResponse == {
        "data": {
            "addTypeToLocalKit": {
                "type": base,
                "error": None,
            }
        }
    }
