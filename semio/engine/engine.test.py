# region Header
# [👤semio📚engine🥼enginetest](repo://p/u/semio/b/l/engine/f/engine.test.py)

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

import importlib.util as _ilu
import json
import os
import pathlib
import shutil
import sys as _sys
import tempfile
from unittest.mock import MagicMock, patch

import pytest
from mcp.types import CallToolResult
from starlette.testclient import TestClient

_engine_path = str(pathlib.Path(__file__).parent / "main.py")
_engine_spec = _ilu.spec_from_file_location("engine", _engine_path)
engine = _ilu.module_from_spec(_engine_spec)
_sys.modules["engine"] = engine
_engine_spec.loader.exec_module(engine)


def _mcp_app_tool_payload(result: object) -> dict:
    """Unpack kit/design MCP app tool returns (CallToolResult with structuredContent)."""
    assert isinstance(result, CallToolResult), result
    assert result.structuredContent is not None
    return result.structuredContent


# endregion Imports

# region Constants
ASSETS_DIR = pathlib.Path(__file__).parent.parent / "assets" / "semio"
KIT_METABOLISM_PATH = ASSETS_DIR / "metabolism.kit.semio.json"
METABOLISM_DIR = ASSETS_DIR / "metabolism"

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
        assert operation["kind"] == engine.OperationKind.KIT
        assert "kitUri" in operation

    def test_parse_kits_operation(self):
        code = ""
        tree = engine.codeParser.parse(code)
        operation = engine.OperationBuilder().transform(tree)
        assert operation["kind"] == engine.OperationKind.KITS

    def test_parse_types_operation(self):
        code = "C%3A%5Ctest%5Ckit/types"
        tree = engine.codeParser.parse(code)
        operation = engine.OperationBuilder().transform(tree)
        assert operation["kind"] == engine.OperationKind.TYPES
        assert "kitUri" in operation

    def test_parse_designs_operation(self):
        code = "C%3A%5Ctest%5Ckit/designs"
        tree = engine.codeParser.parse(code)
        operation = engine.OperationBuilder().transform(tree)
        assert operation["kind"] == engine.OperationKind.DESIGNS
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

    def test_mcp_kit_tools_reference_kit_viewer_resource(self):
        """Kit-loading tools declare ui://semio/kit-viewer; diagram tools use design-viewer."""
        tools = {t.name: t for t in engine.mcp._tool_manager.list_tools()}
        for name in ("start_working_in_local_kit", "start_new_kit", "start_working_in_remote_kit"):
            assert tools[name].meta["ui"]["resourceUri"] == "ui://semio/kit-viewer"
        assert tools["show_design"].meta["ui"]["resourceUri"] == "ui://semio/design-viewer"

    def test_mcp_app_html_resources_include_ui_csp_meta(self):
        """MCP App HTML resources expose _meta.ui.csp so hosts allow network access to the engine (see .repo/✍️/mcp-app.md)."""
        resources = {str(r.uri): r for r in engine.mcp._resource_manager.list_resources()}
        for uri in ("ui://semio/design-viewer", "ui://semio/kit-viewer"):
            r = resources[uri]
            assert r.meta is not None
            csp = r.meta.get("ui", {}).get("csp")
            assert csp is not None
            assert isinstance(csp.get("connectDomains"), list) and len(csp["connectDomains"]) > 0
            assert r.meta.get("ui/csp") is csp

    def test_mcp_tool_surface_keeps_only_allowed_prefixes(self):
        names = sorted(tool.name for tool in engine.mcp._tool_manager.list_tools())
        assert names == [
            "add_current_design_author",
            "add_current_design_connection",
            "add_current_design_piece",
            "add_current_design_piece_with_plane",
            "add_current_design_prop",
            "clear_current_selection",
            "finish_working_in_design",
            "finish_working_in_kit",
            "finish_working_in_type",
            "read_current_design",
            "read_current_kit",
            "read_current_selection",
            "read_current_type",
            "select_connections",
            "select_pieces",
            "select_pieces_and_connections",
            "set_current_selection",
            "show_design",
            "show_diagram",
            "show_diagram_diff",
            "show_diff",
            "show_scene",
            "start_new_design",
            "start_new_kit",
            "start_transaction",
            "start_working_in_design",
            "start_working_in_local_kit",
            "start_working_in_remote_kit",
            "start_working_in_type",
            "sum_quality_in_design",
            "transaction_abort",
            "transaction_finalize",
        ]

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
        kit = {
            "name": "test",
            "designs": [
                {"guid": "d1", "name": "Root"},
                {"guid": "d2", "name": "Child", "parent": {"guid": "d1"}},
            ],
        }
        result = engine.get_design_family(kit, "d2")
        assert isinstance(result, list)
        assert len(result) == 2

    def test_get_design_siblings_tool(self):
        kit = {
            "name": "test",
            "designs": [
                {"guid": "d1", "name": "Root"},
                {"guid": "d2", "name": "Child1", "parent": {"guid": "d1"}},
                {"guid": "d3", "name": "Child2", "parent": {"guid": "d1"}},
            ],
        }
        result = engine.get_design_siblings(kit, "d2")
        assert isinstance(result, list)
        assert len(result) == 1
        assert result[0].get("guid") == "d3"

    def test_get_design_children_tool(self):
        kit = {
            "name": "test",
            "designs": [
                {"guid": "d1", "name": "Root"},
                {"guid": "d2", "name": "Child", "parent": {"guid": "d1"}},
            ],
        }
        result = engine.get_design_children(kit, "d1")
        assert isinstance(result, list)
        assert len(result) == 1

    def test_are_designs_in_same_family_tool(self):
        kit = {
            "name": "test",
            "designs": [
                {"guid": "d1", "name": "Root"},
                {"guid": "d2", "name": "Child", "parent": {"guid": "d1"}},
            ],
        }
        result = engine.are_designs_in_same_family(kit, "d1", "d2")
        assert result.get("result") is True

    def test_can_use_design_as_piece_tool(self):
        kit = {
            "name": "test",
            "designs": [
                {"guid": "d1", "name": "Root"},
                {"guid": "d2", "name": "Other"},
            ],
        }
        result = engine.can_use_design_as_piece(kit, "d1", "d2")
        assert result.get("result") is True

    def test_find_same_family_design_pieces_tool(self):
        kit = {
            "name": "test",
            "designs": [
                {
                    "guid": "d1",
                    "name": "Design1",
                    "pieces": [
                        {"guid": "p1", "name": "Piece1", "design": {"guid": "d1"}},
                    ],
                },
            ],
        }
        result = engine.find_same_family_design_pieces(kit, "d1")
        assert isinstance(result, list)

    def test_get_primitive_type_tool(self):
        kit = {"name": "test", "types": [{"guid": "t1", "name": "Type1"}]}
        result = engine.get_primitive_type(kit, "t1")
        assert result.get("guid") == "t1"

    def test_get_type_family_tool(self):
        kit = {
            "name": "test",
            "types": [
                {"guid": "t1", "name": "Root"},
                {"guid": "t2", "name": "Child", "parent": {"guid": "t1"}},
            ],
        }
        result = engine.get_type_family(kit, "t2")
        assert isinstance(result, list)
        assert len(result) == 2

    def test_get_type_siblings_tool(self):
        kit = {
            "name": "test",
            "types": [
                {"guid": "t1", "name": "Root"},
                {"guid": "t2", "name": "ChildA", "parent": {"guid": "t1"}},
                {"guid": "t3", "name": "ChildB", "parent": {"guid": "t1"}},
            ],
        }
        result = engine.get_type_siblings(kit, "t2")
        assert isinstance(result, list)
        assert len(result) == 1

    def test_get_type_children_tool(self):
        kit = {
            "name": "test",
            "types": [
                {"guid": "t1", "name": "Root"},
                {"guid": "t2", "name": "Child", "parent": {"guid": "t1"}},
            ],
        }
        result = engine.get_type_children(kit, "t1")
        assert isinstance(result, list)
        assert len(result) == 1

    def test_are_types_in_same_family_tool(self):
        kit = {
            "name": "test",
            "types": [
                {"guid": "t1", "name": "Root"},
                {"guid": "t2", "name": "Child", "parent": {"guid": "t1"}},
            ],
        }
        result = engine.are_types_in_same_family(kit, "t1", "t2")
        assert result.get("result") is True

    def test_find_piece_type_in_design_tool(self):
        kit = {
            "name": "test",
            "types": [{"guid": "t1", "name": "Type1"}],
            "designs": [
                {
                    "guid": "d1",
                    "name": "Design1",
                    "pieces": [
                        {"guid": "p1", "name": "Piece1", "type": {"guid": "t1"}},
                    ],
                },
            ],
        }
        result = engine.find_piece_type_in_design(kit, "d1", "p1")
        assert result.get("guid") == "t1"

    def test_find_used_connectors_by_piece_in_design_tool(self):
        kit = {
            "name": "test",
            "types": [
                {"guid": "t1", "name": "Type1", "connectors": [{"guid": "c1", "name": "Con1"}]},
            ],
            "designs": [
                {
                    "guid": "d1",
                    "name": "Design1",
                    "pieces": [
                        {"guid": "p1", "name": "Piece1", "type": {"guid": "t1"}},
                        {"guid": "p2", "name": "Piece2", "type": {"guid": "t1"}},
                    ],
                    "connections": [
                        {"guid": "conn1", "connected": {"piece": {"guid": "p1"}, "connector": {"guid": "c1"}}, "connecting": {"piece": {"guid": "p2"}, "connector": {"guid": "c1"}}},
                    ],
                },
            ],
        }
        result = engine.find_used_connectors_by_piece_in_design(kit, "d1", "p1")
        assert isinstance(result, list)
        assert len(result) == 1

    def test_create_clustered_design_tool(self):
        design = {
            "name": "test",
            "pieces": [
                {"guid": "p1", "name": "P1"},
                {"guid": "p2", "name": "P2"},
            ],
            "connections": [
                {"guid": "c1", "connected": {"piece": {"guid": "p1"}}, "connecting": {"piece": {"guid": "p2"}}},
            ],
        }
        result = engine.create_clustered_design(design, ["p1", "p2"], "Cluster")
        assert "clusteredDesign" in result
        assert "externalConnections" in result

    def test_get_clusterable_groups_tool(self):
        design = {
            "pieces": [
                {"guid": "p1", "name": "P1"},
                {"guid": "p2", "name": "P2"},
            ],
            "connections": [
                {"guid": "c1", "connected": {"piece": {"guid": "p1"}}, "connecting": {"piece": {"guid": "p2"}}},
            ],
        }
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
        kit = {
            "name": "test",
            "types": [
                {"guid": "t1", "name": "Type1", "connectors": [{"guid": "c1"}]},
                {"guid": "t2", "name": "Type2", "connectors": [{"guid": "c2"}]},
            ],
            "designs": [
                {
                    "guid": "d1",
                    "name": "Design1",
                    "pieces": [
                        {"guid": "p1", "name": "Piece1", "type": {"guid": "t1"}},
                    ],
                    "connections": [],
                },
            ],
        }
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
        kit = {
            "name": "test",
            "types": [
                {
                    "guid": "t1",
                    "name": "TypeA",
                    "props": [
                        {"guid": "p1", "quality": {"guid": "q1"}, "value": "10.5"},
                    ],
                },
                {
                    "guid": "t2",
                    "name": "TypeB",
                    "props": [
                        {"guid": "p2", "quality": {"guid": "q1"}, "value": "20.0"},
                    ],
                },
            ],
            "designs": [
                {
                    "guid": "d1",
                    "name": "Design1",
                    "pieces": [
                        {"guid": "pc1", "name": "Piece1", "type": {"guid": "t1"}},
                        {"guid": "pc2", "name": "Piece2", "type": {"guid": "t2"}},
                        {"guid": "pc3", "name": "Piece3", "type": {"guid": "t1"}},
                    ],
                },
            ],
        }
        mock_ctx = type("MockCtx", (), {"session": object()})()
        engine._mcp_session_kits[id(mock_ctx.session)] = kit
        result = engine.sum_quality_in_design("d1", "q1", mock_ctx)
        assert abs(result.get("result") - 41.0) < 0.001

    def test_start_working_in_local_kit_loads_from_path(self):
        """start_working_in_local_kit loads kit from metabolism JSON path."""
        mock_ctx = type("MockCtx", (), {"session": object()})()
        result = engine.start_working_in_local_kit(str(KIT_METABOLISM_PATH), mock_ctx)
        assert isinstance(result, CallToolResult)
        payload = _mcp_app_tool_payload(result)
        assert "kitArtifacts" in payload
        assert id(mock_ctx.session) in engine._mcp_session_kits

    def test_start_working_in_local_kit_loads_from_folder(self):
        """start_working_in_local_kit loads kit from folder containing metabolism.kit.semio.json."""
        mock_ctx = type("MockCtx", (), {"session": object()})()
        result = engine.start_working_in_local_kit(str(ASSETS_DIR), mock_ctx)
        assert isinstance(result, CallToolResult)
        payload = _mcp_app_tool_payload(result)
        assert "kitArtifacts" in payload
        kit = engine._mcp_session_kits[id(mock_ctx.session)]
        assert "designs" in kit

    def test_start_working_in_local_kit_loads_from_metabolism_folder(self):
        """start_working_in_local_kit loads kit from a folder backed by .semio/kit.db."""
        mock_ctx = type("MockCtx", (), {"session": object()})()
        result = engine.start_working_in_local_kit(str(METABOLISM_DIR), mock_ctx)
        assert isinstance(result, CallToolResult)
        payload = _mcp_app_tool_payload(result)
        assert "kitArtifacts" in payload
        assert payload["kitArtifacts"]["name"] == "Metabolism"
        assert payload["kitArtifacts"].get("version") == "r25.07-1"
        flat_variant = next(design for design in payload["kitArtifacts"]["designs"] if design.get("guid") == "019ab4e0-7295-7e1e-bb5f-9dfae8c0c4cf")
        assert flat_variant.get("parent") == {"guid": "9a890dd4-0a9c-48ac-920a-9e62666465ef"}
        root_design = next(design for design in payload["kitArtifacts"]["designs"] if design.get("guid") == "9a890dd4-0a9c-48ac-920a-9e62666465ef")
        assert "Japanese Metabolism" in root_design.get("description", "")
        assert root_design.get("image") == "images/nakagin-capsule-tower.png"
        ellipsoid = next(kind for kind in payload["kitArtifacts"]["types"] if kind.get("guid") == "4ca3b87b-cd76-4228-9f7e-1459b711f0ab")
        assert ellipsoid.get("parent") == {"guid": "71749140-9db9-43f6-bd81-d89011667b80"}
        assert ellipsoid.get("name") == "Ellipsoid"
        kit = engine._mcp_session_kits[id(mock_ctx.session)]
        assert "designs" in kit
        assert any(design.get("name") == "Nakagin Capsule Tower" for design in kit.get("designs", []))

    def test_build_kit_artifact_data_preserves_parent_dependencies(self):
        """_build_kit_artifact_data keeps nested design and type parent refs for breadcrumb chains."""
        payload = engine._build_kit_artifact_data(
            {
                "guid": "kit-guid",
                "name": "Metabolism",
                "version": "1",
                "description": "Kit description",
                "homepage": "https://example.com/kit",
                "designs": [
                    {"guid": "root-design", "name": "Root", "description": "Root design", "image": "root.png"},
                    {"guid": "child-design", "name": "Child", "parent": {"guid": "root-design"}, "createdAt": "2026-03-27T00:00:00Z"},
                ],
                "types": [
                    {"guid": "root-kind", "name": "Root Kind"},
                    {"guid": "child-kind", "name": "Child Kind", "parent": {"guid": "root-kind"}, "description": "Child kind", "connectors": []},
                ],
            }
        )

        assert payload["description"] == "Kit description"
        assert payload["homepage"] == "https://example.com/kit"
        assert payload["designs"][0]["description"] == "Root design"
        assert payload["designs"][0]["image"] == "root.png"
        assert payload["designs"][1]["parent"] == {"guid": "root-design"}
        assert payload["designs"][1]["createdAt"] == "2026-03-27T00:00:00Z"
        assert payload["types"][1]["parent"] == {"guid": "root-kind"}
        assert payload["types"][1]["description"] == "Child kind"

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
        """start_working_in_design selects a design by GUID from the session kit and opens the MCP app payload."""
        mock_ctx = type("MockCtx", (), {"session": object()})()
        sid = id(mock_ctx.session)
        engine._mcp_session_kits[sid] = kitMetabolismJson
        design = next(d for d in kitMetabolismJson.get("designs", []) if d.get("name") == "Nakagin Capsule Tower" and not d.get("parent"))
        result = engine.start_working_in_design(design["guid"], mock_ctx)
        assert isinstance(result, CallToolResult)
        payload = _mcp_app_tool_payload(result)
        assert "points" in payload and "lines" in payload
        assert "kitArtifacts" in payload
        assert "designs" in payload["kitArtifacts"]
        assert sid in engine._mcp_session_designs
        assert engine._mcp_session_designs[sid]["guid"] == design["guid"]

    def test_start_working_in_design_not_found(self, kitMetabolismJson: dict):
        """start_working_in_design returns error for unknown GUID."""
        mock_ctx = type("MockCtx", (), {"session": object()})()
        engine._mcp_session_kits[id(mock_ctx.session)] = kitMetabolismJson
        result = engine.start_working_in_design("nonexistent-guid", mock_ctx)
        assert isinstance(result, CallToolResult)
        assert result.isError is True
        payload = _mcp_app_tool_payload(result)
        assert "error" in payload

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

    def test_start_transaction_rejects_nested_transaction(self):
        mock_ctx = type("MockCtx", (), {"session": object()})()
        sid = id(mock_ctx.session)
        try:
            first = engine.start_transaction(mock_ctx)
            second = engine.start_transaction(mock_ctx)
            assert first.get("ok") is True
            assert "error" in second
        finally:
            engine._mcp_session_transactions.pop(sid, None)

    def test_finalize_transaction_removes_active_transaction(self):
        mock_ctx = type("MockCtx", (), {"session": object()})()
        sid = id(mock_ctx.session)
        started = engine.start_transaction(mock_ctx)
        assert started.get("ok") is True
        result = engine.finalize_transaction(mock_ctx)
        assert result.get("ok") is True
        assert sid not in engine._mcp_session_transactions

    def test_abort_transaction_unwinds_recorded_kit_changes(self):
        mock_ctx = type("MockCtx", (), {"session": object()})()
        sid = id(mock_ctx.session)
        initial_kit = {"name": "Initial", "version": "1.0.0", "designs": [], "types": []}
        changed_kit = {"name": "Changed", "version": "1.0.0", "designs": [], "types": []}
        engine._mcp_session_kits[sid] = initial_kit
        started = engine.start_transaction(mock_ctx)
        assert started.get("ok") is True
        engine._set_session_kit(mock_ctx, changed_kit)
        engine._clear_session_kit(mock_ctx)
        assert sid not in engine._mcp_session_kits
        result = engine.abort_transaction(mock_ctx)
        assert result.get("ok") is True
        assert sid not in engine._mcp_session_transactions
        assert sid in engine._mcp_session_kits
        assert engine._mcp_session_kits[sid].get("name") == "Initial"

    def test_abort_transaction_without_active_transaction_returns_error(self):
        mock_ctx = type("MockCtx", (), {"session": object()})()
        result = engine.abort_transaction(mock_ctx)
        assert "error" in result

    def test_stateful_flat_tools_build_nakagin_capsule_tower_flat_payload(self, kitMetabolismJson: dict):
        mock_ctx = type("MockCtx", (), {"session": object()})()
        expected_design = next(d for d in kitMetabolismJson.get("designs", []) if d.get("name") == "Nakagin Capsule Tower" and not d.get("parent"))

        started_kit = engine.start_new_kit("Temporary Kit", "1.0.0", mock_ctx)
        assert isinstance(started_kit, CallToolResult)
        started_kit_payload = _mcp_app_tool_payload(started_kit)
        assert "kitArtifacts" in started_kit_payload

        started_transaction = engine.start_transaction(mock_ctx)
        assert started_transaction.get("ok") is True
        aborted_design = engine.start_new_design(
            "aborted-design",
            "Aborted Draft",
            "should be rolled back",
            "m",
            "icons/aborted.svg",
            "images/aborted.png",
            "2025-01-01T00:00:00.000Z",
            "2025-01-01T00:00:00.000Z",
            mock_ctx,
        )
        assert aborted_design.get("ok") is True
        aborted_piece = engine.add_current_design_piece(
            "aborted-piece",
            "x",
            "aborted-kind",
            mock_ctx,
        )
        assert aborted_piece.get("ok") is True
        aborted = engine.transaction_abort(mock_ctx)
        assert aborted.get("ok") is True
        current_kit_after_abort = engine.read_current_kit(mock_ctx)
        assert current_kit_after_abort.get("designs") == []
        assert "error" in engine.read_current_design(mock_ctx)

        started_transaction = engine.start_transaction(mock_ctx)
        assert started_transaction.get("ok") is True
        created_design = engine.start_new_design(
            expected_design["guid"],
            expected_design["name"],
            expected_design["description"],
            expected_design["unit"],
            expected_design["icon"],
            expected_design["image"],
            expected_design["createdAt"],
            expected_design["updatedAt"],
            mock_ctx,
        )
        assert created_design.get("ok") is True

        for author in expected_design.get("authors", []):
            result = engine.add_current_design_author(author["guid"], mock_ctx)
            assert result.get("ok") is True

        for prop in expected_design.get("props", []):
            result = engine.add_current_design_prop(
                prop["guid"],
                prop["quality"]["guid"],
                prop["value"],
                prop["unit"],
                mock_ctx,
            )
            assert result.get("ok") is True

        for piece in expected_design.get("pieces", []):
            if "plane" in piece and "center" in piece:
                plane = piece["plane"]
                result = engine.add_current_design_piece_with_plane(
                    piece["guid"],
                    piece["name"],
                    piece["type"]["guid"],
                    piece["center"]["u"],
                    piece["center"]["v"],
                    plane["origin"]["x"],
                    plane["origin"]["y"],
                    plane["origin"]["z"],
                    plane["xAxis"]["x"],
                    plane["xAxis"]["y"],
                    plane["xAxis"]["z"],
                    plane["yAxis"]["x"],
                    plane["yAxis"]["y"],
                    plane["yAxis"]["z"],
                    mock_ctx,
                    description=piece["description"],
                    is_hidden=piece["isHidden"],
                    is_locked=piece["isLocked"],
                )
            else:
                result = engine.add_current_design_piece(
                    piece["guid"],
                    piece["name"],
                    piece["type"]["guid"],
                    mock_ctx,
                    description=piece["description"],
                    is_hidden=piece["isHidden"],
                    is_locked=piece["isLocked"],
                )
            assert result.get("ok") is True

        for connection in expected_design.get("connections", []):
            result = engine.add_current_design_connection(
                connection["guid"],
                connection["connected"]["piece"]["guid"],
                connection["connected"]["connector"]["guid"],
                connection["connecting"]["piece"]["guid"],
                connection["connecting"]["connector"]["guid"],
                connection["rotation"],
                connection["u"],
                connection["v"],
                connection["shift"],
                mock_ctx,
                description=connection["description"],
                gap=connection["gap"],
                rise=connection["rise"],
                tilt=connection["tilt"],
                turn=connection["turn"],
            )
            assert result.get("ok") is True

        finalized = engine.transaction_finalize(mock_ctx)
        assert finalized.get("ok") is True
        current_design = engine.read_current_design(mock_ctx)
        expected_flat_design = {key: value for key, value in expected_design.items() if key != "layers"}
        assert current_design == expected_flat_design
        assert "layers" not in current_design

    def test_read_current_selection_default_empty(self):
        """read_current_selection returns empty lists when no selection is set."""
        mock_ctx = type("MockCtx", (), {"session": object()})()
        result = engine.read_current_selection(mock_ctx)
        assert result == {"pieceGuids": [], "connectionGuids": []}

    def test_set_current_selection_pieces(self):
        """set_current_selection stores piece guids in session."""
        mock_ctx = type("MockCtx", (), {"session": object()})()
        result = engine.set_current_selection(mock_ctx, piece_guids=["p1", "p2"])
        assert result.get("ok") is True
        sel = engine.read_current_selection(mock_ctx)
        assert sel["pieceGuids"] == ["p1", "p2"]
        assert sel["connectionGuids"] == []

    def test_set_current_selection_connections(self):
        """set_current_selection stores connection guids in session."""
        mock_ctx = type("MockCtx", (), {"session": object()})()
        result = engine.set_current_selection(mock_ctx, connection_guids=["c1", "c2"])
        assert result.get("ok") is True
        sel = engine.read_current_selection(mock_ctx)
        assert sel["pieceGuids"] == []
        assert sel["connectionGuids"] == ["c1", "c2"]

    def test_set_current_selection_both(self):
        """set_current_selection stores both piece and connection guids."""
        mock_ctx = type("MockCtx", (), {"session": object()})()
        result = engine.set_current_selection(mock_ctx, piece_guids=["p1"], connection_guids=["c1"])
        assert result.get("ok") is True
        sel = engine.read_current_selection(mock_ctx)
        assert sel["pieceGuids"] == ["p1"]
        assert sel["connectionGuids"] == ["c1"]

    def test_clear_current_selection(self):
        """clear_current_selection removes selection from session."""
        mock_ctx = type("MockCtx", (), {"session": object()})()
        engine.set_current_selection(mock_ctx, piece_guids=["p1"])
        result = engine.clear_current_selection(mock_ctx)
        assert result.get("ok") is True
        sel = engine.read_current_selection(mock_ctx)
        assert sel == {"pieceGuids": [], "connectionGuids": []}

    def test_show_design_returns_diagram_json(self, kitMetabolismJson: dict):
        """show_design returns CallToolResult with points, lines, and capabilities in structuredContent."""
        mock_ctx = type("MockCtx", (), {"session": object()})()
        sid = id(mock_ctx.session)
        engine._mcp_session_kits[sid] = kitMetabolismJson
        design = next(d for d in kitMetabolismJson.get("designs", []) if d.get("name") == "Nakagin Capsule Tower" and not d.get("parent"))
        engine.start_working_in_design(design["guid"], mock_ctx)
        result = engine.show_design(mock_ctx)
        assert isinstance(result, CallToolResult)
        data = _mcp_app_tool_payload(result)
        assert "points" in data
        assert "lines" in data
        assert "capabilities" in data
        assert isinstance(data["points"], list)
        assert isinstance(data["lines"], list)

    def test_show_diagram_returns_diagram_json(self, kitMetabolismJson: dict):
        """show_diagram returns JSON string with diagram data."""
        mock_ctx = type("MockCtx", (), {"session": object()})()
        sid = id(mock_ctx.session)
        engine._mcp_session_kits[sid] = kitMetabolismJson
        design = next(d for d in kitMetabolismJson.get("designs", []) if d.get("name") == "Nakagin Capsule Tower" and not d.get("parent"))
        engine.start_working_in_design(design["guid"], mock_ctx)
        result = engine.show_diagram(mock_ctx)
        data = _mcp_app_tool_payload(result)
        assert "points" in data
        assert "lines" in data

    def test_show_scene_returns_diagram_json(self, kitMetabolismJson: dict):
        """show_scene returns diagram data in structuredContent."""
        mock_ctx = type("MockCtx", (), {"session": object()})()
        sid = id(mock_ctx.session)
        engine._mcp_session_kits[sid] = kitMetabolismJson
        design = next(d for d in kitMetabolismJson.get("designs", []) if d.get("name") == "Nakagin Capsule Tower" and not d.get("parent"))
        engine.start_working_in_design(design["guid"], mock_ctx)
        result = engine.show_scene(mock_ctx)
        data = _mcp_app_tool_payload(result)
        assert "points" in data
        assert "lines" in data

    def test_shallow_kit_hydrates_nakagin_design_from_disk(self):
        """metabolism.shallow.kit.semio.json lists designs without pieces; load nakagin-capsule-tower.shallow.design.semio.json by guid."""
        mock_ctx = type("MockCtx", (), {"session": object()})()
        shallow_kit_path = ASSETS_DIR / "metabolism.shallow.kit.semio.json"
        engine.start_working_in_local_kit(str(shallow_kit_path), mock_ctx)
        engine.start_working_in_design("9a890dd4-0a9c-48ac-920a-9e62666465ef", mock_ctx)
        d = engine._get_session_design(mock_ctx)
        assert len(d.get("pieces", [])) > 50

    def test_show_diff_returns_diagram_json(self, kitMetabolismJson: dict):
        """show_diff returns diagram data and default capabilities in structuredContent."""
        mock_ctx = type("MockCtx", (), {"session": object()})()
        sid = id(mock_ctx.session)
        engine._mcp_session_kits[sid] = kitMetabolismJson
        design = next(d for d in kitMetabolismJson.get("designs", []) if d.get("name") == "Nakagin Capsule Tower" and not d.get("parent"))
        engine.start_working_in_design(design["guid"], mock_ctx)
        result = engine.show_diff(mock_ctx)
        data = _mcp_app_tool_payload(result)
        assert "points" in data
        assert "lines" in data
        assert data["capabilities"]["pieceSelection"] is False
        assert data["capabilities"]["connectionSelection"] is False

    def test_show_diagram_diff_returns_diagram_json(self, kitMetabolismJson: dict):
        """show_diagram_diff returns diagram data in structuredContent."""
        mock_ctx = type("MockCtx", (), {"session": object()})()
        sid = id(mock_ctx.session)
        engine._mcp_session_kits[sid] = kitMetabolismJson
        design = next(d for d in kitMetabolismJson.get("designs", []) if d.get("name") == "Nakagin Capsule Tower" and not d.get("parent"))
        engine.start_working_in_design(design["guid"], mock_ctx)
        result = engine.show_diagram_diff(mock_ctx)
        data = _mcp_app_tool_payload(result)
        assert "points" in data
        assert "lines" in data

    def test_show_diff_with_design_diff_adds_pieces(self, kitMetabolismJson: dict):
        """show_diff with design_diff includes added pieces in points."""
        mock_ctx = type("MockCtx", (), {"session": object()})()
        sid = id(mock_ctx.session)
        engine._mcp_session_kits[sid] = kitMetabolismJson
        design = next(d for d in kitMetabolismJson.get("designs", []) if d.get("name") == "Nakagin Capsule Tower" and not d.get("parent"))
        engine.start_working_in_design(design["guid"], mock_ctx)
        diff = {"pieces": {"added": [{"guid": "new-piece", "id": "added-1", "center": {"u": 10, "v": 20}}]}}
        result = engine.show_diff(mock_ctx, design_diff=diff)
        data = _mcp_app_tool_payload(result)
        added_guids = [p["guid"] for p in data["points"] if p.get("status") == "added"]
        assert "new-piece" in added_guids

    def test_select_pieces_capabilities(self, kitMetabolismJson: dict):
        """select_pieces sets pieceSelection capability."""
        mock_ctx = type("MockCtx", (), {"session": object()})()
        sid = id(mock_ctx.session)
        engine._mcp_session_kits[sid] = kitMetabolismJson
        design = next(d for d in kitMetabolismJson.get("designs", []) if d.get("name") == "Nakagin Capsule Tower" and not d.get("parent"))
        engine.start_working_in_design(design["guid"], mock_ctx)
        result = engine.select_pieces(mock_ctx)
        data = _mcp_app_tool_payload(result)
        assert data["capabilities"]["pieceSelection"] is True
        assert data["capabilities"]["connectionSelection"] is False

    def test_select_connections_capabilities(self, kitMetabolismJson: dict):
        """select_connections sets connectionSelection capability."""
        mock_ctx = type("MockCtx", (), {"session": object()})()
        sid = id(mock_ctx.session)
        engine._mcp_session_kits[sid] = kitMetabolismJson
        design = next(d for d in kitMetabolismJson.get("designs", []) if d.get("name") == "Nakagin Capsule Tower" and not d.get("parent"))
        engine.start_working_in_design(design["guid"], mock_ctx)
        result = engine.select_connections(mock_ctx)
        data = _mcp_app_tool_payload(result)
        assert data["capabilities"]["pieceSelection"] is False
        assert data["capabilities"]["connectionSelection"] is True

    def test_select_pieces_and_connections_capabilities(self, kitMetabolismJson: dict):
        """select_pieces_and_connections sets both selection capabilities."""
        mock_ctx = type("MockCtx", (), {"session": object()})()
        sid = id(mock_ctx.session)
        engine._mcp_session_kits[sid] = kitMetabolismJson
        design = next(d for d in kitMetabolismJson.get("designs", []) if d.get("name") == "Nakagin Capsule Tower" and not d.get("parent"))
        engine.start_working_in_design(design["guid"], mock_ctx)
        result = engine.select_pieces_and_connections(mock_ctx)
        data = _mcp_app_tool_payload(result)
        assert data["capabilities"]["pieceSelection"] is True
        assert data["capabilities"]["connectionSelection"] is True

    def test_app_tools_require_kit_and_design(self):
        """All app tools return CallToolResult with error in structuredContent when kit or design is not set."""
        mock_ctx = type("MockCtx", (), {"session": object()})()
        sid = id(mock_ctx.session)
        # Ensure clean state
        engine._mcp_session_kits.pop(sid, None)
        engine._mcp_session_designs.pop(sid, None)
        for tool_fn in (engine.show_design, engine.show_diagram, engine.show_scene, engine.select_pieces, engine.select_connections, engine.select_pieces_and_connections):
            result = tool_fn(mock_ctx)
            assert isinstance(result, CallToolResult), f"{tool_fn.__name__} should return CallToolResult"
            assert result.isError is True, f"{tool_fn.__name__} should signal error"
            data = _mcp_app_tool_payload(result)
            assert "error" in data, f"{tool_fn.__name__} should require kit+design"

    def test_show_design_points_have_required_fields(self, kitMetabolismJson: dict):
        """show_design points contain guid, id, u, v, status fields."""
        mock_ctx = type("MockCtx", (), {"session": object()})()
        sid = id(mock_ctx.session)
        engine._mcp_session_kits[sid] = kitMetabolismJson
        design = next(d for d in kitMetabolismJson.get("designs", []) if d.get("name") == "Nakagin Capsule Tower" and not d.get("parent"))
        engine.start_working_in_design(design["guid"], mock_ctx)
        result = engine.show_design(mock_ctx)
        data = _mcp_app_tool_payload(result)
        for point in data["points"]:
            assert "guid" in point
            assert "id" in point
            assert "u" in point
            assert "v" in point
            assert "status" in point

    def test_selection_isolated_between_sessions(self):
        """Selection state is isolated between different sessions."""
        ctx_a = type("MockCtx", (), {"session": object()})()
        ctx_b = type("MockCtx", (), {"session": object()})()
        engine.set_current_selection(ctx_a, piece_guids=["p1"])
        engine.set_current_selection(ctx_b, piece_guids=["p2"])
        assert engine.read_current_selection(ctx_a)["pieceGuids"] == ["p1"]
        assert engine.read_current_selection(ctx_b)["pieceGuids"] == ["p2"]


