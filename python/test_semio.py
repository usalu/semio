from graphene.test import Client

from semio import schema

typeQuery = """{
        name
        representations {
          url
          lod
          tags
        }
        ports {
          plane {
            origin {
              x
              y
              z
            }
            xAxis {
              x
              y
              z
            }
            yAxis {
              x
              y
              z
            }
          }
          specifiers {
            context
            group
          }
        }
        qualities {
          name
          value
          unit
        }"""

formationQuery = """{
      name
      explanation
      icon
      pieces {
        transient {
          id
        }
        type {
          name
          qualities {
            name
            value
            unit
          }
        }
      }
      attractions {
        attracting {
          piece {
            transient {
              id
            }
            type {
              port {
                specifiers {
                  context
                  group
                }
              }
            }
          }
        }
        attracted {
          piece {
            transient {
              id
            }
            type {
              port {
                specifiers {
                  group
                  context
                }
              }
            }
          }
        }
      }
      qualities {
        name
        value
        unit
      }
    }"""

kitQuery = (
    """{
      name
      explanation
      icon
      url
      types """
    + typeQuery
    + """
      formations """
    + formationQuery
    + """
    }"""
)

createLocalKitMutation = (
    """mutation CreateLocalKit($directory: String!, $kit: KitInput!) {
  createLocalKit(directory: $directory, kitInput: $kit) {
    kit """
    + kitQuery
    + """
    error{
      code
      message
    }
  }
}"""
)

addTypeToLocalKitMutation = (
    """mutation AddTypeToLocalKit($directory: String!, $type: TypeInput!) {
  addTypeToLocalKit(directory: $directory, typeInput: $type) {
    type """
    + typeQuery
    + """
    error {
      code
      message
    }
  }
}"""
)

removeTypeFromLocalKitMutation = """mutation RemoveTypeFromLocalKit($directory: String!, $type: TypeIdInput!) {
  removeTypeFromLocalKit(directory: $directory, typeId: $type) {
    error {
      code
      message
    }
  }
}"""

deleteLocalKitMutation = """mutation DeleteLocalKit($directory: String!){
  deleteLocalKit(
    directory:$directory,
  ){
    error
  }
}"""


def test_kit_crud(tmp_path):
    client = Client(schema)
    name = "metabolistic"
    explanation = "A metabolism clone"
    icon = "🫀"
    url = "https://github.com/usalu/semio/tree/main/examples/metabolic"
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
        createLocalKitMutation, variables={"directory": str(tmp_path), "kit": kit}
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
        removeLocalKitMutation, variables={"directory": str(tmp_path), "type": shaftId}
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
        addTypeToLocalKitMutation,
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
