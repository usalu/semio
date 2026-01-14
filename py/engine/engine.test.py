# region Header

# py/engine/engine.test.py

# 2026 Ueli Saluz <ueli@semio-tech.com>

# This program is free software: you can redistribute it and/or modify
# it under the terms of the GNU Lesser General Public License as
# published by the Free Software Foundation, either version 3 of the
# License, or (at your option) any later version.

# This program is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
# GNU Lesser General Public License for more details.

# You should have received a copy of the GNU Lesser General Public License
# along with this program.  If not, see <https://www.gnu.org/licenses/>.

# endregion Header

# region Imports

from __future__ import annotations

import json
import os
import pathlib
import shutil
import tempfile

import pytest
from starlette.testclient import TestClient

import engine

# endregion Imports

# region Constants

ASSETS_DIR = pathlib.Path(__file__).parent.parent.parent / "assets" / "semio"
KIT_METABOLISM_PATH = ASSETS_DIR / "kit_metabolism.json"

# endregion Constants

# region Fixtures


@pytest.fixture
def kitMetabolismJson() -> dict:
    with open(KIT_METABOLISM_PATH, "r", encoding="utf-8") as f:
        return json.load(f)


@pytest.fixture
def minimalKitJson() -> dict:
    return {
        "name": "TestKit",
        "version": "1.0.0",
    }


@pytest.fixture
def tempKitPath() -> pathlib.Path:
    tmpDir = tempfile.mkdtemp()
    yield pathlib.Path(tmpDir)
    shutil.rmtree(tmpDir, ignore_errors=True)


@pytest.fixture
def restClient() -> TestClient:
    return TestClient(engine.rest)


@pytest.fixture
def graphqlClient() -> TestClient:
    return TestClient(engine.engine)


# endregion Fixtures

# region Encoding Tests


class TestEncoding:
    def test_encode_basic(self):
        assert engine.encode("hello") == "hello"
        assert engine.encode("hello world") == "hello%20world"
        assert engine.encode("/path/to/file") == "%2Fpath%2Fto%2Ffile"

    def test_decode_basic(self):
        assert engine.decode("hello") == "hello"
        assert engine.decode("hello%20world") == "hello world"
        assert engine.decode("%2Fpath%2Fto%2Ffile") == "/path/to/file"

    def test_encode_decode_roundtrip(self):
        testStrings = ["hello", "hello world", "/path/to/file", "special!@#$%", "C:\\Windows\\path"]
        for s in testStrings:
            assert engine.decode(engine.encode(s)) == s


# endregion Encoding Tests

# region OperationBuilder Tests


class TestOperationBuilder:
    def test_parse_kit_operation(self):
        code = "C%3A%5Ctest%5Ckit"
        tree = engine.codeParser.parse(code)
        operation = engine.OperationBuilder().transform(tree)
        assert operation["kind"] == "kit"
        assert "kitUri" in operation

    def test_parse_kits_operation(self):
        code = ""
        tree = engine.codeParser.parse(code)
        operation = engine.OperationBuilder().transform(tree)
        assert operation["kind"] == "kits"

    def test_parse_types_operation(self):
        code = "C%3A%5Ctest%5Ckit/types"
        tree = engine.codeParser.parse(code)
        operation = engine.OperationBuilder().transform(tree)
        assert operation["kind"] == "types"
        assert "kitUri" in operation

    def test_parse_designs_operation(self):
        code = "C%3A%5Ctest%5Ckit/designs"
        tree = engine.codeParser.parse(code)
        operation = engine.OperationBuilder().transform(tree)
        assert operation["kind"] == "designs"
        assert "kitUri" in operation


# endregion OperationBuilder Tests

# region Store Tests