class TestAppEndpoint:
    def test_app_design_viewer_returns_html(self):
        """GET /app/design-viewer returns the built MCP App HTML that uses @semio/ui."""
        client = TestClient(engine.rest)
        response = client.get("/app/design-viewer")
        assert response.status_code == 200
        assert "text/html" in response.headers["content-type"]
        assert "semio design viewer" in response.text

    def test_app_design_viewer_csp_header(self):
        """The app endpoint includes Content-Security-Policy allowing iframe embedding."""
        client = TestClient(engine.rest)
        response = client.get("/app/design-viewer")
        assert "content-security-policy" in response.headers
        assert "frame-ancestors *" in response.headers["content-security-policy"]

    def test_app_design_viewer_html_structure(self):
        """The HTML contains root element for the React MCP App from @semio/ui."""
        client = TestClient(engine.rest)
        response = client.get("/app/design-viewer")
        html = response.text
        assert 'id="root"' in html

    def test_app_kit_viewer_returns_html(self):
        """GET /app/kit-viewer returns the built MCP App HTML that mounts McpKitViewer from @semio/ui."""
        client = TestClient(engine.rest)
        response = client.get("/app/kit-viewer")
        assert response.status_code == 200
        assert "text/html" in response.headers["content-type"]
        assert "semio kit viewer" in response.text
        assert 'data-mcp-viewer="kit"' in response.text


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
        assert resultOperation["kind"] == engine.OperationKind.KIT
        assert resultOperation["kitUri"] == str(tempKitPath)


