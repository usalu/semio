import pytest
import semio


@pytest.mark.parametrize(
    "guid, entity",
    [
        pytest.param(
            "",
            {},
            id="no rotation, no rotation",
        ),
    ],
)
def test_entityByGuid(guid, entity):
    pass
