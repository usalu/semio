# region Header
# [👤semio📚engine🥼enginetest](semiorepo://p/u/semio/b/l/engine/f/engine.test.py)

# 2026 Ueli Saluz <ueli@semio-tech.com>

# This program is free software: you can redistribute it and/or modify
# it under the terms of the GNU Affero General Public License as
# published by the Free Software Foundation, either version 3 of the
# License, or (at your option) any later version.
# This program is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
# GNU Affero General Public License for more details.
# You should have received a copy of the GNU Affero General Public License
# along with this program.  If not, see <https://www.gnu.org/licenses/>.

# endregion Header

# region Imports
from __future__ import annotations

import json
import os
import pathlib
import shutil
import tempfile

import engine
import pytest
from starlette.testclient import TestClient

# endregion Imports

# region Constants
ASSETS_DIR = pathlib.Path(__file__).parent.parent / "assets" / "semio"
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
        testStrings = [
            "hello",
            "hello world",
            "/path/to/file",
            "special!@#$%",
            "C:\\Windows\\path",
        ]
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
        assert response.status_code in [400, 404, 500]


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

    def test_flatten_design_tool(self, minimalKitJson: dict):
        result = engine.flatten_design(minimalKitJson, "test-design-guid")
        assert isinstance(result, dict)

    def test_pieces_metadata_tool(self, minimalKitJson: dict):
        result = engine.pieces_metadata(minimalKitJson, "test-design-guid")
        assert isinstance(result, dict)

    def test_get_primitive_design_tool(self):
        kit = {"name": "test", "designs": [{"guid": "d1", "name": "Design1"}]}
        result = engine.get_primitive_design(kit, "d1")
        assert result.get("guid") == "d1"

    def test_get_design_family_tool(self):
        kit = {"name": "test", "designs": [
            {"guid": "d1", "name": "Root"},
            {"guid": "d2", "name": "Child", "parent": {"guid": "d1"}},
        ]}
        result = engine.get_design_family(kit, "d2")
        assert isinstance(result, list)
        assert len(result) == 2

    def test_get_design_siblings_tool(self):
        kit = {"name": "test", "designs": [
            {"guid": "d1", "name": "Root"},
            {"guid": "d2", "name": "Child1", "parent": {"guid": "d1"}},
            {"guid": "d3", "name": "Child2", "parent": {"guid": "d1"}},
        ]}
        result = engine.get_design_siblings(kit, "d2")
        assert isinstance(result, list)
        assert len(result) == 1
        assert result[0].get("guid") == "d3"

    def test_get_design_children_tool(self):
        kit = {"name": "test", "designs": [
            {"guid": "d1", "name": "Root"},
            {"guid": "d2", "name": "Child", "parent": {"guid": "d1"}},
        ]}
        result = engine.get_design_children(kit, "d1")
        assert isinstance(result, list)
        assert len(result) == 1

    def test_are_designs_in_same_family_tool(self):
        kit = {"name": "test", "designs": [
            {"guid": "d1", "name": "Root"},
            {"guid": "d2", "name": "Child", "parent": {"guid": "d1"}},
        ]}
        result = engine.are_designs_in_same_family(kit, "d1", "d2")
        assert result.get("result") is True

    def test_can_use_design_as_piece_tool(self):
        kit = {"name": "test", "designs": [
            {"guid": "d1", "name": "Root"},
            {"guid": "d2", "name": "Other"},
        ]}
        result = engine.can_use_design_as_piece(kit, "d1", "d2")
        assert result.get("result") is True

    def test_find_same_family_design_pieces_tool(self):
        kit = {"name": "test", "designs": [
            {"guid": "d1", "name": "Design1", "pieces": [
                {"guid": "p1", "name": "Piece1", "design": {"guid": "d1"}},
            ]},
        ]}
        result = engine.find_same_family_design_pieces(kit, "d1")
        assert isinstance(result, list)

    def test_get_primitive_type_tool(self):
        kit = {"name": "test", "types": [{"guid": "t1", "name": "Type1"}]}
        result = engine.get_primitive_type(kit, "t1")
        assert result.get("guid") == "t1"

    def test_get_type_family_tool(self):
        kit = {"name": "test", "types": [
            {"guid": "t1", "name": "Root"},
            {"guid": "t2", "name": "Child", "parent": {"guid": "t1"}},
        ]}
        result = engine.get_type_family(kit, "t2")
        assert isinstance(result, list)
        assert len(result) == 2

    def test_get_type_siblings_tool(self):
        kit = {"name": "test", "types": [
            {"guid": "t1", "name": "Root"},
            {"guid": "t2", "name": "ChildA", "parent": {"guid": "t1"}},
            {"guid": "t3", "name": "ChildB", "parent": {"guid": "t1"}},
        ]}
        result = engine.get_type_siblings(kit, "t2")
        assert isinstance(result, list)
        assert len(result) == 1

    def test_get_type_children_tool(self):
        kit = {"name": "test", "types": [
            {"guid": "t1", "name": "Root"},
            {"guid": "t2", "name": "Child", "parent": {"guid": "t1"}},
        ]}
        result = engine.get_type_children(kit, "t1")
        assert isinstance(result, list)
        assert len(result) == 1

    def test_are_types_in_same_family_tool(self):
        kit = {"name": "test", "types": [
            {"guid": "t1", "name": "Root"},
            {"guid": "t2", "name": "Child", "parent": {"guid": "t1"}},
        ]}
        result = engine.are_types_in_same_family(kit, "t1", "t2")
        assert result.get("result") is True

    def test_find_piece_type_in_design_tool(self):
        kit = {"name": "test", "types": [{"guid": "t1", "name": "Type1"}], "designs": [
            {"guid": "d1", "name": "Design1", "pieces": [
                {"guid": "p1", "name": "Piece1", "type": {"guid": "t1"}},
            ]},
        ]}
        result = engine.find_piece_type_in_design(kit, "d1", "p1")
        assert result.get("guid") == "t1"

    def test_find_used_connectors_by_piece_in_design_tool(self):
        kit = {"name": "test", "types": [
            {"guid": "t1", "name": "Type1", "connectors": [{"guid": "c1", "name": "Con1"}]},
        ], "designs": [
            {"guid": "d1", "name": "Design1", "pieces": [
                {"guid": "p1", "name": "Piece1", "type": {"guid": "t1"}},
                {"guid": "p2", "name": "Piece2", "type": {"guid": "t1"}},
            ], "connections": [
                {"guid": "conn1", "connected": {"piece": {"guid": "p1"}, "connector": {"guid": "c1"}}, "connecting": {"piece": {"guid": "p2"}, "connector": {"guid": "c1"}}},
            ]},
        ]}
        result = engine.find_used_connectors_by_piece_in_design(kit, "d1", "p1")
        assert isinstance(result, list)
        assert len(result) == 1

    def test_create_clustered_design_tool(self):
        design = {"name": "test", "pieces": [
            {"guid": "p1", "name": "P1"},
            {"guid": "p2", "name": "P2"},
        ], "connections": [
            {"guid": "c1", "connected": {"piece": {"guid": "p1"}}, "connecting": {"piece": {"guid": "p2"}}},
        ]}
        result = engine.create_clustered_design(design, ["p1", "p2"], "Cluster")
        assert "clusteredDesign" in result
        assert "externalConnections" in result

    def test_get_clusterable_groups_tool(self):
        design = {"pieces": [
            {"guid": "p1", "name": "P1"},
            {"guid": "p2", "name": "P2"},
        ], "connections": [
            {"guid": "c1", "connected": {"piece": {"guid": "p1"}}, "connecting": {"piece": {"guid": "p2"}}},
        ]}
        result = engine.get_clusterable_groups(design, ["p1", "p2"])
        assert isinstance(result, list)

    def test_expand_design_pieces_tool(self):
        design = {"name": "test", "pieces": [{"guid": "p1"}], "connections": []}
        kit = {"name": "kit", "designs": [design]}
        result = engine.expand_design_pieces(design, kit)
        assert isinstance(result, dict)

    def test_find_attribute_value_tool(self):
        entity = {"attributes": [{"key": "color", "value": "red"}]}
        result = engine.find_attribute_value(entity, "color")
        assert result.get("value") == "red"

    def test_find_replaceable_types_for_piece_in_design_tool(self):
        kit = {"name": "test", "types": [
            {"guid": "t1", "name": "Type1", "connectors": [{"guid": "c1"}]},
            {"guid": "t2", "name": "Type2", "connectors": [{"guid": "c2"}]},
        ], "designs": [
            {"guid": "d1", "name": "Design1", "pieces": [
                {"guid": "p1", "name": "Piece1", "type": {"guid": "t1"}},
            ], "connections": []},
        ]}
        result = engine.find_replaceable_types_for_piece_in_design(kit, "d1", "p1")
        assert isinstance(result, list)

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

    def test_sum_quality_in_design_tool(self):
        kit = {"name": "test", "types": [
            {"guid": "t1", "name": "TypeA", "props": [
                {"guid": "p1", "quality": {"guid": "q1"}, "value": "10.5"},
            ]},
            {"guid": "t2", "name": "TypeB", "props": [
                {"guid": "p2", "quality": {"guid": "q1"}, "value": "20.0"},
            ]},
        ], "designs": [
            {"guid": "d1", "name": "Design1", "pieces": [
                {"guid": "pc1", "name": "Piece1", "type": {"guid": "t1"}},
                {"guid": "pc2", "name": "Piece2", "type": {"guid": "t2"}},
                {"guid": "pc3", "name": "Piece3", "type": {"guid": "t1"}},
            ]},
        ]}
        mock_ctx = type("MockCtx", (), {"session": object()})()
        engine._mcp_session_kits[id(mock_ctx.session)] = kit
        result = engine.sum_quality_in_design("d1", "q1", mock_ctx)
        assert abs(result.get("result") - 41.0) < 0.001

    def test_start_working_in_local_kit_loads_from_path(self):
        """start_working_in_local_kit loads kit from metabolism JSON path."""
        mock_ctx = type("MockCtx", (), {"session": object()})()
        result = engine.start_working_in_local_kit(str(KIT_METABOLISM_PATH), mock_ctx)
        assert result.get("ok") is True
        assert "kit_metabolism" in result.get("path", "")
        assert id(mock_ctx.session) in engine._mcp_session_kits

    def test_start_working_in_local_kit_loads_from_folder(self):
        """start_working_in_local_kit loads kit from folder containing kit_metabolism.json."""
        mock_ctx = type("MockCtx", (), {"session": object()})()
        result = engine.start_working_in_local_kit(str(ASSETS_DIR), mock_ctx)
        assert result.get("ok") is True
        kit = engine._mcp_session_kits[id(mock_ctx.session)]
        assert "designs" in kit

    def test_start_working_in_local_kit_clears_design_and_type(self):
        """start_working_in_local_kit clears any previously set design and type."""
        mock_ctx = type("MockCtx", (), {"session": object()})()
        sid = id(mock_ctx.session)
        engine._mcp_session_designs[sid] = {"guid": "old-design"}
        engine._mcp_session_types[sid] = {"guid": "old-type"}
        engine.start_working_in_local_kit(str(KIT_METABOLISM_PATH), mock_ctx)
        assert sid not in engine._mcp_session_designs
        assert sid not in engine._mcp_session_types

    def test_start_working_in_local_kit_and_sum_quality_metabolism(self, kitMetabolismJson: dict):
        """start_working_in_local_kit then sum_quality_in_design for Nakagin effective floor area."""
        mock_ctx = type("MockCtx", (), {"session": object()})()
        engine._mcp_session_kits[id(mock_ctx.session)] = kitMetabolismJson
        design = next(d for d in kitMetabolismJson.get("designs", []) if d.get("name") == "Nakagin Capsule Tower" and not d.get("parent"))
        quality = next(q for q in kitMetabolismJson.get("qualities", []) if q.get("name") == "effective floor area")
        result = engine.sum_quality_in_design(design["guid"], quality["guid"], mock_ctx)
        assert abs(result.get("result") - 2349.53) < 0.01

    def test_start_working_in_design(self, kitMetabolismJson: dict):
        """start_working_in_design selects a design by GUID from the session kit."""
        mock_ctx = type("MockCtx", (), {"session": object()})()
        sid = id(mock_ctx.session)
        engine._mcp_session_kits[sid] = kitMetabolismJson
        design = next(d for d in kitMetabolismJson.get("designs", []) if d.get("name") == "Nakagin Capsule Tower" and not d.get("parent"))
        result = engine.start_working_in_design(design["guid"], mock_ctx)
        assert result.get("ok") is True
        assert result.get("guid") == design["guid"]
        assert sid in engine._mcp_session_designs
        assert engine._mcp_session_designs[sid]["guid"] == design["guid"]

    def test_start_working_in_design_not_found(self, kitMetabolismJson: dict):
        """start_working_in_design returns error for unknown GUID."""
        mock_ctx = type("MockCtx", (), {"session": object()})()
        engine._mcp_session_kits[id(mock_ctx.session)] = kitMetabolismJson
        result = engine.start_working_in_design("nonexistent-guid", mock_ctx)
        assert "error" in result

    def test_read_current_design(self, kitMetabolismJson: dict):
        """read_current_design returns the design set by start_working_in_design."""
        mock_ctx = type("MockCtx", (), {"session": object()})()
        sid = id(mock_ctx.session)
        engine._mcp_session_kits[sid] = kitMetabolismJson
        design = next(d for d in kitMetabolismJson.get("designs", []) if d.get("name") == "Nakagin Capsule Tower" and not d.get("parent"))
        engine.start_working_in_design(design["guid"], mock_ctx)
        result = engine.read_current_design(mock_ctx)
        assert result.get("guid") == design["guid"]
        assert result.get("name") == "Nakagin Capsule Tower"

    def test_read_current_design_without_start(self):
        """read_current_design returns error if no design was set."""
        mock_ctx = type("MockCtx", (), {"session": object()})()
        result = engine.read_current_design(mock_ctx)
        assert "error" in result

    def test_finish_working_in_design(self, kitMetabolismJson: dict):
        """finish_working_in_design clears the current design from session."""
        mock_ctx = type("MockCtx", (), {"session": object()})()
        sid = id(mock_ctx.session)
        engine._mcp_session_kits[sid] = kitMetabolismJson
        design = next(d for d in kitMetabolismJson.get("designs", []) if d.get("name") == "Nakagin Capsule Tower" and not d.get("parent"))
        engine.start_working_in_design(design["guid"], mock_ctx)
        assert sid in engine._mcp_session_designs
        result = engine.finish_working_in_design(mock_ctx)
        assert result.get("ok") is True
        assert sid not in engine._mcp_session_designs

    def test_start_working_in_type(self, kitMetabolismJson: dict):
        """start_working_in_type selects a type by GUID from the session kit."""
        mock_ctx = type("MockCtx", (), {"session": object()})()
        sid = id(mock_ctx.session)
        engine._mcp_session_kits[sid] = kitMetabolismJson
        t = kitMetabolismJson.get("types", [])[0]
        result = engine.start_working_in_type(t["guid"], mock_ctx)
        assert result.get("ok") is True
        assert result.get("guid") == t["guid"]
        assert sid in engine._mcp_session_types
        assert engine._mcp_session_types[sid]["guid"] == t["guid"]

    def test_start_working_in_type_not_found(self, kitMetabolismJson: dict):
        """start_working_in_type returns error for unknown GUID."""
        mock_ctx = type("MockCtx", (), {"session": object()})()
        engine._mcp_session_kits[id(mock_ctx.session)] = kitMetabolismJson
        result = engine.start_working_in_type("nonexistent-guid", mock_ctx)
        assert "error" in result

    def test_read_current_type(self, kitMetabolismJson: dict):
        """read_current_type returns the type set by start_working_in_type."""
        mock_ctx = type("MockCtx", (), {"session": object()})()
        sid = id(mock_ctx.session)
        engine._mcp_session_kits[sid] = kitMetabolismJson
        t = kitMetabolismJson.get("types", [])[0]
        engine.start_working_in_type(t["guid"], mock_ctx)
        result = engine.read_current_type(mock_ctx)
        assert result.get("guid") == t["guid"]

    def test_read_current_type_without_start(self):
        """read_current_type returns error if no type was set."""
        mock_ctx = type("MockCtx", (), {"session": object()})()
        result = engine.read_current_type(mock_ctx)
        assert "error" in result

    def test_finish_working_in_type(self, kitMetabolismJson: dict):
        """finish_working_in_type clears the current type from session."""
        mock_ctx = type("MockCtx", (), {"session": object()})()
        sid = id(mock_ctx.session)
        engine._mcp_session_kits[sid] = kitMetabolismJson
        t = kitMetabolismJson.get("types", [])[0]
        engine.start_working_in_type(t["guid"], mock_ctx)
        assert sid in engine._mcp_session_types
        result = engine.finish_working_in_type(mock_ctx)
        assert result.get("ok") is True
        assert sid not in engine._mcp_session_types

    def test_finish_working_in_kit(self, kitMetabolismJson: dict):
        """finish_working_in_kit clears kit, design, and type from session."""
        mock_ctx = type("MockCtx", (), {"session": object()})()
        sid = id(mock_ctx.session)
        engine._mcp_session_kits[sid] = kitMetabolismJson
        design = next(d for d in kitMetabolismJson.get("designs", []) if d.get("name") == "Nakagin Capsule Tower" and not d.get("parent"))
        engine.start_working_in_design(design["guid"], mock_ctx)
        t = kitMetabolismJson.get("types", [])[0]
        engine.start_working_in_type(t["guid"], mock_ctx)
        result = engine.finish_working_in_kit(mock_ctx)
        assert result.get("ok") is True
        assert sid not in engine._mcp_session_kits
        assert sid not in engine._mcp_session_designs
        assert sid not in engine._mcp_session_types


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