# endregion Integration Tests


# region Auth Error Classes Tests
class TestAuthErrors:
    def test_authentication_error(self):
        error = engine.AuthenticationError()
        assert "Authentication failed" in str(error)

    def test_invalid_auth_token_error(self):
        error = engine.InvalidAuthToken("https://example.com")
        assert "https://example.com" in str(error)
        assert "invalid or expired" in str(error)

    def test_auth_token_not_found_error(self):
        error = engine.AuthTokenNotFound("https://example.com")
        assert "https://example.com" in str(error)
        assert "No auth token found" in str(error)

    def test_server_unreachable_error(self):
        error = engine.ServerUnreachable("https://example.com")
        assert "https://example.com" in str(error)
        assert "not reachable" in str(error)

    def test_remote_kit_uri_not_valid_error(self):
        error = engine.RemoteKitUriNotValid("http://bad-uri")
        assert "http://bad-uri" in str(error)
        assert "not valid" in str(error)


# endregion Auth Error Classes Tests


# region Auth Credential Management Tests
class TestAuthCredentials:
    def test_load_auth_empty(self, tmp_path):
        """_load_auth returns empty dict when no auth file exists."""
        with patch.object(engine, "AUTH_FILE", str(tmp_path / "auth.json")):
            result = engine._load_auth()
            assert result == {}

    def test_save_and_load_auth(self, tmp_path):
        """_save_auth writes and _load_auth reads auth credentials."""
        auth_file = str(tmp_path / "auth.json")
        with patch.object(engine, "AUTH_FILE", auth_file):
            auth_data = {"https://server.com": {"token": "tok123", "email": "user@test.com"}}
            engine._save_auth(auth_data)
            loaded = engine._load_auth()
            assert loaded == auth_data

    def test_get_auth_token_found(self, tmp_path):
        """getAuthToken returns the stored token for a server."""
        auth_file = str(tmp_path / "auth.json")
        with patch.object(engine, "AUTH_FILE", auth_file):
            engine._save_auth({"https://server.com": {"token": "tok123", "email": "user@test.com"}})
            token = engine.getAuthToken("https://server.com")
            assert token == "tok123"

    def test_get_auth_token_not_found(self, tmp_path):
        """getAuthToken raises AuthTokenNotFound when no token exists."""
        auth_file = str(tmp_path / "auth.json")
        with patch.object(engine, "AUTH_FILE", auth_file):
            engine._save_auth({})
            with pytest.raises(engine.AuthTokenNotFound):
                engine.getAuthToken("https://server.com")

    def test_get_auth_token_strips_trailing_slash(self, tmp_path):
        """getAuthToken strips trailing slash from server URL."""
        auth_file = str(tmp_path / "auth.json")
        with patch.object(engine, "AUTH_FILE", auth_file):
            engine._save_auth({"https://server.com": {"token": "tok123", "email": "user@test.com"}})
            token = engine.getAuthToken("https://server.com/")
            assert token == "tok123"

    def test_get_auth_status_authenticated(self, tmp_path):
        """getAuthStatus returns authenticated=True when token exists."""
        auth_file = str(tmp_path / "auth.json")
        with patch.object(engine, "AUTH_FILE", auth_file):
            engine._save_auth({"https://server.com": {"token": "tok123", "email": "user@test.com"}})
            status = engine.getAuthStatus("https://server.com")
            assert status["authenticated"] is True
            assert status["email"] == "user@test.com"

    def test_get_auth_status_not_authenticated(self, tmp_path):
        """getAuthStatus returns authenticated=False when no token exists."""
        auth_file = str(tmp_path / "auth.json")
        with patch.object(engine, "AUTH_FILE", auth_file):
            engine._save_auth({})
            status = engine.getAuthStatus("https://server.com")
            assert status["authenticated"] is False
            assert status["email"] == ""

    def test_login_success(self, tmp_path):
        """login stores token on successful server response."""
        auth_file = str(tmp_path / "auth.json")
        mock_response = MagicMock()
        mock_response.json.return_value = {"token": "new-token-123"}
        mock_response.raise_for_status.return_value = None
        with patch.object(engine, "AUTH_FILE", auth_file), patch("engine.requests.post", return_value=mock_response):
            result = engine.login("https://server.com", "user@test.com", "pass123")
            assert result["ok"] is True
            assert result["token"] == "new-token-123"
            assert result["email"] == "user@test.com"
            loaded = engine._load_auth()
            assert "https://server.com" in loaded
            assert loaded["https://server.com"]["token"] == "new-token-123"

    def test_login_connection_error(self, tmp_path):
        """login raises ServerUnreachable on connection error."""
        auth_file = str(tmp_path / "auth.json")
        with patch.object(engine, "AUTH_FILE", auth_file), patch("engine.requests.post", side_effect=engine.requests.exceptions.ConnectionError):
            with pytest.raises(engine.ServerUnreachable):
                engine.login("https://unreachable.com", "user@test.com", "pass")

    def test_login_401_error(self, tmp_path):
        """login raises InvalidAuthToken on 401 response."""
        auth_file = str(tmp_path / "auth.json")
        mock_response = MagicMock()
        mock_response.status_code = 401
        http_error = engine.requests.exceptions.HTTPError(response=mock_response)
        mock_response.raise_for_status.side_effect = http_error
        with patch.object(engine, "AUTH_FILE", auth_file), patch("engine.requests.post", return_value=mock_response):
            with pytest.raises(engine.InvalidAuthToken):
                engine.login("https://server.com", "user@test.com", "wrong-pass")

    def test_logout_removes_token(self, tmp_path):
        """logout removes the stored token for a server."""
        auth_file = str(tmp_path / "auth.json")
        with patch.object(engine, "AUTH_FILE", auth_file):
            engine._save_auth({"https://server.com": {"token": "tok123", "email": "user@test.com"}})
            result = engine.logout("https://server.com")
            assert result["ok"] is True
            loaded = engine._load_auth()
            assert "https://server.com" not in loaded

    def test_logout_nonexistent_server(self, tmp_path):
        """logout succeeds even if server was never logged in."""
        auth_file = str(tmp_path / "auth.json")
        with patch.object(engine, "AUTH_FILE", auth_file):
            engine._save_auth({})
            result = engine.logout("https://nonexistent.com")
            assert result["ok"] is True


