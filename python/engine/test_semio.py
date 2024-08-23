import pytest
import semio


@pytest.mark.parametrize(
    "url, entityDict",
    [
        pytest.param(
            "engine2.sqlite3/kit/types/Y2Fwc3VsZQ==,/representations/aHR0cHM6Ly93d3cuZ29vZ2xlLmNvbQ==",
            {},
            id="local, no scheme, no netloc, ",
        ),
    ],
)
def test_getRowByUrl(url, entityDict):
    row = semio.getRowByUrl(url)
    assert row == entityDict
    return
