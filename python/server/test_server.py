from pytest import mark
from graphene.test import Client
from deepdiff import DeepDiff, Delta
from server import schema

createLocalKit = open("../../graphql/createLocalKit.graphql", "r").read()
updateLocalKitMetadata = open(
    "../../graphql/updateLocalKitMetadata.graphql", "r"
).read()
deleteLocalKit = open("../../graphql/deleteLocalKit.graphql", "r").read()
addTypeToLocalKit = open("../../graphql/addTypeToLocalKit.graphql", "r").read()
removeTypeFromLocalKit = open(
    "../../graphql/removeTypeFromLocalKit.graphql", "r"
).read()
addFormationToLocalKit = open(
    "../../graphql/addFormationToLocalKit.graphql", "r"
).read()
removeFormationFromLocalKit = open(
    "../../graphql/removeFormationFromLocalKit.graphql", "r"
).read()
formationToSceneFromLocalKit = open(
    "../../graphql/formationToSceneFromLocalKit.graphql", "r"
).read()


@mark.skip
def test_integration_graphql_local_kit_crud(tmp_path):
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
                    "origin": {"x": 0.0, "y": 0.0, "z": 0.0},
                    "xAxis": {"x": 1.0, "y": 0.0, "z": 0.0},
                    "yAxis": {"x": 0.0, "y": 1.0, "z": 0.0},
                },
                "specifiers": [
                    {"context": "tower", "group": "left"},
                ],
            },
            {
                "plane": {
                    "origin": {"x": 0.0, "y": 0.0, "z": 0.0},
                    "xAxis": {"x": 1.0, "y": 0.0, "z": 0.0},
                    "yAxis": {"x": 0.0, "y": 1.0, "z": 0.0},
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
                    "origin": {"x": 0.0, "y": 0.0, "z": 0.0},
                    "xAxis": {"x": 1.0, "y": 0.0, "z": 0.0},
                    "yAxis": {"x": 0.0, "y": 1.0, "z": 0.0},
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
                    "origin": {"x": 0.0, "y": 0.0, "z": 0.0},
                    "xAxis": {"x": 1.0, "y": 0.0, "z": 0.0},
                    "yAxis": {"x": 0.0, "y": 1.0, "z": 0.0},
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
                    "origin": {"x": 0.0, "y": 0.0, "z": 0.0},
                    "xAxis": {"x": 1.0, "y": 0.0, "z": 0.0},
                    "yAxis": {"x": 0.0, "y": 1.0, "z": 0.0},
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
                "id": "s",
                "type": {
                    "name": "shaft",
                    "qualities": [
                        {"name": "floor height", "value": "3", "unit": "m"},
                        {"name": "storeys", "value": "10", "unit": None},
                    ],
                },
            },
            {
                "id": "c1",
                "type": {
                    "name": "capsule",
                    "qualities": [
                        {"name": "door", "value": "behind", "unit": None},
                        {"name": "window", "value": "front", "unit": None},
                    ],
                },
            },
            {
                "id": "c2",
                "type": {
                    "name": "capsule",
                    "qualities": [
                        {"name": "door", "value": "behind", "unit": None},
                        {"name": "window", "value": "front", "unit": None},
                        {"name": "mirrored", "value": "true", "unit": None},
                    ],
                },
            },
        ],
        "attractions": [
            {
                "attracting": {
                    "piece": {
                        "id": "s",
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
                        "id": "c1",
                        "type": {
                            "port": {
                                "specifiers": [{"context": "doors", "group": "front"}]
                            }
                        },
                    }
                },
            }
        ],
        "qualities": [{"name": "storeys", "value": "12", "unit": None}],
    }
    kit = {
        "name": name,
        "explanation": explanation,
        "icon": icon,
        "url": url,
        "types": types,
        "formations": [nakaginCapsuleTower],
    }
    createResponse = client.execute(
        createLocalKit, variables={"directory": str(tmp_path), "kit": kit}
    )
    createResponseExpected = {
        "data": {
            "createLocalKit": {
                "kit": kit,
                "error": None,
            }
        }
    }
    createResponseDiff = DeepDiff(createResponse, createResponseExpected)
    assert not createResponseDiff, f"Response difference: {createResponseDiff}"
    removeShaftResponse = client.execute(
        removeTypeFromLocalKit,
        variables={"directory": str(tmp_path), "type": shaftId},
    )
    removeShaftResponseExpected = {
        "data": {
            "removeTypeFromLocalKit": {
                "error": {"code": "FORMATION_DEPENDS_ON_TYPE", "message": None},
            }
        }
    }
    removeShaftResponseDiff = DeepDiff(removeShaftResponse, removeShaftResponseExpected)
    assert (
        not removeShaftResponseDiff
    ), f"Response difference: {removeShaftResponseDiff}"
    addBaseResponse = client.execute(
        addTypeToLocalKit,
        variables={"directory": str(tmp_path), "type": base},
    )
    addBaseResponseExpected = {
        "data": {
            "addTypeToLocalKit": {
                "type": base,
                "error": None,
            }
        }
    }
    addBaseResponseDiff = DeepDiff(addBaseResponse, addBaseResponseExpected)
    assert not addBaseResponseDiff, f"Response difference: {addBaseResponseDiff}"