# endregion Auth Credential Management Tests


# region RemoteStore Tests
class TestRemoteStore:
    def test_from_uri_valid(self):
        """RemoteStore.fromUri parses server URL and kit URI from remote URI."""
        uri = "https://server.com/api/kits/my-kit"
        store = engine.RemoteStore.fromUri(uri)
        assert store.serverUrl == "https://server.com"
        assert store.kitUri == "my-kit"
        assert store.uri == uri

    def test_from_uri_with_encoded_kit(self):
        """RemoteStore.fromUri handles encoded kit URI."""
        encodedKit = engine.encode("/path/to/kit")
        uri = f"https://server.com/api/kits/{encodedKit}"
        store = engine.RemoteStore.fromUri(uri)
        assert store.serverUrl == "https://server.com"
        assert store.kitUri == "/path/to/kit"

    def test_from_uri_invalid(self):
        """RemoteStore.fromUri raises RemoteKitUriNotValid for bad URIs."""
        with pytest.raises(engine.RemoteKitUriNotValid):
            engine.RemoteStore.fromUri("https://server.com/bad/path")

    def test_get_kit_success(self, tmp_path):
        """RemoteStore.get retrieves kit from remote server."""
        auth_file = str(tmp_path / "auth.json")
        with patch.object(engine, "AUTH_FILE", auth_file):
            engine._save_auth({"https://server.com": {"token": "tok123", "email": "user@test.com"}})
            store = engine.RemoteStore("https://server.com/api/kits/my-kit", "https://server.com", "my-kit")
            mock_response = MagicMock()
            mock_response.json.return_value = {"uri": "my-kit", "name": "TestKit", "version": "1.0.0"}
            mock_response.raise_for_status.return_value = None
            with patch("engine.requests.get", return_value=mock_response):
                result = store.get({"kind": engine.OperationKind.KIT, "kitUri": "my-kit"})
                assert result is not None
                assert result.name == "TestKit"

    def test_get_kit_unauthorized(self, tmp_path):
        """RemoteStore.get raises InvalidAuthToken on 401."""
        auth_file = str(tmp_path / "auth.json")
        with patch.object(engine, "AUTH_FILE", auth_file):
            engine._save_auth({"https://server.com": {"token": "expired-token", "email": "user@test.com"}})
            store = engine.RemoteStore("https://server.com/api/kits/my-kit", "https://server.com", "my-kit")
            mock_response = MagicMock()
            mock_response.status_code = 401
            http_error = engine.requests.exceptions.HTTPError(response=mock_response)
            mock_response.raise_for_status.side_effect = http_error
            with patch("engine.requests.get", return_value=mock_response):
                with pytest.raises(engine.InvalidAuthToken):
                    store.get({"kind": engine.OperationKind.KIT, "kitUri": "my-kit"})

    def test_get_kit_not_found(self, tmp_path):
        """RemoteStore.get raises KitNotFound on 404."""
        auth_file = str(tmp_path / "auth.json")
        with patch.object(engine, "AUTH_FILE", auth_file):
            engine._save_auth({"https://server.com": {"token": "tok123", "email": "user@test.com"}})
            store = engine.RemoteStore("https://server.com/api/kits/my-kit", "https://server.com", "my-kit")
            mock_response = MagicMock()
            mock_response.status_code = 404
            http_error = engine.requests.exceptions.HTTPError(response=mock_response)
            mock_response.raise_for_status.side_effect = http_error
            with patch("engine.requests.get", return_value=mock_response):
                with pytest.raises(engine.KitNotFound):
                    store.get({"kind": engine.OperationKind.KIT, "kitUri": "my-kit"})

    def test_get_kit_connection_error(self, tmp_path):
        """RemoteStore.get raises ServerUnreachable on connection error."""
        auth_file = str(tmp_path / "auth.json")
        with patch.object(engine, "AUTH_FILE", auth_file):
            engine._save_auth({"https://server.com": {"token": "tok123", "email": "user@test.com"}})
            store = engine.RemoteStore("https://server.com/api/kits/my-kit", "https://server.com", "my-kit")
            with patch("engine.requests.get", side_effect=engine.requests.exceptions.ConnectionError):
                with pytest.raises(engine.ServerUnreachable):
                    store.get({"kind": engine.OperationKind.KIT, "kitUri": "my-kit"})

    def test_get_kit_no_auth(self, tmp_path):
        """RemoteStore.get raises AuthTokenNotFound when not logged in."""
        auth_file = str(tmp_path / "auth.json")
        with patch.object(engine, "AUTH_FILE", auth_file):
            engine._save_auth({})
            store = engine.RemoteStore("https://server.com/api/kits/my-kit", "https://server.com", "my-kit")
            with pytest.raises(engine.AuthTokenNotFound):
                store.get({"kind": engine.OperationKind.KIT, "kitUri": "my-kit"})

    def test_put_kit_success(self, tmp_path):
        """RemoteStore.put creates a kit on the remote server."""
        auth_file = str(tmp_path / "auth.json")
        with patch.object(engine, "AUTH_FILE", auth_file):
            engine._save_auth({"https://server.com": {"token": "tok123", "email": "user@test.com"}})
            store = engine.RemoteStore("https://server.com/api/kits/my-kit", "https://server.com", "my-kit")
            mock_response = MagicMock()
            mock_response.raise_for_status.return_value = None
            with patch("engine.requests.put", return_value=mock_response):
                result = store.put({"kind": engine.OperationKind.KIT, "kitUri": "my-kit"}, engine.KitInput(name="TestKit", version="1.0.0"))
                assert result is None

    def test_put_type_success(self, tmp_path):
        """RemoteStore.put creates a type on the remote server."""
        auth_file = str(tmp_path / "auth.json")
        with patch.object(engine, "AUTH_FILE", auth_file):
            engine._save_auth({"https://server.com": {"token": "tok123", "email": "user@test.com"}})
            store = engine.RemoteStore("https://server.com/api/kits/my-kit", "https://server.com", "my-kit")
            mock_response = MagicMock()
            mock_response.raise_for_status.return_value = None
            with patch("engine.requests.put", return_value=mock_response) as mock_put:
                result = store.put(
                    {"kind": engine.OperationKind.TYPE, "kitUri": "my-kit", "typeName": "Brick", "typeVariant": ""},
                    engine.TypeInput(name="Brick", variant=""),
                )
                assert result is None
                call_args = mock_put.call_args
                assert "types/" in call_args[0][0]

    def test_put_design_success(self, tmp_path):
        """RemoteStore.put creates a design on the remote server."""
        auth_file = str(tmp_path / "auth.json")
        with patch.object(engine, "AUTH_FILE", auth_file):
            engine._save_auth({"https://server.com": {"token": "tok123", "email": "user@test.com"}})
            store = engine.RemoteStore("https://server.com/api/kits/my-kit", "https://server.com", "my-kit")
            mock_response = MagicMock()
            mock_response.raise_for_status.return_value = None
            with patch("engine.requests.put", return_value=mock_response) as mock_put:
                result = store.put(
                    {"kind": engine.OperationKind.DESIGN, "kitUri": "my-kit", "designName": "MyDesign", "designVariant": "", "designView": ""},
                    engine.DesignInput(name="MyDesign", variant="", view=""),
                )
                assert result is None
                call_args = mock_put.call_args
                assert "designs/" in call_args[0][0]

    def test_delete_kit_success(self, tmp_path):
        """RemoteStore.delete removes a kit from the remote server."""
        auth_file = str(tmp_path / "auth.json")
        with patch.object(engine, "AUTH_FILE", auth_file):
            engine._save_auth({"https://server.com": {"token": "tok123", "email": "user@test.com"}})
            store = engine.RemoteStore("https://server.com/api/kits/my-kit", "https://server.com", "my-kit")
            mock_response = MagicMock()
            mock_response.raise_for_status.return_value = None
            with patch("engine.requests.delete", return_value=mock_response):
                result = store.delete({"kind": engine.OperationKind.KIT, "kitUri": "my-kit"})
                assert result is None

    def test_delete_type_success(self, tmp_path):
        """RemoteStore.delete removes a type from the remote server."""
        auth_file = str(tmp_path / "auth.json")
        with patch.object(engine, "AUTH_FILE", auth_file):
            engine._save_auth({"https://server.com": {"token": "tok123", "email": "user@test.com"}})
            store = engine.RemoteStore("https://server.com/api/kits/my-kit", "https://server.com", "my-kit")
            mock_response = MagicMock()
            mock_response.raise_for_status.return_value = None
            with patch("engine.requests.delete", return_value=mock_response) as mock_del:
                result = store.delete({"kind": engine.OperationKind.TYPE, "kitUri": "my-kit", "typeName": "Brick", "typeVariant": ""})
                assert result is None
                call_args = mock_del.call_args
                assert "types/" in call_args[0][0]

    def test_delete_design_success(self, tmp_path):
        """RemoteStore.delete removes a design from the remote server."""
        auth_file = str(tmp_path / "auth.json")
        with patch.object(engine, "AUTH_FILE", auth_file):
            engine._save_auth({"https://server.com": {"token": "tok123", "email": "user@test.com"}})
            store = engine.RemoteStore("https://server.com/api/kits/my-kit", "https://server.com", "my-kit")
            mock_response = MagicMock()
            mock_response.raise_for_status.return_value = None
            with patch("engine.requests.delete", return_value=mock_response) as mock_del:
                result = store.delete({"kind": engine.OperationKind.DESIGN, "kitUri": "my-kit", "designName": "MyDesign", "designVariant": "", "designView": ""})
                assert result is None
                call_args = mock_del.call_args
                assert "designs/" in call_args[0][0]

    def test_initialize_noop(self, tmp_path):
        """RemoteStore.initialize is a no-op (server-side initialization)."""
        auth_file = str(tmp_path / "auth.json")
        with patch.object(engine, "AUTH_FILE", auth_file):
            store = engine.RemoteStore("https://server.com/api/kits/my-kit", "https://server.com", "my-kit")
            store.initialize()  # Should not raise

    def test_update_not_supported(self, tmp_path):
        """RemoteStore.update raises FeatureNotYetSupported."""
        auth_file = str(tmp_path / "auth.json")
        with patch.object(engine, "AUTH_FILE", auth_file):
            store = engine.RemoteStore("https://server.com/api/kits/my-kit", "https://server.com", "my-kit")
            with pytest.raises(engine.FeatureNotYetSupported):
                store.update({}, "")