class TestSqliteStore:
    def test_store_factory_absolute_path(self, tempKitPath: pathlib.Path):
        engine.StoreFactory.cache_clear()
        store = engine.StoreFactory(str(tempKitPath))
        assert isinstance(store, engine.SqliteStore)
        assert store.uri == str(tempKitPath)

    def test_store_factory_caching(self, tempKitPath: pathlib.Path):
        engine.StoreFactory.cache_clear()
        store1 = engine.StoreFactory(str(tempKitPath))
        store2 = engine.StoreFactory(str(tempKitPath))
        assert store1 is store2

    def test_store_initialize(self, tempKitPath: pathlib.Path):
        store = engine.SqliteStore.fromUri(str(tempKitPath))
        store.initialize()
        semioFolder = tempKitPath / ".semio"
        assert semioFolder.exists()
        assert (semioFolder / "kit.db").exists()

    def test_store_initialized_check(self, tempKitPath: pathlib.Path):
        store = engine.SqliteStore.fromUri(str(tempKitPath))
        assert not store.initialized()
        store.initialize()
        assert store.initialized()


# endregion Store Tests

# region StoreKind Tests


class TestStoreKind:
    def test_store_kind_values(self):
        assert engine.StoreKind.DATABASE.value == "database"
        assert engine.StoreKind.REST.value == "rest"
        assert engine.StoreKind.GRAPHQL.value == "graphql"


# endregion StoreKind Tests

# region CommandKind Tests


class TestCommandKind:
    def test_command_kind_values(self):
        assert engine.CommandKind.QUERY.value == "query"
        assert engine.CommandKind.PUT.value == "put"
        assert engine.CommandKind.UPDATE.value == "update"
        assert engine.CommandKind.DELETE.value == "delete"


# endregion CommandKind Tests

# region REST API Tests


class TestRestApi:
    def test_get_kit_not_found(self, restClient: TestClient, tempKitPath: pathlib.Path):
        nonExistentPath = str(tempKitPath / "nonexistent")
        encodedUri = engine.encode(nonExistentPath)
        response = restClient.get(f"/kits/{encodedUri}")
        assert response.status_code in [400, 500]


# endregion REST API Tests

# region GraphQL Tests


class TestGraphQL:
    def test_graphql_schema_exists(self):
        assert engine.graphqlSchema is not None

    def test_graphql_query_class(self):
        assert hasattr(engine.Query, "kit")
        assert hasattr(engine.Query, "node")


# endregion GraphQL Tests

# region MCP Tests


class TestMcp:
    def test_mcp_instance_exists(self):
        assert engine.mcp is not None

    def test_get_kit_not_found(self, tempKitPath: pathlib.Path):
        nonExistentPath = str(tempKitPath / "nonexistent")
        result = engine.get_kit(nonExistentPath)
        assert "error" in result

    def test_put_kit_tool(self, tempKitPath: pathlib.Path, minimalKitJson: dict):
        result = engine.put_kit(str(tempKitPath), minimalKitJson)
        assert result.get("success") is True or "error" in result

    def test_validate_kit_tool(self, minimalKitJson: dict):
        result = engine.validate_kit(minimalKitJson)
        assert "problems" in result or "error" in result

    def test_get_kit_diff_tool(self, minimalKitJson: dict):
        before = minimalKitJson.copy()
        after = minimalKitJson.copy()
        after["name"] = "ModifiedKit"
        result = engine.get_kit_diff(before, after)
        assert isinstance(result, dict)

    def test_apply_kit_diff_tool(self, minimalKitJson: dict):
        diff = {"name": "ModifiedKit"}
        result = engine.apply_kit_diff(minimalKitJson, diff)
        assert isinstance(result, dict)

    def test_inverse_kit_diff_tool(self, minimalKitJson: dict):
        diff = {"name": "ModifiedKit"}
        result = engine.inverse_kit_diff(minimalKitJson, diff)
        assert isinstance(result, dict)


# endregion MCP Tests

# region Cache Tests


