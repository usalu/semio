import pytest
import graphene
import deepdiff
import engine


@pytest.mark.parametrize(
    "yAxis, phi, expectedXAxis",
    [
        pytest.param(
            [0.0, 1.0, 0.0],
            0.0,
            [1.0, 0.0, 0.0],
            id="no rotation, no rotation",
        ),
        pytest.param(
            [0.0, 1.0, 0.0],
            135,
            [-0.707107, 0, -0.707107],
            id="no rotation, 135° around y rotation",
        ),
        pytest.param(
            [-0.707107, 0.707107, 0.0],
            0.0,
            [0.707107, 0.707107, 0],
            id="45° around z, no rotation",
        ),
        pytest.param(
            [0, 0.866025, -0.5],
            0.0,
            [1, 0, 0],
            id="-30° around x, no rotation",
        ),
        pytest.param(
            [0, 0.866025, -0.5],
            45,
            [0.707107, -0.353553, -0.612372],
            id="-30° around x, 45° rotation",
        ),
        pytest.param(
            [0.707107, -0.612372, 0.353553],
            45,
            [0.251059, -0.25, -0.935131],
            id="135° around z then -30° around x, 45° rotation",
        ),
    ],
)
def test_planeFromYAxis(yAxis, phi, expectedXAxis):
    yAxisVector = engine.Vector(*yAxis)
    plane = engine.Plane.fromYAxis(yAxisVector, phi)
    expectedPlane = engine.Plane(
        engine.Point(), engine.Vector(*expectedXAxis), yAxisVector
    )
    assert plane.isClose(expectedPlane)


# @pytest.mark.parametrize(
#     "code, entity",
#     [
#         pytest.param(
#             "",
#             {},
#             id="",
#         ),
#     ],
# )
# def test_get(code, entity):
#     pass


@pytest.mark.skip
def test_integration_graphql_local_kit_crud(tmp_path):
    pass


@pytest.mark.skip
def test_integration_graphql_local_kit_designToScene(tmp_path):
    pass