# endregion RemoteStore Tests


# region StoreFactory Remote Tests
class TestStoreFactoryRemote:
    def test_store_factory_remote_uri(self, tmp_path):
        """StoreFactory returns RemoteStore for remote server URIs."""
        engine.StoreFactory.cache_clear()
        uri = "https://server.com/api/kits/my-kit"
        store = engine.StoreFactory(uri)
        assert isinstance(store, engine.RemoteStore)
        assert store.serverUrl == "https://server.com"
        assert store.kitUri == "my-kit"

    def test_store_factory_invalid_remote_uri(self):
        """StoreFactory raises RemoteKitUriNotValid for http URIs without /api/kits/."""
        engine.StoreFactory.cache_clear()
        with pytest.raises(engine.RemoteKitUriNotValid):
            engine.StoreFactory("https://server.com/some/other/path")

    def test_store_factory_local_still_works(self, tempKitPath: pathlib.Path):
        """StoreFactory still returns SqliteStore for local absolute paths."""
        engine.StoreFactory.cache_clear()
        store = engine.StoreFactory(str(tempKitPath))
        assert isinstance(store, engine.SqliteStore)

    def test_store_factory_relative_path_raises(self):
        """StoreFactory raises LocalKitUriIsNotAbsolute for relative paths."""
        engine.StoreFactory.cache_clear()
        with pytest.raises(engine.LocalKitUriIsNotAbsolute):
            engine.StoreFactory("relative/path")