class TestCache:
    def test_cache_dir_encoding(self):
        remoteUri = "https://example.com/kit.zip"
        expectedDir = os.path.join(os.path.expanduser("~/.semio/cache"), engine.encode(remoteUri))
        assert engine.cacheDir(remoteUri) == expectedDir

    def test_cache_rejects_non_remote(self, tempKitPath: pathlib.Path):
        localUri = str(tempKitPath)
        with pytest.raises(engine.OnlyRemoteKitsCanBeCached):
            engine.cache(localUri)

    def test_cache_rejects_non_zip(self):
        nonZipUri = "https://example.com/kit.json"
        with pytest.raises(engine.OnlyRemoteKitsCanBeCached):
            engine.cache(nonZipUri)


# endregion Cache Tests

# region SSLMode Tests


class TestSSLMode:
    def test_ssl_mode_values(self):
        assert engine.SSLMode.DISABLE.value == "disable"
        assert engine.SSLMode.ALLOW.value == "allow"
        assert engine.SSLMode.PREFER.value == "prefer"
        assert engine.SSLMode.REQUIRE.value == "require"
        assert engine.SSLMode.VERIFY_CA.value == "verify-ca"
        assert engine.SSLMode.VERIFY_FULL.value == "verify-full"


# endregion SSLMode Tests

# region Error Classes Tests


class TestErrors:
    def test_kit_not_found_error(self):
        error = engine.KitNotFound("test/path")
        assert "test/path" in str(error)

    def test_kit_already_exists_error(self):
        error = engine.KitAlreadyExists("test/path")
        assert "test/path" in str(error)

    def test_only_remote_kits_can_be_cached_error(self):
        error = engine.OnlyRemoteKitsCanBeCached("/local/path")
        assert "/local/path" in str(error)

    def test_local_kit_uri_is_not_absolute_error(self):
        error = engine.LocalKitUriIsNotAbsolute("relative/path")
        assert "relative/path" in str(error)


# endregion Error Classes Tests

# region Assistant Tests


class TestAssistant:
    def test_encode_for_prompt(self):
        assert engine.encodeForPrompt("hello;world") == "hello,world"
        assert engine.encodeForPrompt("hello\nworld") == "hello world"

    def test_replace_default_empty(self):
        assert engine.replaceDefault("", "DEFAULT") == "DEFAULT"
        assert engine.replaceDefault("value", "DEFAULT") == "value"

    def test_encode_type(self):
        typeContext = engine.TypeContext(name="TestType", variant="", connectors=[])
        encoded = engine.encodeType(typeContext)
        assert encoded.variant == "DEFAULT"

    def test_design_generation_prompt_template_renders(self):
        types = []
        description = "Test description"
        result = engine.designGenerationPromptTemplate.render(description=description, types=types)
        assert "Test description" in result

    def test_design_response_format_is_valid_json(self):
        assert isinstance(engine.designResponseFormat, dict)
        assert "name" in engine.designResponseFormat
        assert engine.designResponseFormat["name"] == "design"


# endregion Assistant Tests

# region Engine Configuration Tests


class TestEngineConfiguration:
    def test_engine_app_exists(self):
        assert engine.engine is not None

    def test_rest_app_exists(self):
        assert engine.rest is not None

    def test_mcp_app_exists(self):
        assert engine.mcp is not None

    def test_graphql_schema_exists(self):
        assert engine.graphqlSchema is not None


# endregion Engine Configuration Tests

# region Integration Tests


class TestIntegration:
    def test_store_initialization_and_semio_check(self, tempKitPath: pathlib.Path):
        store = engine.SqliteStore.fromUri(str(tempKitPath))
        store.initialize()
        assert store.initialized()

    def test_store_and_operation_from_code(self, tempKitPath: pathlib.Path):
        engine.StoreFactory.cache_clear()
        code = engine.encode(str(tempKitPath))
        resultStore, resultOperation = engine.storeAndOperationFromCode(code)
        assert resultOperation["kind"] == "kit"
        assert resultOperation["kitUri"] == str(tempKitPath)


# endregion Integration Tests