def test_integration_graphql_local_kit_formationToScene(tmp_path):
    #   ┌──────────┐ ┌──────────┐   │   ┌──────────┐ xxxxxxxxxxxx
    #   │          │ │          │   │   │          │ x          x
    # ┌─▼─┐        │ │        ┌─▼─┐ │ ┌─▼─┐        │ x        ┌───┐
    # │ a │       ┌┴─┴┐       │ b │ │ │ a │       ┌┴──┐       │ b │
    # └─▲─┘    ┌──►1ab◄──┐    └─▲─┘ │ └─▲─┘    ┌──►1ab│xxx    └─▲─┘
    #   │      │  └───┘  │      │   │   │      │  └───┘  x      │
    # ┌─┴─┐  ┌─┴─┐     ┌─┴─┐  ┌─┴─┐ │ ┌─┴─┐  ┌─┴─┐     ┌─x─┐  ┌─┴─┐
    # │1a1◄──┤1a │     │1b ├──►1b1│ │ │1a1◄──┤1a │     │1b ├──►1b1│
    # └───┘  └─▲─┘     └─▲─┘  └───┘ │ └───┘  └─▲─┘     └─▲─┘  └───┘
    #          │  ┌───┐  │          │          │  ┌───┐  │
    #          └──┤ 1 ├──┘          │          └──┤ 1 ├──┘
    #             └───┘             │             └───┘
    #             before            │             after
    client = Client(schema)
    type1 = {
        "name": "type1",
        "representations": [{"url": "url1"}],
        "ports": [
            {
                "plane": {
                    "origin": {"x": 0.0, "y": 0.0, "z": 0.0},
                    "xAxis": {"x": 1.0, "y": 0.0, "z": 0.0},
                    "yAxis": {"x": 0.0, "y": 1.0, "z": 0.0},
                },
                "specifiers": [
                    {"context": "context1", "group": "group1"},
                ],
            },
            {
                "plane": {
                    "origin": {"x": 0.0, "y": 0.0, "z": 0.0},
                    "xAxis": {"x": 1.0, "y": 0.0, "z": 0.0},
                    "yAxis": {"x": 0.0, "y": 1.0, "z": 0.0},
                },
                "specifiers": [
                    {"context": "context1", "group": "group2"},
                ],
            },
        ],
    }
    formation1 = {
        "name": "formation1",
        "explanation": "A formation with circular attractions.",
        "icon": "🔁",
        "pieces": [
            {
                "id": "1",
                "type": {
                    "name": "type1",
                },
            },
            {
                "id": "1a",
                "type": {
                    "name": "type1",
                },
            },
            {
                "id": "1b",
                "type": {
                    "name": "type1",
                },
            },
            {
                "id": "1ab",
                "type": {
                    "name": "type1",
                },
            },
            {
                "id": "1a1",
                "type": {
                    "name": "type1",
                },
            },
            {
                "id": "1b1",
                "type": {
                    "name": "type1",
                },
            },
            {
                "id": "a",
                "type": {
                    "name": "type1",
                },
            },
            {
                "id": "b",
                "type": {
                    "name": "type1",
                },
            },
        ],
        "attractions": [
            {
                "attracting": {
                    "piece": {
                        "id": "1",
                        "type": {
                            "port": {
                                "specifiers": [
                                    {"context": "context1", "group": "group1"},
                                ]
                            }
                        },
                    }
                },
                "attracted": {
                    "piece": {
                        "id": "1a",
                        "type": {
                            "port": {
                                "specifiers": [
                                    {"context": "context1", "group": "group2"},
                                ]
                            }
                        },
                    }
                },
            },
            {
                "attracting": {
                    "piece": {
                        "id": "1",
                        "type": {
                            "port": {
                                "specifiers": [
                                    {"context": "context1", "group": "group2"},
                                ]
                            }
                        },
                    }
                },
                "attracted": {
                    "piece": {
                        "id": "1b",
                        "type": {
                            "port": {
                                "specifiers": [
                                    {"context": "context1", "group": "group1"},
                                ]
                            }
                        },
                    }
                },
            },
            {
                "attracting": {
                    "piece": {
                        "id": "1a",
                        "type": {
                            "port": {
                                "specifiers": [
                                    {"context": "context1", "group": "group1"},
                                ]
                            }
                        },
                    }
                },
                "attracted": {
                    "piece": {
                        "id": "1a1",
                        "type": {
                            "port": {
                                "specifiers": [
                                    {"context": "context1", "group": "group2"},
                                ]
                            }
                        },
                    }
                },
            },
            {
                "attracting": {
                    "piece": {
                        "id": "1b",
                        "type": {
                            "port": {
                                "specifiers": [
                                    {"context": "context1", "group": "group2"},
                                ]
                            }
                        },
                    }
                },
                "attracted": {
                    "piece": {
                        "id": "1b1",
                        "type": {
                            "port": {
                                "specifiers": [
                                    {"context": "context1", "group": "group1"},
                                ]
                            }
                        },
                    }
                },
            },
            {
                "attracting": {
                    "piece": {
                        "id": "1a",
                        "type": {
                            "port": {
                                "specifiers": [
                                    {"context": "context1", "group": "group1"},
                                ]
                            }
                        },
                    }
                },
                "attracted": {
                    "piece": {
                        "id": "1ab",
                        "type": {
                            "port": {
                                "specifiers": [
                                    {"context": "context1", "group": "group1"},
                                ]
                            }
                        },
                    }
                },
            },
            {
                "attracting": {
                    "piece": {
                        "id": "1b",
                        "type": {
                            "port": {
                                "specifiers": [
                                    {"context": "context1", "group": "group2"},
                                ]
                            }
                        },
                    }
                },
                "attracted": {
                    "piece": {
                        "id": "1ab",
                        "type": {
                            "port": {
                                "specifiers": [
                                    {"context": "context1", "group": "group2"},
                                ]
                            }
                        },
                    }
                },
            },
            {
                "attracting": {
                    "piece": {
                        "id": "1a1",
                        "type": {
                            "port": {
                                "specifiers": [
                                    {"context": "context1", "group": "group1"},
                                ]
                            }
                        },
                    }
                },
                "attracted": {
                    "piece": {
                        "id": "a",
                        "type": {
                            "port": {
                                "specifiers": [
                                    {"context": "context1", "group": "group1"},
                                ]
                            }
                        },
                    }
                },
            },
            {
                "attracting": {
                    "piece": {
                        "id": "1b1",
                        "type": {
                            "port": {
                                "specifiers": [
                                    {"context": "context1", "group": "group2"},
                                ]
                            }
                        },
                    }
                },
                "attracted": {
                    "piece": {
                        "id": "b",
                        "type": {
                            "port": {
                                "specifiers": [
                                    {"context": "context1", "group": "group2"},
                                ]
                            }
                        },
                    }
                },
            },
            {
                "attracting": {
                    "piece": {
                        "id": "1ab",
                        "type": {
                            "port": {
                                "specifiers": [
                                    {"context": "context1", "group": "group1"},
                                ]
                            }
                        },
                    }
                },
                "attracted": {
                    "piece": {
                        "id": "a",
                        "type": {
                            "port": {
                                "specifiers": [
                                    {"context": "context1", "group": "group2"},
                                ]
                            }
                        },
                    }
                },
            },
            {
                "attracting": {
                    "piece": {
                        "id": "1ab",
                        "type": {
                            "port": {
                                "specifiers": [
                                    {"context": "context1", "group": "group2"},
                                ]
                            }
                        },
                    }
                },
                "attracted": {
                    "piece": {
                        "id": "b",
                        "type": {
                            "port": {
                                "specifiers": [
                                    {"context": "context1", "group": "group1"},
                                ]
                            }
                        },
                    }
                },
            },
        ],
    }
    formation1Id = {
        "name": "formation1",
    }
    scene1 = {
        "objects": [
            {
                "piece": {
                    "id": "1",
                    "type": {
                        "representations": [{"url": "url1"}],
                    },
                },
                "plane": {
                    "origin": {"x": 0.0, "y": 0.0, "z": 0.0},
                    "xAxis": {"x": 1.0, "y": 0.0, "z": 0.0},
                    "yAxis": {"x": 0.0, "y": 1.0, "z": 0.0},
                },
                "parent": None,
            }
        ]
    }
    kit = {
        "name": "kit1",
        "types": [type1],
        "formations": [formation1],
    }
    createLocalKitResponse = client.execute(
        createLocalKit,
        variables={
            "directory": str(tmp_path),
            "kit": kit,
        },
    )
    assert not createLocalKitResponse.get("errors"), f"Errors: {createLocalKitResponse}"
    formation1ToSceneResponse = client.execute(
        formationToSceneFromLocalKit,
        variables={"directory": str(tmp_path), "formation": formation1Id},
    )
    formation1ToSceneResponseExpected = {
        "data": {
            "formationToScene": {
                "scene": scene1,
                "error": None,
            }
        }
    }
    formation1ToSceneResponseDiff = DeepDiff(
        formation1ToSceneResponse, formation1ToSceneResponseExpected
    )
    assert (
        not formation1ToSceneResponseDiff
    ), f"Response difference: {formation1ToSceneResponseDiff}"