# endregion StoreFactory Remote Tests


# region MCP Auth Tools Tests
class TestMcpAuth:
    def test_mcp_login(self, tmp_path):
        """mcp_login calls login and returns result."""
        auth_file = str(tmp_path / "auth.json")
        mock_response = MagicMock()
        mock_response.json.return_value = {"token": "mcp-token"}
        mock_response.raise_for_status.return_value = None
        with patch.object(engine, "AUTH_FILE", auth_file), patch("engine.requests.post", return_value=mock_response):
            result = engine.mcp_login("https://server.com", "user@test.com", "pass")
            assert result["ok"] is True
            assert result["token"] == "mcp-token"

    def test_mcp_login_error(self, tmp_path):
        """mcp_login returns error dict on connection failure."""
        auth_file = str(tmp_path / "auth.json")
        with patch.object(engine, "AUTH_FILE", auth_file), patch("engine.requests.post", side_effect=engine.requests.exceptions.ConnectionError):
            result = engine.mcp_login("https://unreachable.com", "user@test.com", "pass")
            assert "error" in result

    def test_mcp_logout(self, tmp_path):
        """mcp_logout calls logout and returns result."""
        auth_file = str(tmp_path / "auth.json")
        with patch.object(engine, "AUTH_FILE", auth_file):
            engine._save_auth({"https://server.com": {"token": "tok", "email": "user@test.com"}})
            result = engine.mcp_logout("https://server.com")
            assert result["ok"] is True

    def test_mcp_auth_status_authenticated(self, tmp_path):
        """mcp_auth_status returns authenticated status."""
        auth_file = str(tmp_path / "auth.json")
        with patch.object(engine, "AUTH_FILE", auth_file):
            engine._save_auth({"https://server.com": {"token": "tok", "email": "user@test.com"}})
            result = engine.mcp_auth_status("https://server.com")
            assert result["authenticated"] is True

    def test_mcp_auth_status_not_authenticated(self, tmp_path):
        """mcp_auth_status returns not authenticated status."""
        auth_file = str(tmp_path / "auth.json")
        with patch.object(engine, "AUTH_FILE", auth_file):
            engine._save_auth({})
            result = engine.mcp_auth_status("https://unknown.com")
            assert result["authenticated"] is False


# endregion MCP Auth Tools Tests


# region MCP Remote Kit Tests
class TestMcpRemoteKit:
    def test_start_working_in_remote_kit_success(self, tmp_path):
        """start_working_in_remote_kit fetches kit from remote server."""
        auth_file = str(tmp_path / "auth.json")
        kit_data = {"name": "RemoteKit", "version": "1.0.0", "designs": [], "types": []}
        mock_response = MagicMock()
        mock_response.json.return_value = kit_data
        mock_response.raise_for_status.return_value = None
        mock_ctx = type("MockCtx", (), {"session": object()})()
        with patch.object(engine, "AUTH_FILE", auth_file), patch("engine.requests.get", return_value=mock_response):
            engine._save_auth({"https://server.com": {"token": "tok123", "email": "user@test.com"}})
            result = engine.start_working_in_remote_kit("https://server.com", "my-kit", mock_ctx)
            assert isinstance(result, CallToolResult)
            payload = _mcp_app_tool_payload(result)
            assert "kitArtifacts" in payload
            sid = id(mock_ctx.session)
            assert sid in engine._mcp_session_kits
            assert engine._mcp_session_kit_mode[sid] == "remote"
            assert "/api/kits/" in engine._mcp_session_kit_source[sid]

    def test_start_working_in_remote_kit_no_auth(self, tmp_path):
        """start_working_in_remote_kit returns error when not logged in."""
        auth_file = str(tmp_path / "auth.json")
        mock_ctx = type("MockCtx", (), {"session": object()})()
        with patch.object(engine, "AUTH_FILE", auth_file):
            engine._save_auth({})
            result = engine.start_working_in_remote_kit("https://server.com", "my-kit", mock_ctx)
            assert isinstance(result, CallToolResult)
            assert result.isError is True
            assert "error" in _mcp_app_tool_payload(result)

    def test_start_working_in_remote_kit_connection_error(self, tmp_path):
        """start_working_in_remote_kit returns error on connection failure."""
        auth_file = str(tmp_path / "auth.json")
        mock_ctx = type("MockCtx", (), {"session": object()})()
        with patch.object(engine, "AUTH_FILE", auth_file), patch("engine.requests.get", side_effect=engine.requests.exceptions.ConnectionError):
            engine._save_auth({"https://server.com": {"token": "tok", "email": "user@test.com"}})
            result = engine.start_working_in_remote_kit("https://server.com", "my-kit", mock_ctx)
            assert isinstance(result, CallToolResult)
            assert result.isError is True
            assert "error" in _mcp_app_tool_payload(result)

    def test_start_working_in_remote_kit_clears_previous_state(self, tmp_path):
        """start_working_in_remote_kit clears design, type, and sets mode to remote."""
        auth_file = str(tmp_path / "auth.json")
        kit_data = {"name": "RemoteKit", "version": "1.0.0", "designs": [], "types": []}
        mock_response = MagicMock()
        mock_response.json.return_value = kit_data
        mock_response.raise_for_status.return_value = None
        mock_ctx = type("MockCtx", (), {"session": object()})()
        sid = id(mock_ctx.session)
        engine._mcp_session_designs[sid] = {"guid": "old-design"}
        engine._mcp_session_types[sid] = {"guid": "old-type"}
        engine._mcp_session_kit_mode[sid] = "local"
        with patch.object(engine, "AUTH_FILE", auth_file), patch("engine.requests.get", return_value=mock_response):
            engine._save_auth({"https://server.com": {"token": "tok", "email": "user@test.com"}})
            engine.start_working_in_remote_kit("https://server.com", "my-kit", mock_ctx)
            assert sid not in engine._mcp_session_designs
            assert sid not in engine._mcp_session_types
            assert engine._mcp_session_kit_mode[sid] == "remote"

    def test_start_working_in_local_kit_sets_mode_local(self):
        """start_working_in_local_kit sets session mode to local."""
        mock_ctx = type("MockCtx", (), {"session": object()})()
        result = engine.start_working_in_local_kit(str(KIT_METABOLISM_PATH), mock_ctx)
        assert isinstance(result, CallToolResult)
        payload = _mcp_app_tool_payload(result)
        assert "kitArtifacts" in payload
        sid = id(mock_ctx.session)
        assert engine._mcp_session_kit_mode[sid] == "local"

    def test_get_session_kit_mode_default(self):
        """_get_session_kit_mode returns 'local' when not set."""
        mock_ctx = type("MockCtx", (), {"session": object()})()
        sid = engine._session_id(mock_ctx)
        engine._mcp_session_kit_mode.pop(sid, None)
        mode = engine._get_session_kit_mode(mock_ctx)
        assert mode == "local"

    def test_get_session_kit_mode_remote(self, tmp_path):
        """_get_session_kit_mode returns 'remote' for remote kit sessions."""
        auth_file = str(tmp_path / "auth.json")
        kit_data = {"name": "RemoteKit", "version": "1.0.0", "designs": [], "types": []}
        mock_response = MagicMock()
        mock_response.json.return_value = kit_data
        mock_response.raise_for_status.return_value = None
        mock_ctx = type("MockCtx", (), {"session": object()})()
        with patch.object(engine, "AUTH_FILE", auth_file), patch("engine.requests.get", return_value=mock_response):
            engine._save_auth({"https://server.com": {"token": "tok", "email": "user@test.com"}})
            engine.start_working_in_remote_kit("https://server.com", "my-kit", mock_ctx)
            mode = engine._get_session_kit_mode(mock_ctx)
            assert mode == "remote"

    def test_finish_working_in_kit_clears_mode_and_source(self, kitMetabolismJson: dict):
        """finish_working_in_kit clears mode and source in addition to kit, design, type."""
        mock_ctx = type("MockCtx", (), {"session": object()})()
        sid = id(mock_ctx.session)
        engine._mcp_session_kits[sid] = kitMetabolismJson
        engine._mcp_session_kit_mode[sid] = "remote"
        engine._mcp_session_kit_source[sid] = "https://server.com/api/kits/test"
        result = engine.finish_working_in_kit(mock_ctx)
        assert result["ok"] is True
        assert sid not in engine._mcp_session_kit_mode
        assert sid not in engine._mcp_session_kit_source

    def test_all_mcp_tools_work_after_remote_kit_login(self, tmp_path):
        """All existing MCP tools work after start_working_in_remote_kit (design/type operations)."""
        auth_file = str(tmp_path / "auth.json")
        kit_data = {
            "name": "RemoteKit",
            "version": "1.0.0",
            "designs": [
                {"guid": "d1", "name": "Design1", "pieces": [], "connections": []},
            ],
            "types": [
                {"guid": "t1", "name": "Type1", "connectors": []},
            ],
        }
        mock_response = MagicMock()
        mock_response.json.return_value = kit_data
        mock_response.raise_for_status.return_value = None
        mock_ctx = type("MockCtx", (), {"session": object()})()
        with patch.object(engine, "AUTH_FILE", auth_file), patch("engine.requests.get", return_value=mock_response):
            engine._save_auth({"https://server.com": {"token": "tok", "email": "user@test.com"}})
            engine.start_working_in_remote_kit("https://server.com", "remote-kit", mock_ctx)

        # start_working_in_design works for remote kits
        result = engine.start_working_in_design("d1", mock_ctx)
        assert isinstance(result, CallToolResult)
        payload = _mcp_app_tool_payload(result)
        assert "points" in payload and "lines" in payload
        assert "kitArtifacts" in payload

        # read_current_design works
        design = engine.read_current_design(mock_ctx)
        assert design["guid"] == "d1"

        # finish_working_in_design works
        result = engine.finish_working_in_design(mock_ctx)
        assert result["ok"] is True

        # start_working_in_type works for remote kits
        result = engine.start_working_in_type("t1", mock_ctx)
        assert result["ok"] is True

        # read_current_type works
        t = engine.read_current_type(mock_ctx)
        assert t["guid"] == "t1"

        # finish_working_in_type works
        result = engine.finish_working_in_type(mock_ctx)
        assert result["ok"] is True

        # finish_working_in_kit clears everything
        result = engine.finish_working_in_kit(mock_ctx)
        assert result["ok"] is True


# endregion MCP Remote Kit Tests


# region REST Auth Endpoints Tests
class TestRestAuthEndpoints:
    def test_rest_login_endpoint(self, restClient: TestClient, tmp_path):
        """POST /auth/login endpoint calls login and returns token."""
        auth_file = str(tmp_path / "auth.json")
        mock_response = MagicMock()
        mock_response.json.return_value = {"token": "rest-token"}
        mock_response.raise_for_status.return_value = None
        with patch.object(engine, "AUTH_FILE", auth_file), patch("engine.requests.post", return_value=mock_response):
            response = restClient.post(
                "/auth/login",
                json={
                    "serverUrl": "https://server.com",
                    "email": "user@test.com",
                    "password": "pass123",
                },
            )
            assert response.status_code == 200
            data = response.json()
            assert data["ok"] is True
            assert data["token"] == "rest-token"

    def test_rest_logout_endpoint(self, restClient: TestClient, tmp_path):
        """POST /auth/logout endpoint removes token."""
        auth_file = str(tmp_path / "auth.json")
        with patch.object(engine, "AUTH_FILE", auth_file):
            engine._save_auth({"https://server.com": {"token": "tok", "email": "user@test.com"}})
            response = restClient.post("/auth/logout", json={"serverUrl": "https://server.com"})
            assert response.status_code == 200
            data = response.json()
            assert data["ok"] is True

    def test_rest_auth_status_endpoint(self, restClient: TestClient, tmp_path):
        """GET /auth/status endpoint returns auth status."""
        auth_file = str(tmp_path / "auth.json")
        with patch.object(engine, "AUTH_FILE", auth_file):
            engine._save_auth({"https://server.com": {"token": "tok", "email": "user@test.com"}})
            response = restClient.get("/auth/status", params={"serverUrl": "https://server.com"})
            assert response.status_code == 200
            data = response.json()
            assert data["authenticated"] is True
            assert data["email"] == "user@test.com"

    def test_rest_auth_status_not_authenticated(self, restClient: TestClient, tmp_path):
        """GET /auth/status returns not authenticated for unknown server."""
        auth_file = str(tmp_path / "auth.json")
        with patch.object(engine, "AUTH_FILE", auth_file):
            engine._save_auth({})
            response = restClient.get("/auth/status", params={"serverUrl": "https://unknown.com"})
            assert response.status_code == 200
            data = response.json()
            assert data["authenticated"] is False


# endregion REST Auth Endpoints Tests


# region Load Kit From Remote Tests
class TestLoadKitFromRemote:
    def test_load_kit_from_remote_success(self, tmp_path):
        """_load_kit_from_remote fetches kit from server."""
        auth_file = str(tmp_path / "auth.json")
        kit_data = {"name": "RemoteKit", "version": "1.0.0"}
        mock_response = MagicMock()
        mock_response.json.return_value = kit_data
        mock_response.raise_for_status.return_value = None
        with patch.object(engine, "AUTH_FILE", auth_file), patch("engine.requests.get", return_value=mock_response):
            engine._save_auth({"https://server.com": {"token": "tok", "email": "user@test.com"}})
            result = engine._load_kit_from_remote("https://server.com", "my-kit")
            assert result["name"] == "RemoteKit"

    def test_load_kit_from_remote_connection_error(self, tmp_path):
        """_load_kit_from_remote raises ServerUnreachable on connection error."""
        auth_file = str(tmp_path / "auth.json")
        with patch.object(engine, "AUTH_FILE", auth_file), patch("engine.requests.get", side_effect=engine.requests.exceptions.ConnectionError):
            engine._save_auth({"https://server.com": {"token": "tok", "email": "user@test.com"}})
            with pytest.raises(engine.ServerUnreachable):
                engine._load_kit_from_remote("https://server.com", "my-kit")

    def test_load_kit_from_remote_401(self, tmp_path):
        """_load_kit_from_remote raises InvalidAuthToken on 401."""
        auth_file = str(tmp_path / "auth.json")
        mock_response = MagicMock()
        mock_response.status_code = 401
        http_error = engine.requests.exceptions.HTTPError(response=mock_response)
        mock_response.raise_for_status.side_effect = http_error
        with patch.object(engine, "AUTH_FILE", auth_file), patch("engine.requests.get", return_value=mock_response):
            engine._save_auth({"https://server.com": {"token": "expired", "email": "user@test.com"}})
            with pytest.raises(engine.InvalidAuthToken):
                engine._load_kit_from_remote("https://server.com", "my-kit")

    def test_load_kit_from_remote_404(self, tmp_path):
        """_load_kit_from_remote raises KitNotFound on 404."""
        auth_file = str(tmp_path / "auth.json")
        mock_response = MagicMock()
        mock_response.status_code = 404
        http_error = engine.requests.exceptions.HTTPError(response=mock_response)
        mock_response.raise_for_status.side_effect = http_error
        with patch.object(engine, "AUTH_FILE", auth_file), patch("engine.requests.get", return_value=mock_response):
            engine._save_auth({"https://server.com": {"token": "tok", "email": "user@test.com"}})
            with pytest.raises(engine.KitNotFound):
                engine._load_kit_from_remote("https://server.com", "my-kit")

    def test_load_kit_from_remote_no_token(self, tmp_path):
        """_load_kit_from_remote raises AuthTokenNotFound without login."""
        auth_file = str(tmp_path / "auth.json")
        with patch.object(engine, "AUTH_FILE", auth_file):
            engine._save_auth({})
            with pytest.raises(engine.AuthTokenNotFound):
                engine._load_kit_from_remote("https://server.com", "my-kit")


# endregion Load Kit From Remote Tests
